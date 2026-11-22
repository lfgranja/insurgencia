# Release (R7) — 016-vitoria-central

**Data:** 2026-11-20T10:00Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `nucleo/desafios/ilha_central.gd` — pré-requisito de 3 ilhas, 4 s de canalização, escalação fixa de Artesão/Guerreiro vivos, interrupção que zera o progresso e vitória declarada pelo servidor (FR-001/002).
- `tests/test_vitoria_central.gd` — 18 verificações de recusas, identidade, tempo, interrupção, evento, integração e paridade (FR-006).

### Changed
- `nucleo/mundo.gd` — integração da Ilha Central ao tick e ao `hash_estado`, preservando todos os preloads e ilhas anteriores.
- `modulos/base/regras.cfg` — comentário registrando que a regra de produção permanece 6 ilhas/20 s.

## Rastreabilidade
FR-001..FR-007 · T001..T010 · SC-001..SC-005 · R4 aprovado.

## Rollback
`git revert <merge commit do PR>`; remove o arquivo novo, restaura `mundo.gd`, `regras.cfg` e `feature.json` ao ciclo 015. Nenhuma baseline, gate, ADR ou trabalho alheio afetado.

## Não incluído
Cena de UI de resultado, 5v5, UDP real, deploy/remote.
