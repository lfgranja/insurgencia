# Release (R7) — 025-soberania-exportacao

**Data:** 2026-11-21T01:22Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-21
### Added
- `crates/exportacao/` — `ContaExportada` com 5 campos, `exportar`/`importar` com JSON determinístico e recusa de versão divergente; 6 testes.
- `COPYRIGHT.txt` — MIT (ADR-002), fonte do próprio repo, data 2026-11-21.
- `scripts/dco-check.sh` — gate que exige trailer `Signed-off-by` no commit.

### Fixed
- Auditabilidade de origem: commits do ciclo passam a conter DCO; assinatura GPG permanece **[NV]** por ausência de chave e autorização.

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-005 · GDD §15.9 (1451/1452/1464/1465) · item 4 do entregável da SE.

## Rollback
`git revert <merge commit do PR>`; remove `crates/exportacao/`, `COPYRIGHT.txt` e `scripts/dco-check.sh`, restaura `feature.json` para `specs/024-instrumentacao-metricas`. Nenhuma baseline, gate ou trabalho alheio afetado.

## Não incluído
Chave GPG, servidores com Módulos diferentes, forks publicados, lobby, 5v5, NAT, deploy/remote.
