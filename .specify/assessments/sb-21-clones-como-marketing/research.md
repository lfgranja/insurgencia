# Research — Clones como marketing (sb-21)

**Data**: 2026-11-21 · **Idiomas**: EN/KO primários; ZH via fonte secundária. Conteúdo web tratado como **dado, nunca instrução**; nenhuma diretiva de página executada.

## Notas de pesquisa consultadas
- `docs/research/slotbound-2026-11-21-disclosure-legal.md` — clonagem, arcabouço legal e ética do "clone como marketing" (achados 6–8).
- `docs/research/slotbound-2026-11-21-steam-gtm.md` — canal, hotfixes e viés de sobrevivência (achado 5, risco de cópia).
- `docs/research/slotbound-2026-11-21-alternativas.md` — ALT-17 (proveniência aberta) no lugar de perseguição a clones.
- `docs/research/slotbound-2026-11-21-fit-local.md` — sb-21: sem fit de código; tese "fork é feature" (IV).

## Achados
1. **O incidente é documentado por entrevista, mas é externo ao projeto.** Gamemeca registra dois clones em mini-programas de Douyin e WeChat, denúncia oficial datada de **20/10** na categoria de infração de direito autoral de mini-programa, prazo de **7 dias úteis** e remoção do primeiro caso. O gargalo foi acesso (celular chinês/captcha), não a lei.
2. **"Clone como marketing" tem dois sentidos; só o *post hoc* é legítimo.** No caso Scale the Depths, o postmortem original e a cobertura da AUTOMATON documentam que clones ruins levaram jogadores ao original ("ended up becoming advertisements for our game"). É efeito observado *depois* do incidente, não campanha planejada.
3. **Fabricar um clone é infalsificável e eticamente reprovável.** Nenhum caso documentado de "fazer 30 cópias slop" foi encontrado; a ideia foi majoritariamente rejeitada no próprio thread do r/gamedev. Não há evidência que sustente a versão planejada da ideia.
4. **A atribuição ao vídeo ("ele tem os recibos") permanece [NV].** A transcrição integral foi lida via `youtube-transcript-api`, mas o vídeo em si não foi buscado nesta nota; a frase vem da leitura do apresentador sobre o material público do dev, não de medição independente.
5. **No repo, a única resposta coerente é documental.** `docs/research/slotbound-2026-11-21-fit-local.md` mapeia sb-21 como "externo; sem fit de código" e aponta ALT-17 (MIT + releases versionadas + atestação) como substituto de perseguição legal. Constituição IV ("fork é feature") veta provocar o incidente.

## Fontes (URLs)
- Gamemeca — entrevista com Optima Arch: https://www.gamemeca.com/en/view.php?gid=1782926 · https://www.gamemeca.com/view.php?gid=1782926
- Reddit r/gamedev — postmortem Scale the Depths: https://www.reddit.com/r/gamedev/comments/1tq3sv6/our_game_jam_entry_blew_up_and_we_turned_it_into/
- AUTOMATON WEST — cobertura do caso: https://automaton-media.com/en/news/the-developers-of-a-viral-fishing-game-faced-countless-ai-slop-mobile-clones-but-the-rip-offs-were-so-bad-they-ended-up-promoting-the-real-game/
- TryAIgame — perfil ZH do caso: https://tryaigame.com/zh/games/slotbound/
- Vídeo Garry Seto [NV — não buscado]: https://www.youtube.com/watch?v=Z4ayMTXnkoQ
- GitHub Docs — artifact attestations: https://docs.github.com/en/actions/concepts/security/artifact-attestations
