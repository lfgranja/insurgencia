# Research — Pixel art IA (sb-02)

**Data**: 2026-11-21. **Escopo**: ponteiros para as notas de evidência; não repito a pesquisa completa.

## Notas de evidência consultadas
- `docs/research/slotbound-2026-11-21-ai-art.md` — ferramentas, copyright e disclosure de arte/sprites/UI (escopo sb-02/03/04/13).
- `docs/research/slotbound-2026-11-21-alternativas.md` — IA assistida + verificação determinística em vez de "100% IA" (DORA, METR, Aider, OpenCode).
- `docs/research/slotbound-2026-11-21-fit-local.md` — cliente procedural (`criar_terreno`, `CORES_TERRENO`), `COPYRIGHT.txt`, gate local.
- `docs/research/slotbound-2026-11-21-ai-code.md` — origem da afirmação "100% IA" relatada no vídeo [NV].

## Achados relevantes
1. Saída puramente IA não tem copyright automático: USCO (2025) exige controle humano sobre os elementos expressivos e "prompts do not alone provide sufficient control"; D.C. Circuit, *Thaler v. Perlmutter* (2025) exige autoria humana. https://www.copyright.gov/ai/Copyright-and-Artificial-Intelligence-Part-2-Copyrightability-Report.pdf ; https://media.cadc.uscourts.gov/opinions/docs/2025/03/23-5233.pdf
2. Pixel art de baixa resolução não "esconde" falhas: forense detecta IA a 32×32 com 98,65% (https://openaccess.thecvf.com/content/ICCV2025W/APAI/papers/Sharma_Explainable_AI-Generated_Image_Forensics_A_Low-Resolution_Perspective_with_Novel_Artifact_ICCVW_2025_paper.pdf); NoiseShift mostra que baixa resolução *cria* artefatos (https://arxiv.org/html/2510.02307v1); SD-πXL exige paleta/resolução fixas explícitas (https://arxiv.org/html/2410.06236v1).
3. Ferramentas generativas trazem dependência externa: PixelLab tem modelos proprietários, MCP HTTP remoto com bearer token e ToS com Open RAIL-M e suspensão unilateral; Pixelorama é MIT e Piskel Apache-2.0 (editores, não geradores). https://www.pixellab.ai/termsofservice ; https://pixelorama.org/faq ; https://github.com/piskelapp/piskel
4. Proveniência embutida é frágil: 0/14 assinaturas C2PA sobreviveram a reencode/crop/screenshot (2026); a fonte da verdade deve ser inventário local (hash + licença + ferramenta + prompt/seed + data). https://imageevidence.com/research/ai-image-provenance ; https://c2pa.org/a-new-implementation-guide-for-content-credentials/
5. Fit local: nenhum sprite versionado hoje; `COPYRIGHT.txt` exige identificar e listar material de terceiros antes de versionar; já existe estética procedural por código. Alternativa de menor risco é gerador determinístico local (seed+paleta+hash) ou CC0/humano com contrato escrito — nunca IA remota no caminho crítico (constituição IV/VI). [NV — não executei ferramentas de IA, não auditei a página da Steam do Slotbound, não verifiquei a proporção alegada no intake.]
