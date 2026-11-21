# Release (R7) — 028-ci-rust-governanca

**Data:** 2026-11-21T04:29Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-21
### Added
- Job `rust` em `.github/workflows/ci-gates.yml` executando `./verify.sh` (check → clippy → nextest/test → fmt); toolchain lida de `rust-toolchain.toml`; job `gates` (docs) preservado.

### Changed
- `AGENTS.md` reescrito para refletir a realidade Rust/Bevy (5 crates, 34 testes, `verify.sh`, ADR-004, regra da janela) mantendo governança local (Bloco 0.a, SDD, append-only, 80/20, untrusted content).
- `docs/GDD - Insurgência.md:1627` — CI remoto materializado; check `gates SUCCESS` observado em 2026-11-21T02:57Z; job `rust` **[NV]**.

### Fixed
- `verify.sh:2` — cabeçalho corrigido de "casv-rust" para "Insurgência".

## Rastreabilidade
FR-001..FR-008 · T001..T011 · SC-001..SC-005 · C1..C5 (analyze) · constituição art. CI/CD · ADR-004.

## Rollback
`git revert <merge commit do PR>`; restaura `ci-gates.yml`, `AGENTS.md`, `verify.sh:2` e GDD:1627 ao estado anterior. Nenhuma baseline, gate ou trabalho alheio afetado.

## Não incluído
Provider, CLI de PR, deploy, credenciais, assinatura GPG, itens 5-6, rede/5v5, trabalho da factory.
