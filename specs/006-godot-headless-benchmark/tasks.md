# Tasks: Godot headless benchmark

**Feature**: `006-godot-headless-benchmark` — ver `spec.md`, `plan.md`, `decision.md` (R1 `go`, Opção A) e `checklists/benchmark-quality.md`.

**Convenção**: cada tarefa usa `- [ ] T### [P?] [USn?] descrição com caminho`. Este documento termina em tasks: nenhuma seção de implementação, converge, validação ou release aparece aqui. Evidência de execução é citada com origem e data/hora; o que não foi verificado nesta etapa permanece `[NV]`.

## Phase 1: Setup

- [X] T001 [P] Conferir em `specs/006-godot-headless-benchmark/spec.md` que todos os FRs e SCs possuem rastreabilidade para ao menos um cenário de aceitação, antes de qualquer registro de evidência (FR-014, SC-009).
- [X] T002 [P] Confirmar em `.specify/feature.json` que o diretório da feature é `specs/006-godot-headless-benchmark/`, sem usar nome de branch como identidade (FR-014, SC-009; `AGENTS.md:41`).
- [X] T003 [P] Inventariar em `specs/006-godot-headless-benchmark/research.md` as sete evidências de `plan.md` §Inventário de evidência, com valores e carimbos idênticos aos do pedido, sem reexecutar comandos (FR-010, SC-008).
- [X] T004 Registrar em `specs/006-godot-headless-benchmark/plan.md` o escopo de escrita restrito aos dez artefatos e a ausência de gates, stage, commit, PR e merge nesta etapa (FR-012, SC-010).

## Phase 2: Foundational

- [X] T005 Fixar em `specs/006-godot-headless-benchmark/spec.md` e `plan.md` a identidade `4.7.2-stable` como valor da release upstream, rastreada a `gh api repos/godotengine/godot/releases/tags/4.7.2-stable` (FR-001, SC-001, D1).
- [X] T006 Registrar em `specs/006-godot-headless-benchmark/research.md` o esperado `9aa00f7a...c65` de `SHA512-SUMS.txt` e o calculado idêntico, com o resultado `CHECKSUM_OK` em 2026-11-20T00:43:52Z (FR-002, FR-003, SC-002, D2).
- [X] T007 Registrar em `specs/006-godot-headless-benchmark/plan.md` que divergência de checksum é falha explícita e impede afirmar identidade de binário, sem tolerância ou resultado verde artificial (FR-003, Edge Cases).
- [X] T008 Registrar em `specs/006-godot-headless-benchmark/spec.md` a string `4.7.2.stable.official.ed1daf0bf` como identidade reportada pelo executável, com origem e carimbo 2026-11-20T00:44:11Z (FR-004, SC-003, D3).
- [X] T009 Confirmar em `specs/006-godot-headless-benchmark/plan.md` que nenhuma credencial é necessária e que conteúdo de release, issue ou web é dado não confiável, sem instruções executadas (FR-011, SC-008, D9).

## Phase 3: User Story 1 — Release oficial identificada e integridade provada (Priority: P1)

**Goal**: provar de onde veio a engine e que o artefato não divergiu entre publicação e uso.
**Independent Test**: ler release e par de checksums em `research.md`/`plan.md` e conferir `CHECKSUM_OK` com carimbo.

- [X] T010 [US1] Verificar em `specs/006-godot-headless-benchmark/research.md` que a release `4.7.2-stable` aparece com o comando de consulta e sem URL com userinfo, parâmetro assinado ou credencial (FR-001, SC-001, SC-008).
- [X] T011 [US1] Conferir em `specs/006-godot-headless-benchmark/plan.md` §Inventário de evidência que esperado e calculado são idênticos e que o carimbo 2026-11-20T00:43:52Z é o mesmo citado em `research.md` (FR-002, SC-002, D2).
- [X] T012 [US1] Registrar em `specs/006-godot-headless-benchmark/spec.md` o cenário de divergência de checksum como falha explícita, com nenhuma identidade de binário afirmada a partir do artefato suspeito (FR-003, US1/AC3).
- [X] T013 [P] [US1] Revisar `specs/006-godot-headless-benchmark/checklists/benchmark-quality.md` para confirmar que integridade e origem são tratadas como requisitos de qualidade, não como execução de instalação (FR-002, FR-003, SC-002).

## Phase 4: User Story 2 — Identidade do binário instalado (Priority: P1)

**Goal**: fixar a identidade do executável pela string auto-reportada, não por inferência.
**Independent Test**: ler a string em `spec.md`/`plan.md` e conferir origem e carimbo.

- [X] T014 [US2] Conferir em `specs/006-godot-headless-benchmark/spec.md` que a identidade é `4.7.2.stable.official.ed1daf0bf` com data/hora 2026-11-20T00:44:11Z e origem de comando de consulta (FR-004, SC-003).
- [X] T015 [US2] Registrar em `specs/006-godot-headless-benchmark/plan.md` que versão diferente da pretendida é registrada como observada e a divergência é documentada, nunca ocultada (FR-004, Edge Cases, D3).
- [X] T016 [P] [US2] Verificar em `specs/006-godot-headless-benchmark/checklists/requirements.md` que a identidade do binário possui item de qualidade rastreável a FR-004 e SC-003 (FR-014, SC-009).

## Phase 5: User Story 3 — Medição headless da carga sintética (Priority: P1)

**Goal**: registrar a linha de base de custo por tick com dispersão e comparação à referência.
**Independent Test**: ler `tests/benchmark/headless_tick.gd` e os valores em `research.md`/`plan.md`, conferindo 600×64, 7 amostras, mediana, mínimo e máximo.

