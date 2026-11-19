# Tasks: Baselines locais de governança

**Feature**: `003-baseline-governanca` | **Created**: 2026-11-19 | **Fase**: 11 (tasks) do contrato SDD, conforme `docs/governance/sdd-phase-contracts.md:20`.

**Input**: `specs/003-baseline-governanca/spec.md` (US1/US2/US3, FR-001–FR-012, SC-001–SC-008, `## Clarifications`), `specs/003-baseline-governanca/plan.md` (D1–D10), `specs/003-baseline-governanca/research.md` (RQ-1–RQ-10), `specs/003-baseline-governanca/data-model.md`, `specs/003-baseline-governanca/contracts/baseline-bootstrap-contract.md`, `specs/003-baseline-governanca/quickstart.md`, `specs/003-baseline-governanca/checklists/baseline-quality.md`, `.specify/memory/constitution.md`.

**Caminhos canônicos de materialização**: `docs/governance/sdd-completeness-baseline.json`, `docs/governance/converge-append-only-baseline.json`, `docs/governance/task-id-uniqueness-baseline.json`. Os três scripts em `scripts/ci/` são lidos e executados, nunca alterados (FR-011, SC-006). A factory é fonte do aparato, não autoridade dos baselines (FR-005). Nenhum `converge.md` é reescrito e nenhum efeito remoto é produzido.

**Formato obrigatório**: `- [ ] T### [P?] [USn?] descrição com caminho`. Cada tarefa aponta para ao menos um arquivo dos caminhos canônicos, de planejamento ou dos scripts, e pertence a uma fase Setup→Foundational→User Stories→Polish. IDs sequenciais e únicos, T001–T036. A marcação `[X]` pertence à portaria posterior de materialização, não a este arquivo. A regra de STOP por item `[ ]` vale somente para a revisão humana do checklist de domínio (`checklists/baseline-quality.md`); itens ainda não revisados não bloqueiam a entrega documental. `checklists/requirements.md` segue seu ciclo próprio de clarify. Analyze, implement, converge, validate (R4) e release (R7) são portarias separadas, não tarefas deste arquivo.

## Phase 1: Setup

**Goal**: validar o gate de entrada da fase 11, conferir a identidade da feature, a presença dos três scripts, a ausência dos baselines e o vocabulário de derivação antes de qualquer materialização.

- [X] T001 [P] Validar o gate de entrada da fase 11 lendo `.specify/feature.json` e `specs/003-baseline-governanca/spec.md`, conferindo que a identidade da feature não depende do nome do branch e que `checklists/requirements.md` registra 18/18 itens aprovados após a clarify (contrato de fase item 23; spec §Input).
- [X] T002 [P] Conferir por leitura, sem alterar, os três scripts `scripts/ci/check_sdd_completeness.py`, `scripts/ci/check_converge_append_only.py` e `scripts/ci/check_task_id_uniqueness.py`, localizando os valores de `DEFAULT_BASELINE`, os schemas `047.2.0`/`087.1.0`, a regex `T###` e o gerador `--bootstrap` (FR-001–FR-003, FR-011; research RQ-1–RQ-3).
- [X] T003 [P] Conferir a listagem local de `docs/governance/`, registrando que os três arquivos de baseline ainda não existem e que o diretório é o destino autoritativo antes de qualquer gravação (spec FR-001–FR-003; quickstart §Comandos de setup).
- [X] T004 [P] Conferir por leitura `specs/003-baseline-governanca/contracts/baseline-bootstrap-contract.md` §3.2, fixando o vocabulário de derivação `regressions → allow_missing/allow_duplicates` e a saída literal de `--bootstrap → files`, sem executar comandos nesta tarefa (D1; contrato §3).

## Phase 2: Foundational

**Goal**: travar as regras compartilhadas — caminhos canônicos, proveniência, autoridade do estado corrente, critério de gap legítimo, imutabilidade dos scripts e honestidade de mapas vazios — que bloqueiam todas as histórias.

