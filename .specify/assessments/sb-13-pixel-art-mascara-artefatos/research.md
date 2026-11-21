# Research — Pixel art mascara artefatos (sb-13)

**Data**: 2026-11-21. **Escopo**: ponteiros para as notas de evidência; não repito a pesquisa completa.

## Notas de evidência consultadas
- `docs/research/slotbound-2026-11-21-ai-art.md` — achado central: tese do youtuber é especulação, com contra-evidência técnica (escopo sb-02/03/04/13).
- `docs/research/slotbound-2026-11-21-fit-local.md` — low-poly procedural (`malha`/`achatar`, `CORES_TERRENO`) já resolve legibilidade sem IA; decisões locais anteriores.
- `docs/research/slotbound-2026-11-21-alternativas.md` — proveniência aberta e verificação determinística em vez de atalhos opacos.
- `docs/research/slotbound-2026-11-21-ai-code.md` — contexto da afirmação "100% IA" relatada no vídeo [NV].

## Achados relevantes
1. A tese é opinião, não evidência: na transcrição o autor diz "it's not as easy to see all the flaws because the pixel art squishy guys are quite small" e "probably the right genre right now" — linguagem de incerteza, sem teste, amostra ou métrica. https://www.youtube.com/watch?v=Z4ayMTXnkoQ
2. Contra-evidência técnica direta: forense ConvNeXt classifica imagens IA a **32×32 com 98,65%** de acurácia, com "Aliasing and Pixelation" como artefato detectável (https://openaccess.thecvf.com/content/ICCV2025W/APAI/papers/Sharma_Explainable_AI-Generated_Image_Forensics_A_Low-Resolution_Perspective_with_Novel_Artifact_ICCVW_2025_paper.pdf); NoiseShift mostra que baixa resolução *cria* artefatos (https://arxiv.org/html/2510.02307v1); SD-πXL exige constraints explícitos (https://arxiv.org/html/2410.06236v1).
3. Fit local: já existem facetas procedurais (`malha`/`achatar`) e paleta versionada (`CORES_TERRENO`) — a estética low-poly é legítima por legibilidade, custo e determinismo, independente de IA. local://crates/cliente/src/main.rs ; local://docs/GDD%20-%20Insurg%C3%AAncia.md
4. "Ocultar falhas" é infalsificável e colide com verificação honesta (VI) e com as decisões locais `ast-20/21`, `doc-11/12` (proveniência obrigatória, IA nunca fonte única). Baixa resolução como escolha estética é legítima; como máscara, não.
5. **[NV]** Vídeo lido via transcrição automática (2026-11-18); não comparei builds nem auditei a página da Steam, não verifiquei a legalidade da saída IA no Brasil e não rodei build/gate. Nenhuma afirmação numérica do vídeo foi adotada como fato.
