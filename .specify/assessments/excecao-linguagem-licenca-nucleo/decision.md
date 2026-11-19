# Decision: Exceção de linguagem e licença do Núcleo

- **Slug**: excecao-linguagem-licenca-nucleo
- **Decided**: 2026-11-19 — 21:20 UTC
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Problem validity | strong | Sete desvios de intenção registrados em `docs/AVALIACAO-STACK-MDMV.md`, dois deles bloqueantes (Godot/GDScript sem ADR; N3 MIT/AGPL aberta apesar de `LICENSE` MIT). |
| Evidence strength | strong | Documentação local verificada e triangulada: `GDD:1265,1678,1451,1659,1668`, `LICENSE:1`, `AVALIACAO-STACK-MDMV.md` desvios #1/#3/#10, spec/plano do ciclo 80/20. `[NV]` apenas para remoto. |
| Value vs. inaction | strong | Sem a decisão, qualquer implementação futura herda ambiguidade de licença e conflito normativo; custo de inação = rework e risco jurídico. |
| Feasibility / appetite | strong | Opção A é `small`: documentos ADR, sem engine, sem serviço externo, sem autorização remota. |
| Strategic fit | strong | Preserva o Pilar I (soberania) e o Godot-first vinculante; não altera mecânicas, pilares ou MVP. |
| Risk posture | strong | Risco principal é interpretar a ADR como reabertura do GDD; mitigado com texto explícito de não-reabertura e marco de reavaliação. |

## Verdict & Rationale

**go (R1 atendido).** A opção A — ADR local de exceção de linguagem para GDScript e resolução da
pergunta N3 em favor de MIT-only no Núcleo — tem validity e evidence `strong`, opção recomendada em
`concept.md`, e não depende de nada que esteja `[NV]`. A alternativa B (emenda da norma) exige
autorização externa e fica DEFER; a C (servidor Rust) é o Plano B já DEFER em
`docs/AVALIACAO-COMPARATIVA-ENGINES.md:538`. Nenhuma das três reverte `GDD:1265,1678`.

## Se adotado

Dois ADRs locais no ciclo 001: ADR-001 (exceção de linguagem) e ADR-002 (licença MIT-only do
Núcleo, com `COPYING`替代 tratado como fora de escopo do ciclo). Nenhum código, engine, CI ou
serviço externo entra neste ciclo.

## Handoff para specify

Dois ADRs locais no ciclo 001: ADR-001 (exceção de linguagem) e ADR-002 (licença MIT-only do
Núcleo, com a questão `COPYING` tratada como fora de escopo do ciclo). Nenhum código, engine, CI ou
serviço externo entra neste ciclo.
- **Chosen approach**: opção A (ADRs locais, sem reabrir o GDD).
- **In scope**: dois ADRs versionados, rastreabilidade para os desvios #1/#3/#10 e para R1.
- **Out of scope**: emenda da norma MDMV, código Godot, servidor Rust, CI remota, Postgres, telemetria.
- **Success metrics**: ADRs aprovados no ciclo; 0 desvios de intenção sem decisão local; `check_sdd_completeness PASS`.
- **Open questions carried forward**: autoria/histórico da ideia `[NEEDS CLARIFICATION]`; status remoto/CI `[NV]`.
