# Problem Definition: Validação local de Postgres MDMV

- **Slug**: postgres-local-validacao
- **Created**: 2026-11-20
- **Inputs used**: intake.md | research.md | pedido do mantenedor 2026-11-20

## Problem Statement

O alvo registra o padrão Postgres MDMV apenas como intenção documental — `PgPool`/`sqlx`, migrações versionadas, JSONB com índice GIN e privilégio mínimo via papel de aplicação — mas não tem nenhuma prova local de que esse padrão funciona na versão fixada. Sem executar as migrações e o smoke em um ambiente descartável, não há como demonstrar que o operador `@>` responde, que os grants concedem SELECT/INSERT sem conceder UPDATE/DELETE, ou que a remoção do ambiente não deixa estado; uma regressão nesses pontos só apareceria quando o adaptador de persistência real fosse escrito. Fazer essa prova contra Postgres remoto ou com credencial real agravaria o risco, pois as credenciais ficam fora do alvo e o resultado não seria auditável no mesmo grau.

## Affected Users & Stakeholders

- **Users**: mantenedor que vai escrever o adaptador de persistência — precisa de uma referência observada de que JSONB/GIN e os grants funcionam juntos; quem opera o hub localmente — precisa de uma verificação que não dependa de serviço externo nem de segredo real.
- **Stakeholders**: auditores do hub, que exigem comando, saída e data/hora antes de aceitar afirmação como verde; a governança do projeto, cuja constituição exige determinismo (I), fluxo SDD ordenado (III) e verificação honesta com `[NV]` até confirmação ao vivo (VI).

## Goals

- Registrar a versão efetiva do servidor Postgres obtida da imagem `postgres:16-alpine`, lida do servidor em execução e não inferida da tag.
- Registrar a aplicação integral das migrações versionadas `001_schema.sql` e `002_grants.sql` no container, com exit 0.
- Provar o comportamento do smoke autenticado como `insurgencia_app`: inserção bem-sucedida, predicado JSONB `@>` verdadeiro e exatamente os grants previstos.
- Provar o privilégio mínimo por um teste negativo de UPDATE, com erro de permissão e exit 1.
- Registrar o teardown do container com zero remanescentes, garantindo que a validação seja descartável.
- Usar exclusivamente credencial de fixture descartável, mantendo `[NV]` tudo o que depende de Postgres remoto, Coolify, Zitadel e pgmq.
- Entregar o planejamento em dez artefatos rastreáveis, com requisitos funcionais testáveis e critérios de sucesso mensuráveis.

## Non-Goals

- Validar Postgres remoto, Coolify ou Zitadel, inclusive suas credenciais; todos permanecem `[NV]`.
- Usar segredo real, token ou credencial de produção em qualquer artefato, log ou mensagem.
- Afirmar que `pgmq` foi testado ou que a extensão foi verificada; `pgmq` permanece `[NV]`.
- Afirmar funcionamento funcional de `pgcrypto` além da aceitação da instrução `CREATE EXTENSION` pela migração.
- Validar conexão via `PgPool`/`sqlx`, réplicas, volumes persistentes ou comportamento de deployment.
- Alterar código de produto, esquema versionado ou arquivos de teste em `tests/postgres/`.
- Executar gates, stage, commit, abertura de PR, merge ou reescrita de histórico nesta etapa de planejamento.
- Escrever portarias posteriores a tasks (implement, converge, validate, release) neste conjunto de artefatos.

## Success Metrics

- Versão do servidor: 1 registro de `16.15` lido de `SHOW server_version;` em 2026-11-20T01:02:01Z; 0 versões inferidas apenas da tag `postgres:16-alpine`. (baseline: nenhuma execução; meta 1)
- Migrações: 2 de 2 arquivos (`001_schema.sql`, `002_grants.sql`) aplicados com `exit 0` em 2026-11-20T01:02:36Z; 0 migrações com erro. (baseline: nenhuma aplicação registrada; meta 2/2, 0 erros)
- Objeto de schema: 1 schema com 2 tabelas (`module_manifest`, `evidence`), 1 coluna `jsonb` (`payload`) e 1 índice GIN (`idx_module_manifest_payload_gin`), conforme `tests/postgres/001_schema.sql`; 0 divergências. (baseline: arquivos versionados sem resultado; meta 1, 0 divergências)
- Inserção: 1 saída `INSERT 0 1` registrada para a inserção como `insurgencia_app` em `module_manifest`. (baseline: nenhuma; meta 1)
- JSONB: 1 predicado `payload @> '{"pilares":["soberania"]}'::jsonb` registrado como verdadeiro (`tem_pilares = t`). (baseline: nenhum; meta 1)
- Privilégios: 4 grants de tabela registrados para `insurgencia_app` (SELECT/INSERT em `module_manifest` e `evidence`); 0 grants de UPDATE/DELETE/TRUNCATE observados. (baseline: nenhuma consulta registrada; meta 4, 0)
- Teste negativo: 1 UPDATE como `insurgencia_app` registrado com `ERROR: permission denied` e `exit 1`; 0 UPDATEs bem-sucedidos. (baseline: nenhum teste; meta 1, 0)
- Teardown: 1 remoção de container registrada em 2026-11-20T01:03Z com 0 remanescentes. (baseline: nenhum container; meta 1, 0)
- Isolamento: 100% das afirmações com origem e data/hora; 0 credenciais reais; 0 afirmações de Postgres remoto/Coolify/Zitadel validados; 0 afirmações de pgmq testado — todos `[NV]`. (baseline: padrão documental sem prova; meta 100%, 0, 0, 0)
- Artefatos: 10 de 10 presentes em `specs/007-postgres-local-validacao/`, sem seção vazia e sem marcador de pendência, com checklist `[ ]` e tarefas `T###` sem portarias posteriores. (baseline: 1 template de spec; meta 10 de 10)

## Cost of Inaction

Sem a prova local, o padrão Postgres MDMV continuaria sendo uma afirmação de intenção em documentos de arquitetura, e qualquer erro de sintaxe nas migrações, operador JSONB incompatível ou grant amplo demoraria a ser descoberto — possivelmente já no adaptador de persistência real. Sem o teste negativo, um papel de aplicação com UPDATE não seria distinguido de um com privilégio mínimo. Sem a credencial descartável e o teardown, a verificação poderia deixar segredo ou estado no alvo. O custo é a perda de auditabilidade e de confiança no padrão, não apenas a ausência de um relatório de execução.

## Open Questions

- Pergunta 1 — como rotular o escopo para não induzir leitura de que Postgres remoto, Coolify ou Zitadel foram validados? Respondida em `spec.md` / `## Clarifications` / Q1: validação estritamente local e descartável, com credencial de fixture; remoto permanece `[NV]`.
- Pergunta 2 — como tratar pgmq e a extensão citada pela migração? Respondida em `spec.md` / `## Clarifications` / Q2: `pgcrypto` teve sua instrução aceita com `exit 0`, sem teste funcional; `pgmq` não foi testado e fica `[NV]`.
- Pergunta 3 — como tratar a evidência já obtida pelo orquestrador nesta etapa? Respondida em `spec.md` / `## Clarifications` / Q3: como fatos citados com comando, saída e carimbo, sem reexecução; implementação e portarias R4/R7 pertencem a fases posteriores, ausentes destes artefatos.
