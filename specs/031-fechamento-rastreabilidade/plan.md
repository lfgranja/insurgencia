# Implementation Plan — 031-fechamento-rastreabilidade

1. Confirmar R1 `go` no veredito de `.specify/assessments/fechamento-rastreabilidade/decision.md` (scorecard 6 critérios, evidências datadas de 2026-11-21).
2. Materializar os artefatos 031 na fronteira A: assess (intake/research/problem/concept/decision) → spec → plan → tasks → checklist → analyze (READ-ONLY) → converge (append-only, data 2026-11-21).
3. Na fronteira B, fechar a rastreabilidade herdada: marcar T004–T008 (029) e T011 (028) `[x]` e acrescentar notas datadas `2026-11-21T13:23Z` com PRs #35/#34, CI runs e STATE 04:33Z — sem reescrever a Analyze existente.
4. Corrigir a errata de redação nos 8 arquivos designados: "`verify.sh --all-targets`" → "`./verify.sh` (que executa `--all-targets` internamente)", conforme `grep` em `verify.sh:5-15,70,75` (13:19Z).
5. Fazer append datado em `specs/030-demo-deterministica-headless/converge.md` e `docs/PROGRESSO.md` — nenhuma linha antiga editada; `converge.md` permanece append-only.
6. Ajustar somente a frase de Lightyear em `AGENTS.md` para "declarado em ADR-004; ausente de `Cargo.toml`/`Cargo.lock` (verificado 21/11)".
7. Colar as evidências datadas como autoridade de estado: `./verify.sh` 13:21Z (`ALL STAGES OK`, 34 testes, `VERIFY_EXIT=0`), 5 gates 13:18:27Z, `gh` 13:19:29Z (0 PRs, 5 runs success, proteção de `dev`, `dev`=HEAD=`3709c56`), factory N/A (Bloco 0.a).
8. Validar (R4 aprovado) em `.specify/validations/fechamento-rastreabilidade/validate.md` e emitir release (R7) em `.specify/releases/fechamento-rastreabilidade/release.md`: `1.0.0`, changelog FR-001..FR-006 + T-IDs, rollback documentado.
9. Encerrar sem build/gate/commit e sem tocar factory, `sb-*`, `docs/OBJETIVO.md`, `docs/ROADMAP.md`, `docs/research/index.md`, janelas do cliente ou credenciais.
