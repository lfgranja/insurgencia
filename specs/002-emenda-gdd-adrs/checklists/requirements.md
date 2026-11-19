# Specification Quality Checklist: Emenda pontual do GDD a partir dos ADRs 001/002

**Purpose**: Validar a completude e a qualidade da especificação antes da fase de planejamento.
**Created**: 2026-11-19
**Feature**: [spec.md](../spec.md) — `002-emenda-gdd-adrs`

> Formato do contrato: itens `- [x] CHK### …`, rastreáveis para seções/IDs da spec.
> Revalidado em 2026-11-19 após a sessão de clarificação: **18/18 itens aprovados**.

## Content Quality

- [x] CHK001 — Nenhum detalhe de implementação (comandos, caminhos de arquivo, stack, APIs) — rastreia: spec.md integral; seção "Contexto e Orçamento" (descritiva, sem comandos).
- [x] CHK002 — Foco em valor e necessidade de negócio (coerência documental, risco jurídico e soberania do jogador) — rastreia: User Story 1, User Story 2, SC-005, SC-006.
- [x] CHK003 — Escrito para stakeholders não técnicos — rastreia: User Scenarios, Assumptions, Clarifications.
- [x] CHK004 — Todas as seções obrigatórias preenchidas, nenhuma vazia — rastreia: User Scenarios, Requirements, Key Entities, Success Criteria, Assumptions, Clarifications.

## Requirement Completeness

- [x] CHK005 — Nenhum marcador `[NEEDS CLARIFICATION]` nem `TBD` restante (0 marcadores; limite ≤ 3) — rastreia: spec.md integral; Clarifications.
- [x] CHK006 — Exatamente 7 requisitos funcionais numerados, um por passagem do GDD, testáveis por leitura — rastreia: FR-001–FR-007; Key Entities (Mapa de 7 passagens).
- [x] CHK007 — Requisitos inequívocos, com verbo normativo "DEVE"/"NÃO DEVE" — rastreia: FR-001–FR-007; Edge Cases.
- [x] CHK008 — Critérios de sucesso mensuráveis, com baseline e meta — rastreia: SC-001–SC-007.
- [x] CHK009 — Critérios de sucesso tecnologicamente agnósticos — rastreia: SC-001–SC-007.
- [x] CHK010 — Critérios cobrem explicitamente 7/7 emendas, 0 âncoras alteradas e verificações documentais executadas neste ciclo com resultado observável; os gates determinísticos locais pertencem à portaria R4 (validate), não a tarefas desta feature — rastreia: SC-001, SC-002, SC-003.
- [x] CHK011 — Todos os cenários de aceitação definidos em formato Given/When/Then — rastreia: User Story 1 (3), User Story 2 (2), User Story 3 (3).
- [x] CHK012 — Casos-limite identificados — rastreia: Edge Cases (6 casos) e FR-001, FR-004, FR-006, FR-007.
- [x] CHK013 — Escopo claramente limitado (sem código, gate de build, CI remota, reescrita ampla, efeito externo) — rastreia: Assumptions, Edge Cases, SC-007; decisão R1 (opção A).
- [x] CHK014 — Dependências e premissas identificadas — rastreia: Assumptions (9 premissas) e Clarifications (3 respostas por evidência local).

## Feature Readiness

- [x] CHK015 — Cada requisito funcional possui aceitação verificável por leitura — rastreia: FR-001–FR-007 ↔ User Story 1/2/3 e SC-001–SC-007.
- [x] CHK016 — Cenários cobrem os fluxos primários — rastreia: US1 (licenciamento e N3), US2 (CI e DEFER), US3 (auditoria, âncoras e rastreabilidade).
- [x] CHK017 — R3 respeitado: ≤3 esclarecimentos, um por vez, com 0 perguntas ao usuário registradas — rastreia: Clarifications / Session 2026-11-19 (3 Q/A e nota final).
- [x] CHK018 — Nenhum detalhe de implementação vaza na spec — rastreia: Requirements, Success Criteria, Key Entities e Contexto e Orçamento.

## Notes

- 18 itens rastreáveis de 18 (100% ≥ 80% exigido pelo contrato).
- Revalidação pós-clarificação: 18/18 → 18/18 (sem regressões; nenhum item mudou de estado).
- 0 perguntas ao usuário; ambiguidades resolvidas por evidência local (decisão R1, ADR-001, ADR-002, avaliação de stack e estado do repositório), conforme `## Clarifications` / `### Session 2026-11-19`.
- Itens que venham a ficar incompletos exigem atualização da spec antes da fase de planejamento.
