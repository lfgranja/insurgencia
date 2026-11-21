# Problem — Pixel art IA (sb-02)

**Data**: 2026-11-21

O intake afirma que a maior parte do pixel art do Slotbound teria sido gerada por IA, com visuais "refinados pelo dev". No Insurgência o problema concreto não é imitar essa afirmação (não verificada [NV]), mas decidir como produzir sprites sem quebrar soberania e verificação. Hoje não há nenhum sprite versionado, e `COPYRIGHT.txt` exige que qualquer material de terceiros seja identificado, atribuído e listado **antes** da inclusão. Adotar geração remota de pixel art como fonte (PixelLab/MCP) introduziria serviço externo no caminho crítico (constituição IV), titularidade incerta (USCO/D.C. Circuit) e risco de disclosure — sem que o repositório ganhe anything reproduzível.

## Sucesso observável
- Todo sprite novo é produzido por gerador determinístico local (seed + paleta + hash registrados) ou vem de CC0/humano com contrato de originalidade; nenhum serviço remoto no caminho crítico.
- Cada asset tem entrada no inventário de proveniência: URL/ferramenta, licença SPDX, seed/prompt, hash e data; `COPYRIGHT.txt` atualizado antes de versionar.
- A estética low-poly/facetada existente (`malha`/`achatar`, `CORES_TERRENO`) é preservada; saída reproduzível passa em `verify.sh` (check → clippy → nextest → fmt) e 5 gates, sem `.unwrap()`/`.expect()` e com `#![forbid(unsafe_code)]`/`#![deny(clippy::pedantic)]`.
- Nenhuma janela normal aberta (regra `--minimizado`); ADR-004 (Bevy+Lightyear) intacto.

## Custo de não agir
Sem inventário e sem fonte determinística, forks não conseguem reproduzir nem auditar os visuais, a titularidade permanece opaca e uma eventual mudança de redação de disclosure na loja vira risco reputacional. Perseguir "pixel art IA" pelo vídeo troca um ativo verificável por dependência de fornecedor e incerteza jurídica.

**[NV]** Proporção de arte IA no Slotbound, redação atual da página da Steam e legalidade da saída IA no Brasil não verificadas aqui; nenhum build/gate executado nesta escrita (constituição VI).
