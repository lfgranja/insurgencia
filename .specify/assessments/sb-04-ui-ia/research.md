# Research — UI IA (sb-04)

**Data**: 2026-11-21. **Escopo**: ponteiros para as notas de evidência; não repito a pesquisa completa.

## Notas de evidência consultadas
- `docs/research/slotbound-2026-11-21-ai-art.md` — copyright, disclosure e ferramentas de arte/UI (escopo sb-02/03/04/13).
- `docs/research/slotbound-2026-11-21-fit-local.md` — HUD em `format!` puro em `atualizar_texto`, sem assets; gate local.
- `docs/research/slotbound-2026-11-21-alternativas.md` — verificação determinística em vez de geração remota; Aider/OpenCode, DORA.
- `docs/research/slotbound-2026-11-21-ai-code.md` — origem da afirmação "100% IA" relatada no vídeo [NV].

## Achados relevantes
1. Fit local decisivo: `atualizar_texto` (main.rs:547-561) monta o HUD **inteiramente em `format!`** de posição/ilha/limites, sem texturas nem serviço externo; UI "gerada por IA" adicionaria dependência e risco de copyright sem ganho funcional. local://crates/cliente/src/main.rs
2. Steam considera conteúdo que "ships with your game and is consumed by players", pré-gerado ou ao vivo; itch.io exige campo de AI Disclosure, mas isenta algoritmos autocontidos (ex.: procedural level generation) — UI textual determinística se enquadra na isenção. https://partner.steamgames.com/doc/gettingstarted/contentsurvey ; https://itch.io/docs/creators/quality-guidelines ; https://itch.io/t/4309690/generative-ai-disclosure-tagging
3. Autoria humana continua exigida: USCO (2025) e D.C. Circuit (2025) negam copyright a material puramente IA; prompts isolados não bastam. https://www.copyright.gov/ai/Copyright-and-Artificial-Intelligence-Part-2-Copyrightability-Report.pdf ; https://media.cadc.uscourts.gov/opinions/docs/2025/03/23-5233.pdf
4. Proveniência embutida é frágil (0/14 C2PA sobreviveram a reencode/screenshot); se IA inspirar layout, versionar apenas o resultado autoral revisado e atribuído, nunca gerar em runtime. https://imageevidence.com/research/ai-image-provenance
5. **[NV]** O intake não especifica quais elementos da interface seriam gerados por IA nem se a afirmação está no post original; não auditei a página viva da loja nem rodei build/gate. A recomendação baseia-se no que é observável no repositório.
