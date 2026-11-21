# Decision — Tolerância do mercado a IA

- **Slug**: sb-25-tolerancia-mercado-ia
- **Decided**: 2026-11-21
- **Veredito**: DEFER

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | Reação do mercado é relevante, mas a pergunta é externa ao controle do projeto. |
| Força da evidência | weak | Caso único; amostragem marginal, comentários hostis e "outro jogo" **[NV]**. |
| Valor vs. inação | weak | Não adotar a premissa custa quase nada; adotá-la pode corroer transparência. |
| Viabilidade / Appetite | adequate | Monitorar é barato, mas não muda nenhum gate nem surface de código. |
| Alinhamento estratégico | adequate | Manter métricas locais casa com I/VI; importar clima externo não. |
| Postura de risco | weak | Viés de sobrevivência é o risco central (outlier entre 7.240 demos); não mitigável com o dado atual. |

## Racional
DEFER: a tolerância observada em Slotbound é **caso único de sobrevivência** — 79% positivo é outlier entre milhares de demos, a reação anti-IA foi marginal na amostragem mas hostil em fóruns, e o "outro jogo vendendo bem na mesma semana" não foi identificado **[NV]**. Não se generaliza tolerância de mercado a partir disso. A hipótese fica registrada, sem virar KPI, gate ou justificativa para afrouxar disclosure (sb-24) nem para segmentar por percentuais não verificados (sb-23). Reavaliar apenas com múltiplos casos independentes e fonte primária.

## Condição/responsável
Condição: reabrir somente com dados primários de mais de um caso, com população/date explícitos; enquanto isso, rótulo **[NV]** para o caso paralelo. Responsável: SE se trouxer evidência nova; nenhuma mudança de artefato é autorizada por esta decisão. CI/merge **[NV]** até verificação ao vivo (VI).
