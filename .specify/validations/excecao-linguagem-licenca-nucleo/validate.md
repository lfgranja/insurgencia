# Validate (R4) — excecao-linguagem-licenca-nucleo

- **Feature**: `specs/001-excecao-linguagem-licenca-nucleo`
- **Data/hora (execução 1)**: 2026-11-19T21:46:12Z
- **ALVO**: `/home/luis/development/MDMV/projetos/insurgencia` — HEAD `6e25758` — branch `feature/001-excecao-linguagem-licenca-nucleo`
- **Fonte do aparato**: factory HEAD `b1deab15f482dd85871cd6c6a857ad710b89e2a6` (somente fonte, não autoridade — Bloco 0.a)
- **Veredito R4**: aprovado após a execução 2 (ver §Evidência final), com ressalvas registradas.

## Gate determinístico — execução 1 (antes deste artefato)

```
$ python3 scripts/ci/check_sdd_completeness.py
sdd-completeness: FAIL (spec_dirs=1 gaps=1 baselined=0 strict_from=88)
REGRESSIONS (missing artifacts not in baseline):
  - 001-excecao-linguagem-licenca-nucleo: release, validate
exit=1

$ python3 scripts/ci/check_work_order_dag.py
work-order-dag: PASS (files=1 tasks=35 edges=0 cycles=0)
exit=0

$ python3 scripts/ci/check_task_id_uniqueness.py
task-id-uniqueness: PASS (tasks_files=1 duplicates=0 baselined=0)
exit=0

$ python3 scripts/ci/check_converge_append_only.py
converge-append-only: PASS (files=0 baselined=0)
exit=0

$ python3 scripts/ci/hub_docs_gate.py
hub-docs-gate: PASS (0 errors, 0 warnings)
exit=0

$ python3 scripts/ci/check_production_gate.py --check
python3: can't open file '.../scripts/ci/check_production_gate.py': [Errno 2] No such file or directory
exit=2
```

Leitura: o FAIL de `check_sdd_completeness` na execução 1 é esperado — os artefatos `validate` e
`release` ainda não existiam. `exit=2` de `check_production_gate.py` não é RED: o comando foi
classificado como N/A no Bloco 0.a (gate da factory ausente no alvo por decisão de governança).

## Conformidade verificada

| Requisito | Evidência local | Resultado |
|---|---|---|
| R1 — decisão `go` | `.specify/assessments/excecao-linguagem-licenca-nucleo/decision.md:5` | PASS |
| R3 —≤3 esclarecimentos, um por vez | `spec.md` `## Clarifications` (4 Q/A, a 4ª por premissa) | PASS |
| Spec/plan/checklist/tasks materializados | `specs/001-.../` (9 artefatos) | PASS |
| Analyze read-only; 14 achados remediated | `docs/governance/analyze-remediation-2026-11-19.md` + 3 reruns | PASS |
| Implement documental | `docs/adr/001-*.md`, `docs/adr/002-*.md`; 35/35 tarefas `[X]` | PASS (verificador isolado) |
| Converge sem gaps | `tasks.md` inalterado byte-for-byte após o implement; nenhuma entrada append | PASS |
| Sem código/CI/serviço introduzido | somente Markdown materializado | PASS |

## Limitações

- `[NV]` — aprovação humana dos ADRs, status de branch/PR/CI/merge/proteções no GitHub.
- `[NV]` — execução remota de qualquer gate; apenas execução local foi observada.
- `docs/governance/sdd-completeness-baseline.json` não foi copiado da factory (conteria chaves de
  specs alheias); o mantenedor da governança do alvo deve materializá-lo quando necessário.

## Evidência final — execução 2 (2026-11-19T21:46:45Z)

```
$ python3 scripts/ci/check_sdd_completeness.py
sdd-completeness: PASS (spec_dirs=1 gaps=0 baselined=0 strict_from=88)
exit=0

$ python3 scripts/ci/check_work_order_dag.py
work-order-dag: PASS (files=1 tasks=35 edges=0 cycles=0)
exit=0

$ python3 scripts/ci/check_task_id_uniqueness.py
task-id-uniqueness: PASS (tasks_files=1 duplicates=0 baselined=0)
exit=0

$ python3 scripts/ci/check_converge_append_only.py
converge-append-only: PASS (files=0 baselined=0)
exit=0

$ python3 scripts/ci/hub_docs_gate.py
hub-docs-gate: PASS (0 errors, 0 warnings)
exit=0

$ python3 -m pytest -q
no tests ran in 0.36s
exit=5   # N/A — nenhuma suíte pytest existe no alvo; não é RED
```

R4 aprovado com base nas cinco gates determinísticos locais acima. Nenhuma afirmação remota foi
usada para aprovar este artefato.
