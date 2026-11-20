# Tasks — 012-evidencia-make-check

**Data:** 2026-11-20T03:19Z

Marcações `[X]` registam a frente de trabalho protocolada nesta etapa de escrita; execução de Godot/gates e portarias R4/R7 permanecem **[NV]** e não são afirmadas como artefatos escritos aqui.

## Phase 1: Implementação

- [X] **T001** Completar o `Makefile` com o alvo `check` como `.PHONY`, `GODOT ?= godot`, `GODOT_FLAGS := --headless --path . --script` e dependências `test-contrato test-servidor test-validacao test-ilha test-manifesto check-compat` nessa ordem, com falha rápida e sem rede/conta/Cargo/.NET (Task 6/Step 1; FR-001, SC-001)
- [X] **T002** Manter os alvos `test-contrato` e `test-servidor` invocando `res://tests/test_contrato.gd` e `res://tests/test_servidor.gd`, sem editar os scripts (Task 6/Step 1, `Test:`; FR-004)
- [X] **T003** Manter os alvos `test-validacao` e `test-ilha` invocando `res://tests/test_validacao.gd` e `res://tests/test_ilha_sincronia.gd`, sem editar os scripts (Task 6/Step 1, `Test:`; FR-004)
- [X] **T004** Manter `test-manifesto` e `check-compat` invocando `res://tests/test_manifesto.gd` e `res://ferramentas/verificar_compatibilidade.gd`, preservando `COMPAT_OK module_id=insurgencia.exemplo` (Task 6/Step 1; FR-004, SC-001)
- [X] **T005** Criar `docs/progress/evidence.csv` com o cabeçalho literal de 9 colunas `id,criterio,verdict,source,command,result,confidence,unverified,recorded_at`, sem renomear ou reordenar campos (Task 6/Step 2; FR-002, SC-002)
- [X] **T006** Povoar as linhas `EV-20261119-001` a `EV-20261119-007`, uma por critério CE-1..CE-5, com `source` e `command` literais e os testes de loopback ligados a CE-2.1/CE-2.5 e CE-3.6 (Task 6/Step 2; FR-002)
- [X] **T007** Aplicar a regra inicial a toda linha: `verdict=NAO_VERIFICADO`, `result` e `recorded_at` vazios, `unverified` não vazio; transição para `PASS` com `confidence` e carimbo somente após comando executado e saída observada (Task 6/Step 2; FR-005, SC-003)
- [X] **T008** Criar `docs/progress/2026-11-19-ciclo-80-20.md` com título datado, ponteiro para `docs/progress/evidence.csv` e os ids, estado `ilha_concluida`, fora do escopo GDD §16.3/§16.4 e marco de reavaliação após `make check` (Task 6/Step 2; FR-003, SC-004)
- [X] **T009** Verificar o marco CE-1..CE-5: `make check` com a sequência `OK test_contrato`/`OK test_servidor`/`OK test_validacao`/`OK test_ilha_sincronia`/`OK test_manifesto`/`COMPAT_OK` e exit 0; `grep -c` do marcador `≥ 1`; `test -f` do registro datado; `make check 2>&1 | tail -1` copiado literalmente na linha CE-5.1; e os 5 gates PASS, com registo em R4 na fase própria sem escrever R4/R7 aqui (Task 6/Step 3; FR-006, SC-001/003/004/005)
- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra a Task 6 do plano (`docs/plans/2026-11-19-ciclo-80-20-omoba.md:1219-1304`), corrigir divergências na fonte e revalidar os 5 gates, sem alegação de 39/39 caminhos (SC-005)

## Phase 2: Convergence

- A convergência é **T010**: artefato derivado nunca é usado para mascarar divergência; a correção vai sempre na fonte e a revalidação usa os 5 gates do alvo. Nenhuma alegação de cobertura integral da árvore canônica (39/39) é feita; o não executado permanece **[NV]**.
