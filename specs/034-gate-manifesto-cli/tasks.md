# Tasks — 034-gate-manifesto-cli

- [x] T001 branch `034-gate-manifesto-cli` (base `7329c53`) e stub `spec.md`
- [x] T002 assess.intake — registrar ideia, âncora e evidências de `2026-11-22T16:54:17Z` — Bloqueado por: T001
- [x] T003 assess.research — fontes lidas, 10 evidências com [source] e grau de confiança — Bloqueado por: T002
- [x] T004 assess.problem — problema, sucesso observável, custo de inação e fora de escopo — Bloqueado por: T003
- [x] T005 assess.shape (concept) — 4 opções e recomendação de adaptador fino — Bloqueado por: T004
- [x] T006 assess.decide (go) — scorecard 6/6 strong, condições e handoff — Bloqueado por: T005
- [x] T007 specify — substituir stub por FR-001..FR-006 e SC-001..SC-006 — Bloqueado por: T006
- [x] T008 clarify — manifesto inválido vs. caminho inexistente; regras ausentes como domínio — Bloqueado por: T007
- [x] T009 plan — 12 passos na ordem normativa — Bloqueado por: T008
- [x] T010 checklist de requisitos — um item por FR — Bloqueado por: T009
- [x] T011 revisar checklist — conflito A3 corrigido na fonte; aprovado para analyze — Bloqueado por: T010
- [x] T012 analyze (READ-ONLY) — A1..A8, convergência e veredito PASS — Bloqueado por: T011
- [x] T013 tasks — T-IDs únicos e DAG acíclica — Bloqueado por: T012
- [x] T014 implement — `crates/sim/examples/verificar_manifesto.rs` (adaptador I/O, SHA-256, `conferir`, exit 0/1/2) — Bloqueado por: T013
- [x] T015 test-roadmap — 8 casos, capturas, `automatizável?` e rollback — Bloqueado por: T014
- [x] T016 converge — primeira entrada append-only datada `2026-11-22` — Bloqueado por: T015
- [x] T017 validate (R4 aprovado) — referência ao roteiro, pendências **[NV]** — Bloqueado por: T016
- [x] T018 release (R7 `1.0.0`) — changelog FR/T, rastreabilidade e rollback — Bloqueado por: T017

## Dependências
A DAG é linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012 → T013 → T014 → T015 → T016 → T017 → T018. Nenhuma auto-aresta, aresta órfã ou ciclo. Ordem de execução estável por sufixo numérico.

## Analyze (READ-ONLY, 2026-11-22T16:54:17Z)
Conflitos: nenhum entre spec, plano, checklist e tasks após A1–A4 terem sido corrigidos nas fontes. A autoridade é `conferir` (011/033); o CLI não inventa esquema, ordem nem código. `NotFound` em regras → `None` → `REGRAS_AUSENTES`; demais leituras falhas → exit 2. Nenhuma reexecução de gates/build/`verify.sh`: as evidências de `16:54:17Z` e a CI run `44952738240` são coladas como observadas. O baseline de converge do arquivo novo não foi atualizado (fora de fronteira) — cabe ao orquestrador. `converge.md` é append-only: primeira entrada em `2026-11-22`. `[x]` em checklist = revisão de requisitos; `[x]` em tasks registra a conclusão documental e de escrita do arquivo de exemplo, **não** execução de build/teste/gate. R4 precede R7; rollback documentado, nunca executado. Nenhum commit, push ou PR.
