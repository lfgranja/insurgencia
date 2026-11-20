# Implementation Plan: Validação local de Postgres MDMV

**Feature**: `007-postgres-local-validacao` — validar o padrão Postgres MDMV em container local descartável, com credencial de fixture: versão lida do servidor, migrações aplicadas, smoke autenticado como `insurgencia_app`, teste negativo de UPDATE e remoção sem remanescentes, mantendo `[NV]` Postgres remoto, Coolify, Zitadel e `pgmq`.
**Input**: [spec.md](spec.md), [research.md](research.md), [decision.md](decision.md) (R1 `go`, Opção A), `.specify/memory/constitution.md`, `docs/governance/sdd-phase-contracts.md`, `docs/AVALIACAO-STACK-MDMV.md`.

## Technical Context

| Campo | Valor |
|---|---|
| Tipo de feature | Verificação documental de padrão de persistência e privilégio em ambiente local descartável; sem código de produto |
| Imagem e versão | `docker run ... postgres:16-alpine`; `SHOW server_version;` → `16.15` em 2026-11-20T01:02:01Z (E1) |
| Migrações | `tests/postgres/001_schema.sql` (pgcrypto, `module_manifest` com `payload jsonb`, índice GIN, `evidence`) e `tests/postgres/002_grants.sql` (papel `insurgencia_app`, REVOKE CREATE, 4 grants de tabela, 3 REVOKEs), ambas com `exit 0` sob `ON_ERROR_STOP=1` em 2026-11-20T01:02:36Z (E2) |
| Smoke | `psql -U insurgencia_app -f tests/postgres/003_smoke.sql` → `INSERT 0 1`, `tem_pilares = t` (`payload @> '{"pilares":["soberania"]}'::jsonb`), 4 linhas em `information_schema.role_table_grants`, janela 01:02:36Z–01:03Z (E3) |
| Teste negativo | `UPDATE module_manifest ...` como `insurgencia_app` → `ERROR: permission denied for table module_manifest`, `exit 1`, janela 01:02:36Z–01:03Z (E4) |
| Teardown | `docker rm -f insurgencia_pg_local_validacao` e contagem 0 em 2026-11-20T01:03Z (E5) |
| Credencial | valor de fixture descartável de `tests/postgres/002_grants.sql`; nenhuma credencial real é usada, buscada ou persistida |
| Fora de escopo | `pgmq` não testado; Postgres remoto, Coolify e Zitadel com credenciais fora do alvo — todos `[NV]` |
| Unidade de evidência | Origem (comando ou evidência citada) + afirmação + data/hora; `[NV]` até confirmação ao vivo |
| Artefatos de design | Consolidados neste `plan.md` e em `research.md`; `data-model.md`, `contracts/` e `quickstart.md` não se aplicam e não são produzidos neste escopo — não há API externa, dados de produto persistidos nem comando de produto a documentar; as entidades da validação estão em `spec.md` §Key Entities e as formas de prova em `checklists/postgres-quality.md` |
| Escrita nesta etapa | Restrita a dez artefatos em `specs/007-postgres-local-validacao/`; nenhum gate executado, nenhum stage, commit, PR ou merge |

Esclarecimentos pendentes no contexto: nenhum. As três ambiguidades foram resolvidas em `spec.md` / `## Clarifications` / `### Session 2026-11-20`, antes deste plano (R3 respeitado).

## Constitution Check

