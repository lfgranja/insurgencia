# Release (R7) — excecao-linguagem-licenca-nucleo

- **Versão**: 1.0.0 (SemVer — versionamento semântico)
- **Data**: 2026-11-19
- **Feature**: `specs/001-excecao-linguagem-licenca-nucleo`
- **R4**: aprovado em `.specify/validations/excecao-linguagem-licenca-nucleo/validate.md` antes deste release
- **Branch**: `feature/001-excecao-linguagem-licenca-nucleo` — sem merge, sem auto-merge, sem force-push
- **Remote/CI/PR/proteções**: `[NV]` até confirmação ao vivo no GitHub via `gh`

## Changelog (1.0.0)

| ID | Mudança | Origem |
|---|---|---|
| FR-001 | Registro local da decisão de exceção de linguagem para GDScript | `spec.md` |
| FR-002–FR-004 | Contexto, alternativas e consequências do ADR-001 | `spec.md`; ADR-001 |
| FR-005–FR-008 | Contexto, decisão MIT-only, alternativas e consequências do ADR-002 | `spec.md`; ADR-002 |
| FR-009–FR-013 | Estrutura comum dos ADRs, aprovação humana, rastreabilidade e fora-do-escopo | `spec.md` |
| T001–T035 | 35 tarefas de materialização e verificação documental concluídas (`[X]`) | `tasks.md` |
| DEV-LING-01, DEV-EXC-01, DEV-LIC-01 | Desvios de intenção 1/3/10 com decisão local correspondente | `docs/AVALIACAO-STACK-MDMV.md` |
| DEC-R1 | Veredito `go` do assess | `.specify/assessments/.../decision.md` |
| BUG-### | Nenhum bug reportado neste ciclo (formato definido para ciclos futuros) | `contracts/adr-contract.md` |

## Entregáveis versionados

- `docs/adr/001-excecao-linguagem-gdscript.md` — estado `Pendente de aprovação`
- `docs/adr/002-licenca-mit-nucleo.md` — estado `Pendente de aprovação`

## Plano de rollback (documental)

1. Não há código, schema, migração ou configuração de runtime neste release; rollback é a remoção
   controlada dos dois ADRs e do registro de release, preservando os artefatos SDD da feature.
2. Reverter a marcação `[X]` das tarefas para `[ ]` **somente** em novo ciclo de correção, nunca
   reescrevendo `converge.md` ou apagando entradas.
3. Manter `.specify/assessments/.../decision.md` e o analyze/remediação como histórico auditável.
4. Se a aprovação humana dos ADRs for recusada, abrir novo ciclo (não emendar este release).

## Restrições

- Nenhum merge, auto-merge, force-push ou deleção de branch executada.
- Nenhuma afirmação de CI/check verde remoto; `[NV]` até confirmação ao vivo.
- Nenhum código, dependência, engine, serviço externo ou emenda da norma externa introduzido.
