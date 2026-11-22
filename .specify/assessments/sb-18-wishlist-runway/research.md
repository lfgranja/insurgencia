# Idea Research: Runway de wishlists

- **Slug**: sb-18-wishlist-runway
- **Created**: 2026-11-21
- **Evidence confidence (overall)**: low para os números do resumo (`[NV]`); high para a documentação oficial da Valve e para as métricas locais
- **Notas consultadas**: `docs/research/slotbound-2026-11-21-steam-gtm.md`; `docs/research/slotbound-2026-11-21-alternativas.md`; `docs/research/slotbound-2026-11-21-fit-local.md`

## Achados

1. **Os números do resumo não resistem à checagem.** "591 em março" não foi localizado: a fonte primária diz ~590 wishlists em **julho** de 2026, antes da demo. "40 mil em 2 semanas" também não aparece; a série documentada é ~590 → ~7.000 em 3 dias → 50.000 em 17 dias (27/07) → 70k (11/08) → 100k (27/09). O "runway de 4,5 meses" não tem fonte primária — `[NV]` nos três. [source: steam-gtm C1/C2/C10; https://www.reddit.com/r/IndieDev/comments/1utoziq/i_expected_maybe_10_concurrent_players_my_first/; https://www.reddit.com/r/SoloDevelopment/comments/1vldq7y/my_first_game_had_590_wishlists_when_i_released/; https://www.ohmygame.ai/blog/slotbound; confidence: high de que são `[NV]`, não de valores alternativos]
2. **100k wishlists pré-lançamento é o único número confirmado** (Steam News 26–27/09 e post do dev 27/09), distinto de 100k **cópias** em 3 dias (16/11) e de 79% de 1.598 reviews em 21/11 — populações e momentos diferentes. [source: steam-gtm C3/C4/C5; https://store.steampowered.com/news/app/4459590/view/696524994028504801; https://steamcommunity.com/app/4459590/allnews; https://store.steampowered.com/app/4459590/Slotbound/; confidence: high]
3. **A Valve desmente a mecânica implícita na meta.** "Wishlists (Mostly) Aren't a Factor" (exceto Popular Upcoming), tráfego e conversão não contam, não há fórmula para prever vendas a partir de wishlists e não há mínimo exigido. Planejar runway em cima de wishlists é otimizar uma variável que a plataforma diz não usar. [source: steam-gtm achado 4; https://partner.steamgames.com/doc/marketing/visibility; https://partner.steamgames.com/doc/marketing/wishlist; confidence: high]
4. **O repo já tem métricas próprias e distribuição de custo zero.** GDD §17 mede verificação da tese, não wishlists; `verify.sh` + 5 gates + 34 testes são a autoridade; GitHub Releases é gratuito e itch.io permite 0% de comissão. Runway pode ser expresso como orçamento registrado, custo de CI/arte e prazo até o próximo marco — sem serviço externo. [source: fit-local implicações sb-18; alternativas ALT-8/9 e "runway otimizado"; local://docs/GDD%20-%20Insurgência.md §17; confidence: high]
5. **Outliers enviesam qualquer benchmark de "tempo até 100k".** Mediana de ganho por demo em out/2026 foi 806–1.079 wishlists (7.240 demos), contra +80k nos dez maiores; DORA 2025 e METR reforçam que velocidade atribuída a IA/campanha é amplificador condicionado ao sistema, não receita. [source: steam-gtm achado 6; alternativas achados 2–3; https://voxbooster.com/pt-BR/blog/steam-next-fest-demo-statistics-2026; https://dora.dev/research/2025/dora-report; https://metr.org/blog/2026-02-24-uplift-update; confidence: high]

## Contra-evidência e `[NV]`

- "591 em março", "40k em duas semanas", "4,5 meses" e a autoria exata das frases do vídeo: `[NV]` (vídeo não recuperado; host youtu.be recusado pela política de URL).
- Runway financeiro do dev do caso é pessoal e não transferível; nenhuma fonte primária o confirma `[NV]`.
- Nenhum dado de conversão wishlist→compra foi publicado; estimativas de 2,7%–3,8% são de terceiros `[NV]`.
- Medir interesse por downloads de releases é sinal local, não prova de demanda de mercado `[NV]`.

## Fontes

- `docs/research/slotbound-2026-11-21-steam-gtm.md` (local) — C1/C2/C3/C10, tabela claim→verificado, achado 4.
- `docs/research/slotbound-2026-11-21-alternativas.md` (local) — ALT-8/9, DORA/METR, custo zero de distribuição.
- `docs/research/slotbound-2026-11-21-fit-local.md` (local) — GDD §17, gates, matriz sb-18.
- URLs primárias nos achados 1–5; nenhuma fonte remota adicional buscada neste artefato.
