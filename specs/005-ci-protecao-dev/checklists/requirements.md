# Specification Quality Checklist: CI de gates determinísticos e proteção de dev

**Purpose**: Validar a completude e a qualidade da especificação antes da fase de planejamento.
**Created**: 2026-11-20
**Feature**: [spec.md](../spec.md) — `005-ci-protecao-dev`

> Formato do contrato: itens `- [ ] CHK### …`, rastreáveis para seções/IDs da spec.
> Os marcadores permanecem `[ ]` conforme a convenção reviewer-owned adotada no hub; a avaliação do autor não substitui a aprovação do revisor de requisitos.

## Content Quality

- [ ] CHK001 — A spec descreve o resultado de verificação sem fixar sintaxe de YAML, sistema operacional do runner ou detalhes de implementação do workflow — rastreia: spec.md integral; §Contexto e Orçamento.
- [ ] CHK002 — O foco é valor e necessidade de governança: merge verificado, nome observado e proteção provada — rastreia: US1–US4, SC-001–SC-010.
- [ ] CHK003 — Escrita para stakeholders não técnicos: mantenedor, contribuidores e auditores de PR — rastreia: User Scenarios, Assumptions, Clarifications.
- [ ] CHK004 — Todas as seções obrigatórias estão preenchidas e nenhuma está vazia — rastreia: User Scenarios, Edge Cases, Requirements, Key Entities, Success Criteria, Assumptions, Clarifications, Contexto e Orçamento.

## Requirement Completeness

- [ ] CHK005 — Nenhum marcador de pendência permanece; as três ambiguidades estão respondidas e rastreáveis — rastreia: §Clarifications / Session 2026-11-20; SC-009.
- [ ] CHK006 — Quatorze requisitos funcionais numerados e testáveis cobrem gatilhos, cinco gates, falha explícita, observação do nome, proteção, verificação por API, rótulo `[NV]`, exclusões e período de planejamento — rastreia: FR-001–FR-014.
- [ ] CHK007 — Requisitos inequívocos, com DEVE/NÃO DEVE e exceções nomeadas — rastreia: FR-001–FR-014; Edge Cases.
- [ ] CHK008 — Critérios de sucesso mensuráveis, com baseline, meta e estado `[NV]` quando dependem de execução — rastreia: SC-001–SC-010.
- [ ] CHK009 — Critérios tecnologicamente agnósticos: contagens de eventos, gates, nomes, regras e evidências, sem exigir ferramenta específica — rastreia: SC-001–SC-010.
- [ ] CHK010 — Critérios cobrem explicitamente os três resultados ao vivo (gates PASS, nome observado, proteção verificada) e as salvaguardas de escopo — rastreia: SC-002, SC-003, SC-004, SC-005, SC-006, SC-007.
- [ ] CHK011 — Todos os cenários de aceitação estão em formato Given/When/Then — rastreia: US1 (4), US2 (3), US3 (4), US4 (4).
- [ ] CHK012 — Casos-limite identificados, inclusive nome ausente, repo privado, HTTP 403, lacuna de diretórios derivados, fork, proteção fora de ordem e divergência local↔CI — rastreia: Edge Cases (9 casos); FR-003, FR-004, FR-010.
- [ ] CHK013 — Escopo claramente limitado: sem publicação/deploy, sem `.forgejo/**`, sem mudança de gates/baselines/governança e sem efeito remoto nesta etapa — rastreia: Assumptions; FR-008, FR-009, FR-010, FR-012; decisão R1.
- [ ] CHK014 — Dependências e premissas identificadas — rastreia: Assumptions (7 premissas); Clarifications (3 respostas por evidência local e autorização).

## Feature Readiness

- [ ] CHK015 — Cada requisito funcional possui aceitação verificável por contagem, presença de evidência ou leitura de resultado — rastreia: FR-001–FR-014 ↔ US1/US2/US3/US4 e SC-001–SC-010.
- [ ] CHK016 — Cenários cobrem os fluxos primários: verificação em PR/push, observação do check, proteção de `dev` e auditoria de salvaguardas — rastreia: US1, US2, US3, US4.
- [ ] CHK017 — R3 respeitado: no máximo três esclarecimentos, um por vez, resolvidos antes do planejamento — rastreia: §Clarifications / Session 2026-11-20 (3 Q/A e nota final).
- [ ] CHK018 — Nenhuma seção de planejamento antecipada na spec, além da seção documental de contexto — rastreia: Requirements, Success Criteria, Key Entities e §Contexto e Orçamento.
- [ ] CHK019 — R1 registrado com `go` legítimo: validade `strong`, evidência `adequate` e Opção A nomeada — rastreia: decision.md §Scorecard e §Verdict & Rationale.
- [ ] CHK020 — Os três resultados dependentes de execução são tratados como requisitos de observação e rotulados `[NV]`, não como fatos verde — rastreia: FR-004, FR-006, FR-007; SC-002–SC-005; SC-008.

## Notes

- Vinte itens rastreáveis de vinte (100%, acima do mínimo de 80% do contrato).
- Nenhum item promovido a `[x]`: os marcadores são de propriedade do revisor de requisitos.
- Nenhuma pergunta foi feita ao usuário; as três ambiguidades foram resolvidas por evidência local, ordem autorizada e política de `[NV]`.
- Itens que venham a ficar incompletos exigem atualização da spec antes da fase de planejamento.
