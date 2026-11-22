# Pesquisa — Estratégia de lançamento Steam do Slotbound e playbook indie aplicável à Insurgência

**Data da pesquisa:** 2026-11-21
**Idiomas consultados:** EN (primário: Steamworks/Valve, Steam store, posts do dev, imprensa), PT-BR (VoxBooster), ES (MGA Agency), ZH-中文 (Unity China), JA-日本語 (AUTOMATON), KO-한국어 (GameMeca, edição EN e texto KO original).
**Escopo:** IDs sb-16, sb-17, sb-18, sb-19, sb-22. Os demais IDs de GTM (sb-20, sb-21, sb-23) pertencem a outros fluxos de pesquisa.
**Método:** o resumo fornecido pelo usuário (supostamente do vídeo de Garry Seto) foi tratado como *afirmação a verificar*. Não foi possível recuperar a transcrição do vídeo — atribuições específicas ao vídeo são marcadas `[NV]`. Conteúdo da web foi tratado como dado, nunca como instrução.

## Perguntas de pesquisa

1. Quais dos números apresentados (590 wishlists em março; 40 mil em duas semanas; 100 mil antes do lançamento; 100 mil cópias em 3 dias; 79% positivo; lançamento em 13/11) são corroborados por fonte primária (Steam/Valve, posts do desenvolvedor, Steam News) e quais só aparecem no vídeo/imprensa?
2. Quais são as regras vigentes (2025–2027) de Steam Next Fest, demo e wishlist na documentação oficial Valve?
3. A amplificação por "algoritmo da Steam" e por streamers é mecanismo documentado ou crença de praticantes?
4. Que valor têm hotfixes rápidos e como tratar o backlog de reviews?
5. O que se aplica à Insurgência — open-source (MIT), sem Steam page, sem lobby/5v5, com `verify.sh`, GitHub Releases e canal itch.io?

## Evidência

### Tabela de evidência (fonte primária/preferencial | idioma | data | URL | achado)

