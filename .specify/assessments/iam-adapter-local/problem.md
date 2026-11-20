# Problem Definition: IAMAdapterLocal em processo

- **Slug**: iam-adapter-local
- **Created**: 2026-11-20
- **Inputs used**: intake.md | research.md | aceite da SE 2026-11-20T11:00Z | ADR-003 | `docs/architecture/iam-interface.md`
- **Impacto**: 3 (em escala 1–5) — afeta a fronteira de identidade do Núcleo e destrava autorização/revogação locais futuras, sem impacto em gameplay, produto remoto ou jogadores hoje.
- **Confiança**: 4 (em escala 1–5) — decisão, contrato, ausência de código e restrições são verificados localmente; permanece incerteza apenas sobre regras concretas de permissão e sessão, a definir na especificação.

## Problem Statement

A porta `IAMInterface` e o adaptador local determinístico foram decididos em 2026-11-20 (ADR-003, `docs/adr/003-iam-adapter.md:21-28`) e detalhados em `docs/architecture/iam-interface.md`, mas **hoje não existe nenhum código IAM no Núcleo**: o grep local em `nucleo/` por `iam|autenticar|identidade|permiss|revogar|sessao|sessão` retornou nenhuma linha, e o próprio documento de arquitetura declara que nenhum `.gd`, classe `IAMAdapterLocal` ou teste existe (`docs/architecture/iam-interface.md:76`). Consequentemente, o domínio não tem como exercer, por contrato, autenticação local, resolução de sessão, autorização/recusa de ações sobre recursos nem revogação — e qualquer necessidade futura de permissões de módulos ou revogação exigiria editar o domínio, contrariando P-II (`.specify/memory/constitution.md:9`). O aceite da SE às 11:00Z remove a ambiguidade de escopo: implementar em processo, sem servidor central, sem nova abstração, sem credencial real e sem login obrigatório.

## Affected Users & Stakeholders

- **Domínio/Núcleo**: consumidor da porta; precisa de um adaptador que implemente os quatro métodos sem conhecer SDK ou provedor concreto (P-II).
- **Módulos futuros com permissões**: hoje sem mecanismo contratual de `validar_permissao` e `revogar`; serão os primeiros beneficiários da tabela local de regras.
- **SE e mantenedor**: precisam que a decisão ADR-003 seja materializada sem reabertura, sem servidor e sem credencial, preservando o ciclo solo/single.
- **Jogador do modo solo/single**: afetado apenas de forma negativa se a implementação introduzir login obrigatório, serviço externo ou dependência proprietária — todos explicitamente fora de escopo (`.specify/memory/constitution.md:13`; `docs/architecture/iam-interface.md:89`).
- **Auditores/governança**: dependem de que códigos de retorno estáveis e verificação honesta (`[NV]` até execução) sejam preservados (`.specify/memory/constitution.md:17`).

## Goals

- Implementar `IAMAdapterLocal` em processo como adaptador de `IAMInterface`, sem criar nova abstração e sem alterar ADR-003.
- Preservar o contrato exato: `autenticar(credenciais: Dictionary)->Dictionary`, `identidade(sessao: String)->Dictionary`, `validar_permissao(identidade: Dictionary, acao: String, recurso: String)->bool` e `revogar(sessao: String)->bool`.
- Preservar os códigos legíveis: `AUTENTICADO`, `CREDENCIAIS_INVALIDAS`, `CREDENCIAIS_AUSENTES`, `SESSAO_VALIDA`, `SESSAO_INVALIDA`, `PERMISSAO_CONCEDIDA`, `PERMISSAO_NEGADA`, `REVOGADA`, `NAO_REVOGADA`.
- Garantir determinismo e ausência de segredo: nenhuma credencial real, token, endpoint, persistência de credencial ou assinatura; nenhum login obrigatório.
- Cobrir explicitamente os caminhos negativos — `PERMISSAO_NEGADA`/`false`, `SESSAO_INVALIDA` e `NAO_REVOGADA` — conforme regra 4 de `docs/architecture/iam-interface.md:85`.
- Entregar os arquivos previstos `nucleo/iam/iam_interface.gd`, `nucleo/iam/iam_adapter_local.gd` e `tests/test_iam_adapter_local.gd` em ciclo de implementação, não nesta etapa de assess.

