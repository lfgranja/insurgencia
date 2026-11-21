# Pesquisa — Design de roguelike auto-battler, mecânicas de slot/fusão, RNG e legibilidade de combate (propostas sb-09 a sb-15)

**Data:** 2026-11-21. **Idiomas consultados:** EN (primário: Steam, Riot/TFT, GDC, Lost Garden, GameDeveloper, GameDiscoverCo, Reddit), PT-BR (IGN Brasil), JA (AUTOMATON), ZH-汉 (机核 GCORES, 36氪). ES não produziu fonte primária relevante nesta rodada; ZH-Hant apareceu apenas em resultados de busca (巴哈姆特/GSE), não utilizado como evidência principal.

**Escopo:** avaliar as afirmações do vídeo *Slotbound Sold 100K on Steam With 100% AI Code and AI Art. How?* (Garry Seto) como **hipóteses**, não fatos. Esta nota cobre design (sb-09, sb-10, sb-11, sb-12, sb-14, sb-15); vendas/disclosure/GTM sb-16–sb-28 ficam com as notas paralelas. Nenhuma ferramenta de build/lint/teste foi executada; nada aqui é afirmação de verde (Constituição VI).

## Perguntas de pesquisa

1. O gênero "gambling-roguelike" (Balatro, Luck Be a Landlord, Slots & Daggers, CloverPit, "Sagamore"?) tem recepção/vendas verificáveis em fontes primárias? É tendência estrutural ou moda?
2. O que, na literatura de design (GDC, dev blogs, papers), explica a retenção de slot e de fusão — e o que é moda passageira?
3. Como balancear aleatoriedade sem destruir agência (GDC, notas de Balatro, posts de devs/Reddit)?
4. Quais técnicas verificáveis melhoram legibilidade de combate e *juice* em auto-battlers?
5. Auto-battler solo é antitético aos pilares de Insurgência (materialismo, ação coletiva, servidor determinístico, sem campeão solitário)?

## Evidência