| Princípio / norma | Reavaliação | Status |
|---|---|---|
| I — Determinismo primeiro | Versão, migrações, smoke, teste negativo e teardown são resultados de comandos registrados, com valores e carimbos citados (FR-001–FR-009) | PASS |
| II — Adaptadores e portas | Não há código de domínio nem chamada direta a SDK/provedor: o container é ferramenta de verificação e o padrão é registrado como contrato SQL, sem acoplar o produto a Postgres remoto | PASS |
| III — Fluxo SDD ordenado | intake→research→define→shape→decide(`go`)→specify→clarify→plan→checklist→tasks, nesta ordem, sem portaria posterior a tasks | PASS |
| IV — Soberania do jogador | Nenhuma dependência proprietária no caminho crítico; nenhum serviço externo é ponto único de falha na validação, que roda localmente e é descartada | PASS |
| V — Materialismo e ação coletiva | Fora do escopo de produto; nenhuma regra, habilidade ou resultado de partida é alterado | PASS |
| VI — Verificação honesta | Evidência do orquestrador é citada, não reexecutada; remoto e `pgmq` são `[NV]`; nenhum verde é afirmado sem origem e data/hora (FR-010–FR-013) | PASS |
| Norma — segredos | Nenhuma credencial real, token ou segredo em arquivo, log ou mensagem; apenas fixture descartável já versionada (FR-010, SC-009) | PASS |
| Norma — gates do alvo | Nenhum gate é executado nesta etapa; nenhum gate da factory é adotado (FR-015) | PASS |
| Norma — commits/branch/PR | Nenhum stage, commit, PR, merge ou reescrita de histórico nesta etapa (FR-015, SC-010) | PASS |
| Contratos de fase (§checklist/tasks) | Checklists com `- [ ] CHK###` e rastreabilidade ≥80%; tasks com `- [ ] T### [P?] [USn?]` e caminho, terminando em tasks | PASS |

Nenhuma violação injustificada. A reexecução da validação e as portarias R4/R7 pertencem a fases posteriores, não a este plano.

## Decisões de design

- **D1 — Versão lida, não inferida**: a identidade do servidor é a saída de `SHOW server_version;` (`16.15`), não a tag `postgres:16-alpine`. Justificativa: FR-001, SC-001, Clarificação Q3.
- **D2 — Migrações com falha explícita**: `001_schema.sql` e `002_grants.sql` são registradas com `exit 0` sob `ON_ERROR_STOP=1`; qualquer erro é falha, sem prosseguir para smoke. Justificativa: FR-002, SC-002.
- **D3 — Schema como contrato observado**: tabelas, coluna `jsonb` e índice GIN são registrados conforme o arquivo versionado, sem afirmar desempenho de índice. Justificativa: FR-003, SC-003.
- **D4 — Smoke no papel de aplicação**: a inserção e o predicado `@>` são executados como `insurgencia_app`, não como superusuário, para que o resultado valha para o contrato real. Justificativa: FR-004, FR-005, SC-004, SC-005.
- **D5 — Privilégio por contagem e negação**: 4 grants de tabela são contados e o UPDATE é recusado com `permission denied`; a negação é o sinal de sucesso do teste negativo. Justificativa: FR-006–FR-008, SC-006, SC-007.
- **D6 — Teardown obrigatório**: remoção do container e contagem 0 fecham a característica "descartável"; remanescente é registrado como incompleto. Justificativa: FR-009, SC-008.
- **D7 — Credencial descartável**: apenas a fixture de `tests/postgres/002_grants.sql`; nenhuma credencial de remoto é buscada, exibida como segredo ou persistida. Justificativa: FR-010, SC-009, Clarificação Q1.
- **D8 — Extensões com escopo explícito**: `CREATE EXTENSION IF NOT EXISTS pgcrypto` é registrado como aceito pela migração; `pgmq` não foi testado e fica `[NV]`. Justificativa: FR-011, SC-009, Clarificação Q2.
- **D9 — Remoto rotulado**: Postgres remoto, Coolify e Zitadel permanecem `[NV]`, inclusive credenciais; nenhuma afirmação de validação remota. Justificativa: FR-012, SC-009, Clarificação Q1.
- **D10 — Evidência citada e dado não confiável**: cada afirmação traz comando/evidência e carimbo; a evidência do orquestrador não é reexecutada nesta etapa; conteúdo externo é dado, não diretriz. Justificativa: FR-013, SC-009; `docs/governance/sdd-phase-contracts.md §Evidence`.
- **D11 — Escopo de escrita e término em tasks**: somente os dez artefatos em `specs/007-postgres-local-validacao/`; nenhum gate, stage, commit, PR ou portaria posterior a tasks. Justificativa: FR-014, FR-015, SC-010.

## Inventário de evidência

