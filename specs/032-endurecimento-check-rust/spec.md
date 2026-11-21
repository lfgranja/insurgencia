# Feature Specification: Endurecimento do check `rust` na proteção de `dev`

**Branch**: `032-endurecimento-check-rust` (base `ce2d1f0` = `origin/dev`) · **Created**: 2026-11-21 · **Status**: Draft · **R1**: go

## User Scenarios
### US1 — Proteção de `dev` exige `rust` (P1)
Um merge para `dev` só é permitido quando os checks obrigatórios `gates` **e** `rust` passam; o check `rust` deixa de ser meramente informativo.

**Acceptance Scenarios**: 1) antes (`2026-11-21T20:24:58Z`) a proteção de `dev` era `{"contexts":["gates"],"enforce_admins":false,"restrictions":null,"reviews":null,"strict":true}` — `rust` ausente (risco de confiança); 2) depois (`2026-11-21T20:25:48Z`) um `gh api -X PUT` autorizado resultou em `{"contexts":["gates","rust"],"enforce_admins":false,"restrictions":null,"reviews":null,"strict":true}`; 3) GET de verificação confirmou `contexts=["gates","rust"]`, `enforce_admins=false`, `strict=true`.

### US2 — Evidências datadas coladas (P1)
Toda afirmação de "CI verde", merge ou mutação remota vem acompanhada de comando executado e saída colada, com timestamp — nenhuma reexecução, nenhuma afirmação sem evidência.

**Acceptance Scenarios**: 1) re-leitura ao vivo em `20:24:58Z` (PR #36 OPEN, `gates pass 9s`, `rust pass 3m48s`); 2) `./verify.sh` e 5 gates em `20:25:17Z` (`ALL STAGES OK`, 34 testes, `VERIFY_EXIT=0`, converge `14/14`); 3) merge em `20:25:32Z` (`MERGE_EXIT=0`, `mergeCommit.oid=ce2d1f0...`); 4) mutação em `20:25:48Z` (`PUT_EXIT=0`).

### US3 — SDD ordenado, histórico preservado e rollback documentado (P2)
O ciclo segue R1 → R4 → R7, preserva `converge.md` append-only, mantém a fronteira restrita de 12 artefatos e registra rollback sem executá-lo.

**Acceptance Scenarios**: 1) primeira entrada datada `2026-11-21` em `converge.md` (append-only, sem linhas antigas); 2) `analyze.md` READ-ONLY e checklist `[x]` = revisão de requisitos, não implementação; 3) release `1.0.0` com changelog FR/T e rollback documentado (restaurar `contexts=["gates"]` via `gh api -X PUT`, nunca executar); 4) deploy/publicação e assinatura GPG permanecem [NV]/DEFER.

## Requirements
- **FR-001**: registrar como evidência datada `2026-11-21T20:24:58Z` o estado anterior: `HEAD=fe82843`, branch `031-fechamento-rastreabilidade`, PR #36 `OPEN`; `gh api repos/lfgranja/insurgencia/branches/dev/protection` → `{"contexts":["gates"],"enforce_admins":false,"restrictions":null,"reviews":null,"strict":true}`; `gh pr checks 36` → `gates pass 9s`, `rust pass 3m48s`. `rust` não era obrigatório = risco de confiança a fechar.
- **FR-002**: registrar como evidência datada `2026-11-21T20:25:17Z`: `./verify.sh` → `ALL STAGES OK`, `34 tests run: 34 passed`, `VERIFY_EXIT=0`; 5 gates Python PASS (SDD `spec_dirs=31 gaps=0`, DAG `274/0`, task-id `0 duplicatas`, converge `14/14`, hub-docs `0 erros`); `check_production_gate.py` ausente → exit 2 = N/A (factory é fonte, não autoridade).
- **FR-003**: registrar como evidência datada `2026-11-21T20:25:32Z` o merge autorizado do PR #36: `gh pr merge 36 --merge` → `MERGE_EXIT=0`; `gh pr view 36` → `state=MERGED`, `mergedAt=2026-11-21T20:25:35Z`, `mergeCommit.oid=ce2d1f077b31fd0b6e16cc298ecf1406ca2fc9a2`; `git ls-remote origin refs/heads/dev` = `ce2d1f0...`; 0 PRs abertos. Base legal: autorização do SE para merge para `dev` sempre que CI verde.
- **FR-004**: registrar como evidência datada `2026-11-21T20:25:48Z` a mutação autorizada: `gh api -X PUT repos/lfgranja/insurgencia/branches/dev/protection --input -` com payload `{"required_status_checks":{"strict":true,"contexts":["gates","rust"]},"enforce_admins":false,"required_pull_request_reviews":null,"restrictions":null}` → resposta `{"contexts":["gates","rust"],"enforce_admins":false,"restrictions":null,"reviews":null,"strict":true}`, `PUT_EXIT=0`; GET de verificação → `{"contexts":["gates","rust"],"enforce_admins":false,"strict":true,"url":"https://api.github.com/repos/lfgranja/insurgencia/branches/dev/protection"}`. Risco de confiança fechado.
- **FR-005**: preservar SDD ordenado R1 `go` → R4 → R7; `converge.md` append-only (primeira entrada, datada `2026-11-21`); `analyze.md` READ-ONLY; checklist `[x]` = revisão de requisitos, não implementação; escrever apenas os 12 artefatos da fronteira, sem tocar código, factory, 194 paths untracked alheios, credenciais ou janelas; manter deploy/publicação e assinatura GPG como [NV]/DEFER.
- **FR-006**: emitir release R7 `1.0.0` (SemVer) com changelog FR-001..FR-006 + T-IDs e rollback documentado: restaurar `contexts=["gates"]` via `gh api -X PUT repos/lfgranja/insurgencia/branches/dev/protection` com o payload anterior `{"required_status_checks":{"strict":true,"contexts":["gates"]},"enforce_admins":false,"required_pull_request_reviews":null,"restrictions":null}` — nunca executar, apenas documentar.

## Success Criteria
SC-001: proteção de `dev` com `contexts=["gates","rust"]`, `strict=true`, `enforce_admins=false` confirmada por GET em `20:25:48Z`. SC-002: PR #36 mesclado em `dev` (`mergeCommit ce2d1f0...`), 0 PRs abertos, checks `gates`/`rust` pass. SC-003: evidências datadas coladas (`20:24:58Z`, `20:25:17Z`, `20:25:32Z`, `20:25:48Z`) com comandos e saídas literais. SC-004: `./verify.sh` `ALL STAGES OK`/34 testes/`VERIFY_EXIT=0` e 5 gates PASS (converge `14/14`); factory N/A. SC-005: SDD ordenado R1 → R4 → R7, `converge.md` append-only (primeira entrada 2026-11-21), `analyze.md` READ-ONLY, checklist = revisão. SC-006: release `1.0.0` com changelog FR/T e rollback documentado (restaurar `contexts=["gates"]`, nunca executar); nenhum build/gate/commit; fronteira de 12 artefatos respeitada; [NV]/DEFER preservado.