| Fonte primária | Idioma | Data | URL sanitizada | Trecho/achado observado |
|:--|:--|:--|:--|:--|
| Steam — página oficial de `Slotbound` (Optima Arch) | EN | acessado 2026-11-21 | https://store.steampowered.com/app/4459590/Slotbound | "A roguelike auto-battler defense where luck meets strategy. Spin a 3x3 slot machine to summon units and grow your army by absorbing others"; "Singleplayer"; "79% of the 1,598 user reviews ... are positive"; disclosure de IA nos visuais; EULA de terceiros; online apenas para leaderboards. |
| Playstack (editora) — comunicado oficial | EN | 2025-01-21 | https://www.playstack.com/news/balatro-5-million-copies-sold | "Balatro has hit an incredible milestone with over 5 million units sold"; pico de interesse após The Game Awards. |
| GameDiscoverCo (entrevista com LocalThunk e Playstack) | EN | 2024-03-18 | https://newsletter.gamediscover.co/p/how-balatro-hit-1-million-sales-in | 1M unidades em <1 mês; demo com 100k downloads em 3 meses; 3 pilares de marketing; EUA 33%, China 26%, Coreia 3% (até 01/03); LocalThunk: "having a clear choice in a given situation is the same as having no choice at all" e usa retenção de 1 h como métrica de hook. |
| TrampolineTales (dev), blog oficial | EN | 2023-01-22 | https://blog.trampolinetales.com/whats-next-for-luck-be-a-landlord | ~150 CCU antes do 1.0, ~1.000 CCU após; 30× vendas da semana de lançamento vs. out/2022; >75% das impressões vieram de "New & Trending"; 23 hotfixes; "don't foresee myself making any balance changes"; modding API planejada. |
| GameDiscoverCo — dados diretos do back-end Steam + devs | EN | 2025-10-14 | https://newsletter.gamediscover.co/p/real-data-how-cloverpit-hit-750k | CloverPit: 750k unidades em 2 semanas (relato dos devs); reembolso 6,4%; vendas US 29%, DE 10%, JA 9%, KR 9%, CN 6%; developer: "the more experience the player has, the easier it is to manipulate the odds. Cheap rerolls of the store in the early game is one example"; explode o jogo → conteúdo viral. |
| AUTOMATON (JP) — cobertura de Slots & Daggers | JA | 2025-10-25 | https://automaton-media.com/articles/newsjp/slots-and-daggers-20251025-363218 | ~250 reviews, 94% positivas no momento; 4–8 h; "目押し" (parar os rolos) permite mirar críticos — habilidade suporta o RNG; progressão meta com "chips"; elogios a som/UI. |
| Game Maker's Toolkit (Mark Brown) | EN | 2024-04-02 | https://gmtk.substack.com/p/balatros-cursed-design-problem | Balatro não mostra preview de pontos; "Rube Goldberg machine"; números sobem com efeitos sonoros crescentes; deck peek foi adicionado após playtests mostrarem que jogadores rastreiam cartas "even though it really wasn't much fun"; risco de calculadoras externas. |
| GameDeveloper — LocalThunk sobre signposting | EN | 2024-07-17 | https://www.gamedeveloper.com/design/balatro-creator-unveils-a-few-slick-signposting-strategies | Carta "Four Fingers" no demo = "huge teachable moment"; "primeiro momento em que entenderam o hook"; método descrito como "extended low-calorie tutorial". |
| Riot/TFT — /dev: Design Pillars of TFT | EN | 2019-06-18 | https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-design-pillars-of-tft | Pilares: mastery, playful competition, discovery; "TFT won't reward you for ... the ability to coordinate with teammates"; "paired matchmaking" e "shared draft" são centrais — a camada social é **deliberadamente** projetada. |
| Riot/TFT — /dev: TFT Set 1 Learnings | EN | 2019-10-28 | https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-set-1-learnings | "keep random effects in positive and controlled spaces"; Imperial (opt-in, previsível) bom vs. Phantom/Hextech (perda de itens/HP sem contrapartida) ruins; "I don't get to play" com many disables; Zephyr bom porque "predictable and could be played around"; menos hard counters; B-patches gigantes desestimulam investimento. |
| Riot/TFT — /Dev TFT: Inkborn Fables Learnings | EN | 2024-06-24 | https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-inkborn-fables-learnings | Encounters geraram variação, mas "lack of control was pretty unappealing" para otimizadores; "TFT is at its complexity ceiling"; "players be able to look at a champion, and quickly grok what they should do"; mecânica de set não é para manter. |
| GDC Austin '09 — Greg Costikyan, "Randomness: Blight or Bane?" | EN | 2009 | http://www.costik.com/randomness-blight-or-bane.htm | Regressão à média: muitos testes aleatórios + estratégia → estratégia domina; randomizar com (a) balanceamento entre opções, (b) exposição igual de todos aos mesmos riscos, (c) muitos testes; "a handful of random tests are critical" = defeito (ex.: monarcas 9-9-9). |
| Lost Garden (Daniel Cook) | EN | 2012-12-31 | https://lostgarden.com/2012/12/31/understanding-randomness-in-terms-of-mastery/ | Aleatoriedade como *rules noise*; amostragem tem custo; "pull random outcomes from a fixed pool" (sem reposição) permite maestria; "black hat": near-misses, odds opacas, alta variabilidade para impedir aprendizado. |
| GDC Europe 2012 — Jonasson & Purho, "Juice It or Lose It" (abstract) | EN | 2012 | https://www.gdcvault.com/play/1016487/Juice-it-or-Lose-it | "neat little tricks you can apply to any game to make it more satisfying to play"; partículas/som; **conteúdo do vídeo [NV — não assistido]; apenas abstract lido**. |
| GamesIndustry.biz — GDC 2023 Designer's Notebook (Big Fish/EverMerge) | EN | 2023-03-23 | https://www.gamesindustry.biz/merge-games-as-a-fulfillment-of-the-american-dream | "16,000 merges to get to the 14th level golden tea service"; fusão vende fantasia rags-to-riches; crítica: "it looks like a hoarder lives here ... This is *so* stressful" → contraponto Marie Kondo (ordem, não acumulação). |
| gamedesign.consulting — deconstrução do merge | EN | 2020-12-04 | https://gamedesign.consulting/at-the-frontier-of-merge-games/ | Atrativos: organizar (ordem do caos), crescer (números maiores), háptico (drag & drop); "Feedback for merging needs to be exceptional ... Players will see this hundreds of times"; "merge ... quite easy to build, so your innovation has to go somewhere and is prone to being copied"; precisa de purpose. |
| AppMagic via Game World Observer | EN | 2022-07-25 | https://gameworldobserver.com/2022/07/25/merge-games-trends-analysis-worth-trying-or-not-appmagic | Merge mobile: D1 50–60% (até 75% em testes); Merge-3/5 "approaching its dead-end"; 3 líderes = 99% da receita; custo típico 15–20 pessoas/ano. **Métricas de F2P mobile, não transferíveis diretamente a premium PC.** |
| 机核 GCORES — análise ZH de Balatro | ZH-汉 | 2024-03-22 | https://www.gcores.com/articles/179323 | "赌徒心理" (sunk cost, loss aversion); rodada ~45 s, replay sem confirmação; BGM contínua "como cassino"; UI/UE动效 com consistência rítmica; conclui que o jogo tem "原罪" pelo componente de apostas. |
| 36氪 / 游戏新知 — novo framework pós-Balatro | ZH-汉 | 2025-08-22 | https://eu.36kr.com/zh/p/3433853425913479 | "Roguelike玩法便出现了一种新框架": 麻将/骰子/xadrez/パチンコ; 推币机 (coin pusher) 国产 《浣熊推币机》 com 50.000 wishlists em uma semana; "数值膨胀" e "数值可视化"; remove os dark slots e adiciona "垂死摇晃" (salvamento) — anti-frustração desenhada. |
| IGN Brasil | PT-BR | 2024-12-11 | https://br.ign.com/balatro/133580/news/indie-do-ano-balatro-ja-vendeu-35-milhoes-de-copias-mundialmente-revela-estudio | 500k em 2 semanas; 2M em agosto/2024; 3,5M em dez/2024; €4,2M só em mobile (AppMagic); Metacritic 90. |
| r/gamedev — postmortem "Pip My Dice" (dev) | EN | 2026-03-04 | https://www.reddit.com/r/gamedev/comments/1rkndhv/pip_my_dice_post_mortem_7_months_after_release/ | Inspirado no demo de Balatro; 8.744 cópias em 7 meses, 14% reembolsadas, ~US$65k brutos Steam, 126 reviews/88%; streamer influente não gostou porque comparava com Balatro. Contra-exemplo de sobrevivência. |
| "Sagamore" | — | — | — | **[NV — não localizado]** em buscas EN/JA/ZH/PT nesta data; provável confusão com Slots & Daggers/CloverPit. Não usar como evidência. |
| Theseus — *Balancing Randomness in Roguelike Game Design* (Hannula & Rama) | EN | ~2025 | https://www.theseus.fi/bitstream/handle/10024/886802/Hannula_Rama.pdf | **[NV — acesso falhou: HTTP 403]**; título e trecho parcial vistos apenas em resultado de busca. Substituído por Costikyan/Lost Garden/TFT. |

