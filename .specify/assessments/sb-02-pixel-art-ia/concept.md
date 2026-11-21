# Concept — Pixel art IA (sb-02)

**Data**: 2026-11-21

## Opções
1. **Gerador determinístico local + inventário de proveniência (recomendada).** Estender o que já existe (`malha`/`achatar`, `CORES_TERRENO`, `hash_estado`) a sprites com seed e paleta versionados; registrar hash, licença, ferramenta e data no inventário e no `COPYRIGHT.txt`. Custo baixo, saída reproduzível por `verify.sh`, sem serviço externo. Coerente com ADAPT e com as decisões locais `ast-20/21`, `doc-11/12`.
2. **CC0-first com arte humana e editores FOSS.** Packs CC0 baixados uma vez (SPDX + URL + hash + data) e autoria humana em Pixelorama (MIT)/Piskel (Apache-2.0); cessão escrita quando humano. Titularidade clara e relicenciável sob MIT, mas exige disciplina de registro e trabalho manual.
3. **PixelLab/MCP remoto como fonte de sprites.** Gera 4/8 direções com idle/walk/run, mas impõe modelo proprietário, token remoto, Open RAIL-M e suspensão unilateral (ToS) — ponto único de falha e copyright incerto. Rejeitada como fonte; no máximo ferramenta opcional fora do caminho crítico, com inventário.

## Recomendação (ADAPT)
Opção 1 como padrão, com Opção 2 como via complementar quando fizer sentido estético. Mantém a afirmação do vídeo como hipótese [NV] e preserva o que é verificável localmente: determinismo (I), soberania do jogador/fork (IV) e verificação honesta (VI). IA, se usada, é inspiração informativa — nunca fonte única nem autoridade; gates determinísticos continuam decidindo o que entra.
