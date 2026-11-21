# Problem — Pixel art mascara artefatos (sb-13)

**Data**: 2026-11-21

O intake registra a tese de Garry de que personagens pequenos e pixel art de baixo detalhe mascarariam artefatos de IA [NV — opinião do autor do vídeo, sem teste]. No Insurgência, o problema concreto é que essa tese, se adotada como **justificativa de design**, tornaria a qualidade infalsificável: "esconder falhas" não pode ser verificado e contraria a constituição VI (verificação honesta) e as decisões locais de proveniência. Ao mesmo tempo, o projeto já tem uma estética low-poly/facetada legítima (`malha`/`achatar`, `CORES_TERRENO`) que precisa ser documentada **por suas próprias razões** — legibilidade e determinismo — e não como cortina para arte opaca.

## Sucesso observável
- A estética low-poly/facetada é mantida e documentada por legibilidade, custo e determinismo; nenhum documento a justifica como forma de "ocultar artefatos".
- Qualidade visual é avaliada por inspeção e testes reproduzíveis, não pela ausência de detectabilidade; qualquer arte passa pelo inventário de proveniência antes de versionar.
- Mudanças visuais passam por `verify.sh` (check → clippy → nextest → fmt) + 5 gates, sem `.unwrap()`/`.expect()`, com `#![forbid(unsafe_code)]`/`#![deny(clippy::pedantic)]`.
- Regra `--minimizado` preservada e ADR-004 (Bevy+Lightyear) intacto; nenhuma dependência de IA remota é introduzida.

## Custo de não agir
Aceitar a tese como fundamento substituiria evidência por intuição, enfraquecendo a verificação honesta que é vantagem competitiva do projeto: arte opaca não auditável, forks divergentes e perda de confiança. Manter a estética low-poly sem a justificativa de máscara custa nada — ela já existe e é reproduzível.

**[NV]** A tese foi lida na transcrição automática do vídeo; não comparei builds, não auditei a página da Steam e não rodei build/gate nesta escrita (constituição VI).
