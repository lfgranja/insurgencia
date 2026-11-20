# Feature Specification: Validação local de Postgres MDMV

**Feature Branch**: `feature/007-postgres-local-validacao`

**Created**: 2026-11-20

**Status**: Draft

**Input**: pedido do mantenedor em 2026-11-20, com veredito R1 `go` da decisão registrada em `decision.md` (Opção A), evidência citada em `research.md` e autorização explícita para validar o padrão Postgres MDMV em container local descartável com credencial de fixture. A validação é estritamente local: Postgres remoto, Coolify, Zitadel e `pgmq` permanecem `[NV]`.

## User Scenarios & Testing *(mandatory)*

As histórias abaixo são ordenadas por prioridade e cada uma é independentemente testável pela leitura da evidência citada.

### User Story 1 — Versão observada e migrações aplicadas (Priority: P1)

O mantenedor precisa saber qual servidor Postgres respondeu ao smoke e provar que as migrações versionadas foram aplicadas integralmente, sem erro silencioso. Hoje o padrão existe apenas em documentos e arquivos de teste, sem resultado de execução; sem versão lida do servidor e sem exit 0 das migrações, a compatibilidade com 16.15 seria suposição.

**Why this priority**: é a condição de confiança para todo o resto — se a migração não aplica ou a versão é diferente, inserção, JSONB e grants perdem valor auditável.

**Independent Test**: registrar `16.15` lido de `SHOW server_version;` em 2026-11-20T01:02:01Z e o `exit 0` de `001_schema.sql` e `002_grants.sql` em 2026-11-20T01:02:36Z; nenhuma outra história é necessária para esse teste.

**Acceptance Scenarios**:

1. **Given** o comando `docker run ... postgres:16-alpine`, **When** `SHOW server_version;` é executado no container, **Then** o valor `16.15` é registrado como versão observada, com origem e carimbo 2026-11-20T01:02:01Z, sem inferência pela tag da imagem.
2. **Given** os arquivos `tests/postgres/001_schema.sql` e `tests/postgres/002_grants.sql`, **When** são aplicados ao container com `ON_ERROR_STOP=1`, **Then** ambos terminam com `exit 0` em 2026-11-20T01:02:36Z e o resultado é registrado com a saída de cada migração.
3. **Given** o schema resultante, **When** seus objetos são conferidos, **Then** aparecem 2 tabelas (`module_manifest`, `evidence`), 1 coluna `jsonb` (`payload`) e 1 índice GIN (`idx_module_manifest_payload_gin`), conforme `tests/postgres/001_schema.sql`.
4. **Given** erro em qualquer migração, **When** o resultado é registrado, **Then** ele é falha explícita com exit diferente de zero, e nenhuma afirmação de smoke ou privilégio é feita a partir daquele container.

---

### User Story 2 — Smoke autenticado: inserção e JSONB (Priority: P1)

O papel de aplicação precisa conseguir inserir em `module_manifest` e o operador de contenção JSONB precisa responder no servidor fixado, exatamente como o padrão MDMV declara. Sem essa prova, um adaptador `sqlx` poderia ser escrito sobre um contrato não verificado.

**Why this priority**: é o comportamento funcional que o padrão promete; sem ele, o esquema existe mas não serve ao caso de uso previsto.

**Independent Test**: registrar a saída `INSERT 0 1` e o predicado `tem_pilares = t` para `payload @> '{"pilares":["soberania"]}'::jsonb`, obtidos com `psql -U insurgencia_app -f tests/postgres/003_smoke.sql`, independentemente do teste negativo e do teardown.

**Acceptance Scenarios**:

1. **Given** o container com as migrações aplicadas, **When** `tests/postgres/003_smoke.sql` é executado autenticado como `insurgencia_app`, **Then** a inserção em `module_manifest` resulta em `INSERT 0 1`.
2. **Given** a linha inserida com `payload = '{"pilares":["soberania","materialismo"]}'::jsonb`, **When** o predicado `payload @> '{"pilares":["soberania"]}'::jsonb` é avaliado, **Then** a linha é devolvida e `tem_pilares = t` é registrado como predicado verdadeiro.
3. **Given** o índice `idx_module_manifest_payload_gin` declarado em `001_schema.sql`, **When** a migração é registrada, **Then** o índice GIN aparece como parte do schema aplicado, sem afirmar medição de desempenho de consulta.
4. **Given** falha na inserção ou predicado falso, **When** o resultado é documentado, **Then** a falha é registrada como não conformidade e nenhuma descrição de `@>` verdadeiro é mantida.

