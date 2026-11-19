# Specification Quality Checklist: Baselines locais de governança

**Purpose**: Validar a completude e a qualidade da especificação antes da fase de planejamento.
**Created**: 2026-11-19
**Feature**: [spec.md](../spec.md) — `003-baseline-governanca`

> Formato do contrato: itens `- [ ]`/`- [x] CHK### …`, rastreáveis para seções/IDs da spec.
> Revalidado em 2026-11-19 após a sessão de clarificação: **18/18 itens aprovados** (17/18 → 18/18; nenhuma regressão).

## Content Quality

- [x] CHK001 — Nenhum detalhe de implementação de stack, linguagem, engine, API ou banco vaza na spec; os comandos citados são o mecanismo de bootstrap do próprio objeto da feature — rastreia: spec.md integral; §Contexto e Orçamento.
- [x] CHK002 — Foco em valor e necessidade de negócio (auditabilidade, detecção de regressão, dívida 1) — rastreia: US1, US2, US3, SC-001, SC-003, SC-008.
- [x] CHK003 — Escrito para stakeholders não técnicos: mantenedores e auditores, sem jargão de implementação — rastreia: User Scenarios, Assumptions, Clarifications.
- [x] CHK004 — Todas as seções obrigatórias preenchidas, nenhuma vazia — rastreia: User Scenarios, Edge Cases, Requirements, Key Entities, Success Criteria, Assumptions, Clarifications.

## Requirement Completeness

- [x] CHK005 — Nenhum marcador `[NEEDS CLARIFICATION]` nem `TBD` restante (0 marcadores; limite ≤ 3) — rastreia: spec.md integral; `## Clarifications` / `### Session 2026-11-19`.
- [x] CHK006 — Doze requisitos funcionais numerados, testáveis: três bootstraps, proveniência, anti-factory, três gates, honestidade de mapa vazio, critério de aceite, imutabilidade dos scripts e evidência — rastreia: FR-001–FR-012.
- [x] CHK007 — Requisitos inequívocos, com verbo normativo "DEVE"/"NÃO DEVE" e saídas exit 0/1 nomeadas — rastreia: FR-001–FR-012; Edge Cases.
- [x] CHK008 — Critérios de sucesso mensuráveis, com baseline e meta — rastreia: SC-001–SC-008.
- [x] CHK009 — Critérios de sucesso tecnologicamente agnósticos: contagens, exit code, zero `stale`, zero chaves órfãs — rastreia: SC-001–SC-008.
- [x] CHK010 — Critérios cobrem explicitamente os três baselines, `baselined>0` onde aplicável e zero `stale`, além da recusa de regressão controlada — rastreia: SC-001, SC-003, SC-004, SC-005, SC-008.
- [x] CHK011 — Todos os cenários de aceitação definidos em formato Given/When/Then — rastreia: US1 (3), US2 (3), US3 (3).
- [x] CHK012 — Casos-limite identificados, incluindo estado limpo, factory, contagens divergentes, apêndice legítimo e `stale` — rastreia: Edge Cases (9 casos); FR-005, FR-009, FR-010.
- [x] CHK013 — Escopo claramente limitado (sem alteração de scripts, sem importação da factory, sem código/engine/CI/remoto, sem reclassificar dívidas 2–6) — rastreia: Assumptions; FR-011; decisão R1 (opção A).
- [x] CHK014 — Dependências e premissas identificadas — rastreia: Assumptions (9 premissas); Clarifications (3 respostas por evidência local).

## Feature Readiness

- [x] CHK015 — Cada requisito funcional possui aceitação verificável por contagem, schema ou exit code — rastreia: FR-001–FR-012 ↔ US1/US2/US3 e SC-001–SC-008.
- [x] CHK016 — Cenários cobrem os fluxos primários: bootstrap de converge, bootstrap de completude/unicidade e evidência de origem local — rastreia: US1, US2, US3.
- [x] CHK017 — R3 respeitado: ≤3 esclarecimentos, um por vez, com 0 perguntas ao usuário registradas — rastreia: Clarifications / Session 2026-11-19 (3 Q/A e nota final).
- [x] CHK018 — Nenhuma seção de planejamento (arquitetura, tarefas, checklists embutidas) antecipada na spec — rastreia: Requirements, Success Criteria, Key Entities e Contexto e Orçamento.

## Notes

- 18 itens rastreáveis de 18 (100% ≥ 80% exigido pelo contrato).
- Revalidação pós-clarificação: 17/18 → 18/18; único item promovido: CHK005 (três marcadores resolvidos por evidência local, um por vez).
- 0 perguntas ao usuário; ambiguidades resolvidas por evidência local (decisão R1, scripts de gate, STATE, PROGRESSO e artefatos da feature 003), conforme `## Clarifications` / `### Session 2026-11-19`.
- Itens que venham a ficar incompletos exigem atualização da spec antes da fase de planejamento.
