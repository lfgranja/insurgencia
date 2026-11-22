# Concept — Verificação determinística de código IA

**Data**: 2026-11-21

## Opções
1. **Formalizar checklist de comprehension debt + revisor separado + `proptest` ancorado em `verify.sh` (recomendada)**: `verify.sh` continua o oráculo; revisor lê só o diff e os invariantes, sem poder alterar código; PBT cobre `contratos`/`sim`. Coerente com ADOPT.
2. Pedir à própria IA para revisar o código que gerou: rejeitada — ciclo de autoengano (PGS, arXiv 2506.18315).
3. Confiar apenas em cobertura de testes / DORA: rejeitada — métricas não medem compreensão (Osmani, Anthropic).

## Decisão (ADOPT)
Opção 1. Maior alavanca local (27,0 no ranking 80/20). Impacto 3, confiança 3, facilidade 3, risco 1 → 27. Handoff: checklist de revisão no gate, agente revisor separado sob `--minimizado`, `proptest` e teste de determinismo em `sim`; exigir explicação do módulo antes de merge.