- [X] T017 [US3] Conferir em `tests/benchmark/headless_tick.gd` que `TICKS=600`, `ENTITIES=64`, a carga é `sin((tick+entity)*0.01)` e a saída contém `ticks`, `entities`, `elapsed_us`, `per_tick_us` e `accumulator` (FR-005, FR-006, SC-004).
- [X] T018 [US3] Registrar em `specs/006-godot-headless-benchmark/research.md` que a medição tem mediana 37,48 µs/tick em 7 amostras, mínimo 22,71 e máximo 42,95, com carimbo 2026-11-20T00:44:48Z (FR-007, SC-005, D5).
- [X] T019 [US3] Verificar em `specs/006-godot-headless-benchmark/plan.md` que nenhuma amostra é descartada e que a dispersão é registrada como parte da linha de base, não como erro a suprimir (FR-007, SC-005, Edge Cases).
- [X] T020 [US3] Conferir em `specs/006-godot-headless-benchmark/spec.md` a conversão de 37,48 µs/tick para 0,03748 ms/tick e a comparação com 16,7 ms/tick, com registro de conformidade e sem alteração da referência (FR-008, SC-006, D6).
- [X] T021 [P] [US3] Registrar em `specs/006-godot-headless-benchmark/checklists/benchmark-quality.md` que a mensurabilidade de amostras, dispersão e conformidade é critério de qualidade dos requisitos, não teste de execução (SC-005, SC-006, Checklist §Acceptance Criteria Quality).
- [X] T022 [US3] Registrar em `specs/006-godot-headless-benchmark/research.md` que uma mediana acima da referência seria registrada como não conforme, sem ajuste de carga, descarte de amostra ou relaxamento do limiar (FR-008, Edge Cases).

## Phase 6: User Story 4 — Rotulagem honesta e rastreabilidade (Priority: P2)

**Goal**: impedir leitura da carga como partida real e provar que os artefatos estão completos e rastreáveis.
**Independent Test**: contar descrições e artefatos em `specs/006-godot-headless-benchmark/` e conferir ausência de seções vazias, marcadores de pendência e portarias posteriores.

- [X] T023 [US4] Revisar `specs/006-godot-headless-benchmark/spec.md`, `plan.md`, `decision.md` e `research.md` e confirmar que toda descrição identifica a carga como simulação headless sintética de 600 ticks × 64 entidades (FR-005, FR-009, SC-007, D7).
- [X] T024 [US4] Confirmar que nenhum artefato em `specs/006-godot-headless-benchmark/` usa a medição como benchmark 5v5 real ou como prova de desempenho de partida multiplayer; toda ocorrência do termo aparece em negação explícita (FR-009, SC-007, Clarificação Q1).
- [X] T025 [US4] Conferir em `specs/006-godot-headless-benchmark/spec.md` que afirmações não verificadas nesta etapa permanecem `[NV]` e que instalação, reexecução e R4/R7 são identificadas como trabalho de fases posteriores (FR-010, FR-013, SC-008, D8).
- [X] T026 [P] [US4] Auditar os dez artefatos em `specs/006-godot-headless-benchmark/` para contar 10 de 10 presentes, sem seção vazia, sem marcador de pendência, com checklist `[ ]` e tarefas `T###` sem portarias posteriores (FR-014, SC-009).
- [X] T027 [US4] Confirmar em `specs/006-godot-headless-benchmark/plan.md` que nenhum gate foi executado, nenhum arquivo fora do diretório da feature foi escrito e nenhum stage, commit, PR ou merge foi realizado nesta etapa (FR-012, SC-010, D10).

## Phase 7: Polish & cross-cutting concerns

- [X] T028 Revisar os carimbos em `specs/006-godot-headless-benchmark/research.md` e `plan.md` para manter 00:43:52Z, 00:44:11Z e 00:44:48Z idênticos aos do pedido, sem arredondamento ou substituição por data de reexecução (FR-010, SC-002, SC-003, SC-005).
- [X] T029 Confirmar em `specs/006-godot-headless-benchmark/decision.md` que o veredito `go` registra evidência `strong`, opção recomendada nomeada e nenhum `unknown` glossado, sem antecipar implementação (R1, FR-013).
- [X] T030 Verificar em `specs/006-godot-headless-benchmark/spec.md` que as três clarificações da Session 2026-11-20 permanecem uma por vez, no formato `- Q: … → A: …`, com R3 respeitado (R3, SC-009).

## Dependencies

A ordem de execução é a numeração das tarefas: T001–T009 preparam o terreno e fixam as evidências; T010–T013 dependem do registro de release e checksum; T014–T016 dependem da identidade do binário; T017–T022 dependem da carga e dos valores de medição; T023–T027 dependem do inventário completo para auditar rotulagem e rastreabilidade; T028–T030 são acabamento. Não há arestas circulares nem referências a identificadores não declarados.

## Parallel execution examples

- T001, T002 e T003 podem ser executadas em paralelo (arquivos distintos: `spec.md`, `.specify/feature.json` e `research.md`).
- T010 e T011 podem ser executadas em paralelo após T006 (revisão de `research.md` e `plan.md`).
- T021 e T022 podem ser executadas em paralelo após T018 (checklist de qualidade e registro de evidência em arquivos distintos).
- T026 e T027 podem ser executadas em paralelo após T025 (contagem de artefatos e auditoria de escopo).

## Implementation strategy

Incremento mínimo: T001–T011 entregam proveniência e integridade documentadas; T014–T016 fecham a identidade do binário; T017–T022 entregam a linha de base com dispersão e comparação; T023–T027 garantem rotulagem sintética e rastreabilidade; T028–T030 preservam carimbos, R1 e R3. Nenhuma tarefa afirma instalação reexecutada, benchmark reexecutado ou portaria posterior a tasks; essas ações pertencem a fases seguintes e permanecem `[NV]` até comando executado com saída colada e data/hora.