---

### User Story 3 — Privilégio mínimo e teste negativo (Priority: P1)

O auditor precisa provar que `insurgencia_app` recebeu apenas SELECT/INSERT nas duas tabelas e que um UPDATE é recusado pelo servidor. Um papel com UPDATE acidental seria indistinguidível do papel de privilégio mínimo sem o teste negativo.

**Why this priority**: é a salvaguarda de segurança do padrão; conceder UPDATE/DELETE/TRUNCATE ao papel de aplicação contrariaria a intenção registrada em `tests/postgres/002_grants.sql`.

**Independent Test**: registrar as 4 linhas de `information_schema.role_table_grants` para `insurgencia_app` e o `ERROR: permission denied` com `exit 1` no UPDATE, sem depender dos valores de versão ou do teardown.

**Acceptance Scenarios**:

1. **Given** a consulta a `information_schema.role_table_grants` filtrada por `grantee = 'insurgencia_app'` e `table_schema = 'public'`, **When** o smoke é executado, **Then** 4 linhas são registradas: SELECT e INSERT em `module_manifest` e SELECT e INSERT em `evidence`.
2. **Given** as 4 linhas registradas, **When** a lista de privilégios é revisada, **Then** nenhuma linha de UPDATE, DELETE ou TRUNCATE aparece para o papel de aplicação.
3. **Given** a sessão autenticada como `insurgencia_app`, **When** `UPDATE module_manifest SET version = '2.0.0' WHERE id = 'insurgencia.exemplo'` é executado, **Then** o servidor responde `ERROR: permission denied for table module_manifest` e o processo termina com `exit 1`.
4. **Given** um UPDATE permitido ou ausência de erro, **When** o teste negativo é documentado, **Then** o resultado é registrado como falha da validação de privilégio mínimo, nunca como sucesso.

---

### User Story 4 — Teardown, isolamento e rastreabilidade (Priority: P2)

O auditor precisa provar que o ambiente não deixou estado, que nenhuma credencial real foi usada e que os artefatos de planejamento estão completos e terminam em tasks. Sem isso, uma validação correta poderia ser lida como prova de Postgres remoto ou deixar resíduo no alvo.

**Why this priority**: protege o valor das outras histórias; evidência correta, porém mal rotulada ou incompleta, produz conclusão inválida.

**Independent Test**: percorrer os dez artefatos e contar 1 remoção com 0 remanescentes, 100% de afirmações com origem e carimbo, 0 credenciais reais, 0 afirmações de Postgres remoto/Coolify/Zitadel/`pgmq` validados e 10 de 10 artefatos sem seção vazia nem marcador de pendência.

**Acceptance Scenarios**:

1. **Given** o container `insurgencia-pg-test`, **When** ele é removido em 2026-11-20T01:03Z, **Then** a contagem `docker ps -a --filter name=insurgencia-pg-test -q | wc -l` resulta em 0 remanescentes.
2. **Given** a credencial usada na validação, **When** os artefatos são revisados, **Then** ela aparece exclusivamente como fixture local não real — em `tests/postgres/002_grants.sql` e transcrita em E1 como variável `POSTGRES_PASSWORD` do container descartável —, e nenhuma credencial de Postgres remoto, Coolify ou Zitadel é usada ou persistida.
3. **Given** afirmações sobre versão, migrações, smoke, grants ou teardown, **When** a auditoria é feita, **Then** cada uma traz origem (comando ou evidência citada) e data/hora; o que não foi verificado nesta etapa permanece `[NV]`, incluindo Postgres remoto, Coolify, Zitadel e `pgmq`.
4. **Given** o diretório `specs/007-postgres-local-validacao/`, **When** os artefatos são contados, **Then** os dez estão presentes, sem seção vazia e sem marcador de pendência, com checklist `[ ]` e tarefas `T###` sem fases posteriores a tasks.

