# Tasks: Emenda pontual do GDD a partir dos ADRs 001/002

**Feature**: `002-emenda-gdd-adrs` | **Created**: 2026-11-19 | **Fase**: 11 (tasks) do contrato SDD, conforme `docs/governance/sdd-phase-contracts.md:20`.

**Input**: `specs/002-emenda-gdd-adrs/spec.md` (US1/US2/US3, FR-001–FR-007, SC-001–SC-007, mapa de 7 passagens, `## Clarifications`), `specs/002-emenda-gdd-adrs/plan.md` (D1–D9), `specs/002-emenda-gdd-adrs/research.md` (RQ-1–RQ-10), `specs/002-emenda-gdd-adrs/data-model.md`, `specs/002-emenda-gdd-adrs/contracts/gdd-amendment-contract.md`, `specs/002-emenda-gdd-adrs/quickstart.md`, `specs/002-emenda-gdd-adrs/checklists/gdd-amendment.md`, `.specify/memory/constitution.md`.

**Caminho canônico de materialização**: `docs/GDD - Insurgência.md`, linhas `1389,1451,1459,1594,1659,1668,1673`. As âncoras `1265,1678` são premissa intocável, usadas só como baseline de comparação (SC-002). Os ADRs invocados são lidos como autoridade e não são alterados. Nenhum código, compilação, teste, gate, pipeline, serviço externo ou efeito remoto é produzido (spec SC-007).

**Formato obrigatório**: `- [ ] T### [P?] [USn?] descrição com caminho`. Cada tarefa aponta para ao menos uma das sete passagens do GDD, descreve verificação por leitura e pertence a uma fase Setup→Foundational→User Stories→Polish. IDs sequenciais e únicos, T001–T026. A marcação `[X]` pertence à portaria posterior de materialização, não a este arquivo. A regra de STOP por item `[ ]` vale somente para a revisão humana do checklist de domínio (`checklists/gdd-amendment.md`); itens ainda não revisados não bloqueiam a entrega documental. `checklists/requirements.md` segue seu ciclo próprio de clarify. Analyze, implement, converge, validate (R4) e release são portarias separadas, não tarefas deste arquivo. O GDD é o único artefato normativo de origem alterado: nenhuma tarefa escreve em `data-model.md`, contrato, spec ou checklists — tarefas que os citam são conferência por leitura, sem alterar, pois o conteúdo já existe no planejamento. Os gates determinísticos locais pertencem à portaria R4 (validate), não a tarefas desta feature (SC-003).

## Phase 1: Setup

**Goal**: validar o gate de entrada, conferir por leitura o baseline já registrado das sete passagens e das duas âncoras e o vocabulário de verificação por leitura antes de qualquer emenda, sem alterar artefatos fora do GDD.

- [X] T001 [P] Validar o gate de entrada da fase 11 conferindo por leitura que as sete passagens `1389,1451,1459,1594,1659,1668,1673` de `docs/GDD - Insurgência.md` estão mapeadas uma a uma em `specs/002-emenda-gdd-adrs/spec.md` (§Mapa de 7 passagens) e em `specs/002-emenda-gdd-adrs/contracts/gdd-amendment-contract.md` §2, sem duplicata de FR ou linha (FR-001–FR-007).
- [X] T002 [P] Conferir por leitura, sem alterar, que `specs/002-emenda-gdd-adrs/data-model.md` (§Âncora Godot-first) já registra o baseline textual das âncoras `1265,1678` e o baseline das sete passagens `1389,1451,1459,1659,1668,1673` de `docs/GDD - Insurgência.md`, verificando que os trechos divergentes ainda correspondem ao mapa da spec (SC-002; contrato §1).
- [X] T003 [P] Conferir por leitura, sem alterar, que `specs/002-emenda-gdd-adrs/contracts/gdd-amendment-contract.md` §4 já fixa as regras R-1…R-6 e o vocabulário de verificação por leitura aplicáveis a `docs/GDD - Insurgência.md:1389,1451,1459,1659,1668,1673`, verificando que nenhuma regra depende de execução de comando (SC-003; D9).

