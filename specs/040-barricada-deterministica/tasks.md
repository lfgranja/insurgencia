# Tasks — 040-barricada-deterministica

- [ ] T001 00 constituição/branch — worktree `.specify/queue/worktrees/barricada-040/`, branch `040-barricada-deterministica`, base `6b913f0`, stub `spec.md`; constituição v1.2.2 aplicada, R6 N/A, R1 `go` — Bloqueado por: nenhum
- [ ] T002 01 intake — sessão `20261123T024300Z-barricada-deterministica`, título `insurgencia - barricada deterministica de materiais`, 80/20 = 400/625 — Bloqueado por: T001
- [ ] T003 02 research — GDD §9.3:475,483 e §10.3:574,577-579 / H4 lidos; valores canônicos separados das `[HIPÓTESE]`; R6 N/A e conflito §16.3/H4 como risco — Bloqueado por: T002
- [ ] T004 03 define + 04 shape — problema/critérios observáveis, 4 opções, recomendação `construir_barreira` — Bloqueado por: T003
- [ ] T005 05 decide + 07 clarify — `Veredito: go` (R1), scorecard 5×5×4×4 = 400/625, R3 = 0; recusa sem consumo parcial, limite por jogador, `B` ≠ `Q`, R6 N/A — Bloqueado por: T004
- [ ] T006 06 specify — substituir stub por `spec.md` com US1–US6, FR-001..FR-006, SC-001..SC-006 — Bloqueado por: T005
- [ ] T007 08 plan — passos 00→26, fronteira explícita (só 4 artefatos), roteiro de 10 passos, R4 antes de R7, R6 N/A — Bloqueado por: T006
- [ ] T008 09 checklist de requisitos — um item por FR-001..FR-006 em `checklists/requirements.md` — Bloqueado por: T007
- [ ] T009 revisão do checklist — 6 FR/6 SC verificáveis, `[x]` = revisão de requisitos (não implementação); aprovado para `tasks` — Bloqueado por: T008
- [ ] T010 10 tasks — este arquivo: T001..T014 com IDs únicos e DAG; `tasks` antes de `analyze`, R4 antes de R7 — Bloqueado por: T009
- [ ] T011 11 analyze (READ-ONLY) — A1..A6, convergência e veredito, **após `tasks`** conforme v1.2.2; fora da fronteira de escrita deste agente — Bloqueado por: T010
- [ ] T012 12 implement (código Rust, agentes de implementação) — FR-001 constantes/estado/API; FR-002 validação sem consumo parcial; FR-003 `fase_skills`, movimento e revalidação; FR-004 limite/remoção/PV/duração; FR-005 `hash_estado`; FR-006 input `B`/HUD; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, sem tocar `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml` — Bloqueado por: T011
- [ ] T013 13 converge (append-only) + 27 validate (R4) — primeira entrada datada em `converge.md`; `validate.md` aprovando documentalmente com roteiro de 10 passos; gates/build/testes **[NV]** até o orquestrador — Bloqueado por: T012
- [ ] T014 26 release (R7) — `release.md` versão `1.0.0`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4 — Bloqueado por: T013

## Dependências
DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012 → T013 → T014. Nenhuma auto-aresta, aresta órfã ou ciclo; ordem de execução estável por sufixo numérico. **`tasks` (T010) vem antes de `analyze` (T011)**, conforme a constituição v1.2.2 do alvo. R4 (T013) vem antes de R7 (T014). **R6 não pertence à DAG** (N/A por v1.2.2: PARE se invocada, nenhuma issue simulada; dívida de automação registrada).

## Rastreabilidade FR → T
| FR | Task |
|---|---|
| FR-001 | T012 |
| FR-002 | T012 |
| FR-003 | T012 |
| FR-004 | T012 |
| FR-005 | T012 |
| FR-006 | T012 |

Os 4 artefatos deste agente são produzidos por T006 (`spec.md`), T007 (`plan.md`), T008 (`checklists/requirements.md`) e T010 (`tasks.md`); T009 registra a revisão do checklist. T011–T014 e o código de T012 estão fora da fronteira de escrita deste agente e serão executados conforme a ordem v1.2.2. Nenhum build, gate, teste, formatador, commit, push ou PR é executado por este agente; resultados de execução ficam **[NV]** até confirmação ao vivo.
