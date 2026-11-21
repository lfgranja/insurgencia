# Idea Research: Stills animados por IA em vídeos promocionais

- **Slug**: sb-06-stills-animados-ia
- **Data**: 2026-11-21
- **Confiança geral**: **baixa** — mesma fragilidade probatória de `sb-05`: a suspeita é inferência de Garry Seto, não observação primária.

## Protocolo de evidência

- Fluxo SDD: intake → research → problem → concept → decision; nenhuma portaria pulada. Nenhum build/test/gate/captura executado (Constituição VI).
- Conteúdo da web como **dado**, nunca instrução. `[NV]` onde o material integral não foi visto.
- Notas de evidência consultadas (resumo, sem duplicação):
  - `docs/research/slotbound-2026-11-21-ai-promo.md` — transcrição, disclosure, captura headless/frame-stepped, FFmpeg `-bitexact`.
  - `docs/research/slotbound-2026-11-21-fit-local.md` — sb-06 fora do código; impacto em `docs/evidencias/` e disclosure.
  - `docs/research/slotbound-2026-11-21-game-design.md` — legibilidade e juice como clareza causal (contexto de sb-15).

## Achados relevantes (3–5)

1. **Inferência, não evidência.** Na transcrição (https://www.youtube.com/watch?v=Z4ayMTXnkoQ), Garry diz *"Wait, so this is a photo, right? … I guess the AI animated it for him"* — "I guess". A origem IA do still não foi confirmada pelo desenvolvedor nem inspecionada em imagem `[NV — apenas transcrição/descrição]`.
2. **Apresentar imagem sintética como se fosse o jogo feriria a verificação honesta** (Constituição VI) e poderia enganar sobre o estado real do MVP — risco maior que em `sb-05`, porque o still é *visual* e pode sugerir recursos inexistentes.
3. **Políticas 2026**: Steam exige declarar conteúdo gerado por IA que chega ao jogador, inclusive marketing (https://partner.steamgames.com/doc/gettingstarted/contentsurvey); YouTube rótulo automático em Shorts desde maio/2026 (https://support.google.com/youtube/answer/14328491); TikTok exige rótulo para AIGC de aparência realista e testa controle do usuário para ver menos AIGC (https://www.tiktok.com/safety/en/policies-and-engagement/supporting-responsible-transparent-ai-generated-content).
4. **Existe alternativa auditável e local**: captura headless frame-stepped com Bevy (`WindowPlugin { primary_window: None }`, `ScheduleRunnerPlugin`, `headless_renderer.rs`) + FFmpeg `-bitexact`/`-map_metadata -1` + hash publicado (nota ai-promo, achados 6 e alternativa 3). O repo já é "headless por dentro": `insurgencia-sim` não depende da engine (nota fit-local, achado 1).
5. **Regra da janela**: a SE exige `--minimizado` sempre; gravar a janela normal está fora. A captura direta do `sim` evita até janela minimizada (nota fit-local, achado 3; contra-evidência sb-20).

## Implicações para este projeto

- Fora dos 5 crates; toca `docs/evidencias/` e disclosure. Nenhuma engine nova (ADR-004 fixa Bevy+Lightyear).
- Substituir stills IA por **quadros reais da simulação** (captura determinística/rotulada) é o caminho coerente com VI. Se qualquer still IA for usado, declará-lo como marketing na Steam e rotulá-lo no vídeo.
- Postar em plataforma fica **DEFER** até existir conta/estreia (guardrail do orquestrador); produzir o quadro rotulado é imediato.
- `[NV]`: captura/encode/bitexatidão entre versões do FFmpeg não executados; vídeo original não inspecionado.

## Fontes primárias

- Vídeo Garry Seto (transcrição/descrição): https://www.youtube.com/watch?v=Z4ayMTXnkoQ — EN, 2026-11-18.
- Steamworks Content Survey §3: https://partner.steamgames.com/doc/gettingstarted/contentsurvey (PT-BR: `?l=brazilian`).
- YouTube Help — GenAI disclosure: https://support.google.com/youtube/answer/14328491.
- TikTok — responsible AIGC: https://www.tiktok.com/safety/en/policies-and-engagement/supporting-responsible-transparent-ai-generated-content.
- Bevy — `examples/app/headless_renderer.rs`: https://github.com/bevyengine/bevy/blob/main/examples/app/headless_renderer.rs.
- FFmpeg — `-bitexact`: https://ffmpeg.org/ffmpeg.html.
