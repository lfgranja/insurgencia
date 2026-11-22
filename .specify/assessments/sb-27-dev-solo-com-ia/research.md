# Research — Dev solo com IA

**Data**: 2026-11-21
**Notas-base**: `docs/research/slotbound-2026-11-21-ai-code.md` (E1/E2/E5/E8/E15/E16), `docs/research/slotbound-2026-11-21-alternativas.md` (ALT-1/ALT-2/ALT-3–6, DORA amplificador), `docs/research/slotbound-2026-11-21-fit-local.md` (GDD §3.2).

## Achados
1. O caso Slotbound é n=1: dev que não sabia programar, ~6–12 meses de trabalho intenso, sprites comprados e nenhum teste automatizado/e2e (E1/E2). [NV — relato autodeclarado, sem auditoria]
2. O próprio autor atribui o resultado a design/teste/descarte de ideias, não à IA decidir o que é divertido (https://www.reddit.com/r/aigamedev/comments/1usuhxn/i_still_cant_code_but_after_8_months_the_steam/).
3. Iniciante + IA sem compreensão gera dívida: Anthropic (majoritariamente junior) 50% vs 67%; pior padrão delegação total, melhor "gerar e explicar" (86%) (https://www.anthropic.com/research/AI-assistance-coding-skills).
4. Viés de sobrevivência: censo de 53.597 jogos — topo 1% ≈94% da receita, 28% sem avaliações, jogos com IA atingem ≥100 avaliações a ~55% da taxa dos sem IA; 87% são não-performers (https://fragwyz.substack.com/p/three-years-of-ai-on-steam; https://automaton-media.com/articles/newsjp/20260729-457268).
5. Fit local: público já definido no GDD §3.2 (engenheiros sem estúdio); IA só reduz barreira de entrada. Ferramentas open-source (Aider Apache-2.0, OpenCode MIT) evitam provedor no caminho crítico (IV); Cursor AVOID por Data Sharing (https://github.com/aider-ai/aider; https://github.com/anomalyco/opencode). [NV — gates não executados nesta análise]

## URLs primárias
- https://www.reddit.com/r/aigamedev/comments/1tkeg86/ (post original, 22/05/2026)
- https://www.reddit.com/r/aigamedev/comments/1usuhxn/i_still_cant_code_but_after_8_months_the_steam/ (10/07/2026)
- https://www.anthropic.com/research/AI-assistance-coding-skills
- https://fragwyz.substack.com/p/three-years-of-ai-on-steam (censo, 13/07/2026)
- https://github.com/aider-ai/aider · https://github.com/anomalyco/opencode
