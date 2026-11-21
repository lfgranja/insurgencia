# Release (R7) — 033-manifesto-compatibilidade-rust

**Data:** 2026-11-21 · **Versão:** `1.0.0` (SemVer) · **R4:** aprovado

## Changelog
### [1.0.0] — 2026-11-21
### Added
- FR-001 / T005 — âncora registrada: os 6 campos e valores de `manifesto.json`, `specs/011` FR-001..FR-006 e o GDScript de referência (GDD §1/§14.2); nenhum esquema novo.
- FR-002 / T006 — lacuna Rust registrada: constantes de recusa presentes em `lib.rs`, validador e `REGRAS_AUSENTES` ausentes; porte em `crates/contratos/src/manifesto.rs` com `ManifestoCompatibilidade`, `ErroCompatibilidade`, `codigo()` e `pub mod manifesto;` adicionado apenas.
- FR-003 / T007 — `validar_formato` exige 6 campos/tipos → `MANIFEST_INVALID`, sem checar tamanho do hash nessa etapa.
- FR-004 / T007 — `conferir` na ordem 011: formato → versão → módulo → 64 hex → regras → hash, com `Option<&str>` e `eq_ignore_ascii_case`, sem I/O nem pânico.
- FR-005 / T007 — higiene Rust (`forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`, `thiserror`, `BTreeMap`, sem dependências novas); `Cargo.toml` intacto.
- FR-006 / T007, T009, T010 — 7 testes Rust espelhando `tests/test_manifesto.gd`; SDD ordenado R1 go → R4 → R7, `converge.md` append-only (1ª entrada 2026-11-21), `analyze.md` READ-ONLY, fronteiras A (13 artefatos) e B (3 arquivos) disjuntas.
- T008 — evidências: 5 gates PASS em `2026-11-21T21:24:08Z`; `./verify.sh` **[NV]** por contenção de `cargo`; factory `b1deab1` somente fonte.
- T011 — release `1.0.0` com este changelog e rollback documentado; nenhum build/gate/commit/PR.

## Rastreabilidade
FR-001..FR-006 · T001..T011 · SC-001..SC-006 · R1 go (`.specify/assessments/manifesto-compatibilidade-rust/decision.md`) · R4 `.specify/validations/manifesto-compatibilidade-rust/validate.md`. Âncora: `specs/011-compatibilidade-manifesto/spec.md`, `manifesto.json`, `nucleo/compatibilidade/manifesto.gd`, `tests/test_manifesto.gd`.

## Rollback

Para reverter o ciclo 033 ao estado anterior, remover `crates/contratos/src/manifesto.rs`, `crates/contratos/tests/manifesto.rs` e a linha `pub mod manifesto;` de `crates/contratos/src/lib.rs` (restaurando o doc do módulo), além dos 13 artefatos SDD criados nas pastas `.specify/assessments/manifesto-compatibilidade-rust/`, `specs/033-manifesto-compatibilidade-rust/`, `.specify/validations/manifesto-compatibilidade-rust/` e `.specify/releases/manifesto-compatibilidade-rust/`. **Nunca executar neste ciclo — apenas documentar.** As constantes e funções existentes em `lib.rs`, `Cargo.toml`, `manifesto.json`, `specs/011`, GDScript, factory `b1deab1`, `docs/GDD*`, baselines, gates/CI e 194 paths untracked alheios não são afetados. Nenhum commit, build ou gate foi executado; `./verify.sh` segue **[NV]**.

## Não incluído
Segundo esquema de manifesto, I/O ou cálculo de hash na validação, dependências novas, alteração de `Cargo.toml`, factory `b1deab1` (somente fonte), `docs/GDD*`, baselines/gates/CI, 194 paths untracked alheios, janelas do cliente, credenciais, deploy/remote, builds/gates/commits/PRs e reexecução de `./verify.sh`.
