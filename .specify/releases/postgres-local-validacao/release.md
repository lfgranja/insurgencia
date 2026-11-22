# Release (R7) — postgres-local-validacao

- **Versão**: 1.0.0 (SemVer)
- **Data**: 2026-11-20
- **Feature**: `specs/007-postgres-local-validacao` — `feature/007-postgres-local-validacao`
- **R4**: registrado em `.specify/validations/postgres-local-validacao/validate.md` antes deste release
- **Branch**: `feature/007-postgres-local-validacao` — sem merge, sem auto-merge, sem force-push e sem deleção de branch
- **Remote/CI/PR/proteção**: `[NV]` até confirmação ao vivo via `gh`; publicação/deploy `[NV]`

## Changelog (1.0.0)

| ID | Mudança | Origem |
|---|---|---|
| FR-001 | Versão `16.15` registrada como lida de `SHOW server_version;` no container `postgres:16-alpine`, com origem e carimbo 2026-11-20T01:02:01Z; 0 versões inferidas da tag | `spec.md` §FR-001; E1 em `research.md`; T005, T010 |
| FR-002 | Aplicação de `001_schema.sql` e `002_grants.sql` registrada com `exit 0` sob `ON_ERROR_STOP=1` em 2026-11-20T01:02:36Z; erro como falha explícita | `spec.md` §FR-002; E2; T006, T011, T012 |
| FR-003 | Schema registrado com 2 tabelas (`module_manifest`, `evidence`), coluna `payload jsonb` e índice GIN `idx_module_manifest_payload_gin`; sem afirmação de desempenho | `spec.md` §FR-003; E2; T007, T016 |
| FR-004 | Smoke autenticado como `insurgencia_app` com saída `INSERT 0 1` em `module_manifest` | `spec.md` §FR-004; E3; T014 |
| FR-005 | Predicado `payload @> '{"pilares":["soberania"]}'::jsonb` registrado como verdadeiro, com `tem_pilares = t`, sem ajuste do dado de teste | `spec.md` §FR-005; E3; T015, T017 |
| FR-006 | Exatamente 4 grants de tabela para `insurgencia_app` (SELECT/INSERT em `module_manifest` e `evidence`) | `spec.md` §FR-006; E3; T019 |
| FR-007 | Nenhum privilégio de UPDATE, DELETE ou TRUNCATE concedido ao papel de aplicação nas duas tabelas | `spec.md` §FR-007; E3; T020, T023 |
| FR-008 | Teste negativo registrado: UPDATE como `insurgencia_app` → `ERROR: permission denied for table module_manifest` e `exit 1`; UPDATE permitido como falha | `spec.md` §FR-008; E4; T021, T022 |
| FR-009 | Remoção do container `insurgencia_pg_local_validacao` registrada em 2026-11-20T01:03Z com 0 remanescentes | `spec.md` §FR-009; E5; T024 |
| FR-010 | Somente ambiente local descartável e credencial de fixture (`throwaway-local-only`); nenhuma credencial real, token ou segredo usado ou persistido | `spec.md` §FR-010; E1/E2; T008, T025 |
| FR-011 | `pgmq` não afirmado como testado nem extensão verificada; `CREATE EXTENSION IF NOT EXISTS pgcrypto` registrado apenas como aceito com `exit 0`, sem teste funcional | `spec.md` §FR-011; E2; T009, T026 |
| FR-012 | Postgres remoto, Coolify e Zitadel, inclusive credenciais, permanecem `[NV]` | `spec.md` §FR-012; T009, T026 |
| FR-013 | Toda afirmação com origem (comando executado ou evidência citada) e data/hora; evidência do orquestrador citada, não apresentada como reexecutada | `spec.md` §FR-013; T003, T029 |
| FR-014 | Dez artefatos presentes em `specs/007-postgres-local-validacao/`, sem seção vazia nem marcador de pendência, com checklist `[ ]` e tarefas `T###` rastreáveis a FRs e SCs | `spec.md` §FR-014; T001–T004, T027 |
| FR-015 | Nenhum gate, stage, commit, PR, merge ou efeito remoto; nenhuma seção de implementação, converge, validação ou release nos artefatos de planejamento | `spec.md` §FR-015; T028, T032 |
| T001–T032 | 32 tarefas de setup, fixação de evidências, US1–US4, teardown, rotulagem `[NV]` e acabamento concluídas (`[X]`), IDs preservados | `specs/007-postgres-local-validacao/tasks.md` |
| SC-001–SC-010 | Conformidade registrada no R4: versão, 2/2 migrações, schema, `INSERT 0 1`, `@>` verdadeiro, 4 grants, teste negativo, teardown com 0 remanescentes, 100% com origem/carimbo e 10/10 artefatos; dependências remotas/`pgmq` mantidas `[NV]` | `.specify/validations/postgres-local-validacao/validate.md` |
| CHK001–CHK020, CHK001–CHK023 | Rastreabilidade FR↔SC↔CHK nos checklists `requirements.md` (20 itens) e `postgres-quality.md` (23 itens) | `specs/007-postgres-local-validacao/checklists/` |
| DEC-R1 | Veredito `go` da Opção A — validação local descartável com versão lida do servidor, migrações, smoke, teste negativo e teardown; evidência `strong`, nenhum `unknown` glossado | `.specify/assessments/postgres-local-validacao/decision.md` |
| BUG-### | Nenhum bug reportado neste ciclo | — (nenhum registro `BUG-###` no ciclo) |

