# Release (R7) — 010-ilha-sincronia

**Data:** 2026-11-20T03:13Z · **Versão:** `1.0.0` · **R4:** aprovado em 2026-11-20T03:13Z

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `nucleo/desafios/ilha_sincronia.gd` — regra da Ilha Meio: janela 5 s, 2 placas, gás 8 por amostra, expulsão em 3 s, `estado_persistido = "ilha_concluida"` e `reiniciar()` (FR-001).
- `modulos/exemplo/mapa.json` — mapa declarativo com `ilha_meio` e placas 1/2 (FR-002).
- `modulos/base/regras.cfg` — regra de produção com `required_players = 3`, fora do Módulo de exemplo (FR-003).
- `tests/test_ilha_sincronia.gd` — 14 verificações (FR-007).
- `tests/test_loopback_ilha.gd` — evento entre clientes com `DELTA_EVENTO_MS` e comparação base/exemplo (FR-007).

### Changed
- `nucleo/mundo.gd` — integração de sinalização, gás, expulsão e `estado_persistido` ao tick (FR-004).
- `nucleo/sim/validacao.gd` — validação de placa 1 ou 2 com `PLACA_INVALIDA` (FR-005).

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-004 · R4 aprovado.

## Rollback
`git revert <merge commit do PR>`; remove os 4 arquivos novos, restaura `nucleo/mundo.gd` e `nucleo/sim/validacao.gd` ao estado do ciclo 009 e `feature.json` para `specs/009-servidor-validacao`. Nenhuma baseline, gate, ADR ou trabalho alheio é afetado.

## Não incluído
Manifesto, `Makefile`, `evidence.csv`, 5v5, vitória da Ilha Central e conexão UDP real **[NV]**.
