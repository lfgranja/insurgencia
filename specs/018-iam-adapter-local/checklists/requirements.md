# Checklist de requisitos — 018-iam-adapter-local

**Data:** 2026-11-20T11:17Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação, execução de Godot, gates ou `make check`.

- [x] A decisão é a registrada em ADR-003 (`docs/adr/003-iam-adapter.md`, Accepted 2026-11-20) e no contrato de `docs/architecture/iam-interface.md`: `IAMInterface` é a porta, `IAMAdapterLocal` é o adaptador previsto, sem servidor central (FR-001, FR-002, SC-005)
- [x] Nenhuma abstração nova é introduzida: o contrato é exatamente `IAMInterface` de `docs/architecture/iam-interface.md`; não há segunda interface, fachada, roteador ou trait alternativo (FR-001)
- [x] A porta fixa `class_name IAMInterface extends RefCounted` com os quatro métodos literais: `autenticar(credenciais: Dictionary) -> Dictionary`, `identidade(sessao: String) -> Dictionary`, `validar_permissao(identidade: Dictionary, acao: String, recurso: String) -> bool`, `revogar(sessao: String) -> bool` (FR-001, SC-001: 4/4 métodos)
- [x] Nomes de parâmetros e tipos de retorno são parte do contrato e não serão renomeados pelo adaptador; nenhum método recebe ou devolve segredo, token de provedor, endpoint ou SDK concreto (FR-001, FR-002)
- [x] Os 9 códigos legíveis imutáveis estão enumerados: `AUTENTICADO`, `CREDENCIAIS_INVALIDAS`, `CREDENCIAIS_AUSENTES`, `SESSAO_VALIDA`, `SESSAO_INVALIDA`, `PERMISSAO_CONCEDIDA`, `PERMISSAO_NEGADA`, `REVOGADA`, `NAO_REVOGADA` (SC-001: 9/9)
- [x] `autenticar` devolve `sucesso`, `codigo` e `mensagem`; `AUTENTICADO` inclui `sessao`/`identidade`, e `CREDENCIAIS_AUSENTES`/`CREDENCIAIS_INVALIDAS` não emitem sessão. O domínio compara `codigo`, nunca texto de mensagem (FR-003)
- [x] `identidade` devolve `SESSAO_VALIDA` com identidade para sessão ativa e `SESSAO_INVALIDA` para sessão desconhecida, expirada ou revogada (FR-004)
- [x] `validar_permissao` usa regra local sobre `(identidade, acao, recurso)` e retorna `bool`; `PERMISSAO_NEGADA`/`false` é caminho normal de recusa explícita, não erro (FR-005)
- [x] `revogar` usa estado local: `REVOGADA`/`true` somente se a sessão existia, `NAO_REVOGADA`/`false` para sessão inexistente ou já revogada, sem efeito colateral (FR-006)
- [x] Servidor central está explicitamente negado: sem serviço de identidade remoto, socket, Zitadel, OIDC, OAuth, endpoint ou autoridade externa no caminho crítico (FR-002, SC-005; `docs/architecture/iam-interface.md:10`, `:89`)
- [x] Credenciais estão explicitamente negadas: nenhuma credencial real, segredo, token ou fixture versionada; nada é persistido em arquivo, log ou mensagem de erro (FR-002, SC-005; regra 3 em `docs/architecture/iam-interface.md:84`)
- [x] Login não é obrigatório: o modo solo/single verificado não exige conta corporativa, senha real, pairing, lobby, matchmaking, partidas 5v5, NAT ou federação — nenhum desses itens é entregue (FR-002, SC-005)
- [x] Identidade do adaptador local é determinística e não depende de segredo, aleatoriedade, relógio, serviço externo ou estado persistido (FR-002)
- [x] A recusa é parte do critério de aceite: testes cobrem explicitamente ao menos 1 recusa de permissão (`PERMISSAO_NEGADA`/`false`) e 1 sessão inválida (`SESSAO_INVALIDA`), além dos caminhos de sucesso (FR-007, SC-002)
- [x] Falha de autenticação não distingue "usuário inexistente" de "valor incorreto" na mensagem, evitando vazamento; nenhuma sessão é emitida em falha (FR-003)
- [x] `tests/test_iam_adapter_local.gd` respeita TAB, tipos explícitos, `HashingContext` (não `sha256_string`), `Time.get_ticks_msec` (não `OS.get_ticks_msec`) e variável local sem colisão com `i` de laço (FR-007)
- [x] FR-001..FR-008 e SC-001..SC-005 são binários e rastreiam para os três paths literais (`nucleo/iam/iam_interface.gd`, `nucleo/iam/iam_adapter_local.gd`, `tests/test_iam_adapter_local.gd`) ou para comando verificável
- [x] Critérios de verificação são mensuráveis: 4/4 métodos e 9/9 códigos (SC-001); testes 100% PASS/OK, exit 0, 0 FAIL, 1 recusa e 1 sessão inválida (SC-002); `make check GODOT=$G` exit 0, 5 OK + `COMPAT_OK`, 0 FAIL (SC-003); 5/5 gates PASS (SC-004); 0 segredo/credencial/servidor/federação/5v5/login obrigatório, remoto **[NV]**, R4 antes de R7 (SC-005)
- [x] Plano fixa `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`, a invocação `--headless --path . --script` do teste, `make check GODOT=$G` e os 5 gates determinísticos (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) (FR-008)
- [x] Tasks T001..T010 cobrem implementação, verificação e convergência READ-ONLY, com marcas `[X]` entendidas como frente protocolada, não como execução
- [x] R4 (validate com gate, saída e data/hora) precede R7 (release com SemVer, changelog FR/T/bug e rollback); nenhum artefato das portarias é escrito nesta etapa (FR-008, SC-005)
- [x] Fora de escopo confirmado: gates, baselines, `evidence.csv`, `Makefile`, artefatos R4/R7, código de provedor remoto e qualquer alegação de execução observada
- [x] Saídas não observadas permanecem **[NV]**; registro com data/hora pertence a R4, e versão, changelog com FR/T e rollback a R7
- [x] Nenhum artefato R4/R7, código, gate, servidor, credencial ou comando executado nesta etapa de escrita; nenhum percentual de conclusão afirmado
