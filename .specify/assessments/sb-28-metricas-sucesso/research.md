# Research — Métricas de sucesso (100K/3 dias)

**Data**: 2026-11-21
**Notas-base**: `docs/research/slotbound-2026-11-21-ai-code.md` (E3/E4/E5/E15/E16, achado 7), `docs/research/slotbound-2026-11-21-fit-local.md` (sb-28 = anti-métrica; GDD §17).

## Achados
1. O que é observável hoje: 27.374 jogadores simultâneos em 21/11 (Steam Web API) e 79% de 1.598 avaliações positivas na loja; não são histórico de vendas (https://api.steampowered.com/ISteamUserStats/GetNumberOfCurrentPlayers/v1/?appid=4459590; https://store.steampowered.com/app/4459590/Slotbound/).
2. 100K cópias/3 dias e US$ 356K/48h são autodeclarados via X e reportados por terceiros [NV — sem acesso direto ao post]; análise independente diz "We can't verify the 100% figure" (https://quickmatch.si/blog/slotbound-ai-coded-steam-hit/).
3. Volatilidade: avaliações caíram de 87% (239, em 14/11) para 78–79% (986–1.598) em poucos dias, com nove atualizações pós-lançamento (E5).
4. Viés de sobrevivência: censo de 53.597 jogos — topo 1% ≈94% da receita, mediana ~US$ 300, 28% sem avaliações, jogos com IA atingem ≥100 avaliações a ~55% da taxa dos sem IA; 87% não-performers (https://fragwyz.substack.com/p/three-years-of-ai-on-steam; https://automaton-media.com/articles/newsjp/20260729-457268).
5. Local: GDD §17 mede verificação da tese (soberania), não 100K/3 dias; métricas locais falsificáveis: tempo até gate verde, invariantes verificados, determinismo reproduzível em headless (local: `fit-local.md`, `verify.sh`, `crates/sim/src/lib.rs`). [NV — gates não executados nesta análise]

## URLs primárias
- https://api.steampowered.com/ISteamUserStats/GetNumberOfCurrentPlayers/v1/?appid=4459590 (JSON, 21/11/2026)
- https://store.steampowered.com/app/4459590/Slotbound/ (loja, acesso 21/11/2026)
- https://quickmatch.si/blog/slotbound-ai-coded-steam-hit/ (17/11/2026)
- https://fragwyz.substack.com/p/three-years-of-ai-on-steam (censo, 13/07/2026)
