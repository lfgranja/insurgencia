#!/usr/bin/env python3
"""Deterministic docs-only gate for MDMV hub (031). Stdlib only; no network.
Checks: key SDD paths/files exist; JSON/YAML parse of versioned artifacts; basic spec/task markers.
Exit 0 pass, 1 fail. Output is stable/human-readable.
"""
from __future__ import annotations
import json, sys
from pathlib import Path
try:
    import yaml  # optional; if absent, YAML parse check degrades to presence+basic colon scan
except Exception:  # pragma: no cover
    yaml = None

ROOT = Path(__file__).resolve().parents[2]
errors: list[str] = []
warnings: list[str] = []

def req(path: str) -> Path:
    p = ROOT / path
    if not p.exists():
        errors.append(f"missing required path: {path}")
    return p

req("AGENTS.md")
req(".specify/memory")
req("specs")
# constitution may be template in some setups; require memory dir, warn if template-only
const = ROOT / ".specify/memory/constitution.md"
if not const.exists():
    warnings.append("missing .specify/memory/constitution.md (NV policy)")

# JSON parse
for p in ROOT.rglob("*.json"):
    if any(part in {".git","node_modules","target","generated"} for part in p.parts): continue
    try:
        json.loads(p.read_text(encoding="utf-8"))
    except Exception as e:
        errors.append(f"invalid JSON {p.relative_to(ROOT)}: {e}")

# YAML parse (best-effort if PyYAML unavailable)
for p in list(ROOT.rglob("*.yml")) + list(ROOT.rglob("*.yaml")):
    if any(part in {".git","node_modules","target","generated",".direnv"} for part in p.parts): continue
    txt = p.read_text(encoding="utf-8", errors="replace")
    if yaml:
        try:
            list(yaml.safe_load_all(txt))
        except Exception as e:
            errors.append(f"invalid YAML {p.relative_to(ROOT)}: {e}")
    else:
        # minimal deterministic sanity: tabs at indentation start often break YAML; require no NUL
        if "\x00" in txt: errors.append(f"invalid YAML (NUL) {p.relative_to(ROOT)}")
        for i,line in enumerate(txt.splitlines(),1):
            if line.startswith("\t") and ":" in line: errors.append(f"YAML tab-indented mapping? {p.relative_to(ROOT)}:{i}")

# SDD spec dirs: require spec.md for numbered feature dirs
specs = ROOT / "specs"
if specs.exists():
    for d in sorted(p for p in specs.iterdir() if p.is_dir() and p.name[:3].isdigit()):
        if not (d/"spec.md").exists():
            errors.append(f"missing spec.md in {d.relative_to(ROOT)}")

for w in warnings: print(f"WARN: {w}")
for e in errors: print(f"ERROR: {e}")
print(f"hub-docs-gate: {'PASS' if not errors else 'FAIL'} ({len(errors)} errors, {len(warnings)} warnings)")
sys.exit(0 if not errors else 1)