## Achados

1. **O gênero é comercialmente real, mas os números são altamente assimétricos e concentrados no topo.** Balatro: 1M em <1 mês (mar/2024) → 3,5M (dez/2024) → >5M (jan/2025, fonte da editora). CloverPit: 750k em 2 semanas. Slots & Daggers: 94% de reviews positivas (~250) em 25/10/2025; manchete de 100k em 10 dias é **secundária** (dev citado pela imprensa). Já o-postmortem típico (Pip My Dice) é 8.744 cópias/7 meses. Ou seja: "100K" é o *outlier* visível, não a base. **Nenhum desses números prova que a mecânica, sozinha, cause sucesso** — todos os casos de sucesso tinham demo, streamers e/ou posicionamento na loja.

2. **A mecânica de slot retém quando o jogador pode converter sorte em habilidade, não quando a sorte é opaca.** Casos concretos: CloverPit — "rerolls baratos no início" são o alavanca ignorada por iniciantes; Slots & Daggers —— 目押し permite mirar críticos; Balatro —— "decisão clara = nenhuma decisão". Costikyan: regressão à média + muitos testes; Lost Garden: pool sem reposição e custo de amostragem. RNG *pós-ação* (rolar depois de decidir) corrói agência; RNG *pré-ação* (amostra antes da decisão) sustenta maestria.

