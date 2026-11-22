# Validação (R4) — 034-gate-manifesto-cli

**Data:** 2026-11-22T16:54:17Z · **Resultado**: **PASS (documental) — aprovado para R7** · R4 precede R7

| Verificação | Saída |
|---|---|
| Âncora | `specs/011` FR-001..FR-006, `manifesto.json`, ciclo 033 (`manifesto.rs`/`lib.rs`) e GDD §1/§14.2 lidos; nenhum esquema novo e nenhuma alteração no contrato. |
| Escopo do código | Somente `crates/sim/examples/verificar_manifesto.rs` escrito; `conferir`, `Cargo.toml`, `Cargo.lock`, `crates/sim/src` e `manifesto.json` intactos. |
| Adaptador (inspeção) | `args_os()` + `into_string()`/`parse::<u8>()` com `map_err`, `fs::read_to_string`/`fs::read`, `Sha256::digest`, hex minúsculo manual, `main() -> ExitCode`; sem `unwrap`/`expect`/`unsafe`/`async`, `forbid(unsafe_code)` + `deny(clippy::pedantic)` presentes. |
| Mapeamento de falhas | `NotFound` em regras → `None` → `REGRAS_AUSENTES`; JSON/campos ausentes → `MANIFEST_INVALID`; uso e demais leituras → exit 2; ordem de `conferir` preservada. |
| Roteiro de testes | `.specify/validations/gate-manifesto-cli/test-roadmap.md` escrito antes do PR: 10 passos (8 casos funcionais + determinismo + inspeção), superfície CLI, capturas de terminal/JSON sem UI, coluna `automatizável?` e rollback. |
| Evidência local (observada) | `./verify.sh` em `2026-11-22T16:54:17Z`: `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`; 5 gates PASS: SDD `33/0`, DAG `296/0`, task-id `0 duplicatas`, converge `16/16`, hub-docs `0 erros`. |
| Evidência remota (observada) | CI GitHub Actions (contextos `gates` + `rust`) no head `7329c53`: run `44952738240`, `conclusion=success`, via `gh`. |
| Execução do roteiro | **[NV]** — nenhum build, teste ou gate executado neste ciclo; a ser executado pelo orquestrador conforme o roteiro. |
| Baseline de converge | **[NV]** — `specs/034-gate-manifesto-cli/converge.md` é arquivo novo e ainda não consta no baseline append-only; bootstrap pelo orquestrador (fora da fronteira deste agente). |
| Governança | `analyze.md` READ-ONLY, checklist revisado, `converge.md` append-only com 1ª entrada em `2026-11-22`, R1 go (scorecard 6/6 strong), rollback documentado. Factory `b1deab1` somente fonte; `docs/GDD*`, baselines, `.gitignore`, `STATE.md`, `PROGRESSO.md`, ledger, CI e 193 paths untracked alheios intactos. |

## R4
Aprovado para R7. A aprovação é documental: baseia-se na âncora lida, na inspeção do adaptador e nas evidências observadas em `2026-11-22T16:54:17Z` e na CI run `44952738240`. Nenhum verde é afirmado para o roteiro, para um `./verify.sh` pós-mudança ou para gates reexecutados — todos **[NV]** até o orquestrador. A autoridade continua sendo `conferir` (011/033); o CLI apenas traduz I/O e saídas. Rollback documentado em `release.md`, nunca executado.
