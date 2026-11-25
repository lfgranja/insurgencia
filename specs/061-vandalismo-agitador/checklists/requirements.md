# Specification Quality Checklist: 061-vandalismo-agitador

**Purpose**: validar a completude e a qualidade da especificação antes do planejamento.
**Created**: 2026-11-25
**Feature**: [spec.md](../spec.md) — `061-vandalismo-agitador` · **Feature ID**: 061 · **Status**: Especificado.

> Formato do contrato: itens `- [ ] CHK### …`, rastreáveis a IDs de FR/SC da spec.
> Os marcadores permanecem `[ ]` conforme a convenção reviewer-owned do hub: a avaliação do autor não substitui a aprovação do revisor de requisitos. `[ ]` **não** significa que gates, testes, build, commit ou PR foram executados — toda execução permanece **[NV]** até o orquestrador/SWE.

## Content Quality

- [ ] CHK001 — A spec descreve o comportamento observável (`vandalismo` com destino, dano 120, silêncio 45 ticks, cancelamento de canalização) sem fixar sintaxe de implementação, sem promover números `[HIPÓTESE]` a cânone e sem reescrever o GDD — rastreia: spec.md integral; §Contexto e Orçamento; FR-001, FR-003.
- [ ] CHK002 — O foco é valor material para o jogador da Função Agitadora no cliente Bevy e para a automação via `insurgencia-servidor`, não diagnóstico ou exposição de hash — rastreia: US1–US4, FR-004, SC-004.
- [ ] CHK003 — Escrita para stakeholders não técnicos: cenários Given/When/Then, por que cada prioridade e teste independente, com nomes de tecla e efeitos legíveis — rastreia: User Scenarios, Acceptance Scenarios, Assumptions.
- [ ] CHK004 — Todas as seções obrigatórias estão preenchidas e nenhuma permanece com placeholder — rastreia: User Scenarios, Edge Cases (quando aplicável), Requirements, Success Criteria, Clarifications, Assumptions, Out of scope.

## Requirement Completeness

- [ ] CHK005 — Nenhum marcador de pendência resta; a única ambiguidade registrada foi respondida na Session 2026-11-25 (tecla `H` é protótipo `[HIPÓTESE]`; `Q` pertence a `atacar_forte`) — rastreia: §Clarifications / Session 2026-11-25; R3.
- [ ] CHK006 — Quatro requisitos funcionais numerados e testáveis cobrem contrato, validação, efeito e consumidor, com limites explícitos — rastreia: FR-001 (15º token, `acao_exige_destino` true), FR-002 (alvo, 2 m, 20 Fôlego, 210 ticks), FR-003 (120 dano, 45 silêncio, cancelamento, monotônicos só no aceite, hash), FR-004 (`H` + HUD).
- [ ] CHK007 — Requisitos inequívocos, com limites numéricos e exceções nomeadas (recusa preserva estado e monotônicos; `Q` continua em `atacar_forte`) — rastreia: FR-001–FR-004; User Scenarios; Out of scope.
- [ ] CHK008 — Critérios de sucesso mensuráveis, com grandeza observável e estado **[NV]** quando dependem de execução ainda não realizada — rastreia: SC-001 (15 tokens/aridade), SC-002 (≤ 2 m, ≥ 20 Fôlego, recarga 0 vs. 210), SC-003 (120/45/210 e hash), SC-004 (tecla `H`, HUD, comandos de prova e backlog).

## Traceability

| Item | Rastreia | Verificável por |
|:--|:--|:--|
| CHK001 | FR-001, FR-003 | inspeção da spec e do rótulo `[HIPÓTESE]` |
| CHK002 | US1–US4, FR-004, SC-004 | leitura dos cenários e do consumidor nomeado |
| CHK003 | User Scenarios, Assumptions | presença de Given/When/Then e do teste independente |
| CHK004 | seções obrigatórias | ausência de placeholders |
| CHK005 | §Clarifications, R3 | exatamente 1 Q/A respondida e nenhum marcador de pendência |
| CHK006 | FR-001–FR-004 | presença dos quatro requisitos e dos limites |
| CHK007 | FR-001–FR-004, Out of scope | limites e exceções explícitos |
| CHK008 | SC-001–SC-004 | grandezas mensuráveis e rótulo **[NV]** |

## Notes

- Oito itens rastreáveis de oito (100%, acima do mínimo de 80% do contrato).
- Nenhum item promovido a `[x]`: os marcadores são de propriedade do revisor de requisitos.
- Nenhuma pergunta foi feita ao usuário nesta etapa: aclaramento único veio da decisão registrada em `.specify/memory/STATE.md` (2026-11-25T05:10:03Z).
- A fronteira documental é estrita: somente `spec.md`, `plan.md`, `tasks.md` e este checklist em `specs/061-vandalismo-agitador/`. Nenhum arquivo de código, GDD, CI, contrato de rede ou `STATE.md` é escrito; gates, testes, formatadores, build, commit, push e PR ficam **[NV]** até o orquestrador/SWE.
