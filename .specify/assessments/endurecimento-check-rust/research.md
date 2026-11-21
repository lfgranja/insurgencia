# Research — Endurecimento do check `rust` na proteção de `dev`

Fontes: execuções e consultas datadas de 2026-11-21 (comandos executados pelo orquestrador, saídas coladas literalmente); estado da proteção de `dev` via `gh api`; CI via `gh pr checks`; autoridade local `./verify.sh` + 5 gates Python.

## Evidências observadas

1. **`2026-11-21T20:24:58Z` — re-leitura ao vivo**: `HEAD=fe82843`, branch `031-fechamento-rastreabilidade`, PR #36 `OPEN`. `gh api repos/lfgranja/insurgencia/branches/dev/protection` → `{"contexts":["gates"],"enforce_admins":false,"restrictions":null,"reviews":null,"strict":true}`. `gh pr checks 36` → `gates pass 9s`, `rust pass 3m48s`. **Achado**: `rust` passa na CI, mas **não é exigido** pela proteção de `dev` — risco de confiança (merge possível com Rust falha).
2. **`2026-11-21T20:25:17Z` — verificação local**: `./verify.sh` → `ALL STAGES OK`, `34 tests run: 34 passed`, `VERIFY_EXIT=0`. 5 gates Python PASS: SDD `spec_dirs=31 gaps=0`, DAG `274/0`, task-id `0 duplicatas`, converge `14/14`, hub-docs `0 erros`. `check_production_gate.py` da factory ausente → exit 2 = N/A (factory é fonte, não autoridade).
3. **`2026-11-21T20:25:32Z` — merge autorizado**: `gh pr merge 36 --merge` → `MERGE_EXIT=0`. `gh pr view 36` → `state=MERGED`, `mergedAt=2026-11-21T20:25:35Z`, `mergeCommit.oid=ce2d1f077b31fd0b6e16cc298ecf1406ca2fc9a2`. `git ls-remote origin refs/heads/dev` = `ce2d1f0...`. 0 PRs abertos. Base legal: autorização do SE para merge para `dev` sempre que CI verde.
4. **`2026-11-21T20:25:48Z` — mutação autorizada**: `gh api -X PUT repos/lfgranja/insurgencia/branches/dev/protection --input -` com payload `{"required_status_checks":{"strict":true,"contexts":["gates","rust"]},"enforce_admins":false,"required_pull_request_reviews":null,"restrictions":null}` → resposta `{"contexts":["gates","rust"],"enforce_admins":false,"restrictions":null,"reviews":null,"strict":true}`, `PUT_EXIT=0`. GET de verificação → `{"contexts":["gates","rust"],"enforce_admins":false,"strict":true,"url":"https://api.github.com/repos/lfgranja/insurgencia/branches/dev/protection"}`. Risco de confiança fechado.
5. **Autorização e limites**: o SE autorizou explicitamente (a) merge para `dev` com CI verde e (b) endurecimento da proteção com `rust`. Deploy/publicação e assinatura GPG permanecem **[NV]/DEFER** — nenhuma evidência de execução, nenhuma afirmação de feito.

## Síntese

Todas as evidências necessárias já estão coletadas e datadas; o trabalho do ciclo 032 é de registro auditável e governança de proteção de branch, não de implementação de código. O merge (#36 → `ce2d1f0`) e a mutação da proteção já ocorreram sob autorização; os artefatos apenas os documentam. Nenhuma reexecução, nenhuma credencial, nenhuma janela.