3. **Fusão retém por ordem, crescimento e tapete háptico — não por "números maiores" sozinhos.** O merge clássico vende "do lixo ao ouro" (16.000 merges para o nível 14), mas o próprio palestrante da GDC chama o tabuleiro de "hoarder" e estressante. O que sobrevive como princípio é: feedback excepcional repetido centenas de vezes, propósito fora da mecânica, e caminhos curtos + longos. AppMagic mostra o lado sombrio: subgênero saturado, 3 líderes com 99% da receita, "dead-end" para newcomings — e são métricas de *mobile F2P*, incompatíveis com a soberania do jogador e com o modelo sem serviço externo de Insurgência.

4. **A onda "pós-Balatro" é rápida e já é lida como框架 (framework) na China.** 36氪 lista 麻将/骰子/xadrez/パチンコ/coin-pusher em menos de 2 anos, com 50k wishlists em uma semana para um pusher chinês. Isso é evidência de *moda com ciclo curto de replicação* — e também de que a inovação está na **interface entre domínios** (materialismo de Insurgência pode ser esse domínio diferente), não no caça-níquel em si.

5. **Legibilidade não é "menos RNG": é controle, previsibilidade e custo de processamento.** Riot mediu explicitamente: efeitos bons são *opt-in e previsíveis* (Imperial); ruins retiram agência (Phantom/Hextech); Zephyr, embora desative 6 s, era aceito porque "podia ser jogado em volta". Em Inkborn Fables, variação alta gerou desconforto em otimizadores e o jogo bateu no "complexity ceiling". Mark Brown: esconder informação gera workarounds externos (calculadoras, wikis). Conclusão: legibilidade é *ferramenta de agência*, não cosmético.

6. **Juice é feedback proporcional e consistente, não volume.** Balatro coreografa cada carta/joker entrando, números subindo e multiplicador pegando fogo; GCORES descreve a mesma função na continuidade da BGM e na consistência rítmica do UI motion; GDC "Juice It or Lose It" é a referência canônica (abstract verificado; vídeo **[NV]**). Juice serve à legibilidade quando sinaliza *causa*; quando esconde causa, vira ruído (Lost Garden: *feedback noise*).

7. **O auto-battler solo não é "neutro" em relação a Insurgência.** A própria Riot define que TFT *não* recompensa coordenação com companheiros e constrói "paired matchmaking"/"shared draft" como pilares sociais. Importar o modo solo como núcleo importa, junto, a negação do Pilar IV (ação coletiva) e o risco de "campeão solitário" (Pilar V/Constituição V) via fusão de unidades — exatamente o que Slotbound faz ("absorb others ... ascend into a stronger unit"). O conflito não é com a *auto-resolução* (determinista, inspecionável), é com a *autoria solitária e a corrida power-level*.

8. **Determinismo é vantagem competitiva aqui, não restrição.** Um auto-battler resolvido por semente + traço de simulação permite auditar "o RNG foi justo?", exportar replay e comparar builds — o oposto da opacidade que a literatura condena. Ainda assim, **servidor autoritativo/headless e 5v5 não existem no alvo**, então qualquer forma coletiva depende de pré-requisito.

## Implicações para Insurgência (ID → impacto → decisão)

| ID | Impacto observável | Conflito constitucional / gate | Decisão sugerida (1 linha) |
|:--|:--|:--|:--|
| **sb-09** roguelike auto-battler | Auto-resolução de escaramuças após fase de planejamento coletivo; útil como *replay* legível de coordenação. | Pilar IV e Constituição V: TFT deliberadamente remove coordenação; no alvo não há servidor/5v5 → depende de pré-requisito. | **DEFER** — só depois de servidor determinístico/5v5; se entrar, deve ser auto-resolução de ação coletiva, nunca modo solo como núcleo. |
| **sb-10** mecânica de slot | Gerador de recursos com odds explícitas, semente auditável, reroll com custo conhecido e pool sem reposição. | Pilar I/IV e Constituição I/IV: rule opaca = dependência de autoridade; near-miss/odds escondidas são "black hat" (Lost Garden). | **ADAPT** — converter em transformador determinístico de materiais com seed/odds inspecionáveis e zero aposta; não importar a estética de cassino. |
| **sb-11** fusão | Fundir materiais/supply/funções em produto de tier superior; ótimo para cadeias de ilha e feedback tátil. | Constituição V: fundir *unidades* cria hiper-unidade e substitui coordenação (Slotbound faz isso); Pilar II exige ação material. | **ADAPT** — fundir apenas materiais e papéis complementares, com teto de poder por ator e registro auditável; proibir "superunidade". |
| **sb-12** tendência gambling-roguelike | Onda pós-Balatro é rápida (ZH: 4 domínios em ~2 anos, pusher com 50k WL/semana) e concentrada no topo. | Pilar II (materialismo) e IV: identidade de "caça-níquel" desloca o produto de coordenação para sorte/espetáculo. | **AVOID** — não perseguir rótulo/estética; manter como dados de mercado apenas, sem copiar o ciclo de moda. |
| **sb-14** RNG/juice/balance acima da fidelidade artística | Maior retorno vem de RNG pré-ação controlado + feedback de causa; "RNG acima de tudo" é falsificado por TFT e Balatro. | Constituição I (determinismo) e VI (verificação honesta): RNG obscuro e "juice" sem telegrafia impedem auditoria. | **ADAPT** — substituir "mais RNG" por RNG semeado, positivo e amostrável; juice subordinado à clareza causal e medido por teste, não por opinião. |
| **sb-15** legibilidade de combate / iteração | Telegrafia, evitar "I don't get to play", desabilitados previsíveis, *signposting* progressivo, reduzir teto de complexidade, feedback por causa. | Compatível; validação deve respeitar regra SE (SEMPRE `--minimizado`; nunca abrir janela normal por iniciativa de agente). | **ADOPT** — adotar como critério de aceitação de UI/HUD, capturando evidência reproduzível sob `--minimizado`. |

