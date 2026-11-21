# Decision — Fusão

- **Slug**: sb-11-fusao
- **Decided**: 2026-11-21
- **Veredito**: ADAPT
- **Artifacts reviewed**: intake.md; research.md; problem.md; concept.md; `docs/research/slotbound-2026-11-21-game-design.md`; `docs/research/slotbound-2026-11-21-fit-local.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | Fusão é real em Slotbound, e fundir materiais serve o Pilar II; a demanda não é da SE e o uso cru conflita com V. |
| Força da evidência | strong | GDC/GamesIndustry, gamedesign.consulting e AppMagic primários sobre merge (feedback, saturação, estresse); Steam confirma hiper-unidade. |
| Valor vs. inação | adequate | Cadeia produtiva auditável agrega valor material; inação evita risco, mas perde mecanismo com apelo para o Pilar II. |
| Viabilidade/Appetite | strong | `hash_estado`/`sim` determinístico já dão auditoria; sem serviço externo; ADR-004 intacto. |
| Alinhamento estratégico | adequate | Serve II/IV/VI se materiais + teto + registro; conflita com V se fundir unidades — guardrail resolve. |
| Postura de risco | adequate | Risco de carry/hiper-unidade e complexidade de HUD mitigado por teto de poder, registro e críticas de aceite. |

## Racional

ADAPT: o que a fusão tem de bom (ordem, crescimento, feedback tátil repetido — GDC/GamesIndustry, gamedesign.consulting) pode servir ao **Pilar II (ação material)** se for aplicado a **materiais e funções complementares** em uma cadeia produtiva para a Ilha Central, e não a unidades. O que Slotbound faz — "absorb others … ascend into a stronger unit" — é hiper-unidade/carry, exatamente o "campeão solitário" proibido pela Constituição V e GDD §4.4/§5.1 (nota fit-local, achado 4). A saturação do gênero (AppMagic: 3 líderes com 99% da receita, "dead-end" em Merge-3/5) e o "complexity ceiling" da Riot reforçam a reformulação. Portanto, a fusão entra **somente como cadeia produtiva de materiais/funções complementares**, com **teto de poder por ator** e **registro auditável** (`hash_estado`), sem hiper-unidade e sem carry (guardrail do orquestrador). Nenhum serviço externo (IV), nenhuma vitória por poder individual (V), clareza causal preservada (VI). ADR-004 (Bevy+Lightyear), regra `--minimizado`, 5 crates/34 testes e CI verde seguem norma; saída de LLM informativa. Nenhum build/teste/gate rodado nesta etapa.

## Condição/responsável

- **Responsável**: mantenedor/SE — autoridade de design; receitas, tetos e feedback definidos na spec, não aqui.
- **Guardrails obrigatórios**: (1) fundir apenas **materiais e papéis/funções complementares** — nunca unidades/heróis; (2) **teto de poder por ator** e proibição explícita de "superunidade"/carry; (3) registro auditável de entradas/saídas via `hash_estado` + semente/commit; (4) vitória por coordenação (V/GDD §4.4), nunca por poder acumulado; (5) sem economia premium ou serviço externo (IV).
- **Gate**: CI verde autoriza merge, nunca auto-merge cego; nenhuma alegação de verde sem comando executado (VI). Nenhum build/teste/gate rodado nesta etapa.

## Handoff

- **Problema**: como capturar o valor material da fusão sem criar hiper-unidade/carry que substitua a coordenação.
- **Abordagem**: Opção 1 do concept — cadeia produtiva de materiais/funções complementares para a Ilha Central, com teto de poder por ator e registro auditável no `sim`; coordenar com `sb-10` (variedade de materiais) e `sb-15` (legibilidade no HUD).
- **In scope (handoff)**: definir receitas complementares, invariante de teto de poder, registro de entradas/saídas e feedback de causalidade no HUD.
- **Out of scope**: elenco de campeões, hiper-unidade, "ascensão" de unidade, carry, economia premium, RNG opaco, serviço externo, estética de caça-níquel, alteração na condição de vitória do GDD ou em ADR-004.
- **Métricas**: zero unidades fundidas em superunidade; teto de poder verificável como invariante no `sim`; cada fusão reproduzível por semente/commit com `hash_estado`; nenhuma vitória atribuída a poder individual; legibilidade validada sob `--minimizado`.
- **Perguntas remanescentes**: `[NV]` quais materiais/funções do GDD são complementares? `[NV]` como expressar o teto de poder sem número opaco? `[NV]` o feedback de causalidade cabe no HUD (Riot: "complexity ceiling")? `[NV]` como medir satisfação sem confiar em opinião (Constituição VI)?
