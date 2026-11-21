# Release (R7) — 026-cliente-lowpoly

**Data:** 2026-11-21T02:27Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-21
### Added
- Cliente 3D low-poly: `Camera3d`, iluminação, ilhas como cubos/prismas de 4 lados e avatar cone de 6 lados, sem textura.
- `Geometria3d` e `Forma::geometria_3d()` em `lib.rs`; constantes e 5 testes preservados.
- Regra da janela mantida: padrão normal (SE), `--minimizado` para validação de agente.

### Fixed
- Lock do Cargo liberado ao encerrar `cargo-nextest` órfão de verify anterior.

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-005 · GDD §13.2 · item 1 redefinido pela SE.

## Rollback
`git revert <merge commit do PR>`; restaura `crates/cliente` ao estado 2D do ciclo 023 e `feature.json` para `specs/025-soberania-exportacao`. Nenhuma baseline, gate ou trabalho alheio afetado.

## Não incluído
Assets Kenney CC0, texturas, 5v5, rede, deploy, inspeção visual por agente.
