# Tasks — 014-ilha-inferior

**Data:** 2026-11-20T09:38Z

Marcações `[X]` registram a frente de trabalho protocolada nesta etapa de escrita; execução de Godot/gates/`make check` e portarias R4/R7 permanecem **[NV]** e não são afirmadas aqui.

## Phase 1: Implementação

- [X] **T001** Criar `nucleo/desafios/ilha_inferior.gd` com `CICLO_SEGUNDOS=3.0`, `MATERIAIS_POR_CICLO=5`, `CICLOS_ESTOQUE=3`, `RECARGA_SEGUNDOS=20.0`, `EVENTO_COLETA` e os campos `proximo_ciclo_em`, `ciclos_restantes`, `recarregando_ate`, `materiais_creditados` e `eventos` (FR-001)
- [X] **T002** Implementar `coletar(peer_id, agora)` com os motivos literais, crédito de 5 Materiais por ciclo de 3 s e evento `ilha_inferior_coleta` com `tempo_segundos=agora` (FR-002, SC-001)
- [X] **T003** Implementar esgotamento no 3º ciclo com `recarregando_ate = agora + 20.0`, recusa `RECARGA_EM_ANDAMENTO`, `_recarregar()` e `reiniciar()` com limpeza de janela, estoque, recarga, saldo e eventos (FR-003, SC-002)
- [X] **T004** Integrar em `nucleo/mundo.gd`: preservar os preloads de `IlhaSincronia`/`IlhaTopo`, acrescentar o de `IlhaInferior`, criar `var ilha_inferior`, adicionar o ramo `coletar` em `_fase_skills()` com `float(tick) * PASSO_SEGUNDOS` e expor `materiais`/recarga sem copiar valores do comando (FR-004)
- [X] **T005** Validar em `nucleo/sim/validacao.gd`: incluir `coletar` em `ACOES_PERMITIDAS`, recusar `materiais`/`quantidade`/`credito` com `CREDITO_FORJADO` e `alvo` fora do id do ponto com `PONTO_INVALIDO`, antes de qualquer mutação (FR-005, SC-003)
- [X] **T006** Declarar `ilha_inferior` em `modulos/exemplo/mapa.json` com `tipo="coleta"`, `ciclo_segundos=3.0`, `materiais_por_ciclo=5`, `ciclos_estoque=3` e `recarga_segundos=20.0`, preservando `ilha_meio` e `ilha_topo` (FR-006)
- [X] **T007** Criar `tests/test_ilha_inferior.gd` (`extends SceneTree`, tempo injetado, `: Mundo` explícito) com crédito por ciclo, esgotamento/recarga, recusa de material forjado e `reiniciar()`, imprimindo `PASS ...` e `OK test_ilha_inferior` (FR-006, SC-001..SC-004)

## Phase 2: Verificação e convergência

- [X] **T008** Verificar marco da suíte: `PASS ...`, `OK test_ilha_inferior`, exit `0` e zero `FAIL` com `"$G" --headless --path . --script res://tests/test_ilha_inferior.gd`; saídas só entram com data/hora na portaria de validação na fase própria, sem escrever R4/R7 aqui (FR-007, SC-004)
- [X] **T009** Verificar porta de regressão `make check GODOT=$G` com exit `0` e os 5 gates determinísticos PASS (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) com o conjunto completo de artefatos da spec (FR-007, SC-005)
- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra GDD 1504 e 1529–1531, corrigir divergências na fonte e reexecutar os 5 gates (SC-005)
