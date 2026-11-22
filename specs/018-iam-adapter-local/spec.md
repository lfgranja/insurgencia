# Feature Specification: IAMAdapterLocal em processo

**Feature Branch**: `feature/018-iam-adapter-local`

**Created**: 2026-11-20

**Status**: Draft

**Input**: decisão R1 `go` de `.specify/assessments/iam-adapter-local/decision.md` (Opção A), aceite da SE em 2026-11-20T10:58Z (`.specify/memory/STATE.md:5,7,9`), ADR-003 (`docs/adr/003-iam-adapter.md`) e contrato em `docs/architecture/iam-interface.md`.

## User Scenarios & Testing *(mandatory)*

As histórias abaixo são priorizadas por valor e cada uma é testável de forma independente. Nenhuma pressupõe login, servidor central, federação ou partida 5v5.

### User Story 1 - Autenticar e resolver identidade localmente (Priority: P1)

Como módulo de domínio do modo solo, preciso autenticar um pedido e resolver a identidade de sua sessão sem consultar serviço externo, para que o jogo funcione de forma determinística e sem segredo.

**Why this priority**: é o caminho fundamental que materializa a porta aprovada em ADR-003; sem ele, nenhuma das demais histórias existe.

**Independent Test**: executar `autenticar` com credenciais ausentes, inválidas e aceitas e, em seguida, `identidade` para a sessão emitida e para uma sessão desconhecida, verificando os códigos exatos.

**Acceptance Scenarios**:

1. **Given** um `Dictionary` de credenciais ausentes ou incompleto, **When** `autenticar` é chamado, **Then** retorna `CREDENCIAIS_AUSENTES` sem emitir `sessao`.
2. **Given** credenciais recusadas pela regra local, **When** `autenticar` é chamado, **Then** retorna `CREDENCIAIS_INVALIDAS`, sem sessão e sem distinção de causa na mensagem.
3. **Given** credenciais aceitas, **When** `autenticar` é chamado, **Then** retorna `AUTENTICADO` com `sucesso=true`, `sessao` e `identidade` presentes.
4. **Given** a sessão emitida no cenário 3, **When** `identidade` é chamado, **Then** retorna `SESSAO_VALIDA` com `identidade` presente; para sessão desconhecida, retorna `SESSAO_INVALIDA` sem `identidade`.

---

### User Story 2 - Autorizar e recusar ação localmente (Priority: P2)

Como regra de gameplay, preciso decidir se uma identidade pode executar uma ação sobre um recurso, recebendo `false` como resposta esperada quando a regra local não concede.

**Why this priority**: estabelece o caminho de decisão de permissão sem introdução de autoridade externa e cobre explicitamente a recusa, exigida pelas regras do contrato.

**Independent Test**: chamar `validar_permissao` com uma combinação concedida e outra não concedida, conferindo `true`/`false` e o caminho normal da recusa.

**Acceptance Scenarios**:

1. **Given** uma identidade resolvida e uma combinação `(acao, recurso)` coberta por regra local de concessão, **When** `validar_permissao` é chamado, **Then** retorna `true` (`PERMISSAO_CONCEDIDA`).
2. **Given** uma combinação sem regra de concessão, **When** `validar_permissao` é chamado, **Then** retorna `false` (`PERMISSAO_NEGADA`) como caminho normal, sem erro e sem consulta remota.

---

### User Story 3 - Revogar sessão no estado local (Priority: P3)

Como adaptador em processo, preciso revogar uma sessão e refletir essa revogação nas consultas seguintes, para que o estado local seja consistente dentro do mesmo objeto.

**Why this priority**: fecha o ciclo de vida da sessão sem introduzir expiração distribuída nem revogação remota.

**Independent Test**: revogar uma sessão ativa e uma sessão inexistente, conferindo `REVOGADA`/`NAO_REVOGADA` e o resultado de `identidade` após a revogação.

**Acceptance Scenarios**:

1. **Given** uma sessão ativa, **When** `revogar` é chamado, **Then** retorna `true`/`REVOGADA` e uma chamada posterior a `identidade` retorna `SESSAO_INVALIDA`.
2. **Given** uma sessão inexistente ou já revogada, **When** `revogar` é chamado, **Then** retorna `false`/`NAO_REVOGADA`, sem efeito duplicado.

---

### Edge Cases