## Phase 2: Foundational

**Goal**: conferir por leitura as regras compartilhadas de autoridade, limite documental e formato de versionamento já fixadas no planejamento, que bloqueiam todas as histórias, sem alterar o contrato.

- [X] T004 [P] Conferir por leitura, sem alterar, que `specs/002-emenda-gdd-adrs/contracts/gdd-amendment-contract.md` §2 já fixa a redação canônica da referência ao ADR-002 para as seis passagens de licenciamento `docs/GDD - Insurgência.md:1389,1451,1459,1659,1668,1673`, verificando que o texto exige a citação no próprio GDD e veta numeração provisória (FR-001–FR-003, FR-005–FR-007; SC-004; RQ-2).
- [X] T005 [P] Conferir por leitura, sem alterar, que `specs/002-emenda-gdd-adrs/contracts/gdd-amendment-contract.md` §2 já fixa a redação canônica do DEFER e da referência ao ADR-001 para `docs/GDD - Insurgência.md:1594`, verificando que a regra veta "DEFER com gatilho", pipeline, build e efeito remoto (FR-004; SC-006; RQ-4).
- [X] T006 [P] Conferir por leitura, sem alterar, que `specs/002-emenda-gdd-adrs/contracts/gdd-amendment-contract.md` §4 já registra o limite documental aplicável a `docs/GDD - Insurgência.md:1389,1451,1459,1594,1659,1668,1673` — nenhum código, compilação, teste, gate de build, pipeline, serviço externo ou emenda da norma externa —, verificando que a restrição cobre as sete passagens (SC-007).
- [X] T007 Conferir por leitura, sem alterar, que `specs/002-emenda-gdd-adrs/contracts/gdd-amendment-contract.md` §5 já fixa somente o formato documental — versão SemVer `1.0.0`, changelog por IDs FR/`T###`/`BUG-###` e rollback documental —, usando `docs/GDD - Insurgência.md:1451` como passagem de reversão de exemplo e verificando que o procedimento preserva as âncoras `1265,1678`; a entidade `EM-002-1.0.0` e o campo `contagens` têm morada única em `specs/002-emenda-gdd-adrs/data-model.md` §Entidade Registro de emenda (contrato §5; D8).

## Phase 3: User Story 1 — Fechar N3 e a licença única no GDD (P1)

**Goal**: fazer o leitor determinar a licença vigente a partir do próprio GDD, com N3 respondida e a ressalva de terceiro esclarecida, sem tocar CI nem âncoras.

**Independent Test**: ler somente `docs/GDD - Insurgência.md:1389,1451,1459,1659,1668,1673` e verificar MIT única, ausência de alternativa e de AGPL no código do projeto, N3 respondida e ADR-002 citado em cada passagem.

