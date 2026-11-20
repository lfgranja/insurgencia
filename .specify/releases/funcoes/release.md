# Release (R7) — 015-funcoes

**Data:** 2026-11-20T09:49Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `nucleo/sim/funcoes.gd` — Barreira (20 Materiais/1,5 s/200 PV/12 s/interrupção) e Ataque Forte (1,2 s/2×/6 s) (FR-001/002).
- `tests/test_funcoes.gd` — 18 verificações de custo, canalização, dano, duração, interrupção, recarga, fraude, integração e paridade (FR-006).

### Changed
- `nucleo/mundo.gd` — integração das funções ao tick, preservando todos os preloads e ilhas anteriores.
- `nucleo/sim/validacao.gd` — novas ações aceitas e `RECURSO_FORJADO` recusado.

### Fixed
- Colisão de variável `i` e limite de loop da recarga no teste.

## Rastreabilidade
FR-001..FR-007 · T001..T010 · SC-001..SC-005 · R4 aprovado.

## Rollback
`git revert <merge commit do PR>`; remove o arquivo novo, restaura `mundo.gd`, `validacao.gd` e `feature.json` ao ciclo 014. Nenhuma baseline, gate, ADR ou trabalho alheio afetado.

## Não incluído
Vitória Central, 5v5, UDP real, deploy/remote.
