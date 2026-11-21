# Decision — Segmentação regional

- **Slug**: sb-23-segmentacao-regional
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | Localização futura é real, mas os percentuais que motivaram a ideia são **[NV]**. |
| Força da evidência | weak | 79% global verificado; 68/93/79/77 vêm só do vídeo; API redirecionou **[NV]**. |
| Valor vs. inação | adequate | Texto em dados barateia i18n futura; campanha sem canal não agrega hoje. |
| Viabilidade / Appetite | strong | Externar strings é barato e local; campanha depende de tradução e loja **[NV]**. |
| Alinhamento estratégico | strong | Preparar o produto para o jogador casa com IV; sem serviço externo no caminho crítico. |
| Postura de risco | adequate | Risco está em agir sobre números não reproduzidos; mitigado ao mantê-los **[NV]** e adiar campanha. |

## Racional
ADAPT **somente na parte defensiva/documental**: internacionalizar **texto em dados desde já** (strings externas, nada de texto desenhado em imagens), porque é barato e evita retrabalho futuro. A campanha regional e a publicação em loja ficam **DEFER**: não existe superfície de distribuição no repo e os percentuais 68/93/79/77 são leitura de tela do vídeo, não reproduzidos pela API **[NV]**. O 79% global é dinâmico e verificável (921 em 17/11; 1.598 em 21/11), mas não orienta segmentação.

## Condição/responsável
Condição: campanha só após superfície de publicação e capacidade de tradução; percentuais regionais não entram em planejamento enquanto **[NV]**. Responsável: SE pela eventual campanha; texto em dados é responsabilidade de quem toca HUD/docs. CI/merge **[NV]** até verificação ao vivo (VI).

## Handoff
Manter strings de HUD e documentação em dados, não em imagens; rotular os percentuais regionais como **[NV]**; não usar "aversão regional a IA" como premissa; campanha/publicação regionais ficam **DEFER**; saída de LLM informativa, nenhum serviço externo no caminho crítico.
