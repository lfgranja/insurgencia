# Implementation Plan — 041-coleta-jogavel

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 antes de R7 `1.0.0-local`; `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação fica registrada, não executada.

**Fronteira explícita**: este agente escreve **somente** `specs/041-coleta-jogavel/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/coleta-cliente-041/` (branch `041-coleta-jogavel`, base `bf05cfa`). Não toca `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, código-fonte (salvo agentes de implementação na etapa 12), documentos derivados nem alterações preexistentes. Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]**.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`.
2. **01 Intake**: sessão `20261123T035400Z-coleta-jogavel`, título `insurgencia - coleta jogavel no cliente`; 80/20 = 500/625 (coleta jogável no cliente) contra 250/625 (só doc comment de `destino`), 108/625 (ponte provisória) e 10/625 (netcode).
3. **02 Research**: GDD §7.1 (6 s = 180 ticks a 1/30 s → +3 Suprimentos/+2 Materiais; saída zera parcial) e `crates/sim/src/lib.rs` lidos; confirmados `TICKS_COLETA = 180`, campos `materiais`/`suprimentos`/`coleta_ticks`/`coleta_ponto`, `hash_estado` refletivo e o padrão `BTreeSet<u64>` do ciclo 040; R6 N/A citada com fonte.
4. **03 Define (problem)**: coleta não jogável sem auto-avanço e sem feedback no cliente; critérios observáveis por FR/SC.
5. **04 Shape (concept)**: 4 opções avaliadas (coleta jogável, barricada/HUD, combate/Fôlego, servidor+Lightyear); recomendação = coleta jogável no cliente.
6. **05 Decide**: scorecard 5×5×5×4 = **500/625** (contra 250/625, 108/625 e 10/625), `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006; regra canônica preservada e auto-avanço separado do protocolo.
8. **07 Clarify**: auto-avanço ≠ reenvio de comando; limite por jogador ≠ global; `mover` para o mesmo ponto ≠ cancelamento; `C` do cliente ≠ teclas de outras seções; doc comment de `Comando.destino` ≠ mudança de contrato; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = revisão de requisitos, **não** implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: `analyze.md` fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos — auto-avanço `BTreeSet<u64>`, regra canônica preservada, validação/dedup, doc comment de `Comando.destino`, cliente `C`/HUD — com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`. Fora da fronteira deste agente.
15. **13 Converge (append-only)**: primeira entrada em `specs/041-coleta-jogavel/converge.md`, datada; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade, rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Desenho da implementação futura (fora da fronteira SDD — **[NV]**)
Escopo de código: **`crates/sim/src/lib.rs`** e **`crates/cliente/src/main.rs`**.
- **Core**: em `fase_skills`, acrescentar o auto-avanço da coleta com o mesmo padrão de `BTreeSet<u64>` local já usado por `coleta_avancada`/`construcao_avancada`; enquanto `coleta_ponto` for igual à posição, incrementar `coleta_ticks` em 1 por jogador por tick; ao atingir `TICKS_COLETA = 180`, creditar `+3` suprimentos e `+2` materiais atomicamente e zerar o contador; `mover` com posição diferente zera `coleta_ticks`/`coleta_ponto`; um comando no mesmo tick não conta duas vezes.
- **Preservar**: `TICKS_COLETA = 180`, `PASSO_SEGUNDOS = 1/30`, os quatro campos de `Jogador` e a inclusão deles em `hash_estado` (ciclo 039).
- **Doc comment**: `Comando.destino` passa a citar `mover`, `coletar_materiais` e `construir_barreira` (em `crates/sim/src/lib.rs`, não em `crates/contratos`).
- **Cliente**: tecla `C` em *just pressed* agenda `coletar_materiais` com `destino` = posição atual e `seq`/`request_id` monotônicos via recurso local; HUD passa a mostrar Suprimentos e progresso de Coleta (ticks/180), além de Materiais/Barreiras/Construindo.
- **Não tocar**: `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`; nenhuma constante canônica ou recompensa é reaberta.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/coleta-jogavel/test-roadmap.md` (fora da fronteira deste agente). Superfície: core determinístico + cliente/HUD; capturas de terminal; coluna `automatizável?`; rollback doc/estado.