**Precedência em conflito (GDD §4: I → III → II → IV):** sb-10 e sb-11 passam pelo filtro de soberania/materialismo antes de virar feature; sb-09 esbarra em IV e fica preservado até existir coordinateção real no Núcleo.

## Riscos, contra-evidência e [NV]

- **Viés de sobrevivência (contra os 100K):** 3,5M/5M de Balatro e 750k de CloverPit são outliers; Pip My Dice (mesmo gênero, mesma inspiração) fez 8.744 cópias com 14% de reembolso e um streamer influente comparou desfavoravelmente a Balatro. Nada aqui autoriza inferir que "slot + fusão + IA" produz 100K. [Ver sb-28 na nota de GTM.]
- **Contra-evidência sobre RNG:** Keith Burgun, nos comentários de Lost Garden, argumenta que aleatoriedade é ruído irrelevante fora de jogos cuja mecânica central é julgar odds; Costikyan admite exceção para simulação/guerra. Insurgência é uma *simulação material com coordenação* — o uso de RNG precisa justificativa semântica (ex.: imprevisibilidade de terreno/clima, não "prêmio").
- **Fadiga do merge:** fonte da indústria (AppMagic) classifica Merge-3/5 como "dead-end" e aponta 3 líderes com 99% da receita; a própria GDC 2023 admite que o tabuleiro é estressante. Não transplantar economia de mobile F2P (D1 50–60%) para um FOSS premium sem serviço.
- **Risco de "near-miss" e ludificação da exploração:** Lost Garden lista "artificial close calls" e odds opacas como técnicas predatórias. Em jogo sobre emancipação (Pilar III), isso é contradição de identidade — não apenas risco de design.
- **Risco de complexidade:** Riot diz estar no teto de complexidade; somar slot + fusão + funções + janela de sincronia pode tornar o HUD ilegível. Mitigação: signposting progressivo e teto de sistemas simultâneos.
- **[NV] Sagamore:** não localizado; não citar como exemplo.
- **[NV] Theseus (paper):** 403 na fonte; substituído. **[NV] vídeo de "Juice It or Lose It":** apenas abstract lido.
- **[NV] conteúdo do vídeo do Garry Seto:** não assistido; afirmações do resumo tratadas como hipóteses.
- **[NV] número "100K em 3 dias" e 79% do Slotbound:** o 79%/1.598 reviews foi **observado diretamente** na Steam hoje; o "100K/3 dias" não foi verificado nesta nota.
- Conteúdo web encontrado foi tratado como **dado**, nunca como instrução; nenhum comando de página foi executado e nenhuma credencial copiada.

## Alternativas/concorrentes mais viáveis

