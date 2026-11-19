# Tasks: Exceção de linguagem e licença MIT-only do Núcleo

**Feature**: `001-excecao-linguagem-licenca-nucleo` | **Created**: 2026-11-19 | **Fase**: 11 (tasks) do contrato SDD, conforme `docs/governance/sdd-phase-contracts.md:20`.

**Input**: `specs/001-excecao-linguagem-licenca-nucleo/spec.md` (US1/US2/US3, FR-001–FR-013, SC-001–SC-008, `## Clarifications`), `specs/001-excecao-linguagem-licenca-nucleo/plan.md` (D1–D8), `specs/001-excecao-linguagem-licenca-nucleo/data-model.md`, `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md`, `specs/001-excecao-linguagem-licenca-nucleo/quickstart.md`, `specs/001-excecao-linguagem-licenca-nucleo/checklists/adr-quality.md`, `.specify/memory/constitution.md`.

**Caminhos canônicos de materialização**: ADR-001 em `docs/adr/001-excecao-linguagem-gdscript.md`; ADR-002 em `docs/adr/002-licenca-mit-nucleo.md`. São registros Markdown versionados; nenhum código, compilação, teste, gate, pipeline, serviço externo ou emenda da norma externa é produzido nesta fase de planejamento (`spec.md` FR-010, SC-007).

**Formato obrigatório**: `- [X] T### [P?] [USn?] descrição com caminho` na fase implement (35 itens marcados). O formato de definição na fase tasks era `- [ ] T### …`; a marcação `[X]` pertence à fase implement (`docs/governance/sdd-phase-contracts.md:21`). A regra de STOP por item `[ ]` vale somente para a revisão humana do checklist de domínio (`checklists/adr-quality.md`); um item `[ ]` ainda não revisado não bloqueia a implementação documental destas tarefas. A vedação de marcação `[x]` pelo gerador vale somente para `adr-quality.md` — `checklists/requirements.md` segue seu ciclo próprio, com 16 itens `[x]` aprovados na fase clarify.
**Escopo temporal e gate R4**: nesta fase de planejamento nenhum gate é executado; R4 (validate) e R7 (release) são portarias do ciclo completo, posteriores a esta fase de tarefas. O resultado esperado do gate determinístico `python3 scripts/ci/check_sdd_completeness.py` é `check_sdd_completeness PASS` quando o baseline em `docs/governance/sdd-completeness-baseline.json` for materializado pelo mantenedor da governança e não houver regressão; sem baseline, o gate reporta FAIL e esse é o resultado honesto a registrar em R4. Nenhum verde é afirmado nesta fase (`docs/governance/sdd-phase-contracts.md:26`).

## Phase 1: Setup

**Goal**: validar o gate de entrada, criar o diretório canônico e fixar o esquema único de campos antes de escrever qualquer registro.

- [X] T001 Validar o gate de entrada da fase 11 conferindo presença e ausência de marcadores pendentes em `specs/001-excecao-linguagem-licenca-nucleo/spec.md`, `specs/001-excecao-linguagem-licenca-nucleo/plan.md`, `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md` e `docs/governance/sdd-phase-contracts.md:20`.
- [X] T002 [P] Criar o diretório canônico `docs/adr/` para materializar `docs/adr/001-excecao-linguagem-gdscript.md` e `docs/adr/002-licenca-mit-nucleo.md`, conforme a decisão D2 em `specs/001-excecao-linguagem-licenca-nucleo/plan.md` e o contrato §1.
- [X] T003 [P] Fixar o esquema comum de campos de `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md §2.1` e a ordem de títulos de `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md §2.4` como modelo único dos dois registros (FR-009).

## Phase 2: Foundational

**Goal**: travar as regras compartilhadas de rastreabilidade, não reabertura, limite documental e versionamento que bloqueiam as duas histórias.

