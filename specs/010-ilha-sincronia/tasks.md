# Tasks — 010-ilha-sincronia

**Data:** 2026-11-20T03:10Z

Marcações `[X]` registram a frente de trabalho protocolada nesta etapa de escrita; execução de Godot/gates e portarias R4/R7 permanecem **[NV]** e não são afirmadas aqui.

## Phase 1: Implementação

- [X] **T001** Criar `nucleo/desafios/ilha_sincronia.gd` com as cinco constantes e os campos de janela, placas, captura, exposição, amostras e `estado_persistido` (Task 4/Step 1; FR-001)
- [X] **T002** Implementar `sinalizar`, `_iniciar_gas`, `aplicar_gas` e `reiniciar` com os motivos literais e o dano de 8 por amostra de 1,0 s (Task 4/Step 1; FR-002, SC-003)
- [X] **T003** Criar `modulos/exemplo/mapa.json` com `ilha_meio`, `placas=[1, 2]`, `janela_segundos=5.0` e `jogadores_necessarios=2` (Task 4/Step 2; FR-003)
- [X] **T004** Criar `modulos/base/regras.cfg` com a regra de produção de 3 jogadores e janela de 5 s, separada do Módulo de exemplo (Task 4/Step 7; FR-004)
- [X] **T005** Integrar `ilha_sincronia` e `estado_persistido` ao tick de `Mundo` em `_fase_skills` e `_fase_finish`, com gás, dano e expulsão (Task 4/Step 4; FR-005)
- [X] **T006** Recusar placa fora de 1..2 com `PLACA_INVALIDA` em `nucleo/sim/validacao.gd`, ponto único de aplicação (Task 4/Step 3; FR-006)
- [X] **T007** Criar `tests/test_ilha_sincronia.gd` (`extends SceneTree`) com 14 verificações, incluindo 24 pontos em 3 s e vida 100/92/84/76 (Task 4/Step 5; FR-007, SC-001, SC-003)
- [X] **T008** Criar `tests/test_loopback_ilha.gd` (`extends SceneTree`) com `DELTA_EVENTO_MS` e 3 verificações entre regra base e exemplo (Task 4/Step 7; FR-007, SC-002)
- [X] **T009** Verificar marcos CE-3: 14 `PASS` + `OK test_ilha_sincronia` + exit 0, `DELTA_EVENTO_MS` + 3 `PASS` + `OK test_loopback_ilha` e 5 gates PASS; registrar saídas com data/hora na portaria de validação na fase própria, sem escrever R4/R7 aqui (Task 4/Steps 6–7; FR-008, SC-001/002/003/004)

## Phase 2: Convergence

- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra a Task 4 do plano, corrigir divergências na fonte e reexecutar os 5 gates (SC-004)
