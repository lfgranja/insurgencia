# Requirements Quality Checklist: CI de gates determinísticos e proteção de dev

**Purpose**: Validar a qualidade, clareza, completude e mensurabilidade dos requisitos de CI e proteção de branch — não a implementação do workflow.
**Created**: 2026-11-20
**Feature**: [spec.md](../spec.md) — `005-ci-protecao-dev`

> Propriedade: artefato de revisão de requisitos. `[x]` significa que o revisor considerou o critério de qualidade atendido; não significa que a implementação foi concluída. Nenhum item é marcado `[x]` por esta geração.

## Requirement Completeness

- [ ] CHK001 — Os quatro eventos que disparam a verificação estão explicitamente enumerados como requisito, e não deixados para inferência? [Spec §FR-001, §SC-001; Completeness]
- [ ] CHK002 — Os cinco gates do alvo estão identificados como conjunto fechado, com critério de exclusão para gates não materializados? [Spec §FR-002, §FR-009; Completeness]
- [ ] CHK003 — O comportamento diante de falha de gate está especificado, incluindo a proibição de resultado verde artificial? [Spec §FR-003, §SC-007; Completeness]
- [ ] CHK004 — A origem do nome do status check, o ato de proteger e a prova por API estão especificados como três requisitos distintos? [Spec §FR-004–FR-006; Completeness]
- [ ] CHK005 — O requisito de rotulagem `[NV]` cobre explicitamente CI, check, merge e proteção? [Spec §FR-007, §SC-008; Completeness]
- [ ] CHK006 — As exclusões de publicação/deploy, `.forgejo/**`, mudança de gates/baselines/governança e segredos estão escritas como requisitos testáveis? [Spec §FR-008–FR-011; Completeness]

## Requirement Clarity

- [ ] CHK007 — "Nome real do check" está definido como valor lido de execução ao vivo, sem deixar margem para nome presumido? [Spec §FR-004, §Clarifications Q1; Clarity]
- [ ] CHK008 — "Proteção aplicada" está definida como configuração exigindo o nome observado, e não como mera intenção? [Spec §FR-005, §SC-004; Clarity]
- [ ] CHK009 — "Verificado via API" está definido com saída colada e data/hora, incluindo o caso de recusa da API? [Spec §FR-006, §SC-005; Clarity]
- [ ] CHK010 — Os verbos normativos DEVE/NÃO DEVE são inequívocos em FR-001–FR-014? [Spec §Requirements; Clarity]

## Requirement Consistency

- [ ] CHK011 — A ordem observar → proteger → verificar é consistente entre cenários, requisitos, clarificações e decisão R1? [Spec §US2, §US3, §FR-004–FR-006, §Clarifications; Plan §D5–D7; Consistency]
- [ ] CHK012 — A lista de gates em FR-002 corresponde exatamente à lista de `AGENTS.md` e ao inventário do plano, sem gate extra? [Spec §FR-002; Plan §Inventário de gates; Consistency]
- [ ] CHK013 — Os escopos de escrita declarados em spec, plan e decisão não se contradizem? [Spec §FR-012; Plan §D10; Decision §Handoff; Consistency]

## Acceptance Criteria Quality

- [ ] CHK014 — Cada SC possui baseline, meta e, quando remoto, estado `[NV]` explícito até execução? [Spec §SC-001–SC-010; Measurability]
- [ ] CHK015 — Os critérios de gates PASS, nome observado e proteção verificada são mensuráveis por contagem ou evidência colada, sem depender de impressão? [Spec §SC-002–SC-005; Measurability]
- [ ] CHK016 — Os critérios de salvaguarda 0 deploy, 0 Forgejo e 0 alteração de gates/baselines são verificáveis por inspeção? [Spec §SC-006, §SC-007; Measurability]

## Scenario Coverage

- [ ] CHK017 — Os cenários cobrem PR para `dev`, PR para `main`, push para `dev` e push para `main` como fluxos primários independentes? [Spec §US1/AC1–AC3; Coverage]
- [ ] CHK018 — Os cenários cobrem reexecução, mudança de nome do check e tentativa de merge com check falho? [Spec §US2/AC3, §US3/AC4, §Edge Cases; Coverage]

## Edge Case Coverage

- [ ] CHK019 — Estão especificados os casos de nome ausente, HTTP 403, repositório privado, lacuna de diretórios derivados e PR de fork? [Spec §Edge Cases; Clarifications Q2; Edge Case]

## Dependencies & Assumptions

- [ ] CHK020 — Premissas de visibilidade pública, autorização separada para proteção e estabilidade do nome estão explícitas, com a ressalva de que falha de gate não é mascarada? [Spec §Assumptions; Plan §D9; Assumption]

## Notes

- Vinte itens, todos com rastreabilidade para seções ou IDs da spec/plano/decisão (100%, acima do mínimo de 80%).
- Nenhum item verifica comportamento de código, executa comando ou mede resultado de implementação: o objeto é a qualidade dos requisitos escritos.
- A lacuna de diretórios derivados é tratada como requisito de honestidade (Clarificação Q2), não como item de implementação deste checklist.
- Itens que ficarem incompletos em revisão humana exigem atualização da spec antes da implementação.