1. Confirmar que R6 está fora do fluxo por constituição v1.2.2 (PARE se invocada; nenhuma issue simulada; dívida de automação registrada).
2. Conferir 6 FR e 6 SC na spec e o mapeamento um-para-um no checklist; T001..T012 únicos em DAG.
3. Verificar `TICKS_COLETA = 180`, os quatro campos de `Jogador` e o `hash_estado` refletivo, sem tocar `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`.
4. Validar um único comando válido `coletar_materiais` → conclusão em exatamente 180 ticks com +3 Suprimentos e +2 Materiais.
5. Validar a guarda `BTreeSet<u64>`: comandos repetidos no mesmo tick avançam 1; jogadores diferentes avançam no mesmo tick.
6. Validar cancelamento por `mover` com mudança de posição (zera parcial, sem crédito) e continuidade com `mover` para a mesma posição.
7. Validar recusa com `destino` ausente ou diferente da posição (`DestinoInvalido`, sem avanço).
8. Comparar `hash_estado` entre duas execuções com o mesmo input e conferir divergência quando acúmulo/estoques mudam.
9. Verificar tecla `C` em *just pressed*: um comando, `seq`/`request_id` monotônicos, `destino` = posição atual; HUD com Materiais/Barreiras/Construindo/Suprimentos/Coleta; doc comment de `Comando.destino` com os três comandos.
10. Rodar gates/build/testes e comparar notas com o estado ao vivo; registrar resultados como observados ou **[NV]** — nunca afirmar verde sem saída colada.

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 auto-avanço `BTreeSet`/crédito atômico | 12 implement | T011 |
| FR-002 regra canônica/estado/hash preservados | 12 implement | T011 |
| FR-003 validação/dedup/limite por jogador | 12 implement | T011 |
| FR-004 doc comment `Comando.destino`/fronteira | 12 implement | T011 |
| FR-005 tecla `C`/HUD | 12 implement | T011 |
| FR-006 estilo/governança/R6 N/A | 00/12/26 | T001, T011, T012 |

Os 4 artefatos deste agente são produzidos por T005 (`spec.md`), T006 (`plan.md`), T007 (`checklists/requirements.md`) e T009 (`tasks.md`); T008 registra a revisão do checklist. T010–T012 e o código de T011 estão fora da fronteira de escrita deste agente e seguem a ordem v1.2.2.

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Coleta não jogável sem reenvio de comando | FR-001: auto-avanço 1 tick/jogador/tick enquanto o ponto permanecer igual |
| Avanço duplicado no mesmo tick | FR-001/FR-003: guarda `BTreeSet<u64>` local (padrão do ciclo 040) |
| Crédito parcial indevido na saída | FR-002/FR-003: crédito atômico só em 180 ticks; `mover` com mudança zera o parcial |
| Divergência entre clientes | FR-002 + `hash_estado` refletivo; comparação no passo 8 do roteiro |
| Reabertura de valores canônicos | Clarificações: GDD §7.1 e H4 permanecem intactos; hipóteses não viram requisito |
| Doc comment induzindo mudança de contrato | FR-004: correção em `crates/sim/src/lib.rs`; nada em `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml` |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida registrada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) antes de R7 (T012) |
| Afirmar verde sem execução | Gates/build/testes **[NV]** até o orquestrador; nada afirmado como verde sem saída colada |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md` e código intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)
- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`expect`/`#[allow]`.
- Confirmar determinismo com o roteiro de 10 passos e registrar capturas; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo e a dívida de automação registrada, sem `gh` improvisado.
