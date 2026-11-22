# Idea Research: Demo Steam + Next Fest

- **Slug**: sb-17-demo-steam-next-fest
- **Created**: 2026-11-21
- **Evidence confidence (overall)**: high para pré-requisitos e janelas; low/`[NV]` para o efeito atribuído ao evento
- **Notas consultadas**: `docs/research/slotbound-2026-11-21-steam-gtm.md`; `docs/research/slotbound-2026-11-21-alternativas.md`; `docs/research/slotbound-2026-11-21-fit-local.md`

## Achados

1. **O efeito do Next Fest no caso estudado não está demonstrado.** 100k wishlists foram confirmadas em 26–27/09/2026, antes do evento de outubro; o dev anunciou participação e update de demo, mas slotbound.org não encontrou post confirmando inscrição. Causalidade `[NV]`/contradizida. [source: steam-gtm C3/C8, achado 3; https://store.steampowered.com/news/app/4459590/view/696524994028504801; https://www.reddit.com/r/SoloDevelopment/comments/1wr9z8o/my_first_game_just_reached_100k_wishlists_still/; https://slotbound.org/release-date; confidence: high para a data, low para o efeito]
2. **Next Fest é janela única por título e cara em preparação.** Três edições/ano (fev, jun, out), uma participação por título, store page pública, demo jogável e build review com 3–5 semanas de antecedência; edição Feb/2027: 22/02–01/03, registro até 10/01/2027. [source: steam-gtm tabela Steamworks Next Fest; https://partner.steamgames.com/doc/marketing/upcoming_events/nextfest; https://partner.steamgames.com/doc/marketing/upcoming_events/nextfest/feb_2027; confidence: high]
3. **A distribuição de atenção é extremamente assimétrica.** Em out/2026, 7.240 demos competiram; ganho mediano autorrelatado foi de 806–1.079 wishlists, enquanto os dez maiores somaram +80k; "é muito raro um jogo aparecer do nada" e recomenda-se contatar criadores antes do evento. [source: steam-gtm achado 6; https://voxbooster.com/pt-BR/blog/steam-next-fest-demo-statistics-2026; https://www.makegoodartagency.com/como-preparar-tu-participacion-en-steam-next-fest; confidence: high]
4. **Pré-requisitos de plataforma não existem no repo.** Não há Steam page, App ID, build Steam nem dados de wishlist; Steam Direct custa US$100 por app e exige SDK proprietário (MIT é compatível, mas licença não é infraestrutura). O demo headless (`crates/sim/examples/demo_deterministica.rs`) é pré-requisito local de qualquer empacotamento. [source: fit-local achados 1–2 e matriz sb-17; alternativas ALT-11; https://partner.steamgames.com/doc/gettingstarted/appfee; https://partner.steamgames.com/doc/sdk/uploading/distributing_opensource; confidence: high]
5. **A tese "wishlists alimentam o algoritmo" é contrariada pela própria Valve.** Wishlists não são fator de visibilidade (exceto Popular Upcoming); tráfego e conversão tampouco; o que pesa é jogar/comprar, idioma e tags. A visibilidade dos primeiros dias do festival é aleatória. [source: steam-gtm achado 4; https://partner.steamgames.com/doc/marketing/visibility; https://partner.steamgames.com/doc/marketing/wishlist; confidence: high]

## Contra-evidência e `[NV]`

- Dev disse textualmente "steam next festival on October" e preparava update de demo — fonte conflita com slotbound.org; inscrição real `[NV]`.
- Nenhum dado de ganho atribuível ao festival foi publicado `[NV]`.
- Números de pico de jogadores vieram de imprensa e posts, não de telemetria Steam (SteamDB 403) `[NV]`.
- O repo não tem “green de loja” para mostrar; nada aqui autoriza publicar.

## Fontes

- `docs/research/slotbound-2026-11-21-steam-gtm.md` (local) — C3/C8, tabelas Steamworks, achados 3/4/6.
- `docs/research/slotbound-2026-11-21-alternativas.md` (local) — ALT-11 (Steam Direct), custo e compatibilidade MIT.
- `docs/research/slotbound-2026-11-21-fit-local.md` (local) — ausência de Steam page/App ID e pré-requisito `sb-16`.
- URLs primárias nos achados 1–5; nenhuma fonte remota adicional buscada neste artefato.