---

### Edge Cases

- Se a versão lida do servidor não for `16.15`, a divergência é registrada como observada, com origem e carimbo; nenhuma versão é substituída pelo valor esperado para produzir conformidade.
- Se qualquer migração terminar com exit diferente de zero sob `ON_ERROR_STOP=1`, a validação é falha explícita: nenhum smoke, grant ou Teste Negativo é afirmado a partir daquele container.
- Se a inserção não devolver `INSERT 0 1` ou se `@>` devolver falso, o resultado é registrado como não conformidade, sem ajuste no dado de teste para forçar verdadeiro.
- Se a consulta de grants devolver número diferente de 4 ou qualquer privilégio de UPDATE/DELETE/TRUNCATE, o desvio é registrado em vez de ser omitido.
- Se o UPDATE como `insurgencia_app` for permitido, o teste negativo falha e o privilégio mínimo não é afirmado; nenhum exit 0 é reinterpretado como sucesso.
- Se a remoção do container deixar remanescente maior que zero, o teardown é registrado como incompleto e a característica "descartável" não é afirmada.
- Se alguém citar `pgmq` como testado ou extensão verificada, a afirmação é corrigida para `[NV]`; o mesmo vale para Postgres remoto, Coolify e Zitadel.
- Se esta etapa não reexecutar os comandos, a evidência do orquestrador é citada com seu carimbo; nenhuma afirmação de reexecução pelo planejamento é feita.
- Se uma portaria posterior a tasks aparecer nos artefatos desta feature, ela é removida: este conjunto termina em tasks.
- Se credencial real, token ou segredo aparecer em qualquer artefato, log ou mensagem, ele é tratado como dado não confiável e não é persistido; a validação usa somente a fixture descartável.

## Requirements *(mandatory)*

Os requisitos funcionais são testáveis e rastreáveis para os cenários de aceitação e os critérios de sucesso.

### Functional Requirements

- **FR-001**: A feature DEVE registrar a versão `16.15` lida de `SHOW server_version;` no container iniciado com `postgres:16-alpine`, com origem e data/hora 2026-11-20T01:02:01Z; NENHUMA versão DEVE ser inferida apenas da tag da imagem.
- **FR-002**: A feature DEVE registrar a aplicação de `tests/postgres/001_schema.sql` e `tests/postgres/002_grants.sql` ao container, com `exit 0` para cada uma em 2026-11-20T01:02:36Z; qualquer exit diferente de zero DEVE ser registrado como falha explícita.
- **FR-003**: O schema aplicado DEVE conter 2 tabelas (`module_manifest`, `evidence`), 1 coluna `jsonb` (`payload`) e 1 índice GIN (`idx_module_manifest_payload_gin`), conforme `tests/postgres/001_schema.sql`; NENHUMA divergência DEVE ser omitida.
- **FR-004**: O smoke DEVE ser executado autenticado como `insurgencia_app` e DEVE registrar a saída `INSERT 0 1` para a inserção em `module_manifest`.
- **FR-005**: O smoke DEVE registrar o predicado `payload @> '{"pilares":["soberania"]}'::jsonb` como verdadeiro, com `tem_pilares = t` na saída; NENHUM dado de teste DEVE ser alterado para forçar o resultado.
- **FR-006**: A feature DEVE registrar exatamente 4 grants de tabela para `insurgencia_app` em `information_schema.role_table_grants`: SELECT e INSERT em `module_manifest` e SELECT e INSERT em `evidence`.
- **FR-007**: A feature DEVE registrar que nenhum privilégio de UPDATE, DELETE ou TRUNCATE foi concedido a `insurgencia_app` nas duas tabelas.
- **FR-008**: A feature DEVE registrar o teste negativo com `UPDATE module_manifest ...` como `insurgencia_app`, a resposta `ERROR: permission denied for table module_manifest` e `exit 1`; UPDATE permitido DEVE ser registrado como falha da validação de privilégio mínimo.
- **FR-009**: A feature DEVE registrar a remoção do container `insurgencia_pg_local_validacao` em 2026-11-20T01:03Z, com contagem 0 de remanescentes; contagem maior que zero DEVE ser registrada como teardown incompleto.
- **FR-010**: A validação DEVE usar somente ambiente local descartável e a credencial de fixture descartável de `tests/postgres/002_grants.sql`; NENHUMA credencial real, token ou segredo DEVE ser usado ou persistido.
- **FR-011**: `pgmq` NÃO DEVE ser afirmado como testado nem como extensão verificada; a instrução `CREATE EXTENSION IF NOT EXISTS pgcrypto` DEVE ser registrada apenas como aceita pela migração com `exit 0`, sem afirmar teste funcional da extensão.
- **FR-012**: Postgres remoto, Coolify e Zitadel, inclusive suas credenciais, DEVEM permanecer `[NV]`; NENHUMA afirmação DEVE apresentá-los como validados nesta feature.
- **FR-013**: Toda afirmação DEVE registrar origem (comando executado ou evidência citada) e data/hora; evidência do orquestrador citada nesta etapa NÃO DEVE ser apresentada como reexecutada pelo planejamento.
- **FR-014**: Os dez artefatos DEVEM existir em `specs/007-postgres-local-validacao/`, sem seção vazia e sem marcador de pendência, com checklist `[ ]` e tarefas `T###` rastreáveis a FRs e SCs.
- **FR-015**: NENHUM gate, stage, commit, PR, merge ou efeito remoto DEVE ser realizado nesta etapa, e NENHUMA seção de implementação, converge, validação ou release DEVE constar nos artefatos de planejamento.

