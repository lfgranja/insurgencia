# Concept — Métricas de sucesso (100K/3 dias)

**Data**: 2026-11-21

## Opções
1. **Substituir por métricas locais de verificação da tese (GDD §17) (recomendada)**: tempo até gate verde, invariantes verificados, determinismo headless (`hash_estado`); números externos só como contexto com [NV]. Coerente com AVOID da métrica original.
2. Manter 100K/3 dias e 79% como KPI: rejeitada — autodeclarado [NV], sem contrafactual, volátil e alheio à tese.
3. Não medir nada: rejeitada — conflita com verificação honesta (VI).

## Decisão (AVOID)
Opção 1. A métrica do vídeo é descartada como critério; o que entra é o conjunto falsificável do GDD §17. Impacto 2, confiança 3, facilidade 3, risco 1 → 18 (heurística local). Sem handoff de implementação: trata-se de política de medição, não de feature. Registrar a troca em ADR/decisão documental.
