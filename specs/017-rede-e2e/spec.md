# Feature Specification: Rede E2E — conexão UDP real entre dois clientes na porta 7780

**Feature Branch**: `017-rede-e2e` · **Created**: 2026-11-20T10:00Z · **Status**: Draft · **Data:** 2026-11-20T10:00Z

**Input**: GDD `:1171` (ENet/UDP e "nunca confiar no cliente"), `:1514` (2 clientes em IP:porta na mesma partida, conexão real **[NV]**), `:1515` (servidor aceita só intenções; integração de rede pendente), `:1582`/`:1588` (CE-2 e 1514 **[NV]**). R1 `go` em `.specify/assessments/rede-e2e/decision.md`. A pesquisa oficial `docs/research/2026-11-20-godot-headless-oficial.md` fixa que o fio de multiplayer de alto nível é detalhe de implementação: o contrato durável é o envelope JSON próprio sobre ENet/UDP.

## User Scenarios & Testing

### User Story 1 — Dois processos conectam por UDP e recebem join_accept (P1)
Dois clientes headless, em processos distintos, informam `127.0.0.1:7780` na inicialização, abrem um `ENetMultiplayerPeer` cliente e enviam JOIN pela rede. O servidor, já escutando com `abrir()`, liga `peer_packet` a `handle_join`, valida versão, módulo e hash pelo `Protocolo`, registra o peer em `peers_aceitos`/`mundo` e devolve `join_accept` ao `peer_id` de origem. Nenhum cliente chama `handle_join` diretamente e nenhum serviço intermediário participa.

**Why this priority**: é o critério canônico 1514 e o marco CE-2 (GDD `:1582`); sem dois processos efetivamente conectados, o handshake continua sendo apenas unitário.

**Independent Test**: orquestrador sobe o servidor com `--porta=7780`, espera `SERVIDOR_PRONTO porta=7780`, lança 2 clientes e exige `JOIN_ACEITO` para ambos; cada `join_accept` traz `aceito=true`, `codigo="OK"` e o `peer_id` atribuído pelo ENet.

### User Story 2 — JOIN divergente é recusado pela rede (P1)
Um cliente envia pela rede um JOIN com `protocol_version` diferente de 1, `module_id` vazio/divergente ou `content_hash` que não tenha 64 caracteres. O handler de rede decodifica o envelope e entrega o dicionário a `handle_join`, que recusa com `VERSAO_PROTOCOLO_DIVERGENTE`, `MODULO_AUSENTE` ou `HASH_MODULO_DIVERGENTE`; a resposta `join_reject` volta ao peer de origem e o peer não entra em `peers_aceitos` nem em `mundo.jogadores`.

**Why this priority**: materializa o aviso "nunca confiar no cliente" de `:1171` e a recusa de versão/hash já previstos em `:1515`; sem isso, a conexão real seria uma porta aberta a qualquer módulo.

**Independent Test**: com o servidor no ar, um cliente de recusa envia `protocol_version=0` (ou módulo vazio/hash curto) e recebe `join_reject` com o código correspondente; um COMANDO enviado em seguida por esse peer obtém `PEER_NAO_AUTORIZADO`.

### User Story 3 — Comando pela rede é validado no servidor (P1)
Após `join_accept`, o cliente envia COMANDO no envelope próprio. O servidor liga `peer_packet` a `handle_command`, que exige peer aceito, normaliza o comando, chama `ValidacaoComandos.validar`, atualiza `seq`/`request_id` no servidor e agenda a intenção no `Mundo`. Um cliente modificado que pede "capturar ilha" sem condição prévia, excede o limite de movimento, repete `seq`/`request_id` ou usa ação desconhecida recebe recusa pela rede e o estado permanece inalterado (ilha neutra).

**Why this priority**: é a continuação de `:1515` e o ponto em que a arquitetura deixa de depender de chamadas em processo único; sem a validação sobre a rede, ACE-2 estaria incompleto mesmo com sockets funcionando.

