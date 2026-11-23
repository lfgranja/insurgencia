# Tasks — 047-combate-folego

- [ ] T001 00 constituição/branch — worktree `.specify/queue/worktrees/combate-047/`, branch `047-combate-folego`, base `0ac4c9f`, stub `spec.md`; constituição v1.2.2 aplicada, R6 **N/A**, R1 `go`, R4 antes de R7 `1.0.0-local` — Bloqueado por: nenhum
- [ ] T002 01 intake + 02 research — sessão `20261123T073500Z-combate-folego`, título `insurgencia - combate e folego`, scorecard **4×2×2×3 = 48/625** contra rede 10/625 (bloqueada por ADR-004 até decisão da SE; 256/625 ciclo 044, 108/625 ciclo 045 e 48/625 ciclo 046 já entregues); lidos GDD §10.1 (Fôlego 100, regeneração 10/s fora e 5/s em combate, alcance 2 m — `[HIPÓTESE]`), `.specify/memory/STATE.md` (decisão 07:35Z), `crates/sim/src/lib.rs` (`ACOES_PERMITIDAS` contém `atacar_forte`, `Jogador`, `vida`, `PASSO_SEGUNDOS`, `ponto_igual`, `fase_finish`, `hash_estado`, `CodigoRecusa`, `BTreeMap`, `BTreeSet`) e `crates/cliente/src/main.rs` (HUD/teclas); R6 N/A registrada com fonte — Bloqueado por: T001
- [ ] T003 03 define + 04 shape — problema/critérios observáveis (`atacar_forte` inerte; falta Fôlego, custo, dano `saturating_sub`, validação de alvo/alcance/Fôlego, deduplicação, combate de 90 ticks, regeneração acumulada, hash e HUD; todos os valores `[HIPÓTESE]`/protótipo, nunca cânone), 2 opções avaliadas, recomendação `combate-folego` sem mudança de stack — Bloqueado por: T002
- [ ] T004 05 decide + 07 clarify — `Veredito: go` (R1), scorecard **48/625** (impacto × confiança × facilidade ÷ risco, denominador 625), R3 = 0; clarificar GDD §10.1 como fonte de hipóteses e não de cânone, combate como transição de estado sem física/projétil, alvo no `destino` a ≤ 2 m (`[HIPÓTESE]`), `BTreeSet<u64>` deduplicando ataques por atacante/tick, acumulador `folego_ticks` preservado na troca de taxa, R6 N/A — Bloqueado por: T003
- [ ] T005 06 specify — substituir stub por `spec.md` com US1–US7, FR-001..FR-006, SC-001..SC-006, 6 FR mapeados como regra de Fôlego/combate / contrato-estado / comportamento-fase / hash / HUD / governança, com scorecard, fronteira, 80/20 e rotulagem explícita de todos os valores como `[HIPÓTESE]`/protótipo — Bloqueado por: T004
- [ ] T006 08 plan — `plan.md`: passos 00→26, fronteira explícita (só 4 artefatos), roteiro de 10 passos, R4 (T011) antes de R7 (T012 `1.0.0-local`), R6 N/A — Bloqueado por: T005
- [ ] T007 09 checklist de requisitos — um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`, `[x]` = **revisão de requisitos** (não implementação) — Bloqueado por: T006
- [ ] T008 revisão do checklist — 6 FR/6 SC verificáveis por inspeção, rastreabilidade idêntica à dos artefatos de governança, hipóteses/protótipo rotulados e fronteira confirmada; aprovado para `tasks` — Bloqueado por: T007
- [ ] T009 10 tasks — este arquivo: T001..T012 com IDs únicos e DAG linear, passos 00→26; `tasks` antes de `analyze`, R4 antes de R7 — Bloqueado por: T008
- [ ] T010 11 analyze (READ-ONLY) — A1..A6, convergência e veredito, **após `tasks`** conforme v1.2.2; fora da fronteira de escrita deste agente — Bloqueado por: T009
- [ ] T011 12 implement + 13 converge + 27 validate (R4) — agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos: dar efeito a `atacar_forte` (já em `ACOES_PERMITIDAS` na sim, sem tocar `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`); `Jogador` ganha `folego`, `folego_ticks`, `combate_ticks` incluídos no `hash_estado`; constantes documentadas `FOLEGO_MAXIMO = 100`, `CUSTO_ATAQUE_FORTE = 30`, `DANO_ATAQUE_FORTE = 10`, `TICKS_COMBATE = 90`, `ALCANCE_CONTATO_M = 2.0`, `FOLEGO_REGENERACAO_FORA = 10`, `FOLEGO_REGENERACAO_COMBATE = 5` (todas `[HIPÓTESE]`/protótipo, nunca cânone); validação de alvo no ponto/alcance e de Fôlego com `CodigoRecusa::AlvoInvalido`/`FolegoInsuficiente`; dano `saturating_sub` e um ataque por jogador por tick via `BTreeSet<u64>`; regeneração em `fase_finish` com acumulador (+1 a cada 3 ticks fora de combate, +1 a cada 6 em combate, teto preservado); cliente com tecla `Q` (`KeyCode::KeyQ`) e HUD `Fôlego`/`Vida`; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; sem Lightyear, async, sockets, janela, relógio ou efeito em barricadas/pontes/súditos; `converge.md` append-only e `validate.md` documental com gates/build/testes **[NV]** — Bloqueado por: T010
- [ ] T012 26 release (R7 `1.0.0-local`) — `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4 — Bloqueado por: T011

## Dependências

DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012. Nenhuma auto-aresta, aresta órfã ou ciclo; ordem de execução estável por sufixo numérico. **`tasks` (T009) vem antes de `analyze` (T010)**, conforme a constituição v1.2.2 do alvo; a cadeia de passos vai de 00 a 26, com `27 validate` (T011) imediatamente antes de `26 release` (T012). R4 (T011) vem antes de R7 (T012). **R6 não pertence à DAG** (N/A por v1.2.2: PARE se invocada, nenhuma issue simulada; dívida de automação registrada). Nenhum número de combate é tratado como cânone: todos os valores de GDD §10.1 são `[HIPÓTESE]` e 30/10/90 ticks são escolhas de protótipo.

## Rastreabilidade FR → T

| FR | Task |
|---|---|
| FR-001 regra de Fôlego/combate (30/10/90 ticks, regeneração 3/6 ticks, 2 m — `[HIPÓTESE]`/protótipo) | T011 |
| FR-002 contrato/estado (`atacar_forte`, campos do `Jogador`, constantes, `CodigoRecusa`) | T011 |
| FR-003 comportamento/fase (alvo/alcance/Fôlego, `BTreeSet`, `saturating_sub`, regeneração em `fase_finish`) | T011 |
| FR-004 hash/determinismo | T011 |
| FR-005 cliente/HUD `Fôlego`/`Vida` e tecla `Q` | T011 |
| FR-006 governança/testes/R6 N/A | T001, T011, T012 |

Os 4 artefatos deste agente são produzidos por T005 (`spec.md`), T006 (`plan.md`), T007 (`checklists/requirements.md`) e T009 (`tasks.md`); T008 registra a revisão do checklist. T010–T012 e o código de T011 estão fora da fronteira de escrita deste agente e serão executados conforme a ordem v1.2.2. Nenhum build, gate, teste, formatador, commit, push ou PR é executado por este agente; resultados de execução ficam **[NV]** até confirmação ao vivo.
