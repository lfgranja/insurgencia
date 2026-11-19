#!/usr/bin/env python3
"""Deterministic work-order DAG integrity gate (spec 090; stdlib/offline).

Parses ``T###`` task declarations and ``Bloqueado por:`` / ``blocked_by:``
edges from markdown task files. Fails closed on duplicate IDs, self-edges,
orphan edges and cycles, and emits a stable Kahn topological order with a
reproducible SHA-256 hash. Pure function of its inputs: no network, no
subprocess, no file writes.

Deterministic choices (documented for callers):

* A path that is not a readable file yields ``file_not_found:<key>`` (exit 1),
  never a traceback.
* When the graph has a cycle no full topological order exists; the emitted
  order is the Kahn prefix followed by the remaining IDs in stable numeric
  order, so ``order_sha256`` stays reproducible even on failing input.
* Errors are keyed by file (path relative to ``--root`` when possible) and
  sorted lexicographically; cycle IDs are rotated to start at the smallest
  ``T###`` and joined by commas.
"""
from __future__ import annotations

import argparse
import hashlib
import heapq
import json
import re
from collections import Counter
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
SCHEMA = "090.1.0"

TASK_RE = re.compile(r"^\s*-\s*\[[ xX]\]\s+(T\d{3,})\b")
MARKER_RE = re.compile(r"bloqueado por:|blocked_by:", re.IGNORECASE)
TARGET_RE = re.compile(r"T\d{3,}\b")


def _tid_key(tid: str) -> tuple[int, str]:
    """Ordena ``T010`` depois de ``T003`` pelo inteiro do sufixo."""
    return (int(tid[1:]), tid)


