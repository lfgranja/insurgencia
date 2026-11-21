# Decision — Mecânica de caça-níquel

- **Slug**: sb-10-mecanica-slot
- **Decided**: 2026-11-21
- **Veredito**: ADAPT
- **Artifacts reviewed**: intake.md; research.md; problem.md; concept.md; `docs/research/slotbound-2026-11-21-game-design.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | A slot é real em Slotbound, mas a demanda não é da SE; o valor é variedade por run, não aposta. |
| Força da evidência | strong | Riot/Costikyan/Lost Garden/GameDiscoverCo primários e datados sobre RNG pré-ação, odds expostas e maestria. |
| Valor vs. inação | adequate | Transformador semeado agrega variedade auditável; inação evita risco, mas deixa o tema sem reformulação. |
| Viabilidade/Appetite | strong | `fator`/`misturar`/`hash_estado` já existem no `sim`; sem serviço externo; ADR-004 intacto. |
| Alinhamento estratégico | adequate | Serve I/IV/VI se transformador e auditável; conflita com V se virar vitória por sorte — guardrail resolve. |
| Postura de risco | adequate | Risco de reapresentar aposta/opacidade mitigado por critérios de aceite explícitos (odds inspecionáveis, zero estética de cassino). |

## Racional

ADAPT: a literatura é clara de que o que retém em slot é a **conversão de sorte em habilidade** (rerolls baratos em CloverPit, 目押し em Slots & Daggers, "decisão clara" em Balatro) e a aleatoriedade **exposta, positiva e controlada** (Costikyan, Lost Garden, Riot). O que o GDD §2.6 e a Constituição V proíbem é a vitória por azar/opacidade, não a variação por semente — e o repo já tem RNG determinístico em `crates/sim/src/lib.rs`. Portanto, a mecânica entra **somente como transformador semeado de materiais/funções**, com odds inspecionáveis, pool sem reposição, RNG pré-ação e reroll de custo conhecido — **sem aposta e sem estética de cassino** (guardrail do orquestrador). Nenhum backend externo (IV), nenhuma vitória por "sortudo" (V), clareza causal preservada (VI). ADR-004 (Bevy+Lightyear), regra `--minimizado`, 5 crates/34 testes e CI verde seguem norma; saída de LLM informativa. Nenhum build/teste/gate rodado nesta etapa.

## Condição/responsável

- **Responsável**: mantenedor/SE — autoridade de design; a forma final (odds, custos, exposição no HUD) é decidida na spec, não aqui.
- **Guardrails obrigatórios**: (1) seed registrada + odds inspecionáveis no código; (2) RNG pré-ação, pool sem reposição, reroll com custo conhecido; (3) **zero aposta, zero estética de cassino, zero prêmio/near-miss**; (4) vitória por coordenação (V/GDD §4.4), nunca por sorte; (5) nenhum serviço externo de odds (IV).
- **Gate**: CI verde autoriza merge, nunca auto-merge cego; nenhuma alegação de verde sem comando executado (VI). Nenhum build/teste/gate rodado nesta etapa.

## Handoff

- **Problema**: como dar variedade por run sem importar aposta/opacidade, usando o RNG determinístico já presente no `insurgencia-sim`.
- **Abordagem**: Opção 1 do concept — transformador semeado de materiais/funções, odds expostas, pool sem reposição, RNG pré-ação, custo de reroll conhecido, sem cassino; coordenar com `sb-11` (fusão como cadeia produtiva) e `sb-15` (clareza no HUD).
- **In scope (handoff)**: definir odds/custo, semântica material da amostra, exposição legível das odds e teste de reprodutibilidade por `hash_estado()` no `sim`.
- **Out of scope**: aposta, prêmio, near-miss, odds opacas, RNG pós-ação, backend de probabilidades, estética de rolos/luzes/som de cassino, moeda premium, serviço externo, alteração em ADR-004 ou na condição de vitória do GDD.
- **Métricas**: mesma semente → mesma sequência (reprodutibilidade auditável); odds consultáveis no código e no HUD; zero mecanismos de aposta; nenhuma vitória atribuída a sorte; legibilidade validada sob `--minimizado`.
- **Perguntas remanescentes**: `[NV]` onde o transformador entra no loop (materialização vs. cadeia produtiva)? `[NV]` custo de reroll em materiais (não moeda premium)? `[NV]` as odds cabem no teto de complexidade do HUD (Riot)? `[NV]` validação de legibilidade por teste, não por opinião — como medir?
