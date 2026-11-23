# Tasks — 038-estado-pos-v122

- [x] T001 branch `038-estado-pos-v122` (base `e010363`), worktree isolado `.specify/queue/worktrees/doc-drift-038/`, stub `spec.md` — Bloqueado por: nenhum
- [x] T002 01 intake — sessão `20261123T012952Z`, 80/20 (375/625 vs. 180/625), evidências de `01:29–01:31Z` — Bloqueado por: T001
- [x] T003 02 research — HEADs/PR/gates/contagens/inventário lidos, deriva dos cinco documentos e constituição v1.2.2 registradas, R6 suspensa — Bloqueado por: T002
- [x] T004 03 define (problem) — rastreabilidade/deriva/governança, sucesso observável, custo de inação — Bloqueado por: T003
- [x] T005 04 shape (concept) — 6 opções, recomendação documental, 80/20 = 375/625 — Bloqueado por: T004
- [x] T006 05 decide (go) — scorecard 6/6 strong, `Veredito: go`, R3 = 0, R6 suspensa por decisão ratificada — Bloqueado por: T005
- [x] T007 06 specify — substituir stub por US1–US5, FR-001..FR-009, SC-001..SC-006 — Bloqueado por: T006
- [x] T008 07 clarify — escalonar ≠ mesclar; deriva ≠ edição; R6 suspensa ≠ dívida; contagens versionadas ≠ checkout; ordem v1.2.2 ≠ 037 — Bloqueado por: T007
- [x] T009 08 plan — passos na ordem v1.2.2, `tasks` antes de `analyze`, R4 antes de R7, R6 fora do fluxo — Bloqueado por: T008
- [x] T010 09 checklist de requisitos — um item por FR-001..FR-009 — Bloqueado por: T009
- [x] T011 revisão do checklist — ordem v1.2.2 confirmada, R6 suspensa confirmada; aprovado para `tasks` — Bloqueado por: T010
- [x] T012 10 tasks — T-IDs únicos e DAG linear; `analyze` vem **depois** de `tasks` conforme v1.2.2 — Bloqueado por: T011
- [x] T013 11 analyze (READ-ONLY) — A1..A8, convergência e veredito PASS, **após `tasks`** — Bloqueado por: T012
- [x] T014 12 implement (documental) — redação dos artefatos SDD na fronteira, sem build/gate/teste — Bloqueado por: T013
- [x] T015 12 test-roadmap — antes do PR: escopo/FR/T, pré-condições, 10 passos, superfície documental/CLI, capturas, `automatizável?`, rollback, passos de PR #48 green, notas vs HEAD/contagens e R6 fora do fluxo — Bloqueado por: T014
- [x] T016 13 converge — primeira entrada append-only em `specs/038-*/converge.md`, datada, sem tocar converge de outros ciclos — Bloqueado por: T015
- [x] T017 27 validate (R4 aprovado, documental) — referência ao roteiro, pendências **[NV]** — Bloqueado por: T016
- [x] T018 26 release (R7 `1.0.0`) — changelog FR/T, rastreabilidade e rollback — Bloqueado por: T017
- [ ] T019 escalonamento/promoção (SWE/orquestrador, **fora da fronteira SDD**) — acompanhar PR #48 e ACR; decidir merge de `dev → main` e deploy; executar verificações futuras do roteiro — Bloqueado por: T018 — **[NV]**

## Dependências
DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012 → T013 → T014 → T015 → T016 → T017 → T018 → T019. Nenhuma auto-aresta, aresta órfã ou ciclo; ordem de execução estável por sufixo numérico. **`tasks` (T012) vem antes de `analyze` (T013)**, conforme a constituição v1.2.2 do alvo — inversão em relação ao ciclo 037. R4 (T017) vem antes de R7 (T018). R6 não pertence à DAG (suspensa por decisão ratificada).

## Analyze (READ-ONLY, 2026-11-23T01:31Z)
Conflitos: nenhum entre spec, plano e checklist. O escalonamento do PR #48 é explícito e não implica merge (A1); a deriva dos cinco documentos é registrada sem edição e as contagens são distinguidas (A2/A3); R6 permanece suspensa por decisão ratificada, com PARE se invocada e nenhuma issue simulada (A4); a ordem v1.2.2 mantém `tasks` antes de `analyze` e R4 antes de R7 (A5). Evidências de `01:29–01:31Z` são coladas sem reexecução; gates, CI, merge e deploy permanecem **[NV]** (A6); o roteiro é escrito antes do PR com os passos exigidos (A7); factory `b1deab1` é somente fonte, converge é novo e alterações preexistentes fora da fronteira não são tocadas (A8). `[x]` em checklist = revisão de requisitos; `[x]` em T001–T018 registra a **escrita dos artefatos SDD**, não a execução de gates, build, commit, push ou PR. PASS.