- [X] T004 [P] Consolidar a matriz de rastreabilidade com `DEV-LING-01`, `DEV-EXC-01`, `DEV-LIC-01` e `DEC-R1` em `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md §2.3`, preservando 3/3 desvios e 4/4 referências e excluindo o desvio de CI (FR-012, SC-002, SC-008).
- [X] T005 [P] Fixar a redação canônica de não reabertura de Godot-first com base em `docs/GDD - Insurgência.md:1265,1678` e `specs/001-excecao-linguagem-licenca-nucleo/research.md RQ-4`, para replicação idêntica nos dois ADRs (FR-003, SC-003, SC-004).
- [X] T006 [P] Fixar o limite documental de `specs/001-excecao-linguagem-licenca-nucleo/quickstart.md` — nenhum código, compilação, teste, gate, pipeline, serviço externo ou emenda normativa — como restrição de redação em `docs/adr/001-excecao-linguagem-gdscript.md` e `docs/adr/002-licenca-mit-nucleo.md` (FR-010, SC-007).
- [X] T007 Registrar o formato de `versao` `1.0.0`, de `changelog` com IDs FR/T e de rollback documental em `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md §4–§5` como requisito comum aos dois registros.

## Phase 3: User Story 1 — Registrar a exceção de linguagem para GDScript (P1)

**Goal**: autorizar GDScript como linguagem de engine e script de jogo sem reabrir o pilar Godot-first.

**Independent Test**: ler somente `docs/adr/001-excecao-linguagem-gdscript.md` e verificar a exceção declarada, o pilar intocável e os critérios de reavaliação.

- [X] T008 [US1] Materializar `docs/adr/001-excecao-linguagem-gdscript.md` com o título `# ADR-001 — Exceção de linguagem para GDScript`, cabeçalho `data: 2026-11-19`, `versao: 1.0.0`, `estado: Pendente de aprovação`, e as seções `## Contexto`, `## Decisão`, `## Alternativas consideradas`, `## Consequências`, `## Não reabertura de Godot-first`, `## Critérios de reavaliação`, `## Autoridade de aprovação`, `## Rastreabilidade` e `## Histórico de versões` (FR-001, FR-009; contrato §2.1, §2.4).
- [X] T009 [US1] Declarar `linguagem_adotada` na seção `## Decisão` de `docs/adr/001-excecao-linguagem-gdscript.md`, nomeando GDScript como linguagem de engine e script de jogo adotada no ciclo, sem segunda linguagem ou troca de engine (FR-002).
- [X] T010 [US1] Inserir em `docs/adr/001-excecao-linguagem-gdscript.md` a declaração explícita de que Godot-first permanece vinculante e não é reaberta, revista ou revertida (FR-003, SC-004).
- [X] T011 [US1] Registrar `criterios_reeavaliacao` na seção `## Critérios de reavaliação` de `docs/adr/001-excecao-linguagem-gdscript.md`, exigindo evidência prévia e aprovação formal para qualquer mudança de linguagem e encaminhando propostas futuras a aprovação formal (FR-004; US1 cenário 3).
- [X] T012 [US1] Preencher `## Alternativas consideradas` em `docs/adr/001-excecao-linguagem-gdscript.md` com a emenda normativa e o servidor Rust como alternativas não adotadas, e `## Consequências` com a contingência futura de emenda sem execução neste ciclo, conforme `specs/001-excecao-linguagem-licenca-nucleo/research.md RQ-1` (FR-009).
- [X] T013 [US1] Preencher `## Autoridade de aprovação` em `docs/adr/001-excecao-linguagem-gdscript.md` nomeando o maintainer como autoridade, registrando a aprovação humana como necessária e o registro efetivo como `[NV]` (FR-011, SC-006).
- [X] T014 [US1] Preencher `## Rastreabilidade` em `docs/adr/001-excecao-linguagem-gdscript.md` com `DEV-LING-01`, `DEV-EXC-01` e `DEC-R1`, sem incluir `DEV-LIC-01`, conforme `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md §2.3` (FR-012).
- [X] T015 [US1] Preencher `## Histórico de versões` em `docs/adr/001-excecao-linguagem-gdscript.md` com a entrada `### [1.0.0] — 2026-11-19`, os IDs FR-001–FR-004, FR-009, FR-011, FR-012 e os IDs `T008`–`T014` como origem da materialização (contrato §4).