**Independent Test**: cliente envia COMANDO válido e recebe `COMANDO_ACEITO`; cliente envia COMANDO forjado e recebe `COMANDO_RECUSADO` com o código da validação; em nenhum caso o cliente informa posição, vida ou ilha como verdade.

### User Story 4 — Orquestrador, alvo e evidência auditável (P2)
Um orquestrador de 2 processos de cliente + 1 servidor usa a porta UDP 7780, coleta as saídas, impõe timeout determinístico, encerra os processos ao final e só devolve exit 0 quando há `SERVIDOR_PRONTO porta=7780`, dois `OK cliente_rede` e zero `FAIL`. O alvo `make test-rede-e2e GODOT=$G` encapsula essa execução, e a saída observada é registrada com data/hora na portaria de validação R4; versão, changelog FR/T e rollback pertencem a R7.

**Why this priority**: sem um comando único e uma evidência datada, o critério 1514 não pode ser reexecutado nem auditado, e o campo UDP real de `evidence.csv` continuaria `NAO_VERIFICADO`.

**Independent Test**: `make test-rede-e2e GODOT=$G` termina com exit 0, imprime os marcos do orquestrador e não deixa processos órfãos na porta 7780.

### Edge Cases
- Cliente conecta mas não envia JOIN dentro do timeout: não entra em `peers_aceitos`, nenhum comando é aceito e o orquestrador encerra com `FAIL`/exit não zero, limpando os processos.
- JOIN com `protocol_version=0`, `module_id` vazio/divergente ou `content_hash` com 61/65 caracteres: `join_reject` com o código de rejeição correspondente; nenhum jogador é adicionado ao `Mundo`.
- JOIN repetido pelo mesmo peer ou COMANDO antes do JOIN: o `peer_id` do transporte permanece a identidade; COMANDO sem peer aceito é recusado com `PEER_NAO_AUTORIZADO`.
- Envelope com JSON inválido, `tipo` desconhecido ou campos fora do `Protocolo`: o servidor descarta o pacote sem alterar estado e sem derrubar o processo; nenhum campo extra se torna verdade.
- COMANDO forjado (captura de ilha sem pré-condição, movimento acima de 1,0 m, `seq`/`request_id` repetidos, ação desconhecida): recusa pelo servidor, ilha permanece neutra e `hash_estado()` não muda por causa do comando inválido.
- `create_server(7780, ...)` falha (porta ocupada): `abrir()` faz `push_error` e não imprime `SERVIDOR_PRONTO`; o orquestrador falha por timeout em vez de reportar sucesso.
- Portas 7777/7778/7779 continuam reservadas ao servidor padrão e aos loopbacks; o cenário E2E não as reutiliza. Nenhum comportamento de perda de pacote, latência WAN, NAT ou reconexão é afirmado.

## Requirements

