# Decision — Rede E2E: conexão UDP real entre dois clientes na porta 7780

**Data:** 2026-11-20T10:00Z · **R1:** `go`

## Veredito
**go** — problema válido (impacto 5, evidência 5); opção escolhida = Opção 1 (`peer_packet` ligado aos handlers existentes, envelope JSON próprio sobre ENet/UDP, cliente headless e orquestrador de 2 processos na porta 7780, com alvo `make test-rede-e2e`).

## Gate de entrada
- Fonte: `docs/GDD - Insurgência.md:1171` (transporte ENet/UDP e aviso "nunca confiar no cliente"), `:1514` (2 clientes em IP:porta na mesma partida; conexão real **[NV]**), `:1515` (servidor aceita só intenções; integração de rede pendente), `:1582`/`:1588` (CE-2 e 1514 com conexão real **[NV]**). A pesquisa oficial `docs/research/2026-11-20-godot-headless-oficial.md` fixa que o fio de alto nível é detalhe de implementação e que o contrato durável são bytes próprios sobre ENet/UDP.
- Gates determinísticos do alvo: 5/5 PASS em 2026-11-20T03:21:31Z (`.specify/memory/STATE.md:5`; `spec_dirs=12`, `tasks=222`); reexecução neste ciclo, execução de Godot, do alvo `test-rede-e2e` e status remoto/CI permanecem **[NV]** até registro datado na portaria própria.
- `.specify/feature.json` → `specs/017-rede-e2e`; branch `feature/017-rede-e2e`, HEAD `a841ac3`.
- Binário fixado: `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`; `--version`, execuções `--headless --script` e saídas do orquestrador são **[NV]** nesta etapa de escrita.

## Restrições de execução
- Preservar integralmente `nucleo/servidor.gd`: `PORTA_PADRAO=7777`, `MAX_CLIENTES=4`, `abrir()` com `ENetMultiplayerPeer` e `Engine.get_main_loop().get_multiplayer()`, `SERVIDOR_PRONTO`, `peers_aceitos`, `handle_join`, `handle_command`, `aceitar_saida`, `emitir_snapshot` e `avancar_com_snapshots`. A ligação via `peer_packet` é aditiva e não cria uma segunda autoridade: JOIN e COMANDO continuam sendo decididos por `handle_join`/`handle_command`, com `Protocolo.normalizar_pacote` e `ValidacaoComandos.validar`.
- O envelope JSON próprio é o contrato versionado (`tipo`, `protocol_version=1`, `module_id`, `content_hash` de 64 hex, `request_id`, `seq`, `acao`, `alvo`, `destino`); RPC/replicação de alto nível é implementation detail e não é fonte da verdade. Nenhuma posição, vida, ilha ou vitória pronta vinda do cliente é aceita.
- Respostas `join_accept`/`join_reject` voltam ao `peer_id` de origem pelo mesmo canal, com `aceito`, `codigo` e `peer_id`; `VERSAO_PROTOCOLO_DIVERGENTE`, `MODULO_AUSENTE` e `HASH_MODULO_DIVERGENTE` permanecem como códigos de recusa. Somente peers aceitos podem enviar COMANDO; fora disso, `PEER_NAO_AUTORIZADO`.
- Cliente headless `extends SceneTree` com `ENetMultiplayerPeer.create_client()` contra `127.0.0.1:7780`, sem chamar `handle_join`/`handle_command` diretamente; marcos `CLIENTE_PRONTO`, `JOIN_ACEITO`/`JOIN_RECUSADO`, `COMANDO_ACEITO`/`COMANDO_RECUSADO`, `OK cliente_rede`, `quit(0)`/`quit(1)`. Orquestrador de exatamente 2 processos de cliente + 1 servidor, timeout determinístico, limpeza ao final e exit 0 condicionado a `SERVIDOR_PRONTO porta=7780`, dois `OK cliente_rede` e zero `FAIL`.
- Alvo `make test-rede-e2e GODOT=$G` sem alterar `check`, os demais alvos, gates, baselines ou `evidence.csv`. Registro de saídas com data/hora pertence a R4; versão, changelog com FR/T e rollback pertencem a R7. Nenhum artefato R4/R7 é escrito nesta etapa.
- Fora de escopo: NAT, *port forwarding*, lobby, matchmaking, serviço intermediário, partidas 5v5, escala de produção, deploy/remote, factory e portarias R4/R7. Nenhuma alegação de NAT, lobby, produção, escala 5v5 ou de execução observada é feita.
- Sem branches, commits, pushes, gates, testes ou código nesta etapa de escrita; implementação e verificação pertencem à fase própria.
- Convergence/analyze é READ-ONLY: divergência detectada é corrigida na fonte (spec/plano/tasks) e revalidada, nunca mascarada no artefato derivado.
