# Tasks: Validação local de Postgres MDMV

**Feature**: `007-postgres-local-validacao` — ver `spec.md`, `plan.md`, `decision.md` (R1 `go`, Opção A) e `checklists/postgres-quality.md`.

**Convenção**: cada tarefa usa `- [ ] T### [P?] [USn?] descrição com caminho`. Este documento termina em tasks: nenhuma seção de implementação, converge, validação ou release aparece aqui. Evidência de execução é citada com origem e data/hora; o que não foi verificado nesta etapa permanece `[NV]`.

## Phase 1: Setup

- [X] T001 [P] Conferir em `specs/007-postgres-local-validacao/spec.md` que todos os FRs e SCs possuem rastreabilidade para ao menos um cenário de aceitação, antes de qualquer registro de evidência (FR-014, SC-010).
- [X] T002 [P] Confirmar em `.specify/feature.json` que o diretório da feature é `specs/007-postgres-local-validacao`, sem usar nome de branch como identidade (FR-014, SC-010; `AGENTS.md`).
- [X] T003 [P] Inventariar em `specs/007-postgres-local-validacao/plan.md` §Inventário de evidência as nove entradas de evidência do pedido (a décima linha da tabela é referência constitucional), com valores, saídas e carimbos idênticos aos do pedido, sem reexecutar comandos (FR-013, SC-009).
- [X] T004 Registrar em `specs/007-postgres-local-validacao/plan.md` o escopo de escrita restrito aos dez artefatos e a ausência de gates, stage, commit, PR e merge nesta etapa (FR-015, SC-010).

## Phase 2: Foundational

- [X] T005 Fixar em `specs/007-postgres-local-validacao/spec.md` e `plan.md` a versão `16.15` como valor lido de `SHOW server_version;`, rastreada ao comando `docker run ... postgres:16-alpine`, sem inferência pela tag (FR-001, SC-001, D1).
- [X] T006 Registrar em `specs/007-postgres-local-validacao/research.md` o `exit 0` de `001_schema.sql` e `002_grants.sql` com `ON_ERROR_STOP=1` em 2026-11-20T01:02:36Z (FR-002, SC-002, D2).
- [X] T007 Registrar em `specs/007-postgres-local-validacao/spec.md` as 2 tabelas, a coluna `payload jsonb` e o índice GIN `idx_module_manifest_payload_gin` como contrato observado, sem afirmar desempenho de índice (FR-003, SC-003, D3).
- [X] T008 Registrar em `specs/007-postgres-local-validacao/plan.md` que a credencial usada é fixture descartável de `tests/postgres/002_grants.sql`, sem segredo real buscado, exibido ou persistido (FR-010, SC-009, D7).
- [X] T009 Registrar em `specs/007-postgres-local-validacao/spec.md` e `research.md` que `pgmq`, Postgres remoto, Coolify e Zitadel permanecem `[NV]`, inclusive credenciais (FR-011, FR-012, SC-009, D8, D9).

## Phase 3: User Story 1 — Versão observada e migrações aplicadas (Priority: P1)

**Goal**: provar qual servidor respondeu e que as migrações versionadas aplicaram integralmente.
**Independent Test**: ler E1 e E2 em `research.md`/`plan.md` e conferir `16.15` com carimbo 01:02:01Z e `exit 0` das duas migrações com carimbo 01:02:36Z.

- [X] T010 [US1] Verificar em `specs/007-postgres-local-validacao/research.md` que E1 traz `docker run ... postgres:16-alpine`, a saída `16.15` de `SHOW server_version;` e o carimbo 2026-11-20T01:02:01Z, sem URL com userinfo ou credencial real (FR-001, SC-001, SC-009).
- [X] T011 [US1] Conferir em `specs/007-postgres-local-validacao/plan.md` §Inventário de evidência que as entradas 2 e 3 registram `exit 0` para `001_schema.sql` e `002_grants.sql` sob `ON_ERROR_STOP=1`, com o mesmo carimbo citado em `research.md` (FR-002, SC-002, D2).
- [X] T012 [US1] Registrar em `specs/007-postgres-local-validacao/spec.md` o cenário de migração com erro como falha explícita, sem afirmar smoke, grants ou teste negativo a partir daquele container (FR-002, US1/AC4, Edge Cases).
- [X] T013 [P] [US1] Revisar `specs/007-postgres-local-validacao/checklists/postgres-quality.md` para confirmar que versão observada e migrações são tratadas como requisitos de qualidade, não como execução de comandos (FR-001, FR-002, SC-001, SC-002).

