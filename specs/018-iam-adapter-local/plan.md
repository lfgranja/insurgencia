# Implementation Plan — IAMAdapterLocal: identidade local determinística atrás de `IAMInterface`

**Data:** 2026-11-20T11:17Z · **Fonte:** ADR-003 (`docs/adr/003-iam-adapter.md`, Accepted 2026-11-20) e contrato em `docs/architecture/iam-interface.md` (`:18-40` assinaturas, `:44-70` códigos, `:72-76` adaptador previsto, `:80-85` regras). Decisão da SE: implementar em processo, **sem servidor central**, **sem nova abstração** — a porta é `IAMInterface`, exatamente a de `docs/architecture/iam-interface.md`/ADR-003. Nenhuma credencial é persistida e login não é obrigatório.

## Arquivos (exatamente os do plano)

Implementação: `nucleo/iam/iam_interface.gd` (FR-001), `nucleo/iam/iam_adapter_local.gd` (FR-002..FR-006), `tests/test_iam_adapter_local.gd` (FR-007). Leitura sem alteração: `docs/architecture/iam-interface.md`, `docs/adr/003-iam-adapter.md`, `Makefile`, `scripts/ci/*.py`, `AGENTS.md`. Nenhum outro path é tocado nesta etapa de escrita: nenhum gate, baseline, `evidence.csv`, artefato R4/R7, servidor, serviço, SDK, segredo ou credencial versionada.

## Steps

1. Criar `nucleo/iam/iam_interface.gd` com `class_name IAMInterface` e `extends RefCounted`, contendo apenas os quatro métodos do contrato com assinaturas e nomes de parâmetros literais: `autenticar(credenciais: Dictionary) -> Dictionary`, `identidade(sessao: String) -> Dictionary`, `validar_permissao(identidade: Dictionary, acao: String, recurso: String) -> bool`, `revogar(sessao: String) -> bool`. A porta é a já documentada: não criar outra abstração, não renomear métodos, não referenciar provedor, endpoint, token ou SDK (FR-001, SC-001).
2. Criar `nucleo/iam/iam_adapter_local.gd` com `class_name IAMAdapterLocal extends IAMInterface`, inteiramente em processo. Identidade determinística derivada de dados locais canônicos: sem segredo, sem aleatoriedade, sem relógio na derivação e sem credencial persistida em arquivo, log ou mensagem. Nenhum servidor, socket, chamada remota ou login obrigatório (FR-002, SC-005).
3. Implementar `autenticar` devolvendo `Dictionary` com `sucesso`, `codigo` e `mensagem`. `AUTENTICADO` inclui `sessao` e `identidade`; dicionário vazio/incompleto devolve `CREDENCIAIS_AUSENTES`; dados presentes mas recusados devolvem `CREDENCIAIS_INVALIDAS`, ambos sem emitir sessão e sem distinguir "usuário inexistente" de "valor incorreto" na mensagem. O domínio compara `codigo`, nunca `mensagem` (FR-003, SC-001).
4. Implementar `identidade`: sessão ativa devolve `SESSAO_VALIDA` com `identidade`; sessão desconhecida, expirada ou revogada devolve `SESSAO_INVALIDA` sem identidade (FR-004, SC-001).
5. Implementar `revogar` com estado local: devolve `REVOGADA`/`true` somente se a sessão existia e estava ativa; sessão inexistente ou já revogada devolve `NAO_REVOGADA`/`false`, sem efeito colateral (FR-006, SC-001).
6. Fixar a tabela local de permissões em `validar_permissao`: regras estáticas sobre `(identidade, acao, recurso)`; conceder somente quando a regra corresponde e negar explicitamente qualquer combinação não coberta. Retorno `bool` — `PERMISSAO_CONCEDIDA` equivale a `true` e `PERMISSAO_NEGADA` a `false`, tratada como caminho normal, não como erro (FR-005, SC-001).
7. Escrever `tests/test_iam_adapter_local.gd` (`extends SceneTree`, execução `--headless --script`) cobrindo sucesso, ao menos uma recusa de permissão e ao menos uma sessão inválida: `AUTENTICADO`, `CREDENCIAIS_AUSENTES`, `CREDENCIAIS_INVALIDAS`, `SESSAO_VALIDA`, `SESSAO_INVALIDA`, `PERMISSAO_NEGADA`/`false`, `REVOGADA`, `NAO_REVOGADA`. Observar TAB, tipos explícitos, `HashingContext` (não `sha256_string`), `Time.get_ticks_msec` se tempo for necessário (não `OS.get_ticks_msec`) e variável local sem colisão com `i` de laço. Imprimir marcos legíveis e terminar com `quit(0)`/`quit(1)` (FR-007, SC-002).
8. Fechar o contrato de retorno nos 9 códigos legíveis imutáveis (`AUTENTICADO`, `CREDENCIAIS_INVALIDAS`, `CREDENCIAIS_AUSENTES`, `SESSAO_VALIDA`, `SESSAO_INVALIDA`, `PERMISSAO_CONCEDIDA`, `PERMISSAO_NEGADA`, `REVOGADA`, `NAO_REVOGADA`) e manter o escopo negado: nenhum provedor real, Zitadel, OIDC, federação, partida 5v5, lobby, login obrigatório, gate, baseline ou artefato R4/R7 nesta etapa (FR-001, FR-002, FR-007, SC-001, SC-005).
9. Verificar com o binário fixado: executar o teste em headless (100% PASS/OK, exit 0, 0 FAIL, com 1 recusa e 1 sessão inválida), `make check GODOT=$G` com exit 0, 5 OK + `COMPAT_OK` e 0 FAIL, e os 5 gates determinísticos PASS. Saída efetivamente observada entra com data/hora apenas na portaria R4; versão, changelog com FR/T e rollback pertencem a R7. Nada disso é afirmado como executado nesta etapa de escrita (FR-008, SC-002, SC-003, SC-004).
10. Convergência READ-ONLY: revisar spec/plano/tasks contra `docs/architecture/iam-interface.md`, ADR-003 e a decisão R1, corrigindo divergências sempre na fonte e reexecutando os 5 gates. Nenhuma alegação de servidor central, provedor remoto, credencial real, login obrigatório, federação, 5v5 ou execução não observada; R4 precede R7 e nenhum artefato das portarias é escrito aqui (FR-008, SC-005).

