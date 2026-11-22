# Release (R7) — 013-ilha-topo

**Data:** 2026-11-20T09:37Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `nucleo/desafios/ilha_topo.gd` — 3 alavancas em ordem fixa, 8 s por alavanca, reinício em 2 s, limite 45 s, evento `ilha_topo_concluida` (FR-001).
- `tests/test_ilha_topo.gd` — 16 verificações de ordem, tempo, reinício, janela, validação e paridade (FR-006).
- `ilha_topo` em `modulos/exemplo/mapa.json` (FR-005).

### Changed
- `nucleo/mundo.gd` — integração da Topo ao tick e ao estado persistido, preservando Ilha Meio e `hash_estado`.
- `nucleo/sim/validacao.gd` — aceitação de `sinalizar_alavanca` com `ALAVANCA_INVALIDA`.

### Fixed
- `preload` de `IlhaSincronia` restaurado; inferência de tipo no teste corrigida.

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-005 · R4 aprovado.

## Rollback
`git revert <merge commit do PR>`; remove os 2 arquivos novos e restaura `mundo.gd`, `validacao.gd`, `mapa.json` e `feature.json` ao estado do ciclo 012. Nenhuma baseline, gate, ADR ou trabalho alheio é afetado.

## Não incluído
Ilha Inferior, funções, vitória da Ilha Central, 5v5, conexão UDP real, deploy/remote.