## Phase 4: User Story 2 — Fechar a licença do Núcleo como MIT-only (P1)

**Goal**: fixar uma posição única de licenciamento MIT para Núcleo e servidor headless do ciclo, sem cláusula alternativa e sem AGPL.

**Independent Test**: ler somente `docs/adr/002-licenca-mit-nucleo.md` e confirmar MIT-only, exclusão de "ou GPL" e de AGPL, regra de não cópia e não atribuição de licença exclusiva ao motor.

- [X] T016 [US2] Materializar `docs/adr/002-licenca-mit-nucleo.md` com o título `# ADR-002 — Licença MIT-only do Núcleo`, cabeçalho `data: 2026-11-19`, `versao: 1.0.0`, `estado: Pendente de aprovação`, e as seções `## Contexto`, `## Decisão`, `## Alternativas consideradas`, `## Consequências`, `## Não reabertura de Godot-first`, `## Licenciamento`, `## Autoridade de aprovação`, `## Rastreabilidade` e `## Histórico de versões` (FR-001, FR-009; contrato §2.1, §2.4).
- [X] T017 [US2] Declarar `posicao_licenca` na seção `## Licenciamento` de `docs/adr/002-licenca-mit-nucleo.md`, estabelecendo MIT como posição única para todo o código do ciclo, incluindo Núcleo e servidor headless, com a licença MIT vigente preservada (FR-005, SC-005).
- [X] T018 [US2] Registrar `exclusao_alternativas` em `docs/adr/002-licenca-mit-nucleo.md`, excluindo expressamente a cláusula "MIT ou GPL" e o licenciamento AGPL para o código do ciclo (FR-006).
- [X] T019 [US2] Registrar `regra_copyleft` em `docs/adr/002-licenca-mit-nucleo.md`, declarando que nenhum código sob licença copyleft é copiado e que contribuições de terceiros são identificadas e atribuídas, nunca apropriadas (FR-007).
- [X] T020 [US2] Registrar `ressalva_motor` em `docs/adr/002-licenca-mit-nucleo.md`, afirmando que a licença do motor adotado não é alterada nem declarada exclusiva e preservando a ressalva de que o motor não é o único MIT (FR-008; US2 cenário 3).
- [X] T021 [US2] Registrar `fora_escopo` em `docs/adr/002-licenca-mit-nucleo.md`, registrando a decisão sobre o arquivo de licenças de terceiros como fora do escopo do ciclo, sem resolução implícita (FR-013).
- [X] T022 [US2] Inserir em `docs/adr/002-licenca-mit-nucleo.md` a declaração de não reabertura de Godot-first, idêntica à de `docs/adr/001-excecao-linguagem-gdscript.md` (FR-003, SC-004).
- [X] T023 [US2] Preencher `## Autoridade de aprovação` em `docs/adr/002-licenca-mit-nucleo.md` nomeando o maintainer como autoridade, registrando a aprovação humana como necessária e o status remoto como `[NV]` (FR-011, SC-006).
- [X] T024 [US2] Preencher `## Rastreabilidade` em `docs/adr/002-licenca-mit-nucleo.md` com `DEV-LIC-01` e `DEC-R1`, sem incluir `DEV-LING-01` ou `DEV-EXC-01`, conforme `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md §2.3` (FR-012).
- [X] T025 [US2] Preencher `## Histórico de versões` em `docs/adr/002-licenca-mit-nucleo.md` com a entrada `### [1.0.0] — 2026-11-19`, os IDs FR-001, FR-003, FR-005–FR-009, FR-011, FR-012, FR-013 e os IDs `T016`–`T024` como origem da materialização (contrato §4).

