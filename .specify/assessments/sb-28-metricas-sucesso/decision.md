# Decision — Métricas de sucesso (100K/3 dias)

- **Slug**: sb-28-metricas-sucesso
- **Decided**: 2026-11-21
- **Veredito**: AVOID

| Critério | Avaliação | Justificativa (1 linha) |
|---|---|---|
| Validade do problema | strong | Risco real de importar KPI de vaidade alheio à tese de soberania. |
| Força da evidência | strong | Dados observáveis (Steam API/loja) vs. 100K autodeclarado [NV] + censo de 53.597 jogos. |
| Valor vs. inação | adequate | Evita decisões guiadas por sobrevivente amostrado; não entrega feature nova. |
| Viabilidade/Appetite | strong | Substituição por métricas locais já disponíveis em `verify.sh`/`sim`. |
| Alinhamento estratégico | strong | GDD §17 (verificação da tese) e constituição VI (verificação honesta). |
| Postura de risco | strong | Nenhuma mudança de código; risco apenas de narrativa, mitigado por política documental. |

## Racional
79% de 1.598 avaliações e 27.374 simultâneos são observáveis hoje, mas 100K/3 dias é autodeclarado [NV] e a queda 87%→78% mostra volatilidade. O censo de 53.597 jogos (topo 1% ≈94% da receita, 28% sem avaliações, jogos com IA a ~55% da taxa de conversão dos sem IA) mostra que Slotbound é o sobrevivente amostrado, não a base. Na Insurgência, o sucesso se mede por verificação da tese (GDD §17): tempo até gate verde, invariantes verificados e determinismo headless — nunca por números de vaidade.

## Condição/responsável
Responsável: SE (Garry Seto) e quem escreve specs/ADRs. Condição: nenhum KPI de 100K/3 dias ou percentual de avaliações; números externos apenas como contexto com fonte e [NV]; métricas oficiais do projeto são as locais e falsificáveis do GDD §17.
