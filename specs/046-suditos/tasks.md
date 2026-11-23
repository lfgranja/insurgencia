# Tasks — 046-suditos

- [ ] T001 00 constituição/branch — worktree `.specify/queue/worktrees/suditos-046/`, branch `046-suditos`, base `d410bd6`, stub `spec.md`; constituição v1.2.2 aplicada, R6 **N/A**, R1 `go`, R4 antes de R7 `1.0.0-local` — Bloqueado por: nenhum
- [ ] T002 01 intake + 02 research — sessão `20261123T065800Z-suditos`, título `insurgencia - suditos e propaganda`, scorecard **4×2×2×3 = 48/625** contra 48/625 (Combate/Fôlego) e 10/625 (rede, bloqueada por ADR-004 até decisão da SE; 256/625 ciclo 044 e 108/625 ciclo 045 já entregues); lidos GDD §9.2:464, `.specify/memory/STATE.md` (06:58Z), `crates/sim/src/lib.rs` (`ACOES_PERMITIDAS`, `Mundo`, `BTreeMap`, `BTreeSet`, `hash_estado`, `fase_finish`, `PASSO_SEGUNDOS`) e `crates/cliente/src/main.rs` (HUD/teclas); R6 N/A registrada com fonte — Bloqueado por: T001
- [ ] T003 03 define + 04 shape — problema/critérios observáveis (regra canônica de Súditos ausente; 120 ticks de exposição convertem e 90 ticks de reversão por dano; spawn/6 m/funções são `[HIPÓTESE]` e ficam fora de escopo), 3 opções avaliadas, recomendação `suditos` sem mudança de stack — Bloqueado por: T002
- [ ] T004 05 decide + 07 clarify — `Veredito: go` (R1), scorecard **48/625** (impacto × confiança × facilidade ÷ risco, denominador 625), R3 = 0; clarificar GDD §9.2:464 como canônica (120/90 ticks), hipóteses fora de escopo, conversão como transição de estado sem sistema de ataque/panfletar, dano via `aplicar_dano_a_sudito(id)` sem física, `BTreeSet<u64>` deduplicando exposição por ID/tick, R6 N/A — Bloqueado por: T003
- [ ] T005 06 specify — substituir stub por `spec.md` com US1–US6, FR-001..FR-006, SC-001..SC-006, 6 FR mapeados como regra canônica / contrato-estado / comportamento-fase / hash / HUD / governança, scorecard, fronteira e 80/20 — Bloqueado por: T004
- [ ] T006 08 plan — `plan.md`: passos 00→26, fronteira explícita (só 4 artefatos), roteiro de 10 passos, R4 (T011) antes de R7 (T012 `1.0.0-local`), R6 N/A — Bloqueado por: T005
- [ ] T007 09 checklist de requisitos — um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`, `[x]` = **revisão de requisitos** (não implementação) — Bloqueado por: T006
- [ ] T008 revisão do checklist — 6 FR/6 SC verificáveis por inspeção, rastreabilidade idêntica à dos artefatos de governança, hipóteses e conversão-como-estado rotuladas; aprovado para `tasks` — Bloqueado por: T007
- [ ] T009 10 tasks — este arquivo: T001..T012 com IDs únicos e DAG, passos 00→26; `tasks` antes de `analyze`, R4 antes de R7 — Bloqueado por: T008
- [ ] T010 11 analyze (READ-ONLY) — A1..A6, convergência e veredito, **após `tasks`** conforme v1.2.2; fora da fronteira de escrita deste agente — Bloqueado por: T009
- [ ] T011 12 implement + 13 converge + 27 validate (R4) — agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos: acrescentar `propaganda` a `ACOES_PERMITIDAS` na sim (sem tocar `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`); struct pública `Sudito { id, posicao, convertido, propaganda_ticks, reversao_ticks }` em `BTreeMap<u64, Sudito>` (`Mundo::suditos`), APIs `adicionar_sudito(id, posicao)` e `aplicar_dano_a_sudito(id)` e constantes `TICKS_PROPAGANDA = 120`, `TICKS_REVERSAO = 90`; auto-avanço com `BTreeSet<u64>` (1 tick por Súdito por tick, deduplicado por ID) e conversão no tick 120 (para de atacar estruturas e passa a panfletar no estado); dano fixa 90 ticks e a conversão cessa ao zerar, em ordem de `BTreeMap` e sem relógio/aleatoriedade; `hash_estado` inclui os súditos; HUD `Convertidos` e tecla `G` gerando `propaganda`; testes determinísticos (120/90 ticks, deduplicação, ID inexistente, ordem, hash, tecla/HUD) com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; combatentes/física e hipóteses de spawn/raio/funções fora de escopo; gates/build/testes **[NV]** até o orquestrador — Bloqueado por: T010
- [ ] T012 26 release (R7 `1.0.0-local`) — `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4 — Bloqueado por: T011

## Dependências

DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012. Nenhuma auto-aresta, aresta órfã ou ciclo; ordem de execução estável por sufixo numérico. **`tasks` (T009) vem antes de `analyze` (T010)**, conforme a constituição v1.2.2 do alvo; a cadeia de passos vai de 00 a 26, com `27 validate` (T011) imediatamente antes de `26 release` (T012). R4 (T011) vem antes de R7 (T012). **R6 não pertence à DAG** (N/A por v1.2.2: PARE se invocada, nenhuma issue simulada; dívida de automação registrada).

## Rastreabilidade FR → T

| FR | Task |
|---|---|
| FR-001 regra canônica (120 ticks de Propaganda, 90 ticks de reversão; GDD §9.2:464) | T011 |
| FR-002 contrato/estado (`propaganda`, `Sudito`, APIs, constantes, `Mundo::suditos`) | T011 |
| FR-003 comportamento/fase (auto-avanço com `BTreeSet`, conversão e reversão) | T011 |
| FR-004 hash/determinismo | T011 |
| FR-005 cliente/HUD `Convertidos` e tecla `G` | T011 |
| FR-006 governança/testes/R6 N/A | T001, T011, T012 |

Os 4 artefatos deste agente são produzidos por T005 (`spec.md`), T006 (`plan.md`), T007 (`checklists/requirements.md`) e T009 (`tasks.md`); T008 registra a revisão do checklist. T010–T012 e o código de T011 estão fora da fronteira de escrita deste agente e serão executados conforme a ordem v1.2.2. Nenhum build, gate, teste, formatador, commit, push ou PR é executado por este agente; resultados de execução ficam **[NV]** até confirmação ao vivo.