## Phase 5: User Story 3 — Verificar coerência, aprovação e não reabertura do pilar (P2)

**Goal**: conferir por leitura cruzada que os dois registros estão aprovados como autoridade necessária, ligados aos desvios e a R1, e que nada reabre o pilar nem transforma o ciclo em entrega de código.

**Independent Test**: ler `docs/adr/001-excecao-linguagem-gdscript.md`, `docs/adr/002-licenca-mit-nucleo.md` e `.specify/assessments/excecao-linguagem-licenca-nucleo/decision.md`, contando registros, declaracões e associações sem executar artefato algum.

- [X] T026 [P] [US3] Contar exatamente dois registros em `docs/adr/` — `docs/adr/001-excecao-linguagem-gdscript.md` e `docs/adr/002-licenca-mit-nucleo.md` — e conferir os campos de `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md §2` (FR-001, FR-009, SC-001).
- [X] T027 [P] [US3] Comparar `## Rastreabilidade` dos dois ADRs com a matriz de `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md §2.3`, confirmando 3/3 desvios, 4/4 referências e `DEC-R1` em ambos (FR-012, SC-002, SC-008).
- [X] T028 [P] [US3] Comparar textualmente as seções `## Não reabertura de Godot-first` dos dois ADRs com `docs/GDD - Insurgência.md:1265,1678`, registrando zero alterações nas passagens vinculantes (FR-003, SC-003, SC-004).
- [X] T029 [P] [US3] Verificar que `docs/adr/001-excecao-linguagem-gdscript.md` e `docs/adr/002-licenca-mit-nucleo.md` não contêm código, compilação, teste, gate de build, pipeline, serviço externo ou emenda da norma externa, conforme `specs/001-excecao-linguagem-licenca-nucleo/quickstart.md` etapa 9 (FR-010, SC-007).
- [X] T030 [P] [US3] Verificar nos dois ADRs que a aprovação humana aparece como autoridade necessária e que aprovação remota, integração contínua, merge e proteção de branch permanecem `[NV]`, conforme `specs/001-excecao-linguagem-licenca-nucleo/spec.md FR-011` e `docs/governance/sdd-phase-contracts.md:26` (FR-011, SC-006).
- [X] T031 [P] [US3] Verificar em `docs/adr/002-licenca-mit-nucleo.md` a pendência de mudança documental do servidor e o arquivo de licenças de terceiros como fora de escopo, comparando com `specs/001-excecao-linguagem-licenca-nucleo/spec.md ## Edge Cases` (FR-013, SC-005).
- [X] T032 [US3] Consolidar por leitura o resultado das etapas 7–9 (2/2 ADRs, 3/3 desvios, 4/4 referências) como evidência a registrar em `.specify/validations/excecao-linguagem-licenca-nucleo/validate.md` na fase R4 (portaria separada), sem escrever em `specs/001-excecao-linguagem-licenca-nucleo/quickstart.md`, sem afirmar execução de gate e registrando qualquer divergência com o ID FR/SC correspondente.

## Phase 6: Polish & Cross-Cutting Concerns

**Goal**: uniformizar terminologia, preservar os checklists como gate somente leitura e encerrar o escopo documental.

- [X] T033 [P] Revisar a consistência terminológica entre `docs/adr/001-excecao-linguagem-gdscript.md` e `docs/adr/002-licenca-mit-nucleo.md` quanto a Godot-first, MIT-only e rótulo `[NV]`, conforme `specs/001-excecao-linguagem-licenca-nucleo/contracts/adr-contract.md §7`.
- [X] T034 [P] Confirmar que os itens agregados `ADR-CHK001`–`ADR-CHK040` de `specs/001-excecao-linguagem-licenca-nucleo/checklists/adr-quality.md` permanecem `[ ]` e que os itens agregados `REQ-CHK001`–`REQ-CHK016` de `specs/001-excecao-linguagem-licenca-nucleo/checklists/requirements.md` permanecem `[x]`, sem marcação feita pelo gerador — a regra vale somente para `adr-quality.md`, cuja marcação `[x]` é decidida pelo revisor humano; `requirements.md` segue seu ciclo próprio de clarify (`docs/governance/sdd-phase-contracts.md:9,21`).
- [X] T035 Revisar os dois registros em `docs/adr/` quanto à ausência de terceiro ADR, de emenda da norma externa e de qualquer conteúdo vetado por `specs/001-excecao-linguagem-licenca-nucleo/spec.md FR-010`, preservando exatamente dois ADRs (FR-001).

