# Problem — Tolerância do mercado a IA (sb-25)

**Data**: 2026-11-21

## Problema concreto no projeto
Existe a tentação de usar "o mercado tolera IA" como premissa de planejamento: afinal, Slotbound teve 79% de reviews positivas e supostamente quase nenhuma reclamação anti-IA. O problema é que isso é **um caso único**, com amostragem marginal, comentários hostis documentados em fóruns e um "outro jogo" sequer identificado **[NV]**. Importar essa tolerância como premissa levaria a afrouxar disclosure (sb-24), a perseguir mercados por percentuais não verificados (sb-23) e a trocar as métricas verificáveis do projeto por clima de mercado — exatamente o viés de sobrevivência que o próprio caso exemplifica.

## Sucesso observável
1. A afirmação "mercado tolera IA" é registrada como **hipótese**, nunca como KPI, gate ou justificativa de decisão.
2. Nenhuma decisão de disclosure, localização ou produto é baseada no caso único; o "outro jogo" continua **[NV]** até identificado com fonte primária.
3. Valor é medido por sinais locais verificáveis: `verify.sh`, gates, 34 testes, downloads de releases — não por sentimento externo.
4. O 79% quando citado vem com data/fonte e população (jogo em 21/11, não demo em 04/11).
5. Nada é chamado de verificado sem comando executado (VI); CI/merge **[NV]** até verificação ao vivo.

## Custo de não agir
"Não agir" aqui é justamente não adotar a premissa — e isso custa quase nada. O custo de **agir** sobre ela é alto: planejar para um mercado inferido de um outlier, enfraquecer a transparência que é identidade do projeto e substituir evidência determinística por anedota. Por isso o veredito é DEFER, não ADOPT.
