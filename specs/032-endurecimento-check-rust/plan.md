# Implementation Plan — 032-endurecimento-check-rust

1. Confirmar R1 `go` no veredito de `.specify/assessments/endurecimento-check-rust/decision.md` (scorecard 6 critérios, evidências datadas de 2026-11-21).
2. Materializar os artefatos 032 na fronteira exata: assess (intake/research/problem/concept/decision) → spec → plan → tasks → checklist → analyze (READ-ONLY) → converge (append-only, primeira entrada datada 2026-11-21) → validate → release.
3. Registrar a evidência de re-leitura `2026-11-21T20:24:58Z`: proteção anterior `contexts=["gates"]` (sem `rust`), PR #36 OPEN, `gates pass 9s`, `rust pass 3m48s` — risco de confiança.
4. Registrar a verificação local `20:25:17Z`: `./verify.sh` (`ALL STAGES OK`, 34 testes, `VERIFY_EXIT=0`) e 5 gates Python PASS (SDD `31/0`, DAG `274/0`, `0 duplicatas`, converge `14/14`, hub-docs `0 erros`; factory N/A).
5. Registrar o merge autorizado `20:25:32Z`: `gh pr merge 36 --merge` (`MERGE_EXIT=0`), `state=MERGED`, `mergedAt=2026-11-21T20:25:35Z`, `mergeCommit.oid=ce2d1f0...`, `dev=ce2d1f0`, 0 PRs abertos.
6. Registrar a mutação autorizada `20:25:48Z`: `gh api -X PUT` da proteção de `dev` com `contexts=["gates","rust"]`, `PUT_EXIT=0`, e GET de verificação confirmando `contexts=["gates","rust"]`, `strict=true`, `enforce_admins=false`.
7. Preservar o SDD ordenado (R1 → R4 → R7), `converge.md` append-only (primeira entrada 2026-11-21), `analyze.md` READ-ONLY e checklist `[x]` = revisão de requisitos; manter `[NV]/DEFER` para deploy/publicação e assinatura GPG.
8. Validar (R4 aprovado) em `.specify/validations/endurecimento-check-rust/validate.md` e emitir release (R7) em `.specify/releases/endurecimento-check-rust/release.md`: `1.0.0`, changelog FR-001..FR-006 + T-IDs, rollback documentado (restaurar `contexts=["gates"]` via `gh api -X PUT` com o payload anterior — nunca executar).
9. Encerrar sem build/gate/commit e sem tocar código, factory, 194 paths untracked alheios, janelas do cliente ou credenciais.
