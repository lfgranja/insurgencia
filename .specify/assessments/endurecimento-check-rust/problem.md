# Problem — Endurecimento do check `rust` na proteção de `dev`

**Problema**: a proteção da branch `dev` exigia apenas o check `gates` (`{"contexts":["gates"],...,"strict":true}`, observado em `2026-11-21T20:24:58Z`) e **não** o check `rust`. Como a CI Rust (`rust pass 3m48s`) é informativa e não obrigatória, um PR podia ser mesclado em `dev` com a suíte Rust falha — risco de confiança na integração. Além disso, o merge do PR #36 e a mutação corretiva da proteção precisam ficar registrados de forma datada e auditável, sem reexecução e sem extrapolar a autorização do SE.

**Sucesso observável**: proteção de `dev` com `contexts=["gates","rust"]` confirmada por GET em `2026-11-21T20:25:48Z`; merge do PR #36 registrado (`MERGE_EXIT=0`, `mergeCommit ce2d1f0...`, 0 PRs abertos); evidências datadas coladas nos artefatos 032; SDD ordenado R1 → R4 → R7, `converge.md` append-only, `analyze.md` READ-ONLY, rollback documentado (nunca executado); deploy/publicação e assinatura GPG mantidos [NV]/DEFER.

**Custo de não agir**: `dev` continuaria aceitando merges com Rust vermelha, minando a verificação honesta (Constituição I/VI); sem registro datado, a cadeia evidência→decisão→mutação fica quebrada e um auditor não consegue provar quem autorizou o quê e quando.