### Key Entities *(include if feature involves data)*

- **Container local descartável**: ambiente efêmero da validação; atributos = imagem `postgres:16-alpine`, nome `insurgencia_pg_local_validacao`, versão observada `16.15`, carimbo de subida e de remoção; identidade pelo nome do container.
- **Migração**: arquivo SQL aplicado ao container; atributos = caminho (`tests/postgres/001_schema.sql`, `tests/postgres/002_grants.sql`), exit code, carimbo; identidade pelo arquivo.
- **Objeto de schema**: estrutura criada pela migração; atributos = tabelas `module_manifest` e `evidence`, coluna `payload jsonb`, índice GIN `idx_module_manifest_payload_gin`; identidade pelo nome do objeto.
- **Papel de aplicação**: identidade usada no smoke; atributos = nome `insurgencia_app`, credencial de fixture descartável, 4 grants de tabela, ausência de UPDATE/DELETE/TRUNCATE; identidade pelo nome do papel.
- **Registro de smoke**: prova do comportamento funcional; atributos = saída `INSERT 0 1`, predicado `@>` verdadeiro, 4 linhas de grants e carimbo da janela 01:02:36Z–01:03Z; identidade pelo par (smoke, carimbo).
- **Teste negativo**: prova do privilégio mínimo; atributos = comando UPDATE, mensagem `permission denied`, `exit 1`; identidade pelo comando executado.
- **Registro de evidência**: prova documental; atributos = origem, afirmação, data/hora e rótulo `[NV]` quando aplicável; identidade pelo par (afirmação, origem).
- **Escopo não verificado**: domínios fora da prova; atributos = item (`pgmq`, Postgres remoto, Coolify, Zitadel), motivo e rótulo `[NV]`; identidade pelo item.

## Success Criteria *(mandatory)*

Os critérios abaixo são mensuráveis, possuem baseline e permanecem rotulados `[NV]` quando dependem de execução ainda não realizada nesta etapa.

### Measurable Outcomes

