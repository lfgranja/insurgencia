# Concept — Pixel art mascara artefatos (sb-13)

**Data**: 2026-11-21

## Opções
1. **Manter low-poly facetado documentado por legibilidade e determinismo (recomendada).** Consolidar `malha`/`achatar`/`CORES_TERRENO` como estética provisória com razões próprias — clareza de silhueta, custo baixo, saída reproduzível por `hash_estado`. Rejeita explicitamente a justificativa de "ocultar artefatos" como infalsificável e contrária a VI. Coerente com ADAPT e com as decisões `ast-20/21`, `doc-11/12`.
2. **Low-res como escolha estética com inspeção e inventário.** Manter resolução/facetas baixas por gosto visual, mas com revisão e proveniência registradas — trata baixa resolução como estilo, não como máscara. Compatível, porém redundante com a Opção 1 se o inventário já existir.
3. **Usar baixa resolução para esconder falhas de IA.** Adotar a tese do vídeo como diretriz. Contradiz a contra-evidência (forense 32×32 a 98,65%, NoiseShift, SD-πXL), é infalsificável e fere a constituição VI e a política de proveniência. Rejeitada.

## Recomendação (ADAPT)
Opção 1. Preservamos o que é verificável localmente — a estética low-poly já resolve legibilidade sem IA — e recusamos a justificativa de ocultação. A afirmação do vídeo permanece hipótese [NV]; a decisão apoia-se em contra-evidência técnica e nos princípios I/IV/VI. Gates determinísticos continuam sendo a autoridade sobre a qualidade, não a ausência de detectabilidade.
