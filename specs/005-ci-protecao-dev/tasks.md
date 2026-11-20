# Tasks: CI de gates determinísticos e proteção de dev

**Feature**: `005-ci-protecao-dev` — ver `spec.md`, `plan.md`, `decision.md` (R1 `go`, Opção A) e `checklists/ci-quality.md`.

**Convenção**: cada tarefa usa `- [ ] T### [P?] [USn?] descrição com caminho`. Nenhuma fase posterior a tasks é incluída neste documento. Afirmações de CI, check, merge e proteção permanecem `[NV]` até comando executado com saída colada e data/hora.

## Phase 1: Setup

- [X] T001 [P] Criar o arquivo `.github/workflows/ci-gates.yml` com os quatro gatilhos (pull request para `dev`, pull request para `main`, push para `dev`, push para `main`), conforme `specs/005-ci-protecao-dev/plan.md` §Matriz de gatilhos e FR-001/SC-001.
- [X] T002 [P] Declarar em `.github/workflows/ci-gates.yml` o menor conjunto de permissões necessário e nenhuma variável de segredo, credential ou token, conforme FR-011/SC-010 e `.specify/memory/constitution.md:24`.
- [X] T003 [P] Conferir em `specs/005-ci-protecao-dev/spec.md` que todos os FRs e SCs possuem rastreabilidade para pelo menos um cenário de aceitação, antes de tocar em qualquer configuração remota (FR-014, SC-009).

## Phase 2: Foundational

- [X] T004 Adicionar ao arquivo `.github/workflows/ci-gates.yml` os cinco passos de verificação na ordem de `AGENTS.md:16-22` — `check_sdd_completeness.py`, `check_work_order_dag.py`, `check_task_id_uniqueness.py`, `check_converge_append_only.py`, `hub_docs_gate.py` — conforme FR-002 e §Inventário de gates de `specs/005-ci-protecao-dev/plan.md`.
- [X] T005 Garantir em `.github/workflows/ci-gates.yml` que código de saída diferente de zero em qualquer um dos cinco gates falha a verificação, sem `continue-on-error`, tolerância a falha ou resultado verde artificial, conforme FR-003/SC-007 (resultado real `[NV]` até execução).
- [X] T006 Confirmar em `.github/workflows/ci-gates.yml` que nenhuma etapa de publicação, release, deploy ou transferência de artefato externo e nenhuma referência a `.forgejo/**`, `fj`, `check_production_gate.py --check`, `pytest -q`, `verify.sh` ou `nextest` foi introduzida, conforme FR-008/FR-009/SC-006.
- [X] T007 Confirmar em `docs/governance/*-baseline.json` e `scripts/ci/*.py` que nenhum gate, baseline ou documento de governança foi alterado para produzir verde, conforme FR-010/SC-007; toda divergência é registrada, não suprimida.
- [X] T008 Confirmar por leitura que a premissa de visibilidade pública já está registrada em `specs/005-ci-protecao-dev/research.md` com a evidência de 2026-11-20T00:26:47Z e a leitura via `gh api` de 2026-11-20T00:29Z, mantendo `[NV]` o efeito ainda não observado na API de proteção (FR-007, SC-008).

## Phase 3: User Story 1 — Gates determinísticos verificam PR e push (Priority: P1)

**Goal**: toda mudança em `dev`/`main` passa pelos cinco gates, com sinal explícito de sucesso ou falha.
**Independent Test**: um PR para `dev` com mudança íntegra apresenta cinco resultados `PASS`; estado `[NV]` até execução ao vivo.

- [X] T009 [US1] Abrir ou atualizar um PR com base `dev` e confirmar em `specs/005-ci-protecao-dev/plan.md` o registro do gatilho observado, com comando, saída e data/hora; check real `[NV]` até a confirmação (FR-001, FR-013, SC-001).
- [X] T010 [US1] Coletar e colar em `specs/005-ci-protecao-dev/plan.md` a saída dos cinco gates executados no PR para `dev`, verificando cinco resultados `PASS`, ou registrar cada falha sem supressão; estado `[NV]` antes da execução (FR-002, FR-003, SC-002).
- [X] T011 [P] [US1] Repetir a verificação de gatilho e resultados para um PR com base `main`, registrando a saída em `specs/005-ci-protecao-dev/plan.md` e confirmando ausência de etapa de publicação (FR-001, FR-008, SC-001, SC-006).
- [X] T012 [P] [US1] Verificar em `specs/005-ci-protecao-dev/plan.md` os gatilhos de push para `dev` e `main`, registrando comando, saída e data/hora; estado `[NV]` até cada push observado (FR-001, SC-001).

## Phase 4: User Story 2 — Nome real do check observado ao vivo (Priority: P1)

**Goal**: o nome do status check vem de execução real e é registrado com prova.
**Independent Test**: listar os checks do PR e registrar o nome exibido com comando, saída e data/hora.

- [X] T013 [US2] Listar os checks do PR da história 1 e capturar o nome exato exibido, registrando em `specs/005-ci-protecao-dev/plan.md` o comando, a saída e a data/hora; nenhum nome é inferido do YAML (FR-004, FR-013, SC-003).
- [X] T014 [US2] Registrar em `specs/005-ci-protecao-dev/research.md` a ressalva de que o nome capturado vale para aquela execução e deve ser reobservado se o workflow for renomeado, conforme Edge Cases e SC-003 (FR-007).
- [X] T015 [US2] Caso nenhum check apareça na listagem, registrar a ausência como `[NV]` em `specs/005-ci-protecao-dev/plan.md` e não prosseguir para a configuração de proteção (FR-004, SC-003).

