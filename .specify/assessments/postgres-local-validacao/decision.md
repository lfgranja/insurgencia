# Decision: Validação local de Postgres MDMV

- **Slug**: postgres-local-validacao
- **Decided**: 2026-11-20
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | intake/problem/research: `docs/AVALIACAO-STACK-MDMV.md:270` registra o padrão `PgPool`/`sqlx`/migrations/JSONB-GIN como intenção, sem execução no alvo; os três arquivos em `tests/postgres/` materializam o padrão mas não tinham resultado versionado. O dano é a ausência de prova de que grants, operador `@>` e migrações funcionam juntos na versão fixada. |
| Evidence strength | strong | research cita cinco blocos com comando, saída e carimbo: `16.15` em 2026-11-20T01:02:01Z (E1), `exit 0` das duas migrações em 01:02:36Z (E2), `INSERT 0 1` + `@>` verdadeiro + 4 grants na janela 01:02:36Z–01:03Z (E3), `ERROR: permission denied` com `exit 1` (E4) e remoção com 0 remanescentes em 01:03Z (E5). Os arquivos executados foram lidos localmente. Nenhum critério depende de `ASSUMPTION` ou de fato remoto não observado. |
| Value vs. inaction | strong | problema: sem a execução local, a adoção do padrão repousa em confiança documental e um grant amplo ou operador incompatível só apareceria no adaptador real; com ela, obtêm-se versão, migrações, smoke, negação e teardown auditáveis por custo de minutos. |
| Feasibility / appetite | strong | concept: Opção A, appetite small; comandos já executados pelo orquestrador e arquivos já versionados, sem código de produto e sem serviço externo no caminho crítico. |
| Strategic fit | strong | constituição I (determinismo), III (fluxo ordenado), IV (sem dependência proprietária no caminho crítico) e VI (verificação honesta); `docs/AVALIACAO-STACK-MDMV.md:268-271` fixa Postgres+`sqlx` com adaptadores trocáveis, e `AGENTS.md` exige comando, saída e data/hora como padrão de evidência. |
| Risk posture | adequate | riscos nomeados em `research.md §Evidence Against the Idea`: credencial real (mitigação: fixture descartável já versionada, nenhum segredo persistido); leitura como validação remota (mitigação: rótulo `[NV]` para Postgres remoto, Coolify e Zitadel); `pgmq` e `pgcrypto` (mitigação: `pgmq` `[NV]`, `pgcrypto` limitado à aceitação da instrução); evidência citada e não reexecutada nesta etapa (mitigação: distinção explícita entre citação e verificação ao vivo); estado residual (mitigação: teardown com 0 remanescentes). |

R1 verificado: `problem.md` presente; `concept.md` presente; validade `strong` (≥adequate); evidência `strong` (nunca `weak`/`unknown`); opção recomendada nomeada (Opção A). Nenhum `unknown` é glossado: a não reexecução nesta etapa, a natureza local da prova e o não teste de `pgmq` são reconhecidos e convertidos em requisitos de rotulagem, não em afirmação de validação remota ou de filas.

## Verdict & Rationale

**go** para a Opção A. O problema é real, circunscrito e sustentado por evidência com comando e carimbo; os cinco blocos de `research.md` são fatos a citar, não resultados a reexecutar nesta etapa. A decisão registra explicitamente que a validação é local e descartável, que a credencial é de fixture, que `pgmq`, Postgres remoto, Coolify e Zitadel permanecem `[NV]`, e que o `exit 0` das migrações não implica teste funcional de `pgcrypto`. A Opção B é preterida porque descreve intenção sem produzir prova; a Opção C ultrapassa a autoridade e introduz segredo fora do alvo. O handoff abaixo limita-se ao que `/speckit.specify` precisa; nenhuma implementação, reexecução ou portaria posterior a tasks é antecipada aqui.

## If needs-clarification

Não aplicável — nenhuma pergunta bloqueante restou. As três ambiguidades de `problem.md` foram resolvidas uma por vez pelo pedido autorizado, pela evidência local e pela política de rotulagem, e registradas em `spec.md` / `## Clarifications` / `### Session 2026-11-20`, respeitando R3 (≤3 esclarecimentos).

## If go — Handoff to `/speckit.specify`

- **Problem**: o padrão Postgres MDMV existe apenas como intenção documental no alvo, sem prova local de que versão, migrações, JSONB/GIN e privilégio mínimo funcionam juntos.
- **Chosen approach**: Opção A — container local descartável a partir de `postgres:16-alpine`, versão lida do servidor (`16.15`), migrações `001_schema.sql`/`002_grants.sql` com `exit 0`, smoke como `insurgencia_app` (`INSERT 0 1`, `@>` verdadeiro, 4 grants), UPDATE negado com `ERROR: permission denied` e `exit 1`, remoção com 0 remanescentes.
- **In scope / out of scope**: dentro — versão observada, aplicação das migrações, objeto de schema, inserção, predicado `@>`, grants, teste negativo, teardown, credencial descartável, evidência com comando/saída/carimbo, dez artefatos rastreáveis; fora — Postgres remoto/Coolify/Zitadel e suas credenciais, segredo real, `pgmq` testado, verificação funcional de `pgcrypto`, `PgPool`/`sqlx`, réplicas, deployment, gates, stage/commit/PR/merge nesta etapa e portarias posteriores a tasks.
- **Success metrics**: 1 versão `16.15` com carimbo; 2 de 2 migrações exit 0; 1 schema com 2 tabelas, 1 coluna `jsonb` e 1 índice GIN; 1 `INSERT 0 1`; 1 `@>` verdadeiro; 4 grants e 0 de UPDATE/DELETE/TRUNCATE; 1 UPDATE negado com exit 1; 1 remoção e 0 remanescentes; 100% de afirmações com origem e carimbo, 0 credenciais reais, 0 afirmações de remoto/`pgmq` validados; 10 de 10 artefatos sem seção vazia nem marcador de pendência.
- **Carried-forward open questions**: nenhuma; as três respostas estão em `spec.md` / `## Clarifications`. Reexecução, validação ao vivo (R4) e release (R7) dependem de fases posteriores e não são afirmadas como concluídas por esta decisão.