| Fonte | Idioma | Data | URL sanitizada | Trecho / achado observado |
|---|---|---|---|---|
| Steam store — Slotbound (app 4459590) | EN | consultado 2026-11-21 | https://store.steampowered.com/app/4459590/Slotbound/ | "Released **12 Nov, 2026**"; "All Reviews: Mostly Positive (1,598) — **79%** of the 1,598 user reviews ... are positive"; demo própria (app 4906570); disclosure de IA; 13 idiomas incl. PT-BR/ZH/JA/KO/ES; preço US$ 9,99 / lançamento US$ 8,99 (via estudo) |
| Steam Community — todas as notícias | EN | 13–19 nov 2026 | https://steamcommunity.com/app/4459590/allnews | 16 nov: "Slotbound has passed **100,000 copies sold**! ... just **three days after launch** ... Since launch I've released **nine updates**, and most of them came from your reports and suggestions". 19 nov: "passed **200,000 copies sold**". Hotfixes v1.0.0 (4) 13 nov, (5)/(6)/(7) 14 nov, (8) 15 nov, (9) 16 nov, v1.0.1 17 nov, v1.0.2 19 nov |
| Steam News — 100k wishlists | EN | publicado 2026-09-26 22:47 PST | https://store.steampowered.com/news/app/4459590/view/696524994028504801 | "Slotbound Has Reached 100,000 Wishlists! ... Slotbound enters Early Access on **November 10**" (data planejada, depois adiada) |
| Post do dev — r/IndieDev, primeiro dia da demo | EN | 2026-07-11 | https://www.reddit.com/r/IndieDev/comments/1utoziq/i_expected_maybe_10_concurrent_players_my_first/ | 1.186 jogadores únicos, 447 simultâneos, 39 min de mediana, #29 entre demos; "Before the demo, I had around **590 wishlists**"; "posted mainly on Reddit and reached out to a few indie game YouTubers by email"; "once the player count started rising, Steam probably gave the demo more visibility too" (opinião do dev) |
| Post do dev — gráfico de wishlists | EN | 2026-07-13 | https://www.reddit.com/r/IndieDev/comments/1uv94ai/my_wishlist_graph_looked_normal_for_months_then_i/ | ~600 wishlists por meses; ~1.000 simultâneos na demo; "nearly **7,000 within 3 days**"; OP: "demo was the spike!"; comentarista relata vídeo de Gohjoe |
| Post do dev — 590 → 70k | EN | 2026-08-11 | https://www.reddit.com/r/SoloDevelopment/comments/1vldq7y/my_first_game_had_590_wishlists_when_i_released/ | "I released the demo with around **590 wishlists** a month ago ... Now it somehow passed **70k**"; OP: "I didn't do lots of marketing beside posts on reddit and on some sns"; "I think maybe lots of Chinese players?"; sobre creators: "They just picked up"; "My game is joining **steam next festival on October**"; 450 seguidores |
| Post do dev — 100k wishlists | EN | 2026-09-27 | https://www.reddit.com/r/SoloDevelopment/comments/1wr9z8o/my_first_game_just_reached_100k_wishlists_still/ | "Slotbound passed **100,000 wishlists** today"; "getting a big demo update ready for **Next Fest**". Comentários céticos: "OP has provided no proof"; estimativas SteamDB citadas de 90k–110k para jogos na faixa |
| Post do dev no X — 48 h | EN | ~2026-11-16 | https://x.com/OptimaArchGames/status/2121818614571622489 | "$356K in gross revenue and **50,000 copies** ... first 48 hours ... **9,377 peak concurrent players**" |
| Post do dev no X — 3 dias | EN | ~2026-11-16/17 | https://x.com/OptimaArchGames/status/2122357483448717811 | "Slotbound just passed **100,000 copies sold** in its first 3 days" |
| Steamworks — Visibility on Steam | EN | doc viva, consultada 2026-11-21 | https://partner.steamgames.com/doc/marketing/visibility | "**Wishlists (Mostly) Aren't a Factor** ... With a few exceptions like the **Popular Upcoming** tab, wishlists are **not a factor** in your game's algorithmic visibility"; "Store Page Traffic Doesn't Matter"; "Conversion Rate Doesn't Matter"; "Review Score (Mostly) Doesn't Matter" (só abaixo de 40% afeta); "Players and Purchases Matter"; "Main capsule ... driven by what players spend time and money on, rather than Valve curation"; "Nope" para pagar por visibilidade |
| Steamworks — Wishlists | EN | doc viva, consultada 2026-11-21 | https://partner.steamgames.com/doc/marketing/wishlist | "Frequency limited to 2 weeks"; notificação de release/desconto ≥20%; "You can send a notification ... **once** ... up to two weeks following the release of your demo"; "there's **no formula** to accurately predict sales from wishlists"; "Is there a minimum number of wishlists ...? **No.**" |
| Steamworks — Demos | EN | doc viva, consultada 2026-11-21 | https://partner.steamgames.com/doc/store/application/demos | Demo é App ID separado; página própria opcional aceita reviews; "Demos can appear anywhere in Steam that a free game could appear ... **New & Trending**"; notificação única em até 14 dias; "we generally recommend waiting until launch" (com exceções); multiplayer demo≠jogo cheio |
| Steamworks — Steam Next Fest (hub) | EN | doc viva, consultada 2026-11-21 | https://partner.steamgames.com/doc/marketing/upcoming_events/nextfest | 3 edições/ano (fev, jun, out); **uma** edição por título; store page pública + demo jogável + não lançar antes do fim; registro pelo **jogo base**; prazos: 5 semanas (Press Preview)/3 semanas (build review)/11 dias (press)/4 dias (trailer)/7 dias (wrap-up); "the first few days of Next Fest are **completely randomized** ... After a few days ... carousels personalized by an algorithm ... based on user behavior"; "Are there any paid advertising opportunities ...? **No.**" |
| Steamworks — Next Fest: Feb 2027 | EN | doc viva, consultada 2026-11-21 | https://partner.steamgames.com/doc/marketing/upcoming_events/nextfest/feb_2027 | 22 fev 2027 10:00 PST → 1 mar 2027 10:00 PST; registro até **10 jan 2027 23:59 PST**; build para Press Preview até 18 jan; próxima edição: jun 2027 |
| Steamworks — Next Fest: Tips | EN | doc viva, consultada 2026-11-21 | https://partner.steamgames.com/doc/marketing/upcoming_events/nextfest/tips | Link de feedback na tela da demo; sub-fórum; botão de demo acima das opções de compra; testar streaming antes; "build review ... is **not** a full QA process" (doc hub) |
| Steamworks — Updating Your Game | EN | doc viva, consultada 2026-11-21 | https://partner.steamgames.com/doc/store/updates | "Feel free to update as often as you need"; "no better response than to fix the issue as quickly as possible (if reasonable)"; anunciar no Community Hub; Valve sugere grandes updates a cada 1–2 meses; testar em branch antes do default |
| Steamworks — Update Visibility Rounds | EN | doc viva, consultada 2026-11-21 | https://partner.steamgames.com/doc/marketing/visibility/update_rounds | **5 rounds** por produto; até 30 dias ou 1M impressões; só após visibilidade de lançamento; "randomly selected to appear"; precisa de announcement dos últimos 30 dias |
| Steamworks — User Reviews | EN | doc viva, consultada 2026-11-21 | https://partner.steamgames.com/doc/store/reviews | "Don't ask customers to review your product from within your application"; resposta oficial disponível, mas "**not every review needs to be responded to**"; "Set a limit on the amount of time spent reading reviews"; review score é fator só abaixo de 40% |
| Steamworks — Distributing Open Source Applications on Steam | EN | doc viva, consultada 2026-11-21 | https://partner.steamgames.com/doc/sdk/uploading/distributing_opensource | Licenças permissivas (MIT, BSD, Apache 2.0) "work fine"; copyleft (GPL) "problematic"; deve-se garantir direitos de distribuição |
| OhMyGame — case study Slotbound | EN | 2026-11-17 | https://www.ohmygame.ai/blog/slotbound | Síntese de >1 ano de devlogs; 590 → 50k em 17 dias → 70k ~1 mês → 100k até fim de set; "the **first 100,000 wishlists** [não dependeram do Next Fest]"; Gohjoe postou vídeo em 11 jul; comentaristas relatam ter wishlistado após o vídeo; "we can't establish how much each of those steps contributed"; 9 updates nos 3 primeiros dias; estimativa tributária é modelo do autor, não ganho real |
| GameMeca — entrevista com o dev (EN/KO) | KO→EN | 2026-11-04 | https://www.gamemeca.com/en/view.php?gid=1782926 | demo "on **itch.io in July and Steam in July**" (texto KO diz itch.io em **6월**/junho e Steam em 7월/julho); demo "Very Positive 82%"; >100k wishlists antes do launch; #2 em concurrent players de demos; dev: "putting out a demo is the right call"; clones em Douyin/WeChat |
| AUTOMATON | JA | 2026-11-19 | https://automaton-media.com/articles/newsjp/20261119-479967/ | "**6月にitch.io**でデモ版... **7月にはSteam**"; #2 em demos; 100k wishlists em 27 set; EA prevista 10 nov, adiada por build review, "11月13日にバージョン1.0として正式リリース"; 50k/48h, US$ 356k; 100k em 3 dias; pico ~32.770 (via SteamDB); ~1.200 reviews, 79% |
| slotbound.org — release date (terceiro, com fontes) | EN | atualizado 2026-11-19 | https://slotbound.org/release-date | "The Steam store date is **12 November 2026**. The developer posted '1.0 Is Out Now' on **13 November 2026**. I print both, because they are not the same line."; 50k wishlists 27 jul; 100k 27 set; "I found no developer post that says the game joined a Steam Next Fest" |
| VoxBooster — estatísticas Next Fest 2026 | PT-BR | 2026-11-15 | https://voxbooster.com/pt-BR/blog/steam-next-fest-demo-statistics-2026 | Out/2026: **7.240** demos (+65,2% vs jun); ganho mediano de wishlists dos top-10 caiu de +143k (jun) para **+80k** (out); mediana autorrelatada por edição: 806–1.079 wishlists; **19,3%** dos jogadores de demo wishlistaram (jun/2026); compradores entre wishlisters 1 semana pós-launch: 2,7%–3,8% (estimativas Alinea); "audiência construída antes do festival pesou mais que a idade da demo" |
| MGA Agency — guía Next Fest | ES | 2025-09-02, mod. 2026-05-06 | https://www.makegoodartagency.com/como-preparar-tu-participacion-en-steam-next-fest | "sólo puedes participar una vez"; "el número de wishlists ... aumenta ... alrededor de un **30%**"; "es muy, muy, muy raro ... aparecer un juego de la nada"; recomienda contactar creadores y prensa **antes** del evento |
| Unity China — 5 dicas Next Fest | ZH | 2025-06-02 | https://unity.com/cn/blog/steam-next-fest-developer-tips-unity | Chris Zukowski: "不要在 Next Fest 期间首次亮相演示" (não estrear a demo no festival); "第一天是算法注意到的时间" (crença de praticante); Gareth Martin: "你必须考虑如何吸引受众 ... 依靠外部观众推动演示" |