## Dependencies

| Bloco | Tarefas | Dependência |
|---|---|---|
| Setup | T001–T003 | Nenhuma; gate de entrada da fase 11. |
| Foundational | T004–T007 | Regras fixadas por T001–T003. |
| US1 (P1) | T008–T015 | Esquema de T003 e regras de T004–T007; entrega isolada do ADR-001. |
| US2 (P1) | T016–T025 | Esquema de T003 e regras de T004–T007; entrega isolada do ADR-002. |
| US3 (P2) | T026–T032 | Materialização de T008–T025; somente leitura cruzada. |
| Polish | T033–T035 | T008–T032 concluídas. |

## Parallel Opportunities

| Oportunidade | Tarefas | Motivo |
|---|---|---|
| Criação do diretório e fixação do esquema | T002, T003 | Artefatos distintos, sem dependência entre si. |
| Regras compartilhadas | T004, T005, T006 | Leitura de fontes distintas, sem escrita concorrente no mesmo arquivo. |
| Materialização dos ADRs | Blocos T008–T015 e T016–T025 | Arquivos canônicos diferentes; cada bloco é entregável de forma independente. |
| Verificação cruzada | T026–T031 | Leitura somente dos dois ADRs e do contrato, sem escrita concorrente. |
| Polish | T033, T034 | Revisão e conferência independentes. |

## Implementation Strategy

Entrega incremental por user story, com MVP limitado à US1 (ADR-001 materializado e verificável isoladamente). A US2 é o segundo incremento independente; a US3 fecha a rastreabilidade por leitura cruzada. Nenhuma tarefa introduz código, compilação, teste automatizado, gate de build, pipeline, serviço externo ou emenda da norma externa (`spec.md` FR-010, SC-007). Gates de checklist são somente leitura e a marcação `[x]` de `adr-quality.md` é do revisor humano; R4 precede R7 como portarias do ciclo completo, e nenhuma afirma status remoto sem confirmação ao vivo.

## Marcos

| Marco | Faixa de tarefas | Resultado observável |
|---|---|---|
| M1 — Gate e estrutura prontos | T001–T003 | `docs/adr/` criado e esquema de campos fixado. |
| M2 — Regras de governança travadas | T004–T007 | Matriz 3/3 e 4/4, cláusula de não reabertura, limite documental e SemVer definidos. |
| M3 — ADR-001 entregue | T008–T015 | `docs/adr/001-excecao-linguagem-gdscript.md` completo e rastreável. |
| M4 — ADR-002 entregue | T016–T025 | `docs/adr/002-licenca-mit-nucleo.md` completo e rastreável. |
| M5 — Verificação local concluída | T026–T032 | 2/2 ADRs, 3/3 desvios, 4/4 referências e `[NV]` conferidos por leitura. |
| M6 — Consistência encerrada | T033–T035 | Polish aplicado; checklists preservados como gate somente leitura e escopo documental encerrado. |

## Format Validation

As 35 tarefas T001–T035 usam ID sequencial e único, descrição com caminho explícito e marcador `[P]`/`[USn]` conforme a fase; na fase implement todas estão marcadas `- [X]`. Analyze, implement, converge, validate e release são portarias separadas, não tarefas deste arquivo. O único checkbox `[ ]` legítimo restante pertence ao checklist de domínio `adr-quality.md`, sob revisão humana; nenhum campo da spec permanece pendente de definição.
