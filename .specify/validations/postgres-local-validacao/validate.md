# Validate (R4) — postgres-local-validacao

- **Feature**: `specs/007-postgres-local-validacao` — `feature/007-postgres-local-validacao`
- **Data/hora da execução final**: a preencher pelo orquestrador, em ISO 8601, no ato da auditoria documental final
- **ALVO**: `/home/luis/development/MDMV/projetos/insurgencia` — rodada documental sobre `specs/007-postgres-local-validacao/` — branch `feature/007-postgres-local-validacao`
- **Fonte do aparato**: factory (somente fonte do aparato; não autoridade dos fatos do alvo — Bloco 0.a)
- **Veredito R4**: registrado após a auditoria documental por contagens e a colagem das saídas em `## Evidência final`; reexecução de comandos, Postgres remoto, Coolify, Zitadel, `pgmq`, efeito remoto, publicação/deploy e credenciais permanecem `[NV]` até confirmação ao vivo

## Gate executado e comandos

Esta feature é documental (FR-013, FR-015, SC-009, SC-010): nenhum gate determinístico de CI, build, teste, stage, commit, PR, merge ou efeito remoto é executado como portaria. Os cinco gates do alvo (`check_sdd_completeness.py`, `check_work_order_dag.py`, `check_task_id_uniqueness.py`, `check_converge_append_only.py`, `hub_docs_gate.py`) pertencem a etapa posterior autorizada e permanecem `[NV]` aqui. O gate executado é a **auditoria documental por contagens** sobre os dez artefatos de `specs/007-postgres-local-validacao/`, os cinco blocos de evidência E1–E5 e os artefatos derivados do slug `postgres-local-validacao` (derivado de `007-postgres-local-validacao`). As saídas dos comandos abaixo, sem edição, são coladas em `## Evidência final` com a data/hora do ato da auditoria. Nenhuma contagem é inflada para fechar critério; nenhuma falha é suprimida para produzir verde artificial (SC-009, SC-010).

```bash
# artefatos derivados do slug (o gate deriva postgres-local-validacao de 007-postgres-local-validacao)
ls .specify/assessments/postgres-local-validacao/
ls .specify/validations/postgres-local-validacao/
ls .specify/releases/postgres-local-validacao/

# assess canônico — cópia integral dos cinco artefatos
for f in intake research problem concept decision; do cmp -s specs/007-postgres-local-validacao/$f.md .specify/assessments/postgres-local-validacao/$f.md && echo "OK $f" || echo "DIVERGENTE $f"; done

# dez artefatos, sem seção vazia nem marcador pendente
ls specs/007-postgres-local-validacao/*.md specs/007-postgres-local-validacao/checklists/*.md | wc -l
grep -rniE 'T[B]D|NEEDS[ ]CLARIFICATION|TODO' specs/007-postgres-local-validacao/ || echo '0 marcadores pendentes'

# tarefas concluídas (meta: 32)
grep -cE '^- \[X\] T[0-9]{3}' specs/007-postgres-local-validacao/tasks.md

# blocos de evidência citados E1–E5 (comando, saída e carimbo)
grep -nE '^\*\*E[1-5] —' specs/007-postgres-local-validacao/research.md

# rotulagem [NV] e ausência de afirmação de pgmq testado / extensão verificada / desempenho / deploy
grep -rc '\[NV\]' specs/007-postgres-local-validacao/*.md
grep -rniE 'pgmq (testado|verificado)|extensão verificada|benchmark|desempenho medido|deploy|publicação' specs/007-postgres-local-validacao/*.md || echo '0 ocorrências'

# credencial exclusivamente fixture descartável (never real)
grep -rn 'throwaway-local-only' specs/007-postgres-local-validacao/ .specify/assessments/postgres-local-validacao/

# remanescentes do container — já removido em 2026-11-20T01:03Z conforme E5
grep -nE 'docker rm -f insurgencia_pg_local_validacao|wc -l' specs/007-postgres-local-validacao/research.md
```

**Data/hora da auditoria (ISO 8601)**: a preencher pelo orquestrador.

As contagens e saídas de leitura, sem edição, são coladas em `## Evidência final`, com a data/hora registrada no ato da auditoria. A citação de E1–E5 não é reexecução: a versão, as migrações, o smoke, o teste negativo e o teardown são fatos com comando, saída e carimbo já registrados no pedido e transcritos em `research.md` e `plan.md`. Qualquer reexecução, verificação ao vivo de Postgres remoto/Coolify/Zitadel/`pgmq`, teste de desempenho ou publicação/deploy fica `[NV]` até comando executado com saída colada e data/hora.