- [X] T008 [P] [US1] Materializar a emenda de `docs/GDD - Insurgência.md:1389` (FR-001) restringindo a ressalva a servidores de terceiros, deixando explícito que ela não impõe AGPL ao código do projeto e preservando portabilidade e direito de saída, e verificar por leitura a presença do ADR-002 na passagem (SC-004; contrato §2 P-1389).
- [X] T009 [P] [US1] Materializar a emenda de `docs/GDD - Insurgência.md:1451` (FR-002) declarando MIT única para Núcleo e servidor, sem AGPL-3.0 e sem `COPYING`, mantendo a declaração MIT, e verificar por leitura a presença do ADR-002 na linha da tabela do MVP (contrato §2 P-1451).
- [X] T010 [P] [US1] Materializar a emenda de `docs/GDD - Insurgência.md:1459` (FR-003) substituindo o par "MIT/AGPL" por MIT única, preservando a afirmação de que licença e contrato protegem código e autoria, mas não o jogador de instância injusta, e verificar por leitura o ADR-002 na ressalva do MVP (contrato §2 P-1459).
- [X] T011 [P] [US1] Materializar a emenda de `docs/GDD - Insurgência.md:1659` (FR-005) registrando N3 como respondida pela MIT única, sem "MIT ou GPL" e sem reabertura da licença do motor, preservando a ressalva de que o motor não é o único motor MIT, e verificar por leitura o ADR-002 na nota do designer (contrato §2 P-1659).
- [X] T012 [P] [US1] Materializar a emenda de `docs/GDD - Insurgência.md:1668` (FR-006) retirando N3 da coluna de bloqueantes e registrando-a como decidida pelo ADR-002, sem condicionar a emenda a publicação externa, e verificar por leitura que N4, N6 e N8 permanecem intactos (contrato §2 P-1668).
- [X] T013 [P] [US1] Materializar a emenda de `docs/GDD - Insurgência.md:1673` (FR-007) removendo N3 da lista de perguntas não respondidas, mantendo N4, N6 e N8 e registrando que a Fase 0 segue com N3 respondido, e verificar por leitura o ADR-002 no texto de encerramento e a ausência de alteração nas âncoras `1265,1678` (contrato §2 P-1673).
- [X] T014 [US1] Verificar por leitura as seis passagens `docs/GDD - Insurgência.md:1389,1451,1459,1659,1668,1673`, contando 6 de 6 com ADR-002 citado e zero ocorrências de "MIT ou GPL", "MIT/AGPL", AGPL imposta ao código do projeto e `COPYING` (FR-001–FR-003, FR-005–FR-007; SC-001, SC-004, SC-005).

## Phase 4: User Story 2 — Reconciliar a promessa de integração contínua com o DEFER (P1)

**Goal**: substituir a promessa de CI no dia 1 pelo registro do DEFER, sem introduzir efeito remoto, gatilho ou pipeline.

**Independent Test**: ler somente `docs/GDD - Insurgência.md:1594` e verificar que a promessa desapareceu, o DEFER está registrado e o ADR-001 é citado.

- [X] T015 [P] [US2] Materializar a emenda de `docs/GDD - Insurgência.md:1594` (FR-004) substituindo "integração contínua desde o primeiro dia" pelo registro de que integração contínua e gate remoto ficam diferidos neste ciclo, sem efeito remoto, e verificar por leitura a presença do ADR-001 e a ausência de gatilho, pipeline, build ou serviço externo na linha de risco de sustentabilidade (contrato §2 P-1594).
- [X] T016 [US2] Verificar por leitura `docs/GDD - Insurgência.md:1594` confrontando a redação com `specs/002-emenda-gdd-adrs/contracts/gdd-amendment-contract.md` §4 R-3: 1 de 1 promessa reconciliada, DEFER vigente preservado e nenhum efeito remoto afirmado como obtido (SC-006; SC-003).

## Phase 5: User Story 3 — Auditar a emenda, as âncoras e a rastreabilidade (P2)

**Goal**: conferir por leitura cruzada que as sete passagens foram reconciliadas, que as âncoras estão intactas e que cada alteração identifica o ADR que a autoriza, sem efeito remoto.

**Independent Test**: ler as sete passagens e as duas âncoras de `docs/GDD - Insurgência.md`, contando cobertura e comparando texto, sem executar artefato algum.

