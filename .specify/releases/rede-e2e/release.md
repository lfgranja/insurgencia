# Release (R7) — 017-rede-e2e

**Data:** 2026-11-20T10:04Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- Handlers de rede em `nucleo/servidor.gd`: `peer_packet` → `handle_join`/`handle_command`, respostas `join_accept`/`join_reject` e envelope JSON próprio via `send_bytes` (FR-001/002).
- `tests/test_cliente_rede.gd` — cliente headless ENet com `extends SceneTree`, JOIN/COMANDO e timeout de 5 s (FR-003).
- `tests/run_e2e_rede.sh` — orquestrador de 2 processos na porta 7780 com limpeza e `OK test_e2e_rede` (FR-004).
- Alvo `make test-rede-e2e` no `Makefile` (FR-005).

### Changed
- `docs/progress/evidence.csv` — `EV-20261120-008` (UDP real) passou a `PASS`; `EV-20261120-009` registra NAT externo como **[NV]**.

### Fixed
- `Time.get_ticks_msec()` no lugar de `OS.get_ticks_msec()` (API Godot 3).

## Rastreabilidade
FR-001..FR-007 · T001..T010 · SC-001..SC-005 · R4 aprovado.

## Rollback
`git revert <merge commit do PR>`; restaura `servidor.gd` e `Makefile`, remove os 2 arquivos novos e reverte a evidência ao estado do ciclo 016. Nenhuma baseline, gate, ADR ou trabalho alheio afetado.

## Não incluído
NAT traversal, clientes na Internet pública, lobby/matchmaking, 5v5, escala de produção, deploy/remote.
