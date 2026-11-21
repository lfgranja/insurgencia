# Release (R7) — 022-simulacao-rust

**Data:** 2026-11-21T00:14Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-21
### Added
- `crates/sim/` — `insurgencia-sim` com `Mundo` (30 Hz, 3 fases prepare→skills→finish), `Jogador`, `Comando`, `hash_estado()` SHA-256 e `ValidacaoComandos` com 5 códigos de recusa.
- `crates/sim/tests/simulacao.rs` — 8 testes determinísticos.

### Fixed
- Lockfile atualizado com `sha2 0.10.9`; lint pedantic `manual_assert_eq` corrigido; rustfmt aplicado.

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-005 · ADR-004 · skill `mdmv-rust-thin`.

## Rollback
`git revert <merge commit do PR>`; remove `crates/sim/` e restaura `feature.json` para `specs/021-migracao-bevy`. Nenhuma baseline, gate ou trabalho alheio afetado.

## Não incluído
Servidor Bevy, Lightyear, ilhas, funções, tela de resultado, benchmark de rede, deploy/remote.
