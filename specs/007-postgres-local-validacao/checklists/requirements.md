# Specification Quality Checklist: Validação local de Postgres MDMV

**Purpose**: Validar a completude e a qualidade da especificação antes da fase de planejamento.
**Created**: 2026-11-20
**Feature**: [spec.md](../spec.md) — `007-postgres-local-validacao`

> Formato do contrato: itens `- [ ] CHK### …`, rastreáveis para seções/IDs da spec.
> Os marcadores permanecem `[ ]` conforme a convenção reviewer-owned adotada no hub; a avaliação do autor não substitui a aprovação do revisor de requisitos.

## Content Quality

- [ ] CHK001 — A spec descreve versão observada, migrações, smoke, privilégio mínimo e teardown sem fixar sintaxe de implementação de adaptador, código de produto ou configuração de deployment — rastreia: spec.md integral; §Contexto e Orçamento.
- [ ] CHK002 — O foco é valor de auditoria: prova local auditável do padrão Postgres MDMV, com isolamento de remoto e de `pgmq` — rastreia: US1–US4, SC-001–SC-010, FR-010–FR-012.
- [ ] CHK003 — Escrita para stakeholders não técnicos: mantenedor, implementadores do adaptador de persistência e auditores — rastreia: User Scenarios, Assumptions, Clarifications.
- [ ] CHK004 — Todas as seções obrigatórias estão preenchidas e nenhuma está vazia — rastreia: User Scenarios, Edge Cases, Requirements, Key Entities, Success Criteria, Assumptions, Clarifications, Evidência bruta, Contexto e Orçamento.

## Requirement Completeness

- [ ] CHK005 — Nenhum marcador de pendência permanece; as três ambiguidades estão respondidas e rastreáveis — rastreia: §Clarifications / Session 2026-11-20; SC-010.
- [ ] CHK006 — Quinze requisitos funcionais numerados e testáveis cobrem versão, migrações, schema, inserção, JSONB, grants, teste negativo, teardown, isolamento e escopo — rastreia: FR-001–FR-015.
- [ ] CHK007 — Requisitos inequívocos, com DEVE/NÃO DEVE e exceções nomeadas — rastreia: FR-001–FR-015; Edge Cases.
- [ ] CHK008 — Critérios de sucesso mensuráveis, com baseline e meta, e estado `[NV]` quando dependem de execução ainda não realizada — rastreia: SC-001–SC-010.
- [ ] CHK009 — Critérios tecnologicamente agnósticos: contagens, valores observados e rótulos, sem exigir ferramenta específica para verificar o requisito escrito — rastreia: SC-001–SC-010.
- [ ] CHK010 — Critérios cobrem explicitamente versão, migrações, JSONB `@>`, 4 grants, teste negativo e teardown — rastreia: SC-001, SC-002, SC-005, SC-006, SC-007, SC-008.
- [ ] CHK011 — Todos os cenários de aceitação estão em formato Given/When/Then — rastreia: US1 (4), US2 (4), US3 (4), US4 (4).
- [ ] CHK012 — Casos-limite identificados, inclusive versão divergente, erro de migração, `@>` falso, grants inesperados, UPDATE permitido, remanescente após teardown e menção indevida de remoto/`pgmq` — rastreia: Edge Cases (10 casos); FR-001, FR-002, FR-006, FR-008, FR-009, FR-011, FR-012.
- [ ] CHK013 — Escopo claramente limitado: sem Postgres remoto, Coolify, Zitadel, segredo real, `pgmq` testado, gates, stage/commit e portarias posteriores a tasks — rastreia: Assumptions; FR-010, FR-012, FR-015; decisão R1.
- [ ] CHK014 — Dependências e premissas identificadas — rastreia: Assumptions (7 premissas); Clarifications (3 respostas por pedido autorizado, evidência local e política de rotulagem).

## Feature Readiness

- [ ] CHK015 — Cada requisito funcional possui aceitação verificável por contagem, presença de registro ou leitura de valores — rastreia: FR-001–FR-015 ↔ US1/US2/US3/US4 e SC-001–SC-010.
- [ ] CHK016 — Cenários cobrem os fluxos primários: migrações aplicadas, smoke autenticado, privilégio mínimo e teardown com auditoria — rastreia: US1, US2, US3, US4.
- [ ] CHK017 — R3 respeitado: no máximo três esclarecimentos, um por vez, resolvidos antes do planejamento — rastreia: §Clarifications / Session 2026-11-20 (3 Q/A e nota final).
- [ ] CHK018 — Nenhuma seção de planejamento antecipada na spec, além da seção documental de contexto e da seção documental de evidência — rastreia: Requirements, Success Criteria, Key Entities, §Evidência bruta e §Contexto e Orçamento.
- [ ] CHK019 — R1 registrado com `go` legítimo: validade `strong`, evidência `strong` e Opção A nomeada — rastreia: decision.md §Scorecard e §Verdict & Rationale.
- [ ] CHK020 — A natureza local da prova, o não teste de `pgmq` e a não reexecução nesta etapa são tratados como requisitos de rotulagem e citação, não como afirmação de validação remota — rastreia: FR-010, FR-011, FR-012, FR-013; SC-009, SC-010; Clarifications Q1, Q2, Q3.

## Notes

- Vinte itens rastreáveis de vinte (100%, acima do mínimo de 80% do contrato).
- Nenhum item promovido a `[x]`: os marcadores são de propriedade do revisor de requisitos.
- Nenhuma pergunta foi feita ao usuário; as três ambiguidades foram resolvidas por pedido autorizado, evidência local e política de rotulagem.
- Itens que venham a ficar incompletos exigem atualização da spec antes da fase de planejamento.
