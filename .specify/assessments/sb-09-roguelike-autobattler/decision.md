# Decision — Roguelike auto-battler

- **Slug**: sb-09-roguelike-autobattler
- **Decided**: 2026-11-21
- **Veredito**: DEFER
- **Artifacts reviewed**: intake.md; research.md; problem.md; concept.md; `docs/research/slotbound-2026-11-21-game-design.md`; `docs/research/slotbound-2026-11-21-fit-local.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | Gênero real e confirmado na Steam, mas a demanda não é da SE — é uma descrição do jogo alheio. |
| Força da evidência | adequate | Steam/Riot primários (modo solo; TFT não recompensa coordenação); "100K/3 dias" é autorrelato `[NV]`. |
| Valor vs. inação | weak | Inação custa pouco: nada é implementado e a coerência com IV/V é preservada; valor futuro depende de pré-requisito. |
| Viabilidade/Appetite | weak | Sem servidor determinístico/5v5 no alvo; cliente ainda não chama `avancar_tick`; reescrita da condição de vitória. |
| Alinhamento estratégico | weak | Modo solo conflita com IV (ação coletiva) e V (sem campeão solitário / soma de desempenhos) e GDD §4.4/§5.1. |
| Postura de risco | strong | DEFER é reversível e de custo zero agora; retomar sem pré-requisito exigiria novo assessment. |

## Racional

DEFER: o gênero é comercialmente real (Slotbound: single-player, "summon units… absorbing others", 79%/1.598 reviews na Steam), e a auto-resolução determinística poderia ser útil como replay auditável. Mas a Riot define que TFT **não** recompensa coordenação, e o GDD §4.4/§5.1 proíbem "vitória por soma de desempenhos independentes" e "campeão solitário" — exatamente o que o loop solo importa. Pior: o repo não tem o pré-requisito que tornaria o modo compatível (servidor determinístico, 5v5; o cliente não chama `avancar_tick`). Implementar agora contrariaria Constituição IV/V e criaria dependência de backend (IV) com base em números outliers/autorrelato `[NV]`. Registra-se, portanto, a possibilidade futura de **auto-resolução de ações coletivas** (nunca modo solo), condicionada a servidor/5v5. Nada aqui toca os 5 crates, os 34 testes ou o CI; nenhum build/teste rodado.

## Condição/responsável

- **Responsável**: SE/mantenedor — autoridade sobre a direção de design; reabertura do tema exige novo assessment, não decisão informal.
- **Condição para retomar**: (1) servidor determinístico/headless no alvo; (2) sessão 5v5 real; (3) forma de auto-resolução que **reporte coordenação** (funções/materialização) com semente + `hash_estado()`, nunca "carry" ou corrida de power-level; (4) precedência I→III→II→IV do GDD §4 preservada.
- **Gate**: CI verde autoriza merge, nunca auto-merge cego; nenhuma alegação de verde sem comando executado (Constituição VI). Nenhum build/teste/gate rodado nesta etapa.

## Registro para ciclo futuro (sem handoff — DEFER)

Não se aplica (`DEFER` — nenhum handoff de implementação neste ciclo). Registro para ciclo futuro, se as condições acima se materializarem e um novo assessment aprovar:

- **Problema a reconsiderar**: como tornar legível a coordenação em uma simulação de 5v5 sem recair em autoria solitária.
- **Abordagem candidata**: auto-resolução determinística pós-planejamento coletivo, sobre `avancar_tick` + `hash_estado()` do `insurgencia-sim`.
- **In scope condicional**: replay de ações coletivas; **out of scope permanente**: modo solo como núcleo, campeões/carry, RNG opaco, estética de caça-níquel, backend proprietário.
- **Métricas condicionais**: replay reproduzível por semente; nenhuma vitória por desempenho independente; legibilidade validada sob `--minimizado`.
- **Perguntas remanescentes**: `[NV]` quando surgem servidor/5v5? `[NV]` que forma observável de "replay de coordenação" evita power-level? `[NV]` o pré-requisito de `avancar_tick` no cliente é satisfeito em qual ciclo?