- [X] T005 [P] Fixar os três caminhos canônicos em `docs/governance/` — `sdd-completeness-baseline.json`, `converge-append-only-baseline.json` e `task-id-uniqueness-baseline.json` — conferindo que coincidem com os valores padrão de `--baseline` lidos em `scripts/ci/` (contrato §1; spec FR-001–FR-003; D2).
- [X] T006 [P] Registrar o formato do campo `_provenance` — `origem=alvo-local`, `metodo`, `data` em ISO 8601 e `factory=nao-utilizada` — a acrescentar em cada um dos três JSON de `docs/governance/`, verificando em `specs/003-baseline-governanca/data-model.md` §Entidade Proveniência local que os carregadores ignoram campos extras (FR-004; D3; research RQ-6).
- [X] T007 [P] Registrar a regra de autoridade do estado corrente e o critério gap legítimo versus defeito a partir de `specs/003-baseline-governanca/spec.md` §Clarifications Q1/Q3, aplicando-os à entrada `003-baseline-governanca: release, validate` prevista para `docs/governance/sdd-completeness-baseline.json` (D4, D5; FR-010; research RQ-5, RQ-8).
- [X] T008 [P] Registrar a política de imutabilidade dos scripts de `scripts/ci/` e de mapas vazios honestos, lendo `specs/003-baseline-governanca/spec.md` FR-009/FR-011 e SC-004/ SC-006, de modo que nenhum baselines inventado e nenhuma alteração de gate entre no escopo (D6, D7; research RQ-7, RQ-9).
- [X] T009 Consolidar o desenho de bootstrap lendo `specs/003-baseline-governanca/research.md` RQ-1–RQ-4 e `specs/003-baseline-governanca/quickstart.md` §Procedimento de bootstrap, sem executar comandos nem gravar em `docs/governance/` nesta tarefa (plan §Technical Decisions; contrato §3.1).

## Phase 3: User Story 1 — Bootstrap local do baseline de convergência (P1)

**Goal**: materializar `BL-CONV` a partir do gerador nativo do alvo, com proveniência e verificação de integridade, sem reescrever nenhum `converge.md`.

**Independent Test**: gerar o JSON com `--bootstrap`, gravá-lo em `docs/governance/converge-append-only-baseline.json` e executar o gate com esse baseline, obtendo PASS com `baselined_files` igual à contagem de `converge.md` e nenhum achado.

- [X] T010 [P] [US1] Executar `python3 scripts/ci/check_converge_append_only.py --bootstrap`, capturar a saída e gravá-la em `docs/governance/converge-append-only-baseline.json`, preservando `schema_version="087.1.0"`, `enforce_unbaselined=true` e o mapa `files` com `sha256` e `bytes` por `specs/<NNN>-*/converge.md` (FR-001; contrato §2.1; quickstart etapa 1).
- [X] T011 [P] [US1] Acrescentar o campo `_provenance` a `docs/governance/converge-append-only-baseline.json` com `metodo=scripts/ci/check_converge_append_only.py --bootstrap`, sem alterar `schema_version`, `enforce_unbaselined` ou `files` (FR-004; data-model §Entidade Proveniência local).
- [X] T012 [P] [US1] Conferir em `docs/governance/converge-append-only-baseline.json` que nenhuma chave de `converge.md` da factory, órfã ou removida aparece em `files`, e que nenhum arquivo `specs/<NNN>-*/converge.md` foi reescrito (FR-005; SC-005; contrato §1, §4.1).
- [X] T013 [P] [US1] Executar `python3 scripts/ci/check_converge_append_only.py --baseline docs/governance/converge-append-only-baseline.json` e conferir PASS com `baselined_files` igual à contagem de `converge.md` e `rewritten_or_truncated`, `missing`, `unbaselined` e `stale_baseline` vazios (FR-006; SC-003; quickstart etapa 6).
- [X] T014 [P] [US1] Aplicar `RG-2` de forma isolada e reversível sobre um `converge.md` de teste acrescentado ao alvo, observando a recusa do gate com `unbaselined` (prova de ratchet com baseline vazio) e registrando a assinatura em `specs/003-baseline-governanca/contracts/baseline-bootstrap-contract.md` §4.3; registrar `RG-1` como cenário CONDICIONAL, não executado, pois o mapa `files` do baseline está vazio e não há `converge.md` baselinado a ser reescrito/truncado (`rewritten_or_truncated`), sem tocar nos arquivos reais (SC-008; contrato §4.3).
- [X] T015 [US1] Registrar em `specs/003-baseline-governanca/data-model.md` §Entidade Evidência de gate apenas as contagens e o exit code observados na verificação de `docs/governance/converge-append-only-baseline.json` já executada em T013, mantendo efeito remoto `[NV]`; sem saída colada com data/hora nesta tarefa, pois evidência datada pertence exclusivamente a `.specify/validations/baseline-governanca/validate.md` (R4) (FR-012; SC-007).

## Phase 4: User Story 2 — Bootstrap local dos baselines de completude e unicidade (P1)

**Goal**: materializar `BL-COMP` e `BL-UNIQ` por derivação determinística do inventário local, com proveniência e verificação de integridade.