- Credenciais vazias ou incompletas → `CREDENCIAIS_AUSENTES`; credenciais recusadas → `CREDENCIAIS_INVALIDAS`, sem sessão e sem vazamento de causa na mensagem.
- Sessão desconhecida, expirada localmente ou revogada em `identidade` → `SESSAO_INVALIDA`, sem campo `identidade`.
- `validar_permissao` sem regra local de concessão → `false` (`PERMISSAO_NEGADA`), sem exceção e sem erro de validação.
- `revogar` de sessão inexistente ou revogada → `false` (`NAO_REVOGADA`); uma segunda revogação não altera o estado.
- Tentativa de usar provedor remoto, token, chave privada ou segredo → fora de escopo e **[NV]**; nenhuma credencial é lida, copiada ou versionada.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: A porta `IAMInterface` DEVE existir em `nucleo/iam/iam_interface.gd` com exatamente quatro métodos: `autenticar(credenciais: Dictionary) -> Dictionary`, `identidade(sessao: String) -> Dictionary`, `validar_permissao(identidade: Dictionary, acao: String, recurso: String) -> bool` e `revogar(sessao: String) -> bool`. Nomes de parâmetros e tipos são imutáveis; nenhum método recebe ou devolve segredo, token de provedor, endpoint ou SDK concreto (`docs/architecture/iam-interface.md:42`).
- **FR-002**: `IAMAdapterLocal` DEVE ser implementado em `nucleo/iam/iam_adapter_local.gd` como adaptador em processo que implementa a porta, com identidade determinística, sem servidor, sem segredo e sem credencial persistida; login NÃO é obrigatório. Nenhum código de domínio chama SDK ou provedor concreto (P-II).
- **FR-003**: `autenticar` DEVE devolver `Dictionary` com `sucesso`, `codigo` e `mensagem` e, quando aplicável, `sessao` e `identidade`. Os únicos códigos são `AUTENTICADO` (sucesso, com `sessao` e `identidade`), `CREDENCIAIS_INVALIDAS` (recusa, nenhuma sessão emitida) e `CREDENCIAIS_AUSENTES` (dicionário vazio ou incompleto). A mensagem NÃO distingue "usuário inexistente" de "segredo incorreto" (`docs/architecture/iam-interface.md:70`).
- **FR-004**: `identidade` DEVE devolver `SESSAO_VALIDA` com `identidade` presente para sessão conhecida e ativa, e `SESSAO_INVALIDA` para sessão desconhecida, expirada localmente ou revogada; nenhuma sessão remota é consultada.
- **FR-005**: `validar_permissao` DEVE devolver `true` somente quando uma regra local fixa concede a `(acao, recurso)` para a identidade; caso contrário, DEVE devolver `false` (`PERMISSAO_NEGADA`), tratado como caminho normal, não como erro (`docs/architecture/iam-interface.md:65-66`). A tabela de regras é local e não versiona credencial.
- **FR-006**: `revogar` DEVE operar somente sobre estado local da sessão e devolver `true`/`REVOGADA` se a sessão existia e foi revogada, e `false`/`NAO_REVOGADA` se não existia ou já estava revogada. Nenhuma revogação remota ou federada é afirmada.
- **FR-007**: `tests/test_iam_adapter_local.gd` DEVE cobrir explicitamente os caminhos de sucesso, a recusa de permissão (`PERMISSAO_NEGADA`/`false`) e a sessão inválida (`SESSAO_INVALIDA`), com saída `PASS`/`OK` e exit 0. O código DEVE usar TAB, tipos explícitos, `HashingContext` (não `sha256_string`), `Time.get_ticks_msec` (não `OS.get_ticks_msec`) e variáveis locais que não colidem com `for i`.
- **FR-008**: A entrega DEVE passar por R4 (`validate`) com gate executado, saída colada, data/hora e conformidade, e por R7 (`release`) com SemVer + changelog contendo IDs FR/T/bug + rollback, com R4 aprovado antes de R7. Remoto, CI e merge permanecem **[NV]** até confirmação com saída colada e data/hora (`docs/governance/sdd-phase-contracts.md:26-27`).

### Key Entities *(include if feature involves data)*

- **Porta `IAMInterface`**: contrato imutável; atributos = quatro métodos, nomes de parâmetros, tipos e códigos de retorno; identidade pelo nome da porta.
- **`IAMAdapterLocal`**: adaptador em processo; atributos = regras locais fixas, sessões em memória e estado de revogação; identidade pela classe.
- **Sessão**: identificador opaco local; atributos = valor e estado (ativa ou revogada); identidade pelo valor.
- **Identidade**: resultado da resolução local; atributos = campos determinísticos derivados da sessão; nenhum segredo.
- **Resultado de autenticação/identidade**: `Dictionary` com `sucesso`, `codigo`, `mensagem` e, quando aplicável, `sessao` e `identidade`; o domínio compara códigos, nunca mensagens.
- **Regra de permissão**: par `(acao, recurso)` → decisão local; recusa é caminho normal, não erro.
- **Evidência R4/R7**: gate executado, saída colada, data/hora, conformidade, SemVer, changelog com IDs FR/T/bug e rollback.

## Success Criteria *(mandatory)*

Critérios de aceite a aferir na implementação e no R4; nenhum comando de gate, teste, `make` ou Godot foi executado nesta etapa de especificação.

### Measurable Outcomes

