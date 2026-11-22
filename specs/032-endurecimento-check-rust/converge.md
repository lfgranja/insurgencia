# Convergência — 032-endurecimento-check-rust

**Data**: 2026-11-21 · **Modo**: READ-ONLY, append-only. Primeira entrada do ciclo 032.

## Evidências (observadas — comandos executados pelo orquestrador, saídas coladas)

- `2026-11-21T20:24:58Z` — re-leitura ao vivo: `HEAD=fe82843`, branch `031-fechamento-rastreabilidade`, PR #36 `OPEN`; `gh api repos/lfgranja/insurgencia/branches/dev/protection` → `{"contexts":["gates"],"enforce_admins":false,"restrictions":null,"reviews":null,"strict":true}`; `gh pr checks 36` → `gates pass 9s`, `rust pass 3m48s`. `rust` ausente da proteção = risco de confiança.
- `2026-11-21T20:25:17Z` — verificação local: `./verify.sh` → `ALL STAGES OK`, `34 tests run: 34 passed`, `VERIFY_EXIT=0`; 5 gates Python PASS — SDD `spec_dirs=31 gaps=0`, DAG `274/0`, task-id `0 duplicatas`, converge `14/14`, hub-docs `0 erros`; `check_production_gate.py` ausente → exit 2 = N/A (factory é fonte, não autoridade).
- `2026-11-21T20:25:32Z` — merge autorizado: `gh pr merge 36 --merge` → `MERGE_EXIT=0`; `gh pr view 36` → `state=MERGED`, `mergedAt=2026-11-21T20:25:35Z`, `mergeCommit.oid=ce2d1f077b31fd0b6e16cc298ecf1406ca2fc9a2`; `git ls-remote origin refs/heads/dev` = `ce2d1f0...`; 0 PRs abertos.
- `2026-11-21T20:25:48Z` — mutação autorizada: `gh api -X PUT repos/lfgranja/insurgencia/branches/dev/protection --input -` com payload `{"required_status_checks":{"strict":true,"contexts":["gates","rust"]},"enforce_admins":false,"required_pull_request_reviews":null,"restrictions":null}` → resposta `{"contexts":["gates","rust"],"enforce_admins":false,"restrictions":null,"reviews":null,"strict":true}`, `PUT_EXIT=0`; GET de verificação → `{"contexts":["gates","rust"],"enforce_admins":false,"strict":true,"url":"https://api.github.com/repos/lfgranja/insurgencia/branches/dev/protection"}`.
- Autorização explícita do SE nesta sessão: (a) merge para `dev` sempre que CI verde; (b) endurecer a proteção de `dev` com o check `rust`. Deploy/publicação e assinatura GPG permanecem **[NV]/DEFER**.

## Atendimento
FR-001..FR-006 e SC-001..SC-006 atendidos; R1 go → R4 aprovado → R7 `1.0.0`. Risco de confiança fechado: `dev` passou a exigir `gates` **e** `rust`, confirmado por GET em `20:25:48Z`. Merge do PR #36 (`ce2d1f0...`) e mutação da proteção registrados com evidências datadas coladas, sem reexecução. `converge.md` permanece append-only (esta é a primeira entrada, 2026-11-21); `analyze.md` READ-ONLY; checklist `[x]` = revisão de requisitos. Fronteira restrita de 12 artefatos respeitada; nenhum build/gate/commit, nenhuma janela do cliente, nenhuma credencial, nenhum código/factory/194 paths untracked alheios tocado. Rollback documentado (restaurar `contexts=["gates"]` via `gh api -X PUT` com o payload anterior), nunca executado.
