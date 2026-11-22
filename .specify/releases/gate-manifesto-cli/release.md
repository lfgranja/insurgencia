# Release (R7) — 034-gate-manifesto-cli

**Data:** 2026-11-22 · **Versão:** `1.0.0` (SemVer) · **R4:** aprovado em `.specify/validations/gate-manifesto-cli/validate.md` (antes desta release)

## Changelog
### [1.0.0] — 2026-11-22
### Added
- FR-001 / T001, T014 — CLI headless `verificar_manifesto <manifesto.json> <regras.cfg> [module_id] [versao_local]`, com defaults de `MODULO_ID`/`PROTOCOL_VERSION` e exit 2 para uso inválido.
- FR-002 / T008, T014 — adaptador com toda a I/O: leitura do manifesto e das regras; `NotFound` nas regras mapeado a `None` → `REGRAS_AUSENTES`; demais falhas de leitura → exit 2; biblioteca `contratos` intacta.
- FR-003 / T014 — SHA-256 do arquivo de regras com `sha2 = "=0.10.9"` e hex minúsculo por formatação manual, sem crate `hex` e sem dependência nova.
- FR-004 / T014 — chamada a `conferir` como única autoridade; `COMPAT_OK` em stdout com exit 0 e `COMPAT_FALHA <codigo>` em stderr com exit 1, na ordem formato → versão → módulo → 64 hex → regras → hash.
- FR-005 / T014 — higiene Rust: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`unsafe`/`async`, `main() -> ExitCode`, `max_width=100`, nenhum `Cargo.toml`/`Cargo.lock` alterado.
- FR-006 / T002–T013, T015–T018 — SDD completo na ordem normativa, 14 artefatos + 1 arquivo de código, `analyze.md` READ-ONLY, `converge.md` append-only, R4 aprovado antes de R7, `test-roadmap.md` com 8 casos e 3 execuções idênticas por `diff`, rollback documentado.
- Evidências coladas: `./verify.sh` `ALL STAGES OK` / 41 de 41 / `VERIFY_EXIT=0` e 5 gates PASS em `2026-11-22T16:54:17Z`; CI run `44952738240` `success` no head `7329c53`. Roteiro e gates pós-mudança permanecem **[NV]** até execução pelo orquestrador.

## Rastreabilidade
FR-001..FR-006 · SC-001..SC-006 · T001..T018 · R1 go (`.specify/assessments/gate-manifesto-cli/decision.md`, 6/6 strong) · R4 (`.specify/validations/gate-manifesto-cli/validate.md`). Âncora: `specs/011-compatibilidade-manifesto/spec.md`, `manifesto.json`, ciclo 033, `docs/GDD - Insurgência.md` §1/§14.2. Roteiro: `.specify/validations/gate-manifesto-cli/test-roadmap.md`.

## Rollback

Para reverter o ciclo 034, remover `crates/sim/examples/verificar_manifesto.rs`, os 5 artefatos de `.specify/assessments/gate-manifesto-cli/`, os 6 artefatos de `specs/034-gate-manifesto-cli/`, `validate.md`, `release.md` e `test-roadmap.md`, e restaurar o stub original de `specs/034-gate-manifesto-cli/spec.md`. Se o orquestrador já tiver bootstrapado o baseline de converge, remover também a entrada de `specs/034-gate-manifesto-cli/converge.md` desse baseline. **Nunca executar neste ciclo — apenas documentar.** `crates/contratos`, `crates/sim/src`, `Cargo.toml`, `Cargo.lock`, `manifesto.json`, `specs/011`, factory `b1deab1`, `docs/GDD*`, baselines pré-existentes, `.gitignore`, `STATE.md`, `PROGRESSO.md`, ledger, CI e 193 paths untracked alheios não são afetados. Nenhum commit, build, gate, push ou PR executado.

## Não incluído
Segunda implementação da validação, I/O na biblioteca, dependências novas, alteração de `Cargo.toml`/`Cargo.lock`, mudanças em `crates/contratos` ou `crates/sim/src`, factory `b1deab1` (somente fonte), `docs/GDD*`, baselines, `.gitignore`, `STATE.md`, `PROGRESSO.md`, ledger, CI, 193 paths untracked alheios, janelas, credenciais, deploy/remote, builds/gates/commits/push/PRs e reexecução de `./verify.sh` ou do roteiro.
