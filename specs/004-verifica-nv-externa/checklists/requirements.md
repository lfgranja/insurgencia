# Specification Quality Checklist: Verifica NV externa

**Purpose**: Validar a completude e a qualidade da especificação antes da fase de planejamento.
**Created**: 2026-11-19
**Feature**: [spec.md](../spec.md) — `004-verifica-nv-externa`

> Formato do contrato: itens `- [ ] CHK### …`, rastreáveis para seções/IDs da spec.
> Revalidação pós-clarificação em 2026-11-19: 20/20 itens avaliados como atendidos, 0 lacunas, 0 regressões. Os marcadores permanecem `[ ]` conforme a convenção reviewer-owned adotada nesta feature; a avaliação do autor não substitui a aprovação do revisor.

## Content Quality

- [ ] CHK001 — Nenhum detalhe de stack, linguagem, engine, API ou banco vaza na spec; o objeto é citação documental — rastreia: spec.md integral; §Contexto e Orçamento.
- [ ] CHK002 — Foco em valor e necessidade de negócio: auditabilidade, ressalvas e preservação de salvaguardas — rastreia: US1–US4, SC-001–SC-010.
- [ ] CHK003 — Escrito para stakeholders não técnicos: mantenedor, auditor e leitores das avaliações — rastreia: User Scenarios, Assumptions, Clarifications.
- [ ] CHK004 — Todas as seções obrigatórias preenchidas, nenhuma vazia — rastreia: User Scenarios, Edge Cases, Requirements, Key Entities, Success Criteria, Assumptions, Clarifications.

## Requirement Completeness

- [ ] CHK005 — Nenhum marcador `[NEEDS CLARIFICATION]` ou `TBD` pendente (0 marcadores; as três ambiguidades respondidas e os 11 marcadores dos artefatos de assess convertidos em "Resolvido:" em `intake.md`, `problem.md` e `research.md`; SC-010 continua exigindo 0 pendente) — rastreia: §Clarifications / Session 2026-11-19; SC-010.
- [ ] CHK006 — Quatorze requisitos funcionais numerados e testáveis, cobrindo substituição, citação, ressalvas, proibição de benchmark, classes B/C, snapshots, imutabilidade de scores e governança — rastreia: FR-001–FR-014.
- [ ] CHK007 — Requisitos inequívocos, com verbo normativo DEVE/NÃO DEVE e exceções nomeadas — rastreia: FR-001–FR-014; Edge Cases.
- [ ] CHK008 — Critérios de sucesso mensuráveis, com baseline e meta — rastreia: SC-001–SC-010.
- [ ] CHK009 — Critérios tecnologicamente agnósticos: contagens de fatos, fontes, marcadores preservados, linhas reescritas e artefatos — rastreia: SC-001–SC-010.
- [ ] CHK010 — Critérios cobrem explicitamente os 12 fatos, 0 fatos sem 2 fontes, 0 benchmarks e 100% de preservação B/C, inclusive a ausência de fundação neutra em LÖVE/Haxe/OpenFL em `docs/avaliacao-engines/pesquisa/monogame_love_haxe.md:23` — rastreia: SC-001, SC-002, SC-003, SC-004, SC-005; US4/AC1; FR-005.
- [ ] CHK011 — Todos os cenários de aceitação em formato Given/When/Then — rastreia: US1 (3), US2 (4), US3 (4), US4 (4).
- [ ] CHK012 — Casos-limite identificados, inclusive fato já citado, EULA como afirmação negativa, `license: null`, cadência 0.19, classes B/C e divergência de contagem — rastreia: Edge Cases (11 casos); FR-003, FR-005, FR-006, FR-011.
- [ ] CHK013 — Escopo claramente limitado (sem benchmarks, sem reescrita de histórico/governança, sem recálculo de scores, sem novas URLs, sem gates/stage/commit) — rastreia: Assumptions; FR-004, FR-009, FR-010, FR-014; decisão R1.
- [ ] CHK014 — Dependências e premissas identificadas — rastreia: Assumptions (7 premissas); Clarifications (3 respostas por evidência local).

## Feature Readiness

- [ ] CHK015 — Cada requisito funcional possui aceitação verificável por contagem ou presença de citação — rastreia: FR-001–FR-014 ↔ US1/US2/US3/US4 e SC-001–SC-010.
- [ ] CHK016 — Cenários cobrem os fluxos primários: Godot, Stride/Fyrox/Dviglo/Defold, Lightyear/Bevy e salvaguardas — rastreia: US1, US2, US3, US4.
- [ ] CHK017 — R3 respeitado: ≤3 esclarecimentos, um por vez, com 0 perguntas ao usuário — rastreia: §Clarifications / Session 2026-11-19 (3 Q/A e nota final).
- [ ] CHK018 — Nenhuma seção de planejamento antecipada na spec (arquitetura, tarefas, checklists embutidas) — rastreia: Requirements, Success Criteria, Key Entities e Contexto e Orçamento.
- [ ] CHK019 — R1 registrado com `go` legítimo: validity/evidence ≥adequate e opção A nomeada — rastreia: decision.md §Scorecard e §Verdict.
- [ ] CHK020 — Cada uma das 12 ressalvas de FR-003 é mensurável por SC-008 e por tarefa dedicada — rastreia: FR-003, SC-008, tasks.md T007–T022.

## Notes

- 20 itens rastreáveis de 20 (100% ≥ 80% exigido pelo contrato).
- Revalidação pós-clarificação: 20/20 avaliados como atendidos; nenhuma regressão; nenhum item promovido a `[x]` porque os marcadores são reviewer-owned nesta feature.
- 0 perguntas ao usuário; ambiguidades resolvidas por evidência local (três arquivos de evidência e `docs/governance/nv-resolution-2026-11-19.md`), conforme §Clarifications.
- Ajuste de 2026-11-19: 11 marcadores dos artefatos de assess convertidos em "Resolvido:" (0 pendente; SC-010 inalterado) e `docs/avaliacao-engines/pesquisa/monogame_love_haxe.md:23` incluído no escopo de preservação de US4/AC1/SC-004/FR-005, com a ressalva de ausência de fundação neutra em LÖVE/Haxe/OpenFL.
- Itens que venham a ficar incompletos exigem atualização da spec antes da fase de planejamento.
