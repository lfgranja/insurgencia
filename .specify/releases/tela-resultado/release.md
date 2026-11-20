# Release (R7) — 019-tela-resultado

**Data:** 2026-11-20T11:52Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `nucleo/apresentacao/tela_resultado.gd` — componente `Control` com rótulos programáticos, estado inicial `AGUARDANDO` e `definir_resultado()` que exibe `VITORIA` apenas com `evento="vitoria_ilha_central"` e `declarada_por="servidor"` (FR-001..FR-005).
- `tests/test_tela_resultado.gd` — 19 verificações de vitória válida, recusas, tempo, reaplicação, determinismo e ausência de cena/rede/arquivo (FR-006).

### Fixed
- Estado inicial da tela e chamada inexistente `DirAccess.close()` no teste; divergência documental no `decision.md`.

## Rastreabilidade
FR-001..FR-007 · T001..T010 · SC-001..SC-005 · R4 aprovado · GDD:1539.

## Rollback
`git revert <merge commit do PR>`; remove `nucleo/apresentacao/` e o teste, restaura `feature.json` para `specs/018-iam-adapter-local`. Nenhuma baseline, gate, ADR ou trabalho alheio é afetado.

## Não incluído
Cena principal, `.tscn`, renderização visual inspecionada, rede nova, 5v5 e escala de produção.