def _sha256_order(order: list[str]) -> str:
    canonical = json.dumps(order, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest()


@dataclass
class FileReport:
    key: str
    ids: list[str] = field(default_factory=list)
    edges: list[tuple[str, str]] = field(default_factory=list)
    errors: list[str] = field(default_factory=list)
    order: list[str] = field(default_factory=list)
    order_sha256: str = ""

    @property
    def task_count(self) -> int:
        return len(set(self.ids))


def _normalize_cycle(cycle: list[str]) -> str:
    """Rotaciona o ciclo para começar no menor ID (ordem estável)."""
    rotated = min(
        (cycle[i:] + cycle[:i] for i in range(len(cycle))),
        key=lambda c: tuple(_tid_key(t) for t in c),
    )
    return ",".join(rotated)


def _find_cycles(nodes: list[str], edges: list[tuple[str, str]]) -> list[str]:
    """DFS branco/cinza/preto; devolve ciclos distintos em ordem estável."""
    adj: dict[str, list[str]] = {n: [] for n in nodes}
    for src, dst in edges:
        adj[src].append(dst)
    for node in nodes:
        adj[node] = sorted(set(adj[node]), key=_tid_key)

    color: dict[str, int] = {}
    stack: list[str] = []
    cycles: set[str] = set()

    def visit(node: str) -> None:
        color[node] = 1
        stack.append(node)
        for nxt in adj[node]:
            state = color.get(nxt, 0)
            if state == 0:
                visit(nxt)
            elif state == 1:
                cycles.add(_normalize_cycle(stack[stack.index(nxt) :]))
        stack.pop()
        color[node] = 2

    for node in sorted(nodes, key=_tid_key):
        if color.get(node, 0) == 0:
            visit(node)
    return sorted(cycles)


def _topological_order(nodes: list[str], edges: list[tuple[str, str]]) -> list[str]:
    """Ordem de execução: bloqueador antes do dependente, desempate numérico.

    ``edges`` são (src, dst) onde ``dst`` está bloqueado por ``src``. Para que a
    lista sirva como ordem de execução, o grafo de Kahn é invertido: src -> dst
    vira aresta dst -> src, e as raízes são as tarefas sem bloqueadores.
    """
    indeg = {n: 0 for n in nodes}
    adj: dict[str, list[str]] = {n: [] for n in nodes}
    for src, dst in edges:
        adj[dst].append(src)
        indeg[src] += 1

    heap: list[tuple[int, str]] = [(_tid_key(n)[0], n) for n in nodes if indeg[n] == 0]
    heapq.heapify(heap)
    order: list[str] = []
    while heap:
        _, node = heapq.heappop(heap)
        order.append(node)
        for nxt in adj[node]:
            indeg[nxt] -= 1
            if indeg[nxt] == 0:
                heapq.heappush(heap, (_tid_key(nxt)[0], nxt))
    if len(order) < len(nodes):
        placed = set(order)
        order.extend(sorted((n for n in nodes if n not in placed), key=_tid_key))
    return order


def analyze_text(key: str, text: str) -> FileReport:
    """Analisa um arquivo já carregado em memória (base do ``--self-test``)."""
    ids: list[str] = []
    raw_edges: list[tuple[str, str]] = []
    self_edges: set[str] = set()

    for line in text.splitlines():
        match = TASK_RE.match(line)
        if match is None:
            continue
        source = match.group(1)
        ids.append(source)
        marker = MARKER_RE.search(line)
        if marker is None:
            continue
        targets = TARGET_RE.findall(line[marker.end() :])
        if not targets:
            continue
        # A auto-aresta é detectada nos alvos brutos; depois é ignorada no grafo.
        if source in targets:
            self_edges.add(f"self_edge:{source}")
        for target in dict.fromkeys(targets):  # aresta duplicada na linha: dedup silencioso
            if target != source:
                raw_edges.append((source, target))

    errors: list[str] = sorted(self_edges)
    declared = set(ids)
    counts = Counter(ids)
    errors.extend(f"duplicate_task:{tid}" for tid, n in counts.items() if n > 1)

    edges: list[tuple[str, str]] = []
    for src, dst in dict.fromkeys(raw_edges):
        if dst not in declared:
            errors.append(f"orphan_edge:{src}->:{dst}")
        else:
            edges.append((src, dst))

    nodes = sorted(declared, key=_tid_key)
    errors.extend(f"cycle:{cyc}" for cyc in _find_cycles(nodes, edges))

    order = _topological_order(nodes, edges)
    return FileReport(
        key=key,
        ids=ids,
        edges=edges,
        errors=sorted(set(errors)),
        order=order,
        order_sha256=_sha256_order(order),
    )


def _file_key(path: Path, root: Path) -> str:
    try:
        return path.resolve().relative_to(root.resolve()).as_posix()
    except ValueError:
        return str(path)


def analyze_path(key: str, path: Path) -> FileReport:
    if not path.is_file():
        return FileReport(key=key, errors=[f"file_not_found:{key}"], order_sha256=_sha256_order([]))
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        text = path.read_text(encoding="utf-8", errors="replace")
    return analyze_text(key, text)


def collect(root: Path, paths: list[str]) -> list[FileReport]:
    if paths:
        return [analyze_path(_file_key(Path(p), root), Path(p)) for p in paths]
    specs = root / "specs"
    return [
        analyze_path(_file_key(p, root), p)
        for p in sorted(specs.glob("[0-9][0-9][0-9]-*/tasks.md"))
    ]


def summarize(reports: list[FileReport]) -> dict[str, Any]:
    errors = {r.key: r.errors for r in reports if r.errors}
    return {
        "schema": SCHEMA,
        "ok": not errors,
        "files": len(reports),
        "tasks": sum(r.task_count for r in reports),
        "edges": sum(len(r.edges) for r in reports),
        "errors": errors,
        "orders": {
            r.key: {"order": r.order, "order_sha256": r.order_sha256} for r in reports
        },
    }


def emit(summary: dict[str, Any], as_json: bool) -> None:
    if as_json:
        print(json.dumps(summary, sort_keys=True, indent=2, ensure_ascii=False))
        return
    cycles = sum(
        1 for errs in summary["errors"].values() for err in errs if err.startswith("cycle:")
    )
    status = "PASS" if summary["ok"] else "FAIL"
    print(
        f"work-order-dag: {status} (files={summary['files']} tasks={summary['tasks']} "
        f"edges={summary['edges']} cycles={cycles})"
    )
    for key in sorted(summary["errors"]):
        for err in summary["errors"][key]:
            print(f"  - {key}: {err}")


SELF_TEST_CASES: list[tuple[str, str, bool, list[str], int, int]] = [
    (
        "dag-limpo",
        "- [ ] T001 Primeira\n"
        "- [ ] T002 Segunda — Bloqueado por: T001\n"
        "- [ ] T003 Terceira — Bloqueado por: T002\n",
        True,
        [],
        3,
        2,
    ),
    (
        "aresta-orfa",
        "- [ ] T001 Faz — Bloqueado por: T099\n",
        False,
        ["orphan_edge:T001->:T099"],
        1,
        0,
    ),
    (
        "auto-aresta",
        "- [ ] T001 Faz — Bloqueado por: T001\n",
        False,
        ["self_edge:T001"],
        1,
        0,
    ),
    (
        "ciclo",
        "- [ ] T001 A — Bloqueado por: T002\n"
        "- [ ] T002 B — Bloqueado por: T001\n",
        False,
        ["cycle:T001,T002"],
        2,
        2,
    ),
    (
        "id-duplicado",
        "- [ ] T001 A\n- [ ] T001 B\n",
        False,
        ["duplicate_task:T001"],
        1,
        0,
    ),
    (
        "sem-marcador",
        "# Tarefas\n- [ ] T001 A\n- [ ] T002 B\n",
        True,
        [],
        2,
        0,
    ),
]


def run_self_test() -> int:
    passed = 0
    total = len(SELF_TEST_CASES) + 1
    for name, text, expect_ok, expected_errors, expect_tasks, expect_edges in SELF_TEST_CASES:
        report = analyze_text(f"<self-test:{name}>", text)
        ok = (
            (not report.errors) == expect_ok
            and report.task_count == expect_tasks
            and len(report.edges) == expect_edges
            and all(want in report.errors for want in expected_errors)
        )
        if ok:
            passed += 1
        else:
            print(
                f"self-test: FAIL {name} errors={report.errors} "
                f"tasks={report.task_count} edges={len(report.edges)}"
            )

    first = analyze_text("<self-test:ordem>", SELF_TEST_CASES[0][1])
    second = analyze_text("<self-test:ordem>", SELF_TEST_CASES[0][1])
    if first.order == second.order and first.order_sha256 == second.order_sha256:
        passed += 1
    else:
        print("self-test: FAIL ordem-reprodutivel")

    print(f"self-test: {'PASS' if passed == total else 'FAIL'} ({passed}/{total})")
    return 0 if passed == total else 1


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="Work-order DAG integrity gate (deterministic, docs-only)")
    ap.add_argument("paths", nargs="*", help="markdown extra; se dados, substituem a varredura padrão")
    ap.add_argument("--root", default=str(ROOT), help="raiz do repositório")
    ap.add_argument("--json", action="store_true", help="resumo JSON com chaves ordenadas")
    ap.add_argument("--self-test", action="store_true", help="executa casos embutidos em memória")
    args = ap.parse_args(argv)

    if args.self_test:
        return run_self_test()

    summary = summarize(collect(Path(args.root), args.paths))
    emit(summary, args.json)
    return 0 if summary["ok"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
