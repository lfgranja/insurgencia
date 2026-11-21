# Tasks — 033-manifesto-compatibilidade-rust

- [x] T001 branch 033 (base `de3cfb3`) + stub `spec.md` criado pelo script
- [x] T002 assess (intake/research/problem/concept/decision → go, scorecard 6 critérios)
- [x] T003 spec/plan/checklist revisados
- [x] T004 analyze (READ-ONLY)
- [x] T005 registrar âncora: `manifesto.json` (6 campos) + `specs/011` FR-001..FR-006 + GDScript de referência (GDD §1/§14.2)
- [x] T006 registrar lacuna Rust: constantes presentes em `lib.rs`, validador e `REGRAS_AUSENTES` ausentes
- [x] T007 mapear 7 verificações GDScript → 7 testes Rust (`serde_json::json!`, valores do manifesto)
- [x] T008 registrar evidências `21:24:08Z`: 5 gates PASS; `./verify.sh` **[NV]** por contenção de `cargo`; factory `b1deab1` somente fonte
- [x] T009 converge 033 (append-only, primeira entrada 2026-11-21)
- [x] T010 validate (R4 aprovado)
- [x] T011 release (R7, SemVer `1.0.0`, changelog FR-001..FR-006 + T-IDs, rollback documentado — nunca executar)

## Dependências
- T002 — Bloqueado por: T001
- T003 — Bloqueado por: T002
- T004 — Bloqueado por: T003
- T005 — Bloqueado por: T004
- T006 — Bloqueado por: T004
- T007 — Bloqueado por: T005, T006
- T008 — Bloqueado por: T004
- T009 — Bloqueado por: T005, T006, T007, T008
- T010 — Bloqueado por: T009
- T011 — Bloqueado por: T010

## Analyze (READ-ONLY, 2026-11-21T21:24:08Z)
Conflitos: nenhum entre spec, plano, checklist e tasks. O contrato é o de 011 — não há decisão de design aberta; o porte não inventa esquema. A ordem de validação e os códigos literais são extraídos do GDScript de referência; `REGRAS_AUSENTES` é o único literal ainda ausente em Rust. Nenhuma reexecução de gates/build/`verify.sh`: os 5 gates em `21:24:08Z` são colados como evidência e a saída de `verify.sh` permanece **[NV]** por contenção de `cargo`. `converge.md` é append-only: esta é a primeira entrada, datada 2026-11-21. `[x]` em checklist = revisão de requisitos; `[x]` em tasks registra a conclusão documental da Fronteira A (a Fronteira B é de outro dono). R4 precede R7; rollback documentado, nunca executado. Nenhum build/gate/commit/PR, nenhuma alteração em factory, `docs/GDD*`, baselines, CI ou trabalho alheio. PASS.
