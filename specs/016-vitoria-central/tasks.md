# Tasks — 016-vitoria-central

**Data:** 2026-11-20T09:48Z

Marcações `[X]` registram a frente de trabalho protocolada nesta etapa de escrita; execução de Godot/gates/`make check` e portarias R4/R7 permanecem **[NV]** e não são afirmadas aqui.

## Phase 1: Implementação

- [X] **T001** Manter em `nucleo/desafios/ilha_central.gd` (`extends RefCounted`, `class_name IlhaCentral`) as constantes `ILHAS_NECESSARIAS=3`, `CANALIZACAO_SEGUNDOS=4.0`, `EVENTO_VITORIA="vitoria_ilha_central"` e `FUNCOES_EXIGIDAS=["artesao","guerreiro"]`, o sinal `vitoria_ilha_central(resultado)` e o estado `canalizando`, `canalizacao_iniciada_em`, `canalizadores`, `progresso`, `vitoria_declarada`, `estado_persistido` e `eventos` (FR-001)
- [X] **T002** Garantir `iniciar_canalizacao(ilhas_controladas, funcoes_presentes, agora)` com `ILHAS_INSUFICIENTES` abaixo de 3 ilhas, `FUNCOES_AUSENTES`/`faltantes` sem Artesão e Guerreiro vivos e presentes por `peer_id`, aceite com `conclui_em=agora+4.0` e `progresso=0.0`, sem substitutos (FR-002, SC-001, SC-002)
- [X] **T003** Garantir `avancar(agora)` com progresso `agora - inicio` (tolerância `0.000001`), `CANALIZANDO` em 3,99 s, `VITORIA` em 4,0 s e evento único com `declarada_por="servidor"`, `estado_persistido="vitoria_ilha_central"` e sinal emitido uma vez (FR-003, SC-001)
- [X] **T004** Garantir `verificar_funcoes` por identidade de `peer_id`, `interromper()` com zeração de `progresso` e `reiniciar()` limpando vitória/eventos; morte ou saída de uma função interrompe e um novo período exige 4,0 s completos (FR-004, SC-003)
- [X] **T005** Integrar em `nucleo/mundo.gd`: preservar os preloads de `IlhaSincronia`, `IlhaTopo`, `PontoColeta` e `Funcoes`, acrescentar o de `IlhaCentral`, manter `definir_funcao`, `definir_presenca_central`, `_contar_ilhas_concluidas`, `_funcoes_no_centro`, `_atualizar_ilha_central` e `vitoria_declarada_persistida()`, e atualizar em `_fase_finish()` com `float(tick) * PASSO_SEGUNDOS` (FR-005)
- [X] **T006** Manter `hash_estado()` com `HashingContext`, incluindo `vitoria_declarada_persistida()` e `ilha_central.progresso`, e a recusa de campos forjados em `nucleo/sim/validacao.gd`; nenhum `Mundo.new()` com `:=` e nenhum `sha256_string` (FR-005)
- [X] **T007** Criar `tests/test_vitoria_central.gd` (`extends SceneTree`, tempo injetado, `: Mundo` explícito, `HashingContext`) com constantes 3/4,0, recusas, início, 3,99 s vs 4,0 s, evento/resultado do servidor, interrupção em 2,5 s, novo período 3,0 s→7,0 s, morte durante, `reiniciar()`, integração em 130 ticks e paridade de hash, imprimindo `PASS ...` e `OK test_vitoria_central` (FR-006, SC-001..SC-004)

## Phase 2: Verificação e convergência

- [X] **T008** Verificar marco da suíte: `PASS ...`, `OK test_vitoria_central`, exit `0` e zero `FAIL` com `"$G" --headless --path . --script res://tests/test_vitoria_central.gd`; saídas só entram com data/hora na portaria de validação na fase própria, sem escrever R4/R7 aqui (FR-007, SC-004)
- [X] **T009** Verificar porta de regressão `make check GODOT=$G` com exit `0` e os 5 gates determinísticos PASS (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) com o conjunto completo de artefatos da spec (FR-007, SC-005)
- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra GDD 1507 e 1537–1541, corrigir divergências na fonte e reexecutar os 5 gates; nenhuma alegação de partidas 5v5, UDP real ou escala de produção (seis ilhas/20 s) (SC-005)
