# Convergence — 023-cliente-greybox

**Data:** 2026-11-21T00:35Z · **Tipo:** append-only

## Phase 1: Convergence

- **Gap 1 — conflito de `thiserror`:** `bevy 0.20.1` → `bevy_shader` → `wesl` → `wgsl-parse` exige `thiserror ^2.0.12`; os crates `contratos`/`sim` fixavam `=2.0.0`. Unificado em pin exato `=2.0.21` (versão atual, `cargo info`, 21/11), sem substituição de pacote e sem range solto.
- **Gap 2 — lints pedantic iniciais:** `manual_assert_eq`, `needless_pass_by_value` e `assigning_clones` corrigidos no código.
- **Gap 3 — `&Res<T>` inválido no Bevy:** a correção do lint quebrou `SystemParam` (E0599); revertido para `Res<T>` por valor com `#[expect(clippy::needless_pass_by_value, reason = "Bevy 0.20 SystemParam exige Res<T> por valor")]` isolado em cada sistema — única exceção permitida pela skill `mdmv-rust-thin`.
- **Gap 4 — formatação:** `cargo fmt --all` aplicado pelo orquestrador.
- **Verificação:** saída do `./verify.sh` colada no R4.

Nenhuma baseline, ADR-004 ou código dos ciclos 021/022 foi alterado. Entrada append-only.
