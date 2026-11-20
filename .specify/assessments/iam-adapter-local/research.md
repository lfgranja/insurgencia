# Idea Research: IAMAdapterLocal em processo

- **Slug**: iam-adapter-local
- **Created**: 2026-11-20
- **Evidence confidence (overall)**: high — todas as afirmações de projeto são locais e citadas (ADR-003, contrato de arquitetura, constituição, contrato de códigos e ausência de código em `nucleo`); toda afirmação sobre documento remoto, provedor real ou serviço externo está marcada **[NV]**. Os gates 5/5 em 11:16:42Z são registro do contexto da sessão, não reexecução nesta etapa.

## Users & Demand

- A SE é a demandante: aceite após avaliação de viabilidade em 2026-11-20T11:00Z decide implementar o adaptador local em processo, sem servidor central e sem nova abstração. [source: contexto da atribuição, 2026-11-20T11:00Z] (confidence: high — dado do pedido)
- O domínio do Núcleo é o consumidor direto: ele passa a chamar a porta `IAMInterface`, nunca SDK, token ou provedor concreto — exigência do princípio II. [source: `.specify/memory/constitution.md:9`; `docs/architecture/iam-interface.md:10,82`] (confidence: high)
- O jogador do modo solo/single é o beneficiário indireto: a identidade local não cria login obrigatório nem serviço externo no caminho crítico, preservando o pilar de soberania. [source: `docs/adr/003-iam-adapter.md:15,40`; `.specify/memory/constitution.md:13`; `.specify/memory/STATE.md:7`] (confidence: high)
- Usuários remotos, federação e provedor real não são demandantes neste ciclo: Zitadel como autoridade central foi rejeitado e qualquer provedor real permanece **[NV]**. [source: `docs/adr/003-iam-adapter.md:32,42`; `.specify/memory/STATE.md:9`] (confidence: high)

## Prior Art

- **ADR-003 (Accepted, 2026-11-20)** substitui Zitadel por `IAMInterface` + adaptador local determinístico e registra o adaptador como "projeto neste ciclo e não implementado aqui". [source: `docs/adr/003-iam-adapter.md:21-28`] (confidence: high)
- **Contrato da porta** em `docs/architecture/iam-interface.md:18-42` fixa quatro métodos com nomes e parâmetros imutáveis por adaptador: `autenticar(credenciais: Dictionary) -> Dictionary`, `identidade(sessao: String) -> Dictionary`, `validar_permissao(identidade: Dictionary, acao: String, recurso: String) -> bool` e `revogar(sessao: String) -> bool`. [source: leitura local, 2026-11-20] (confidence: high)
- **Contrato de retorno** exige `sucesso`, `codigo` e `mensagem` em todo `Dictionary`, com `sessao`/`identidade` quando aplicável; o domínio compara códigos, nunca mensagens. [source: `docs/architecture/iam-interface.md:44-55,70`] (confidence: high)
- **Códigos legíveis** (tabela estável): `AUTENTICADO`, `CREDENCIAIS_INVALIDAS`, `CREDENCIAIS_AUSENTES`, `SESSAO_VALIDA`, `SESSAO_INVALIDA`, `PERMISSAO_CONCEDIDA` (`true`), `PERMISSAO_NEGADA` (`false`), `REVOGADA`, `NAO_REVOGADA`. [source: `docs/architecture/iam-interface.md:58-68`] (confidence: high)
- **Regras de fronteira**: domínio sem SDK concreto; troca de provedor = novo adaptador; nenhuma credencial em arquivo versionado; testes futuros devem cobrir explicitamente `PERMISSAO_NEGADA` e sessão inválida. [source: `docs/architecture/iam-interface.md:80-85`] (confidence: high)
- **Ausência de implementação**: `docs/architecture/iam-interface.md:76` afirma que nenhum `.gd`, classe `IAMAdapterLocal` ou teste existe; o grep local em `nucleo` (2026-11-20) por `iam|autenticar|identidade|permiss|revogar|sessao|sessão` não retornou nenhuma linha. [source: leitura local e `grep -rniE ... nucleo`, 2026-11-20] (confidence: high)
- **Documentação remota da API Godot `Crypto`**: consultada anteriormente apenas como contexto; qualquer afirmação sobre seu conteúdo, endpoints ou versões é **[NV]** e não foi revalidada nesta etapa. Ela não é assinatura para esta feature: o adaptador é determinístico, sem segredo e não usa assinatura/criptografia. [source: documento remoto docs.godotengine.org, consulta prévia — **[NV]**] (confidence: low para conteúdo remoto; high para a decisão de não usá-lo)

