# Convergência — 039-coleta-materiais-deterministica

**Data**: 2026-11-23T01:56Z · **Modo**: READ-ONLY, append-only (1ª entrada deste ciclo).

## Evidências (lidas/auditadas nesta sessão, não reexecutadas)
- `01:53Z`: worktree isolado `.specify/queue/worktrees/coleta-materiais-039/`, branch `039-coleta-materiais-deterministica`, base `990ab06`; `specs/039-coleta-materiais-deterministica/spec.md` era stub e foi substituído. Sessão `20261123T015356Z-coleta-materiais`.
- `01:53Z`: [source: `docs/GDD - Insurgência.md` §7.1] Fase 1 "Coletar": 6 s em ponto neutro → exatamente 3 Suprimentos + 2 Materiais no pulso; sair antes zera o acúmulo parcial. Tempo alvo 6 s no pulso de 30 s.
- `01:53Z`: [source: `crates/sim/src/lib.rs`, lido] `PASSO_SEGUNDOS=1/30`; `ACOES_PERMITIDAS` já inclui `coletar_materiais`; `Jogador` sem estoques/contador; `fase_skills` só trata `mover`; `hash_estado` não inclui campos de coleta; `validar` exige destino apenas para `mover`.
- `01:53Z`: [source: contrato técnico auditado pelo orquestrador] `TICKS_COLETA=180`, campos `materiais`/`suprimentos`/`coleta_ticks`/`coleta_ponto` em `Jogador`; recusa `DestinoInvalido` para coleta sem destino ou destino ≠ posição; crédito atômico em `fase_skills`; `hash_estado` inclui os novos campos; `Comando.destino` como ID do ponto sem mudança de protocolo. Nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`.
- `01:54Z`: 80/20 = 5×4×3×3 = **180/625** escolhido: fila documental esgotada após o ciclo 038 (375/625) e menor incremento de produto com valor canônico.
- Governança: constituição **v1.2.2** do alvo (`.specify/memory/constitution.md`, ratificada 2026-11-19, última emenda 2026-11-23) — ordem `00 → 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09 → revisão → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release`, `analyze` READ-ONLY **depois** de `tasks`, R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only. **R6 ratificada como intenção, mas suspensa fora do fluxo** até `resolve-tasks.sh`/adaptador existirem; se invocada, **PARE e escale ao SWE**, nunca simular issue, nunca `gh` improvisado. Inclusão exige emenda ratificada.
- Fronteira: nenhum build/gate/formatador/teste/commit/push/PR/merge/deploy executado; `.specify/memory/STATE.md` (memória local do SWE, não versionada) não tocado; alteração preexistente em `.specify/feature.json` preservada; nenhum `converge.md` existente editado.

## Atendimento
FR-001..FR-008 e SC-001..SC-006 registrados na spec, com rastreabilidade até `.specify/validations/coleta-materiais-deterministica/test-roadmap.md` (passos 2–7: 180 ticks, saída antes, recusa sem destino, hash divergente, ausência de consumo parcial e determinismo; passos 8–10: fronteira, governança e rollback). Checklist revisado; R1 `go` (6/6 strong, 180/625); `tasks` (T012) antes de `analyze` READ-ONLY (T013), conforme v1.2.2; roteiro escrito antes do PR; R4 aprovado em `.specify/validations/coleta-materiais-deterministica/validate.md` antes de R7 `1.0.0`; rollback documentado em `.specify/releases/coleta-materiais-deterministica/release.md`. T019 (implementação em `lib.rs` + gates + PR) é handoff ao SWE/orquestrador e permanece **[NV]**, assim como execução do roteiro, CI e verificação determinística.

### Execução pelo orquestrador — 2026-11-23T02:06Z

- **Gate verde**: `./verify.sh` (02:04:57Z) `ALL STAGES OK`, **47 testes run: 47 passed** (41 anteriores + 6 novos), `VERIFY_EXIT=0`. Dois REDs determinísticos de clippy (`explicit_counter_loop`, `range_plus_one`) corrigidos em `crates/sim/tests/simulacao.rs`, sem `#[allow]`; `cargo fmt --all` aplicado uma vez. `lsp diagnostics` indisponível (rust-analyzer ausente).
- **Roteiro 10/10 PASS** (02:04–02:06Z), capturas em `/tmp/tmp.ADNoP5fSgj/saidas/`, relatório em `.specify/validations/coleta-materiais-deterministica/evidence.md`. Seis testes nomeados executados individualmente, todos `exit=0`, 1 passed cada.
- **Gates locais (02:06Z)**: SDD `39/0`, DAG `406 tarefas/104 arestas/0 ciclos`, task-id `0 duplicatas`, hub-docs `0 erros`; converge baselinado por bootstrap após esta entrada.
- **Fronteira**: `STATE.md` do worktree idêntico ao HEAD (memória local do SWE preservada); `crates/contratos`, `PROTOCOL_VERSION` e `Cargo.toml` intactos. `[NV]`: CI/merge do ciclo 039 e decisão do SWE sobre PR #48.

### Correção de bug da ACR e revalidação — 2026-11-23T02:14Z

- **Achado médio (comentário `7089743411`)**: `fase_skills` avançava a coleta por comando; dois comandos no mesmo tick podiam creditar antes de 6 s. Correção: `BTreeSet<u64>` local ao tick limita a um incremento por jogador por tick, preservando replay e ordem determinística. Mensagem de `DestinoInvalido` ampliada para cobrir coleta.
- **Verificação**: `verify.sh` `ALL STAGES OK`, **48/48 testes** (incluindo `dupla_coleta_no_mesmo_tick_nao_acelera_credito`); gates locais PASS. Histórico de 47/47 preservado; nenhuma reescrita.
- **Racional**: blocker semântico corrigido antes do merge; nenhuma mudança de protocolo, Cargo, docs de estado ou R6. `[NV]`: CI/merge no novo head e decisão do SWE sobre PR #48.
