# Implementation Plan — Rede E2E: conexão UDP real entre dois clientes na porta 7780

**Data:** 2026-11-20T10:00Z · **Fonte:** GDD `:1171`, `:1514`, `:1515`, `:1582`, `:1588`; R1 `go` em `.specify/assessments/rede-e2e/decision.md`.

## Arquivos (exatamente os do plano)
Implementação/adaptação: `nucleo/servidor.gd` (ligação `peer_packet` → `handle_join`/`handle_command` e respostas `join_accept`/`join_reject`), `tests/cliente_rede_e2e.gd` (cliente headless ENet), `ferramentas/rede/orquestrador_rede_e2e.py` (2 processos, porta 7780), `Makefile` (alvo `test-rede-e2e`). Leitura sem alteração: `nucleo/main_server.gd`, `nucleo/contratos/protocolo.gd`, `nucleo/sim/validacao.gd`, `nucleo/mundo.gd`, `tests/test_loopback.gd`, `tests/test_loopback_ilha.gd`, `tests/test_servidor.gd`, `scripts/ci/*.py`, `docs/GDD - Insurgência.md`. Nenhum outro path é tocado; nenhuma cena, gate, baseline, `evidence.csv` ou artefato R4/R7 é escrito nesta etapa de escrita.

## Steps
1. Manter `nucleo/servidor.gd` integralmente: `PORTA_PADRAO=7777`, `MAX_CLIENTES=4`, `abrir()` com `ENetMultiplayerPeer.create_server()` e `Engine.get_main_loop().get_multiplayer()`, `SERVIDOR_PRONTO`, `peers_aceitos`, `aceitar_saida`, `emitir_snapshot()` e `avancar_com_snapshots()`; o orquestrador usa `--porta=7780` (FR-001).
2. Ligar o sinal `peer_packet` da API de multiplayer a `handle_join`/`handle_command`, decodificando o envelope JSON próprio e usando o `peer_id` do transporte como identidade; nenhuma validação é duplicada no handler e os métodos existentes permanecem a única autoridade. O fio de alto nível é implementation detail sobre ENet/UDP (FR-001, FR-003).
3. Enviar `join_accept` (`aceito=true`, `codigo="OK"`, `peer_id`) ao peer de origem quando `handle_join` aceita, registrando `peers_aceitos` e `mundo.adicionar_jogador`; enviar `join_reject` com `REJEICAO_VERSAO`/`REJEICAO_MODULO`/`REJEICAO_HASH` quando a validação falha, sem adicionar o jogador (FR-002, SC-002).
4. Manter `Protocolo.normalizar_pacote`, `PROTOCOL_VERSION=1`, `MODULO_ID` e `content_hash` de 64 hex; o envelope carrega `tipo` (JOIN/COMANDO) e os campos canônicos, e nenhum campo de posição, vida, ilha ou vitória pronta é aceito como verdade (FR-003).
5. Preservar `ValidacaoComandos.validar` e `handle_command` como único caminho de COMANDO recebido pela rede: peer aceito, recusas `PEER_NAO_AUTORIZADO`/`SEQ_OU_REQUEST_REPLAY`/`ACAO_DESCONHECIDA`/`MOVIMENTO_FORA_DO_LIMITE`, atualização de `seq`/`request_id` no servidor e agendamento no `Mundo`; captura de ilha sem condição não altera a ilha (FR-005, SC-003).
6. Escrever `tests/cliente_rede_e2e.gd` (`extends SceneTree`, `create_client("127.0.0.1", 7780)`), que envia JOIN e COMANDO pela rede, nunca chama os handlers diretamente, imprime `CLIENTE_PRONTO`, `JOIN_ACEITO`/`JOIN_RECUSADO`, `COMANDO_ACEITO`/`COMANDO_RECUSADO`, `OK cliente_rede` e termina com `quit(0)`/`quit(1)`; incluir o modo de JOIN divergente para a recusa (FR-002, FR-004, SC-001..SC-003).
7. Escrever `ferramentas/rede/orquestrador_rede_e2e.py`: subir servidor + exatamente 2 clientes, esperar `SERVIDOR_PRONTO porta=7780`, exigir `JOIN_ACEITO` e `OK cliente_rede` para ambos, timeout determinístico, limpeza dos processos e exit 0 com zero `FAIL` (FR-006, SC-001, SC-004).
8. Acrescentar ao `Makefile` apenas o alvo `test-rede-e2e`, preservando `check`, `check-compat` e os alvos `test-*`. A verificação usa o binário fixado e não altera gates ou baselines (FR-006, FR-007).
9. Verificar `make test-rede-e2e GODOT=$G` (exit 0, marcos do orquestrador, zero `FAIL`, sem processos órfãos) e `make check GODOT=$G` (exit 0, 5 gates determinísticos PASS). Saídas coladas com data/hora somente quando observadas, na portaria própria; R4/R7 não são escritos aqui (FR-007, SC-004, SC-005).
10. Convergência READ-ONLY: revisar spec/plano/tasks contra GDD `:1171`, `:1514`, `:1515`, `:1582`, `:1588` e contra as restrições de R1, corrigir divergências na fonte e reexecutar os 5 gates. Nenhuma alegação de NAT, lobby, produção, escala 5v5 ou de execução não observada (SC-005).

## Comandos de verificação
```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
make test-rede-e2e GODOT=$G
make check GODOT=$G
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```
Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora na portaria de validação (R4). Versão, changelog com FR/T e rollback pertencem a R7. Nenhum artefato das portarias é escrito aqui.

## Riscos e mitigação
- Handlers desligados da rede → `peer_packet` é ligado a `handle_join`/`handle_command`; estes continuam decidindo, e nenhuma regra é reimplementada no callback.
- Contrato trocado pelo fio de alto nível → envelope JSON próprio versionado sobre ENet/UDP; `send_bytes`/`peer_packet` são transporte, não verdade.
- Cliente forjado → `Protocolo.normalizar_pacote` e `ValidacaoComandos.validar` no servidor; posição, vida, ilha e vitória nunca são aceitas prontas.
- Falsa prova de rede → cliente headless é um processo separado e não chama os handlers diretamente; o orquestrador exige `SERVIDOR_PRONTO`, dois `OK cliente_rede` e zero `FAIL`.
- Porta errada ou ocupada → 7780 é exclusiva deste cenário; `create_server` com erro não imprime `SERVIDOR_PRONTO` e o orquestrador falha por timeout em vez de reportar sucesso.
- Processos órfãos → timeout determinístico e limpeza no `finally` do orquestrador, com verificação final da porta.
- Regressão das suítes existentes → `make check GODOT=$G` precisa terminar com exit 0; nenhum outro alvo, gate ou baseline é alterado.
- Escopo indevido → NAT, lobby, partidas 5v5 e escala de produção ficam explicitamente fora; nenhuma alegação não observada é escrita.
- Árvore suja com trabalho alheio → staging explícito dos quatro paths na fase de commit, nunca `git add -A`; nenhum branch, commit ou push nesta etapa.
- Convergência que mascara divergência → correção sempre na fonte (spec/plano/tasks), nunca no artefato derivado.
