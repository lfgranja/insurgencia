# Specification Quality Checklist: 062-coleta-eficiente

**Purpose**: validar a completude e a qualidade da especificação antes do planejamento.
**Created**: 2026-11-25
**Feature**: [spec.md](../spec.md) — `062-coleta-eficiente` · **Feature ID**: 062 · **Status**: Especificado.

> Formato do contrato: itens `- [ ] CHK### …`, rastreáveis a IDs de FR/SC da spec.
> Os marcadores permanecem `[ ]` conforme a convenção reviewer-owned do hub: a avaliação do autor não substitui a aprovação do revisor de requisitos. `[ ]` **não** significa que gates, testes, build, commit ou PR foram executados — toda execução permanece **[NV]** até o orquestrador/SWE.

## Content Quality

- [ ] CHK001 — A spec descreve o comportamento observável (`coleta_eficiente` com destino na própria posição, custo 15 Fôlego, buff 150 ticks, coleta em 90 vs 180 ticks, recarga 300, sem acumulação) sem fixar sintaxe de implementação, sem promover números `[HIPÓTESE]` a cânone e sem reescrever o GDD — rastreia: spec.md integral; §Contexto e Orçamento; FR-001, FR-003.
- [ ] CHK002 — O foco é valor material para o jogador do Camponês no cliente Bevy e para a automação via `insurgencia-servidor`, não diagnóstico ou exposição de hash — rastreia: US1–US4, FR-004, SC-004.
- [ ] CHK003 — Escrita para stakeholders não técnicos: cenários Given/When/Then, por que cada prioridade e teste independente, com nome de tecla e efeitos legíveis — rastreia: User Scenarios, Edge Cases, Acceptance Scenarios, Assumptions.
- [ ] CHK004 — Todas as seções obrigatórias estão preenchidas e nenhuma permanece com placeholder: User Scenarios, Edge Cases, Requirements, Success Criteria, Clarifications, Assumptions e Out of scope — rastreia: spec.md integral.

## Requirement Completeness

- [ ] CHK005 — Nenhum marcador de pendência resta; a única ambiguidade registrada foi respondida na Session 2026-11-25 (tecla `K` é protótipo `[HIPÓTESE]`; passiva de Ouro/XP fora de escopo por ausência de primitiva no núcleo) — rastreia: §Clarifications / Session 2026-11-25; R3.
- [ ] CHK006 — Quatro requisitos funcionais numerados e testáveis cobrem contrato, validação, efeito e consumidor, com limites explícitos — rastreia: FR-001 (novo token `coleta_eficiente`, whitelist com 16 tokens, `acao_exige_destino` true), FR-002 (destino = posição, 15 Fôlego, recarga 300, sem *stacking*), FR-003 (buff 150, 90 vs 180 ticks, monotônicos só no aceite, hash e decrescentes), FR-004 (`K` + HUD).
- [ ] CHK007 — Requisitos inequívocos, com limites numéricos e exceções nomeadas (recusa preserva Fôlego/IDs/hash; reaplicação sob buff não acumula; passiva Ouro/XP adiada) — rastreia: FR-001–FR-004; User Scenarios; Edge Cases; Out of scope.
- [ ] CHK008 — Critérios de sucesso mensuráveis, com grandeza observável e estado **[NV]** quando dependem de execução ainda não realizada — rastreia: SC-001 (16 tokens/aridade), SC-002 (destino = posição, ≥ 15 Fôlego, recarga 0 vs 300, sem *stacking*), SC-003 (150/90/180/300 e hash), SC-004 (tecla `K`, HUD, comandos de prova e backlog).

## Traceability

| Item | Rastreia | Verificável por |
|:--|:--|:--|
| CHK001 | FR-001, FR-003 | inspeção da spec e do rótulo `[HIPÓTESE]` |
| CHK002 | US1–US4, FR-004, SC-004 | leitura dos cenários e do consumidor nomeado |
| CHK003 | User Scenarios, Edge Cases, Assumptions | presença de Given/When/Then e do teste independente |
| CHK004 | seções obrigatórias | ausência de placeholders |
| CHK005 | §Clarifications, R3 | exatamente 1 Q/A respondida e nenhum marcador de pendência |
| CHK006 | FR-001–FR-004 | presença dos quatro requisitos e dos limites |
| CHK007 | FR-001–FR-004, Out of scope | limites e exceções explícitos |
| CHK008 | SC-001–SC-004 | grandezas mensuráveis e rótulo **[NV]** |

## Notes

- Oito itens rastreáveis de oito (100%, acima do mínimo de 80% do contrato).
- Nenhum item promovido a `[x]`: os marcadores são de propriedade do revisor de requisitos.
- Nenhuma pergunta foi feita ao usuário nesta etapa: o aclaramento único veio da decisão registrada em `.specify/memory/STATE.md` (2026-11-25T06:40:58Z).
- A fronteira documental é estrita: somente `spec.md`, `plan.md`, `tasks.md` e este checklist em `specs/062-coleta-eficiente/`. Nenhum arquivo de código, GDD, CI, contrato de rede ou `STATE.md` é escrito; gates, testes, formatadores, build, commit, push e PR ficam **[NV]** até o orquestrador/SWE.
