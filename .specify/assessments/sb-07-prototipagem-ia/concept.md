# Concept — Prototipagem rápida com IA

**Data**: 2026-11-21

## Opções
1. **Protótipo como insumo descartável (recomendada)**: IA gera rascunho em modo exploração (OpenCode `plan`/Aider opcional); o que sobrevive passa por `verify.sh` e revisão humana. Nada de janela normal; ADR-004 intacta. Coerente com ADAPT.
2. Protótipo como produto final: rejeitada — sem gates vira dívida de compreensão e ignora a desaceleração pós-rascunho documentada.
3. Proibir prototipagem: rejeitada — descarta o ganho real no primeiro rascunho e empobrece o fluxo SDD.

## Decisão (ADAPT)
Opção 1. A IA acelera o primeiro rascunho; design, balanceamento e legibilidade permanecem decisões humanas. Impacto 3, confiança 3, facilidade 3, risco 1 → 27 (topo da matriz de encaixe local, empatado com sb-08). Handoff: formalizar "protótipo informativo, nunca artefato final sem gates" no fluxo SDD.