## Phase 4: User Story 2 — Smoke autenticado: inserção e JSONB (Priority: P1)

**Goal**: registrar a inserção e o predicado `@>` verdadeiro no papel de aplicação.
**Independent Test**: ler E3 em `research.md` e conferir `INSERT 0 1`, `tem_pilares = t` e o operador `@>` com o dado `'{"pilares":["soberania"]}'`.

- [X] T014 [US2] Conferir em `specs/007-postgres-local-validacao/research.md` que E3 registra a execução como `psql -U insurgencia_app -f tests/postgres/003_smoke.sql` e a saída `INSERT 0 1` para `module_manifest` (FR-004, SC-004).
- [X] T015 [US2] Conferir em `specs/007-postgres-local-validacao/spec.md` que o predicado `payload @> '{"pilares":["soberania"]}'::jsonb` aparece como verdadeiro, com `tem_pilares = t` na saída (FR-005, SC-005).
- [X] T016 [US2] Registrar em `specs/007-postgres-local-validacao/plan.md` que o índice GIN é registrado como objeto aplicado, sem converter a migração em afirmação de desempenho de consulta (FR-003, D3).
- [X] T017 [US2] Registrar em `specs/007-postgres-local-validacao/spec.md` que inserção falha ou `@>` falso são não conformidade, sem ajuste do dado de teste para forçar verdadeiro (FR-005, US2/AC4, Edge Cases).
- [X] T018 [P] [US2] Verificar em `specs/007-postgres-local-validacao/checklists/requirements.md` que o predicado JSONB possui item de qualidade rastreável a FR-005 e SC-005 (FR-014, SC-010).

## Phase 5: User Story 3 — Privilégio mínimo e teste negativo (Priority: P1)

**Goal**: provar que `insurgencia_app` tem apenas SELECT/INSERT e que UPDATE é recusado.
**Independent Test**: ler E3 e E4 em `research.md`, contar 4 linhas de grants e conferir `ERROR: permission denied` com `exit 1`.

- [X] T019 [US3] Conferir em `specs/007-postgres-local-validacao/research.md` que E3 registra 4 linhas em `information_schema.role_table_grants` para `insurgencia_app`: SELECT e INSERT em `module_manifest` e `evidence` (FR-006, SC-006).
- [X] T020 [US3] Registrar em `specs/007-postgres-local-validacao/spec.md` que nenhum privilégio de UPDATE, DELETE ou TRUNCATE aparece para o papel de aplicação nas duas tabelas (FR-007, SC-006).
- [X] T021 [US3] Conferir em `specs/007-postgres-local-validacao/research.md` que E4 registra `UPDATE module_manifest ...` como `insurgencia_app`, `ERROR: permission denied for table module_manifest` e `exit 1` (FR-008, SC-007).
- [X] T022 [US3] Registrar em `specs/007-postgres-local-validacao/spec.md` que UPDATE permitido é falha da validação de privilégio mínimo, sem reinterpretar exit 0 como sucesso (FR-008, US3/AC4, Edge Cases).
- [X] T023 [P] [US3] Registrar em `specs/007-postgres-local-validacao/checklists/postgres-quality.md` que a contagem de grants e a negação de UPDATE são critérios de qualidade dos requisitos, não teste de execução (SC-006, SC-007, Checklist §Acceptance Criteria Quality).

## Phase 6: User Story 4 — Teardown, isolamento e rastreabilidade (Priority: P2)

**Goal**: provar que o ambiente foi removido e que os artefatos estão completos e rotulados.
**Independent Test**: ler E5 em `research.md` e contar artefatos, afirmações com origem/carimbo e rótulos `[NV]` em `specs/007-postgres-local-validacao/`.