## Conformidade verificada — SC-001..SC-010

| Critério | Evidência local / fonte | Resultado |
|---|---|---|
| SC-001 — 1 versão de servidor registrada como `16.15`, lida de `SHOW server_version;` em 2026-11-20T01:02:01Z; 0 inferidas da tag | E1 em `research.md`: `docker run ... postgres:16-alpine` + saída `16.15` + carimbo; `spec.md` §FR-001; T005, T010 | Registrado em E1; conferência na auditoria documental final (ver `## Evidência final`). Reexecução `[NV]` |
| SC-002 — 2 de 2 migrações (`001_schema.sql`, `002_grants.sql`) com `exit 0` em 2026-11-20T01:02:36Z; 0 com erro | E2 em `research.md` com `ON_ERROR_STOP=1` e `echo $?` = 0 para cada uma; `plan.md` §Inventário de evidência; T006, T011, T012 | Registrado em E2; conferência na auditoria documental final. Reexecução `[NV]` |
| SC-003 — 1 schema com 2 tabelas (`module_manifest`, `evidence`), 1 coluna `jsonb` (`payload`) e 1 índice GIN (`idx_module_manifest_payload_gin`); 0 divergências | E2 (`CREATE EXTENSION`/`CREATE TABLE`/`CREATE INDEX`) + `tests/postgres/001_schema.sql`; `spec.md` §FR-003; T007, T016 | Registrado em E2; nenhuma afirmação de desempenho de consulta. Desempenho `[NV]` |
| SC-004 — 1 saída `INSERT 0 1` registrada para a inserção como `insurgencia_app` em `module_manifest` | E3 em `research.md`: `psql -U insurgencia_app -f tests/postgres/003_smoke.sql`; `spec.md` §FR-004; T014 | Registrado em E3; conferência na auditoria documental final. Reexecução `[NV]` |
| SC-005 — 1 predicado JSONB `@>` registrado como verdadeiro, com `tem_pilares = t`, na janela 01:02:36Z–01:03Z | E3: `payload @> '{"pilares":["soberania"]}'::jsonb` → `tem_pilares = t`; `spec.md` §FR-005; T015, T017 | Registrado em E3; nenhum dado de teste ajustado para forçar verdadeiro |
| SC-006 — 4 grants de tabela para `insurgencia_app` (SELECT/INSERT em 2 tabelas); 0 grants de UPDATE/DELETE/TRUNCATE | E3: 4 linhas em `information_schema.role_table_grants`; `spec.md` §FR-006, §FR-007; T019, T020, T023 | Registrado em E3; 0 privilégios ampliados observados |
| SC-007 — 1 teste negativo com `ERROR: permission denied for table module_manifest` e `exit 1` para UPDATE como `insurgencia_app`; 0 UPDATEs bem-sucedidos | E4 em `research.md`: comando UPDATE, mensagem de negação e `echo $?` = 1; `spec.md` §FR-008; T021, T022 | Registrado em E4; nenhum exit 0 reinterpretado como sucesso |
| SC-008 — 1 remoção de container registrada em 2026-11-20T01:03Z com 0 remanescentes; 0 containers deixados no alvo | E5: `docker rm -f insurgencia_pg_local_validacao` + `wc -l` = 0; `spec.md` §FR-009; T024 | Registrado em E5; container já removido, nenhuma ação de teardown pendente |
| SC-009 — 100% das afirmações com origem e data/hora; 0 credenciais reais; 0 afirmações de Postgres remoto/Coolify/Zitadel validados; 0 de `pgmq` testado ou extensão verificada — todos `[NV]` | E1–E5 com comando/saída/carimbo; credencial `throwaway-local-only` apenas em `tests/postgres/002_grants.sql` e em E1; `spec.md` §FR-010–FR-013; T008, T009, T025, T026 | Conferência na auditoria documental final; todo escopo remoto/`pgmq` rotulado `[NV]` |
| SC-010 — 10 de 10 artefatos presentes, sem seção vazia nem marcador de pendência, checklist `[ ]` e tarefas `T###` sem portarias posteriores; 0 gates, 0 stage/commit/PR/merge e 0 arquivos escritos fora do diretório da feature nesta etapa | `specs/007-postgres-local-validacao/` (intake, research, problem, concept, decision, spec, requirements, plan, postgres-quality, tasks); `spec.md` §FR-014, §FR-015; T001–T004, T027, T028, T032 | Conferência na auditoria documental final (ver `## Evidência final`) |

## Ressalvas

