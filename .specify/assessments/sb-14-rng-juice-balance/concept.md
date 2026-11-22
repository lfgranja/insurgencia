# Concept — RNG, juice e balance (sb-14)

**Data**: 2026-11-21

## Opções
1. **RNG semeado, positivo e amostrável + juice subordinado à clareza causal (recomendada).** Manter `fator`/`misturar`/`hash_estado`, registrar a semente por execução, amostrar antes da decisão, publicar odds no HUD e exigir que cada efeito de feedback corresponda a uma causa da sim. Custo baixo: expor o que já existe e medir. Coerente com ADAPT.
2. **Adotar "mais RNG" e loop viciante como direção de design.** Maximiza variedade e espetáculo, mas importa near-miss/odds opacas (Lost Garden), corrói agência (TFT Set 1) e colide com constituição I/VI. Rejeitada.
3. **Perseguir juice de produção (partículas/áudio) antes de haver causalidade legível.** O abstract de "Juice It or Lose It" apoia o princípio, mas sem telegrafia o juice vira ruído; exige assets e janela para avaliar [NV]. Rejeitada neste ciclo.

## Recomendação (ADAPT)
Opção 1. A afirmação do vídeo é hipótese; o que é verificável localmente é o determinismo existente. O adaptador de apresentação mostra semente/odds/causa; a sim permanece autoritativa. Medição por diff de `hash_estado` com semente fixa e inspeção do HUD sob `--minimizado`, nunca por opinião estética.