- [X] T024 [US4] Conferir em `specs/007-postgres-local-validacao/research.md` que E5 registra `docker rm -f` em 2026-11-20T01:03Z e contagem 0 de remanescentes (FR-009, SC-008).
- [X] T025 [US4] Revisar `specs/007-postgres-local-validacao/spec.md` e `plan.md` e confirmar que a credencial é exclusivamente fixture local não real (em `tests/postgres/002_grants.sql` e em E1 como variável do container descartável), sem credencial de Postgres remoto, Coolify ou Zitadel (FR-010, SC-009, D7).
- [X] T026 [US4] Confirmar em `specs/007-postgres-local-validacao/spec.md`, `research.md`, `decision.md` e `plan.md` que `pgmq`, Postgres remoto, Coolify e Zitadel aparecem somente como `[NV]`, sem afirmação de validação (FR-011, FR-012, SC-009, Clarificações Q1 e Q2).
- [X] T027 [P] [US4] Auditar os dez artefatos em `specs/007-postgres-local-validacao/` para contar 10 de 10 presentes, sem seção vazia, sem marcador de pendência, com checklist `[ ]` e tarefas `T###` (FR-014, SC-010).
- [X] T028 [US4] Confirmar em `specs/007-postgres-local-validacao/plan.md` que nenhum gate foi executado, nenhum arquivo fora do diretório da feature foi escrito e nenhum stage, commit, PR ou merge foi realizado nesta etapa (FR-015, SC-010, D11).

## Phase 7: Polish & cross-cutting concerns

- [X] T029 Revisar os carimbos em `specs/007-postgres-local-validacao/research.md`, `spec.md` e `plan.md` para manter 01:02:01Z, 01:02:36Z e 01:03Z idênticos aos do pedido, sem arredondamento ou substituição por data de reexecução (FR-013, SC-001, SC-002, SC-008).
- [X] T030 Confirmar em `specs/007-postgres-local-validacao/decision.md` que o veredito `go` registra evidência `strong`, opção recomendada nomeada e nenhum `unknown` glossado, sem antecipar implementação (R1, FR-015).
- [X] T031 Verificar em `specs/007-postgres-local-validacao/spec.md` que as três clarificações da Session 2026-11-20 permanecem uma por vez, no formato `- Q: … → A: …`, com R3 respeitado (R3, SC-010).
- [X] T032 Conferir em `specs/007-postgres-local-validacao/tasks.md` que o documento termina na Phase 7, com tarefas `- [ ] T###` e sem seção de implementação, converge, validação ou release (FR-015, SC-010).

## Dependencies

A ordem de execução é a numeração das tarefas: T001–T009 preparam o terreno e fixam as evidências; T010–T013 dependem do registro de versão e migrações; T014–T018 dependem do smoke autenticado; T019–T023 dependem dos grants e do teste negativo; T024–T028 dependem do inventário completo para auditar teardown, isolamento e rastreabilidade; T029–T032 são acabamento. Não há arestas circulares nem referências a identificadores não declarados.

## Parallel execution examples

- T001, T002 e T003 podem ser executadas em paralelo (arquivos distintos: `spec.md`, `.specify/feature.json` e `plan.md`).
- T013 e T014 podem ser executadas em paralelo após T006 (checklist de qualidade e leitura de E3 em arquivos distintos).
- T018 e T023 podem ser executadas em paralelo após T015 e T021 (dois checklists distintos).
- T027 e T028 podem ser executadas em paralelo após T026 (contagem de artefatos e auditoria de escopo).

## Implementation strategy

Incremento mínimo: T001–T011 entregam versão e migrações documentadas; T014–T018 fecham o smoke com JSONB; T019–T023 entregam o privilégio mínimo com teste negativo; T024–T028 garantem teardown, rotulagem `[NV]` e rastreabilidade; T029–T032 preservam carimbos, R1, R3 e o término em tasks. Nenhuma tarefa afirma reexecução de comandos, validação remota de Postgres/Coolify/Zitadel, teste de `pgmq` ou portaria posterior a tasks; essas ações pertencem a fases seguintes e permanecem `[NV]` até comando executado com saída colada e data/hora.
