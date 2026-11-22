# Research — Vídeos de curto formato (sb-20)

**Data**: 2026-11-21. **Escopo**: ponteiros para as notas de evidência; não repito a pesquisa completa.

## Notas de evidência consultadas
- `docs/research/slotbound-2026-11-21-ai-promo.md` — short-form, captura auditável, disclosure (sb-05/06/20).
- `docs/research/slotbound-2026-11-21-fit-local.md` — captura sob `--minimizado` / exemplo headless do `sim`.
- `docs/research/slotbound-2026-11-21-alternativas.md` — devlogs versionados como alternativa.

## Achados relevantes
1. A recomendação de "shorts bem editados" é opinião de Garry Seto, que observou que o TikTok de Slotbound "não parece ter viralizado" (https://www.youtube.com/watch?v=Z4ayMTXnkoQ). Números vêm de fontes com interesse: Trapster, campanha **paga**, +11.961 wishlists a US$0,20 cada (https://www.trapster.gg/case-studies/catch-meow-if-you-can-tempest-games); Mortal Rite ~1 wishlist por 2.000+ views (https://newsletter.gamediscover.co/p/this-game-used-tiktok-to-get-120m); Kasspian, 44 relatos de insucesso contra 10 de sucesso (https://kasspian.com/research/wishlists).
2. Medição oficial existe: Steamworks UTM Analytics registra visitas/conversões em janela de 72 h (https://partner.steamgames.com/doc/marketing/utm_analytics) — medir wishlists, não views.
3. Produção auditável sem janela é viável: Bevy `headless_renderer` desliga `WinitPlugin`, usa `primary_window: None` e `ScheduleRunnerPlugin` (https://github.com/bevyengine/bevy/blob/main/examples/app/headless_renderer.rs); FFmpeg documenta `-bitexact` (https://ffmpeg.org/ffmpeg.html). Publicar semente, comando e `sha256` com o clipe. **[NV — captura/encode não executados; bit-exatidão entre versões não verificada]**.
4. Fit local: HUD é `format!` e a sim é headless; captura sob `--minimizado` atende à regra SE, mas o exemplo do `sim` é preferível para quadros 100% reproduzíveis.
5. **[NV]** Não há conta/canal do projeto, nem dados UTM próprios; eficácia para o Insurgência não estimável. Publicar em plataforma fica DEFER até existir conta; rotulagem de qualquer material sintético é obrigatória conforme notas de disclosure.