### Tabela explícita claim → verificado / [NV]

| # | Afirmação do resumo/vídeo | Veredito | Base observada |
|---|---|---|---|
| C1 | "591 wishlists em março" | **[NV] — data e número divergem** | Fonte primária diz "around **590**" em **julho de 2026** (10–11 jul, antes da demo). Nenhum post localizado menciona março ou o número exato 591. |
| C2 | "40 mil em duas semanas" | **[NV] — não localizado** | Série documentada é 590 → ~7.000 em 3 dias → **50.000 em 17 dias** (27 jul) → 70k (11 ago) → 100k (27 set). "40k/2 semanas" pode ser interpolação do gráfico, mas não há post que o afirme. |
| C3 | "100 mil [wishlists] antes do lançamento" | **VERIFICADO** | Steam News 26/27 set 2026; post do dev 27 set; GameMeca 4 nov; OhMyGame. |
| C4 | "100 mil cópias em 3 dias" | **VERIFICADO** | Steam Community 16 nov; X do dev; AUTOMATON 19 nov. |
| C5 | "79% positivo" | **VERIFICADO (instantâneo)** | Steam store: 79% de 1.598 reviews em 21 nov; AUTOMATON: ~79% de ~1.200 em 19 nov. A demo teve **82%** (GameMeca) — populações diferentes. |
| C6 | "lançamento 13/11" | **PARCIAL / ambíguo** | Steam store registra **12 nov**; post "1.0 Is Out Now" e v1.0.0 (4) em **13 nov**; imprensa JA adota 13 nov; plano original era EA em 10 nov, adiado por build review. |
| C7 | "50 mil cópias e US$ 356 mil em 48 h" | **VERIFICADO** | X do dev 16 nov; AUTOMATON; OhMyGame. Bruto, não líquido. |
| C8 | "Steam Next Fest foi essencial para o sucesso" | **CONTRADIZIDO / [NV] no efeito** | 100k wishlists foram atingidas **antes** do evento (27 set); dev anunciou participação em out/2026 e update de demo, mas não há dados de ganho atribuível; slotbound.org não encontrou post confirmando a inscrição. |
| C9 | "demo grátis no itch.io precedeu a Steam" | **VERIFICADO (imprensa); [NV] acesso direto** | GameMeca (KO: junho; EN: julho) e AUTOMATON (junho itch.io → julho Steam); página itch.io retornou HTTP 429 no acesso direto. |
| C10 | "runway de ~4,5 meses" | **[NV]** | Nenhuma fonte primária/localizada encontrada; OhMyGame fala apenas em ">1 ano de despesas de vida" como risco genérico. |
| C11 | "o algoritmo da Steam amplificou a demo por causa das wishlists" | **CRENÇA(parcialmente contradita)** | Valve: wishlists "**not a factor**" na visibilidade algorítmica, exceto Popular Upcoming; dev e praticantes atribuem ao "algoritmo", mas sem telemetria pública (`Traffic Breakdown` é privado). |
| C12 | "streamers amplificaram" | **VERIFICADO como ocorrência; [NV] como causalidade quantificada** | Vídeo de Gohjoe em 11 jul (~376k views segundo comentário); comentaristas relatam wishlist após vídeos; OhMyGame: "não estabelece que vídeos responderam por todo o crescimento". |

