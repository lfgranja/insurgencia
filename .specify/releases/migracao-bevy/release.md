# Release (R7) — 021-migracao-bevy

**Data:** 2026-11-20T23:59Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `Cargo.toml` workspace Rust (resolver 2, `crates/*`) e `Cargo.lock` com versões fixadas.
- `rust-toolchain.toml` fixando 1.99.1 com rustfmt/clippy.
- `crates/contratos/` — `Protocolo` versionado portado: 6 tipos de pacote, 4 códigos de recusa, `SnapshotMeta`, `normalizar_pacote`, `thiserror` para erros.
- `crates/contratos/tests/protocolo.rs` — 7 testes determinísticos.
- `verify.sh` e `clippy.toml` materializados do projeto Rust MDMV `casv-rust` (tree limpa).
- `docs/adr/004-migracao-bevy.md` — aprovação formal da migração com evidência do ciclo 020.

### Fixed
- MSRV corrigido: 1.97.1 no crate publicado, não 1.100.0 do `main`.
- Lints pedantic corrigidos sem relaxar `deny`.

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-005 · ADR-004 · skill `mdmv-rust-thin`.

## Rollback
`git revert <merge commit do PR>`; remove o workspace Rust, ADR-004 e gates Rust, restaura `feature.json` para `specs/020-engines-rust`. ADR-001 volta a ser a decisão vigente de engine; nenhum código GDScript foi apagado do histórico.

## Não incluído
Servidor Bevy, Lightyear, ilhas, funções, vitória, benchmark de rede, deploy/remote.
