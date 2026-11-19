#!/usr/bin/env python3
"""Deterministic SDD completeness gate (hub, docs-only, G10 80/20).

Checks that every numbered feature dir under specs/ has the minimum SDD
artifacts, that corresponding assess/validate/release evidence exists under
.specify/ (by slug), and that historical gaps are explicitly baselined.

Strict ratchet (specs >= STRICT_FROM_SPEC, default 88): additionally require
`converge.md` and `analyze.md` inside the spec directory. Legacy specs below
the threshold keep existing behavior and may rely on the baseline for known
historical gaps.

Exit 0 = PASS (no regressions, no stale baseline entries), 1 = FAIL.
Stdlib only; no network.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_BASELINE = ROOT / "docs/governance/sdd-completeness-baseline.json"

STRICT_FROM_SPEC = 88

REQ_SPEC_FILES = ["spec.md", "plan.md", "tasks.md"]
STRICT_SPEC_FILES = ["analyze.md", "converge.md"]
REQ_DERIVED = ["checklists", "assessment", "validate", "release"]


def _slug(spec_dir_name: str) -> str:
    return spec_dir_name.split("-", 1)[1] if "-" in spec_dir_name else spec_dir_name


def _spec_number(spec_dir_name: str) -> int | None:
    prefix = spec_dir_name.split("-", 1)[0]
    try:
        return int(prefix)
    except ValueError:
        return None


def _has_checklist(spec_dir: Path) -> bool:
    cdir = spec_dir / "checklists"
    return cdir.is_dir() and any(p.suffix == ".md" for p in cdir.iterdir() if p.is_file())


def compute_actual(root: Path, strict_from: int = STRICT_FROM_SPEC) -> dict[str, list[str]]:
    specs = sorted((root / "specs").glob("[0-9][0-9][0-9]-*"))
    v = {p.name for p in (root / ".specify/validations").iterdir() if p.is_dir()} if (root / ".specify/validations").exists() else set()
    r = {p.name for p in (root / ".specify/releases").iterdir() if p.is_dir()} if (root / ".specify/releases").exists() else set()
    a = {p.name for p in (root / ".specify/assessments").iterdir() if p.is_dir()} if (root / ".specify/assessments").exists() else set()
    missing: dict[str, list[str]] = {}
    for s in specs:
        slug = _slug(s.name)
        miss = []
        for f in REQ_SPEC_FILES:
            if not (s / f).is_file():
                miss.append(f)
        if not _has_checklist(s):
            miss.append("checklists")
        if slug not in a:
            miss.append("assessment")
        if slug not in v:
            miss.append("validate")
        if slug not in r:
            miss.append("release")
        num = _spec_number(s.name)
        if num is not None and num >= strict_from:
            for f in STRICT_SPEC_FILES:
                if not (s / f).is_file():
                    miss.append(f)
        if miss:
            missing[s.name] = sorted(miss)
    return missing


def load_baseline(path: Path) -> dict[str, list[str]]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except Exception as e:  # noqa: BLE001
        raise SystemExit(f"sdd-completeness: FAIL (cannot read baseline {path}: {e})") from e
    allow = data.get("allow_missing", {})
    out: dict[str, list[str]] = {}
    for k, v in allow.items():
        if isinstance(v, list):
            out[str(k)] = sorted(str(x) for x in v)
    return out


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(
        description="SDD completeness gate (deterministic, docs-only) with strict ratchet for new specs"
    )
    ap.add_argument("--baseline", default=str(DEFAULT_BASELINE), help="path to baseline JSON")
    ap.add_argument("--root", default=str(ROOT), help="repo root")
    ap.add_argument(
        "--strict-from",
        type=int,
        default=STRICT_FROM_SPEC,
        help=f"require converge.md/analyze.md for specs numbered >= N (default: {STRICT_FROM_SPEC})",
    )
    ap.add_argument("--json", action="store_true", help="emit machine-readable summary")
    args = ap.parse_args(argv)
    root = Path(args.root)
    baseline_path = Path(args.baseline)
    actual = compute_actual(root, strict_from=args.strict_from)
    baseline = load_baseline(baseline_path) if baseline_path.exists() else {}

    regressions: dict[str, list[str]] = {}
    stale: dict[str, list[str]] = {}
    # regressions: missing not covered by baseline
    for feat, miss in actual.items():
        allowed = set(baseline.get(feat, []))
        reg = sorted(set(miss) - allowed)
        if reg:
            regressions[feat] = reg
    # stale baseline: entry for feature/artifact that is now present (or feature gone)
    spec_names = {p.name for p in (root / "specs").glob("[0-9][0-9][0-9]-*")}
    for feat, allowed in baseline.items():
        if feat not in spec_names:
            stale[feat] = sorted(allowed)  # feature removed from specs
            continue
        cur = set(actual.get(feat, []))
        now_ok = sorted(a for a in allowed if a not in cur)
        if now_ok:
            stale[feat] = now_ok

    ok = not regressions and not stale
    summary = {
        "schema": "047.2.0",
        "ok": ok,
        "spec_dirs": len(list((root / "specs").glob("[0-9][0-9][0-9]-*"))),
        "features_with_gaps": len(actual),
        "baselined_features": len(baseline),
        "strict_from": args.strict_from,
        "regressions": regressions,
        "stale_baseline": stale,
    }
    if args.json:
        print(json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True))
    else:
        print(f"sdd-completeness: {'PASS' if ok else 'FAIL'} (spec_dirs={summary['spec_dirs']} gaps={summary['features_with_gaps']} baselined={summary['baselined_features']} strict_from={summary['strict_from']})")
        if regressions:
            print("REGRESSIONS (missing artifacts not in baseline):")
            for f in sorted(regressions):
                print(f"  - {f}: {', '.join(regressions[f])}")
        if stale:
            print("STALE_BASELINE (now present — remove from baseline):")
            for f in sorted(stale):
                print(f"  - {f}: {', '.join(stale[f])}")
        if ok and actual:
            print("NOTE: known gaps are explicitly baselined (see docs/governance/sdd-completeness-baseline.json).")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