## Achados

1. **Os números centrais resistem à verificação, os detalhes não.** 100k wishlists pré-lançamento, 50k/48 h, 100k/3 dias e 79% positivo têm fonte primária (Steam News/Community, posts do dev, loja). Já "591 em março", "40k em duas semanas", "runway de 4,5 meses" e a autoria exata das frases no vídeo não foram observados (C1, C2, C10).
2. **A data de lançamento tem duas linhas legítimas.** 12/11 é a data da loja; 13/11 é o post "1.0 Is Out Now" e o primeiro hotfix. O vídeo/Luis provavelmente usou a segunda. Tratar as duas como a mesma linha é erro de rigor (C6).
3. **O gatilho documentado foi a demo, não o Next Fest.** O dev atribui o salto de ~600 → ~7.000 em 3 dias à demo ("demo was the spike!"); 100k wishlists vieram em 27/09, antes do evento de outubro. Não há evidência publicada de que o Next Fest tenha causado o crescimento (C8).
4. **A tese "wishlists alimentam o algoritmo" é falsa na documentação oficial da Valve.** O que pesa é jogar/comprar, idioma, tags e reação dos jogadores; tráfego da loja e taxa de conversão tampouco contam; abaixo de 40% de reviews positivas há prejuízo de destaque. Exceções nomeadas: Popular Upcoming, listas personalizadas, Update Visibility Rounds.
5. **Streamers funcionaram como canal, não como mecanismo comprovado.** Houve cobertura (Gohjoe no dia seguinte à demo, youtuber vietnamita citado em comentário) e o dev também fez outreach inicial (Reddit, X, Bluesky, communities coreanas, e-mails). A narrativa "o algoritmo fez tudo" é simplificação; o próprio dev oscila entre "game market itself", "steam algorithm and youtubers" e "I was just lucky".
6. **O playbook de Next Fest é de janela única e alta dilution.** 3 edições/ano, uma participação por título, store page pública, demo jogável, build review com 3–5 semanas de antecedência, press preview 11 dias antes. Em out/2026 havia 7.240 demos; ganho mediano autorrelatado de 806–1.079 wishlists, contra +80k para os dez maiores — a distribuição é *extremamente* assimétrica (viés de sobrevivência alto).
7. **Notificação de demo é recurso de uso único.** Só uma vez, em até 14 dias do primeiro lançamento da demo; cooldown de 2 semanas por App ID. Isso torna o timing da demo uma decisão irreversível dentro da janela do Next Fest.
8. **Hotfixes rápidos são documentados e alinhados à prática da Valve.** Nove updates em 3 dias, com patch notes públicos e resposta a relatórios. A Valve recomenda corrigir rápido, anunciar no hub e testar em branch; mas reserva "Update Rounds" (5 por produto) para **grandes** updates, não para cada hotfix.
9. **Backlog de reviews exige triagem, não resposta integral.** A própria Valve orienta limitar o tempo de leitura e não responder a toda review. O dev dizia ler "every review and message" — escalável em 1.598 reviews, não como política permanente.
10. **Licença não é bloqueio para Steam.** MIT é explicitamente compatível com o SDK Steamworks na doc da Valve. O bloqueio real é de infraestrutura: não há Steam page, nem build Steam, nem dados de wishlist.
11. **A alternativa soberana é publisher-independente.** itch.io + GitHub Releases entregam download versionado, checksums e changelog sem SDK proprietário no caminho crítico — exatamente o que a constituição IV exige e o que a Insurgência já possui (MIT, `verify.sh`, CI).

