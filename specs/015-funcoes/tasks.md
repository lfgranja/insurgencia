# Tasks — 015-funcoes

**Data:** 2026-11-20T09:41Z

Marcações `[X]` registram a frente de trabalho protocolada nesta etapa de escrita; execução de Godot/gates/`make check` e portarias R4/R7 permanecem **[NV]** e não são afirmadas aqui.

## Phase 1: Implementação

- [X] **T001** Manter em `nucleo/sim/funcoes.gd` (`extends RefCounted`, `class_name Funcoes`) as constantes `CUSTO_BARREIRA_MATERIAIS=20`, `CANALIZACAO_BARREIRA_SEGUNDOS=1.5`, `PV_BARREIRA=200`, `DURACAO_BARREIRA_SEGUNDOS=12.0`, `CANALIZACAO_ATAQUE_SEGUNDOS=1.2`, `DANO_BASE=100`, `DANO_ATAQUE_FORTE=200`, `RECARGA_ATAQUE_SEGUNDOS=6.0`, `TOLERANCIA=0.000001` e os dicionários `canalizacao`, `barreiras`, `recarga_ate` (FR-001)
- [X] **T002** Garantir `construir_barreira(peer_id, agora, materiais)` com `MATERIAIS_INSUFICIENTES` abaixo de 20, `CANALIZACAO_OU_BARREIRA_ATIVA`, canalização de 1,5 s, `conclui_em=agora+1.5`, `pv=200` e criação/expiração em `avancar()` com `expira_em=agora+12.0`, com o saldo lido do servidor (FR-002, SC-001)
- [X] **T003** Garantir `receber_dano(peer_id, dano, agora)` com absorção até 200 PV, `barreira_destruida`, interrupção da canalização com `interrompida=true` e `reembolso=20`, e `dano_restante` devolvido ao Núcleo (FR-003, SC-002)
- [X] **T004** Garantir `atacar(peer_id, agora)` com canalização de 1,2 s, `dano=200` (2×100), `recarga_ate=agora+6.0`, recusa `EM_RECARGA` e limpeza da recarga expirada em `avancar()` (FR-004, SC-003)
- [X] **T005** Integrar em `nucleo/mundo.gd`: preservar os preloads de `IlhaSincronia`/`IlhaTopo`/`PontoColeta`, acrescentar o de `Funcoes`, criar `var funcoes`, adicionar os ramos `construir_barreira` e `atacar` em `_fase_skills()` com `float(tick) * PASSO_SEGUNDOS`, debitar 20 Materiais só no aceite e chamar `funcoes.avancar()` em `_fase_finish()` aplicando `dano_restante` (FR-005)
- [X] **T006** Validar em `nucleo/sim/validacao.gd`: incluir `construir_barreira` e `atacar` em `ACOES_PERMITIDAS`, recusar `materiais`/`credito`/`saldo`/`quantidade`/`valor`/`materiais_creditados`/`dano` com `MATERIAIS_FORJADOS`/`DANO_FORJADO` e a recarga no servidor, antes de qualquer mutação (FR-006, SC-003)
- [X] **T007** Criar `tests/test_funcoes.gd` (`extends SceneTree`, tempo injetado, `: Mundo` explícito) com cenários de 1534–1535, interrupção por dano, expiração em 12 s, recarga em 6 s e `reiniciar()`, imprimindo `PASS ...` e `OK test_funcoes` (FR-007, SC-001..SC-004)

## Phase 2: Verificação e convergência

- [X] **T008** Verificar marco da suíte: `PASS ...`, `OK test_funcoes`, exit `0` e zero `FAIL` com `"$G" --headless --path . --script res://tests/test_funcoes.gd`; saídas só entram com data/hora na portaria de validação na fase própria, sem escrever R4/R7 aqui (FR-007, SC-004)
- [X] **T009** Verificar porta de regressão `make check GODOT=$G` com exit `0` e os 5 gates determinísticos PASS (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) com o conjunto completo de artefatos da spec (FR-007, SC-005)
- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra GDD 1505–1506 e 1534–1535, corrigir divergências na fonte e reexecutar os 5 gates (SC-005)
