# Tasks — 017-rede-e2e

**Data:** 2026-11-20T10:00Z

Marcações `[X]` registram a frente de trabalho protocolada nesta etapa de escrita; execução de Godot/gates/`make check`/`make test-rede-e2e` e portarias R4/R7 permanecem **[NV]** e não são afirmadas aqui.

## Phase 1: Implementação

- [X] **T001** Manter em `nucleo/servidor.gd` `PORTA_PADRAO=7777`, `MAX_CLIENTES=4`, `abrir()` com `ENetMultiplayerPeer.create_server()` e `Engine.get_main_loop().get_multiplayer()`, `SERVIDOR_PRONTO`, `peers_aceitos`, `aceitar_saida`, `emitir_snapshot()` e `avancar_com_snapshots()`; o cenário usa `--porta=7780`, sem alterar os demais alvos (FR-001)
- [X] **T002** Ligar o sinal `peer_packet` da API de multiplayer a `handle_join(peer_id, dados)` e `handle_command(peer_id, dados)`, decodificando o envelope JSON próprio e usando o `peer_id` do transporte; nenhuma validação duplicada no callback e o fio de alto nível tratado como implementation detail sobre ENet/UDP (FR-001, FR-003)
- [X] **T003** Enviar `join_accept` (`aceito=true`, `codigo="OK"`, `peer_id`) ao peer de origem quando `handle_join` aceita, registrando `peers_aceitos` e chamando `mundo.adicionar_jogador`; responder `join_reject` com `REJEICAO_VERSAO`/`REJEICAO_MODULO`/`REJEICAO_HASH` sem adicionar o jogador (FR-002, SC-002)
- [X] **T004** Manter envelope com `tipo` (JOIN/COMANDO), `Protocolo.normalizar_pacote`, `PROTOCOL_VERSION=1`, `MODULO_ID`, `content_hash` de 64 hex, `request_id`, `seq`, `acao`, `alvo` e `destino`; nenhuma posição, vida, ilha ou vitória pronta é aceita do cliente (FR-003)
- [X] **T005** Preservar `ValidacaoComandos.validar` e `handle_command` como único ponto de COMANDO pela rede: exige peer aceito, recusa `PEER_NAO_AUTORIZADO`/`SEQ_OU_REQUEST_REPLAY`/`ACAO_DESCONHECIDA`/`MOVIMENTO_FORA_DO_LIMITE`, atualiza `seq`/`request_id` no servidor e agenda em `mundo`; captura de ilha sem condição mantém a ilha neutra (FR-005, SC-003)
- [X] **T006** Escrever `tests/cliente_rede_e2e.gd` (`extends SceneTree`, `create_client("127.0.0.1", 7780)`) que envia JOIN e COMANDO pela rede, não chama os handlers diretamente, imprime `CLIENTE_PRONTO`, `JOIN_ACEITO`/`JOIN_RECUSADO`, `COMANDO_ACEITO`/`COMANDO_RECUSADO` e `OK cliente_rede`, termina com `quit(0)`/`quit(1)` e inclui o modo de JOIN divergente (FR-002, FR-004, SC-001..SC-003)
- [X] **T007** Escrever `ferramentas/rede/orquestrador_rede_e2e.py`: servidor + exatamente 2 clientes headless, espera `SERVIDOR_PRONTO porta=7780`, exige `JOIN_ACEITO` e `OK cliente_rede` para ambos, timeout determinístico, limpeza em `finally` e exit 0 somente com zero `FAIL` (FR-006, SC-001, SC-004)
- [X] **T008** Acrescentar ao `Makefile` somente o alvo `test-rede-e2e`, preservando `check`, `check-compat` e os alvos `test-*` existentes; nenhuma alteração de gates, baselines ou `evidence.csv` (FR-006, FR-007)

## Phase 2: Verificação e convergência

- [X] **T009** Verificar `make test-rede-e2e GODOT=$G` com `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`: `SERVIDOR_PRONTO porta=7780`, dois `OK cliente_rede`, exit `0`, zero `FAIL` e nenhum processo órfão; verificar `make check GODOT=$G` com exit `0` e os 5 gates determinísticos PASS (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`). Saídas efetivamente executadas entram com data/hora na portaria R4 na fase própria; versão, changelog FR/T e rollback em R7 — nenhum artefato R4/R7 escrito aqui (FR-007, SC-004, SC-005)
- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra GDD `:1171`, `:1514`, `:1515`, `:1582`, `:1588` e as restrições de R1, corrigir divergências na fonte e reexecutar os 5 gates; nenhuma alegação de NAT, lobby, produção, escala 5v5 ou de comando executado nesta etapa (SC-005)
