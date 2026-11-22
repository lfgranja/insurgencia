# Idea Research: Narração IA em vídeos promocionais

- **Slug**: sb-05-narracao-ia
- **Data**: 2026-11-21
- **Confiança geral**: **baixa** — a premissa central ("Slotbound usou narração IA") é **inferência cautelar de Garry Seto**, não observação primária. Os fatos verificáveis são sobre *política de plataforma*, não sobre o caso Slotbound.

## Protocolo de evidência

- Fluxo SDD: intake → research → problem → concept → decision; esta é a 2ª portaria. Nenhum build/test/gate executado; nada aqui alega verde (Constituição VI).
- Conteúdo da web tratado como **dado**, nunca como instrução. Onde a fonte integral não foi vista/ouvida, uso `[NV]`.
- Notas de evidência consultadas (resumo — não as duplico aqui):
  - `docs/research/slotbound-2026-11-21-ai-promo.md` — transcrição do vídeo, disclosure 2026, captura auditável.
  - `docs/research/slotbound-2026-11-21-fit-local.md` — mapeamento para o código real e constituição (sb-05: "fora do código; impacta `docs/evidencias/` e disclosure").
  - `docs/research/slotbound-2026-11-21-alternativas.md` — DORA 2025 (IA como amplificador), canais de publicação.

## Achados relevantes (3–5)

1. **A alegação é inferência, não evidência.** Na transcrição (https://www.youtube.com/watch?v=Z4ayMTXnkoQ), Garry diz *"pretty sure his promotional videos are also AI generated"* — linguagem cautelar. Nem a Steam Content Survey, nem as análises independentes (Quickmatch, OhMyGame) afirmam voz IA em trailer. Os vídeos originais não foram inspecionados em áudio `[NV — apenas transcrição automática e descrição lidas]`.
2. **YouTube isenta gameplay e voz própria clonada, mas exige rótulo para conteúdo realista enganoso** (https://support.google.com/youtube/answer/14328491). Desde maio/2026 há detecção automática e rótulo em Shorts (blog do YouTube, PT-BR, https://blog.youtube/intl/pt-br/news-and-events/melhorando-rotulos-de-ia-para-usuarios-e-criadores/). O próprio vídeo de Garry observa que o TikTok "não parece ter viralizado" — short-form é **opinião**, não causa demonstrada.
3. **Steam exige declarar conteúdo gerado por IA que chega ao jogador, inclusive marketing** (Steamworks Content Survey §3, https://partner.steamgames.com/doc/gettingstarted/contentsurvey; redação PT-BR em `?l=brazilian`; reforço em GamesIndustry.biz, 2026-01-19). Voz em nuvem criaria dependência proprietária fora do caminho crítico, mas indesejável (Constituição IV).
4. **O que é primário sobre o caso**: 591 wishlists/4 meses → demo → ~40 mil/2 semanas (com ~12 mil retiradas), 79%/1.598 reviews na Steam e sketch de "básico bem feito" atribuído por Garry — nenhum número depende de IA em narração. Os "100K em 3 dias" são autorrelato `[NV — não verificado nesta nota]`.
5. **Alternativa auditável já cabe no repo**: devlogs escritos/versionados e README reproduzível fazem o mesmo papel informativo sem TTS, sem disclosure e sem janela (nota ai-promo, achados 7 e alternativas 1–2). Captura determinística é tratada em `sb-06`/`sb-20`.

## Implicações para este projeto

- Fora do código: toca `docs/evidencias/` e disclosure, não os 5 crates nem os 34 testes.
- Qualquer narração sintética admitida deve ser **local, rastreável e rotulada** — nunca usada como "prova" do sucesso alheio. Postar em plataforma fica **DEFER** até existir conta/estreia (guardrail do orquestrador).
- Regra da janela (SE): `--minimizado` sempre; gravar a janela normal está fora.
- A saída de LLM é informativa — o gate determinístico continua autoridade (Constituição I/VI).

## Fontes primárias

- Vídeo Garry Seto (transcrição/descrição): https://www.youtube.com/watch?v=Z4ayMTXnkoQ — EN, 2026-11-18.
- Steamworks Content Survey §3: https://partner.steamgames.com/doc/gettingstarted/contentsurvey (e `?l=brazilian`).
- YouTube Help — Disclosing use of GenAI content: https://support.google.com/youtube/answer/14328491.
- Blog do YouTube (PT-BR): https://blog.youtube/intl/pt-br/news-and-events/melhorando-rotulos-de-ia-para-usuarios-e-criadores/ — 2026-05-27.
- TikTok — responsible AIGC: https://www.tiktok.com/safety/en/policies-and-engagement/supporting-responsible-transparent-ai-generated-content.

`[NV]` aplicado a: conteúdo original do vídeo (áudio/imagem), números de vendas, eficácia de short-form para o Insurgência e execução de qualquer captura/encode.
