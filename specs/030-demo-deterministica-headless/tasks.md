# Tasks — 030-demo-deterministica-headless

- [x] T001 branch 030 + feature.json
- [x] T002 assess sb-16 (intake/research/problem/concept/decision → ADOPT, R1 go)
- [x] T003 spec/plan/checklist revisado
- [x] T004 analyze (READ-ONLY)
- [x] T005 implementar `crates/sim/examples/demo_deterministica.rs` (headless, apenas `insurgencia_sim`)
- [x] T006 verificar compilação: `rustc -D warnings` (rlib pré-construída) + `clippy-driver pedantic`
- [x] T007 verificar determinismo: duas execuções + `diff` vazio (`DETERMINISTIC_OK`)
- [x] T008 converge (append-only, 2026-11-21)
- [x] T009 validate (R4 aprovado)
- [x] T010 release (R7, SemVer 1.0.0, changelog `sb-16`, rollback)
- [ ] T011 PR + CI verde + merge (remoto **[NV]** até confirmação ao vivo)

## Dependências
- T002 — Bloqueado por: T001
- T003 — Bloqueado por: T002
- T004 — Bloqueado por: T003
- T005 — Bloqueado por: T004
- T006 — Bloqueado por: T005
- T007 — Bloqueado por: T005
- T008 — Bloqueado por: T006, T007
- T009 — Bloqueado por: T008
- T010 — Bloqueado por: T009
- T011 — Bloqueado por: T010

## Analyze (READ-ONLY, 2026-11-21T06:00Z)
Conflitos: nenhum entre artefatos. Código: `crates/sim/examples/demo_deterministica.rs` já materializado, usando somente `Mundo`, `Comando`, `PASSO_SEGUNDOS`, `avancar_tick`, `hash_estado`; sem Bevy/janela/rede. Saída reprodutível observada (30 ticks, `hash_final=6fcc2f9e...`). `./verify.sh` (que executa `--all-targets` internamente) cobre o exemplo automaticamente. Nenhum contrato, gate ou constituição alterado. R4 precede R7; janela `--minimizado`; I/VI preservado. PASS.
