# Concept — HUD glyphs ausentes

**Data**: 2026-11-21T04:59Z

## Opções
1. **Substituir caracteres por ASCII seguro** (recomendada): `Ê`→`E`, `—`→`-`, `·`→`/`. Uma linha em `main.rs:458` e outra em `:555`. Sem assets, sem risco, reversível.
2. Adicionar fonte TTF: exige asset externo e licença auditada — rejeitada (GDD §13.2, allowlist).
3. Ignorar: degrada identidade visual — rejeitada.

## Decisão (R1: go)
Opção 1. Impacto 3, confiança 3 (evidência visual), facilidade 3, risco 1 → score 27. Handoff: alterar exatamente as duas strings do HUD; preservar lógica, testes e regra da janela.
