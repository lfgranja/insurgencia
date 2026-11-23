# Tasks — 039-coleta-materiais-deterministica

- [x] T001 branch `039-coleta-materiais-deterministica` (base `990ab06`), worktree isolado `.specify/queue/worktrees/coleta-materiais-039/`, stub `spec.md` — Bloqueado por: nenhum
- [x] T002 01 intake — sessão `20261123T015356Z-coleta-materiais`, 80/20 (180/625 vs. 375/625 esgotado), GDD §7.1 e contrato auditado — Bloqueado por: T001
- [x] T003 02 research — GDD §7.1 e `lib.rs` lidos, contrato técnico e constituição v1.2.2 registrados, R6 suspensa — Bloqueado por: T002
- [x] T004 03 define (problem) — regra canônica ausente no sim, sucesso observável, custo de inação — Bloqueado por: T003
- [x] T005 04 shape (concept) — 6 opções, recomendação documental, 80/20 = 180/625 — Bloqueado por: T004
- [x] T006 05 decide (go) — scorecard 6/6 strong, `Veredito: go`, R3 = 0, R6 suspensa por decisão ratificada — Bloqueado por: T005
- [x] T007 06 specify — substituir stub por US1–US5, FR-001..FR-008, SC-001..SC-006 — Bloqueado por: T006
- [x] T008 07 clarify — destino como ID do ponto; acúmulo zerado na saída; recusa sem destino; hash divergente; não-execução; R6 suspensa — Bloqueado por: T007
- [x] T009 08 plan — passos na ordem v1.2.2, `tasks` antes de `analyze`, R4 antes de R7, roteiro antes do PR — Bloqueado por: T008
- [x] T010 09 checklist de requisitos — um item por FR-001..FR-008 — Bloqueado por: T009
- [x] T011 revisão do checklist — ordem v1.2.2 confirmada, R6 suspensa confirmada; aprovado para `tasks` — Bloqueado por: T010
- [x] T012 10 tasks — T-IDs únicos e DAG linear; `analyze` vem **depois** de `tasks` conforme v1.2.2 — Bloqueado por: T011
- [x] T013 11 analyze (READ-ONLY) — A1..A8, convergência e veredito PASS, **após `tasks`** — Bloqueado por: T012
- [x] T014 12 implement (documental) — redação dos artefatos SDD na fronteira, sem código, build, gate ou teste — Bloqueado por: T013
- [x] T015 12 test-roadmap — antes do PR: 10 passos, superfície documental/CLI, capturas, `automatizável?`, rollback e os seis casos obrigatórios — Bloqueado por: T014
- [x] T016 13 converge — primeira entrada append-only em `specs/039-*/converge.md`, datada, sem tocar converge de outros ciclos — Bloqueado por: T015
- [x] T017 27 validate (R4 aprovado, documental) — referência ao roteiro, pendências **[NV]** — Bloqueado por: T016
- [x] T018 26 release (R7 `1.0.0`) — changelog FR/T, rastreabilidade e rollback — Bloqueado por: T017
- [ ] T019 implementação em `crates/sim/src/lib.rs` + gates + roteiro + PR (SWE/orquestrador, **fora da fronteira SDD**) — `TICKS_COLETA`, campos do jogador, validação, crédito atômico, hash; nenhum `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml` — Bloqueado por: T018 — **[NV]**

## Dependências
DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012 → T013 → T014 → T015 → T016 → T017 → T018 → T019. Nenhuma auto-aresta, aresta órfã ou ciclo; ordem estável por sufixo numérico. **`tasks` (T012) vem antes de `analyze` (T013)**, conforme a constituição v1.2.2 do alvo. R4 (T017) vem antes de R7 (T018). R6 não pertence à DAG (suspensa por decisão ratificada). T019 é handoff ao SWE/orquestrador, **[NV]** neste ciclo.

## Analyze (READ-ONLY, 2026-11-23T01:56Z)
Conflitos: nenhum entre spec, plano e checklist. A regra canônica 180 ticks/+3/+2 e a perda do acúmulo parcial estão alinhadas em FR-001/FR-002 e no roteiro (A1/A2); a validação `DestinoInvalido`, o crédito atômico e o hash refletivo constam como contrato e casos de teste (A3/A4/A5); a fronteira `lib.rs` e a ausência de mudança de protocolo são explícitas (A6); a ordem v1.2.2 e a suspensão de R6 são mantidas (A7); não-execução, roteiro pré-PR e preservação de `STATE.md`/`feature.json`/`converge` alheio são requisitos (A8). `[x]` em checklist = revisão de requisitos; `[x]` em T001–T018 registra a redação dos artefatos SDD, **não** a implementação em Rust, gates ou PR. T019 permanece **[NV]**. PASS.