## Entregáveis versionados

- `.specify/assessments/postgres-local-validacao/` — assess canônico: cópia integral de `intake.md`, `research.md`, `problem.md`, `concept.md` e `decision.md` (o gate deriva o slug `postgres-local-validacao` de `007-postgres-local-validacao`).
- `.specify/validations/postgres-local-validacao/validate.md` — R4 com `## Evidência final` para o orquestrador colar comandos, saídas e data/hora.
- `.specify/releases/postgres-local-validacao/release.md` — este documento (R7).
- `specs/007-postgres-local-validacao/tasks.md` — 32 de 32 tarefas marcadas `[X]`, IDs T001–T032 preservados.
- Evidência citada E1–E5 (`research.md`, `plan.md`, `spec.md`) sobre as migrações de teste pré-existentes `tests/postgres/001_schema.sql`, `tests/postgres/002_grants.sql` e `tests/postgres/003_smoke.sql`; nenhum desses arquivos é modificado por esta feature.

## Plano de rollback (documental)

Procedimento aplicável à documentação introduzida: as portarias derivadas do slug, as tarefas marcadas e as migrações de teste citadas pela validação. A feature não produz engine, API, serviço, migração de produto, banco persistente, binário, configuração de CI, segredo ou efeito remoto; o container local já foi removido.

1. Registrar o achado com IDs FR, `T###` e, quando existirem, `BUG-###` (gatilhos: versão inferida da tag, migração sem `exit 0`, smoke ou `@>` falso registrado como sucesso, grant amplo, UPDATE permitido não registrado como falha, credencial real, afirmação de Postgres remoto/Coolify/Zitadel/`pgmq` validados, teste de desempenho ou publicação/deploy afirmados, portaria posterior a tasks).
2. **Portarias**: para reverter, remover `.specify/assessments/postgres-local-validacao/`, `.specify/validations/postgres-local-validacao/validate.md` e `.specify/releases/postgres-local-validacao/release.md` em novo ciclo documentado, com nova versão SemVer e data — nunca apagar ou reescrever histórico.
3. **Migrações de teste**: para reverter o padrão validado, remover `tests/postgres/001_schema.sql`, `tests/postgres/002_grants.sql` e `tests/postgres/003_smoke.sql` em ciclo documentado, registrando que eram fixtures pré-existentes citadas por E2/E3 e que a remoção não apaga o registro histórico da validação.
4. **Container**: `insurgencia_pg_local_validacao` já foi removido em 2026-11-20T01:03Z conforme E5, com contagem 0 de remanescentes. Nenhuma ação de teardown está pendente; não reexecutar subida ou remoção como portaria — qualquer nova execução é ato separado e autorizado e permanece `[NV]` até saída colada e data/hora.
5. **Tarefas**: no ciclo de reversão documentada, devolver T001–T032 de `[X]` para `[ ]`, preservando os IDs e a terminação em tasks; nenhuma seção de implementação, converge, validação ou release é introduzida nos artefatos de planejamento.
6. Reconfirmar 0 gates, 0 stage/commit/PR/merge, 0 credenciais reais e 0 afirmações de Postgres remoto/Coolify/Zitadel/`pgmq` validados ou de publicação/deploy; reexecutar a auditoria documental por contagens e conferir as saídas coladas no R4; registrar a reversão no changelog com versão, data e IDs FR/`T###`/`BUG-###`.

**Proibido**: apagar histórico, `force-push`, merge ou auto-merge, reescrever governança, afirmar Postgres remoto/Coolify/Zitadel/`pgmq` validados, teste de desempenho realizado ou CI/merge/publicação/deploy obtidos sem comando executado, saída colada e data/hora.

## Restrições

- Nenhum merge, auto-merge, force-push ou deleção de branch executada.
- Nenhuma afirmação de CI/check verde remoto ou de proteção aplicada; `[NV]` até confirmação ao vivo. Publicação/deploy, Postgres remoto, Coolify, Zitadel e `pgmq` permanecem `[NV]`.
- Nenhuma portaria posterior a tasks (implement, converge, validate, release) executada como gate, stage ou commit por esta feature.
- Nenhum código, dependência, engine, serviço externo, segredo ou emenda da norma externa introduzido; nenhum teste de performance reivindicado.
- A validação registrada é estritamente local e descartável, com credencial de fixture; nada neste release a converte em prova de ambiente remoto ou persistente.