- **SC-001**: 4 de 4 métodos da porta implementados com assinaturas e nomes de parâmetro exatos e 9 de 9 códigos de retorno inalterados; 0 renomeações. (baseline: 0 implementados; meta: 4 de 4 e 9 de 9)
- **SC-002**: 100% dos casos de `tests/test_iam_adapter_local.gd` com saída `PASS` e `OK test_iam_adapter_local`, exit 0 e 0 `FAIL`; 1 caso de `PERMISSAO_NEGADA` e 1 de `SESSAO_INVALIDA` presentes. (baseline: 0 testes; meta: 100% de `PASS`/`OK`, exit 0)
- **SC-003**: `make check GODOT=<binário>` com exit 0, 5 `OK` + `COMPAT_OK` e 0 `FAIL`. (baseline: não executado nesta etapa; meta: exit 0)
- **SC-004**: 5 de 5 gates determinísticos com `PASS` na verificação da implementação, sem alteração de baseline para ocultar falha. (baseline: 5 de 5 registrados em `.specify/memory/STATE.md:11`; meta: 5 de 5 preservados)
- **SC-005**: 0 segredos, 0 chaves privadas, 0 credenciais versionadas, 0 servidor central, 0 federação, 0 login obrigatório e 0 partidas 5v5; 100% das afirmações remotas rotuladas **[NV]** até evidência; R4 aprovado antes de R7, com 1 release SemVer + changelog de FR/T/bug + rollback. (baseline: 0; meta: 0)

## Assumptions

- O modo verificado é solo/single, confirmado pela SE em 2026-11-20T10:58Z (`.specify/memory/STATE.md:9`); nenhum login obrigatório, servidor central, federação ou 5v5 é introduzido.
- A SE autorizou este ciclo (`.specify/memory/STATE.md:5,7`); a ressalva de "projeto, não implementado" de ADR-003 e `docs/architecture/iam-interface.md:76` refere-se ao ciclo anterior.
- A porta e os códigos são imutáveis (`docs/architecture/iam-interface.md:42,70`); eventual provedor real entra como novo adaptador atrás da mesma porta e permanece **[NV]**.
- Convenções GDScript do alvo: TAB, tipos explícitos, `HashingContext`, `Time.get_ticks_msec` e variáveis locais sem colisão com `for i`.
- Nenhum gate, teste, `make`, Godot, stage, commit ou push é executado nesta etapa; nenhum arquivo fora dos três previstos é escrito.
- Os critérios de sucesso são metas a aferir na implementação e em R4, não resultados já observados nesta especificação.

## Clarifications

### Session 2026-11-20

- **Q1: Como a identidade é obtida sem servidor?** → A: de forma determinística e local — `IAMAdapterLocal` resolve a identidade por regras locais fixas sobre `(identidade, acao, recurso)`, em processo, sem consulta remota, sem token de provedor e sem segredo. (fontes: `docs/adr/003-iam-adapter.md:23-27`; `docs/architecture/iam-interface.md:12,72-78`; `.specify/memory/STATE.md:7`)
- **Q2: O que significa revogar uma sessão?** → A: apenas estado local — `revogar` marca a sessão como revogada no adaptador e devolve `REVOGADA`/`NAO_REVOGADA`; nenhuma sessão remota, expiração distribuída ou revogação federada é afirmada, e consultas posteriores à mesma sessão retornam `SESSAO_INVALIDA`. (fontes: `docs/architecture/iam-interface.md:37-39,68`; `.specify/memory/STATE.md:9`)
- **Q3: Como tratar a recusa de permissão?** → A: `validar_permissao` devolve `false` (`PERMISSAO_NEGADA`) como caminho normal, não como erro; a suíte cobre explicitamente esse caminho negativo e o de `SESSAO_INVALIDA`, além dos casos de sucesso. (fontes: `docs/architecture/iam-interface.md:65-66,85`; `docs/adr/003-iam-adapter.md:41`)
- **Q4: Alguma credencial é versionada?** → A: nenhuma — nenhum segredo, token, chave privada ou credencial real entra em arquivo versionado, log ou mensagem; o adaptador local não possui segredo e o login não é obrigatório. (fontes: `docs/architecture/iam-interface.md:84`; `docs/adr/003-iam-adapter.md:41`; `.specify/memory/STATE.md:9`)

0 perguntas ao usuário; ambiguidades resolvidas por evidência local (decisão R1, ADR-003, contrato de arquitetura e diretriz viva da SE). Cada esclarecimento foi formulado e resolvido isoladamente; nenhum marcador `[NEEDS CLARIFICATION]` permaneceu em aberto.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.
>
> Esta feature não produz manifest de orçamento de contexto nem saídas com handles de contexto; nenhum artefato dessa natureza é exigido no ciclo 018. A verificação é a dos cinco gates determinísticos, de `make check` e da suíte `tests/test_iam_adapter_local.gd`, conforme SC-001–SC-005, aferida na implementação e em R4 — nunca nesta etapa de especificação.
