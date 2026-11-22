# Decision — Runway de wishlists

- **Slug**: sb-18-wishlist-runway
- **Decided**: 2026-11-21
- **Veredito**: DEFER
- **Artifacts reviewed**: intake.md; research.md; problem.md; concept.md

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | A necessidade de planejar é real, mas o problema formulado usa números `[NV]` e métrica sem canal mensurável. |
| Força da evidência | weak | Três dos quatro números do resumo são `[NV]`; o único confirmado não é meta de valor e a Valve nega o mecanismo. |
| Valor vs. inação | adequate | Adiar evita metas falsas; instrumentar runway local tem valor moderado e reversível. |
| Viabilidade / Appetite | weak | Wishlists exigem Steam page inexistente (US$100, SDK); dados de solidez financeira não foram fornecidos. |
| Alinhamento estratégico | adequate | GDD §17 já define sucesso como verificação da tese; wishlists são alheias a esse critério. |
| Postura de risco | weak | Risco de decisão financeira baseada em outlier e de "verde falso" por número de terceiro; mitigado só com dados locais. |

## Racional

DEFER: o "runway de 4,5 meses" é `[NV]`, "591 em março" era ~590 em julho e "40 mil em duas semanas" é documentado como 50k em 17 dias. Mesmo a cifra confirmada (100k wishlists em 27/09) é de outro projeto e, segundo a documentação da Valve, wishlists "não são um fator" de visibilidade (exceto Popular Upcoming) e não prevendem vendas. A Insurgência não tem Steam page para medir wishlists, e seu próprio critério de sucesso (GDD §17) é verificação da tese, não contagem de terceiros. O diferimento não congela o planejamento: substitui a meta emprestada por runway registrado localmente (orçamento, custo de CI/arte, marco seguinte) e por contadores próprios (downloads de releases, execuções da demo de sb-16). Reabre quando houver dados locais e data definida, nunca ao atingir número de outlier.

## Condição/responsável

- Responsável: SE (orçamento e marco) + Main (contadores de releases e da demo, registro em `docs/PROGRESSO.md`).
- Condição de reavaliação: registro de runway local preenchido, sb-16 publicada, data de revisão definida antes de qualquer gasto com plataforma. Nenhum número `[NV]` entra em decisão; nada é declarado verde sem comando executado.

## Reavaliação

Não se aplica (DEFER). Reavaliação condicionada a runway local registrado e contadores próprios disponíveis. Nenhuma meta de wishlist, Steam page ou previsão de receita entra no planejamento neste ciclo. Os números do vídeo permanecem rotulados `[NV]` e citados apenas como exemplo de viés de sobrevivência.
