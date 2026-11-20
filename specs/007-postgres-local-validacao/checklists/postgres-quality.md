# Requirements Quality Checklist: Validação local de Postgres MDMV

**Purpose**: Validar a qualidade, clareza, completude e mensurabilidade dos requisitos da validação local de Postgres — não a execução dos comandos, a instalação do Docker nem o comportamento do banco em produção.
**Created**: 2026-11-20
**Feature**: [spec.md](../spec.md) — `007-postgres-local-validacao`

> Propriedade: artefato de revisão de requisitos. `[x]` significa que o revisor considerou o critério de qualidade atendido; não significa que a implementação foi concluída. Nenhum item é marcado `[x]` por esta geração.

## Requirement Completeness

- [ ] CHK001 — A versão observada do servidor está escrita como requisito, com origem rastreável e proibição de inferência pela tag? [Spec §FR-001, §SC-001; Completeness]
- [ ] CHK002 — A aplicação das duas migrações e o comportamento diante de erro estão especificados, incluindo o uso de `ON_ERROR_STOP` como condição de falha explícita? [Spec §FR-002, §SC-002; Completeness]
- [ ] CHK003 — Os objetos de schema esperados (2 tabelas, `payload jsonb`, índice GIN) estão enumerados como requisito, sem afirmar desempenho de índice? [Spec §FR-003, §SC-003; Completeness]
- [ ] CHK004 — O papel usado no smoke e a saída `INSERT 0 1` estão definidos como requisito, com autenticação como `insurgencia_app`? [Spec §FR-004, §SC-004; Completeness]
- [ ] CHK005 — O predicado JSONB `@>` verdadeiro está especificado com o dado exato e a proibição de ajustar o dado para forçar o resultado? [Spec §FR-005, §SC-005, §Edge Cases; Completeness]
- [ ] CHK006 — A quantidade de grants (4), as tabelas e a ausência de UPDATE/DELETE/TRUNCATE estão escritas como requisitos verificáveis por contagem? [Spec §FR-006, §FR-007, §SC-006; Completeness]
- [ ] CHK007 — O teste negativo está definido como requisito, com comando, mensagem `permission denied`, `exit 1` e tratamento de UPDATE permitido como falha? [Spec §FR-008, §SC-007; Completeness]
- [ ] CHK008 — O teardown e a contagem zero de remanescentes estão especificados como requisito, com data/hora e tratamento de remanescente como incompleto? [Spec §FR-009, §SC-008; Completeness]
- [ ] CHK009 — As regras de credencial descartável, `pgmq` `[NV]`, remoto `[NV]`, evidência com origem/carimbo e término em tasks estão especificadas? [Spec §FR-010–FR-015, §SC-009, §SC-010; Completeness]

## Requirement Clarity

- [ ] CHK010 — "Versão observada" está definida como saída de `SHOW server_version;`, sem deixar margem para a tag da imagem ser tomada como versão? [Spec §FR-001, §D1; Clarity]
- [ ] CHK011 — "Privilégio mínimo" está definido como contagem exata de grants e negação de UPDATE, e não como adjetivo vago? [Spec §FR-006–FR-008, §SC-006; Clarity]
- [ ] CHK012 — A janela 01:02:36Z–01:03Z está definida como derivação dos carimbos do pedido, sem inventar segundo para smoke ou teste negativo? [Spec §Assumptions; Plan §D10; Clarity]
- [ ] CHK013 — Os verbos normativos DEVE/NÃO DEVE são inequívocos em FR-001–FR-015, inclusive as proibições de segredo real e de afirmação de remoto? [Spec §Requirements; Clarity]

## Requirement Consistency

- [ ] CHK014 — Os valores de versão, migrações, grants, erro de permissão e teardown são idênticos entre research, spec, decision e plan, sem divergência de carimbo? [Spec §FR-001–FR-009; Plan §Inventário de evidência; Decision §Scorecard; Consistency]
- [ ] CHK015 — O tratamento de `pgmq`, Postgres remoto, Coolify e Zitadel como `[NV]` é consistente entre non-goals, edge cases, requisitos, clarificações e decisão R1? [Spec §Non-Goals, §Edge Cases, §FR-010–FR-012, §Clarifications Q1/Q2; problem.md §Non-Goals; Decision §Verdict; Consistency]
- [ ] CHK016 — O término em tasks é consistente entre spec, plan e decisão, sem seção de implementação, converge, validação ou release? [Spec §FR-015, §SC-010; Plan §D11; Decision §Handoff; Consistency]

## Acceptance Criteria Quality

- [ ] CHK017 — Cada SC possui baseline, meta e valores observáveis, sem depender de impressão ou interpretação subjetiva? [Spec §SC-001–SC-010; Measurability]
- [ ] CHK018 — Os critérios de versão, migrações, contagem de grants, `INSERT 0 1`, `@>` e remanescentes são mensuráveis por leitura de saída e contagem? [Spec §SC-001–SC-005, §SC-006, §SC-008; Measurability]
- [ ] CHK019 — O critério de isolamento é verificável por contagem de afirmações com origem/carimbo e por ausência de segredo e de afirmações de remoto/`pgmq`? [Spec §SC-009; Measurability]

## Scenario Coverage

- [ ] CHK020 — Os cenários cobrem migrações aplicadas, smoke autenticado, privilégio mínimo e teardown como fluxos primários independentes? [Spec §US1/AC1–AC4, §US2/AC1–AC4, §US3/AC1–AC4, §US4/AC1–AC4; Coverage]
- [ ] CHK021 — Os cenários cobrem versão divergente, erro de migração, `@>` falso, grants inesperados, UPDATE permitido e remanescente após remoção? [Spec §US1/AC4, §US2/AC4, §US3/AC4, §US4/AC1, §Edge Cases; Coverage]

## Edge Case Coverage

- [ ] CHK022 — Estão especificados os casos de migração com erro, UPDATE permitido, teardown incompleto, menção indevida de `pgmq`/remoto, não reexecução nesta etapa e credencial real indevida? [Spec §Edge Cases; FR-002, FR-008, FR-009, FR-010, FR-011, FR-012, FR-013; Edge Case]

## Dependencies & Assumptions

- [ ] CHK023 — Premissas de que a evidência do orquestrador é citada e não reexecutada, de que a senha é fixture descartável, de que a janela é derivada e de que `pgcrypto` não foi testado funcionalmente estão explícitas? [Spec §Assumptions; Plan §D7, §D8, §D10; Assumption]

## Notes

- Vinte e três itens, todos com rastreabilidade para seções ou IDs da spec/plano/decisão (100%, acima do mínimo de 80%).
- Nenhum item verifica comportamento de código, executa comando ou mede resultado de implementação: o objeto é a qualidade dos requisitos escritos.
- A distinção entre evidência citada e verificação ao vivo é tratada como requisito de rotulagem (Clarificação Q3), não como item de implementação deste checklist.
- Itens que ficarem incompletos em revisão humana exigem atualização da spec antes da implementação.
