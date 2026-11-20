# Concept: Validação local de Postgres MDMV

- **Slug**: postgres-local-validacao
- **Created**: 2026-11-20
- **Recommended option**: Opção A — validação local descartável com migrações, smoke e teste negativo

## Options

Três opções são consideradas, da prova local completa à ausência de execução e à validação remota; a recomendação é nomeada ao final com base nos objetivos de `problem.md`.

### Option A — Validação local descartável com migrações, smoke e teste negativo

- **Sketch**: um container Postgres descartável é iniciado a partir de `postgres:16-alpine`, a versão é lida do servidor em execução, as duas migrações versionadas são aplicadas e o smoke roda autenticado como `insurgencia_app`. A saída mostra a inserção, o predicado JSONB verdadeiro e os quatro grants; um UPDATE posterior é recusado com erro de permissão. Ao final, o container é removido e a contagem de remanescentes é zero. Quem lê o resultado consegue provar o que foi executado, quando e com que privilégio, sem tocar ambiente remoto.
- **Appetite**: small — minutos de execução com comandos já rodados pelo orquestrador e três arquivos já versionados; sem desenvolvimento de produto e sem dependência de serviço externo.
- **Trade-offs**: ganha prova auditável de versão, migração, JSONB/GIN, privilégio mínimo e teardown; sacrifica cobertura de deployment real, réplicas e persistência. Riscos: a credencial de fixture poderia ser confundida com segredo real, e o resultado local poderia ser lido como validação de Postgres remoto ou de `pgmq`.
- **Rabbit holes**: apontar o smoke para um Postgres remoto "para ser mais realista"; substituir a credencial de fixture por segredo verdadeiro; estender o teste a `pgmq`, `PgPool` ou réplicas para "completar" a validação; transformar o `exit 0` das migrações em garantia de produção.

### Option B — Revisão estática dos arquivos de teste, sem execução

- **Sketch**: ler `001_schema.sql`, `002_grants.sql` e `003_smoke.sql` e descrever o que eles declaram, sem iniciar container nem registrar saída de execução.
- **Appetite**: small — esforço menor que a Opção A.
- **Trade-offs**: ganha rapidez e zero efeito no ambiente; não prova que a sintaxe é aceita pelo servidor 16.15, que o operador `@>` responde ou que o UPDATE é recusado — justamente os pontos em questão. Risco: descrição correta do código ser tomada como validação executada.
- **Rabbit holes**: substituir comando e saída por interpretação de intenção e apresentar a leitura dos arquivos como evidência de comportamento.

### Option C — Validar diretamente Postgres remoto, Coolify ou Zitadel

- **Sketch**: executar as migrações e o smoke contra instância remota já existente, usando suas credenciais.
- **Appetite**: large — depende de ambiente externo, credenciais fora do alvo e autorização que não foi dada.
- **Trade-offs**: ganha proximidade com produção; sacrifica auditabilidade local, descartabilidade e segurança, além de ultrapassar o limite de autoridade do pedido. Risco: vazamento de segredo e afirmação de verde sobre serviço não observado nesta etapa.
- **Rabbit holes**: buscar credenciais em repositório ou cofre, executar Docker em host remoto, tratar resposta de endpoint externo como confirmação sem comando executado no alvo.

## Recommendation

Opção A. Ela atende diretamente aos objetivos de maior valor: provar localmente, com comando, saída e carimbo, que o padrão Postgres MDMV funciona na versão fixada e que o papel de aplicação não pode UPDATE. É a opção com menor superfície de erro — um container, duas migrações, um smoke, um teste negativo e um teardown — e preserva a hierarquia de autoridade do hub: a execução local descartável é autorizada, enquanto credenciais reais, serviços remotos e pgmq não são. A Opção B é preterida porque descreve intenção sem produzir prova; a Opção C viola o limite de autoridade e introduz segredo fora do alvo. Nenhuma opção inclui validação de pgmq, Postgres remoto, Coolify ou Zitadel, nem portaria posterior a tasks.

## Out of Scope (for the recommended option)

- Postgres remoto, Coolify e Zitadel, inclusive credenciais; todos permanecem `[NV]`.
- Uso de segredo real, token ou credencial de produção em qualquer artefato, log ou mensagem.
- `pgmq` e verificação funcional de quaisquer extensões além da aceitação da instrução `CREATE EXTENSION` pela migração.
- Validação de `PgPool`/`sqlx`, réplicas, volumes persistentes ou deployment.
- Alteração de `tests/postgres/`, de código de produto ou de esquema versionado.
- Execução de gates, stage, commit, PR, merge ou reescrita de histórico nesta etapa de planejamento.
- Portarias posteriores a tasks e escrita de arquivos fora de `specs/007-postgres-local-validacao/`.

## Assumptions to Validate

- A evidência obtida pelo orquestrador é o registro fidedigno desta rodada: `16.15` em 2026-11-20T01:02:01Z, `exit 0` das migrações em 01:02:36Z, `INSERT 0 1`/`@>` verdadeiro/4 grants e UPDATE negado na janela 01:02:36Z–01:03Z, remoção com 0 remanescentes em 01:03Z.
- A senha em `tests/postgres/002_grants.sql` é valor de fixture descartável, já versionado; nenhuma credencial real é necessária ou usada.
- A tag `postgres:16-alpine` resolveu para o servidor 16.15 no momento da execução; a versão afirmada é a lida do servidor, não a da tag.
- `pgcrypto` teve sua criação aceita pela migração com `exit 0`; nenhuma função da extensão foi exercitada, e `pgmq` não foi testado.
- O que não foi executado nesta etapa permanece rotulado `[NV]`; reexecução e portarias R4/R7 pertencem a fases posteriores, ausentes destes artefatos.