**Independent Test**: capturar `regressions` sem baseline, gravar os dois JSON nos caminhos canônicos e executar cada gate com seu baseline, obtendo PASS com zero regressões e zero `stale` — sem depender de `BL-CONV`.

- [X] T016 [P] [US2] Executar `python3 scripts/ci/check_sdd_completeness.py --json` antes de gravar `docs/governance/sdd-completeness-baseline.json` e capturar o mapa `regressions` como inventário do alvo (FR-002; contrato §3.1; quickstart etapa 2).
- [X] T017 [P] [US2] Gravar `docs/governance/sdd-completeness-baseline.json` com `allow_missing` igual ao `regressions` capturado e `_provenance` com `metodo=scripts/ci/check_sdd_completeness.py --json (sem baseline) → regressions`, sem acrescentar chaves (FR-002, FR-004; contrato §2.2, §3.2).
- [X] T018 [P] [US2] Conferir em `docs/governance/sdd-completeness-baseline.json` que as entradas `release` e `validate` de `003-baseline-governanca` são dívida legítima de portarias futuras e que nenhum artefato de fase já concluída (`spec.md`, `plan.md`, `tasks.md`, `checklists`, `assessment`) foi baselinado como defeito (FR-010; spec §Clarifications Q3; research RQ-8).
- [X] T019 [P] [US2] Executar `python3 scripts/ci/check_sdd_completeness.py --json --baseline docs/governance/sdd-completeness-baseline.json` e conferir PASS com `baselined_features>0`, `regressions` e `stale_baseline` vazios e `strict_from=88` inalterado (FR-007; SC-003, SC-004; quickstart etapa 6).
- [X] T020 [P] [US2] Executar `python3 scripts/ci/check_task_id_uniqueness.py --json` antes de gravar `docs/governance/task-id-uniqueness-baseline.json` e capturar o mapa `regressions` como inventário do alvo (FR-003; contrato §3.1; quickstart etapa 4).
- [X] T021 [P] [US2] Gravar `docs/governance/task-id-uniqueness-baseline.json` com `allow_duplicates` igual ao `regressions` capturado — mapa vazio no estado sem duplicatas — e `_provenance` com `metodo=scripts/ci/check_task_id_uniqueness.py --json (sem baseline) → regressions`, sem inventar entradas `duplicate_task:T###` (FR-003, FR-004, FR-009; contrato §2.3).
- [X] T022 [P] [US2] Executar `python3 scripts/ci/check_task_id_uniqueness.py --json --baseline docs/governance/task-id-uniqueness-baseline.json` e conferir PASS com `baselined_specs=0`, `regressions` e `stale_baseline` vazios e a regex `T###` inalterada em `scripts/ci/check_task_id_uniqueness.py` (FR-008; SC-003, SC-004; quickstart etapa 6).
- [X] T023 [P] [US2] Aplicar `RG-3` e `RG-4` de forma isolada e reversível sobre artefatos de teste em `specs/`, executando somente a verificação necessária para observar a recusa dos gates (`stale_baseline` em completude e `duplicate_task:T###` em unicidade, respectivamente) e registrando em `specs/003-baseline-governanca/contracts/baseline-bootstrap-contract.md` §4.3 apenas as contagens e os exit codes observados, sem saída colada com data/hora — evidência datada pertence exclusivamente a `.specify/validations/baseline-governanca/validate.md` (R4); registrar `RG-5` (`stale_baseline` em unicidade) como cenário CONDICIONAL, não executado, pois o mapa `allow_duplicates` do baseline está vazio e não há entrada baselinada que possa tornar-se stale (SC-008; contrato §4.3).
- [X] T024 [US2] Registrar em `specs/003-baseline-governanca/data-model.md` §Entidade Evidência de gate apenas as contagens e os exit codes observados nas verificações de `docs/governance/sdd-completeness-baseline.json` e `docs/governance/task-id-uniqueness-baseline.json` já executadas em T019 e T022, mantendo efeito remoto `[NV]`; sem saídas coladas com data/hora nesta tarefa, pois evidência datada pertence exclusivamente a `.specify/validations/baseline-governanca/validate.md` (R4) (FR-012; SC-007).

## Phase 5: User Story 3 — Evidenciar a origem local e a integridade dos três gates (P2)

**Goal**: conferir por leitura e comparação que os três baselines são locais, honestos e completos, que os scripts permanecem intactos e que a evidência está pronta para a portaria de evidência, sem efeito remoto.

**Independent Test**: ler os três JSON, comparar os três scripts e conferir as contagens e os exit codes observados, contando zero chaves da factory e zero alterações de script; evidência com data/hora fica exclusivamente em `.specify/validations/baseline-governanca/validate.md` (R4), não nesta fase.

