# Validate (R4) — emenda-gdd-adrs

- **Feature**: `specs/002-emenda-gdd-adrs`
- **Data/hora (execução 1)**: 2026-11-19T22:38:24Z
- **ALVO**: `/home/luis/development/MDMV/projetos/insurgencia` — HEAD base `723ef49` — branch `feature/002-emenda-gdd-adrs`
- **Fonte do aparato**: factory HEAD `b1deab15f482dd85871cd6c6a857ad710b89e2a6` (somente fonte; Bloco 0.a)
- **Veredito R4**: aprovado após a execução 2 (§Evidência final), com ressalvas registradas.

## Gate determinístico — execução 1 (antes deste artefato)

```
$ python3 scripts/ci/check_sdd_completeness.py
sdd-completeness: FAIL (spec_dirs=2 gaps=1 baselined=0 strict_from=88)
REGRESSIONS (missing artifacts not in baseline):
  - 002-emenda-gdd-adrs: release, validate
exit=1

$ python3 scripts/ci/check_work_order_dag.py
work-order-dag: PASS (files=2 tasks=61 edges=0 cycles=0)
exit=0

$ python3 scripts/ci/check_task_id_uniqueness.py
task-id-uniqueness: PASS (tasks_files=2 duplicates=0 baselined=0)
exit=0

$ python3 scripts/ci/check_converge_append_only.py
converge-append-only: PASS (files=0 baselined=0)
exit=0

$ python3 scripts/ci/hub_docs_gate.py
hub-docs-gate: PASS (0 errors, 0 warnings)
exit=0
```

O FAIL da execução 1 é esperado: validate/release ainda não existiam. Não é RED de implementação.

## Conformidade verificada

| Requisito | Evidência local | Resultado |
|---|---|---|
| R1 `go` | `.specify/assessments/emenda-gdd-adrs/decision.md:5` | PASS |
| R3 — 3 Q/A, uma por vez | `specs/002-.../spec.md` `## Clarifications` | PASS |
| 7 passagens emendadas | GDD `1389,1451,1459,1594,1659,1668,1673` | PASS |
| R-1 / SC-004 — 7/7 passagens com ADR | verificação isolada (ADR-002 em seis; ADR-001 em `1594`) | PASS |
| Âncoras Godot-first `1265`, `1678` intactas | verificação isolada por leitura | PASS |
| MIT-only, sem "ou GPL"/AGPL do projeto | GDD `1451,1459,1659,1668,1673` | PASS |
| CI dia 1 → gate local + DEFER remoto `[NV]` | GDD `1594` | PASS |
| 26/26 tarefas `[X]` | `specs/002-.../tasks.md` | PASS |
| Analyze read-only + remediação | `docs/governance/analyze-remediation-002.md` + 3 reruns | PASS |
| Converge sem gaps | `tasks.md` inalterado após o implement | PASS |

## Limitações

- `[NV]` — aprovação humana final, CI/merge/proteções no GitHub, execução remota de gates.
- GDD é untracked: byte-intactness das âncoras foi verificada por leitura comparada, não por `git diff`.
- `docs/governance/sdd-completeness-baseline.json` permanece não materializado (dívida do ciclo 001).

## Evidência final — execução 2 (2026-11-19T22:38:57Z)

```
$ python3 scripts/ci/check_sdd_completeness.py
sdd-completeness: PASS (spec_dirs=2 gaps=0 baselined=0 strict_from=88)
exit=0

$ python3 scripts/ci/check_work_order_dag.py
work-order-dag: PASS (files=2 tasks=61 edges=0 cycles=0)
exit=0

$ python3 scripts/ci/check_task_id_uniqueness.py
task-id-uniqueness: PASS (tasks_files=2 duplicates=0 baselined=0)
exit=0

$ python3 scripts/ci/check_converge_append_only.py
converge-append-only: PASS (files=0 baselined=0)
exit=0

$ python3 scripts/ci/hub_docs_gate.py
hub-docs-gate: PASS (0 errors, 0 warnings)
exit=0
```

R4 aprovado com base nas cinco gates determinísticos locais acima. Nenhuma afirmação remota foi
usada para aprovar este artefato.
