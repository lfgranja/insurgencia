# Decision — Narração IA em vídeos promocionais

- **Slug**: sb-05-narracao-ia
- **Decided**: 2026-11-21
- **Veredito**: ADAPT
- **Artifacts reviewed**: intake.md; research.md; problem.md; concept.md; `docs/research/slotbound-2026-11-21-ai-promo.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | A suspeita do intake existe, mas o problema real é disclosure/soberania, não a técnica: origem IA da voz é inferência de Garry `[NV]`. |
| Força da evidência | weak | Fonte primária é transcrição cautelar ("pretty sure"); vídeos originais não inspecionados; políticas 2026 (Steam/YouTube/TikTok) são primárias, o caso Slotbound não. |
| Valor vs. inação | adequate | Texto/devlog já comunica o jogo; narração é complemento — não agir evita risco, mas deixa a política de disclosure sem postura. |
| Viabilidade/Appetite | strong | Fora dos 5 crates; gerador local + rótulo é small; nenhuma engine nova (ADR-004 intacto). |
| Alinhamento estratégico | adequate | Serve I/IV/VI se local e rotulada; conflita com VI se inferência virar fato; regra da janela preservada. |
| Postura de risco | adequate | Risco de disclosure e alcance mitigado por rótulo; postar fica DEFER até conta, eliminando exposição prematura. |

## Racional

ADAPT: o que há de transportável é a necessidade de comunicar o jogo; o que é frágil é a premissa ("Slotbound usou narração IA") — inferência cautelar de Garry Seto, não evidência primária, com os vídeos originais não inspecionados `[NV]`. YouTube isenta gameplay e voz própria clonada, mas exige rótulo para conteúdo realista enganoso; Steam exige declarar IA inclusive em marketing. Portanto, se houver narração sintética, ela deve ser **local, rastreável e rotulada** — nunca apresentada como "prova" do sucesso alheio, nunca via TTS em nuvem (Constituição IV), nunca gravando janela normal (regra `--minimizado`). O papel informativo é coberto por devlog escrito/versionado (Constituição VI: gate é autoridade; saída de LLM é informativa). Publicar em plataforma fica **DEFER** até existir conta/estreia; produzir o clipe rotulado é imediato. Nada aqui toca os 5 crates, os 34 testes ou o CI.

## Condição/responsável

- **Responsável**: SE/mantenedor — autoridade sobre o que é publicado e sobre a política de disclosure do projeto. Coordenação com `sb-24`/`DisclosureLegalEvidence` para a checklist de rótulos na Steam.
- **Condição para avançar**: (1) ferramenta TTS local com licença rastreável, se narração for usada; (2) rótulo explícito no vídeo + declaração de marketing na Steam; (3) nenhuma inferência alheia citada como evidência; (4) conta/estreia definida antes de qualquer postagem — enquanto não houver, o item de publicar permanece `DEFER`.
- **Gate**: CI verde autoriza merge, nunca auto-merge cego; nenhuma alegação de verde sem comando executado (Constituição VI). Nenhum build/teste/gate rodado nesta etapa.

## Handoff

- **Problema**: risco de importar "narração IA" como atalho promocional com premissa não verificada, dependência de nuvem e disclosure ausente — fora do código, mas dentro da tese de verificação honesta.
- **Abordagem**: Opção 1 do concept — devlog escrito/versionado como núcleo; voz sintética apenas se local/rastreável/rotulada; postar `DEFER` até conta/estreia.
- **In scope (handoff)**: definir checklist de disclosure (Steam/YouTube/TikTok), escolher ferramenta local licenciada se aplicável, produzir clipe rotulado com captura determinística (coordenar com `sb-06`/`sb-20`), registrando `[NV]` onde a evidência faltar.
- **Out of scope**: TTS em nuvem, voz clonada de terceiros, conteúdo realista enganoso, publicação sem conta, janela normal, qualquer alteração em crates/gates/testes/engine (ADR-004 fixo).
- **Métricas**: 100% dos artefatos promocionais com origem sintética rotulada e declarada; zero afirmações não verificadas em docs; medição em 72 h via Steam UTM quando houver conta — não views.
- **Perguntas remanescentes**: `[NV]` ferramenta TTS local licenciada? `[NV]` checklist de disclosure já documentada? `[NV]` conta/estreia definida? `[NV]` conteúdo original do vídeo inspecionado em áudio (hoje apenas transcrição)?