## Market & Context

- O custo da ausência do adaptador é a impossibilidade de exercer, no domínio, autorização e revogação locais sem editar o próprio domínio — a porta já está decidida, mas o adaptador previsto não existe. [source: `docs/adr/003-iam-adapter.md:26`; `docs/architecture/iam-interface.md:76`; grep local em `nucleo`] (confidence: high)
- A alternativa remota (IdP central/OIDC) foi avaliada e rejeitada por centralização e ponto único de falha; instância, endpoint e credencial de qualquer provedor real seguem **[NV]**. [source: `docs/adr/003-iam-adapter.md:15,32,42`] (confidence: high)
- O ecossistema verificado é solo/single, sem partidas 5v5, lobby, NAT ou escala de produção — não há pressão de mercado por federação neste ciclo. [source: `.specify/memory/STATE.md:7,9`; `docs/adr/003-iam-adapter.md:16`] (confidence: high)
- Não há custo de licença, serviço ou infraestrutura previsto: a implementação é em processo, com regras locais fixas e sem segredo. [source: `docs/adr/003-iam-adapter.md:26,41`; `docs/architecture/iam-interface.md:74,84`] (confidence: high)

## Data & Constraints

- A implementação prevista usa GDScript e os arquivos `nucleo/iam/iam_interface.gd`, `nucleo/iam/iam_adapter_local.gd` e `tests/test_iam_adapter_local.gd`; armadilhas conhecidas do repo: usar TAB, tipos explícitos, `HashingContext` (não `sha256_string`), `Time.get_ticks_msec` (não `OS.get_ticks_msec`) e evitar colisão de variáveis locais com `for i`. [source: contexto da atribuição e decisão da SE, 2026-11-20] (confidence: high — restrição de escrita, não comportamento executado)
- O adaptador não recebe nem devolve segredo, token de provedor, endpoint ou SDK concreto, e os nomes de parâmetros do contrato não são renomeados. [source: `docs/architecture/iam-interface.md:42,82-84`] (confidence: high)
- `validar_permissao` resolve por regras locais fixas sobre `(identidade, acao, recurso)`, com recusa explícita quando a regra não concede a ação; `PERMISSAO_NEGADA` é caminho normal, não erro. [source: `docs/architecture/iam-interface.md:74,66,85`] (confidence: high)
- `revogar` retorna `true` somente se a sessão existia e foi revogada (`REVOGADA`), `false` para sessão inexistente ou já revogada (`NAO_REVOGADA`). [source: `docs/architecture/iam-interface.md:37-39,67-68`] (confidence: high)
- A constituição exige determinismo (P-I), portas e adaptadores (P-II), fluxo SDD ordenado (P-III), soberania sem autoridade central (P-IV) e verificação honesta com `[NV]` até confirmação ao vivo (P-VI). [source: `.specify/memory/constitution.md:7-17`] (confidence: high)
- Gates determinísticos: registro de 5/5 PASS em 2026-11-20T11:16:42Z no contexto da sessão; nenhum gate, teste, `make` ou Godot foi reexecutado nesta etapa de escrita e nenhuma saída de console é colada aqui. [source: contexto da atribuição, 2026-11-20T11:16:42Z] (confidence: high para o registro; nenhuma afirmação de reexecução)
- Escopo de escrita desta etapa: somente `intake.md`, `research.md` e `problem.md` em `.specify/assessments/iam-adapter-local/`; `decision.md`, código e mutações fora dessa lista estão fora da atribuição. [source: atribuição do mantenedor, 2026-11-20] (confidence: high)

## Evidence Against the Idea