- [X] T025 [P] [US3] Conferir por leitura os três arquivos em `docs/governance/`, validando em cada um as chaves exigidas por `specs/003-baseline-governanca/contracts/baseline-bootstrap-contract.md` §2 e a presença de `_provenance` com origem `alvo-local` (SC-001, SC-004; FR-004).
- [X] T026 [P] [US3] Conferir zero chaves de spec da factory e zero entradas órfãs nos três JSON de `docs/governance/`, comparando cada chave com o inventário produzido por `scripts/ci/` no ato da bootstrap (SC-005; FR-005; contrato §1).
- [X] T027 [P] [US3] Calcular `sha256sum` dos três scripts em `scripts/ci/` e comparar com os valores registrados em `specs/003-baseline-governanca/contracts/baseline-bootstrap-contract.md` §8 (146fd007…, a51a5c6e…, b6c5e9ca…), registrando apenas o resultado da comparação (igual/diferente) e a contagem de arquivos conferidos; evidência datada fica em R4 (SC-006; FR-011; D7).
- [X] T028 [P] [US3] Conferir a coerência dos contadores nos três JSON de `docs/governance/` — `baselined>0` em completude e mapas vazios honestos em convergência e unicidade — confrontando `specs/003-baseline-governanca/spec.md` SC-004 com a saída esperada de `specs/003-baseline-governanca/quickstart.md` etapa 6 (SC-004; D6; research RQ-7).
- [X] T029 [P] [US3] Conferir que, dos três gates executados com os baselines de `docs/governance/`, apenas as contagens e os exit codes observados foram registrados em `specs/003-baseline-governanca/data-model.md` §Entidade Evidência de gate, sem saída colada com data/hora em artefato algum — evidência datada pertence exclusivamente a `.specify/validations/baseline-governanca/validate.md` (R4) (FR-012; SC-007; contrato §6).
- [X] T030 [P] [US3] Conferir que nenhum artefato de `specs/003-baseline-governanca/` afirma CI, merge, publicação, aprovação ou proteção de branch como obtidos, mantendo a rotulação `[NV]` em toda evidência (SC-007; constituição VI; contrato §6).
- [X] T031 [US3] Consolidar por leitura, sem alterar artefato algum, apenas as contagens e os exit codes observados — 3/3 baselines presentes, 3/3 gates PASS, 0 regressões, 0 `stale` e 0 chaves da factory — no registro documental de `specs/003-baseline-governanca/data-model.md` §Entidade Evidência de gate, registrando divergência com ID FR/SC ou `BUG-###` e sem saída com data/hora, que pertence exclusivamente a `.specify/validations/baseline-governanca/validate.md` (R4) (SC-001–SC-008; contrato §8).

## Phase 6: Polish & Cross-Cutting Concerns

**Goal**: uniformizar terminologia, preservar os checklists como gate somente leitura, revisar re-baselinação e rollback e encerrar o escopo dos três JSON.

- [X] T032 [P] Revisar a consistência terminológica entre `specs/003-baseline-governanca/spec.md`, `contracts/baseline-bootstrap-contract.md`, `data-model.md` e `quickstart.md`, conferindo o uso uniforme de `alvo-local`, `nao-utilizada`, `regressions`, `stale_baseline`, `baselined` e `[NV]` (contrato §1, §6; BQ-CHK009, BQ-CHK015).
- [X] T033 [P] Confirmar por leitura que os itens `BQ-CHK001`–`BQ-CHK050` de `specs/003-baseline-governanca/checklists/baseline-quality.md` permanecem `[ ]`, que nenhum foi marcado como aprovado pelo gerador e que `checklists/requirements.md` segue seu ciclo próprio sem alteração nesta tarefa (contrato de fase item 9; checklist §Marker Semantics).
- [X] T034 [P] Revisar por leitura a delimitação de escopo em `docs/governance/` e `scripts/ci/`, confirmando que exatamente três JSON foram materializados, que nenhum script foi alterado, que nenhum `converge.md` foi reescrito e que nenhum artefato de `specs/` recebeu código, build, pipeline ou efeito remoto (FR-011; SC-006, SC-007).
- [X] T035 [P] Revisar as regras de re-baselinação e rollback documental em `specs/003-baseline-governanca/contracts/baseline-bootstrap-contract.md` §4.2 e §5, confirmando que versão SemVer, data, changelog por IDs FR/`T###`/`BUG-###` e preservação de histórico estão definidos para os três JSON de `docs/governance/` (contrato §5; BQ-CHK048).
- [X] T036 Revisar por leitura, sem alterar, a conformidade final dos três baselines em `docs/governance/` contra `specs/003-baseline-governanca/contracts/baseline-bootstrap-contract.md` §8, usando `sdd-completeness-baseline.json` como exemplo de reversão e verificando que o procedimento nunca apaga histórico, altera scripts, copia da factory, reescreve `converge.md` ou afirma efeito remoto (contrato §5.3, §8; SC-007).

