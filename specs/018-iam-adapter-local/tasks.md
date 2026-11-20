# Tasks — 018-iam-adapter-local

**Data:** 2026-11-20T11:17Z

Marcações `[X]` registram a frente de trabalho protocolada nesta etapa de escrita; execução de Godot/gates/`make check` e portarias R4/R7 permanecem **[NV]** e não são afirmadas aqui. Nenhum item entrega login obrigatório, servidor central, provedor remoto, federação ou partida 5v5.

## Phase 1: Implementação

- [X] **T001** Criar `nucleo/iam/iam_interface.gd` com `class_name IAMInterface` e `extends RefCounted`, contendo apenas os quatro métodos do contrato com assinaturas e nomes de parâmetros literais: `autenticar(credenciais: Dictionary) -> Dictionary`, `identidade(sessao: String) -> Dictionary`, `validar_permissao(identidade: Dictionary, acao: String, recurso: String) -> bool`, `revogar(sessao: String) -> bool`. Nenhuma abstração nova, renomeação, SDK, endpoint ou token; a porta é a de `docs/architecture/iam-interface.md`/ADR-003 (FR-001, SC-001)
- [X] **T002** Criar `nucleo/iam/iam_adapter_local.gd` com `class_name IAMAdapterLocal extends IAMInterface`, inteiramente em processo: identidade determinística sem segredo, sem aleatoriedade, sem credencial persistida e sem relógio na derivação; nenhum servidor, socket, chamada remota ou login obrigatório (FR-002, SC-005)
- [X] **T003** Implementar `autenticar` devolvendo `Dictionary` com `sucesso`, `codigo` e `mensagem`: `AUTENTICADO` com `sessao` e `identidade`; `CREDENCIAIS_AUSENTES` para dicionário vazio/incompleto e `CREDENCIAIS_INVALIDAS` para recusa, ambos sem sessão e sem distinguir usuário inexistente de valor incorreto; ramificação apenas por `codigo`, nunca por `mensagem` (FR-003, SC-001)
- [X] **T004** Implementar `identidade` com estado local: `SESSAO_VALIDA` e `identidade` para sessão ativa; `SESSAO_INVALIDA` e nenhuma identidade para sessão desconhecida, expirada ou revogada (FR-004, SC-001)
- [X] **T005** Implementar `validar_permissao -> bool` por tabela local fixa sobre `(identidade, acao, recurso)`: conceder somente com regra correspondente (`PERMISSAO_CONCEDIDA`/`true`) e negar explicitamente o não coberto (`PERMISSAO_NEGADA`/`false` como caminho normal, não erro) (FR-005, SC-001)
- [X] **T006** Implementar `revogar` com estado local: `REVOGADA`/`true` somente se a sessão existia e estava ativa; `NAO_REVOGADA`/`false` para sessão inexistente ou já revogada, sem efeito colateral; sessão revogada deixa de resolver `identidade` (FR-006, SC-001)
- [X] **T007** Escrever `tests/test_iam_adapter_local.gd` (`extends SceneTree`, `--headless --script`) com casos de sucesso, ao menos 1 recusa de permissão (`PERMISSAO_NEGADA`/`false`) e ao menos 1 sessão inválida (`SESSAO_INVALIDA`), cobrindo os 9 códigos; imprimir marcos legíveis e terminar com `quit(0)`/`quit(1)`. Observar TAB, tipos explícitos, `HashingContext` (não `sha256_string`), `Time.get_ticks_msec` se necessário (não `OS.get_ticks_msec`) e variável local sem colisão com `i` de laço (FR-007, SC-001, SC-002)
- [X] **T008** Fechar o escopo: exatamente os 9 códigos legíveis imutáveis (`AUTENTICADO`, `CREDENCIAIS_INVALIDAS`, `CREDENCIAIS_AUSENTES`, `SESSAO_VALIDA`, `SESSAO_INVALIDA`, `PERMISSAO_CONCEDIDA`, `PERMISSAO_NEGADA`, `REVOGADA`, `NAO_REVOGADA`), sem segredo, credencial, servidor, Zitadel/OIDC, federação, 5v5, lobby, login obrigatório, gate, baseline ou artefato R4/R7 alterado nesta etapa (FR-001, FR-002, FR-007, SC-001, SC-005)

## Phase 2: Verificação

- [X] **T009** Verificar com `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`: `"$G" --headless --path . --script res://tests/test_iam_adapter_local.gd` com testes 100% PASS/OK, exit `0`, 0 `FAIL`, 1 recusa e 1 sessão inválida; `make check GODOT=$G` com exit `0`, 5 OK + `COMPAT_OK` e 0 `FAIL`; e os 5 gates determinísticos PASS (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`). Saída executada entra com data/hora somente na portaria R4 (validate), que precede R7 (release com SemVer, changelog FR/T/bug e rollback); nenhum artefato das portarias é escrito aqui (FR-008, SC-002, SC-003, SC-004)

## Phase 3: Convergence READ-ONLY

- [X] **T010** Convergência READ-ONLY: revisar spec/plano/tasks contra `docs/architecture/iam-interface.md`, ADR-003 e a decisão R1, corrigindo divergências na fonte e reexecutando os 5 gates. Manter 0 segredo, credencial, servidor, federação, 5v5 e login obrigatório; provedor/remoto permanece **[NV]**; R4 precede R7 e nenhuma alegação de comando executado nesta etapa é feita (FR-008, SC-005)
