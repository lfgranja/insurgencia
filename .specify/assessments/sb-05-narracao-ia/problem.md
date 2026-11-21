# Problem — Risco de importar "narração IA" como atalho de promoção sem evidência nem rótulo

**Data**: 2026-11-21

## Problema

O intake registra a suspeita de que os shorts promocionais de Slotbound usariam voz gerada por IA (https://youtu.be/Z4ayMTXnkoQ, host fora da allowlist — `[UNVERIFIED — fetch skipped]`). A nota `ai-promo` mostra que isso é **inferência cautelar** de Garry Seto ("pretty sure"), não fato verificado, e que nenhuma fonte primária consultada confirma voz IA no trailer. Para **este** projeto, o problema concreto não é "copiar a técnica", mas decidir como produzir material promocional sem (a) tratar uma inferência alheia como receita, (b) criar dependência de serviço TTS proprietário (Constituição IV) e (c) ferir a verificação honesta (Constituição VI) ao não rotular conteúdo sintético. Hoje o repositório não tem pipeline de vídeo, e a regra da SE exige `--minimizado` em qualquer inspeção — gravar a janela normal está fora.

## Sucesso observável
- Nenhuma afirmação do tipo "Slotbound usou IA e vendeu 100K" entra em docs como medida sem fonte primária; a inferência de Garry fica marcada como tal (`[NV]`).
- Todo material promocional com voz sintética, se existir, usa **gerador local/rastreável**, com **rótulo explícito** no vídeo e **declaração de marketing** na Steam (Steamworks Content Survey §3), verificável por inspeção do artefato.
- O papel informativo da narração é coberto por **devlog escrito e versionado** (auditável por `git log`), sem TTS em nuvem e sem abrir janela.
- Publicação em plataforma só ocorre **depois** que existir conta/estreia — antes disso, o clipe é produzido e rotulado localmente (`DEFER` de postar, não de produzir).

## Custo de não agir

Se o tema for ignorado, o projeto corre três riscos concretos: (1) **desvio de autoridade** — adoptar a "receita" de um caso cuja premissa é inferência, desperdiçando esforço em TTS em vez de demo/determinismo (os fatores que Garry realmente cita); (2) **dependência e passivo de disclosure** — TTS em nuvem introduz serviço externo e obrigação de rótulo que podem ser evitados por construção (Constituição IV/VI); (3) **desalinhamento com a regra da janela**, se alguém gravar a janela normal para "produzir o vídeo". O custo é pequeno em código (fora dos 5 crates), mas real em credibilidade: um projeto cuja tese é verificação honesta não pode promover conteúdo sintético sem rótulo.

## Stakeholders e restrições

- **Afetados**: mantenedor único/SE (autoridade sobre o que é publicado); leitores de `docs/evidencias/`; plataformas (Steam/YouTube/TikTok), cujas regras 2026 mudaram.
- **Restrições**: constituição I (gate é autoridade), IV (sem dependência proprietária no caminho crítico), VI (nada afirmado verde sem execução); regra da janela (`--minimizado` sempre); ADR-004 fixa Bevy+Lightyear — nenhuma engine nova entra por causa de vídeo; saída de LLM é informativa.
- **Fora de escopo**: escrever/criar o pipeline de vídeo nesta etapa; postar em qualquer plataforma (depende de conta — `DEFER`); tocar em crates, gates, testes ou docs fora destes 4 artefatos de assessment.

## Perguntas abertas

- `[NV]` Qual ferramenta TTS local (se houver) atende a licença e a rastreabilidade sem serviço externo?
- `[NV]` A política de disclosure da Steam para marketing com voz sintética já foi traduzida em checklist documental (coordenar com `sb-24` e `DisclosureLegalEvidence`)?
- `[NV]` Existe conta/estreia definida para publicar? Enquanto não houver, o veredito de postar permanece `DEFER`.
