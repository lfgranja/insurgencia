# Release (R7) — 012-evidencia-make-check

**Data:** 2026-11-20T03:22Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `Makefile` — alvo agregado `check` executando 5 testes + `check-compat` em sequência, parando no primeiro erro (FR-001).
- `docs/progress/evidence.csv` — 9 colunas literais, 8 linhas: 7 `PASS` com resultado observado e 1 `NAO_VERIFICADO` para UDP real (FR-002/004).
- `docs/progress/2026-11-19-ciclo-80-20.md` — registro datado ligado aos IDs da evidência (FR-003).

### Changed
- Nenhum arquivo de teste foi editado; os 7 scripts existentes foram apenas reexecutados pelo novo alvo.

## Rastreabilidade
FR-001..FR-006 · T001..T010 · SC-001..SC-005 · R4 aprovado.

## Rollback
`git revert <merge commit do PR>`; restaura o `Makefile` da Task 5, remove `docs/progress/` e o `feature.json` volta a `specs/011-compatibilidade-manifesto`. Nenhuma baseline, gate, ADR ou trabalho alheio é afetado.

## Não incluído
Ilha Topo/Inferior, Artesão/Guerreiro, vitória da Ilha Central, 5v5, conexão UDP real, deploy/remote.
