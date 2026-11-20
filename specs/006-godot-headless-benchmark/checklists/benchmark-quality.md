# Requirements Quality Checklist: Godot headless benchmark

**Purpose**: Validar a qualidade, clareza, completude e mensurabilidade dos requisitos de verificação de release e de benchmark headless — não a instalação da engine nem a execução da carga.
**Created**: 2026-11-20
**Feature**: [spec.md](../spec.md) — `006-godot-headless-benchmark`

> Propriedade: artefato de revisão de requisitos. `[x]` significa que o revisor considerou o critério de qualidade atendido; não significa que a implementação foi concluída. Nenhum item é marcado `[x]` por esta geração.

## Requirement Completeness

- [ ] CHK001 — A identidade da release upstream está escrita como requisito, com origem rastreável, e não deixada para inferência? [Spec §FR-001, §SC-001; Completeness]
- [ ] CHK002 — O par esperado/calculado do checksum e a exigência de igualdade estão especificados, incluindo o comportamento diante de divergência? [Spec §FR-002, §FR-003, §SC-002; Completeness]
- [ ] CHK003 — A identidade do binário está definida como string reportada pelo executável, com proibição de inferência pelo nome do arquivo? [Spec §FR-004, §SC-003; Completeness]
- [ ] CHK004 — A carga do benchmark está delimitada a `tests/benchmark/headless_tick.gd`, com ticks, entidades e exclusões explícitas? [Spec §FR-005, §SC-004; Completeness]
- [ ] CHK005 — Os campos da saída de medição e as estatísticas exigidas estão enumerados como requisito? [Spec §FR-006, §FR-007, §SC-005; Completeness]
- [ ] CHK006 — A referência de comparação e a regra de não alteração do limiar estão especificadas? [Spec §FR-008, §SC-006; Completeness]
- [ ] CHK007 — A vedação à reclassificação como partida 5v5 real está escrita como requisito testável, e não apenas como ressalva informal? [Spec §FR-009, §SC-007, §Clarifications Q1; Completeness]
- [ ] CHK008 — As regras de evidência com origem/data-hora, `[NV]`, segredos e conteúdo não confiável estão especificadas? [Spec §FR-010, §FR-011, §SC-008; Completeness]

## Requirement Clarity

- [ ] CHK009 — "Checksum idêntico" está definido como igualdade exata entre esperado e calculado, com `CHECKSUM_OK` e carimbo? [Spec §FR-002, §FR-003; Clarity]
- [ ] CHK010 — "Linha de base" está definida como registro de comparação, sem deixar margem para interpretação como SLA ou garantia de hardware? [Spec §FR-008, §Assumptions; Clarity]
- [ ] CHK011 — "Simulação headless sintética" está definida de modo a excluir renderização, rede, serviços externos e jogadores? [Spec §FR-005, §FR-009; Clarity]
- [ ] CHK012 — Os verbos normativos DEVE/NÃO DEVE são inequívocos em FR-001–FR-014? [Spec §Requirements; Clarity]

## Requirement Consistency

- [ ] CHK013 — Os valores de release, checksum, binário e medição são idênticos entre research, spec, decision e plan, sem divergência de carimbo? [Spec §FR-001–FR-008; Plan §Inventário de evidência; Decision §Scorecard; Consistency]
- [ ] CHK014 — A classificação da carga é consistente entre non-goals, edge cases, requisitos, clarificações e decisão R1? [Spec §Edge Cases, §FR-009, §Clarifications Q1; problem.md §Non-Goals; Decision §Verdict; Consistency]
- [ ] CHK015 — O término em tasks é consistente entre spec, plan e decisão, sem seção de implementação, converge, validação ou release? [Spec §FR-013, §SC-009; Plan §D10; Decision §Handoff; Consistency]

## Acceptance Criteria Quality

- [ ] CHK016 — Cada SC possui baseline, meta e valores observáveis, sem depender de impressão ou interpretação subjetiva? [Spec §SC-001–SC-010; Measurability]
- [ ] CHK017 — Os critérios de amostras, dispersão e conformidade com a referência são mensuráveis por contagem e comparação numérica? [Spec §SC-005, §SC-006; Measurability]
- [ ] CHK018 — O critério de rotulagem é verificável por contagem de descrições e ausência de afirmações de partida real? [Spec §SC-007; Measurability]

## Scenario Coverage

- [ ] CHK019 — Os cenários cobrem release verificada, binário identificado e medição executada como fluxos primários independentes? [Spec §US1/AC1–AC3, §US2/AC1–AC2, §US3/AC1–AC4; Coverage]
- [ ] CHK020 — Os cenários cobrem divergência de checksum, versão inesperada e tentativa de medir carga fora do escopo? [Spec §US1/AC3, §US3/AC4, §Edge Cases; Coverage]

## Edge Case Coverage

- [ ] CHK021 — Estão especificados os casos de checksum divergente, amostra atípica, mediana acima da referência, não reexecução nesta etapa e instrução externa não confiável? [Spec §Edge Cases; FR-003, FR-008, FR-011, FR-013; Edge Case]

## Dependencies & Assumptions

- [ ] CHK022 — Premissas de que a evidência do orquestrador é citada e não reexecutada, de que 16,7 ms/tick é comparação e de que a série 4.7 é compatível estão explícitas? [Spec §Assumptions; Plan §D6, §D8; Assumption]

## Notes

- Vinte e dois itens, todos com rastreabilidade para seções ou IDs da spec/plano/decisão (100%, acima do mínimo de 80%).
- Nenhum item verifica comportamento de código, executa comando ou mede resultado de implementação: o objeto é a qualidade dos requisitos escritos.
- A natureza sintética da carga é tratada como requisito de rotulagem (Clarificação Q1), não como item de implementação deste checklist.
- Itens que ficarem incompletos em revisão humana exigem atualização da spec antes da implementação.
