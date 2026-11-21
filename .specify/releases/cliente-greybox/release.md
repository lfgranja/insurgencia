# Release (R7) — 023-cliente-greybox

**Data:** 2026-11-21T01:01Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-21
### Added
- `crates/cliente/` — cliente Bevy 0.20.1 jogável: `DefaultPlugins`, `Camera2d`, avatar com input WASD/setas, quatro ilhas geométricas coloridas (Topo/Meio/Inferior/Central), painel de texto de estado.
- Janela inicia **minimizada** (`Window::set_minimized(true)`) para não interromper o usuário.
- `Cargo.lock` com 580 crates fixados; `thiserror = "=2.0.21"` unificado.

### Fixed
- Lints pedantic (`manual_assert_eq`, `needless_pass_by_value`, `assigning_clones`) sem relaxar `deny`.
- `&Res<T>` revertido para `Res<T>` com `#[expect(...)]` isolado — Bevy 0.20 não aceita `&Res<T>` como `SystemParam`.

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-005 · ADR-004 · interjeição da SE (item 1 do entregável).

## Rollback
`git revert <merge commit do PR>`; remove `crates/cliente/`, restaura `feature.json` para `specs/022-simulacao-rust` e os pins de `thiserror` se necessário. Nenhuma baseline de governança, gate ou trabalho alheio afetado.

## Não incluído
Servidor/rede, duas partidas consecutivas, instrumentação/métricas, exportação/COPYRIGHT/DCO, 5v5, lobby, NAT, deploy.
