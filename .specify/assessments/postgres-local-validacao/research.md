# Idea Research: Validação local de Postgres MDMV

- **Slug**: postgres-local-validacao
- **Created**: 2026-11-20
- **Evidence confidence (overall)**: high — versão do servidor, aplicação das migrações, smoke, teste negativo e remoção do container foram obtidos por comandos com carimbos de data/hora e são citados abaixo com comando e saída. Nenhuma afirmação central repousa em `ASSUMPTION`; pgmq, Postgres remoto, Coolify e Zitadel não foram observados e permanecem `[NV]`, não usados como evidência a favor.

## Users & Demand

- O mantenedor `lfgranja` autorizou a validação e precisa de uma prova local e reproduzível de que o padrão Postgres do hub funciona na versão fixada, antes de depositá-lo em documentação ou infraestrutura. A demanda é explícita no pedido desta sessão, não inferida. [source: pedido do mantenedor, 2026-11-20] (confidence: high)
- Auditores e revisores precisam de comando, saída e data/hora para aceitar cada afirmação como verde; o par "migração exit 0" + "UPDATE negado" é o que fecha a cadeia de evidência do privilégio mínimo. [source: evidência do orquestrador 2026-11-20T01:02:36Z e teste negativo subsequente] (confidence: high)
- Quem for implementar o adaptador de persistência com `sqlx`/`PgPool` usa o resultado como referência de que JSONB `@>`, o índice GIN e os grants funcionam juntos; sem isso, a compatibilidade seria suposição. [source: `docs/AVALIACAO-STACK-MDMV.md:270`; `tests/postgres/003_smoke.sql:2-7`] (confidence: high)

## Prior Art

- O padrão Postgres MDMV já está registrado no alvo como intenção: `PgPool`, `query!`, `apps/server/migrations/`, JSONB/GIN, `pgmq` "quando aplicáveis", sempre como adaptador trocável; a auditoria Aroeira registra nenhuma ocorrência de `pgmq` em `.rs`/`.sql` e classifica N28/N29/N35 como "NÃO APLICÁVEL AINDA". [source: `docs/AVALIACAO-STACK-MDMV.md:64,97,108,270-271`] (confidence: high)
- Já existem três arquivos de teste versionados que materializam o padrão: `001_schema.sql` (pgcrypto, `module_manifest` com `payload jsonb` e índice GIN, `evidence`), `002_grants.sql` (papel `insurgencia_app`, REVOKE CREATE no schema public, GRANT SELECT/INSERT nas duas tabelas e na sequência, REVOKE UPDATE/DELETE/TRUNCATE) e `003_smoke.sql` (INSERT, `@>` e consulta a `information_schema.role_table_grants`). [source: leitura local dos três arquivos, 2026-11-20] (confidence: high)
- A spec 006 estabeleceu o padrão do hub para evidência auditável: comando executado, saída colada e data/hora, com rótulo `[NV]` até confirmação ao vivo, além do conjunto de dez artefatos de planejamento terminando em tasks. [source: `specs/006-godot-headless-benchmark/research.md:9-40`; `docs/governance/sdd-phase-contracts.md:24-27`] (confidence: high)
- Não há registro anterior de execução de Postgres no alvo; os arquivos em `tests/postgres/` são o único precedente local, e nenhum resultado de execução estava versionado até esta rodada. [source: `docs/AVALIACAO-STACK-MDMV.md:64,97`; listagem de `tests/postgres/`, 2026-11-20] (confidence: high)

- A alternativa atual é manter o padrão apenas como documento: qualquer regressão em grants, em sintaxe de migração ou no operador `@>` só apareceria quando o adaptador real fosse escrito. O custo da inação é a ausência de prova local e o risco de um grau de privilégio maior que o pretendido passar despercebido. [source: `docs/AVALIACAO-STACK-MDMV.md:270`; pedido do mantenedor] (confidence: high)

- Usar um container local descartável com imagem `postgres:16-alpine` é a alternativa a apontar para um Postgres remoto: permite provar a versão exata e destruir o ambiente ao final, sem tocar credenciais de produção nem serviços externos. O valor registrado `16.15` veio do servidor em execução, não da tag da imagem. [source: E1, 2026-11-20T01:02:01Z] (confidence: high)
- A credencial em `002_grants.sql` é valor de fixture descartável, já versionado no arquivo de teste; ela não é segredo real e nenhuma credencial de Postgres remoto, Coolify ou Zitadel entra nesta validação. [source: `tests/postgres/002_grants.sql:2`; pedido do mantenedor] (confidence: high)

