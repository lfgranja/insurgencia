# Tasks — 044-defesa-barricada

- [ ] T001 00 constituição/branch — worktree `.specify/queue/worktrees/defesa-barricada-044/`, branch `044-defesa-barricada`, base `b6d2f36`, stub `spec.md`; constituição v1.2.2 aplicada, R6 **N/A**, R1 `go`, R4 antes de R7 `1.0.0-local` — Bloqueado por: nenhum
- [ ] T002 01 intake + 02 research — sessão `20261123T054700Z-defesa-barricada`, título `insurgencia - defesa de barricada`, scorecard **4×4×4×4 = 256/625** contra 108/625 (ponte), 48/625 (Súditos/Propaganda) e 10/625 (rede, bloqueada por ADR-004 até decisão da SE); lidos GDD §7.1:306/§6.2/:299/:396, `.specify/memory/STATE.md` (05:47Z) e `crates/sim/src/lib.rs` (`Jogador`, `Barreira`, `Mundo`, `fase_finish`, `hash_estado`, `ponto_igual`); R6 N/A registrada com fonte — Bloqueado por: T001
- [ ] T003 03 define + 04 shape — problema/critérios observáveis (barricada 200 PV/360 ticks sem recompensa; 10 s = 300 ticks a 1/30 s; interrupção e hash), 4 opções avaliadas, recomendação `defesa-barricada` sem mudança de stack — Bloqueado por: T002
- [ ] T004 05 decide + 07 clarify — `Veredito: go` (R1), scorecard **256/625** (impacto × confiança × facilidade ÷ risco, denominador 625), R3 = 0; clarificar 300 ticks ≠ «tempo alvo 8 s», «intacta» após decremento, dono = defensor (Guerreiro/Artesão como simplificação), pulso 30 s/vitória fora de escopo, `mover` sem mudança não interrompe, pontos persistem, R6 N/A — Bloqueado por: T003
- [ ] T005 06 specify — substituir stub por `spec.md` com US1–US6, FR-001..FR-006, SC-001..SC-006, 6 FR mapeados como regra canônica / contrato-estado / comportamento-fase / hash / HUD / governança, scorecard, fronteira e 80/20 — Bloqueado por: T004
- [ ] T006 08 plan — `plan.md`: passos 00→26, fronteira explícita (só 4 artefatos), roteiro de 10 passos, R4 (T011) antes de R7 (T012 `1.0.0-local`), R6 N/A — Bloqueado por: T005
- [ ] T007 09 checklist de requisitos — um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`, `[x]` = **revisão de requisitos** (não implementação) — Bloqueado por: T006
- [ ] T008 revisão do checklist — 6 FR/6 SC verificáveis por inspeção, rastreabilidade idêntica à dos artefatos de governança; aprovado para `tasks` — Bloqueado por: T007
- [ ] T009 10 tasks — este arquivo: T001..T012 com IDs únicos e DAG, passos 00→26; `tasks` antes de `analyze`, R4 antes de R7 — Bloqueado por: T008
- [ ] T010 11 analyze (READ-ONLY) — A1..A6, convergência e veredito, **após `tasks`** conforme v1.2.2; fora da fronteira de escrita deste agente — Bloqueado por: T009
- [ ] T011 12 implement + 13 converge + 27 validate (R4) — agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos: regra canônica +1 a cada 300 ticks (10 s a 1/30 s) com barricada intacta e dono no mesmo ponto; `Jogador.pontos_autogestao: i64` e `Barreira.defesa_ticks: i64` com constantes documentadas, sem tocar `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`; passo determinístico em `fase_finish` após decremento/expiração, iteração em ordem de `BTreeMap` por `id`, no máx. 1 avanço por barricada/tick, zera sem crédito em saída do dono/dono ausente/destruição/expiração; `hash_estado` inclui pontos e defesa parcial; HUD exibe `Autogestão` sem nova tecla; testes para 299/300 ticks, interrupção, expiração, ordem e hash; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, sem Lightyear/async/sockets; primeira entrada datada em `converge.md` (append-only); `validate.md` aprovando documentalmente com roteiro de 10 passos; gates/build/testes/formatadores **[NV]** até o orquestrador — Bloqueado por: T010
- [ ] T012 26 release (R7 `1.0.0-local`) — `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4 — Bloqueado por: T011

## Dependências

DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012. Nenhuma auto-aresta, aresta órfã ou ciclo; ordem de execução estável por sufixo numérico. **`tasks` (T009) vem antes de `analyze` (T010)**, conforme a constituição v1.2.2 do alvo; a cadeia de passos vai de 00 a 26, com `27 validate` (T011) imediatamente antes de `26 release` (T012). R4 (T011) vem antes de R7 (T012). **R6 não pertence à DAG** (N/A por v1.2.2: PARE se invocada, nenhuma issue simulada; dívida de automação registrada).

## Rastreabilidade FR → T

| FR | Task |
|---|---|
| FR-001 regra canônica (+1/300 ticks, GDD §7.1:306) | T011 |
| FR-002 contrato/estado (`pontos_autogestao`, `defesa_ticks`, constantes) | T011 |
| FR-003 comportamento/fase (`fase_finish`, interrupção, ordem `BTreeMap`) | T011 |
| FR-004 hash/determinismo | T011 |
| FR-005 cliente/HUD `Autogestão` | T011 |
| FR-006 governança/testes/R6 N/A | T001, T011, T012 |

Os 4 artefatos deste agente são produzidos por T005 (`spec.md`), T006 (`plan.md`), T007 (`checklists/requirements.md`) e T009 (`tasks.md`); T008 registra a revisão do checklist. T010–T012 e o código de T011 estão fora da fronteira de escrita deste agente e serão executados conforme a ordem v1.2.2. Nenhum build, gate, teste, formatador, commit, push ou PR é executado por este agente; resultados de execução ficam **[NV]** até confirmação ao vivo.
