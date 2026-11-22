# Convergence — 025-soberania-exportacao

**Data:** 2026-11-21T01:22Z · **Tipo:** append-only

## Phase 1: Convergence

- **Gap 1 — GPG ausente:** verificado `gpg --list-secret-keys` vazio e `commit.gpgsign`/`user.signingkey` não configurados. Substituído por DCO (`Signed-off-by`) no commit, assinatura de origem verificável sem chave criptográfica; assinatura GPG permanece **[NV]** e exige autorização da SE.
- **Gap 2 — lockfile:** `insurgencia-exportacao` incluído via `cargo generate-lockfile`.
- **Gap 3 — formatação:** `cargo fmt --all` aplicado pelo orquestrador.
- **Verificação:** `verify.sh` ALL STAGES OK, 34 testes; `scripts/dco-check.sh` verde no commit deste ciclo (saída no R4).

Nenhuma baseline de governança, ADR ou trabalho alheio alterado. Entrada append-only.