## Data & Constraints

| Fato observado | Valor registrado | Carimbo / origem |
|---|---|---|
| Imagem e versão | `docker run ... postgres:16-alpine` → `SHOW server_version;` = `16.15` | E1, 2026-11-20T01:02:01Z |
| Migração de schema | `tests/postgres/001_schema.sql` aplicada com `exit 0` (inclui `CREATE EXTENSION IF NOT EXISTS pgcrypto`, 2 tabelas e índice GIN) | E2, 2026-11-20T01:02:36Z |
| Migração de grants | `tests/postgres/002_grants.sql` aplicada com `exit 0` (papel `insurgencia_app`, 4 grants de tabela, 3 REVOKEs) | E2, 2026-11-20T01:02:36Z |
| Inserção | smoke como `insurgencia_app` → `INSERT 0 1` em `module_manifest` | E3, janela 01:02:36Z–01:03Z |
| JSONB | `payload @> '{"pilares":["soberania"]}'::jsonb` → linha devolvida, `tem_pilares = t` | E3, janela 01:02:36Z–01:03Z |
| Privilégios | `information_schema.role_table_grants` para `insurgencia_app` → 4 linhas: SELECT/INSERT em `module_manifest` e `evidence` | E3, janela 01:02:36Z–01:03Z |
| Teste negativo | `UPDATE module_manifest ...` como `insurgencia_app` → `ERROR: permission denied for table module_manifest`, `exit 1` | E4, janela 01:02:36Z–01:03Z |
| Teardown | `docker rm -f` do container e contagem 0 com `docker ps -a --filter name=... -q \| wc -l` | E5, 2026-11-20T01:03Z |
| Fora de escopo | `pgmq` não testado (extensão não verificada); Postgres remoto/Coolify/Zitadel com credenciais fora do alvo | pedido do mantenedor → `[NV]` |
| Governo do alvo | determinismo (I), fluxo SDD ordenado (III), verificação honesta com `[NV]` até confirmação ao vivo (VI) | `.specify/memory/constitution.md:7-25` |

- A janela 01:02:36Z–01:03Z é derivada dos carimbos do pedido: smoke e teste negativo ocorreram depois da aplicação das migrações e antes da remoção; nenhum carimbo mais fino foi fornecido, então nenhum segundo é inventado.
- A migração `001_schema.sql` cria `pgcrypto` com `CREATE EXTENSION IF NOT EXISTS`; o `exit 0` prova que a instrução foi aceita pelo servidor, mas nenhuma função de `pgcrypto` foi exercitada. Afirmar funcionamento funcional de `pgcrypto` seria over-claim. [source: `tests/postgres/001_schema.sql:2`; E2] (confidence: high)
- Escopo de escrita desta etapa: somente os dez artefatos em `specs/007-postgres-local-validacao/`; nenhum gate é executado, nenhum stage ou commit é feito e nenhuma portaria posterior a tasks é escrita. [source: atribuição do mantenedor, 2026-11-20] (confidence: high)

### Evidência bruta (comando + saída + carimbo)

Os cinco blocos abaixo substituem abreviações de origem: comando executado, saída observada e carimbo aparecem juntos. Nenhum bloco foi reexecutado nesta etapa; todos são citação da evidência do orquestrador.

**E1 — subida do container e versão do servidor (2026-11-20T01:02:01Z)**

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

Versão lida do servidor em execução; a tag `16-alpine` identifica a imagem, não a versão afirmada. A senha exibida é valor de fixture do arquivo versionado, não credencial real.

**E2 — aplicação das migrações (2026-11-20T01:02:36Z)**

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

As duas migrações terminaram com `exit 0` sob `ON_ERROR_STOP=1`; nenhuma instrução foi ignorada silenciosamente.

**E3 — smoke autenticado como `insurgencia_app` (janela 01:02:36Z–01:03Z)**

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

`INSERT 0 1` confirma uma linha inserida; `tem_pilares = t` confirma o predicado `payload @> '{"pilares":["soberania"]}'::jsonb`; as quatro linhas confirmam SELECT/INSERT nas duas tabelas e nenhum privilégio de UPDATE/DELETE/TRUNCATE.

**E4 — teste negativo de privilégio (janela 01:02:36Z–01:03Z)**

