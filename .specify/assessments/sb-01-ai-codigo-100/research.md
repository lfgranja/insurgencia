# Research — Código 100% IA

**Data**: 2026-11-21
**Notas-base**: `docs/research/slotbound-2026-11-21-ai-code.md` (E1–E22, mapeamento ID → impacto), `docs/research/slotbound-2026-11-21-alternativas.md` (ALT-1–6), `docs/research/slotbound-2026-11-21-fit-local.md` (matriz de encaixe).

## Achados
1. A afirmação "100% do código gerado por IA" é autodeclarada no post original de 22/05/2026 e é infalsificável: sem repositório, diff, commits ou auditoria independente. "We can't verify the 100% figure" (https://www.reddit.com/r/aigamedev/comments/1tkeg86/; https://quickmatch.si/blog/slotbound-ai-coded-steam-hit/). [NV — sem acesso ao código]
2. O fluxo real descrito pelo próprio autor é delegação feature a feature com verificação humana: ~6 meses até o post, ~8 até o demo, ~1 ano até o launch; ele precisa que a IA explique o código (https://www.reddit.com/r/aigamedev/comments/1usuhxn/i_still_cant_code_but_after_8_months_the_steam/).
3. Evidência de produtividade é dependente de contexto: GitHub RCT 55% mais rápido em tarefa única (2022); METR 2025 encontrou 19% mais lento em devs experientes; a atualização de 02/2026 considera o sinal não confiável; Merico: ~65% das empresas com ganho ≤20% ou nulo (https://github.blog/news-insights/research/research-quantifying-github-copilots-impact-on-developer-productivity-and-happiness/; https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/; https://metr.org/blog/2026-02-24-uplift-update/; https://www.merico.cn/blog/the-practical-application-effect-of-ai-in-r-d-efficiency).
4. Comprehension debt é o risco melhor documentado: Anthropic (52 devs) 50% vs 67% no quiz, pior padrão delegação total (<40%), melhor "gerar e explicar" (86%); Veracode: 45% das amostras falham segurança (https://www.anthropic.com/research/AI-assistance-coding-skills; https://www.veracode.com/blog/genai-code-security-report/).
5. No repo local, `verify.sh` (check → clippy `-D warnings` → nextest → fmt, `--all-targets`) já é a autoridade determinística; saída de LLM é informativa (constituição I/VI). ADR-004 mantém Bevy+Lightyear; `#![forbid(unsafe_code)]` + `#![deny(clippy::pedantic)]` (local: `verify.sh`, `.specify/memory/constitution.md`). [NV — gates não executados nesta análise]

## URLs primárias
- https://www.reddit.com/r/aigamedev/comments/1tkeg86/ (post original, 22/05/2026)
- https://www.reddit.com/r/aigamedev/comments/1usuhxn/i_still_cant_code_but_after_8_months_the_steam/ (fluxo real, 10/07/2026)
- https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/ · https://metr.org/blog/2026-02-24-uplift-update/
- https://www.anthropic.com/research/AI-assistance-coding-skills
- https://www.veracode.com/blog/genai-code-security-report/
