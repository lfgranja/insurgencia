# Checklist de requisitos — 017-rede-e2e

**Data:** 2026-11-20T10:00Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação nem execução.

- [x] Spec identifica FR-001..FR-007 e SC-001..SC-005 com critérios binários (dois clientes em processos distintos recebem `join_accept`; JOIN divergente recebe `join_reject`; COMANDO pela rede é validado e o forjado é recusado; orquestrador exit 0 com `SERVIDOR_PRONTO porta=7780` e dois `OK cliente_rede`; 5 gates PASS e `make check GODOT=$G` exit 0)
- [x] Cada FR rastreia para um caminho literal (`nucleo/servidor.gd`, `tests/cliente_rede_e2e.gd`, `ferramentas/rede/orquestrador_rede_e2e.py`, `Makefile`) ou comando `--headless` verificável
- [x] O problema cita o GDD `:1171` (ENet/UDP, nunca confiar no cliente), `:1514`/`:1515` (2 clientes em IP:porta; integração de rede pendente) e `:1582`/`:1588` (CE-2 e 1514 **[NV]**)
- [x] R1 `go` materializado em `.specify/assessments/rede-e2e/decision.md` com data, branch/HEAD (`feature/017-rede-e2e`, `a841ac3`), binário fixado e restrições; impacto 5 e evidência 5 justificados em `problem.md`, opções em `concept.md`
- [x] O envelope JSON próprio é o contrato versionado sobre ENet/UDP; `send_bytes`/`peer_packet` e a API de alto nível são implementation detail e não fonte da verdade
- [x] `peer_packet` é ligado a `handle_join`/`handle_command` sem duplicar validação; `abrir()`, `SERVIDOR_PRONTO`, `peers_aceitos`, `emitir_snapshot` e `avancar_com_snapshots` permanecem preservados
- [x] `join_accept`/`join_reject` voltam ao `peer_id` de origem com `aceito`, `codigo` e `peer_id`; recusas usam `REJEICAO_VERSAO`, `REJEICAO_MODULO`, `REJEICAO_HASH` e não habilitam COMANDO (`PEER_NAO_AUTORIZADO`)
- [x] Nenhuma posição, vida, ilha ou vitória pronta é aceita do cliente; `ValidacaoComandos.validar` permanece o caminho único de COMANDO e a captura de ilha sem condição não altera o estado
- [x] Cliente headless `extends SceneTree` com `create_client("127.0.0.1", 7780)` envia intenções pela rede, não chama os handlers diretamente e imprime marcos observáveis com `quit(0)`/`quit(1)`
- [x] Orquestrador usa 1 servidor + 2 clientes na porta 7780, timeout determinístico, limpeza dos processos e exit 0 condicionado a `SERVIDOR_PRONTO`, dois `OK cliente_rede` e zero `FAIL`
- [x] Portas 7777/7778/7779 permanecem com seus usos atuais; nenhuma alegação de NAT, *port forwarding*, lobby, matchmaking, serviço intermediário, partidas 5v5, escala de produção, latência/perda de WAN, deploy ou CI remoto
- [x] Escopo explícito de fora: alterações de gates, baselines, `evidence.csv`, outros alvos do `Makefile` e portarias R4/R7; nenhum artefato R4/R7 é escrito nesta etapa
- [x] Saídas não observadas permanecem **[NV]**; registro com data/hora pertence a R4, e versão/changelog FR/T/rollback a R7
- [x] Plano fixa `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`, os comandos `--headless`/`make test-rede-e2e`/`make check`/5 gates; tasks T001..T010 com marcos e convergência READ-ONLY
- [x] Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos), de conexão efetivamente executada, de NAT, lobby, produção ou escala 5v5
- [x] Nenhum artefato R4/R7, código, gate, teste ou comando executado nesta etapa de escrita; convenção 80/20 respeitada, sem percentual de conclusão afirmado e com convergência corrigindo a fonte, nunca o artefato derivado