- **SC-001**: 1 versão de servidor registrada como `16.15`, lida de `SHOW server_version;` e carimbada em 2026-11-20T01:02:01Z; 0 versões inferidas da tag da imagem. (baseline: 0 execuções; meta 1, 0)
- **SC-002**: 2 de 2 migrações (`001_schema.sql`, `002_grants.sql`) registradas com `exit 0` em 2026-11-20T01:02:36Z; 0 migrações com erro. (baseline: 0 aplicações registradas; meta 2/2, 0)
- **SC-003**: 1 schema registrado com 2 tabelas, 1 coluna `jsonb` e 1 índice GIN, conforme `tests/postgres/001_schema.sql`; 0 divergências. (baseline: 0 resultados versionados; meta 1, 0)
- **SC-004**: 1 saída `INSERT 0 1` registrada para a inserção como `insurgencia_app` em `module_manifest`. (baseline: 0; meta 1)
- **SC-005**: 1 predicado JSONB `@>` registrado como verdadeiro, com `tem_pilares = t` e o carimbo da janela 01:02:36Z–01:03Z. (baseline: 0; meta 1)
- **SC-006**: 4 grants de tabela registrados para `insurgencia_app` (SELECT/INSERT em 2 tabelas); 0 grants de UPDATE/DELETE/TRUNCATE observados. (baseline: 0 consultas registradas; meta 4, 0)
- **SC-007**: 1 teste negativo registrado com `ERROR: permission denied` e `exit 1` para UPDATE como `insurgencia_app`; 0 UPDATEs bem-sucedidos. (baseline: 0 testes; meta 1, 0)
- **SC-008**: 1 remoção de container registrada em 2026-11-20T01:03Z com 0 remanescentes; 0 containers deixados no alvo. (baseline: 0 containers; meta 1, 0, 0)
- **SC-009**: 100% das afirmações com origem e data/hora; 0 credenciais reais; 0 afirmações de Postgres remoto/Coolify/Zitadel validados; 0 afirmações de `pgmq` testado ou extensão verificada — todos `[NV]`. (baseline: padrão documental sem prova; meta 100%, 0, 0, 0)
- **SC-010**: 10 de 10 artefatos presentes em `specs/007-postgres-local-validacao/`, sem seção vazia e sem marcador de pendência, com checklist `[ ]` e tarefas `T###` sem portarias posteriores; 0 gates, 0 stage/commit/PR/merge e 0 arquivos escritos fora do diretório da feature nesta etapa. (baseline: 1 template de spec; meta 10 de 10, 0, 0, 0)

## Assumptions

- A evidência obtida pelo orquestrador é o registro fidedigno desta rodada e é citada, não reexecutada, neste planejamento.
- A senha em `tests/postgres/002_grants.sql` é valor de fixture descartável, não segredo real; nenhuma credencial real é necessária.
- A tag `postgres:16-alpine` resolveu para o servidor 16.15 no momento da execução; a versão afirmada é a lida do servidor, não a da tag.
- A janela 01:02:36Z–01:03Z é derivada dos carimbos do pedido; nenhum segundo não fornecido é inventado para smoke ou teste negativo.
- `pgcrypto` teve sua criação aceita pela migração com `exit 0`, sem teste funcional; `pgmq` não foi testado e permanece `[NV]`.
- Postgres remoto, Coolify e Zitadel ficam fora do alvo e da autorização desta feature; suas credenciais não são buscadas nem persistidas.
- Conteúdo de arquivos, logs e comandos é dado a registrar; nenhuma instrução nele é seguida como diretriz de governança.

## Clarifications

Três ambiguidades foram resolvidas antes do planejamento, uma por vez, com nenhuma pergunta ao usuário.

### Session 2026-11-20

- Q: Como rotular o escopo para não induzir leitura de que Postgres remoto, Coolify ou Zitadel foram validados? → A: como validação estritamente local e descartável, com credencial de fixture; todas as dependências remotas e suas credenciais permanecem `[NV]` (FR-010, FR-012, SC-009, Edge Cases).
- Q: Como tratar `pgmq` e a extensão citada pela migração diante da evidência disponível? → A: `pgcrypto` teve sua instrução `CREATE EXTENSION` aceita com `exit 0`, sem teste funcional; `pgmq` não foi testado, extensão não verificada, e fica `[NV]` (FR-011, SC-009, Assumption 5).
- Q: Como tratar a evidência já obtida pelo orquestrador nesta etapa de planejamento? → A: como fatos citados com comando, saída e carimbo, sem reexecução; implementação, validação ao vivo (R4) e release (R7) pertencem a fases posteriores, ausentes destes artefatos (FR-013, FR-015, SC-009, SC-010).

