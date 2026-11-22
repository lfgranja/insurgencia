# Release (R7) — 024-instrumentacao-metricas

**Data:** 2026-11-21T01:12Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-21
### Added
- `crates/metricas/` — `EventoNucleo`, `RegistroPartida`, `Relatorio` e `correr_partidas(n, cliente_modificado)`: 20 partidas determinísticas, 5 métricas da GDD §17 e duas partidas consecutivas com cliente modificado.
- `docs/evidencias/metricas-20-partidas-2026-11-21.md` — relatório datado com mediana/p95, limites `[HIPÓTESE]` e ressalvas.
- 8 testes no corredor de métricas.

### Fixed
- Sombreamento de função no teste, `manual_is_multiple_of` e `struct_excessive_bools` (4 enums de dois estados), sem relaxar lints.

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-005 · GDD:1567 e 1605-1622 · itens 2 e 3 do entregável da SE.

## Rollback
`git revert <merge commit do PR>`; remove `crates/metricas/` e o relatório, restaura `feature.json` para `specs/023-cliente-greybox`. Nenhuma baseline, gate ou trabalho alheio afetado.

## Não incluído
Exportação de conta/personagem, COPYRIGHT.txt, commits assinados, servidores com Módulos diferentes, forks publicados, partidas humanas, lando/5v5, deploy.
