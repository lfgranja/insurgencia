# Research — RNG, juice e balance (sb-14)

**Data**: 2026-11-21. **Escopo**: ponteiros para as notas de evidência; não repito a pesquisa completa.

## Notas de evidência consultadas
- `docs/research/slotbound-2026-11-21-game-design.md` — RNG, maestria, juice e legibilidade (sb-09–sb-15).
- `docs/research/slotbound-2026-11-21-fit-local.md` — `fator()`/`misturar()`/`hash_estado()` no `sim`, gate local.

## Achados relevantes
1. RNG bom é positivo, opt-in e previsível: Riot/TFT Set 1 separou Imperial (bom) de Phantom/Hextech (perda de agência) e Zephyr (aceito por ser "jogável em volta"): https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-set-1-learnings ; teto de complexidade em https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-inkborn-fables-learnings.
2. Maestria exige amostragem com custo e pool sem reposição; odds opacas e *near-misses* são "black hat": Costikyan (http://www.costik.com/randomness-blight-or-bane.htm), Lost Garden (https://lostgarden.com/2012/12/31/understanding-randomness-in-terms-of-mastery/). CloverPit mostra o análogo comercial: rerolls baratos no início (https://newsletter.gamediscover.co/p/real-data-how-cloverpit-hit-750k).
3. Juice é feedback de causa, não volume: GMTK sobre números e multiplicador em Balatro (https://gmtk.substack.com/p/balatros-cursed-design-problem) e o abstract de "Juice It or Lose It" (https://www.gdcvault.com/play/1016487/Juice-it-or-Lose-it) — vídeo **[NV, não assistido]**.
4. Fit local: RNG determinístico **já existe** (`fator`, `misturar`, `hash_estado` SHA-256, `PASSO_SEGUNDOS = 1/30` em `crates/sim/src/lib.rs`); o trabalho é expor e medir legibilidade, não criar sistema novo.
5. **[NV]** A afirmação do intake ("críticas negativas focam em RNG") não foi confirmada em avaliações primárias; a nota game-design registra apenas 79%/1.598 reviews positivas no Slotbound. Nenhum build/teste/gate executado nesta escrita (constituição VI).
