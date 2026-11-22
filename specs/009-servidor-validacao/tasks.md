# Tasks — 009-servidor-validacao

**Data:** 2026-11-20T03:00Z

## Phase 1: Implementação

- [X] **T001** Criar `nucleo/mundo.gd` com passo fixo 1/30 s e fases `prepare → skills → finish` (Task 2/Step 1; FR-001)
- [X] **T002** Criar `nucleo/servidor.gd` com `abrir`, `handle_join`, `aceitar_saida` e handshake por IP:porta (Task 2/Step 2; FR-002)
- [X] **T003** Criar `nucleo/main_server.gd` (`extends SceneTree`) com `--porta=` e `SERVIDOR_PRONTO` (Task 2/Step 3; FR-003)
- [X] **T004** Acrescentar `emitir_snapshot` e `avancar_com_snapshots` com acumulador 30→20 Hz (Task 2/Step 5; FR-004, SC-002)
- [X] **T005** Criar `nucleo/sim/validacao.gd` com os cinco códigos e `LIMITE_MOVIMENTO_M=1.0` (Task 3/Step 1; FR-005)
- [X] **T006** Ligar o validador em `ServidorAutoritativo.handle_command` como ponto único de aplicação (Task 3/Step 2; FR-006)
- [X] **T007** Acrescentar `Mundo.hash_estado()` com SHA-256 de `jogadores` + `tick` (Task 3/Step 3; FR-007)
- [X] **T008** Criar `tests/test_servidor.gd`, `tests/test_loopback.gd` e `tests/test_validacao.gd` (`extends SceneTree`) (Task 2/Steps 4–5, Task 3/Step 4; FR-008, SC-001/002/003)
- [X] **T009** Verificar marcos CE-2: 6/8/9 `PASS`, `SERVIDOR_PRONTO porta=7777` sob `timeout 3s` (status 124) e 5 gates PASS; registrar saídas com data/hora na portaria de validação (Task 2/Step 6, Task 3/Step 5; FR-009, SC-001/002/003/004/005)

## Phase 2: Convergence

- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra as Tasks 2–3 do plano, corrigir divergências na fonte e reexecutar os 5 gates (SC-005)
