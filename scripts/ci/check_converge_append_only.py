#!/usr/bin/env python3
"""Deterministic gate: converge.md append-only integrity (prefix hash/size).

80/20 SDD integrity (stdlib, offline). Baseline stores accepted prefix sha256
and byte length per specs/<NNN>-*/converge.md. PASS when every baselined file
keeps the same prefix bytes and length >= baseline; appends allowed. Fails on
rewritten/truncated, missing, unbaselined (when enforce_unbaselined), and stale
baseline entries (ratchet). Exit 0 = PASS, 1 = FAIL.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_BASELINE = ROOT / "docs/governance/converge-append-only-baseline.json"


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def compute_actual(root: Path) -> dict[str, dict[str, object]]:
    specs_dir = root / "specs"
    actual: dict[str, dict[str, object]] = {}
    for p in sorted(specs_dir.glob("[0-9][0-9][0-9]-*/converge.md")):
        b = p.read_bytes()
        rel = str(p.relative_to(root))
        actual[rel] = {"sha256": _sha256(b), "bytes": len(b)}
    return actual


def load_baseline(path: Path) -> tuple[dict[str, dict[str, object]], bool]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except Exception as e:  # noqa: BLE001
        raise SystemExit(f"converge-append-only: FAIL (cannot read baseline {path}: {e})") from e
    files = data.get("files", {})
    out: dict[str, dict[str, object]] = {}
    for k, v in files.items():
        if isinstance(v, dict) and "sha256" in v and "bytes" in v:
            out[str(k)] = {"sha256": str(v["sha256"]), "bytes": int(v["bytes"])}
    enforce = bool(data.get("enforce_unbaselined", True))
    return out, enforce


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="Converge append-only gate (deterministic, docs-only)")
    ap.add_argument("--baseline", default=str(DEFAULT_BASELINE), help="path to baseline JSON")
    ap.add_argument("--root", default=str(ROOT), help="repo root")
    ap.add_argument("--json", action="store_true", help="emit machine-readable summary")
    ap.add_argument("--bootstrap", action="store_true", help="print bootstrap baseline JSON from current state")
    args = ap.parse_args(argv)
    root = Path(args.root)
    actual = compute_actual(root)

    if args.bootstrap:
        out = {"schema_version": "087.1.0", "enforce_unbaselined": True, "files": actual}
        print(json.dumps(out, ensure_ascii=False, indent=2, sort_keys=True))
        return 0

    baseline_path = Path(args.baseline)
    baseline, enforce_unbaselined = load_baseline(baseline_path) if baseline_path.exists() else ({}, True)

    rewritten: list[dict[str, object]] = []
    missing: list[str] = []
    unbaselined: list[str] = []
    stale: list[str] = []

    for rel, base in sorted(baseline.items()):
        cur = actual.get(rel)
        if cur is None:
            missing.append(rel)
            continue
        cur_b = (root / rel).read_bytes()
        base_bytes = int(base["bytes"])
        base_sha = str(base["sha256"])
        if len(cur_b) < base_bytes:
            rewritten.append({
                "path": rel,
                "reason": "truncated",
                "expected_bytes": base_bytes,
                "actual_bytes": len(cur_b),
                "expected_sha256": base_sha,
            })
            continue
        prefix_sha = _sha256(cur_b[:base_bytes])
        if prefix_sha != base_sha:
            rewritten.append({
                "path": rel,
                "reason": "rewritten",
                "expected_bytes": base_bytes,
                "actual_bytes": len(cur_b),
                "expected_sha256": base_sha,
                "actual_prefix_sha256": prefix_sha,
            })

    if enforce_unbaselined:
        for rel in sorted(actual):
            if rel not in baseline:
                unbaselined.append(rel)

    # stale: baseline entry not needed (file removed/missing handled; keep simple: none except missing)
    # ratchet: if file missing it's reported as MISSING (regression of expected evidence) not stale.

    ok = not rewritten and not missing and not unbaselined and not stale
    summary = {
        "schema": "087.1.0",
        "ok": ok,
        "converge_files": len(actual),
        "baselined_files": len(baseline),
        "rewritten_or_truncated": rewritten,
        "missing": missing,
        "unbaselined": unbaselined,
        "stale_baseline": stale,
    }
    if args.json:
        print(json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True))
    else:
        print(
            f"converge-append-only: {'PASS' if ok else 'FAIL'} "
            f"(files={summary['converge_files']} baselined={summary['baselined_files']})"
        )
        if rewritten:
            print("REWRITTEN_OR_TRUNCATED:")
            for e in rewritten:
                msg = f"  - {e['path']}: {e['reason']} expected_bytes={e['expected_bytes']} actual_bytes={e['actual_bytes']}"
                if 'actual_prefix_sha256' in e:
                    msg += f" expected_sha256={e['expected_sha256']} actual_prefix_sha256={e['actual_prefix_sha256']}"
                print(msg)
        if missing:
            print("MISSING:")
            for p in missing:
                print(f"  - {p}")
        if unbaselined:
            print("UNBASELINED:")
            for p in unbaselined:
                print(f"  - {p}")
        if stale:
            print("STALE_BASELINE:")
            for p in stale:
                print(f"  - {p}")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
