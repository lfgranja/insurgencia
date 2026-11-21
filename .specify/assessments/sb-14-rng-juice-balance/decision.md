# Decision — RNG, juice e balance (sb-14)

- **Slug**: sb-14-rng-juice-balance
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | RNG já existe no `sim`, mas não é inspecionável pelo jogador/fork; problema real de legibilidade. |
| Força da evidência | adequate | TFT, Costikyan e Lost Garden são primárias; atribuição das críticas ao RNG no vídeo é hipótese [NV]. |
| Valor vs. inação | strong | Expor semente/odds transforma "balance" em discussão auditável, com baixo custo. |
| Viabilidade/Appetite | strong | `fator`, `misturar` e `hash_estado` já implementados; falta apresentação e medição. |
| Alinhamento estratégico | strong | Reforça determinismo (I) e verificação honesta (VI) sem tocar em gênero. |
| Postura de risco | adequate | "Mais RNG/juice" pode virar ruído/near-miss; mitigado por amostragem pré-ação e telegrafia. |

## Racional
ADAPT porque o núcleo técnico da ideia é bom, mas a forma proposta pelo vídeo não é: a literatura mostra que a aleatoriedade retém quando é positiva, previsível e convertível em habilidade, e que juice serve à clareza causal. O Insurgência já tem RNG determinístico — a adaptação é expor semente e odds, amostrar antes da decisão e subordinar qualquer feedback à causa observável. Rejeita-se a estética de aposta e o "RNG acima de tudo". Nenhuma dependência externa ou alteração de engine (ADR-004 intacto).

## Condição/responsável
Responsável: SE com o fluxo SDD; a implementação depende do seam de HUD (comparar com sb-15) e da aprovação via `verify.sh` + 5 gates antes de qualquer change. Verde local/CI autoriza merge; saída de LLM permanece informativa.

## Handoff
- **Problema**: aleatoriedade determinística, porém opaca quanto a semente, odds e causalidade do feedback.
- **Abordagem**: RNG semeado, positivo e amostrável antes da decisão; odds/semente no HUD; juice apenas como sinal de causa.
- **In scope**: exposição de semente/odds no adaptador de apresentação; documentação do contrato de RNG no `sim`; verificação por `hash_estado` com semente fixa.
- **Out of scope**: apostas, near-miss, odds ocultas, RNG pós-ação,efeitos que escondam informação, assets externos, troca de engine.
- **Métricas**: reprodução idêntica de `hash_estado` para a mesma semente; 34 testes + 5 gates PASS; inspeção de HUD sob `--minimizado` com causas numeradas **[NV até verificação ao vivo]**.
- **Perguntas remanescentes**: qual semântica material justifica aleatoriedade aqui (terreno/clima, não "prêmio")? Como medir legibilidade sem abrir janela normal? As críticas alegadas existem em fontes primárias [NV]?
