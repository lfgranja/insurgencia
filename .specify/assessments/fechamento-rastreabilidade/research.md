# Research — Fechamento de rastreabilidade (ciclos 028–030)

Fontes: execuções e consultas datadas de 2026-11-21 (comandos executados pelo orquestrador); artefatos dos ciclos 028–030; `verify.sh`; `AGENTS.md`.

## Evidências observadas

1. **`2026-11-21T13:21Z` — `./verify.sh`**: `stage 1 ok (62s)`, `stage 2 ok (160s)`, `34 tests run: 34 passed`, `stage 4 ok (5s)`, `ALL STAGES OK`, `VERIFY_EXIT=0`. Quatro estágios: check → clippy → nextest → fmt. Única autoridade local de "verde".
2. **`2026-11-21T13:18:27Z` — 5 gates Python PASS**: SDD `spec_dirs=30 gaps=0`; DAG `262 tarefas/0 ciclos`; task-id `0 duplicatas`; converge `13/13`; hub-docs `0 erros`. `check_production_gate.py` da factory está ausente → exit 2 = N/A (Bloco 0.a).
3. **`2026-11-21T13:19:29Z` — `gh` ao vivo**: repositório público, default `main`; **0 PRs abertos**; 5 runs `deterministic-gates` `success`; proteção de `dev` com required check `gates`, `enforce_admins=false`, sem reviews; `main` desprotegido (404). `git ls-remote origin refs/heads/dev` = `3709c56` = HEAD local.
4. **`2026-11-21T13:19Z` — sintaxe do comando**: `grep` em `verify.sh:5-15,70,75` confirma que **não há flag `--all-targets`**; o script usa `--all-targets` internamente. A expressão "`verify.sh --all-targets`" em artefatos anteriores está **sintaticamente errada** → corrigir para "`./verify.sh` (que executa `--all-targets` internamente)".
5. **Lightyear**: `grep -rni lightyear Cargo.toml Cargo.lock crates/ .github/` → **nenhuma ocorrência**. Lightyear 0.31 aparece apenas em ADR-004/docs, ausente das manifestações.
6. **Tarefas herdadas**: `specs/029-hud-glyphs/tasks.md` T004–T008 `[ ]` embora PR #35 esteja mesclado (`970af4e`, HUD ASCII, CI success run `44869231266`, confirmação visual da SE em STATE 04:33Z); `specs/028-ci-rust-governanca/tasks.md` T011 `[ ]` embora PR #34 mesclado (`d2b23e4`, CI success run `44868073016`).
7. **Factory**: fonte HEAD `b1deab1`; nada transplantado. Gate autoritativo do alvo = `./verify.sh` + 5 gates Python; CI GitHub Actions, CLI `gh`.
