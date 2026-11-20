# Tasks — 013-ilha-topo

**Data:** 2026-11-20T09:28Z

Marcações `[X]` registram a frente de trabalho protocolada nesta etapa de escrita; execução de Godot/gates/`make check` e portarias R4/R7 permanecem **[NV]** e não são afirmadas aqui.

## Phase 1: Implementação

- [X] **T001** Criar `nucleo/desafios/ilha_topo.gd` com `ORDEM=[1,2,3]`, 8 s por alavanca, reinício em 2 s, limite de 45 s e os campos `indice`, `iniciada_em`, `reinicio_ate`, `capturada`, `aviso_visual` e `estado_persistido` (FR-001)
- [X] **T002** Implementar `sinalizar_alavanca(alavanca, agora)` com os motivos literais, a janela de 8 s por alavanca e a captura até 45 s que grava `ilha_topo_concluida` (FR-002, SC-001)
- [X] **T003** Implementar `reiniciar()` com `reinicio_ate = agora + 2.0`, `aviso_visual`, limpeza de captura/estado e recusa de tentativas durante o reinício (FR-002, SC-002)
- [X] **T004** Declarar `ilha_topo` em `modulos/exemplo/mapa.json` com `alavancas=[1,2,3]`, `janela_alavanca_segundos=8.0`, `reinicio_segundos=2.0` e `limite_segundos=45.0`, preservando `ilha_meio` (FR-003)
- [X] **T005** Integrar `ilha_topo`, o ramo `sinalizar_alavanca` em `_fase_skills()`, o aviso no snapshot e a cópia de `estado_persistido` ao tick de `Mundo` com `float(tick) * PASSO_SEGUNDOS` (FR-004)
- [X] **T006** Validar `sinalizar_alavanca` em `nucleo/sim/validacao.gd`: alvo 1..3, recusa de `JANELA_EXPIRADA`/`LIMITE_EXCEDIDO` e nenhuma mutação de captura pelo cliente (FR-005, SC-003)
- [X] **T007** Criar `tests/test_ilha_topo.gd` (`extends SceneTree`, tempo injetado) com captura em ordem, erro fora de ordem com reinício em 2 s, três ativações fora da janela recusadas e `reiniciar()`, imprimindo `PASS ...` e `OK test_ilha_topo` (FR-006, SC-004)

## Phase 2: Verificação e convergência

- [X] **T008** Verificar marco da suíte: `PASS ...`, `OK test_ilha_topo`, exit `0` e zero `FAIL` com `"$G" --headless --path . --script res://tests/test_ilha_topo.gd`; saídas só entram com data/hora na portaria de validação na fase própria, sem escrever R4/R7 aqui (FR-008, SC-004)
- [X] **T009** Verificar porta de regressão `make check GODOT=$G` com exit `0` e os 5 gates determinísticos PASS (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) com o conjunto completo de artefatos da spec (FR-008, SC-005)
- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra GDD 1502 e 1518–1521, corrigir divergências na fonte e reexecutar os 5 gates (SC-005)
