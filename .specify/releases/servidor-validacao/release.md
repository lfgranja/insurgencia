# Release (R7) — 009-servidor-validacao

**Data:** 2026-11-20T03:08Z · **Versão:** `1.0.0` · **R4:** aprovado em 2026-11-20T03:08:32Z

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `nucleo/mundo.gd` — simulação de passo fixo 30 Hz com fases `prepare→skills→finish` e `hash_estado()` determinístico via `HashingContext` (FR-001, FR-007).
- `nucleo/servidor.gd` — `ServidorAutoritativo` com ENet/UDP, `handle_join`, snapshots 20 Hz com acumulador e `handle_command` como ponto único de aplicação (FR-002, FR-004, FR-005, FR-006).
- `nucleo/main_server.gd` — launcher `extends SceneTree` com `--porta=` (FR-003).
- `nucleo/sim/validacao.gd` — 5 códigos de recusa de intenção (FR-005).
- `tests/test_servidor.gd`, `tests/test_loopback.gd`, `tests/test_validacao.gd` — 6 + 8 + 9 verificações (FR-008).

### Fixed
- API `sha256_string()` inexistente no Godot 4.7 substituída por `HashingContext`.
- Acumulador 30→20 Hz com tolerância para produzir exatamente 20 snapshots/1 s.
- Arranque headless usando `Engine.get_main_loop().get_multiplayer()` com guardas.

## Rastreabilidade
FR-001..FR-009 · T001..T010 · SC-001..SC-005 · R4 aprovado.

## Rollback
`git revert <merge commit do PR>`; remove os 5 arquivos novos, restaura `nucleo/servidor.gd`/`nucleo/mundo.gd` ao estado do ciclo 008 e o `feature.json` para `specs/008-nucleo-contrato`. Nenhuma baseline, gate, ADR ou trabalho de outras sessões é afetado.

## Não incluído
Ilha Meio, manifesto, `Makefile`, `evidence.csv`, conexão UDP real entre clientes, deploy/remote.
