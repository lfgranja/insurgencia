# Tasks — 035-advisories-acr-rastreabilidade

- [x] T001 branch `035-advisories-acr-rastreabilidade` (base `f9d87e0`), stub `spec.md` e `.specify/feature.json → specs/035`
- [x] T002 assess.intake — ideia, âncora e evidências de `18:31Z–18:34Z` e `18:32:17Z` — Bloqueado por: T001
- [x] T003 assess.research — 10 evidências com `[source]` e grau de confiança — Bloqueado por: T002
- [x] T004 assess.problem — problema, sucesso observável, custo de inação e fora de escopo — Bloqueado por: T003
- [x] T005 assess.shape (concept) — 4 opções e recomendação de correção na fonte + rastreabilidade — Bloqueado por: T004
- [x] T006 assess.decide (go) — scorecard 6/6 strong, 80/20 = 400/625, condições e handoff — Bloqueado por: T005
- [x] T007 specify — substituir stub por FR-001..FR-008 e SC-001..SC-005 — Bloqueado por: T006
- [x] T008 clarify — fechar ≠ redefinir contrato; `.gitignore` legítimo; hash canônico minúsculo — Bloqueado por: T007
- [x] T009 plan — passos na ordem normativa, com `analyze` antes de `tasks` — Bloqueado por: T008
- [x] T010 checklist de requisitos — um item por FR — Bloqueado por: T009
- [x] T011 revisar checklist — A3 corrigido na fonte; aprovado para `analyze` — Bloqueado por: T010
- [x] T012 analyze (READ-ONLY) — A1..A8, convergência e veredito PASS, **antes de `tasks`** — Bloqueado por: T011
- [x] T013 tasks — T-IDs únicos e DAG acíclica — Bloqueado por: T012
- [x] T014 test-roadmap — escopo/FR/T, pré-condições, passos numerados, superfície CLI/docs, capturas, `automatizável?`, rollback e passo `content_hash` maiúsculo → hash minúsculo — Bloqueado por: T013
- [x] T015 converge — primeira entrada append-only datada `2026-11-22` (sem tocar `converge.md` de ciclos anteriores) — Bloqueado por: T014
- [x] T016 validate (R4 aprovado, documental) — referência ao roteiro, pendências **[NV]** — Bloqueado por: T015
- [x] T017 release (R7 `1.0.0`) — changelog FR/T, rastreabilidade e rollback — Bloqueado por: T016
- [ ] T018 implement (agente de código/orquestrador, **fora da fronteira SDD**) — aplicar FR-002/FR-003/FR-004 (STATE.md, evidence.md, afirmação de `.gitignore` em 034) e FR-005 (imprimir `hash_regras` minúsculo em `COMPAT_OK`); executar o roteiro e rodar gates/CI — Bloqueado por: T017 — **[NV]**

## Dependências
DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012 → T013 → T014 → T015 → T016 → T017 → T018. Nenhuma auto-aresta, aresta órfã ou ciclo. Ordem de execução estável por sufixo numérico. `analyze` (T012) vem antes de `tasks` (T013); R4 (T016) vem antes de R7 (T017).

## Analyze (READ-ONLY, 2026-11-22T18:34:30Z)
Conflitos: nenhum entre spec, plano e checklist após A1–A4 terem sido corrigidos nas fontes. A autoridade é `conferir` (011/033); "aceitar maiúsculas" é do contrato (`eq_ignore_ascii_case`), "imprimir minúsculas" é do adaptador (`hash_regras`). As quatro divergências são corrigidas na fonte, nunca na expectativa; a regra `/.specify/queue/` do `.gitignore` é preservada. Nenhuma reexecução de gates/build/`verify.sh`: evidências de `18:32:17Z` e PR #40 `OPEN` coladas como observadas, sem merge em `main`. `[x]` em checklist = revisão de requisitos; `[x]` em T001–T017 registra a **escrita dos artefatos SDD**, não aplicação de correções ou execução de build/teste/gate. T018 está **[NV]** e pertence ao agente de código/orquestrador. `converge.md` é append-only com primeira entrada em `2026-11-22`. R4 precede R7; rollback documentado. Factory `b1deab1` somente fonte; 194 paths untracked alheios intactos.