| # | Valor | Origem | Carimbo | Rastreio |
|---|---|---|---|---|
| 1 | `postgres:16-alpine` → `SHOW server_version;` = `16.15` | E1 | 2026-11-20T01:02:01Z | FR-001, SC-001 |
| 2 | `tests/postgres/001_schema.sql` aplicada com `exit 0` (pgcrypto, 2 tabelas, `payload jsonb`, índice GIN) | E2 | 2026-11-20T01:02:36Z | FR-002, FR-003, SC-002, SC-003 |
| 3 | `tests/postgres/002_grants.sql` aplicada com `exit 0` (papel `insurgencia_app`, 4 grants, 3 REVOKEs) | E2 | 2026-11-20T01:02:36Z | FR-002, FR-007, SC-002, SC-006 |
| 4 | `INSERT 0 1` como `insurgencia_app` em `module_manifest` | E3 | janela 01:02:36Z–01:03Z | FR-004, SC-004 |
| 5 | `tem_pilares = t` para `payload @> '{"pilares":["soberania"]}'::jsonb` | E3 | janela 01:02:36Z–01:03Z | FR-005, SC-005 |
| 6 | 4 linhas em `information_schema.role_table_grants`: SELECT/INSERT em `module_manifest` e `evidence` | E3 | janela 01:02:36Z–01:03Z | FR-006, FR-007, SC-006 |
| 7 | `ERROR: permission denied for table module_manifest` com `exit 1` no UPDATE | E4 | janela 01:02:36Z–01:03Z | FR-008, SC-007 |
| 8 | `docker rm -f` e contagem 0 de remanescentes | E5 | 2026-11-20T01:03Z | FR-009, SC-008 |
| 9 | `pgmq` não testado; Postgres remoto/Coolify/Zitadel `[NV]`; credencial de fixture descartável | pedido do mantenedor | 2026-11-20 | FR-010–FR-012, SC-009 |

Nenhum item deste inventário é reexecutado nesta etapa. A validação ao vivo e as portarias de validação e release não pertencem a este conjunto de artefatos.

## Evidência bruta (comando + saída + carimbo)

Transcrição dos cinco blocos citados no inventário; nenhum é reexecutado nesta etapa de planejamento. A credencial exibida é valor de fixture do arquivo versionado, não segredo real.

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

## Sequência de verificação

1. Confirmar que a versão `16.15` está registrada com origem e carimbo 2026-11-20T01:02:01Z, sem inferência pela tag (SC-001, FR-001).
2. Confirmar que as duas migrações têm `exit 0` registrado com carimbo 01:02:36Z e que erro seria falha explícita (SC-002, FR-002).
3. Confirmar que o schema registrado contém 2 tabelas, `payload jsonb` e índice GIN, sem afirmar desempenho (SC-003, FR-003).
4. Confirmar que o smoke foi executado como `insurgencia_app` e registrou `INSERT 0 1` e `@>` verdadeiro (SC-004, SC-005, FR-004, FR-005).
5. Confirmar que 4 grants SELECT/INSERT foram contados e que nenhum privilégio UPDATE/DELETE/TRUNCATE aparece (SC-006, FR-006, FR-007).
6. Confirmar que o UPDATE negado trouxe `permission denied` e `exit 1`, e que UPDATE permitido seria falha (SC-007, FR-008).
7. Confirmar que a remoção em 01:03Z deixou 0 remanescentes (SC-008, FR-009).
8. Confirmar que não há segredo real, que remoto e `pgmq` estão `[NV]`, que cada afirmação tem origem e carimbo e que a evidência do orquestrador é citada como origem, não como reexecução do planejamento (SC-009, FR-010–FR-013).
9. Contar os dez artefatos, verificar ausência de seção vazia, marcador de pendência e portaria posterior a tasks, além de 0 gates, 0 stage/commit/PR/merge e 0 arquivos fora do diretório da feature (SC-010, FR-014, FR-015).

## Abordagem de verificação documental desta etapa

1. Contar artefatos: dez presentes, sem seção vazia e sem marcador de pendência.
2. Conferir rastreabilidade: cada FR mapeado para ao menos um cenário de aceitação e um SC; cada SC com baseline e meta.
3. Conferir R1 e R3 na decisão e nas clarificações: `go` legítimo e três respostas, uma por vez.
4. Conferir o inventário de evidência: valores, saídas e carimbos idênticos aos citados em `research.md`, sem reexecução.
5. Conferir a janela 01:02:36Z–01:03Z como derivação dos carimbos do pedido, sem segundo inventado.
6. Registrar divergências encontradas em `research.md`/`spec.md`; nenhuma é suprimida para produzir aparência de conformidade.

Nenhum comando de gate, build, teste, stage, commit ou efeito remoto é executado nesta feature de planejamento.
