# Research — Tendência gambling-roguelike (sb-12)

**Data**: 2026-11-21. **Escopo**: ponteiros para as notas de evidência; não repito a pesquisa completa.

## Notas de evidência consultadas
- `docs/research/slotbound-2026-11-21-game-design.md` — gênero, RNG, legibilidade (sb-09–sb-15).
- `docs/research/slotbound-2026-11-21-fit-local.md` — mapeamento ao código e à governança locais.
- `docs/research/slotbound-2026-11-21-ai-promo.md` — origem das afirmações do vídeo.

## Achados relevantes
1. O gênero é comercialmente real, porém concentrado no topo: Balatro >5M (https://www.playstack.com/news/balatro-5-million-copies-sold), CloverPit 750k em 2 semanas (https://newsletter.gamediscover.co/p/real-data-how-cloverpit-hit-750k), enquanto Pip My Dice fez 8.744 cópias em 7 meses (https://www.reddit.com/r/gamedev/comments/1rkndhv/pip_my_dice_post_mortem_7_months_after_release/). "100K em 3 dias" é *outlier*, não base.
2. A onda pós-Balatro tem ciclo curto: 36氪 lista 麻将/骰子/xadrez/パチンコ em ~2 anos (https://eu.36kr.com/zh/p/3433853425913479); a análise da 机核 fala em "pecado original" do componente de apostas (https://www.gcores.com/articles/179323).
3. A retenção vem de converter sorte em habilidade (rerolls baratos, 目押し, "decisão clara = nenhuma decisão"): https://newsletter.gamediscover.co/p/how-balatro-hit-1-million-sales-in ; Costikyan (http://www.costik.com/randomness-blight-or-bane.htm) e Lost Garden (https://lostgarden.com/2012/12/31/understanding-randomness-in-terms-of-mastery/) distinguishing maestria de odds opacas.
4. Fit local: GDD §2.6 já posiciona `alea` como **contraste** ao `agon` dos Desafios de Ilha; constituição IV/V e a nota fit-local identificam conflito direto (dependência externa, "carry", vitória por desempenhos independentes).
5. **[NV]** Vídeo original não assistido; a origem IA da promoção é inferência de Garry Seto, não fato primário. Números de vendas são autorrelato; não há medição dessa "tendência" para o Insurgência.

**[NV]** Nenhum build/teste/gate executado nesta escrita (constituição VI).
