#!/usr/bin/env python3
"""Deterministic gate: unique T### task IDs per specs/<NNN>-*/tasks.md.

80/20 SDD integrity check (stdlib, offline). Fails on regressions and stale
baseline entries (ratchet). Exit 0 = PASS, 1 = FAIL.
"""
from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_BASELINE = ROOT / "docs/governance/task-id-uniqueness-baseline.json"
TASK_RE = re.compile(r"^\s*-\s*\[[ xX]\]\s+(T\d{3,})\b")


def compute_actual(root: Path) -> dict[str, list[str]]:
    specs_dir = root / "specs"
    actual: dict[str, list[str]] = {}
    for spec in sorted(specs_dir.glob("[0-9][0-9][0-9]-*")):
        tasks = spec / "tasks.md"
        if not tasks.is_file():
            continue
        ids: list[str] = []
        try:
            text = tasks.read_text(encoding="utf-8")
        except Exception:
            text = tasks.read_text(encoding="utf-8", errors="replace")
        for line in text.splitlines():
            m = TASK_RE.match(line)
            if m:
                ids.append(m.group(1))
        counts = Counter(ids)
        dups = sorted([tid for tid, n in counts.items() if n > 1])
        if dups:
            actual[spec.name] = [f"duplicate_task:{tid}" for tid in dups]
    return actual


def load_baseline(path: Path) -> dict[str, list[str]]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except Exception as e:  # noqa: BLE001
        raise SystemExit(f"task-id-uniqueness: FAIL (cannot read baseline {path}: {e})") from e
    allow = data.get("allow_duplicates", data.get("allow_missing", {}))
    out: dict[str, list[str]] = {}
    for k, v in allow.items():
        if isinstance(v, list):
            out[str(k)] = sorted(str(x) for x in v)
    return out


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="Task ID uniqueness gate (deterministic, docs-only)")
    ap.add_argument("--baseline", default=str(DEFAULT_BASELINE), help="path to baseline JSON")
    ap.add_argument("--root", default=str(ROOT), help="repo root")
    ap.add_argument("--json", action="store_true", help="emit machine-readable summary")
    args = ap.parse_args(argv)
    root = Path(args.root)
    baseline_path = Path(args.baseline)
    actual = compute_actual(root)
    baseline = load_baseline(baseline_path) if baseline_path.exists() else {}

    regressions: dict[str, list[str]] = {}
    stale: dict[str, list[str]] = {}
    for feat, issues in actual.items():
        allowed = set(baseline.get(feat, []))
        reg = sorted(set(issues) - allowed)
        if reg:
            regressions[feat] = reg
    spec_names = {p.name for p in (root / "specs").glob("[0-9][0-9][0-9]-*")}
    for feat, allowed in baseline.items():
        if feat not in spec_names:
            stale[feat] = sorted(allowed)
            continue
        cur = set(actual.get(feat, []))
        now_ok = sorted(a for a in allowed if a not in cur)
        if now_ok:
            stale[feat] = now_ok

    ok = not regressions and not stale
    task_files = len(list((root / "specs").glob("[0-9][0-9][0-9]-*/tasks.md")))
    summary = {
        "schema": "087.1.0",
        "ok": ok,
        "tasks_files": task_files,
        "specs_with_duplicates": len(actual),
        "baselined_specs": len(baseline),
        "regressions": regressions,
        "stale_baseline": stale,
    }
    if args.json:
        print(json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True))
    else:
        print(
            f"task-id-uniqueness: {'PASS' if ok else 'FAIL'} "
            f"(tasks_files={summary['tasks_files']} duplicates={summary['specs_with_duplicates']} "
            f"baselined={summary['baselined_specs']})"
        )
        if regressions:
            print("REGRESSIONS (duplicate task IDs not in baseline):")
            for f in sorted(regressions):
                print(f"  - {f}: {', '.join(regressions[f])}")
        if stale:
            print("STALE_BASELINE (now satisfied or removed — update baseline):")
            for f in sorted(stale):
                print(f"  - {f}: {', '.join(stale[f])}")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
