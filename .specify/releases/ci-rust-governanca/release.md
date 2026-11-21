# Release (R7) — 028-ci-rust-governanca

**Data:** 2026-11-21T04:29Z (revisões 04:35Z/04:42Z/04:51Z) · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-21
### Added
- Job `rust` em `.github/workflows/ci-gates.yml` executando `./verify.sh` (check → clippy → nextest/test → fmt); job `gates` (docs) preservado.

### Changed
- `AGENTS.md` reescrito com stack Rust/Bevy, 34 testes, `verify.sh`, ADR-004 e regra da janela reforçada pela SE (agente sempre `--minimizado`; nunca janela normal durante trabalho de agente).
- `docs/GDD - Insurgência.md:1627` — CI remoto materializado; check `gates SUCCESS` observado 2026-11-21T02:57Z.

### Fixed
- `verify.sh:2` — cabeçalho de "casv-rust" para "Insurgência".
- `toolchain: '1.99.1'` explícito no job `rust` — CI RED 04:29Z (action exige input).
- Step `Install system libraries` — CI RED 04:37Z (`wayland-sys` sem `libwayland.pc`): pkg-config, libwayland-dev, libxkbcommon-dev, libx11-dev, libudev-dev.
- Step ampliado — CI RED 04:45Z (`alsa-sys` sem `alsa.pc`): libasound2-dev, libxrandr-dev, libxinerama-dev, libxcursor-dev, libxi-dev, libgl1-mesa-dev. Conjunto completo Bevy no Ubuntu.

## Rastreabilidade
FR-001..FR-008 · T001..T011 · SC-001..SC-005 · C1..C8 (analyze) · constituição art. CI/CD · ADR-004.

## Rollback
`git revert <merge commit do PR>`; restaura `ci-gates.yml`, `AGENTS.md`, `verify.sh:2` e GDD:1627. Nenhuma baseline ou trabalho alheio afetado.

## Não incluído
Provider, CLI, deploy, credenciais, GPG, itens 5-6, rede/5v5, trabalho da factory, janela normal por iniciativa de agente.
Status do job `rust`: **SUCCESS** observado ao vivo em 2026-11-21T04:56:22Z (run `44868073016`), ambos os checks verdes.