## Non-Goals

- Servidor IAM central, IdP remoto, Zitadel, OIDC, endpoint, token de provedor ou qualquer autoridade externa (rejeitados em ADR-003; provedor real permanece **[NV]**).
- Login obrigatório, conta corporativa, lobby, matchmaking, partidas 5v5, NAT traversal ou escala de produção.
- Federação forte, Postgres/Coolify remotos, `pgmq` ou deploy/publicação.
- Credencial real, segredo, token ou qualquer credencial persistida/versionada — inclusive em log, mensagem ou fixture versionada.
- Nova abstração: nenhuma interface, trait, fachada ou camada além da porta `IAMInterface` existente.
- Reabertura de ADR-003, alteração de contrato de retorno, renomeação de parâmetros ou códigos.
- Nesta etapa: escrever `decision.md`, código, gates, testes, build, Godot, stage/commit/push ou arquivos fora dos três artefatos atribuídos.

## Success Metrics

- Contrato: 4 de 4 métodos implementados com assinatura e nomes de parâmetros idênticos aos de `docs/architecture/iam-interface.md:24-39` (baseline: 0 de 4 implementados; verificar na implementação, **[NV]** até então).
- Códigos: 9 de 9 códigos da tabela preservados e retornados nos caminhos aplicáveis (baseline: 0 implementados; **[NV]** até implementação).
- Caminhos negativos: 100% dos casos de recusa (`PERMISSAO_NEGADA`/`false`), sessão inválida e não revogação cobertos por `tests/test_iam_adapter_local.gd` (baseline: nenhum arquivo de teste; **[NV]** até implementação).
- Determinismo: 2 execuções com as mesmas entradas produzem o mesmo `Dictionary`/`bool`; 0 segredos, 0 credenciais persistidas e 0 chamadas de rede (baseline: 0 implementado; **[NV]** até implementação).
- Fronteira: 0 referências a SDK, endpoint, token ou provedor concreto em `nucleo/iam/` (baseline: 0 arquivos; verificar na implementação).
- Escopo: 0 arquivos fora da lista de três artefatos de assess escritos nesta etapa; 0 `decision.md`; 0 gates/testes/make/Godot executados por esta atribuição (baseline: atribuição; verificar por listagem).
- Salvaguardas: 0 mudanças em ADR-003 e 0 alterações no contrato de `docs/architecture/iam-interface.md`.
- Registro de verificação: gates 5/5 em 2026-11-20T11:16:42Z registrados no contexto da sessão; nenhuma reexecução afirmada nesta etapa.

## Cost of Inaction

Sem o adaptador, a porta aprovada permanece documento sem efeito: módulos futuros que precisarem de permissões ou revogação terão de editar o domínio, violando P-II e tornando a troca de provedor uma reescrita. A ausência de código também mantém a recusa de permissão e a sessão inválida sem cobertura de teste, contrariando a regra 4 da arquitetura; e prolonga o estado em que qualquer afirmação de comportamento do adaptador é **[NV]**. O custo é governança e manutenção, não risco de gameplay: o modo solo/single segue jogável e nenhum serviço externo é introduzido pela inação.

## Open Questions

- Q1 — Qual tabela concreta de `(identidade, acao, recurso)` concede ou recusa cada ação? A definir na especificação; o contrato fixa apenas o formato (`docs/architecture/iam-interface.md:74,76`).
- Q2 — Como diferenciar `CREDENCIAIS_AUSENTES` de `CREDENCIAIS_INVALIDAS` em um `Dictionary` parcialmente preenchido? A definir na especificação (`docs/architecture/iam-interface.md:61-62`).
- Q3 — Como gerar `sessao` opaca de forma determinística, sem segredo nem persistência, e quando considerá-la inválida/expirada? A definir na especificação (`docs/architecture/iam-interface.md:53,64`).
- Q4 — A documentação remota da API Godot `Crypto` influencia a implementação? Não: conteúdo remoto é **[NV]** e não é usado como assinatura; o adaptador é determinístico e local.
