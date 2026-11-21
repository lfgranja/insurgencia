# Convergência — 027-lowpoly-real

**Data**: 2026-11-21T02:54Z · **Modo**: READ-ONLY, append-only.

## Evidências
- `verify.sh`: ALL STAGES OK, 34/34 testes, stage 4 fmt limpo.
- `cargo build -p insurgencia-cliente`: exit 0.
- Runtime minizado: GPU Vulkan RADV RAVEN, 0 avisos B0004, 0 erros.
- Janela normal aberta para inspeção da SE (PID 696504) — inspeção visual **[NV]** até confirmação.
- Correções aplicadas: pattern de array `cor`, precisão do hash, `Transform::IDENTITY` nos 3 spawns sem transform, `Visibility::default()` no pai do avatar (B0004 tornava os filhos invisíveis conforme doc oficial Bevy).

## Atendimento
- FR-001..FR-006: atendidos (terreno irregular 320 triângulos, ilhas 32 triângulos, 3 árvores + 2 rochas por ilha, avatar com base/corpo/cabeça, sem texturas/assets).
- FR-007: `lib.rs` intacto, 5 testes preservados.
- FR-008: gates verdes.
- SC-001: **[NV]** — aguardando SE. SC-002..SC-005: atendidos.
