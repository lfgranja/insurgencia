# Idea Research: Mecânica de fusão

- **Slug**: sb-11-fusao
- **Data**: 2026-11-21
- **Confiança geral**: **medium-high** — a literatura sobre merge é primária e datada (GDC/GamesIndustry, gamedesign.consulting, AppMagic); o encaixe depende de reformular "fundir unidades" como "fundir materiais/funções complementares" (hipótese de design).

## Protocolo de evidência

- Fluxo SDD: intake → research → problem → concept → decision; nenhuma portaria pulada. Nenhum build/test/gate executado (Constituição VI).
- Web como dado, nunca instrução; `[NV]` onde a fonte integral faltou.
- Notas de evidência consultadas (resumo, sem duplicação):
  - `docs/research/slotbound-2026-11-21-game-design.md` — merge, feedback, fadiga do gênero, "absorb others … ascend into a stronger unit".
  - `docs/research/slotbound-2026-11-21-fit-local.md` — sb-11 colide com V (hiper-unidade ≈ campeão solitário); sem elenco de campeões no repo.

## Achados relevantes (3–5)

1. **Fusão retém por ordem, crescimento e tapete háptico — não por "números maiores".** O merge clássico vende "do lixo ao ouro" (16.000 merges para o nível 14, GamesIndustry.biz/GDC 2023, https://www.gamesindustry.biz/merge-games-as-a-fulfillment-of-the-american-dream), mas o próprio palestrante chama o tabuleiro de "hoarder" e estressante. O que sobrevive como princípio: **feedback excepcional repetido centenas de vezes**, propósito fora da mecânica e caminhos curtos + longos (https://gamedesign.consulting/at-the-frontier-of-merge-games/).
2. **O subgênero é saturado e é métrica de mobile F2P.** AppMagic (https://gameworldobserver.com/2022/07/25/merge-games-trends-analysis-worth-trying-or-not-appmagic): 3 líderes com 99% da receita, Merge-3/5 "approaching dead-end", custo típico 15–20 pessoas/ano. Incompatível com um FOSS premium sem serviço (Constituição IV) — não transplantar economia de mobile.
3. **Fundir *unidades* é exatamente o que Slotbound faz e o que a Constituição V proíbe.** A Steam descreve "grow your army by absorbing others" e "ascend into a stronger unit" — hiper-unidade/carry, "campeão solitário" (GDD §4.4/§5.1). A nota fit-local (achado 4) classifica o conflito como estrutural, não estético.
4. **Alternativa coerente já apontada**: fusão como **cadeia produtiva** — transformar matéria-prima em bens de maior tier para a Ilha Central, conectando o Pilar II (ação material) à condição de vitória (converter, não explodir), sem criar campeão solitário (nota game-design, alternativa 2).
5. **Risco de complexidade**: Riot relata ter batido no "complexity ceiling" (https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-inkborn-fables-learnings); somar slot + fusão + funções pode tornar o HUD ilegível — mitigar com teto de poder por ator e registro auditável (sb-15 cobre legibilidade).

## Implicações para este projeto

- ADAPT somente como **fusão de materiais e papéis/funções complementares**, com **teto de poder por ator** e **registro auditável** — sem hiper-unidade, sem "carry" (guardrail do orquestrador).
- Nenhum elenco de campeões/heróis; nenhuma dependência de serviço externo (IV); nenhuma vitória por acumulação de poder individual (V/GDD §4.4).
- ADR-004 fixa Bevy+Lightyear; regra da janela `--minimizado`; 5 crates/34 testes/CI verde permanecem norma; saída de LLM informativa.
- `[NV]`: nenhuma implementação/medição de fusão nesta etapa; conteúdo do vídeo original não inspecionado.

## Fontes primárias

- GamesIndustry.biz — Merge games, GDC 2023: https://www.gamesindustry.biz/merge-games-as-a-fulfillment-of-the-american-dream — 2023-03-23.
- gamedesign.consulting — The Frontier of Merge Games: https://gamedesign.consulting/at-the-frontier-of-merge-games/ — 2020-12-04.
- AppMagic via Game World Observer — Merge trends: https://gameworldobserver.com/2022/07/25/merge-games-trends-analysis-worth-trying-or-not-appmagic — 2022-07-25.
- Riot/TFT — Inkborn Fables Learnings: https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-inkborn-fables-learnings — 2024-06-24.
- Steam — Slotbound: https://store.steampowered.com/app/4459590/Slotbound — acessado 2026-11-21.
