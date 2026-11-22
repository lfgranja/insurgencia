# Release (R7) — emenda-gdd-adrs

- **Versão**: 1.0.0 (SemVer)
- **Data**: 2026-11-19
- **Feature**: `specs/002-emenda-gdd-adrs`
- **R4**: aprovado em `.specify/validations/emenda-gdd-adrs/validate.md` antes deste release
- **Branch**: `feature/002-emenda-gdd-adrs` (derivada de `feature/001-...`; PR dependente do PR #2 — **[NV]** para merge)
- **Remote/CI/PR/proteções**: `[NV]` até confirmação ao vivo via `gh`

## Changelog (1.0.0)

| ID | Mudança | Origem |
|---|---|---|
| FR-001 | Ressalva AGPL restrita a terceiros no GDD | `spec.md`; GDD `1389` |
| FR-002 | MIT para Núcleo e servidor headless; `COPYRIGHT.txt` no lugar de `COPYING` | GDD `1451` |
| FR-003 | Ressalva de licença MIT | GDD `1459` |
| FR-004 | CI dia 1 → gate local + DEFER remoto `[NV]` | GDD `1594`; ADR-001 |
| FR-005–FR-007 | N3 resolvida e removida dos não respondidos | GDD `1659,1668,1673`; ADR-002 |
| T001–T026 | 26 tarefas de emenda e verificação por leitura concluídas (`[X]`) | `tasks.md` |
| DEV-LING-01, DEV-EXC-01, DEV-LIC-01, DEC-R1 | Rastreabilidade preservada e estendida ao GDD | `AVALIACAO-STACK-MDMV.md`; ADRs |
| BUG-### | Nenhum bug reportado neste ciclo (formato definido no contrato) | `contracts/gdd-amendment-contract.md` |

## Entregável versionado

- `docs/GDD - Insurgência.md` — 7 passagens emendadas; âncoras `1265` e `1678` preservadas.

## Plano de rollback (documental)

1. Restaurar cada uma das 7 passagens ao texto anterior, registrando a reversão em novo ciclo — nunca apagar histórico.
2. Reverter a marcação `[X]` de T001–T026 somente em ciclo de correção; `tasks.md` é append-only após converge.
3. Manter analyze/remediação, R4 e este release como histórico auditável.
4. Se a aprovação humana dos ADRs for recusada, abrir novo ciclo; não emendar este release.

## Restrições

- Nenhum merge, auto-merge, force-push ou deleção de branch.
- Nenhuma afirmação de CI/check verde remoto; `[NV]` até confirmação ao vivo.
- Nenhuma emenda da norma MDMV externa, código, engine, gate de build ou serviço introduzido.