## Implicações para Insurgência (mapa explícito ID → impacto)

| ID | Impacto observável | Sugestão | Justificativa em 1 linha |
|---|---|---|---|
| **sb-16** — demo grátis no itch.io | Publicar build jogável versionado no itch.io **antes** de qualquer loja; Link fixo por tag/commit + checksum; existing `verify.sh` como gate de publicação. | **ADOPT** | Canal já compatível com soberania (IV) e com o padrão documentado do caso (itch.io precedeu Steam); não cria dependência proprietária. |
| **sb-17** — demo Steam + Next Fest | Impossível hoje: exige Steamworks, App ID de demo, build review, store page pública e jogo não lançado; próximas janelas 22/02–01/03/2027 (registro até 10/01) e 14–21/06/2027. | **DEFER** | Pré-requisito de plataforma ausente e participação é única por título; antecipar sem release versionado queimaria a chance. |
| **sb-18** — runway ~4,5 meses + wishlists | Substituir o número emprestado por gate local explicitado: orçamento registrado, custo de CI/arte, prazo até próximo marco — e métricas próprias (downloads/retorno do itch.io), não wishlist de terceiro. | **DEFER** | O valor 4,5 meses está `[NV]`; planejamento não pode importar finanças de outro dev, e wishlists dependem de Steam page inexistente. |
| **sb-19** — streamers + algoritmo | Produzir **capturas reproduzíveis** com seed fixa sob `--minimizado` (regra da janela), pacote de imprensa com comandos exatos e CI -> artefato; tratamento de criadores como canal, não como promessa de algoritmo. | **ADAPT** | A documentação Valve nega o mecanismo "wishlist → visibilidade"; o que é transferível é a forma do material (demo curta, legível, reproduzível). |
| **sb-22** — hotfixes rápidos pós-lançamento | Manter cadência de patches pequenos com changelog em GitHub Releases; triagem por impacto (bloqueante > regra > cosmetic); responder poucas reviews, com moderação; marcar verde só após comando executado (VI). | **ADOPT** | É a prática com evidência mais direta no caso (9 updates/3 dias) e casa com CI verde → merge autônomo e releases versionados. |