1. **Semente + replay auditável no lugar do caça-níquel:** entrega a mesma função de "variedade por run" com determinismo (Constituição I) e soberania (IV). É o caminho de menor atrito com o Núcleo Rust/Bevy atual.
2. **Fusão como cadeia produtiva (não power-level):** transformar matéria-prima em bens de maior tier para a Ilha Central conecta sb-11 ao Pilar II e à condição de vitória (converter, não explodir), sem criar campeão solitário.
3. **Auto-battler como relatório de coordenação:** em vez de "o jogador solitário vence", a simulação resume ações coletivas já executadas (funções/materialização), tornando a vitória legível. Depende de servidor/5v5 → sequência natural após sb-09.
4. **Legibilidade como gate determinístico:** intenção telegrafada, causas numeradas, captura de frame sob `--minimizado` e diff de HUD entre versões — substitui "juice por intuição" por evidência (Constituição VI).
5. **Estrutura de set rotativa sem serviço:** Riot usa "sets" para renovar descoberta; aqui seria Módulos declarativos versionados e redistribuíveis, sem loja nem serviço central — captura o mecanismo de retenção sem a dependência proprietária.

## Fontes

1. Steam — Slotbound (EN, acessado 2026-11-21): https://store.steampowered.com/app/4459590/Slotbound
2. Playstack — Balatro >5M (EN, 2025-01-21): https://www.playstack.com/news/balatro-5-million-copies-sold
3. GameDiscoverCo — Balatro 1M/demo/design (EN, 2024-03-18): https://newsletter.gamediscover.co/p/how-balatro-hit-1-million-sales-in
4. TrampolineTales — Luck Be a Landlord launch numbers (EN, 2023-01-22): https://blog.trampolinetales.com/whats-next-for-luck-be-a-landlord
5. GameDiscoverCo — CloverPit 750k (EN, 2025-10-14): https://newsletter.gamediscover.co/p/real-data-how-cloverpit-hit-750k
6. AUTOMATON — スロット＆ダガー (JA, 2025-10-25): https://automaton-media.com/articles/newsjp/slots-and-daggers-20251025-363218
7. GMTK — Balatro's 'Cursed' Design Problem (EN, 2024-04-02): https://gmtk.substack.com/p/balatros-cursed-design-problem
8. GameDeveloper — LocalThunk signposting (EN, 2024-07-17): https://www.gamedeveloper.com/design/balatro-creator-unveils-a-few-slick-signposting-strategies
9. Riot/TFT — Design Pillars (EN, 2019-06-18): https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-design-pillars-of-tft
10. Riot/TFT — Set 1 Learnings (EN, 2019-10-28): https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-set-1-learnings
11. Riot/TFT — Inkborn Fables Learnings (EN, 2024-06-24): https://teamfighttactics.leagueoflegends.com/en-us/news/dev/dev-tft-inkborn-fables-learnings
12. Costikyan — Randomness: Blight or Bane? GDC Austin '09 (EN, 2009): http://www.costik.com/randomness-blight-or-bane.htm
13. Lost Garden — Understanding randomness in terms of mastery (EN, 2012-12-31): https://lostgarden.com/2012/12/31/understanding-randomness-in-terms-of-mastery/
14. GDC Vault — Juice It or Lose It (EN, 2012; abstract): https://www.gdcvault.com/play/1016487/Juice-it-or-Lose-it
15. GamesIndustry.biz — Merge games, GDC 2023 (EN, 2023-03-23): https://www.gamesindustry.biz/merge-games-as-a-fulfillment-of-the-american-dream
16. gamedesign.consulting — The Frontier of Merge Games (EN, 2020-12-04): https://gamedesign.consulting/at-the-frontier-of-merge-games/
17. AppMagic via Game World Observer — Merge trends (EN, 2022-07-25): https://gameworldobserver.com/2022/07/25/merge-games-trends-analysis-worth-trying-or-not-appmagic
18. 机核 GCORES — 浅谈《Balatro》小丑牌的游戏设计与rogue元素 (ZH, 2024-03-22): https://www.gcores.com/articles/179323
19. 36氪/游戏新知 — 被《小丑牌》发行商相中… (ZH, 2025-08-22): https://eu.36kr.com/zh/p/3433853425913479
20. IGN Brasil — Balatro 3,5M (PT-BR, 2024-12-11): https://br.ign.com/balatro/133580/news/indie-do-ano-balatro-ja-vendeu-35-milhoes-de-copias-mundialmente-revela-estudio
21. r/gamedev — Pip My Dice postmortem (EN, 2026-03-04): https://www.reddit.com/r/gamedev/comments/1rkndhv/pip_my_dice_post_mortem_7_months_after_release/