### Functional Requirements
- **FR-001**: `nucleo/servidor.gd` preserva `PORTA_PADRAO=7777`, `MAX_CLIENTES=4`, `abrir()` (`ENetMultiplayerPeer.create_server()` + `Engine.get_main_loop().get_multiplayer()`), `SERVIDOR_PRONTO`, `peers_aceitos`, `aceitar_saida`, `emitir_snapshot()` e `avancar_com_snapshots()`. Acrescenta handlers de rede que ligam o sinal `peer_packet` da API de multiplayer a `handle_join(peer_id, dados)` e `handle_command(peer_id, dados)`, usando o `peer_id` do transporte e o envelope JSON próprio. Nenhuma regra de validação é duplicada no handler: a decisão permanece nos dois métodos existentes, com `Protocolo.normalizar_pacote` e `ValidacaoComandos.validar`.
- **FR-002**: O servidor responde no mesmo canal, ao `peer_id` de origem, com `join_accept` (`aceito=true`, `codigo="OK"`, `peer_id`) quando `handle_join` aceita, registrando `peers_aceitos[peer_id]` e chamando `mundo.adicionar_jogador(peer_id)`; e com `join_reject` (`aceito=false`, `codigo`, `peer_id`) quando há divergência de `protocol_version`, `module_id` ausente/divergente ou `content_hash` com tamanho diferente de 64, usando `REJEICAO_VERSAO`, `REJEICAO_MODULO` e `REJEICAO_HASH`. Peer recusado não entra em `peers_aceitos` nem em `mundo.jogadores` e não pode enviar COMANDO.
- **FR-003**: O envelope JSON próprio carrega `tipo` (`JOIN` ou `COMANDO`) e os campos canônicos já definidos em `nucleo/contratos/protocolo.gd` — `PROTOCOL_VERSION=1`, `MODULO_ID`, `content_hash` de 64 hex para JOIN; `request_id`, `seq`, `acao`, `alvo`, `destino` para COMANDO — passando por `Protocolo.normalizar_pacote`. RPC/replicação de multiplayer de alto nível é implementation detail sobre ENet/UDP e não é a fonte da verdade; nenhuma posição, vida, estado de ilha ou vitória pronta é aceita do cliente.
- **FR-004**: `tests/cliente_rede_e2e.gd` (`extends SceneTree`, `--headless --script`) cria `ENetMultiplayerPeer` com `create_client("127.0.0.1", 7780)`, atribui o peer à API de multiplayer, envia JOIN e COMANDO **pela rede** (sem chamar `handle_join`/`handle_command` diretamente), aguarda `join_accept`/`join_reject`, imprime `CLIENTE_PRONTO`, `JOIN_ACEITO`/`JOIN_RECUSADO`, `COMANDO_ACEITO`/`COMANDO_RECUSADO` e `OK cliente_rede`, e termina com `quit(0)`/`quit(1)`. Um segundo modo de recusa envia JOIN divergente para exercitar FR-002 sem alterar o estado do servidor.
- **FR-005**: `nucleo/sim/validacao.gd` e `ServidorAutoritativo.handle_command` permanecem como o único caminho de aplicação de COMANDO recebido pela rede: exigem peer aceito, normalizam o comando, recusam `PEER_NAO_AUTORIZADO`, `SEQ_OU_REQUEST_REPLAY`, `ACAO_DESCONHECIDA` e `MOVIMENTO_FORA_DO_LIMITE`, atualizam `seq`/`request_id` no servidor e agendam a intenção em `mundo`. Captura de ilha sem condição prévia é recusada e a ilha permanece neutra; o cliente nunca envia verdade de estado.
- **FR-006**: `ferramentas/rede/orquestrador_rede_e2e.py` sobe 1 servidor (`nucleo/main_server.gd -- --porta=7780`) e exatamente 2 processos de cliente headless com o binário fixado, espera `SERVIDOR_PRONTO porta=7780`, exige `JOIN_ACEITO` e `OK cliente_rede` para ambos, usa timeout determinístico, encerra todos os processos ao final e devolve exit 0 somente com zero linhas `FAIL`. O `Makefile` ganha o alvo `test-rede-e2e` (via `$(GODOT) --headless --path . --script` para os scripts Godot e a chamada do orquestrador), preservando `check` e os alvos `test-*` existentes.
- **FR-007**: O alvo `make test-rede-e2e GODOT=$G` com `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64` é a porta de verificação desta frente: exigir `SERVIDOR_PRONTO porta=7780`, dois `OK cliente_rede`, exit 0 e zero `FAIL`, além de `make check GODOT=$G` com exit 0 e os 5 gates determinísticos PASS (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`). Saídas efetivamente executadas, com data/hora e **[NV]** para o não observado, pertencem à portaria de validação R4; versão, changelog com FR/T e rollback pertencem a R7. Nenhum artefato R4/R7 é escrito nesta especificação, e nenhuma alegação de NAT, lobby, produção, escala 5v5 ou de execução observada é feita.

### Key Entities
- **Envelope JSON próprio**: mensagem versionada sobre ENet/UDP com `tipo`, campos normalizados por `Protocolo` e `peer_id` do transporte como identidade; é o contrato durável, não o fio de alto nível do Godot.
- **`join_accept` / `join_reject`**: respostas do servidor ao peer de origem, contendo `aceito`, `codigo` e `peer_id`; só `join_accept` habilita COMANDO.
- **Cliente headless ENet**: processo `SceneTree` que envia intenções (JOIN/COMANDO) pela rede e imprime marcos verificáveis, sem decidir estado.
- **Orquestrador**: processo coordenador que sobe servidor + 2 clientes em `127.0.0.1:7780`, aplica timeout, coleta marcos, limpa processos e traduz o resultado em exit status para `make test-rede-e2e`.

## Success Criteria

- **SC-001**: Com `make test-rede-e2e GODOT=$G` e a porta UDP 7780 livre, dois clientes em processos distintos conectam por ENet/UDP e recebem `join_accept` (`aceito=true`, `codigo="OK"`, `peer_id` preenchido); o orquestrador registra `JOIN_ACEITO` para os dois e nenhum cliente chamou `handle_join` diretamente.
- **SC-002**: Um JOIN divergente enviado pela rede (versão diferente de 1, `module_id` vazio/divergente ou `content_hash` com 61/65 caracteres) recebe `join_reject` com `VERSAO_PROTOCOLO_DIVERGENTE`, `MODULO_AUSENTE` ou `HASH_MODULO_DIVERGENTE`, não entra em `peers_aceitos`/`mundo.jogadores` e um COMANDO posterior recebe `PEER_NAO_AUTORIZADO`.
- **SC-003**: Um COMANDO válido enviado pela rede é validado por `handle_command` + `ValidacaoComandos.validar`, atualiza `seq`/`request_id` no servidor e produz `COMANDO_ACEITO`; um COMANDO forjado (captura de ilha sem condição, movimento > 1,0 m, replay ou ação desconhecida) produz `COMANDO_RECUSADO` e não altera a ilha nem o estado do `Mundo`.
- **SC-004**: `make test-rede-e2e GODOT=$G` imprime `SERVIDOR_PRONTO porta=7780` e dois `OK cliente_rede`, termina com exit `0`, zero linhas `FAIL` e nenhum processo órfão na porta 7780.
- **SC-005**: Ao final da implementação, `make check GODOT=$G` termina com exit `0` e os 5 gates determinísticos (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) estão PASS, sem regressão das suítes existentes.

## Assumptions
- O cenário é estritamente loopback local (`127.0.0.1:7780`) com o binário `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64` em modo `--headless`; o servidor é o ID 1 e cada `peer_id` é atribuído pelo ENet, nunca pelo envelope.
- A porta 7780 é dedicada a este cenário; 7777 (`PORTA_PADRAO`), 7778 e 7779 permanecem com o servidor padrão e os testes de loopback. `nucleo/main_server.gd` já lê `--porta=` de `OS.get_cmdline_user_args()`.
- `nucleo/servidor.gd`, `nucleo/contratos/protocolo.gd`, `nucleo/sim/validacao.gd` e `nucleo/main_server.gd` fornecem os símbolos e padrões existentes; a mudança no servidor é aditiva e nenhuma validação é reimplementada no handler de rede.
- O contrato versionado é o envelope JSON próprio sobre ENet/UDP; a API de multiplayer de alto nível (inclusive `send_bytes`/`peer_packet`) é o transporte e pode mudar sem aviso, logo nenhuma regra de negócio depende dela.
- A evidência com data/hora só entra na portaria de validação R4 quando observada; versão, changelog FR/T e rollback pertencem a R7. Nenhum artefato das portarias é escrito nesta etapa de escrita; execuções de Godot, gates, `make check` e `make test-rede-e2e` permanecem **[NV]** até registro datado próprio.

## Out of scope
NAT, *port forwarding*, descoberta de servidor, lobby, matchmaking, serviço intermediário, partidas 5v5, escala de produção, latência/perda de WAN, deploy, remote, CI remoto, factory, alteração de gates, baselines, `evidence.csv`, de outros alvos do `Makefile` e das portarias R4/R7. Nenhuma alegação de NAT, lobby, produção, escala 5v5, de conexão efetivamente executada ou de cobertura integral da árvore canônica (39/39 caminhos) é feita por este ciclo; o que não foi executado permanece **[NV]**.
