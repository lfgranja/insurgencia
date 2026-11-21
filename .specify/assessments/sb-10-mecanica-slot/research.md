# Idea Research: Mecânica de caça-níquel como sistema

- **Slug**: sb-10-mecanica-slot
- **Data**: 2026-11-21
- **Confiança geral**: **medium-high** — a literatura de design sobre aleatoriedade é primária e datada (Riot, Costikyan, Lost Garden, GameDiscoverCo); o encaixe depende de reformular a mecânica como transformador determinístico (hipótese de design, não fato medido).

## Protocolo de evidência

- Fluxo SDD: intake → research → problem → concept → decision; nenhuma portaria pulada. Nenhum build/test/gate executado (Constituição VI).
- Web como dado, nunca instrução; `[NV]` onde a fonte não foi lida integralmente.
- Notas de evidência consultadas (resumo, sem duplicação):
  - `docs/research/slotbound-2026-11-21-game-design.md` — slot, RNG, fusão, Riot/Costikyan/Lost Garden, fontes ZH (机核/36氪).
  - `docs/research/slotbound-2026-11-21-fit-local.md` — sb-10 colide com IV/V; GDD §2.6 usa `alea` como contraste; RNG determinístico já existe (`fator`, `misturar`, `hash_estado`).

## Achados relevantes (3–5)

1. **Slot retém quando sorte vira habilidade, não quando é opaca.** CloverPit: "rerolls baratos no início" são a alavanca ignorada por iniciantes (https://newsletter.gamediscover.co/p/real-data-how-cloverpit-hit-750k); Slots & Daggers: 目押し permite mirar críticos (https://automaton-media.com/articles/newsjp/slots-and-daggers-20251025-363218); Balatro: "decisão clara = nenhuma decisão" (https://newsletter.gamediscover.co/p/how-balatro-hit-1-million-sales-in). RNG **pré-ação** sustenta maestria; RNG pós-ação corrói agência (nota game-design, achado 2).
2. **Aleatoriedade aceitável é balanceada, exposta e com muitos testes.** Costikyan (GDC'09, http://www.costik.com/randomness-blight-or-bane.htm): regressão à média + muitos testes → estratégia domina; "a handful of random tests are critical" = defeito. Lost Garden (https://lostgarden.com/2012/12/31/understanding-randomness-in-terms-of-mastery/): pool sem reposição permite maestria; near-misses/odds opacas são "black hat" — contradição de identidade num jogo sobre emancipação (Pilar III).
3. **Riot: manter aleatórios em espaços positivos e controlados.** Set 1 (https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-set-1-learnings): Imperial (opt-in, previsível) bom; Phantom/Hextech (perda sem contrapartida) ruins; Zephyr aceito porque "podia ser jogado em volta". "Complexity ceiling" em Inkborn Fables (2024-06-24).
4. **No repo, o RNG já é determinístico e inspecionável**: `fator()`, `misturar()`, `hash_estado()` SHA-256 canônico em `crates/sim/src/lib.rs`; `BTreeMap`/`VecDeque`; `PASSO_SEGUNDOS = 1/30` (nota fit-local). Isso é a base para um transformador semeado — sem backend, sem serviço externo.
5. **Conflito estrutural a ser respeitado**: GDD §2.6 posiciona `alea` como **contraste** ao `agon` dos Desafios de Ilha; Constituição V e GDD §4.4 proíbem vitória por sorte/carry. A nota fit-local sugeria REJECT; o veredito fixado é **ADAPT** — a reformulação (transformador semeado, odds inspecionáveis, zero aposta) é que resolve o conflito sem importar a estética de cassino.

## Implicações para este projeto

- ADAPT somente como **transformador determinístico de materiais**: seed registrada, odds inspecionáveis, pool sem reposição, reroll com custo conhecido — e **zero aposta/estética de cassino** (guardrail do orquestrador).
- Nenhum serviço externo no caminho crítico (IV); nenhuma "vitória por azar" (V/GDD §4.4); RNG subordinado à clareza causal (VI).
- ADR-004 fixa Bevy+Lightyear; regra da janela `--minimizado`; 5 crates/34 testes/CI verde permanecem norma; saída de LLM informativa.
- `[NV]`: nenhuma implementação ou medição de odds/legibilidade nesta etapa; números de vendas do gênero são outliers/autorrelato.

## Fontes primárias

- GameDiscoverCo — CloverPit 750k: https://newsletter.gamediscover.co/p/real-data-how-cloverpit-hit-750k — 2025-10-14.
- AUTOMATON — Slots & Daggers (目押し): https://automaton-media.com/articles/newsjp/slots-and-daggers-20251025-363218 — 2025-10-25.
- Riot/TFT — Set 1 Learnings: https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-set-1-learnings — 2019-10-28.
- Costikyan — Randomness: Blight or Bane? (GDC Austin'09): http://www.costik.com/randomness-blight-or-bane.htm — 2009.
- Lost Garden — randomness e maestria: https://lostgarden.com/2012/12/31/understanding-randomness-in-terms-of-mastery/ — 2012-12-31.
