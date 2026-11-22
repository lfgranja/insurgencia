# Specification Quality Checklist: Godot headless benchmark

**Purpose**: Validar a completude e a qualidade da especificação antes da fase de planejamento.
**Created**: 2026-11-20
**Feature**: [spec.md](../spec.md) — `006-godot-headless-benchmark`

> Formato do contrato: itens `- [ ] CHK### …`, rastreáveis para seções/IDs da spec.
> Os marcadores permanecem `[ ]` conforme a convenção reviewer-owned adotada no hub; a avaliação do autor não substitui a aprovação do revisor de requisitos.

## Content Quality

- [ ] CHK001 — A spec descreve identidade de release, integridade e custo por tick sem fixar comandos de instalação, caminhos de binário ou sintaxe de execução — rastreia: spec.md integral; §Contexto e Orçamento.
- [ ] CHK002 — O foco é valor de auditoria e linha de base: proveniência provada, medição registrada e carga corretamente rotulada — rastreia: US1–US4, SC-001–SC-010.
- [ ] CHK003 — Escrita para stakeholders não técnicos: mantenedor, implementadores da simulação e auditores — rastreia: User Scenarios, Assumptions, Clarifications.
- [ ] CHK004 — Todas as seções obrigatórias estão preenchidas e nenhuma está vazia — rastreia: User Scenarios, Edge Cases, Requirements, Key Entities, Success Criteria, Assumptions, Clarifications, Contexto e Orçamento.

## Requirement Completeness

- [ ] CHK005 — Nenhum marcador de pendência permanece; as três ambiguidades estão respondidas e rastreáveis — rastreia: §Clarifications / Session 2026-11-20; SC-009.
- [ ] CHK006 — Catorze requisitos funcionais numerados e testáveis cobrem release, checksum, binário, carga, saída, amostras, referência, rotulagem, evidência, salvaguardas e escopo — rastreia: FR-001–FR-014.
- [ ] CHK007 — Requisitos inequívocos, com DEVE/NÃO DEVE e exceções nomeadas — rastreia: FR-001–FR-014; Edge Cases.
- [ ] CHK008 — Critérios de sucesso mensuráveis, com baseline e meta, e estado `[NV]` quando dependem de execução ainda não realizada — rastreia: SC-001–SC-010.
- [ ] CHK009 — Critérios tecnologicamente agnósticos: contagens, valores registrados e conformidade com referência, sem exigir ferramenta ou caminho específico — rastreia: SC-001–SC-010.
- [ ] CHK010 — Critérios cobrem explicitamente o par de checksums, a identidade do binário, a medição com dispersão e a rotulagem sintética — rastreia: SC-002, SC-003, SC-005, SC-006, SC-007.
- [ ] CHK011 — Todos os cenários de aceitação estão em formato Given/When/Then — rastreia: US1 (3), US2 (2), US3 (4), US4 (4).
- [ ] CHK012 — Casos-limite identificados, inclusive divergência de checksum, versão inesperada, amostra atípica, mediana acima da referência, mudança de carga e não reexecução nesta etapa — rastreia: Edge Cases (8 casos); FR-003, FR-004, FR-007, FR-008.
- [ ] CHK013 — Escopo claramente limitado: sem partida real, sem renderização/rede, sem SLA, sem gates, sem stage/commit e sem portarias posteriores a tasks — rastreia: Assumptions; FR-005, FR-012, FR-013; decisão R1.
- [ ] CHK014 — Dependências e premissas identificadas — rastreia: Assumptions (6 premissas); Clarifications (3 respostas por evidência local e pedido autorizado).

## Feature Readiness

- [ ] CHK015 — Cada requisito funcional possui aceitação verificável por contagem, presença de registro ou leitura de valores — rastreia: FR-001–FR-014 ↔ US1/US2/US3/US4 e SC-001–SC-010.
- [ ] CHK016 — Cenários cobrem os fluxos primários: release verificada, binário identificado, medição executada e auditoria de rotulagem — rastreia: US1, US2, US3, US4.
- [ ] CHK017 — R3 respeitado: no máximo três esclarecimentos, um por vez, resolvidos antes do planejamento — rastreia: §Clarifications / Session 2026-11-20 (3 Q/A e nota final).
- [ ] CHK018 — Nenhuma seção de planejamento antecipada na spec, além da seção documental de contexto — rastreia: Requirements, Success Criteria, Key Entities e §Contexto e Orçamento.
- [ ] CHK019 — R1 registrado com `go` legítimo: validade `strong`, evidência `strong` e Opção A nomeada — rastreia: decision.md §Scorecard e §Verdict & Rationale.
- [ ] CHK020 — A natureza sintética da carga e a não reexecução nesta etapa são tratadas como requisitos de rotulagem e citação, não como afirmação de desempenho de jogo — rastreia: FR-005, FR-009, FR-010, FR-013; SC-007, SC-008, SC-010; Clarifications Q1 e Q3.

## Notes

- Vinte itens rastreáveis de vinte (100%, acima do mínimo de 80% do contrato).
- Nenhum item promovido a `[x]`: os marcadores são de propriedade do revisor de requisitos.
- Nenhuma pergunta foi feita ao usuário; as três ambiguidades foram resolvidas por evidência local, pedido autorizado e política de rotulagem.
- Itens que venham a ficar incompletos exigem atualização da spec antes da fase de planejamento.