**Nota de fronteira:** a constituição (IV) veta dependência proprietária no caminho crítico. Steam, se algum dia adotado, entra como **canal adicional**, nunca como sistema de registro de progresso, partida ou identidade. O modo headless determinístico de Insurgência (`--minimizado`) é o equivalente funcional da "demo" do caso estudado — com a vantagem de ser inspecionável e redistribuível.

## Riscos, contra-evidência e [NV]

- **[NV — acesso falhou] SteamDB direto:** `https://steamdb.info/app/4459590/` retornou HTTP 403 (Cloudflare). Números de pico de jogadores e wishlists são, portanto, de posts do dev/imprensa ou citados indiretamente (AUTOMATON cita ~32.770; comentaristas citam faixa 90k–110k). Nenhum dado de telemetria Steam foi observado diretamente.
- **[NV — acesso falhou] itch.io:** a página do jogo e o devlog retornaram HTTP 429; a precedência itch.io → Steam apoia-se em GameMeca e AUTOMATON, com divergência de mês (junho vs julho).
- **[NV] Vídeo original:** transcrição de Garry Seto não recuperada. Atribuições "o vídeo diz X" derivam do resumo do usuário; a checagem recaiu sobre o resumo, não sobre o vídeo.
- **Contra-evidência a C8:** o dev disse textualmente "steam next festival on October" (11/08) e "big demo update ready for Next Fest" (27/09), enquanto slotbound.org afirma não ter encontrado post confirmando inscrição. O conflito é de fontes; o efeito do evento permanece `[NV]`.
- **Viés de sobrevivência (sb-28, tratado em outro fluxo):** Slotbound é um outlier. Em out/2026, a mediana de ganhos por demo foi de três dígitos baixos ou menos, e os dez maiores ganharam +80k; 7.240 demos competiram pela mesma atenção. Copiar o playbook sem essa base é erro de inferência.
- **Métricas não comparáveis:** 82% (demo, 4/11), 79% de ~1.200 (jogo, 19/11) e 79% de 1.598 (jogo, 21/11) medem populações e datas distintas. "100k wishlists" e "100k cópias" também são coisas diferentes — o próprio estudo observa que não há dados para dizer quantos wishlisters compraram.
- **Risco de cópia (sb-26, outro fluxo):** a abertura da demo facilitou clones em Douyin/WeChat; o dev só conseguiu agir com ajuda de fãs chineses. Para Insurgência isso reforça publicar com licença MIT explícita, changelog datado e commits assinados/versionados como prova de autoria.
- **Risco de "verde falso":** menções de vendas, reviews e pico de jogadores => são observações de terceiros ou do dev; nenhuma foi reproduzida localmente. Segundo a constituição VI, nada disso vira "verificado por nós" sem comando executado.

## Alternativas/concorrentes mais viáveis

