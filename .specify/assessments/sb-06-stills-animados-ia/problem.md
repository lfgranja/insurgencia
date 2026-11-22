# Problem — Risco de apresentar stills sintéticos como evidência de jogo

**Data**: 2026-11-21

## Problema

O intake registra a suspeita de que os materiais promocionais de Slotbound usariam IA para animar imagens estáticas (https://youtu.be/Z4ayMTXnkoQ, `[UNVERIFIED — fetch skipped]`). A nota `ai-promo` mostra que é inferência ("I guess"), não fato verificado; os vídeos originais não foram inspecionados em imagem `[NV]`. Para **este** projeto, o problema concreto é que um still animado por IA, se apresentado como se fosse o jogo, sugeriria recursos visuais que o MVP ainda não tem — ferindo a **verificação honesta** (Constituição VI) e podendo exigir disclosure de marketing na Steam (Content Survey §3). Hoje o repo tem um HUD 100% textual (`atualizar_texto`) e um `sim` headless determinístico, mas não tem pipeline de captura; a regra da SE (`--minimizado` sempre) impede gravar a janela normal.

## Sucesso observável

- Todo material visual promocional deriva de **quadros reais da simulação** (captura determinística com semente/commit registrados) — nunca de imagem sintética apresentada como gameplay.
- Se um still IA for usado de fato, ele está **declarado como marketing na Steam** e **rotulado no vídeo**, verificável por inspeção do artefato.
- Nenhum documento afirma que "Slotbound usou stills IA" como fato; a suspeita permanece marcada `[NV]`.
- A origem de cada quadro é rastreável: semente + commit no nome do artefato, com hash publicado (Constituição VI).

## Custo de não agir

Ignorar o tema deixa o projeto sem critério para o que é "gameplay real" em vídeo e expõe três riscos: (1) **falsa evidência** — um still bonito mas sintético pode ser lido como prova de um MVP que não existe, corroendo a tese de verificação honesta; (2) **passivo de disclosure** — marketing com IA exige declaração na Steam e rótulo em Shorts (regras 2026); (3) **violação da regra da janela**, se alguém abrir a janela normal para gravar. O custo é evitável por construção: o `sim` já é headless e determinístico, e a captura frame-stepped é documentada pelo Bevy.

## Stakeholders e restrições

- **Afetados**: SE/mantenedor (autoridade sobre o que é publicado); leitores de `docs/evidencias/`; plataformas (Steam/YouTube/TikTok).
- **Restrições**: constituição I (gate é autoridade), IV (sem dependência proprietária), VI (verificação honesta); regra da janela (`--minimizado`); ADR-004 fixa Bevy+Lightyear — nenhuma engine nova por causa de vídeo; saída de LLM é informativa.
- **Fora de escopo**: criar o pipeline de captura/encode nesta etapa; postar em plataforma (`DEFER` até conta/estreia); tocar crates, gates, testes ou docs fora destes 4 artefatos.

## Perguntas abertas

- `[NV]` Qual quadro/ferramenta de captura headless usar no repo: exemplo `headless_renderer` do Bevy ou saída direta do `sim` em texto/PNG?
- `[NV]` Bit-exatidão do FFmpeg entre versões não foi verificada empiricamente — como publicar hash de forma reproduzível?
- `[NV]` Existe conta/estreia definida? Enquanto não houver, postar permanece `DEFER`.
