# Research — Segmentação regional (sb-23)

**Data**: 2026-11-21 · **Idiomas**: EN primário; ZH/JA/KO via fontes secundárias. Números do vídeo tratados como **afirmação a verificar**, não dado.

## Notas de pesquisa consultadas
- `docs/research/slotbound-2026-11-21-disclosure-legal.md` — achado 4 (percentuais regionais) e 9 (causalidade incerta da segmentação).
- `docs/research/slotbound-2026-11-21-steam-gtm.md` — 13 idiomas na loja; visibilidade (idioma/tags pesam, review score só abaixo de 40%); Steam como canal ausente no repo.
- `docs/research/slotbound-2026-11-21-alternativas.md` — sb-23: READMEs traduzidos, itch.io e DNSLink como localização documental.
- `docs/research/slotbound-2026-11-21-fit-local.md` — sb-23: `docs/` hoje só pt-BR; campanha/publicação DEFER sem superfície de distribuição.

## Achados
1. **Os quatro percentuais regionais vêm apenas do vídeo.** ZH simplificado 68%, EN ~25% das avaliações a 93%, KO 79%, ZH tradicional 77% — todos são leitura de tela. A API `appreviews` por idioma redirecionou para a home da Steam **[NV — acesso falhou]**; não foi possível reproduzi-los.
2. **O 79% global, sim, é verificável.** Confirmado na loja em 21/11 (1.598 reviews) e em OhMyGame/tryaigame em 17/11 (921 reviews). É número dinâmico: mudou entre as duas datas; os demais percentuais não têm essa confirmação.
3. **A inferência do apresentador é hipótese, não estudo.** "O público chinês tem menos aversão a arte IA" é leitura do vídeo a partir dos percentuais; a amostragem de reviews negativas é marginal e o próprio vídeo diz que as críticas eram de RNG/balance.
4. **O que é transportável é preparar texto para localização, não perseguir mercado tolerante.** O postmortem do Scale the Depths cita explicitamente o custo de texto embutido em imagens; o projeto tem HUD 100% em `format!` e `docs/` só em pt-BR.
5. **Não há superfície de publicação hoje.** Não existe Steam page nem steamworks no repo; campanha regional depende de canal externo e de capacidade de tradução **[NV]**. A parte documental (texto em dados) pode começar agora.

## Fontes (URLs)
- Steam store — Slotbound (13 idiomas; 79% de 1.598): https://store.steampowered.com/app/4459590/Slotbound/
- Steamworks — Visibility (idioma/tags; score só <40%): https://partner.steamgames.com/doc/marketing/visibility
- Steamworks — User Reviews: https://partner.steamgames.com/doc/store/reviews
- Reddit r/gamedev — postmortem Scale the Depths (texto em imagens): https://www.reddit.com/r/gamedev/comments/1tq3sv6/our_game_jam_entry_blew_up_and_we_turned_it_into/
- OhMyGame — estudo de caso (921 reviews em 17/11): https://www.ohmygame.ai/blog/slotbound
- TryAIgame — perfil ZH: https://tryaigame.com/zh/games/slotbound/
- Steam `appreviews` — tentativa por idioma **[NV]**: https://store.steampowered.com/appreviews/4459590?json=1
- Vídeo Garry Seto [NV — não buscado]: https://www.youtube.com/watch?v=Z4ayMTXnkoQ
