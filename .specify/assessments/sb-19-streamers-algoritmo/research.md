# Idea Research: Streamers e algoritmo

- **Slug**: sb-19-streamers-algoritmo
- **Created**: 2026-11-21
- **Evidence confidence (overall)**: high para a documentação da Valve e para o fit local; low/`[NV]` para causalidade atribuída a streamers e algoritmo
- **Notas consultadas**: `docs/research/slotbound-2026-11-21-steam-gtm.md`; `docs/research/slotbound-2026-11-21-alternativas.md`; `docs/research/slotbound-2026-11-21-fit-local.md`

## Achados

1. **A mecânica "wishlists → visibilidade algorítmica" é negada pela Valve.** "Wishlists (Mostly) Aren't a Factor" (exceto Popular Upcoming); tráfego da loja, conversão e review score (acima de 40%) também não contam; o que pesa é jogar/comprar, idioma e tags. Não há opção paga de visibilidade. [source: steam-gtm achado 4 e C11; https://partner.steamgames.com/doc/marketing/visibility; https://partner.steamgames.com/doc/marketing/wishlist; confidence: high]
2. **Streamers ocorreram como canal, não como causalidade quantificada.** Vídeo de Gohjoe em 11/07 (dia seguinte à demo), youtuber vietnamita citado em comentários, comentaristas relatando wishlist após vídeos; o próprio dev oscila entre "game market itself", "steam algorithm and youtubers" e "I was just lucky". OhMyGame: "não podemos estabelecer quanto cada passo contribuiu". Efeito quantificado `[NV]`. [source: steam-gtm achado 5 e C12; https://www.ohmygame.ai/blog/slotbound; https://www.reddit.com/r/SoloDevelopment/comments/1wr9z8o/my_first_game_just_reached_100k_wishlists_still/; confidence: high para ocorrência, low para efeito]
3. **A crença em "algoritmo" é crença de praticante, não documento oficial.** Unity/Zukowski falam em "primeiro dia em que o algoritmo nota", mas a doc da Valve diz que os primeiros dias do Next Fest são **completamente aleatórios** e depois personalizados por comportamento do usuário; tráfego privado (`Traffic Breakdown`) não foi observado. [source: steam-gtm tabelas Next Fest/Visibility; https://unity.com/cn/blog/steam-next-fest-developer-tips-unity; https://partner.steamgames.com/doc/marketing/upcoming_events/nextfest; confidence: high]
4. **O repo pode produzir material melhor que um vídeo editado.** `--minimizado` mantém a SE sem perturbação; o adaptador `atualizar_texto` monta HUD em `format!` de estado (posição, ilha, limites) sem assets; um exemplo em `crates/sim/examples/` gera saída idêntica com seed fixa e entra em `verify.sh --all-targets`; CI já produz artefatos e há canal CI→`gh`. [source: fit-local achados 1–3 e 6–7; alternativas ALT-13/14/16; local://crates/cliente/src/main.rs:547-561; local://verify.sh; confidence: high]
5. **A alternativa transferível é a forma do material, não o mecanismo de plataforma.** Devlogs honestos com captura reproduzível, comando e seed anexados são falsificáveis e auditáveis (constituição VI); vídeo editado não é. Saída de LLM é informativa, nunca autoritativa. [source: alternativas achado 8 e ALT-13/15; https://itch.io/devlogs; https://docs.github.com/en/actions/concepts/security/artifact-attestations; confidence: high]

## Contra-evidência e `[NV]`

- Alcance/fediverso e qualquer número de views/Conversão atribuído a criadores não foram verificados `[NV]`.
- Telemetria Steam (SteamDB 403) e `Traffic Breakdown` privado não foram observados `[NV]`.
- O vídeo original de Garry Seto não foi recuperado (host youtu.be recusado); atribuições vêm do resumo da SE `[NV]`.
- "O algoritmo amplificou por causa das wishlists" é parcialmente contradito pela doc oficial e permanece sem medição `[NV]`.
- Captura reproduzível não prova que alguém assistiu; é condição de auditoria, não de alcance `[NV]`.

## Fontes

- `docs/research/slotbound-2026-11-21-steam-gtm.md` (local) — C11/C12, achados 4–5, tabelas Steamworks.
- `docs/research/slotbound-2026-11-21-alternativas.md` (local) — ALT-13/14/15/16, devlogs e atestação.
- `docs/research/slotbound-2026-11-21-fit-local.md` (local) — achados 1–3 e 6–7, matriz sb-19 (nota: matriz local sugeria DEFER; o veredito fixado neste fluxo é ADAPT pela forma auditável do material).
- URLs primárias nos achados 1–5; nenhuma fonte remota adicional buscada neste artefato.