- A implementação do adaptador local poderia ser lida como reabertura de ADR-003, que registrou a implementação como projeto e atribuiu a um "ciclo próprio" a tabela concreta de permissões e os testes de recusa. Mitigação: preservar integralmente a porta, os códigos e o status Accepted; nada de ADR-003 é editado. [source: `docs/adr/003-iam-adapter.md:26,35,41`; `docs/architecture/iam-interface.md:76`] (confidence: high)
- Uma identidade "determinística" sem segredo pode ser previsível e, se mal modelada, confundir autenticação com autorização. Mitigação: manter a distinção entre credenciais ausentes/inválidas, sessão inválida e `PERMISSAO_NEGADA`, sem inventar segredo nem persistência. [source: `docs/architecture/iam-interface.md:61-68,84`] (confidence: high)
- A ausência de relógio determinístico torna "expirada" em `SESSAO_INVALIDA` ambígua em um adaptador local; inventar tempo não determinístico contrariaria P-I. Mitigação: tratar expiração apenas se houver fonte explícita e determinística, ou recusá-la explicitamente. [source: `docs/architecture/iam-interface.md:64`; `.specify/memory/constitution.md:7`] (confidence: medium)
- Qualquer tentativa de usar documentação remota da API Godot `Crypto` como justificativa de assinatura introduziria uma dependência que a decisão não pede e cujo conteúdo remoto está **[NV]**. Mitigação: não usar assinatura; o adaptador é determinístico e local. [source: contexto da atribuição; documento remoto **[NV]**] (confidence: high para a restrição, low para conteúdo remoto)
- Gates, build e comportamento de execução não foram observados nesta etapa; afirmar verde seria violar P-VI. Mitigação: registrar apenas o registro de 5/5 em 11:16:42Z e manter **[NV]** qualquer afirmação de execução futura. [source: `.specify/memory/constitution.md:17`; contexto da atribuição] (confidence: high)

## Gaps & Open Questions

- Qual é a tabela concreta de `(identidade, acao, recurso)` que concede ou recusa cada ação? (a definir na especificação/código; o contrato só fixa o formato)
- Como distinguir `CREDENCIAIS_AUSENTES` de `CREDENCIAIS_INVALIDAS` para um `Dictionary` parcialmente preenchido? (a definir na especificação)
- Como gerar `sessao` opaca de forma determinística, sem segredo e sem persistência, e quando considerá-la inválida? (a definir na especificação)
- Os códigos de retorno e os nomes de parâmetros devem permanecer exatamente como no contrato, inclusive em mensagens de erro? (respondido: sim — `docs/architecture/iam-interface.md:42,46,70`)
- Godot/Crypto remoto: alguma afirmação sobre a API remota permanece **[NV]** e não é usada como fundamento de assinatura.

## Sources

- `docs/adr/003-iam-adapter.md` (política: `local-only`) — ADR Accepted de 2026-11-20; decisão, alternativas rejeitadas, consequências e escopo de não implementação.
- `docs/architecture/iam-interface.md` (política: `local-only`) — contrato de quatro métodos, tabela de campos e códigos, regras e fora de escopo.
- `.specify/memory/constitution.md` (política: `local-only`) — P-II em linha 9 e P-IV em linha 13; determinismo (P-I, linha 7) e verificação honesta (P-VI, linha 17).
- `.specify/memory/STATE.md` (política: `local-only`) — estado vivo 2026-11-20T10:58Z: identidade local decidida, provedor remoto **[NV]** e escopo solo/single.
- `nucleo/` (política: `local-only`) — `grep -rniE 'iam|autenticar|identidade|permiss|revogar|sessao|sessão' nucleo`, executado em 2026-11-20, nenhuma linha retornada.
- Contexto da sessão (política: dado do pedido) — aceite da SE às 2026-11-20T11:00Z após avaliação de viabilidade; gates 5/5 às 2026-11-20T11:16:42Z; restrições de escrita e armadilhas GDScript.
- Documentação remota `docs.godotengine.org` (política: remota) — consultada anteriormente como contexto; **toda afirmação sobre seu conteúdo é [NV]**; não usada como assinatura nesta feature.