- [X] T017 [P] [US3] Contar por leitura 7 de 7 passagens emendadas em `docs/GDD - Insurgência.md:1389,1451,1459,1594,1659,1668,1673` e confrontar cada uma com o estado-alvo de `specs/002-emenda-gdd-adrs/contracts/gdd-amendment-contract.md` §2, registrando o ID FR de qualquer divergência (SC-001).
- [X] T018 [P] [US3] Comparar textualmente as âncoras `docs/GDD - Insurgência.md:1265,1678` com o baseline de `specs/002-emenda-gdd-adrs/data-model.md` (§Âncora Godot-first), confirmando zero alterações e que nenhuma das sete passagens `1389,1451,1459,1659,1668,1673` as reescreveu (SC-002, SC-003).
- [X] T019 [P] [US3] Conferir por leitura 7 de 7 referências de ADR nas passagens `1389,1451,1459,1594,1659,1668,1673` de `docs/GDD - Insurgência.md`, com ADR-002 nas seis de licenciamento e ADR-001 em `1594`, sem numeração provisória (SC-004; contrato §1 e §4 R-1).
- [X] T020 [P] [US3] Conferir por leitura que as três ocorrências de N3 em `docs/GDD - Insurgência.md:1659,1668,1673` deixaram de ser pergunta aberta ou bloqueante, que N4, N6 e N8 permanecem como estavam e que zero promessas de CI não reconciliadas restam em `1594` (SC-005, SC-006).
- [X] T021 [P] [US3] Conferir por leitura que nenhuma das sete passagens `docs/GDD - Insurgência.md:1389,1451,1459,1594,1659,1668,1673` contém código, compilação, teste, gate de build, pipeline, serviço externo, emenda da norma externa ou afirmação de aprovação remota, CI executada, merge, proteção ou publicação obtida (SC-007, SC-003).
- [X] T022 [US3] Consolidar por leitura, sem alterar artefato algum, a partir das passagens `docs/GDD - Insurgência.md:1389,1451,1459,1659,1668,1673` as contagens 7/7 passagens emendadas, 0/2 âncoras alteradas, 7/7 ADRs citados e 0 ocorrências proibidas no registro documental `EM-002-1.0.0` em `specs/002-emenda-gdd-adrs/data-model.md` §Entidade Registro de emenda (morada única do registro e de `contagens`), sem escrever em portaria separada, sem afirmar execução de gate e registrando divergência com ID FR/SC ou `BUG-###` (data-model §Entidade Registro de emenda; contrato §5.2, §7).

## Phase 6: Polish & Cross-Cutting Concerns

**Goal**: uniformizar terminologia, preservar os checklists como gate somente leitura e encerrar o escopo documental.

- [X] T023 [P] Revisar a consistência terminológica entre as seis passagens de licenciamento `docs/GDD - Insurgência.md:1389,1451,1459,1659,1668,1673` e a passagem de CI `1594`, conferindo por leitura o uso uniforme de "MIT única", "diferido neste ciclo", rótulo `[NV]` e numeração materializada de ADR (contrato §4 R-1–R-6).
- [X] T024 [P] Confirmar por leitura das sete passagens `docs/GDD - Insurgência.md:1389,1451,1459,1594,1659,1668,1673` que os itens agregados `ADR-CHK001`–`ADR-CHK050` de `specs/002-emenda-gdd-adrs/checklists/gdd-amendment.md` permanecem `[ ]`, que nenhum foi marcado como aprovado pelo gerador e que `specs/002-emenda-gdd-adrs/checklists/requirements.md` segue seu ciclo próprio sem alteração nesta tarefa (contrato de fase itens 9 e 21).
- [X] T025 [P] Revisar por leitura a delimitação de escopo em `docs/GDD - Insurgência.md`, confirmando que exatamente as sete passagens `1389,1451,1459,1594,1659,1668,1673` foram emendadas, que nenhuma oitava passagem ou seção vizinha foi reescrita e que as âncoras `1265,1678` permanecem intactas (SC-002; contrato §2 Regra 4).
- [X] T026 Revisar por leitura, sem alterar, a conformidade final do registro documental `EM-002-1.0.0` já preenchido em `specs/002-emenda-gdd-adrs/data-model.md` §Entidade Registro de emenda — versão, data, changelog e `contagens` —, usando `docs/GDD - Insurgência.md:1451` como passagem de reversão de exemplo e verificando que o procedimento nunca apaga histórico, altera âncoras, emenda a norma externa, decide licenças de terceiros ou afirma efeito remoto (SC-003, SC-007).

