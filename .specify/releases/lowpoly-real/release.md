# Release (R7) — 027-lowpoly-real

**Data:** 2026-11-21T02:56Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-21
### Added
- Terreno irregular low-poly: malha não indexada 16×10 (320 triângulos), alturas determinísticas, 3 manchas verde-oliva.
- Ilhas com 32 triângulos cada: contorno de 8 vértices perturbados, topo em leque e talude com base mais larga; 4 cores preservadas.
- Cenário low-poly: 3 árvores (tronco pentagonal + copa hexagonal) e 2 rochas (cone pentagonal achatado) por ilha.
- Avatar corporificado: base cilíndrica, corpo cônico e cabeça icosaédrica (6 lados).

### Fixed
- E0308: pattern de array para `cor: [f32; 3]`.
- Clippy: precisão/agrupamento da constante do hash.
- B0004: `Transform::IDENTITY` nos spawns sem transform (terreno/topo/taludo) e `Visibility::default()` no pai do avatar — sem o último, os filhos ficavam invisíveis conforme doc oficial Bevy.

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-005 · GDD §13.2 / :1045 · correção do report "continua greybox".

## Rollback
`git revert <merge commit do PR>`; restaura `crates/cliente/src/main.rs` ao estado primitivo do ciclo 026. `lib.rs` intacto; nenhuma baseline ou trabalho alheio afetado.

## Não incluído
Assets Kenney CC0 (host fora da allowlist), texturas, 5v5, rede, deploy, inspeção visual por agente **[NV]**.
