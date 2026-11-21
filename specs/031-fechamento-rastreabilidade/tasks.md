# Tasks — 031-fechamento-rastreabilidade

- [x] T001 branch 031 + stub `spec.md` criado pelo script
- [x] T002 assess (intake/research/problem/concept/decision → go, scorecard 6 critérios)
- [x] T003 spec/plan/checklist revisados
- [x] T004 analyze (READ-ONLY)
- [x] T005 fechar rastreabilidade 029: T004–T008 `[x]` + nota datada (PR #35, CI `44869231266`, STATE 04:33Z)
- [x] T006 fechar rastreabilidade 028: T011 `[x]` + nota datada (PR #34, CI `44868073016`)
- [x] T007 errata de redação nos 8 arquivos designados (`./verify.sh` executa `--all-targets` internamente)
- [x] T008 append datado em `converge.md` (030) e `docs/PROGRESSO.md`
- [x] T009 ajustar frase de Lightyear em `AGENTS.md`
- [x] T010 converge 031 (append-only, 2026-11-21)
- [x] T011 validate (R4 aprovado)
- [x] T012 release (R7, SemVer `1.0.0`, changelog FR-001..FR-006 + T-IDs, rollback)

## Dependências
- T002 — Bloqueado por: T001
- T003 — Bloqueado por: T002
- T004 — Bloqueado por: T003
- T005 — Bloqueado por: T004
- T006 — Bloqueado por: T004
- T007 — Bloqueado por: T004
- T008 — Bloqueado por: T005, T006, T007
- T009 — Bloqueado por: T004
- T010 — Bloqueado por: T008, T009
- T011 — Bloqueado por: T010
- T012 — Bloqueado por: T011

## Analyze (READ-ONLY, 2026-11-21T13:23Z)
Conflitos: nenhum entre spec, plano, checklist e tasks; a errata é de redação, não de comportamento. Código/gates/CLI intactos: `./verify.sh` já executa `--all-targets` internamente (`grep verify.sh:5-15,70,75`, 13:19Z), portanto nenhuma flag deve ser acrescentada ao comando. Evidências datadas são suficientes e não devem ser reexecutadas. `converge.md` é append-only: entradas antigas permanecem, novas entradas `2026-11-21T13:23Z` são acrescentadas. `[x]` em checklist = revisão de requisitos, não implementação. R4 precede R7; release `1.0.0` com changelog FR/T e rollback. Nenhum contrato, gate, constituição ou trabalho alheio alterado. PASS.
