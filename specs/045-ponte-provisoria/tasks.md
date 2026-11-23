# Tasks — 045-ponte-provisoria

- [ ] T001 00 constituição/branch — worktree `.specify/queue/worktrees/ponte-provisoria-045/`, branch `045-ponte-provisoria`, base `fa33052`, stub `spec.md`; constituição v1.2.2 aplicada, R6 **N/A**, R1 `go`, R4 antes de R7 `1.0.0-local` — Bloqueado por: nenhum
- [ ] T002 01 intake + 02 research — sessão `20261123T062500Z-ponte-provisoria`, título `insurgencia - ponte provisoria deterministica`, scorecard **3×3×3×4 = 108/625** contra 48/625 (Súditos/Propaganda), 48/625 (Combate/Fôlego) e 10/625 (rede, bloqueada por ADR-004 até decisão da SE); lidos GDD §9.3:475,483 e §8.4:407, `.specify/memory/STATE.md` (06:25Z), `crates/sim/src/lib.rs` (`ACOES_PERMITIDAS`, `Mundo`, `Barreira`, `ValidacaoComandos`, `fase_finish`, `hash_estado`, `ponto_igual`) e `crates/cliente/src/main.rs` (HUD/teclas); R6 N/A registrada com fonte — Bloqueado por: T001
- [ ] T003 03 define + 04 shape — problema/critérios observáveis (ponte canônica ausente; 15 Materiais, 60 PV, 1350 ticks, recusa sem consumo parcial; atalho de 8 s depende de mapa/ilhas e fica fora de escopo), 4 opções avaliadas, recomendação `ponte-provisoria` sem mudança de stack — Bloqueado por: T002
- [ ] T004 05 decide + 07 clarify — `Veredito: go` (R1), scorecard **108/625** (impacto × confiança × facilidade ÷ risco, denominador 625), R3 = 0; clarificar 15 Materiais/60 PV/45 s canônicos, atalho 8 s fora de escopo, 45 ticks de construção como protótipo (≠ 45 s de duração), `pv = 60` sem sistema de dano, ponte sem autogestão/barricadas/vitória, R6 N/A — Bloqueado por: T003
- [ ] T005 06 specify — substituir stub por `spec.md` com US1–US6, FR-001..FR-006, SC-001..SC-006, 6 FR mapeados como regra canônica / contrato-estado / comportamento-fase / hash / HUD / governança, scorecard, fronteira e 80/20 — Bloqueado por: T004
- [ ] T006 08 plan — `plan.md`: passos 00→26, fronteira explícita (só 4 artefatos), roteiro de 10 passos, R4 (T011) antes de R7 (T012 `1.0.0-local`), R6 N/A — Bloqueado por: T005
- [ ] T007 09 checklist de requisitos — um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`, `[x]` = **revisão de requisitos** (não implementação) — Bloqueado por: T006
- [ ] T008 revisão do checklist — 6 FR/6 SC verificáveis por inspeção, rastreabilidade idêntica à dos artefatos de governança, atalho 8 s e 45 ticks rotulados; aprovado para `tasks` — Bloqueado por: T007
- [ ] T009 10 tasks — este arquivo: T001..T012 com IDs únicos e DAG, passos 00→26; `tasks` antes de `analyze`, R4 antes de R7 — Bloqueado por: T008
- [ ] T010 11 analyze (READ-ONLY) — A1..A6, convergência e veredito, **após `tasks`** conforme v1.2.2; fora da fronteira de escrita deste agente — Bloqueado por: T009
- [ ] T011 12 implement + 13 converge + 27 validate (R4) — agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos: acrescentar `construir_ponte` a `ACOES_PERMITIDAS` na sim (sem tocar `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`); struct pública `Ponte { id, dono, posicao, pv, ticks_restantes }` em `BTreeMap<u64, Ponte>` (`Mundo::pontes`, `proxima_ponte_id`) e constantes `CUSTO_PONTE_MATERIAIS = 15`, `PV_PONTE = 60`, `TICKS_DURACAO_PONTE = 1350`, `TICKS_CONSTRUCAO_PONTE = 45`; validar `destino == posicao` via `ponto_igual` e saldo ≥ 15 com recusa sem consumo parcial; canalização de 45 ticks no máx. 1 por jogador por tick, zerando sem criar nem debitar quando interrompida; débito atômico único no tick 45 com criação condicionada, `pv = 60` e `ticks_restantes = 1351` (unidade extra do tick de criação); expiração determinística em `fase_finish` por decremento/`retain` em ordem de `BTreeMap`, sem relógio/aleatoriedade; `hash_estado` inclui as pontes em ordem estável; HUD exibe `Pontes` com a contagem do jogador local e tecla `P` envia `construir_ponte` na posição atual, sem campo novo de protocolo; testes determinísticos para 14/15 Materiais, destino inválido, 44/45 ticks, débito único, 1350 ticks de duração, expiração, ordem, hash e HUD; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, `///` em itens públicos; primeira entrada append-only em `converge.md`; `validate.md` com pendências **[NV]** — Bloqueado por: T010
- [ ] T012 26 release (R7 `1.0.0-local`) — `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4 — Bloqueado por: T011

## Dependências

DAG linear: T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009 → T010 → T011 → T012. Nenhuma auto-aresta, aresta órfã ou ciclo; ordem de execução estável por sufixo numérico. **`tasks` (T009) vem antes de `analyze` (T010)**, conforme a constituição v1.2.2 do alvo; a cadeia de passos vai de 00 a 26, com `27 validate` (T011) imediatamente antes de `26 release` (T012). R4 (T011) vem antes de R7 (T012). **R6 não pertence à DAG** (N/A por v1.2.2: PARE se invocada, nenhuma issue simulada; dívida de automação registrada).

## Rastreabilidade FR → T

| FR | Task |
|---|---|
| FR-001 regra canônica (15 Materiais, 60 PV, 1350 ticks; GDD §9.3:475,483 e §8.4:407) | T011 |
| FR-002 contrato/estado (`construir_ponte`, `Ponte`, constantes, `Mundo::pontes`) | T011 |
| FR-003 comportamento/fase (validação, débito atômico, expiração em `fase_finish`) | T011 |
| FR-004 hash/determinismo | T011 |
| FR-005 cliente/HUD `Pontes` e tecla `P` | T011 |
| FR-006 governança/testes/R6 N/A | T001, T011, T012 |

Os 4 artefatos deste agente são produzidos por T005 (`spec.md`), T006 (`plan.md`), T007 (`checklists/requirements.md`) e T009 (`tasks.md`); T008 registra a revisão do checklist. T010–T012 e o código de T011 estão fora da fronteira de escrita deste agente e serão executados conforme a ordem v1.2.2. Nenhum build, gate, teste, formatador, commit, push ou PR é executado por este agente; resultados de execução ficam **[NV]** até confirmação ao vivo.
