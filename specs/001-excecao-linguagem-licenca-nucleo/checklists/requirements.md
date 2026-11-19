# Specification Quality Checklist: Exceção de linguagem e licença MIT-only do Núcleo

**Purpose**: Validar a completude e a qualidade da especificação antes da fase de planejamento.
**Created**: 2026-11-19
**Feature**: [spec.md](../spec.md) — `001-excecao-linguagem-licenca-nucleo`

> Formato do contrato: itens `- [x] CHK### …`, rastreáveis para seções/IDs da spec.
> Revalidado em 2026-11-19 após a sessão de clarificação: **16/16 itens aprovados**.

## Content Quality

- [x] CHK001 — Nenhum detalhe de implementação (frameworks, comandos, caminhos de código) — rastreia: Requirements (FR-001–FR-013), Success Criteria (SC-001–SC-008).
- [x] CHK002 — Foco em valor e necessidade de negócio (coerência documental e soberania do jogador) — rastreia: User Story 1, User Story 2, SC-003, SC-005.
- [x] CHK003 — Escrito para stakeholders não técnicos — rastreia: User Scenarios, Assumptions, Clarifications.
- [x] CHK004 — Todas as seções obrigatórias preenchidas, nenhuma vazia — rastreia: User Scenarios, Requirements, Key Entities, Success Criteria, Assumptions.

## Requirement Completeness

- [x] CHK005 — Nenhum marcador `[NEEDS CLARIFICATION]` nem `TBD` restante (0 marcadores; limite ≤ 3) — rastreia: spec.md integral, Clarifications.
- [x] CHK006 — Requisitos testáveis e inequívocos — rastreia: FR-001–FR-013 com verbos "DEVE"/"NÃO DEVE".
- [x] CHK007 — Critérios de sucesso mensuráveis (contagens, baseline e meta) — rastreia: SC-001–SC-008.
- [x] CHK008 — Critérios de sucesso tecnologicamente agnósticos — rastreia: SC-001–SC-008.
- [x] CHK009 — Todos os cenários de aceitação definidos em formato Given/When/Then — rastreia: User Story 1 (3), User Story 2 (3), User Story 3 (3).
- [x] CHK010 — Casos-limite identificados — rastreia: Edge Cases (6 casos) e seus FRs de origem (FR-004, FR-006, FR-007, FR-008, FR-011, FR-013).
- [x] CHK011 — Escopo claramente limitado — rastreia: FR-010, FR-013, Assumptions, Clarifications.
- [x] CHK012 — Dependências e premissas identificadas — rastreia: Assumptions (8 premissas) e Clarifications (4 respostas locais).

## Feature Readiness

- [x] CHK013 — Cada requisito funcional possui aceitação verificável — rastreia: FR-001–FR-013 ↔ User Story 1/2/3 e SC-001–SC-008.
- [x] CHK014 — Cenários cobrem os fluxos primários — rastreia: US1 (ADR-001), US2 (ADR-002), US3 (verificação e rastreabilidade).
- [x] CHK015 — Feature atende aos resultados mensuráveis definidos — rastreia: SC-001 (2 ADRs), SC-002 (3 desvios), SC-008 (4 referências).
- [x] CHK016 — Nenhum detalhe de implementação vaza na spec — rastreia: seções Requirements, Success Criteria e Contexto e Orçamento.

## Notes

- 16 itens rastreáveis de 16 (100% ≥ 80% exigido pelo contrato).
- Revalidação pós-clarificação: 16/16 → 16/16 (sem regressões; nenhum item mudou de estado).
- 0 perguntas ao usuário; ambiguidades resolvidas por evidência local, conforme `## Clarifications` / `### Session 2026-11-19`.
- Itens que venham a ficar incompletos exigem atualização da spec antes da fase de planejamento.
