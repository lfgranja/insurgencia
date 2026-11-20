# Tasks — 019-tela-resultado

**Data:** 2026-11-20T11:39Z

Marcações `[X]` registram a frente de trabalho protocolada nesta etapa de escrita; execução de Godot/gates/`make check` e portarias R4/R7 permanecem **[NV]** e não são afirmadas aqui. Nenhum item entrega main scene, `.tscn`, rede, lobby, 5v5, regra de produção ou vitória inventada.

## Phase 1: Implementação

- [X] **T001** Criar `nucleo/apresentacao/tela_resultado.gd` como `extends Control`, sem `@tool` e sem `class_name` dependente de resolução em headless. A tela não carrega cena, não altera `project.godot`, não é registrada como main scene e não cria `.tscn`; nenhum `load`/`instantiate` de recurso de cena (FR-001, SC-004)
- [X] **T002** Criar os rótulos de título e detalhe em `_init()` ou sob demanda, por instância, por código; nenhum nó externo, nenhuma cena e nenhum arquivo `.tscn`. Em `--headless` a instância é obtida por `preload` e `new()` com tipo explícito (FR-002, SC-004)
- [X] **T003** Definir `definir_resultado(evento: Dictionary) -> void` como único ponto de atualização da tela, lendo somente as chaves reais emitidas por `nucleo/desafios/ilha_central.gd`: `evento`, `declarada_por` e `tempo_segundos`; nenhum campo de comando do cliente é aceito como vitória pronta (FR-003, SC-001)
- [X] **T004** Fixar o critério do título: `VITORIA` somente se `evento.get("evento") == "vitoria_ilha_central"` **e** `evento.get("declarada_por") == "servidor"`; caso contrário `AGUARDANDO`. Comparação literal, sem dedução, sem aceitar declaração de cliente e sem reimplementar as três ilhas/4 s (FR-004, SC-001)
- [X] **T005** Preencher o rótulo de detalhe apenas com dados do dicionário recebido, como `tempo_segundos`, sem `Time`, `OS`, sono, aleatoriedade, rede ou estado externo; ausência de chave não inventa texto de vitória (FR-003, FR-005, SC-001)
- [X] **T006** Garantir reaplicação determinística: evento válido→inválido troca `VITORIA` por `AGUARDANDO`, inválido→válido troca `AGUARDANDO` por `VITORIA`, e duas instâncias com o mesmo evento produzem o mesmo par título/detalhe; sem cache, singleton ou estado oculto (FR-004, FR-005, SC-002)
- [X] **T007** Escrever `tests/test_tela_resultado.gd` com `extends SceneTree`, `const TelaResultadoScript = preload("res://nucleo/apresentacao/tela_resultado.gd")`, instâncias tipadas explicitamente, TAB, tipos explícitos e no mínimo 12 verificações: instância é `Control`; rótulos existem sem cena; estado inicial `AGUARDANDO`; evento real do servidor gera `VITORIA`; `declarada_por="cliente"` gera `AGUARDANDO`; outro `evento` gera `AGUARDANDO`; dicionário vazio gera `AGUARDANDO`; chave `evento` ausente gera `AGUARDANDO`; chave `declarada_por` ausente gera `AGUARDANDO`; grafia/caixa diversa gera `AGUARDANDO`; reaplicação nos dois sentidos; detalhe com `tempo_segundos` sem relógio; determinismo entre instâncias. Imprimir `PASS ...`, usar `printerr` para `FAIL`, fechar com `OK test_tela_resultado` e `quit(0)`/`quit(1)` (FR-006, FR-007, SC-002)
- [X] **T008** Fechar o escopo negado: nenhuma main scene nova, nenhum `.tscn`, nenhuma regra de produção (seis ilhas/Plano Mestre/20 s), nenhum socket, UDP/ENet, lobby, NAT, matchmaking, 5v5, gate, baseline, `evidence.csv`, `Makefile` ou artefato R4/R7 alterado nesta etapa (FR-008, SC-004)

## Phase 2: Verificação

- [X] **T009** Verificar com `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`: `"$G" --headless --version`; `"$G" --headless --path . --script res://tests/test_tela_resultado.gd` com ≥12 verificações, todo `PASS`, `OK test_tela_resultado`, exit `0` e zero `FAIL`; `make check GODOT=$G` com exit `0`; e os 5 gates determinísticos PASS (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`). Saída executada entra com data/hora somente na portaria R4 (validate), que precede R7 (release com SemVer, changelog FR/T/bug e rollback); nenhum artefato das portarias é escrito aqui (FR-008, SC-002, SC-003)

## Phase 3: Convergence READ-ONLY

- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra GDD §16.4 (linhas 1539 e 1541) e contra as chaves literais de `nucleo/desafios/ilha_central.gd`, corrigindo divergências na fonte e reexecutando os 5 gates. Manter zero main scene, zero `.tscn`, zero rede, zero vitória inventada, zero 5v5 e zero artefato R4/R7; não observado permanece **[NV]**; R4 precede R7 e nenhuma alegação de comando executado nesta etapa é feita (FR-008, SC-004, SC-005)
