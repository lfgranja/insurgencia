# Research — Prototipagem rápida com IA

**Data**: 2026-11-21
**Notas-base**: `docs/research/slotbound-2026-11-21-ai-code.md` (achado 6; E1/E2/E5), `docs/research/slotbound-2026-11-21-alternativas.md` (ALT-1/ALT-2, modo `plan`), `docs/research/slotbound-2026-11-21-fit-local.md` (sb-07 = prática corrente no SDD).

## Achados
1. Prototipagem rápida é plausível, mas "produto em 3 dias" não é o que a fonte diz: o autor levou ~6 meses até o post, ~8 até o demo e ~1 ano até o launch, e relatou desaceleração ao crescerem sistemas e decisões de design ("70% problem") (https://www.reddit.com/r/aigamedev/comments/1tkeg86/; https://addyosmani.com/blog/comprehension-debt).
2. O próprio autor separa geração de decisão de produto: "AI couldn't decide whether the game was fun, balanced, or easy to understand" (https://www.reddit.com/r/aigamedev/comments/1usuhxn/i_still_cant_code_but_after_8_months_the_steam/).
3. No repo, a prototipagem já é prática do fluxo SDD: specs e artefatos são insumos informativos; LLM nunca roteia estado — `avancar_tick`/`hash_estado` em `crates/sim/src/lib.rs` são determinísticos (local: `fit-local.md` achado 6; `verify.sh`).
4. Ferramentas com bom fit: OpenCode (MIT) modo `plan` somente-leitura para explorar código e Aider (Apache-2.0) com lint/test automático; Cursor AVOID por lock-in e Data Sharing default on (https://github.com/anomalyco/opencode; https://github.com/aider-ai/aider). [NV — qualidade em Rust/Bevy com modelos locais não medida]
5. A regra `--minimizado` (AGENTS.md) e ADR-004 (manter Bevy+Lightyear) limitam o protótipo: nada de janela normal e nada de troca de engine (local: `AGENTS.md`). [NV — gates não executados nesta análise]

## URLs primárias
- https://www.reddit.com/r/aigamedev/comments/1tkeg86/ (post original, 22/05/2026)
- https://www.reddit.com/r/aigamedev/comments/1usuhxn/i_still_cant_code_but_after_8_months_the_steam/ (decisão de produto é humana, 10/07/2026)
- https://addyosmani.com/blog/comprehension-debt/ ("70% problem")
- https://github.com/anomalyco/opencode · https://github.com/aider-ai/aider
