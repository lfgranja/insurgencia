# Convergence — 026-cliente-lowpoly

**Data:** 2026-11-21T02:18Z · **Tipo:** append-only

## Phase 1: Convergence

- **Gap 1 — cliente 2D:** pedido da SE exige low-poly 3D; `main.rs` migrou de `Camera2d`/`Sprite` para `Camera3d`, iluminação e malhas procedurais, preservando `lib.rs` puro e os 5 testes.
- **Gap 2 — API Bevy 0.20:** conversão `(x,y) → (x, altura, -y)` e prismas de 4 lados validados por `cargo check`.
- **Gap 3 — build travado:** `cargo-nextest` órfão de verify anterior segurava lock do Cargo; encerrado pelo orquestrador, sem afetar trabalho alheio; gate rerodado verde.
- **Gap 4 — formatação:** `cargo fmt --all` aplicado.
- **Verificação:** `verify.sh` ALL STAGES OK, 34 testes; saída do binário minizado colada no R4.

Nenhuma baseline, ADR-004 ou trabalho dos ciclos anteriores alterado. Entrada append-only.
