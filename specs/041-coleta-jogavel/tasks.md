# Tasks — 041-coleta-jogavel

- [ ] T001 00 constituição/branch — worktree `.specify/queue/worktrees/coleta-cliente-041/`, branch `041-coleta-jogavel`, base `bf05cfa`, stub `spec.md`; constituição v1.2.2 aplicada, R6 **N/A**, R1 `go` — Bloqueado por: nenhum
- [ ] T002 01 intake + 02 research — sessão `20261123T035400Z-coleta-jogavel`, título `insurgencia - coleta jogavel no cliente`, 80/20 = 500/625; GDD §7.1 (6 s = 180 ticks a 1/30 s → +3 Suprimentos/+2 Materiais; saída zera parcial) e `crates/sim/src/lib.rs` lidos (`TICKS_COLETA`, campos, `hash_estado`, padrão `BTreeSet<u64>` do ciclo 040); R6 N/A registrada com fonte — Bloqueado por: T001
- [ ] T003 03 define + 04 shape — problema/critérios observáveis, 4 opções avaliadas, recomendação `coleta jogável no cliente` — Bloqueado por: T002
- [ ] T004 05 decide + 07 clarify — `Veredito: go` (R1), scorecard 5×5×5×4 = 500/625, R3 = 0; clarificar auto-avanço vs. reenvio, limite por jogador, `C` do cliente, doc comment vs. contrato, R6 N/A — Bloqueado por: T003
- [ ] T005 06 specify — substituir stub por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006 — Bloqueado por: T004
- [ ] T006 08 plan — este arquivo: passos 00→26, fronteira explícita (só 4 artefatos), roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A — Bloqueado por: T005
- [ ] T007 09 checklist de requisitos — um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`, `[x]` = revisão de requisitos (não implementação) — Bloqueado por: T006
- [ ] T008 revisão do checklist — 6 FR/6 SC verificáveis por inspeção; aprovado para `tasks` — Bloqueado por: T007
- [ ] T009 10 tasks — este arquivo: T001..T012 com IDs únicos e DAG; `tasks` antes de `analyze`, R4 antes de R7 — Bloqueado por: T008
- [ ] T010 11 analyze (READ-ONLY) — A1..A6, convergência e veredito, **após `tasks`** conforme v1.2.2; fora da fronteira de escrita deste agente — Bloqueado por: T009
- [ ] T011 12 implement + 13 converge + 27 validate (R4) — agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos: auto-avanço `BTreeSet<u64>` e crédito atômico, regra canônica/estado/hash preservados, validação/dedup, doc comment de `Comando.destino`, cliente `C`/HUD; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, sem tocar `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`; primeira entrada datada em `converge.md` (append-only); `validate.md` aprovando documentalmente com o roteiro de 10 passos; gates/build/testes **[NV]** até o orquestrador — Bloqueado por: T010
- [ ] T012 26 release (R7 `1.0.0-local`) — `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4 — Bloqueado por: T011

## Dependências
DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012. Nenhuma auto-aresta, aresta órfã ou ciclo; ordem de execução estável por sufixo numérico. **`tasks` (T009) vem antes de `analyze` (T010)**, conforme a constituição v1.2.2 do alvo. R4 (T011) vem antes de R7 (T012). **R6 não pertence à DAG** (N/A por v1.2.2: PARE se invocada, nenhuma issue simulada; dívida de automação registrada).

## Rastreabilidade FR → T
| FR | Task |
|---|---|
| FR-001 auto-avanço `BTreeSet`/crédito atômico | T011 |
| FR-002 regra canônica/estado/hash preservados | T011 |
| FR-003 validação/dedup/limite por jogador | T011 |
| FR-004 doc comment `Comando.destino`/fronteira | T011 |
| FR-005 tecla `C`/HUD | T011 |
| FR-006 estilo/governança/R6 N/A | T001, T011, T012 |

Os 4 artefatos deste agente são produzidos por T005 (`spec.md`), T006 (`plan.md`), T007 (`checklists/requirements.md`) e T009 (`tasks.md`); T008 registra a revisão do checklist. T010–T012 e o código de T011 estão fora da fronteira de escrita deste agente e serão executados conforme a ordem v1.2.2. Nenhum build, gate, teste, formatador, commit, push ou PR é executado por este agente; resultados de execução ficam **[NV]** até confirmação ao vivo.