```
$ docker exec insurgencia_pg_local_validacao \
    psql -U insurgencia_app -d insurgencia_validacao \
    -c "UPDATE module_manifest SET version = '2.0.0' WHERE id = 'insurgencia.exemplo';"
ERROR:  permission denied for table module_manifest
$ echo $?
1
```

O UPDATE foi recusado e o processo terminou com `exit 1`: a negação é o resultado esperado, não uma falha da validação.

**E5 — remoção do container (2026-11-20T01:03Z)**

```
$ docker rm -f insurgencia_pg_local_validacao
insurgencia_pg_local_validacao
$ docker ps -a --filter name=insurgencia_pg_local_validacao -q | wc -l
0
```

Zero remanescentes: o ambiente é descartável e não deixa estado no alvo.

## Evidence Against the Idea

- A validação cobre apenas o que foi executado localmente. `pgmq` não foi testado e a extensão não foi verificada; apresentar esta rodada como prova de filas duráveis seria conclusão inválida. Mitigação: manter `pgmq` como `[NV]` e vedar afirmação em sentido contrário. [source: pedido do mantenedor; `docs/AVALIACAO-STACK-MDMV.md:64,97`] (confidence: high)
- Postgres remoto, Coolify e Zitadel não foram tocados; suas credenciais ficam fora do alvo. Mitigação: rótulo `[NV]` para todos eles e nenhum uso de segredo real em qualquer artefato. [source: pedido do mantenedor] (confidence: high)
- O `exit 0` das migrações prova que `CREATE EXTENSION IF NOT EXISTS pgcrypto` foi aceito, não que funções de `pgcrypto` funcionam. Mitigação: registrar somente o que foi observado e tratar o restante como não testado. [source: `tests/postgres/001_schema.sql:2`; E2] (confidence: high)
- A evidência vem de uma única sessão, em uma única máquina, e não foi reexecutada nesta etapa de planejamento. Afirmar que o planejamento "rodou" os comandos seria desonesto. Mitigação: citar carimbos e deixar reexecução e validação para as fases próprias. [source: atribuição do mantenedor; `docs/governance/sdd-phase-contracts.md:24-27`] (confidence: high)
- Um container descentralizado e descartável não prova comportamento de produções com réplicas, persistência de volume ou conexão via `PgPool`/`sqlx`; ele prova apenas o padrão SQL e de privilégio no servidor 16.15. Mitigação: limitar o escopo ao que foi medido e não converter o resultado em garantia de deployment. [source: E1–E5; `docs/AVALIACAO-STACK-MDMV.md:270`] (confidence: high)

## Gaps & Open Questions

- Como rotular o escopo para não induzir leitura de que Postgres remoto, Coolify ou Zitadel foram validados? (respondido em `spec.md` / `## Clarifications` Q1)
- Como tratar pgmq e a extensão citada pela migração diante da evidência disponível? (respondido em `spec.md` / `## Clarifications` Q2)
- Como tratar a evidência já obtida pelo orquestrador nesta etapa de planejamento? (respondido em `spec.md` / `## Clarifications` Q3)

## Sources

- Evidência de execução do orquestrador transcrita em E1–E5 (política: dado do pedido, não buscado na web): versão `16.15` em 2026-11-20T01:02:01Z (E1); `exit 0` das duas migrações em 2026-11-20T01:02:36Z (E2); `INSERT 0 1`, `@>` verdadeiro e 4 grants na janela 01:02:36Z–01:03Z (E3); `ERROR: permission denied` com `exit 1` no UPDATE na mesma janela (E4); container removido com 0 remanescentes em 2026-11-20T01:03Z (E5). Nenhuma credencial real, parâmetro assinado ou URL com userinfo foi persistido.
- Arquivos locais do alvo (política: `local-only`): `tests/postgres/001_schema.sql`, `tests/postgres/002_grants.sql`, `tests/postgres/003_smoke.sql`, `docs/AVALIACAO-STACK-MDMV.md`, `docs/governance/sdd-phase-contracts.md`, `.specify/memory/constitution.md`, `specs/006-godot-headless-benchmark/research.md`.
- `docs/AVALIACAO-STACK-MDMV.md` (política: `local-only`; nenhuma URL externa foi buscada): registro do padrão `PgPool`/`sqlx`/migrations/JSONB-GIN e do diferimento de `pgmq`.
