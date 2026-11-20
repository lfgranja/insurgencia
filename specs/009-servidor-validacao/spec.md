# Feature Specification: Servidor headless e validação autoritativa

**Feature Branch**: `009-servidor-validacao` · **Created**: 2026-11-20 · **Status**: Draft · **Data:** 2026-11-20T03:00Z

**Input**: Tasks 2–3 de `docs/plans/2026-11-19-ciclo-80-20-omoba.md` (linhas 304–719), literais.

## User Scenarios & Testing

### User Story 1 — Servidor headless aceita conexão direta (P1)
Mantenedor sobe o servidor com `--headless`; dois clientes conectam por IP:porta (sem lobby e sem catálogo) e recebem snapshots periódicos.

**Independent Test**: `SERVIDOR_PRONTO porta=7777` sob `timeout 3s` (status 124), `test_servidor.gd` (6 `PASS`) e `test_loopback.gd` (8 `PASS`, 20 snapshots em 1 s).

### User Story 2 — Intenções do cliente são validadas no servidor (P1)
Posição, vida e estado pronto forjados pelo cliente são descartados por `normalizar_pacote`; sequência repetida, ação desconhecida e salto acima do limite são recusados.

**Independent Test**: `test_validacao.gd` (9 `PASS`), incluindo paridade por `hash_estado`.

## Requirements

- **FR-001**: `Mundo` (`extends RefCounted`, `class_name Mundo`) com `PASSO_SEGUNDOS = 1.0/30.0`, `tick`, `epoch=1`, `match_id=1`, `jogadores`, `comandos_pendentes`, `comandos_do_tick`, `adicionar_jogador`, `remover_jogador`, `agendar_comando` e `avancar_tick` nas fases explícitas `prepare → skills → finish`.
- **FR-002**: `ServidorAutoritativo` (`extends Node`) com `PORTA_PADRAO=7777`, `MAX_CLIENTES=4`, `abrir`, `handle_join`, `aceitar_saida`, `peers_aceitos` e `mundo`; `JOIN` passa por `Protocolo.normalizar_pacote` e recusa versão, módulo ausente e hash com `REJEICAO_VERSAO`, `REJEICAO_MODULO`, `REJEICAO_HASH`. Conexão direta por IP:porta, sem serviço externo.
- **FR-003**: `nucleo/main_server.gd` (`extends SceneTree`) cria o servidor, lê `--porta=` de `OS.get_cmdline_user_args()` (padrão 7777), conecta-o ao `root` e imprime `SERVIDOR_PRONTO porta=<n>`; nenhuma cena principal.
- **FR-004**: `emitir_snapshot` monta `Protocolo.SnapshotMeta` com os cinco campos (`protocol_version`, `epoch`, `match_id`, `tick`, `content_hash`) mais `jogadores`; `avancar_com_snapshots` usa `SNAPSHOT_HZ=20` e `_snapshot_acumulador` para produzir 20 snapshots em 1 s de simulação a 30 Hz.
- **FR-005**: `ValidacaoComandos.validar(mundo, peer_id, cmd)` com `ACOES_PERMITIDAS`, `LIMITE_MOVIMENTO_M=1.0` e os cinco códigos `PEER_NAO_AUTORIZADO`, `SEQ_OU_REQUEST_REPLAY`, `ACAO_DESCONHECIDA`, `MOVIMENTO_FORA_DO_LIMITE`, `OK`.
- **FR-006**: `ServidorAutoritativo.handle_command` é o ponto único de aplicação: exige `peers_aceitos`, normaliza `COMANDO`, valida, atualiza `seq`/`request_id` no servidor e agenda em `mundo`; `vida` e posição forjadas não entram no estado.
- **FR-007**: `Mundo.hash_estado()` retorna SHA-256 determinístico de `jogadores` + `tick`, usado como golden de paridade entre dois mundos alimentados pelas mesmas intenções.
- **FR-008**: Três testes GDScript (`test_servidor.gd`, `test_loopback.gd`, `test_validacao.gd`) com `extends SceneTree`, saídas `PASS ...`, `OK <suíte>` e `quit(0)`/`quit(1)`; nenhuma linha `FAIL`.
- **FR-009**: Registrar as saídas executadas (com data/hora e marcação **[NV]** para o não observado) em R4; versão, changelog com FR/T e rollback em R7. Nenhum artefato das portarias R4/R7 é escrito nesta especificação.

## Success Criteria

- **SC-001**: `test_servidor.gd` via `--headless --script` → 6 linhas `PASS`, `OK test_servidor`, exit 0.
- **SC-002**: `test_loopback.gd` → 8 linhas `PASS` (2 clientes + 1 contagem + 5 campos), `OK test_loopback`, 20 snapshots em 1 s.
- **SC-003**: `test_validacao.gd` → 9 linhas `PASS`, `OK test_validacao`, exit 0, 0 `FAIL`.
- **SC-004**: `timeout 3s <G> --headless --path . --script res://nucleo/main_server.gd -- --porta=7777` → `SERVIDOR_PRONTO porta=7777` e status 124.
- **SC-005**: 5 gates determinísticos do alvo PASS ao final (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`).

## Out of scope
Desafio da Ilha Meio, `Makefile`, `manifesto.json`, `evidence.csv`, deploy, remote, CI remoto, factory e alteração de baselines. Nenhuma alegação de cobertura integral da árvore canônica é feita por este ciclo; o que não foi executado permanece **[NV]**.
