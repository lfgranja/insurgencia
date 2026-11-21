# Research — Disclosure IA na Steam (sb-24)

**Data**: 2026-11-21 · **Idiomas**: EN primário; JA/PT-BR secundários; KO/ZH via fontes citadas. **Não é afirmação de status legal**: é o que a documentação oficial diz hoje.

## Notas de pesquisa consultadas
- `docs/research/slotbound-2026-11-21-disclosure-legal.md` — achados 1–3 (política da Steam, disclosure da página real, ausência de walk-back).
- `docs/research/slotbound-2026-11-21-steam-gtm.md` — Steam como canal ausente no repo; MIT compatível com SDK (doc da Valve).
- `docs/research/slotbound-2026-11-21-alternativas.md` — sb-24 fora de escopo; disclosure é documental, não altera gates.
- `docs/research/slotbound-2026-11-21-fit-local.md` — sb-24: disclosure documental (ADRs), não toca gates; política da loja **[NV]**.

## Achados
1. **A política vigente da Steam é mais estreita que a de 2024 e não exige disclosure de ferramentas de código.** O Steamworks Content Survey §3 foca em conteúdo **embarcado e consumido pelo jogador** (arte, som, narrativa, localização) e em conteúdo **gerado em runtime**; "efficiency gains through the use of [AI-powered] tools is not the focus". A reformulação de janeiro/2026 é confirmada por fonte secundária, e o guia JA registra que o aviso de 2024 listava "コード" entre os exemplos — o texto atual não lista.
2. **O disclosure da página real é curto e cobre só visuais/promocionais.** O texto integral fala em "various visual assets and promotional materials ... created with the assistance of generative AI tools. All AI-generated content was reviewed, refined, and integrated by the developer" — coerente com a regra da Steam, mas mais estreito que a autodescrição do autor no Reddit ("100% of the game's code was generated with AI").
3. **Não há evidência arquivada de "walk-back".** A versão do post original (22/05/2026) no arquivo Arctic Shift mantém a frase "100% ... AI" **sem marcador `EDIT:`**. Comentários posteriores (07 e 16/09) trazem informação nova ("for most additional art now there's someone who do create it for me"), não retratação. A acusação de apagar posts existe e foi negada; Internet Archive estava offline **[NV]**. A diferença é de **escopo**, não contradição factual.
4. **A regra de não exigir código não autoriza omissão num projeto de inspecionabilidade.** Para Insurgência, cuja identidade é verificação honesta (VI) e soberania do jogador (IV), um disclosure mínimo esconderia parte relevante do processo — exatamente o que gerou a suspeita no caso analisado.
5. **ÉDocumental e não altera gates.** A nota de proveniência vive em `docs/adr/` e na documentação; não há Steam page no repo, então a página da loja é **[NV]** até existir publicação.

## Fontes (URLs)
- Steamworks — Content Survey §3 (primária): https://partner.steamgames.com/doc/gettingstarted/contentsurvey
- Steam Store — Slotbound, página real (disclosure + 79% de 1.598): https://store.steampowered.com/app/4459590/Slotbound/
- Game Developer — reformulação 01/2026: https://www.gamedeveloper.com/business/valve-tweaks-and-clarifies-ai-disclosure-rules-for-steam
- FlameLitt — guia JA de disclosure (cita mudança de 2024→2026): https://flamelitt.com/articles/steam-generative-ai-disclosure-how-to/
- Reddit r/aigamedev — post original 22/05/2026 via Arctic Shift: https://www.reddit.com/r/aigamedev/comments/1tkeg86/im_not_a_developer_and_this_slot_machine/
- Reddit — comentário 16/09/2026: https://www.reddit.com/r/aigamedev/comments/1tkeg86/im_not_a_developer_and_this_slot_machine/pa5dicv/
- Reddit r/IndieDev — disputa sobre transparência: https://www.reddit.com/r/IndieDev/comments/1ustzth/8_months_ago_i_had_never_made_a_game_today_my/
- Internet Archive CDX — acesso falhou **[NV]**: https://web.archive.org/cdx/search/cdx
