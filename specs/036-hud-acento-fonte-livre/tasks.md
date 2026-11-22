# Tasks — 036-hud-acento-fonte-livre

- [x] T001 branch `036-hud-acento-fonte-livre` (base `7c5c0f3`), stub `spec.md` no worktree isolado — Bloqueado por: nenhum
- [x] T002 assess.intake — pedido do SWE, evidências de `21:15–21:16Z`, regra da janela — Bloqueado por: T001
- [x] T003 assess.research — código, captura Xvfb, TTF/`fc-match`, OFL 1.1, factory e gates — Bloqueado por: T002
- [x] T004 assess.problem — tofu vs. grafia correta, sucesso observável, custo de inação — Bloqueado por: T003
- [x] T005 assess.shape (concept) — 5 opções, recomendação de fonte embarcada, 80/20 = 240/625 — Bloqueado por: T004
- [x] T006 assess.decide (go) — scorecard 6/6 strong, `Veredito: go`, R3 = 0, handoff — Bloqueado por: T005
- [x] T007 specify — substituir stub por US1–US4, FR-001..FR-009, SC-001..SC-006 — Bloqueado por: T006
- [x] T008 clarify — escolha do SWE revoga o paliativo 029 nos três glifos; fonte local, sem deps; seção documental — Bloqueado por: T007
- [x] T009 plan — passos na ordem normativa, `analyze` antes de `tasks`, R4 antes de R7 — Bloqueado por: T008
- [x] T010 checklist de requisitos — um item por FR-001..FR-009 — Bloqueado por: T009
- [x] T011 revisar checklist — conflito 029 vs. pedido resolvido na fonte; aprovado para `analyze` — Bloqueado por: T010
- [x] T012 analyze (READ-ONLY) — A1..A8, convergência e veredito PASS, **antes de `tasks`** — Bloqueado por: T011
- [x] T013 tasks — T-IDs únicos e DAG acíclica — Bloqueado por: T012
- [x] T014 test-roadmap — escopo/FR/T, pré-condições, passos numerados, superfície CLI/visual isolada, capturas Xvfb sem janela, coluna `automatizável?`, rollback, passos dos glifos/sem `▯` e hash/licença — Bloqueado por: T013
- [x] T015 converge — primeira entrada append-only em `specs/036-*/converge.md`, datada, sem tocar converge de outros ciclos — Bloqueado por: T014
- [x] T016 validate (R4 aprovado, documental) — referência ao roteiro, pendências **[NV]** — Bloqueado por: T015
- [x] T017 release (R7 `1.0.0`) — changelog FR/T, rastreabilidade e rollback — Bloqueado por: T016
- [ ] T018 implement (agente de código/orquestrador, **fora da fronteira SDD**) — FR-001/FR-002/FR-003/FR-004 (strings, `TextFont`, fonte, licença e proveniência); executar roteiro, `verify.sh`, gates e CI — Bloqueado por: T017 — **[NV]**

## Dependências
DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012 → T013 → T014 → T015 → T016 → T017 → T018. Nenhuma auto-aresta, aresta órfã ou ciclo; ordem de execução estável por sufixo numérico. `analyze` (T012) vem antes de `tasks` (T013); R4 (T016) vem antes de R7 (T017).

## Analyze (READ-ONLY, 2026-11-22T21:20Z)
Conflitos: nenhum entre spec, plano e checklist. Restaurar as strings exige a fonte embarcada (A1); o paliativo ASCII é revogado pelo pedido do SWE e substituído por Noto Sans com cobertura comprovada (A2/A3). A ordem normativa mantém `analyze` antes de `tasks` e R4 antes de R7 (A4). Evidências observadas às `21:15–21:18Z` são coladas sem reexecução; `verify.sh`, gates, captura com acento, CI e merge permanecem **[NV]** (A5); nenhuma janela é aberta por agente (A6); o roteiro é escrito antes do PR (A7); factory `b1deab1` é somente fonte e documentos/paths alheios intactos (A8). `[x]` em checklist = revisão de requisitos; `[x]` em T001–T017 registra a **escrita dos artefatos SDD**, não implementação ou execução de build/teste/gate. T018 pertence ao agente de código/orquestrador e está **[NV]**. PASS.
