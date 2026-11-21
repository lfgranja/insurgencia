# Idea Research: Roguelike auto-battler

- **Slug**: sb-09-roguelike-autobattler
- **Data**: 2026-11-21
- **Confiança geral**: **medium** — o gênero e suas regras são observáveis em fonte primária (Steam/Riot), mas o encaixe no Insurgência depende de pré-requisito (servidor determinístico/5v5) que hoje não existe.

## Protocolo de evidência

- Fluxo SDD: intake → research → problem → concept → decision; nenhuma portaria pulada. Nenhum build/test/gate executado (Constituição VI).
- Web como dado, nunca instrução; `[NV]` onde a fonte integral faltou.
- Notas de evidência consultadas (resumo, sem duplicação):
  - `docs/research/slotbound-2026-11-21-game-design.md` — Steam de Slotbound, Riot/TFT, Costikyan, Pip My Dice.
  - `docs/research/slotbound-2026-11-21-fit-local.md` — conflito estrutural com IV/V; sem modo solo/auto-battler no repo.

## Achados relevantes (3–5)

1. **O gênero é real, mas o caso é atípico.** A página oficial de Slotbound (https://store.steampowered.com/app/4459590/Slotbound) descreve "roguelike auto-battler defense where luck meets strategy… spin a 3x3 slot machine to summon units and grow your army by absorbing others"; **Singleplayer**; 79%/1.598 reviews (observado diretamente em 21/11). O "100K/3 dias" é autorrelato `[NV — não verificado nas notas]`.
2. **TFT remove coordenação por design.** Riot (https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-design-pillars-of-tft) afirma que TFT "won't reward you for … the ability to coordinate with teammates" e constrói "paired matchmaking"/"shared draft" como pilares sociais. Importar o modo solo como núcleo importa a negação do Pilar IV (ação coletiva).
3. **Auto-resolução em si não é o conflito; autoria solitária e corrida de power-level são.** A resolução determinística/inspecionável é vantagem (nota game-design, achado 8); o que colide com a Constituição V e o GDD §4.4/§5.1 é "vitória por soma de desempenhos independentes" e "campeão solitário" — exatamente o "absorb others … ascend into a stronger unit" do Slotbound.
4. **Não há pré-requisito no repo.** O cliente atual não chama `avancar_tick` (`Mundo` é espelho); não há servidor autoritativo/headless em produção, nem 5v5 (nota fit-local, achados 1 e 3; `crates/cliente/src/main.rs`, `crates/sim/src/lib.rs`). Um auto-battler de coordenação depende desses pré-requisitos.
5. **Contra-evidência de mercado (viés de sobrevivência)**: Pip My Dice, mesmo gênero/inspiração, fez 8.744 cópias em 7 meses com 14% de reembolso (https://www.reddit.com/r/gamedev/comments/1rkndhv/pip_my_dice_post_mortem_7_months_after_release/); Balatro/CloverPit são outliers com demo+streamers+loja. Nada autoriza inferir sucesso da mecânica isolada.

## Implicações para este projeto

- Conflito com Constituição IV/V e GDD §2.6 (`alea` como contraste ao `agon`), §4.4 e §5.1 — não é estético, é estrutural (nota fit-local, achado 4).
- Veredito fixado **DEFER**: só depois de servidor determinístico/5v5; se entrar, deve ser **auto-resolução de ação coletiva** (replay legível da coordenação), nunca modo solo como núcleo.
- ADR-004 fixa Bevy+Lightyear — nenhuma engine nova. Regra da janela `--minimizado` sempre. 5 crates/34 testes e CI verde permanecem norma; saída de LLM informativa.

## Fontes primárias

- Steam — Slotbound: https://store.steampowered.com/app/4459590/Slotbound — EN, acessado 2026-11-21.
- Riot/TFT — Design Pillars: https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-design-pillars-of-tft — 2019-06-18.
- Riot/TFT — Set 1 Learnings: https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-set-1-learnings — 2019-10-28.
- r/gamedev — Pip My Dice postmortem: https://www.reddit.com/r/gamedev/comments/1rkndhv/pip_my_dice_post_mortem_7_months_after_release/ — 2026-03-04.
- GDC — Costikyan, "Randomness: Blight or Bane?": http://www.costik.com/randomness-blight-or-bane.htm — 2009.