## Dependencies

| Bloco | Tarefas | Dependência |
|---|---|---|
| Setup | T001–T004 | Nenhuma; gate de entrada da fase 11. |
| Foundational | T005–T009 | Regras e formatos conferidos por T001–T004. |
| US1 (P1) | T010–T015 | Regras de T005–T009; entrega isolada de `BL-CONV`. |
| US2 (P1) | T016–T024 | Regras de T005–T009; entrega isolada de `BL-COMP` e `BL-UNIQ`. |
| US3 (P2) | T025–T031 | Materialização de T010–T024; leitura cruzada e comparação. |
| Polish | T032–T036 | T010–T031 concluídas. |

## Parallel Opportunities

| Oportunidade | Tarefas | Motivo |
|---|---|---|
| Gate e formatos | T001, T002, T003, T004 | Leitura de fontes distintas, sem escrita concorrente. |
| Regras compartilhadas | T005, T006, T007, T008 | Dimensões distintas, sem escrita concorrente. |
| Bootstrap | T010, T016, T020 | Caminhos de baseline distintos; a ordem "antes/depois da gravação" é respeitada dentro de cada US. |
| Verificação dos gates | T013, T019, T022 | Gates distintos, sem escrita concorrente em `docs/governance/`. |
| Cenários de regressão | T014, T023 | Isolados e reversíveis, com artefatos de teste distintos. |
| Auditoria cruzada | T025–T030 | Leitura somente dos três JSON, dos três scripts e das evidências. |
| Polish | T032, T033, T034, T035 | Revisões independentes de terminologia, checklist, escopo e rollback. |

## Estratégia de Entrega

Entrega incremental por user story, com MVP limitado à US1 (`BL-CONV` materializado e verificável isoladamente). A US2 é o segundo incremento independente (`BL-COMP` e `BL-UNIQ`); a US3 fecha a auditoria de origem local e integridade. Nenhuma tarefa altera scripts, reescreve `converge.md`, introduz código, build, teste automatizado, pipeline, serviço externo ou efeito remoto. Checklists são gate somente leitura e a marcação `[x]` do checklist de domínio é decidida pelo revisor humano. Analyze, implement, converge, validate (R4) e release (R7) são portarias do ciclo completo, posteriores a esta fase e não incluídas como tarefas.

## Marcos

| Marco | Faixa de tarefas | Resultado observável |
|---|---|---|
| M1 — Gate e formatos prontos | T001–T004 | Identidade da feature, três scripts lidos, ausência de baselines confirmada e vocabulário de derivação fixado. |
| M2 — Regras travadas | T005–T009 | Caminhos, proveniência, autoridade do estado corrente, critério de gap, imutabilidade e mapas vazios definidos. |
| M3 — Convergência baselinada | T010–T015 | `converge-append-only-baseline.json` gravado, proveniência presente, gate PASS e cenário `RG-2` recusado; `RG-1` condicional. |
| M4 — Completude e unicidade baselinadas | T016–T024 | `sdd-completeness-baseline.json` e `task-id-uniqueness-baseline.json` gravados, gates PASS e cenários `RG-3`/`RG-4` recusados; `RG-5` condicional. |
| M5 — Auditoria concluída | T025–T031 | 3/3 baselines locais, 3/3 gates PASS, 0 regressões, 0 `stale`, 0 chaves da factory e contagens/exit codes registrados; evidência com data/hora reservada à portaria R4. |
| M6 — Consistência encerrada | T032–T036 | Polish aplicado, checklists preservados como gate somente leitura e escopo dos três JSON encerrado. |

## Format Validation

As 36 tarefas T001–T036 usam ID sequencial e único, checkbox `- [ ]`, descrição com caminho explícito e marcador `[P]`/`[USn]` conforme a fase. Cada tarefa aponta para ao menos um arquivo dos caminhos canônicos, de planejamento ou dos scripts. Fases na ordem Setup→Foundational→User Stories→Polish; nenhuma portaria posterior a tasks aparece como tarefa. Nenhum campo da spec permanece pendente de definição e nenhum item do checklist de domínio é marcado neste arquivo.