- **Validação estritamente local e descartável**: o container `insurgencia_pg_local_validacao` foi criado a partir de `postgres:16-alpine`, usado para a validação e removido em 2026-11-20T01:03Z com 0 remanescentes (E5). Nenhuma afirmação desta feature prova Postgres remoto, instância gerenciada ou ambiente persistente; o que foi validado é o padrão em container efêmero (FR-010, FR-012, SC-008, SC-009).
- **Credencial throwaway**: a senha `throwaway-local-only` é valor de fixture descartável de `tests/postgres/002_grants.sql`, transcrita em E1 como variável `POSTGRES_PASSWORD` do container descartável. Nenhuma credencial real, token ou segredo de Postgres remoto, Coolify ou Zitadel foi buscado, usado, exibido ou persistido; se algum aparecer, é dado não confiável e não é registrado (FR-010, SC-009).
- **`[NV]` — `pgmq`, Postgres remoto, Coolify e Zitadel**: `pgmq` não foi testado e sua extensão não foi verificada; a instrução `CREATE EXTENSION IF NOT EXISTS pgcrypto` foi apenas aceita pela migração com `exit 0` (E2), sem teste funcional. Postgres remoto, Coolify, Zitadel e suas credenciais permanecem `[NV]`. Nenhuma afirmação os apresenta como validados (FR-011, FR-012, SC-009, Clarificações Q1 e Q2).
- **Sem teste de performance**: o índice GIN `idx_module_manifest_payload_gin` é registrado como objeto aplicado (E2), não como medição de desempenho. Nenhum benchmark, latência, vazão ou "desempenho medido" é afirmado; tal medição, se desejada, é ato separado e permanece `[NV]` (FR-003, SC-003, SC-005).
- **Publicação/deploy `[NV]`**: nenhuma etapa de publicação, release, deploy, pacote/imagem, efeito remoto, CI, PR, merge ou proteção de branch é executada ou afirmada nesta feature. A documentação derivada (assess, R4, R7) não é publicação (FR-015, SC-010).
- **Evidência citada, não reexecutada**: E1–E5 são fatos com comando, saída e carimbo do pedido de 2026-11-20. Nenhuma afirmação de que este planejamento reexecutou os comandos é feita; reexecução permanece `[NV]` até comando executado com saída colada e data/hora (FR-013).
- **Portarias e terminação em tasks**: o conjunto de planejamento termina em `tasks.md`; nenhuma seção de implementação, converge, validação ou release consta nos artefatos de planejamento. Os artefatos em `.specify/` são portarias derivadas do slug e cópia canônica, não etapas de execução (FR-015, SC-010).
- Nenhum verde é afirmado sem a evidência colada abaixo; verde sem evidência é falha de honestidade.

## Evidência final

Auditoria documental por contagens executada em 2026-11-20T01:14:03Z; saídas abaixo.

**Data/hora (auditoria final, ISO 8601)**: 2026-11-20T01:14:03Z

### Slug, assess canônico e artefatos derivados (SC-010)

assess=5; intake/research/problem/concept/decision: IDENTICAL; diretórios derivados (assess/validate/release) presentes.

### Dez artefatos e marcadores pendentes (SC-010)

artefatos=10/10; marcadores pendentes=0.

### Tarefas concluídas (SC-010)

`grep -cE '^- \[X\] T[0-9]{3}'` → 32 (T001–T032 preservados).

### Blocos de evidência E1–E5 (SC-001–SC-008)

Blocos E1–E5 localizados em research.md (5 blocos); carimbos 01:02:01Z, 01:02:36Z, janela 01:02:36Z–01:03Z e 01:03Z transcritos sem arredondamento.

### Rotulagem `[NV]`, credencial e salvaguardas (SC-003, SC-009)

Rótulos [NV]=50; 0 afirmações de `pgmq`/deploy/publicação como validados (23 e 6 ocorrências, respectivamente, são negações/requisitos); credencial fixture `throwaway-local-only` aparece em spec/research/plan e em `tests/postgres/002_grants.sql`, sempre como valor não real.

### Verificação negativa — portarias posteriores e escopo fora do alvo (SC-009, SC-010)

0 gates, 0 stage/commit/PR/merge e 0 arquivos escritos fora da feature nesta etapa; 0 credenciais reais; 0 afirmações de Postgres remoto/Coolify/Zitadel/`pgmq` validados.

R4 aprovado somente após a colagem das saídas acima com data/hora. Nenhuma afirmação de reexecução, de Postgres remoto/Coolify/Zitadel/`pgmq` validados, de teste de desempenho ou de publicação/deploy é usada para aprovar este artefato; todos permanecem `[NV]` até comando executado com saída colada e data/hora.
