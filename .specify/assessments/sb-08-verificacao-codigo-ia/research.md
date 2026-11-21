# Research — Verificação determinística de código IA

**Data**: 2026-11-21
**Notas-base**: `docs/research/slotbound-2026-11-21-ai-code.md` (achados 4, 8, 9; E8–E10, E20–E22), `docs/research/slotbound-2026-11-21-fit-local.md` (achado 6; sb-08 = alvo estrutural).

## Achados
1. Comprehension debt não aparece em DORA/velocity: Anthropic (52 devs) 50% vs 67% no quiz, pior dano em debugging; pior padrão delegação total (<40%), melhor "gerar e explicar" (86%) (https://www.anthropic.com/research/AI-assistance-coding-skills; https://addyosmani.com/blog/comprehension-debt).
2. Testes são necessários, mas insuficientes: revisão por spec não captura decisões implícitas; o gargalo desloca-se de escrever para auditar (https://addyosmani.com/blog/comprehension-debt).
3. Rust é caso favorável a oráculos externos: CRUST-Bench (arXiv 2504.15254) — melhor modelo resolve 19/100 no primeiro disparo; com compilador+testes sobe a 48%; instruções explícitas ("must compile", proibir `unsafe`) reduzem erro de unsafe a quase zero (https://arxiv.org/html/2504.15254).
4. Separar geração de validação vence "pedir para a IA revisar": PGS (arXiv 2506.18315) mostra ciclo de autoengano do mesmo modelo; agente Tester separado com PBT melhora pass@1 em 23,1–37,3% sobre TDD (https://arxiv.org/html/2506.18315v1).
5. Local: `verify.sh` (check → clippy `-D warnings` → nextest → fmt, `--all-targets`) + 5 gates + 34 testes + `avancar_tick`/`hash_estado` determinísticos já são a autoridade; falta formalizar revisão separada, `proptest` e exigência de explicação (local: `verify.sh`, `crates/sim/src/lib.rs`). [NV — gates não executados nesta análise]

## URLs primárias
- https://www.anthropic.com/research/AI-assistance-coding-skills
- https://addyosmani.com/blog/comprehension-debt/
- https://arxiv.org/html/2504.15254 (CRUST-Bench)
- https://arxiv.org/html/2506.18315v1 (Property-Generated Solver)
