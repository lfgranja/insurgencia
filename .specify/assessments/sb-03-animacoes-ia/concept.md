# Concept — Animações IA (sb-03)

**Data**: 2026-11-21

## Opções
1. **Frames versionados derivados do estado (recomendada).** Tabelas de animação indexadas por `hash_estado`/tick ou tempo, com sprites locais (gerador determinístico ou CC0/humano inventariado); a animação é função do estado da sim, nunca de modelo em runtime. Custo baixo, reproduzível por `verify.sh`. Coerente com ADAPT.
2. **Spritesheet autoral/CC0 com sequência definida à mão.** Sequências idle/walk/run desenhadas ou de pack CC0, com inventário (SPDX + URL + hash + data) e cessão escrita quando humano. Dá controle expressivo, mas exige trabalho manual e registro.
3. **Geração IA remota (PixelLab/MCP) em runtime ou como fonte única.** Oferece 4/8 direções e idle/walk/run prontos, mas depende de serviço externo com token, ToS restritivo e saída não reproduzível; colide com IV/VI. Rejeitada — no máximo ferramenta opcional de autoria offline, com prompt/seed/ToS no inventário.

## Recomendação (ADAPT)
Opção 1, apoiada pela Opção 2 para os frames. Mantém a afirmação do vídeo como hipótese [NV] e preserva o verificável localmente: a simulação autoritativa continua dirigindo a apresentação, e o adaptador de apresentação consome frames versionados. IA nunca em runtime; gates determinísticos permanecem a autoridade sobre o que entra no repositório.
