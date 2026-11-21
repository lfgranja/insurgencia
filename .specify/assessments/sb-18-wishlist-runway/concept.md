# Concept: Runway de wishlists

- **Slug**: sb-18-wishlist-runway
- **Created**: 2026-11-21
- **Recommended option**: Option A — Adiar a meta e instrumentar runway local verificável

## Options

### Option A — Adiar a meta e instrumentar runway local verificável

- **Sketch**: não adotar nenhum número do vídeo. Registrar em documento local o orçamento disponível, o custo real de CI/arte e o prazo até o próximo marco do SDD; usar como sinal de interesse os downloads de releases (API do GitHub) e as execuções da demo determinística de sb-16. Manter GDD §17 como critério de sucesso (verificação da tese). Marcar explicitamente `[NV]` os dados do caso e agendar reavaliação antes de qualquer gasto com plataforma.
- **Appetite**: small (horas) — um registro documental e dois contadores já disponíveis.
- **Trade-offs**: produz um plano falsificável e soberano, mas não entrega a sensação de "meta grande" do caso estudado e não mede demanda de mercado fora do repo (isso permanece `[NV]`). Depende de sb-16 para o contador de execuções da demo.
- **Rabbit holes**: disfarçar meta de wishlist como "métrica local"; inflar downloads como prova de adoção; transformar o registro em previsão financeira sem custo observado.

### Option B — Adotar o runway do caso como plano

- **Sketch**: assumir 4,5 meses de marketing e perseguir a curva 591 → 40k → 100k wishlists.
- **Appetite**: large — exige campanha, Steam page e recursos que não existem.
- **Trade-offs**: dá uma meta clara e espelha um sucesso documentado, mas três dos quatro números são `[NV]`, a meta depende de plataforma inexistente e a Valve nega que wishlists movam visibilidade; otimizar a variável errada.
- **Rabbit holes**: corrigir "591 em março" para "590 em julho" e tratar o erro como detalhe; usar viés de sobrevivência como estratégia.

### Option C — Criar Steam page para medir wishlists

- **Sketch**: publicar página e perseguir wishlists diretamente como instrumento de planejamento.
- **Appetite**: large — US$100, SDK, assets e janela de loja; ver sb-17 (DEFER).
- **Trade-offs**: mediria wishlists de fato, mas introduz dependência proprietária antes de existir demo versionada e repete o erro de tratar a métrica da loja como mecanismo de valor; a própria Valve desaconselha usá-las como previsão.
- **Rabbit holes**: confundir "coletar métrica" com "gerar valor"; gastar a janela de Next Fest como efeito colateral.

## Recommendation

Option A, coerente com o veredito DEFER: a decisão sobre runway e wishlists fica suspensa até existirem dados próprios, e o que se faz agora é preparar a medição local. Isso preserva a honestidade exigida pela constituição VI (nada `[NV]` vira premissa), evita otimizar uma variável que a Valve diz ser irrelevante para visibilidade e aproveita o que o repo já possui (gates, GDD §17, releases gratuitas). O deferimento termina quando houver orçamento registrado, sb-16 publicada e uma data de reavaliação — não quando uma meta emprestada for atingida.

## Out of Scope (for the recommended option)

- Adotar 4,5 meses, 591, 40k ou 100k como meta ou premissa de planejamento.
- Steam page, App ID, US$100 ou campanha de wishlists.
- Previsões de receita conversion-rate de terceiros.
- Alterar GDD §17, gates, CI ou constituição.
- Declarar qualquer número `[NV]` como verificado.

## Assumptions to Validate

- A API de releases do GitHub fornece contagem de downloads utilizável como linha de base (a verificar no ato da instrumentação).
- sb-16 publicará a demo determinística que permite contar execuções reproduzíveis.
- [NEEDS CLARIFICATION: orçamento e marco seguinte reais da SE para preencher o registro de runway.]
- [NEEDS CLARIFICATION: data/critérios de reavaliação de plataforma paga.]

## Nota de fronteira

O valor de 100k wishlists do caso é um fato sobre **outro projeto**; serve como exemplo de viés de sobrevivência, não como benchmark. Métricas de sucesso da Insurgência continuam sendo as de verificação da tese (GDD §17) e as saídas dos gates locais.
