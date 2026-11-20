# Idea Intake: Validação local de Postgres MDMV

- **Slug**: postgres-local-validacao
- **Created**: 2026-11-20
- **Source**: pedido do mantenedor nesta sessão (texto autorizado); evidência de execução do orquestrador com carimbos entre 2026-11-20T01:02:01Z e 2026-11-20T01:03Z; arquivos locais `tests/postgres/001_schema.sql`, `tests/postgres/002_grants.sql` e `tests/postgres/003_smoke.sql`; padrão registrado em `docs/AVALIACAO-STACK-MDMV.md`
- **Type**: exploration

## Idea (as captured)

> "Objeto (autorizado): validar padrão Postgres MDMV em container local descartável, sem segredo real. Evidência do orquestrador a citar em research.md (com comando+saída+data/hora): 2026-11-20T01:02:01Z `docker run ... postgres:16-alpine` → PostgreSQL 16.15; 01:02:36Z migrações `001_schema.sql`/`002_grants.sql` exit 0; smoke como `insurgencia_app` → INSERT 0 1, JSONB `@>` verdadeiro, 4 grants (SELECT/INSERT em 2 tabelas); teste negativo UPDATE → `ERROR: permission denied` exit 1; 01:03Z container removido (0 remanescentes). Restrições: NÃO é Postgres remoto/Coolify/Zitadel (credenciais fora do alvo, [NV]); senha é throwaway local; pgmq não testado (extensão não verificada); Zitadel/Coolify remotos ficam [NV]. Execute EM ORDEM intake→research→define→shape→decide(go)→specify→clarify→plan→checklist→tasks, FRs/SCs mensuráveis, checklist `[ ]`, T### sem portarias posteriores."

> Aceitação solicitada: dez artefatos, sem seção vazia e sem marcador de pendência; R1 `go`.

## Restated

Propor que o padrão Postgres do hub MDMV — migrações versionadas, JSONB com índice GIN e privilégio mínimo via papel de aplicação — seja verificado em um container local descartável, usando apenas a credencial de fixture já versionada nos arquivos de teste. A proposta é de exploração e verificação: ela registra o que foi observado (versão do servidor, aplicação das migrações, smoke de leitura/escrita, negação de UPDATE e remoção do container) com comando, saída e data/hora, sem tocar Postgres remoto, Coolify, Zitadel ou pgmq.

## Origin & Context

- **Raised by**: mantenedor `lfgranja`, pedido explícito nesta sessão (2026-11-20).
- **Trigger**: necessidade de substituir a afirmação documental do padrão Postgres (registrada em `docs/AVALIACAO-STACK-MDMV.md` como `PgPool`/`sqlx`/migrations/JSONB-GIN, com `pgmq` diferido) por evidência local auditável, sem usar segredo real nem depender de serviço remoto. Os comandos já foram executados pelo orquestrador e entregues como fatos a citar.
- **Contexto de localização**: os artefatos de assess e de entrega ficam colocalizados em `specs/007-postgres-local-validacao/`, conforme o escopo de escrita da atribuição. Nenhum arquivo fora desse diretório é escrito nesta etapa de planejamento.
- **Limites de autoridade**: a subida de container descartável, a aplicação das migrações de teste, o smoke e o teste negativo foram autorizados; usar credencial real, validar Postgres remoto/Coolify/Zitadel, afirmar pgmq testado e escrever portaria posterior a tasks não foram autorizados e estão explicitamente vedados.

## First-Glance Unknowns

- Resolvido (dado do pedido): `postgres:16-alpine` → PostgreSQL 16.15 em 2026-11-20T01:02:01Z; `001_schema.sql`/`002_grants.sql` exit 0 em 01:02:36Z; smoke como `insurgencia_app` com `INSERT 0 1`, `@>` verdadeiro e 4 grants; UPDATE negado com `ERROR: permission denied` exit 1; container removido em 01:03Z com 0 remanescentes; dez artefatos; R1 `go`; ordem obrigatória até tasks.
- Como rotular o escopo para não induzir leitura de que Postgres remoto, Coolify ou Zitadel foram validados? (respondido em `spec.md` / `## Clarifications` Q1)
- Como tratar pgmq e a extensão citada pela migração diante da evidência disponível? (respondido em `spec.md` / `## Clarifications` Q2)
- Como tratar a evidência já obtida pelo orquestrador nesta etapa de planejamento? (respondido em `spec.md` / `## Clarifications` Q3)
