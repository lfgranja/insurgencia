# Research — Animações IA (sb-03)

**Data**: 2026-11-21. **Escopo**: ponteiros para as notas de evidência; não repito a pesquisa completa.

## Notas de evidência consultadas
- `docs/research/slotbound-2026-11-21-ai-art.md` — ferramentas de sprite/animação, copyright e disclosure (escopo sb-02/03/04/13).
- `docs/research/slotbound-2026-11-21-fit-local.md` — avatar estático, `Update`/`sincronizar_avatar`, sim headless (`avancar_tick`, `hash_estado`), regra `--minimizado`.
- `docs/research/slotbound-2026-11-21-alternativas.md` — IA assistida + verificação determinística em vez de geração remota.
- `docs/research/slotbound-2026-11-21-ai-code.md` — origem da afirmação "100% IA" relatada no vídeo [NV].

## Achados relevantes
1. Ferramentas de animação por IA existem, mas são serviço externo: PixelLab gera sprites 4/8 direções com walk/run/idle; extensão Aseprite exige v1.3+ (trial não suporta); MCP é HTTP remoto com bearer token; ToS impõe Open RAIL-M e suspensão unilateral. https://www.pixellab.ai/docs/ways-to-use-pixellab ; https://www.pixellab.ai/termsofservice
2. Geração pixel art quantizada é problema de pesquisa, não atalho: SD-πXL exige paleta/resolução fixas (https://arxiv.org/html/2410.06236v1); NoiseShift mostra que modelos treinados em resolução fixa falham em resoluções menores, gerando artefatos (https://arxiv.org/html/2510.02307v1).
3. Fit local: o avatar do cliente é estático (`criar_avatar`); animação generativa não é reproduzível por seed determinística e dependeria de modelo remoto em runtime — colisão direta com constituição IV e VI. `avancar_tick`/`hash_estado` já oferecem estado determinístico para derivar frames. local://crates/cliente/src/main.rs ; local://crates/sim/src/lib.rs
4. Proveniência de saída IA é incerta e frágil: USCO/D.C. Circuit exigem autoria humana; 0/14 assinaturas C2PA sobreviveram a reencode/screenshot; inventário local é a fonte da verdade. https://www.copyright.gov/ai/Copyright-and-Artificial-Intelligence-Part-2-Copyrightability-Report.pdf ; https://imageevidence.com/research/ai-image-provenance
5. **[NV]** O intake não especifica quais animações seriam atribuídas à IA, não verifiquei o post original, não executei ferramentas de IA nem rodei build/gate. A afirmação do vídeo permanece hipótese; a recomendação apoia-se no que é observável no repo.
