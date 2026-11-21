# Convergence — 022-simulacao-rust

**Data:** 2026-11-21T00:14Z · **Tipo:** append-only

## Phase 1: Convergence

- **Gap 1 — lockfile:** `sha2 0.10.9`, `insurgencia-sim` e transitives ausentes do `Cargo.lock`; gerado com `cargo generate-lockfile` (versões fixadas).
- **Gap 2 — lint pedantic:** `assert!(diferente != primeiro)` trocado por `assert_ne!` sem relaxar lints.
- **Gap 3 — formatação:** `cargo fmt --all` aplicado pelo orquestrador ao final da fase.
- **Verificação:** `./verify.sh` → ALL STAGES OK; 15 testes nextest (7 contratos + 8 sim), 0 falhas.

Nenhuma baseline, ADR-004 ou código GDScript histórico foi alterado. Entrada append-only.
