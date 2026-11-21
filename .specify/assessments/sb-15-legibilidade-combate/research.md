# Research — Legibilidade do combate (sb-15)

**Data**: 2026-11-21. **Escopo**: ponteiros para as notas de evidência; não repito a pesquisa completa.

## Notas de evidência consultadas
- `docs/research/slotbound-2026-11-21-game-design.md` — telegrafia, complexidade, signposting (sb-09–sb-15).
- `docs/research/slotbound-2026-11-21-fit-local.md` — seam `Mundo → apresentação` no cliente, pré-requisito `avancar_tick`.

## Achados relevantes
1. Legibilidade é ferramenta de agência, não cosmético: Riot/TFT mediu que efeitos devem ser previsíveis e "jogáveis em volta", e que perder turnos ("I don't get to play") corrói a experiência: https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-set-1-learnings .
2. Complexidade tem teto e precisa de *grok* rápido: "TFT is at its complexity ceiling" (https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-inkborn-fables-learnings); esconder informação gera calculadoras/wikis externas (https://gmtk.substack.com/p/balatros-cursed-design-problem); signposting progressivo funciona como tutorial de baixo custo (https://www.gamedeveloper.com/design/balatro-creator-unveils-a-few-slick-signposting-strategies).
3. Fit local: HUD é `format!` puro em `atualizar_texto` (`crates/cliente/src/main.rs:547-561`), sem assets; `sincronizar_avatar`/`sincronizar_sim` são o adaptador de saída — iterar ali não toca na sim (constituição II).
4. **Pré-requisito crítico**: `Mundo` no cliente é espelho e **não chama `avancar_tick`** (`crates/sim/src/lib.rs:119-124`); hoje "combate" no cliente é vacuo. Qualquer ganho de legibilidade depende de ligar o tick antes de ajustar apresentação.
5. **[NV]** As atualizações da demo citadas no intake não foram verificadas em fonte primária; avaliação visual final fica com a SE, sempre sob `--minimizado`. Nenhum build/teste/gate executado nesta escrita (constituição VI).
