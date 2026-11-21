# Convergence — 021-migracao-bevy

**Data:** 2026-11-20T23:59Z · **Tipo:** append-only

## Phase 1: Convergence

- **Gap 1 — MSRV divergente:** a pesquisa registrou `rust-version=1.100.0` do `main` do Bevy; o crate publicado 0.20.1 declara `1.97.1` (`cargo info`, 20/11). Toolchain local 1.99.1 atende. Registrado na ADR-004 e no spec.
- **Gap 2 — `Cargo.lock` ausente:** `cargo check --locked` falhou até `cargo generate-lockfile`; lock gerado com versões fixadas (serde_json 1.0.140, thiserror 2.0.0).
- **Gap 3 — lint `doc_markdown`:** `GDScript` sem backticks em doc comment; corrigido sem relaxar lints.
- **Gap 4 — zero testes:** `nextest` retornou exit 4 com 0 testes; criado `crates/contratos/tests/protocolo.rs` com 7 testes.
- **Gap 5 — `manual_assert_eq` e `unnecessary_get_then_check`:** 21 asserts corrigidos para `assert_eq!`; `.get().is_none()` trocado por `!contains_key()`.
- **Gap 6 — formatação:** `cargo fmt --all` aplicado uma vez pelo orquestrador; `fmt --check` verde.
- **Verificação:** `./verify.sh` → ALL STAGES OK (check/clippy/nextest/fmt); 7 testes passaram.

Nenhuma baseline de governança, ADR-001 ou código GDScript existente foi apagado; o histórico permanece versionado. Esta entrada é append-only.
