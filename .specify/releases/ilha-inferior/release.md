# Release (R7) — 014-ilha-inferior

**Data:** 2026-11-20T09:43Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `nucleo/desafios/ponto_coleta.gd` — ciclo 3 s, 5 Materiais, estoque 3 ciclos, recarga 20 s (FR-001/002).
- `tests/test_ilha_inferior.gd` — 16 verificações de crédito, esgotamento, recarga, fraude e paridade (FR-006).
- `ilha_inferior` em `modulos/exemplo/mapa.json` (FR-005).

### Changed
- `nucleo/mundo.gd` — integração do ponto de coleta ao tick, preservando preloads e ilhas anteriores.
- `nucleo/sim/validacao.gd` — `coletar_materiais` aceito e `MATERIAIS_FORJADOS` recusado.

## Rastreabilidade
FR-001..FR-007 · T001..T010 · SC-001..SC-005 · R4 aprovado.

## Rollback
`git revert <merge commit do PR>`; remove o arquivo novo do ponto, restaura `mundo.gd`, `validacao.gd`, `mapa.json` e `feature.json` ao ciclo 013. Nenhuma baseline, gate, ADR ou trabalho alheio afetado.

## Não incluído
Funções, vitória Central, 5v5, UDP real, deploy/remote.
