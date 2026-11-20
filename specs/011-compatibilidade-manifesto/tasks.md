# Tasks — 011-compatibilidade-manifesto

**Data:** 2026-11-20T03:15Z

Marcações `[X]` registam a frente de trabalho protocolada nesta etapa de escrita; execução de Godot/gates e portarias R4/R7 permanecem **[NV]** e não são afirmadas aqui.

## Phase 1: Implementação

- [X] **T001** Criar `manifesto.json` com os 6 campos obrigatórios (`manifest_version`, `protocol_version`, `module_id`, `content_hash` de 64 hex, `checked_at`, `tool_version`) (Task 5/Step 1; FR-001, CE-4.1)
- [X] **T002** Criar `nucleo/compatibilidade/manifesto.gd` com `CAMPOS`, `carregar` e `validar_formato`, devolvendo `MANIFEST_INVALID` para manifesto ausente/incompleto e `OK` para o formato válido (Task 5/Step 1; FR-002, CE-4.4)
- [X] **T003** Implementar `conferir` com a ordem formato→versão→módulo→comprimento do hash→arquivo de regras→hash, usando `int(...)` para a versão e os quatro códigos literais de `Protocolo` (Task 5/Step 1; FR-003, CE-4.2/CE-4.3)
- [X] **T004** Criar `ferramentas/verificar_compatibilidade.gd` (`extends SceneTree`) com `COMPAT_OK module_id=<id>`/`COMPAT_FALHA <codigo>`, `quit(0)`/`quit(1)` e `CLIENTE_VERSAO_PROTOCOLO`, `CLIENTE_HASH_MODULO`, `CLIENTE_ERRO` (Task 5/Step 2; FR-004, CE-4.4/CE-4.6)
- [X] **T005** Criar o alvo `check-compat` no `Makefile` com `GODOT ?= godot` e `--headless --path . --script res://ferramentas/verificar_compatibilidade.gd`, sem sondagem de lobby/worker e sem publicação de release (Task 5/Step 3; FR-005, SC-002)
- [X] **T006** Criar `tests/test_manifesto.gd` com as verificações `manifesto_original_aceito`, `manifesto_ausente_recusado`, `manifesto_incompleto_recusado`, `divergencia_de_versao_falha` e `divergencia_de_hash_falha` (Task 5/Step 4; FR-006, CE-4.1/CE-4.2/CE-4.3/CE-4.4)
- [X] **T007** Completar as 7 verificações com `modulo_ausente_falha` e `arquivo_de_regras_ausente_falha`, imprimindo `OK test_manifesto` + `quit(0)` com zero falhas ou `FALHAS <n>` + `quit(1)`, sem linhas `FAIL` (Task 5/Step 4; FR-006, SC-001)
- [X] **T008** Verificar marco CE-4: 7 `PASS` + `OK test_manifesto` + exit 0; `make check-compat` com `COMPAT_OK module_id=insurgencia.exemplo` + exit 0; 3 execuções com saída idêntica por `diff -q`; e divergência de hash com `COMPAT_FALHA HASH_MODULO_DIVERGENTE`, status 1 capturado e `modulos/exemplo/regras.cfg` restaurado (Task 5/Step 5; SC-001/002/003/004)
- [X] **T009** Executar os 5 gates determinísticos do alvo e registar as saídas com data/hora na portaria de validação na fase própria, sem escrever R4/R7 aqui (Task 5/Step 5; FR-007, SC-004)

## Phase 2: Convergence

- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra a Task 5 do plano, corrigir divergências na fonte e reexecutar os 5 gates (SC-004)