## Dependencies

| Bloco | Tarefas | Dependência |
|---|---|---|
| Setup | T001–T003 | Nenhuma; gate de entrada da fase 11. |
| Foundational | T004–T007 | Regras e baselines fixados por T001–T003. |
| US1 (P1) | T008–T014 | Regras de T004–T007; entrega isolada das seis passagens de licenciamento. |
| US2 (P1) | T015–T016 | Regras de T004–T007; entrega isolada da passagem de CI. |
| US3 (P2) | T017–T022 | Materialização de T008–T016; somente leitura cruzada. |
| Polish | T023–T026 | T008–T022 concluídas. |

## Parallel Opportunities

| Oportunidade | Tarefas | Motivo |
|---|---|---|
| Gate e baseline | T001, T002, T003 | Leitura de fontes distintas, sem escrita concorrente no mesmo arquivo. |
| Regras compartilhadas | T004, T005, T006 | Autoridades e limites distintos, sem escrita concorrente. |
| Passagens de licenciamento | T008–T013 | Linhas distintas do mesmo documento, cada uma entregável de forma verificável e isolada. |
| Verificação cruzada | T017–T021 | Leitura somente das sete passagens, das duas âncoras e do contrato, sem escrita concorrente. |
| Polish | T023, T024, T025 | Revisões independentes de terminologia, checklist e escopo. |

## Estratégia de Entrega

Entrega incremental por user story, com MVP limitado à US1 (seis passagens de licenciamento emendadas e verificáveis isoladamente). A US2 é o segundo incremento independente (passagem `1594`); a US3 fecha a auditoria por leitura cruzada. Nenhuma tarefa introduz código, compilação, teste automatizado, gate de build, pipeline, serviço externo, efeito remoto ou emenda da norma externa (spec SC-007). Checklists são gate somente leitura e a marcação `[x]` do checklist de domínio é decidida pelo revisor humano. Analyze, implement, converge, validate e release são portarias do ciclo completo, posteriores a esta fase e não incluídas como tarefas.

## Marcos

| Marco | Faixa de tarefas | Resultado observável |
|---|---|---|
| M1 — Gate e baseline prontos | T001–T003 | Sete passagens e duas âncoras mapeadas; regras R-1…R-6 conferidas por leitura no contrato. |
| M2 — Regras de governança travadas | T004–T007 | Autoridade ADR-002/ADR-001 por passagem, limite documental e rollback documental definidos. |
| M3 — Licenciamento e N3 fechados | T008–T014 | 6 de 6 passagens `1389,1451,1459,1659,1668,1673` com MIT única e ADR-002 citado. |
| M4 — CI reconciliada | T015–T016 | `1594` sem promessa de CI dia 1, com DEFER e ADR-001, sem efeito remoto. |
| M5 — Auditoria concluída | T017–T022 | 7/7 passagens, 0/2 âncoras alteradas, 7/7 ADRs citados e contagens consolidadas. |
| M6 — Consistência encerrada | T023–T026 | Polish aplicado, checklists preservados como gate somente leitura e escopo documental encerrado. |

## Format Validation

As 26 tarefas T001–T026 usam ID sequencial e único, checkbox `- [ ]`, descrição com caminho explícito e marcador `[P]`/`[USn]` conforme a fase. Cada tarefa aponta para ao menos uma das sete passagens `1389,1451,1459,1594,1659,1668,1673` e descreve verificação por leitura. Fases na ordem Setup→Foundational→User Stories→Polish; nenhuma portaria posterior a tasks aparece como tarefa. Nenhum campo da spec permanece pendente de definição e nenhum item do checklist de domínio é marcado neste arquivo.
