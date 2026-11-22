# Idea Research: Demo no itch.io

- **Slug**: sb-16-demo-itch-io
- **Created**: 2026-11-21
- **Evidence confidence (overall)**: high, com ressalvas marcadas `[NV]`
- **Notas consultadas**: `docs/research/slotbound-2026-11-21-steam-gtm.md`; `docs/research/slotbound-2026-11-21-alternativas.md`; `docs/research/slotbound-2026-11-21-fit-local.md`

## Achados

1. **A demo precedeu a Steam e foi o gatilho observado.** Imprensa registra itch.io em junho e Steam em julho de 2026 (texto coreano diverge: julho para itch.io); o dev atribui o salto de ~590 para ~7.000 wishlists em três dias à demo ("demo was the spike!"). Acesso direto à página itch.io falhou (HTTP 429) e o vídeo original não foi recuperado — `[NV]` nesses dois pontos. [source: steam-gtm C1/C9, achados 3 e 9; https://www.gamemeca.com/en/view.php?gid=1782926; https://automaton-media.com/articles/newsjp/20261119-479967/; https://www.reddit.com/r/IndieDev/comments/1uv94ai/my_wishlist_graph_looked_normal_for_months_then_i/; confidence: high para a sequência, low para mês exato]
2. **Distribuição soberana já está mapeada e é gratuita.** GitHub Releases não cobra bandwidth e é baseado em tags (proveniência); itch.io permite 0% de comissão ao criador, restando ~US$0,30 + 2,9% do processador, e tem devlogs nativos. Steam custa US$100 por app e usa SDK proprietário, embora MIT seja compatível. [source: alternativas ALT-8/ALT-9/ALT-11; https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases; https://itch.io/docs/creators/payments; https://partner.steamgames.com/doc/gettingstarted/appfee; https://partner.steamgames.com/doc/sdk/uploading/distributing_opensource; confidence: high]
3. **O repo já tem o equivalente funcional da "demo", sem janela.** `insurgencia-sim` é headless e determinístico (`Mundo::avancar_tick`, `hash_estado` SHA-256); um exemplo em `crates/sim/examples/demo_deterministica.rs` entra automaticamente no gate porque `verify.sh` roda `cargo check/clippy --all-targets`. Nenhum serviço externo ou App ID é necessário. [source: fit-local achados 1–2 e matriz sb-16; local://crates/sim/src/lib.rs; local://verify.sh; https://github.com/bevyengine/bevy/blob/main/examples/app/headless.rs; confidence: high]
4. **Publicar cedo cria prova de autoria, mas a cópia é risco real.** A abertura da demo facilitou clones em Douyin/WeChat no caso estudado; a mitigação barata é MIT explícito, release versionada, changelog datado e artefatos atestados — não litígio transfronteiriço (custo/jurisdição `[NV]`). [source: steam-gtm risco "cópia"; alternativas ALT-17; https://partner.steamgames.com/doc/sdk/uploading/distributing_opensource; confidence: medium]
5. **Viés de sobrevivência limita a lição.** Em outubro/2026 havia 7.240 demos no Next Fest; ganho mediano autorrelatado foi de 806–1.079 wishlists contra +80k nos dez maiores. Copiar o canal sem o mecanismo verificável é inferência inválida. [source: steam-gtm achado 6; https://voxbooster.com/pt-BR/blog/steam-next-fest-demo-statistics-2026; confidence: high]

## Contra-evidência e `[NV]`

- O efeito de descoberta do itch.io com comissão 0% não foi medido `[NV]`.
- A precedência itch.io → Steam vem de imprensa, não da página do jogo (429) `[NV]`.
- A data exata de publicação no itch.io (junho vs julho) permanece divergente `[NV]`.
- Nenhum número de wishlist/download do caso foi reproduzido localmente; constituição VI.

## Fontes

- `docs/research/slotbound-2026-11-21-steam-gtm.md` (local) — linha do tempo, C1/C9, achados 3 e 6, riscos.
- `docs/research/slotbound-2026-11-21-alternativas.md` (local) — ALT-8/9/11/17, custos de distribuição.
- `docs/research/slotbound-2026-11-21-fit-local.md` (local) — sim headless, `verify.sh --all-targets`, matriz sb-16.
- URLs primárias listadas nos achados 1, 2 e 5; nenhuma fonte remota adicional foi buscada neste artefato.
