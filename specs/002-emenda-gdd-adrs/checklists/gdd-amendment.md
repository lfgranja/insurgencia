# GDD Amendment Requirements Quality Checklist: Emenda pontual do GDD a partir dos ADRs 001/002

**Purpose**: validar a qualidade, completude e rastreabilidade dos requisitos que regem a emenda do GDD — não validar implementação, comportamento de sistema ou resultado de execução.
**Created**: 2026-11-19
**Feature**: [spec.md](../spec.md) — `002-emenda-gdd-adrs`

**Note**: checklist de domínio gerado na fase 9, com apoio em `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/gdd-amendment-contract.md` e `quickstart.md`.
**Review Ownership**: artefato de revisão de qualidade de requisitos, de propriedade do revisor. Marcar um item como aprovado significa que o revisor confirmou que o critério de qualidade de requisitos está satisfeito; não significa implementação concluída.
**Marker Semantics**: neste arquivo de domínio os itens permanecem `[ ]`; a marcação de aprovado é decidida pelo revisor, nunca pelo gerador. A fase de implementação lê o estado dos checkboxes como gate e não altera marcadores.

## Completude dos requisitos

- [ ] CHK001 — Estão definidas com precisão as sete passagens do GDD que compõem o escopo, uma por requisito funcional? [Spec §FR-001–FR-007; Spec §Key Entities/Mapa de 7 passagens; Completeness]
- [ ] CHK002 — A spec declara explicitamente quais passagens são autorizadas pelo ADR-002 e qual é autorizada pelo ADR-001? [Spec §Key Entities; Research §RQ-2; Completeness]
- [ ] CHK003 — Estão especificados os estados-alvo de todas as sete passagens, sem deixar nenhuma apenas descrita como "a corrigir"? [Spec §Mapa de 7 passagens/Estado-alvo; Contract §2; Completeness]
- [ ] CHK004 — Há requisito explícito de que o texto ao redor das passagens permaneça intacto? [Spec §FR-007; Spec §Assumptions; Contract §4 R-5; Gap]
- [ ] CHK005 — Estão previstos requisitos para a ressalva de servidor de terceiros sem decidir licenças fora da autoridade local? [Spec §FR-001; Spec §Edge Cases; Research §RQ-3]
- [ ] CHK006 — Estão definidos os critérios que impedem a reintrodução de "DEFER com gatilho", pipeline ou build na passagem de CI? [Spec §FR-004; Spec §Edge Cases; Research §RQ-4; Coverage]
- [ ] CHK007 — A especificação declara explicitamente o destino das perguntas N4, N6 e N8 após a emenda? [Spec §FR-007; Spec §SC-005; Contract §2 P-1673; Completeness]

## Clareza e inequivocidade

- [ ] CHK008 — O termo "MIT única" está definido de modo inequívoco, sem alternativa "ou GPL" e sem AGPL para o código do projeto? [Spec §Clarifications; Spec §FR-002; Ambiguity]
- [ ] CHK009 — É claro o que distingue "código do projeto" de "servidor de terceiros" na ressalva de licenciamento? [Spec §FR-001; Spec §Edge Cases; Research §RQ-3; Clarity]
- [ ] CHK010 — A expressão "diferidos neste ciclo" está redigida de forma verificável, sem sugerir gatilho ou prazo implícito? [Spec §FR-004; Spec §Clarifications; Contract §4 R-3; Clarity]
- [ ] CHK011 — É inequívoco o que significa "N3 respondida" para efeito de bloqueio da "Primeira publicação"? [Spec §FR-005; Spec §FR-006; Spec §SC-005; Ambiguity]
- [ ] CHK012 — A numeração de ADR usada na rastreabilidade está identificada sem ambiguidade entre a materializada e a provisória? [Spec §Clarifications; Research §RQ-2; Contract §1; Ambiguity]
- [ ] CHK013 — O significado de `[NV]` está definido para aprovação remota, CI, merge, proteção de branch e publicação externa? [Spec §SC-003; Plan §Constitution Check VI; Contract §4 R-6; Clarity]
- [ ] CHK014 — A expressão "verificação por leitura" está caracterizada de modo a excluir execução de comandos? [Spec §US3/Independent Test; Plan §D9; Quickstart §Tipo; Clarity]

## Consistência e não contradição

