# Release (R7) — 032-endurecimento-check-rust

**Data:** 2026-11-21 · **Versão:** `1.0.0` (SemVer) · **R4:** aprovado

## Changelog
### [1.0.0] — 2026-11-21
### Added
- FR-001 / T005 — evidência datada `2026-11-21T20:24:58Z`: `HEAD=fe82843`, PR #36 `OPEN`, proteção de `dev` `contexts=["gates"]` sem `rust` (risco de confiança); `gates pass 9s`, `rust pass 3m48s`.
- FR-002 / T006 — evidência datada `20:25:17Z`: `./verify.sh` `ALL STAGES OK`/34 testes/`VERIFY_EXIT=0`; 5 gates Python PASS (SDD `31/0`, DAG `274/0`, `0 duplicatas`, converge `14/14`, hub-docs `0 erros`); factory N/A.
- FR-003 / T007 — evidência datada `20:25:32Z`: merge autorizado do PR #36 (`MERGE_EXIT=0`, `state=MERGED`, `mergedAt=20:25:35Z`, `mergeCommit ce2d1f0...`, 0 PRs abertos).
- FR-004 / T008 — evidência datada `20:25:48Z`: mutação autorizada via `gh api -X PUT` → proteção de `dev` com `contexts=["gates","rust"]`, `PUT_EXIT=0`, GET confirmado. Risco de confiança fechado.
- FR-005 / T009, T010 — SDD ordenado R1 go → R4 → R7; `converge.md` append-only (1ª entrada 2026-11-21); `analyze.md` READ-ONLY; fronteira restrita de 12 artefatos; deploy/publicação e assinatura GPG [NV]/DEFER.
- FR-006 / T011 — release `1.0.0` com este changelog e rollback documentado; nenhum build/gate/commit.

## Rastreabilidade
FR-001..FR-006 · T001..T011 · SC-001..SC-006 · R1 go (`.specify/assessments/endurecimento-check-rust/decision.md`) · R4 `.specify/validations/endurecimento-check-rust/validate.md`.

## Rollback
Para reverter a proteção de `dev` ao estado anterior ao ciclo 032, restaurar `contexts=["gates"]` executando (apenas se autorizado — **nunca executar neste ciclo, apenas documentar**):

```bash
gh api -X PUT repos/lfgranja/insurgencia/branches/dev/protection --input - <<'EOF'
{"required_status_checks":{"strict":true,"contexts":["gates"]},"enforce_admins":false,"required_pull_request_reviews":null,"restrictions":null}
EOF
```

O payload anterior é o que produziu a resposta observada em `2026-11-21T20:24:58Z`: `{"contexts":["gates"],"enforce_admins":false,"restrictions":null,"reviews":null,"strict":true}`. O merge do PR #36 (`ce2d1f0...`) permanece em `dev`; desfazê-lo exigiria `git revert` do merge commit, fora desta documentação. Nenhuma baseline, gate, contrato, simulação, CI, constituição ou trabalho alheio é afetado por este rollback. Não houve build, gate ou commit neste ciclo.

## Não incluído
Código, factory, 194 paths untracked alheios, deploy/publicação, assinatura GPG ([NV]/DEFER), janelas do cliente, credenciais, reexecução de gates ou de comandos `gh` além das evidências já coladas.