## Comandos de verificação

```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
"$G" --headless --path . --script res://tests/test_iam_adapter_local.gd
make check GODOT=$G
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```

Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora na portaria de validação (R4). A verificação do teste usa a invocação direta de `--headless --path . --script`, sem inventar alvo novo no `Makefile`. Versão, changelog com FR/T e rollback pertencem a R7; R4 precede R7 e nenhum artefato dessas portarias é escrito aqui.

## Riscos e mitigação

- Servidor central reintroduzido → adaptador inteiramente em processo; nenhum socket, endpoint, serviço ou autoridade externa no caminho (SC-005).
- Nova abstração paralela → a porta é `IAMInterface` de `docs/architecture/iam-interface.md`; `IAMAdapterLocal` apenas a implementa (FR-001).
- Segredo ou credencial vazada → nenhuma credencial é persistida, logada ou devolvida; derivação local determinística sem segredo (FR-002).
- Login obrigatório → `autenticar` resolve localmente as credenciais fornecidas; o modo solo não exige conta, senha real nem provedor (SC-005).
- Recusa não exercitada → testes cobrem explicitamente recusa de permissão e sessão inválida, além do sucesso (SC-002).
- Códigos renomeados ou mensagem usada como condição → 9 códigos imutáveis; ramificação apenas por `codigo`/`bool` (SC-001).
- Determinismo quebrado por tempo ou aleatoriedade → nenhuma derivação depende de relógio, aleatório ou estado externo; usar `HashingContext` sobre dados canônicos (FR-002, FR-007).
- Regressão dos gates → `make check GODOT=$G` precisa terminar com exit 0, 5 OK + `COMPAT_OK` e 0 FAIL; nenhum gate ou baseline é alterado (SC-003, SC-004).
- Árvore suja com trabalho alheio → staging explícito dos três paths na fase de commit; nenhum branch, commit ou push nesta etapa.
- Convergência que mascara divergência → correção na fonte (spec/plano/tasks), nunca no artefato derivado; não observado continua **[NV]**.