- [ ] CHK015 — Os seis requisitos de licenciamento são consistentes entre si quanto à ausência de AGPL e de cláusula alternativa? [Spec §FR-001–FR-003, §FR-005–FR-007; Contract §4 R-2; Consistency]
- [ ] CHK016 — O tratamento de N3 é consistente entre a nota do designer, a tabela bloqueante e o texto de encerramento? [Spec §FR-005–FR-007; Spec §SC-005; Contract §2; Consistency]
- [ ] CHK017 — A redação exigida para a passagem de CI é coerente com o DEFER vigente e com o ADR-001? [Spec §FR-004; Spec §Assumptions; Research §RQ-4; Conflict]
- [ ] CHK018 — A posição MIT única é coerente com a preservação da licença MIT vigente e da ressalva de que o motor não é o único MIT? [Spec §FR-002; Spec §FR-005; ADR-002 §Licenciamento; Conflict]
- [ ] CHK019 — As contagens de 7 passagens, 7 FR, 2 âncoras e 7 critérios de sucesso são coerentes entre si? [Spec §FR-001–FR-007, §SC-001–SC-007; Plan §Technical Context/Scale; Consistency]
- [ ] CHK020 — A autoridade atribuída a cada passagem no mapa é consistente com a divisão ADR-001/ADR-002 declarada na spec? [Spec §Key Entities; Contract §2; Research §RQ-2; Consistency]
- [ ] CHK021 — A exigência de não alterar as âncoras é consistente com a decisão de não reabertura Godot-first registrada nos ADRs? [Spec §SC-002; ADR-001 §Não reabertura; ADR-002 §Não reabertura; Conflict]

## Rastreabilidade e identificadores

- [ ] CHK022 — Cada passagem possui identificador estável e requisito funcional correspondente, sem duplicata? [Spec §Mapa de 7 passagens; Contract §2; Traceability]
- [ ] CHK023 — A associação FR → passagem → `T###` está completa para FR-001 a FR-007? [Spec §FR-001–FR-007; Contract §3.1; Traceability]
- [ ] CHK024 — Está definida a exigência de que o ADR que decide apareça no próprio GDD, e não apenas em fonte externa? [Spec §SC-004; Spec §FR-001–FR-007; Contract §4 R-1; Coverage]
- [ ] CHK025 — A distinção entre autoridade local (`ADR-001`/`ADR-002`), autorização (`DEC-R1`) e achado (`BUG-###`) está documentada? [Spec §Key Entities; Contract §1, §3.2; Clarity]
- [ ] CHK026 — Há regra explícita que impede o uso da numeração provisória de ADR na rastreabilidade? [Spec §Edge Cases; Research §RQ-2; Contract §1; Traceability]
- [ ] CHK027 — Está definido o identificador de achados documentais e o que ele não autoriza? [Contract §3.2; Contract §5.3; Spec §SC-007; Completeness]
- [ ] CHK028 — A rastreabilidade dos critérios de sucesso para os requisitos funcionais está estabelecida sem lacuna? [Spec §SC-001–SC-007 ↔ §FR-001–FR-007; Checklists/requirements.md CHK015; Traceability]

## Cobertura de cenários e casos-limite

- [ ] CHK029 — Está coberta a hipótese de a ressalva AGPL de terceiro ser lida como decisão sobre o código do projeto? [Spec §Edge Cases; Spec §FR-001; Research §RQ-3; Gap]
- [ ] CHK030 — Está coberta a hipótese de reaparecer a numeração provisória de ADR na documentação? [Spec §Edge Cases; Research §RQ-2; Contract §1; Coverage]
- [ ] CHK031 — Está coberta a hipótese de ressurgir a sugestão de "DEFER com gatilho"? [Spec §Edge Cases; Spec §FR-004; Research §RQ-4; Coverage]
- [ ] CHK032 — Está coberta a hipótese de aprovação humana dos ADRs ainda constar como pendente? [Spec §Edge Cases; Spec §Clarifications; Research §RQ-10; Gap]
- [ ] CHK033 — Está coberta a hipótese de N3 deixar de bloquear a publicação sem publicação externa verificada? [Spec §Edge Cases; Spec §FR-006; Research §RQ-5; Coverage]
- [ ] CHK034 — Está coberta a hipótese de a emenda ameaçar as passagens da decisão Godot-first? [Spec §Edge Cases; Spec §SC-002; Contract §1; Coverage]
- [ ] CHK035 — Está coberto o caso de uma oitava passagem ou seção vizinha ser reescrita? [Spec §Assumptions; Contract §2 Regra 4, §5.3; Gap]