1. **itch.io + GitHub Releases como canal primário (substitui sb-17 até que haja Steam page).** Versionamento, checksum, changelog e espelho — sem App ID, sem build review e sem SDK. É a opção que melhor satisfaz a constituição IV hoje.
2. **Demo headless determinística como "demo de imprensa" (substitui sb-19).** Um cenário curto com seed fixa, executado sempre com `--minimizado`, gera captura idêntica em CI; substitui a dependência de streamers por material auditável.
3. **Telemetria opt-in própria em vez de wishlist (substitui sb-18).** Contadores locais/agregados de downloads do itch.io e de execuções do cenário demo dão sinal de interesse sem servidor externo como ponto único de falha.
4. **Next Fest apenas como opção futura e única (sb-17 adiado).** Se decidido, escolher a edição mais próxima de um release real: fev/2027 (registro até 10/01/2027) ou jun/2027; preparar build e assets com 5 semanas de folga e não gastar o "round" de visibilidade em hotfix.
5. **Cadência de patches em GitHub Releases (sb-22 mantido, sem Steam).** Cada correção com tag semver, notas curtas e teste executado — o mesmo comportamento observado no caso, porém com fonte de verdade sob controle do projeto.

## Fontes

1. Steam store — Slotbound: https://store.steampowered.com/app/4459590/Slotbound/
2. Steam Community — notícias/patch notes: https://steamcommunity.com/app/4459590/allnews
3. Steam News — 100k wishlists: https://store.steampowered.com/news/app/4459590/view/696524994028504801
4. Dev — r/IndieDev, primeiro dia da demo (11/07/2026): https://www.reddit.com/r/IndieDev/comments/1utoziq/i_expected_maybe_10_concurrent_players_my_first/
5. Dev — r/IndieDev, gráfico de wishlists (13/07/2026): https://www.reddit.com/r/IndieDev/comments/1uv94ai/my_wishlist_graph_looked_normal_for_months_then_i/
6. Dev — r/SoloDevelopment, 590 → 70k (11/08/2026): https://www.reddit.com/r/SoloDevelopment/comments/1vldq7y/my_first_game_had_590_wishlists_when_i_released/
7. Dev — r/SoloDevelopment, 100k (27/09/2026): https://www.reddit.com/r/SoloDevelopment/comments/1wr9z8o/my_first_game_just_reached_100k_wishlists_still/
8. Dev — X, 48 h: https://x.com/OptimaArchGames/status/2121818614571622489
9. Dev — X, 100k cópias: https://x.com/OptimaArchGames/status/2122357483448717811
10. Valve — Visibility on Steam: https://partner.steamgames.com/doc/marketing/visibility
11. Valve — Wishlists: https://partner.steamgames.com/doc/marketing/wishlist
12. Valve — Demos: https://partner.steamgames.com/doc/store/application/demos
13. Valve — Steam Next Fest: https://partner.steamgames.com/doc/marketing/upcoming_events/nextfest
14. Valve — Next Fest Tips: https://partner.steamgames.com/doc/marketing/upcoming_events/nextfest/tips
15. Valve — Next Fest Feb 2027: https://partner.steamgames.com/doc/marketing/upcoming_events/nextfest/feb_2027
16. Valve — Updating Your Game: https://partner.steamgames.com/doc/store/updates
17. Valve — Update Visibility Rounds: https://partner.steamgames.com/doc/marketing/visibility/update_rounds
18. Valve — User Reviews: https://partner.steamgames.com/doc/store/reviews
19. Valve — Open Source no Steam: https://partner.steamgames.com/doc/sdk/uploading/distributing_opensource
20. OhMyGame — case study (EN, 17/11/2026): https://www.ohmygame.ai/blog/slotbound
21. GameMeca — entrevista (KO/EN, 04/11/2026): https://www.gamemeca.com/en/view.php?gid=1782926
22. AUTOMATON — cobertura (JA, 19/11/2026): https://automaton-media.com/articles/newsjp/20261119-479967/
23. slotbound.org — linhas de data (EN, 19/11/2026): https://slotbound.org/release-date
24. VoxBooster — estatísticas Next Fest (PT-BR, 15/11/2026): https://voxbooster.com/pt-BR/blog/steam-next-fest-demo-statistics-2026
25. MGA Agency — guía Next Fest (ES, 02/09/2025): https://www.makegoodartagency.com/como-preparar-tu-participacion-en-steam-next-fest
26. Unity China — 5 dicas Next Fest (ZH, 02/06/2025): https://unity.com/cn/blog/steam-next-fest-developer-tips-unity
27. LICENSE do projeto (MIT, verificado localmente): `LICENSE`