## Phase 5: User Story 3 — Proteção de dev com required status check verificado (Priority: P1)

**Goal**: `dev` exige exatamente o check observado, com prova por API.
**Independent Test**: a leitura da API devolve a regra com o nome observado; sem isso, a proteção permanece `[NV]`.

- [X] T016 [US3] Obter autorização explícita para a mutação remota e registrar em `specs/005-ci-protecao-dev/decision.md` que a criação do workflow não implica autorização de proteção, conforme FR-005 e `AGENTS.md:7-9`.
- [X] T017 [US3] Configurar, em ato separado posterior à observação do nome, a proteção da branch `dev` para exigir exatamente o status check registrado em T013, sem alterar outras branches (FR-005, SC-004).
- [X] T018 [US3] Ler a configuração de proteção de `dev` pela API e colar a resposta com data/hora em `specs/005-ci-protecao-dev/plan.md`, confirmando o nome exigido; se a API recusar, registrar a limitação e manter `[NV]` (FR-006, SC-005).
- [X] T019 [US3] Registrar em `specs/005-ci-protecao-dev/research.md` o resultado da tentativa de merge com check falho, se observado ao vivo; antes disso, manter o cenário US3/AC4 como `[NV]` (FR-007, SC-008).

## Phase 6: User Story 4 — Salvaguardas e auditoria (Priority: P2)

**Goal**: provar que a mudança não corrompeu o próprio mecanismo de verificação.
**Independent Test**: contar 0 deploy, 0 Forgejo, 0 mudança de gates/baselines/governança, 100% de `[NV]` não confirmado e 10 de 10 artefatos.

- [X] T020 [US4] Auditar `.github/workflows/ci-gates.yml` e `specs/005-ci-protecao-dev/*.md` para confirmar zero etapas de publicação/deploy e zero referências a `.forgejo/**`, registrando o resultado em `specs/005-ci-protecao-dev/checklists/ci-quality.md` (FR-008, FR-009, SC-006).
- [X] T021 [US4] Auditar `scripts/ci/*.py`, `docs/governance/*-baseline.json` e `docs/adr/*.md` para confirmar que nenhum foi modificado para forçar verde, registrando divergências em `specs/005-ci-protecao-dev/checklists/ci-quality.md` (FR-010, SC-007).
- [X] T022 [US4] Percorrer `specs/005-ci-protecao-dev/spec.md` e `specs/005-ci-protecao-dev/plan.md` e confirmar que toda afirmação de CI, check, merge ou proteção sem comando executado está rotulada `[NV]` (FR-007, SC-008).
- [X] T023 [US4] Contar os dez artefatos em `specs/005-ci-protecao-dev/` e verificar que nenhum tem seção vazia ou marcador de pendência, registrando a contagem em `specs/005-ci-protecao-dev/checklists/requirements.md` (FR-014, SC-009).

## Phase 7: Polish & cross-cutting concerns

- [X] T024 Revisar `.github/workflows/ci-gates.yml` e `specs/005-ci-protecao-dev/plan.md` para garantir que a ordem dos cinco gates permanece idêntica à de `AGENTS.md:16-22` após qualquer ajuste de redação (FR-002, D3).
- [X] T025 Confirmar que nenhum segredo, token ou credencial aparece em `.github/workflows/ci-gates.yml`, `specs/005-ci-protecao-dev/` ou em saída registrada, e que as permissões do workflow são as mínimas (FR-011, SC-010).
- [X] T026 Confirmar em `specs/005-ci-protecao-dev/spec.md` e `specs/005-ci-protecao-dev/decision.md` que a lacuna de diretórios derivados do slug `ci-protecao-dev` permanece registrada como falha honesta do gate de completude, sem alteração de baseline (FR-003, FR-010, Clarificação Q2).
- [X] T027 Verificar que nenhum stage, commit, PR, merge ou reescrita de histórico foi realizado como parte desta etapa de planejamento e que a escrita ficou restrita a `specs/005-ci-protecao-dev/` (FR-012, SC-009).

## Dependencies

A ordem de execução é a numeração das tarefas: T001–T008 preparam o terreno; T009–T012 dependem do workflow configurado; T013–T015 dependem de execução real; T016–T019 dependem do nome observado em T013; T020–T027 são auditoria e acabamento. Não há arestas circulares nem referências a identificadores não declarados.

## Parallel execution examples

- T001, T002 e T003 podem ser executadas em paralelo (arquivos distintos: `.github/workflows/ci-gates.yml` e `specs/005-ci-protecao-dev/spec.md`).
- T011 e T012 podem ser executadas em paralelo após T010 (PR `main` e pushes independentes entre si).
- T020 e T021 podem ser executadas em paralelo (checklist de qualidade versus scripts/baselines/ADRs).

## Implementation strategy

Incremento mínimo: T001–T010 entregam verificação dos cinco gates em PR para `dev`; T013 e T017 entregam o nome observado e a proteção de `dev`; T018 fecha a prova por API; T020–T027 garantem que nenhuma salvaguarda foi sacrificada. Nenhuma etapa é marcada como verde antes de comando executado, saída colada e data/hora.
