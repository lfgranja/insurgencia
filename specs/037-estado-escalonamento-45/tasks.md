# Tasks — 037-estado-escalonamento-45

- [x] T001 branch `037-estado-escalonamento-45` (base `efb4d4b`), stub `spec.md` no worktree isolado — Bloqueado por: nenhum
- [x] T002 assess.intake — sessão `20261122T225813Z`, 80/20 (375/625 vs. 60/625), evidências de `22:58–23:01Z` — Bloqueado por: T001
- [x] T003 assess.research — HEADs/PRs/gates/contagens lidos, constituição v1.0.0 lida, factory e divergência registradas — Bloqueado por: T002
- [x] T004 assess.problem — rastreabilidade/escalonamento, sucesso observável, custo de inação — Bloqueado por: T003
- [x] T005 assess.shape (concept) — 5 opções, recomendação documental, 80/20 = 375/625 — Bloqueado por: T004
- [x] T006 assess.decide (go) — scorecard 6/6 strong, `Veredito: go`, R3 = 0, precedência v1.0.0 — Bloqueado por: T005
- [x] T007 specify — substituir stub por US1–US4, FR-001..FR-009, SC-001..SC-006 — Bloqueado por: T006
- [x] T008 clarify — escalonar ≠ mesclar; divergência ≠ dívida; contagens com ressalva; seção documental — Bloqueado por: T007
- [x] T009 plan — passos na ordem do alvo, `analyze` antes de `tasks`, R4 antes de R7, sem R6 — Bloqueado por: T008
- [x] T010 checklist de requisitos — um item por FR-001..FR-009 — Bloqueado por: T009
- [x] T011 revisar checklist — conflito v1.2.0/v1.0.0 resolvido na fonte (Bloco 0.a); aprovado para `analyze` — Bloqueado por: T010
- [x] T012 analyze (READ-ONLY) — A1..A8, convergência e veredito PASS, **antes de `tasks`** — Bloqueado por: T011
- [x] T013 tasks — T-IDs únicos e DAG acíclica — Bloqueado por: T012
- [x] T014 test-roadmap — escopo/FR/T, pré-condições, 6–10 passos, superfície documental/CLI, capturas de terminal, `automatizável?`, rollback, passos notas vs HEAD/contagens e não-adoção de R6 — Bloqueado por: T013
- [x] T015 converge — primeira entrada append-only em `specs/037-*/converge.md`, datada, sem tocar converge de outros ciclos — Bloqueado por: T014
- [x] T016 validate (R4 aprovado, documental) — referência ao roteiro, pendências **[NV]** — Bloqueado por: T015
- [x] T017 release (R7 `1.0.0`) — changelog FR/T, rastreabilidade e rollback — Bloqueado por: T016
- [ ] T018 escalonamento/promoção (SWE/orquestrador, **fora da fronteira SDD**) — acompanhar PR #45 e ACR; decidir merge de `dev → main`; executar verificações futuras do roteiro — Bloqueado por: T017 — **[NV]**

## Dependências
DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012 → T013 → T014 → T015 → T016 → T017 → T018. Nenhuma auto-aresta, aresta órfã ou ciclo; ordem de execução estável por sufixo numérico. `analyze` (T012) vem antes de `tasks` (T013); R4 (T016) vem antes de R7 (T017). Sem R6.

## Analyze (READ-ONLY, 2026-11-22T23:01Z)
Conflitos: nenhum entre spec, plano e checklist. O escalonamento do PR #45 é explícito e não implica merge (A1); a constituição v1.0.0 do alvo prevalece sobre o prompt v4.6 e R6 não é adotado (A2); a divergência é decisão de governança, não dívida (A3); contagens são registradas com `find` e ressalva (A4). A ordem normativa mantém `analyze` antes de `tasks` e R4 antes de R7 (A5). Evidências de `22:58–23:01Z` são coladas sem reexecução; gates, CI e merge permanecem **[NV]** (A6); o roteiro é escrito antes do PR com os passos exigidos (A7); factory `b1deab1` é somente fonte, converge é novo e `taskstoissues` não é usado (A8). `[x]` em checklist = revisão de requisitos; `[x]` em T001–T017 registra a **escrita dos artefatos SDD**, não execução de build/teste/gate nem promoção. T018 permanece **[NV]**.