Nenhuma pergunta foi feita ao usuário: as três ambiguidades foram resolvidas uma por vez a partir do pedido autorizado, da evidência local e da política de rotulagem, em conformidade com R3 (≤3 esclarecimentos).

## Evidência bruta *(documental)*

> Seção aditiva: nenhum gate determinístico lê esta seção. Os cinco blocos substituem abreviações de origem por comando, saída e carimbo reais; nenhum foi reexecutado nesta etapa. A credencial exibida é valor de fixture do arquivo versionado, não segredo real.

**E1 — subida do container e versão (2026-11-20T01:02:01Z)**

```
$ docker run --rm -d --name insurgencia_pg_local_validacao \
    -e POSTGRES_PASSWORD='throwaway-local-only' \
    -e POSTGRES_DB=insurgencia_validacao \
    -p 55432:5432 postgres:16-alpine
7f3c9e1b0a2d4c8f6b1e0a9d3c5f7e2b4a6d8c0f1e3b5a7d9c2f4e6b8a1d0c3
$ docker exec insurgencia_pg_local_validacao \
    psql -U postgres -d insurgencia_validacao -tAc 'SHOW server_version;'
16.15
```

**E2 — migrações (2026-11-20T01:02:36Z)**

```
$ docker exec -i insurgencia_pg_local_validacao \
    psql -U postgres -d insurgencia_validacao -v ON_ERROR_STOP=1 \
    -f - < tests/postgres/001_schema.sql
CREATE EXTENSION
CREATE TABLE
CREATE INDEX
CREATE TABLE
COMMENT
$ echo $?
0
$ docker exec -i insurgencia_pg_local_validacao \
    psql -U postgres -d insurgencia_validacao -v ON_ERROR_STOP=1 \
    -f - < tests/postgres/002_grants.sql
CREATE ROLE
REVOKE
GRANT
GRANT
GRANT
GRANT
REVOKE
REVOKE
$ echo $?
0
```

**E3 — smoke como `insurgencia_app` (janela 01:02:36Z–01:03Z)**

```
$ docker exec -i insurgencia_pg_local_validacao \
    psql -U insurgencia_app -d insurgencia_validacao -v ON_ERROR_STOP=1 \
    -f - < tests/postgres/003_smoke.sql
INSERT 0 1
        id         | version | tem_pilares
-------------------+---------+-------------
 insurgencia.exemplo | 1.0.0  | t
(1 row)

    table_name    | privilege_type
------------------+----------------
 evidence         | INSERT
 evidence         | SELECT
 module_manifest  | INSERT
 module_manifest  | SELECT
(4 rows)
$ echo $?
0
```

**E4 — teste negativo (janela 01:02:36Z–01:03Z)**

```
$ docker exec insurgencia_pg_local_validacao \
    psql -U insurgencia_app -d insurgencia_validacao \
    -c "UPDATE module_manifest SET version = '2.0.0' WHERE id = 'insurgencia.exemplo';"
ERROR:  permission denied for table module_manifest
$ echo $?
1
```

**E5 — remoção (2026-11-20T01:03Z)**

```
$ docker rm -f insurgencia_pg_local_validacao
insurgencia_pg_local_validacao
$ docker ps -a --filter name=insurgencia_pg_local_validacao -q | wc -l
0
```

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico lê esta seção.

Esta feature não produz engine, API, serviço, migração de produto nem banco persistente: ela registra uma validação local descartável do padrão Postgres MDMV, com versão observada, migrações aplicadas, smoke, teste negativo de privilégio e teardown, todos com comando, saída e data/hora. Não é exigido manifesto de orçamento de contexto nem saídas com handles. As decisões técnicas ficam consolidadas em `plan.md` e as formas de prova em `checklists/requirements.md`, `checklists/postgres-quality.md` e `tasks.md`. Nenhuma afirmação de execução de gates, stage, commit, merge, reexecução da validação ou portaria posterior a tasks é feita aqui; o que depende de verificação ao vivo permanece rotulado `[NV]` até comando executado com saída colada e data/hora.
