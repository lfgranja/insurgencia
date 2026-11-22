# Idea Intake: IAMAdapterLocal em processo

- **Slug**: iam-adapter-local
- **Created**: 2026-11-20
- **Source**: aceite da SE após avaliação de viabilidade (2026-11-20T11:00Z, registrado no contexto desta sessão); decisão da SE: implementar `IAMAdapterLocal` em processo, sem servidor central e sem nova abstração — a porta já é `IAMInterface`, definida em `docs/architecture/iam-interface.md` e decidida em `docs/adr/003-iam-adapter.md` (ADR-003)
- **Type**: new-capability (implementação de adaptador já decidida; não é reabertura de decisão)

## Idea (as captured)

> "Decisão da SE (11:00Z): implementar `IAMAdapterLocal` em processo, SEM servidor central, SEM nova abstração (a porta é `IAMInterface` de `docs/architecture/iam-interface.md`, ADR-003). Contrato exato: `autenticar(credenciais: Dictionary)->Dictionary`; `identidade(sessao: String)->Dictionary`; `validar_permissao(identidade: Dictionary, acao: String, recurso: String)->bool`; `revogar(sessao: String)->bool`. Adaptador local determinístico, sem segredo e sem credencial persistida; login NÃO obrigatório."

> Aceitação para esta etapa: escrever EXATAMENTE três artefatos de assess (`intake.md`, `research.md`, `problem.md`); não escrever `decision.md`, não escrever código, não rodar gates/testes/make/Godot, não fazer stage/commit/push e não tocar arquivos fora da lista atribuída.

## Restated

Implementar, no processo do jogo e atrás da porta `IAMInterface` já existente no projeto, um adaptador local determinístico de identidade que autentique credenciais simulatedas, resolva sessões, autorize/recuse ações sobre recursos e revogue sessões — sem servidor IAM, sem segredo, sem credencial persistida e sem login obrigatório. A mudança é de implementação de um adaptador previsto, não de redefinição de arquitetura: a porta, os nomes de método/parâmetro, os códigos de retorno e a decisão ADR-003 ficam intactos.

## Origin & Context

- **Raised by**: Secretaria-Executiva (SE), aceite após avaliação de viabilidade em 2026-11-20T11:00Z (dado do contexto da sessão; nenhuma mutação remota é afirmada).
- **Trigger**: ADR-003 (`docs/adr/003-iam-adapter.md`) aprovou a substituição de Zitadel por `IAMInterface` + adaptador local determinístico, mas registrou explicitamente o adaptador como "projeto neste ciclo e não implementado aqui" (linha 26); `docs/architecture/iam-interface.md:76` confirma que nenhum `.gd`, classe `IAMAdapterLocal` ou teste existe. O aceite da SE autoriza agora sair do estado de projeto para a implementação.
- **Continuidade, não ruptura**: o contrato de quatro métodos e a tabela de códigos legíveis de `docs/architecture/iam-interface.md:18-70` são a entrada obrigatória; nenhuma assinatura, código ou nome de parâmetro é renomeado por esta ideia.
- **Contexto de localização**: os artefatos de avaliação ficam em `.specify/assessments/iam-adapter-local/`, colocalizados com a feature `.specify/feature.json` → `specs/018-iam-adapter-local`. Os arquivos de código previstos para ciclo posterior são `nucleo/iam/iam_interface.gd`, `nucleo/iam/iam_adapter_local.gd` e `tests/test_iam_adapter_local.gd`; nenhum deles é escrito nesta etapa.
- **Limites de autoridade**: a decisão de implementar o adaptador local não autoriza servidor IAM, provedor remoto, credencial real, login obrigatório, 5v5, lobby, federação nem reabertura de ADR-003; provedor real segue **[NV]** (ADR-003, linha 42; `docs/architecture/iam-interface.md:12`).

## Restrições fixadas pelo aceite

1. **Em processo**: o adaptador roda dentro do Núcleo, sem serviço externo no caminho crítico (P-IV, `.specify/memory/constitution.md:13`).
2. **Sem servidor central**: nenhuma instância, endpoint, OIDC, Zitadel ou autoridade remota é introduzida; essa intenção foi rejeitada no ADR-003 e permanece **[NV]** quanto a qualquer provedor real.
3. **Sem segredo e sem credencial persistida**: nenhum token, segredo ou credencial real é lido, gerado, gravado, versionado ou registrado em log/mensagem; todas as entradas são determinísticas e locais.
4. **Login NÃO obrigatório**: o modo solo/single verificado não exige conta, autenticação remota ou etapa de login para jogar.
5. **Não reabrir ADR-003**: a decisão Accepted de 2026-11-20 permanece como está; troca de provedor futuro continua sendo novo adaptador atrás da mesma porta (regra 2 de `docs/architecture/iam-interface.md:83`).
6. **Sem nova abstração**: a porta é `IAMInterface`; nenhuma interface, trait, fachada ou camada adicional é criada para esta feature.
7. **Contrato exato**: os quatro métodos e seus parâmetros são os de `docs/architecture/iam-interface.md:24-39`; os retornos usam somente os códigos de `docs/architecture/iam-interface.md:58-68`.
8. **Escopo de escrita desta etapa**: somente os três artefatos de assess; nenhum gate, teste, build, Godot, stage ou commit executado por esta atribuição.

## First-Glance Unknowns

- Qual tabela concreta de regras locais `(identidade, acao, recurso)` será usada por `validar_permissao`? O documento de arquitetura fixa o formato, mas diz que a tabela concreta fica para ciclo próprio (`docs/architecture/iam-interface.md:74,76`).
- Que forma mínima de `credenciais: Dictionary` é aceita como `CREDENCIAIS_AUSENTES` versus `CREDENCIAIS_INVALIDAS`? A distinção está definida em código (`docs/architecture/iam-interface.md:61-62`), mas o conteúdo do dicionário ainda não foi fixado.
- Como `sessao` é derivada de forma determinística sem segredo nem persistência? A porta exige `sessao` opaca presente em `AUTENTICADO` e ausente em falha (linha 53,60); o mecanismo de geração local é decisão de implementação, não de arquitetura.
- O adaptador local precisará de passagem explícita de tempo para expiração de sessão (`SESSAO_INVALIDA` inclui "expirada" na linha 64)? A ausência de relógio determinístico é risco a resolver na modelagem, sem introduzir serviço externo.
- A documentação remota da API Godot `Crypto` foi consultada antes como contexto, mas qualquer afirmação sobre seu conteúdo é **[NV]**; ela não é usada como assinatura nesta feature — o adaptador é determinístico e não depende de assinatura/criptografia.