## Qualidade dos critérios de aceite

- [ ] CHK036 — O critério SC-001 explicita baseline e meta para a redução de divergências? [Spec §SC-001; Measurability]
- [ ] CHK037 — O critério SC-002 define o que conta como alteração de âncora de modo objetivamente comparável? [Spec §SC-002; Data-model §Âncora Godot-first; Measurability]
- [ ] CHK038 — O critério SC-004 é mensurável por contagem de passagens com referência ao ADR? [Spec §SC-004; Contract §4 R-1; Measurability]
- [ ] CHK039 — O critério SC-005 explicita as três ocorrências de N3 como baseline e zero como meta? [Spec §SC-005; Spec §FR-005–FR-007; Measurability]
- [ ] CHK040 — O critério SC-007 enumera todas as categorias de conteúdo proibido no resultado? [Spec §SC-007; Contract §4 R-6, §7 item 7; Acceptance Criteria]
- [ ] CHK041 — Os critérios de sucesso permanecem tecnologicamente agnósticos e independentes de execução? [Spec §SC-001–SC-007; Plan §Constitution Check I e VI; Measurability]

## Premissas, dependências e limite documental

- [ ] CHK042 — As premissas de natureza documental e ausência de runtime estão registradas e apoiadas em fontes locais? [Spec §Assumptions; Plan §Technical Context; Assumption]
- [ ] CHK043 — A dependência da emenda em relação à autorização do pedido e ao veredito R1 está documentada sem afirmar aprovação remota? [Spec §Clarifications; Research §RQ-10; Contract §5.3; Assumption]
- [ ] CHK044 — O limite de verificação "até confirmação ao vivo" está formulado como restrição de requisito? [Spec §SC-003; Plan §Constitution Check VI; Contract §6; Assumption]
- [ ] CHK045 — A decisão sobre licenças de terceiros está registrada como fora de escopo, sem resolução implícita? [Spec §Assumptions; ADR-002 §Fora de escopo; Contract §6; Dependency]
- [ ] CHK046 — A precedência da governança do alvo sobre documento normativo externo está declarada como premissa? [Spec §Assumptions; Constitution §Normas de trabalho; Research §RQ-2; Assumption]

## Qualidade do contrato, do modelo e do guia

- [ ] CHK047 — O contrato especifica as sete passagens, IDs FR/T, regras de redação e rollback sem deixar seção vazia? [Plan §Phase 0 and Phase 1 Outputs; Contract §1–§7; Completeness]
- [ ] CHK048 — As transições de estado das passagens estão descritas de modo a impedir alteração de âncora e reabertura do pilar? [Data-model §GDD, §Passagem de emenda; Contract §1; Coverage]
- [ ] CHK049 — O procedimento de verificação declara como não aplicáveis os comandos de setup, build e gate, com motivo? [Quickstart §Comandos de setup, §Comandos de teste; Plan §D9; Gap]
- [ ] CHK050 — As decisões de pesquisa registram alternativas rejeitadas e evidência contrária sem inventar fontes externas? [Research §Decisões de pesquisa, §Evidência contrária; Spec §Assumptions; Completeness]

## Notes

- 50 itens, todos com pelo menos uma referência a spec, plano, pesquisa, contrato, modelo ou guia (100% rastreáveis; mínimo contratual 80%).
- Nenhum item foi marcado como aprovado; a avaliação pertence ao revisor de requisitos.
- Anexar novos itens ao final da categoria adequada, continuando a numeração em CHK051, sem editar, renumerar ou remover itens existentes (append-only).
- `checklists/requirements.md` mantém ciclo próprio de `/speckit.specify` e `/speckit.clarify`; este arquivo não o substitui nem o altera.
- Itens que apontarem lacuna devem ser resolvidos na spec, no plano ou no contrato antes da fase de implementação.
- Namespace de agregação (sem renumerar itens existentes, append-only): os IDs deste arquivo são referidos como `ADR-CHK001`…`ADR-CHK050` quando agregados em `tasks.md`, rastreabilidade ou validação, conforme a convenção de agregação ADR-CHK já usada no ciclo 001.
