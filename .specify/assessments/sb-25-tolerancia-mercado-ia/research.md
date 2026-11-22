# Research — Tolerância do mercado a IA (sb-25)

**Data**: 2026-11-21 · **Idiomas**: EN primário; PT-BR/ES/ZH/JA/KO apenas como contexto. Afirmações do vídeo/resumo tratadas como **hipóteses**, nunca como premissa de planejamento.

## Notas de pesquisa consultadas
- `docs/research/slotbound-2026-11-21-disclosure-legal.md` — achado 5 (reação anti-IA marginal, porém fraca; caso único de sobrevivência).
- `docs/research/slotbound-2026-11-21-steam-gtm.md` — viés de sobrevivência (7.240 demos em out/2026; mediana de três dígitos baixos); "algoritmo" como crença, não mecanismo.
- `docs/research/slotbound-2026-11-21-alternativas.md` — DORA/METR: IA é amplificador; metric "100K em 3 dias" é viés de sobrevivência.
- `docs/research/slotbound-2026-11-21-fit-local.md` — sb-25: clima de mercado é externo, sem fit de código; DEFER/AVOID como premissa.

## Achados
1. **A reação anti-IA foi marginal na amostragem, mas a evidência é fraca.** O vídeo diz que as críticas eram de RNG/balance; o breakdown de r/aigamedev diz o mesmo. Porém r/IndieDev e r/aigamedev têm comentários explicitamente hostis ("ai slop", "hate the lack of transparency"), com resposta do autor. é contradição de tom, não ausência de reação.
2. **É um caso único de sobrevivência.** 79% positivo de 1.598 reviews (21/11) é um outlier; em out/2026 havia 7.240 demos e o ganho mediano autorrelatado foi de 806–1.079 wishlists, contra +80k para os dez maiores. Generalizar tolerância de mercado a partir daqui é erro de inferência.
3. **O "outro jogo com IA que vendeu bem na mesma semana" não foi identificado.** Nenhuma fonte primária localizada com nome, vendas e datas; a afirmação permanece **[NV]** e não sustenta comparação.
4. **"Mudança rápida de sentimento" é observação informal.** Não há série temporal de sentimento nem medição; o que existe é a variação do próprio score (82% na demo em 04/11; 79% no jogo em 21/11) — populações e datas distintas, não uma tendência de mercado.
5. **Métricas próprias substituem clima externo.** DORA e o AI Capabilities Model dizem que IA é amplificador e o retorno vem do sistema organizacional; o projeto já tem `verify.sh`, gates e 34 testes como medida de valor verificável.

## Fontes (URLs)
- Reddit r/IndieDev — conflito de transparência: https://www.reddit.com/r/IndieDev/comments/1ustzth/8_months_ago_i_had_never_made_a_game_today_my/
- Reddit r/aigamedev — post original e comentários: https://www.reddit.com/r/aigamedev/comments/1tkeg86/im_not_a_developer_and_this_slot_machine/
- Steam store — Slotbound (79% de 1.598): https://store.steampowered.com/app/4459590/Slotbound/
- GameMeca — demo 82% (04/11): https://www.gamemeca.com/en/view.php?gid=1782926
- VoxBooster — estatísticas Next Fest out/2026 (7.240 demos; mediana): https://voxbooster.com/pt-BR/blog/steam-next-fest-demo-statistics-2026
- DORA — State of AI-assisted Software Development 2025: https://dora.dev/research/2025/dora-report
- METR — desenho do experimento de produtividade: https://metr.org/blog/2026-02-24-uplift-update
- Vídeo Garry Seto [NV — não buscado]: https://www.youtube.com/watch?v=Z4ayMTXnkoQ
