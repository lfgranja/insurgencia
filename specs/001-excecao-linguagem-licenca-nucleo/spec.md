# Feature Specification: Exceção de linguagem e licença MIT-only do Núcleo

**Feature Branch**: `001-excecao-linguagem-licenca-nucleo`

**Created**: 2026-11-19

**Status**: Draft

**Input**: Decisão R1 de 2026-11-19 (veredito `go`, opção A) — dois registros de decisão locais: ADR-001 (exceção de linguagem) e ADR-002 (licença MIT-only do Núcleo), sem reabrir a decisão Godot-first.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Registrar a exceção de linguagem para GDScript (Priority: P1)

O mantenedor do Núcleo precisa de um registro local que autorize GDScript como linguagem de engine e script de jogo, coerente com a decisão Godot-first já vigente, antes que qualquer código exista. O registro converte uma pressuposição documental em decisão rastreável, sem alterar a norma externa nem a escolha de motor.

**Why this priority**: É o bloqueante de governança: sem autorização escrita para a linguagem efetivamente adotada, qualquer trabalho futuro herda um desvio de intenção classificado como bloqueante.

**Independent Test**: Basta ler o registro ADR-001 para verificar que a exceção está declarada, que o pilar permanece intocável e que há critérios de reavaliação; nenhum outro registro é necessário para entregar esse valor.

**Acceptance Scenarios**:

1. **Given** que a decisão Godot-first é vinculante e o repositório ainda é documental, **When** o mantenedor registra a exceção de linguagem, **Then** o ADR-001 declara GDScript como engine/script de jogo e afirma explicitamente que a decisão Godot-first não é reaberta, revista ou revertida.
2. **Given** um ADR-001 registrado, **When** um leitor procura o motivo da exceção, **Then** encontra o contexto, as alternativas consideradas, as consequências, a autoridade de aprovação e os critérios que permitem reavaliá-la no futuro.
3. **Given** uma proposta futura de segunda linguagem ou troca de engine, **When** o ADR-001 é consultado, **Then** ele não autoriza essa mudança no ciclo e a encaminha para aprovação formal e evidência prévia.

---

### User Story 2 — Fechar a licença do Núcleo como MIT-only (Priority: P1)

O mantenedor precisa de uma posição única e não ambígua sobre o licenciamento de todo o código produzido no ciclo — Núcleo e servidor headless — de modo que operadores de servidores e forks possam inspecionar, modificar e redistribuir sem medo de cláusula copyleft acidental.

**Why this priority**: É o segundo risco de maior severidade: a convivência entre a licença MIT vigente e a exigência de AGPL em documento anterior gera divergência entre decisão e documento e risco jurídico para forks.

**Independent Test**: Basta ler o ADR-002 para confirmar a posição MIT-only, a exclusão de alternativas "ou GPL" e de AGPL para o código do ciclo, e a regra de não copiar código de terceiros; isso já resolve a pergunta de licenciamento sem depender do ADR-001.

**Acceptance Scenarios**:

1. **Given** a licença MIT vigente e a pergunta de licenciamento ainda aberta, **When** o mantenedor registra a decisão de licenciamento, **Then** o ADR-002 estabelece MIT como posição única para todo o código do ciclo, sem cláusula alternativa e sem AGPL.
2. **Given** um pedido para incluir código de projeto sob outra licença, **When** a decisão é aplicada, **Then** nenhum código sob licença copyleft é copiado e qualquer contribuição de terceiros é identificada e atribuída, nunca apropriada.
3. **Given** a ressalva de que o motor adotado não é o único motor MIT, **When** a decisão de licenciamento é lida, **Then** ela não atribui licença exclusiva ao motor e não altera a licença do motor, limitando-se ao código do projeto.

---

### User Story 3 — Verificar coerência, aprovação e não reabertura do pilar (Priority: P2)

O mantenedor e a comunidade precisam conferir que as duas decisões foram registradas, aprovadas e ligadas aos desvios de origem, e que nenhuma delas reabre involuntariamente o pilar Godot-first nem transforma o ciclo documental em entrega de código.

**Why this priority**: Sem rastreabilidade e sem a declaração de não reabertura, os ADRs viram intenção verificável apenas por memória — exatamente o risco que motivou a decisão local.

**Independent Test**: A verificação é feita apenas pela leitura cruzada dos dois registros e da decisão R1: contagem de registros, presença das declarações e associação dos desvios, sem executar nenhum artefato.

**Acceptance Scenarios**:

1. **Given** os dois ADRs registrados, **When** um auditor verifica a cobertura, **Then** encontra exatamente dois registros locais, cada um com os campos mínimos exigidos, associados aos desvios de linguagem e de licença e à decisão R1.
2. **Given** status remoto `[NV]`, **When** os ADRs são lidos, **Then** a aprovação humana aparece como autoridade necessária e a execução remota, a integração contínua, o merge e a proteção de branch permanecem explicitamente `[NV]`.
3. **Given** uma pergunta sobre o arquivo de licenças de terceiros, **When** os ADRs são consultados, **Then** ela aparece registrada como fora do escopo do ciclo, sem decisão implícita a seu respeito.

---

### Edge Cases

- Se a norma externa não aceitar a exceção local, o ADR-001 registra a contingência de emenda normativa como possibilidade futura, sem executá-la neste ciclo e sem reabrir o pilar.
- Se a documentação anterior exigir licença diferente para o servidor, o ADR-002 registra que nenhum servidor sob essa licença existe no ciclo e que qualquer mudança documental depende de aprovação explícita — nunca de alteração silenciosa.
- Se aparecer código de terceiros sob licença copyleft, ele não é copiado; a contribuição é identificada e atribuída, e a decisão sobre o arquivo de licenças de terceiros permanece fora do escopo.
- Se o termo "MIT" for aplicado ao motor adotado em documento derivado, a leitura dos ADRs não confirma exclusividade: a ressalva de que o motor não é o único MIT permanece.
- Se a aprovação humana ainda não estiver registrada, os documentos são tratados como decisão pendente de aprovação, não como aprovação obtida; nenhum critério de sucesso depende de status remoto.
- Se uma tentativa de transformar os ADRs em pipeline imediata surgir, ela é reconhecida como fora de escopo, preservando a natureza documental do ciclo.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: A feature DEVE produzir exatamente dois registros de decisão locais versionados: ADR-001 (exceção de linguagem) e ADR-002 (licença MIT-only do Núcleo).
- **FR-002**: O ADR-001 DEVE declarar GDScript como linguagem de engine e script de jogo adotada no ciclo.
- **FR-003**: O ADR-001 DEVE declarar explicitamente que a decisão Godot-first permanece vinculante e não é reaberta, revista ou revertida.
- **FR-004**: O ADR-001 DEVE registrar critérios de reavaliação da exceção, condicionando qualquer mudança de linguagem a evidência e aprovação formal, sem introduzir segunda linguagem neste ciclo.
- **FR-005**: O ADR-002 DEVE estabelecer a licença MIT como posição única para todo o código produzido no ciclo, incluindo o Núcleo e o servidor headless.
- **FR-006**: O ADR-002 DEVE excluir expressamente cláusula alternativa do tipo "MIT ou GPL" e o licenciamento AGPL para o código do ciclo.
- **FR-007**: O ADR-002 DEVE declarar que nenhum código sob licença copyleft é copiado e que contribuições de terceiros, se houver, são identificadas e atribuídas.
- **FR-008**: Os ADRs NÃO DEVEM alterar a licença do motor adotado nem atribuir a ele licença exclusiva.
- **FR-009**: Cada ADR DEVE conter, no mínimo: título, data, contexto, decisão, alternativas consideradas, consequências, autoridade de aprovação e rastreabilidade para os desvios de origem e para a decisão R1.
- **FR-010**: Os ADRs NÃO DEVEM incluir código, compilação, testes, gates de build, pipeline de integração contínua, serviço externo ou emenda da norma externa — o escopo é estritamente documental.
- **FR-011**: Os ADRs DEVEM registrar a aprovação humana como autoridade necessária e manter como `[NV]` o status remoto, a execução de integração contínua, o merge e a proteção de branch.
- **FR-012**: A rastreabilidade DEVE associar o ADR-001 aos desvios de linguagem, o ADR-002 ao desvio de licença e ambos à decisão R1, de modo que nenhum desvio previsto fique sem decisão local correspondente.
- **FR-013**: Os ADRs DEVEM registrar como fora do escopo do ciclo a decisão sobre o arquivo de licenças de terceiros, evitando resolução implícita dessa questão.

### Key Entities *(include if feature involves data)*

- **ADR-001 (exceção de linguagem)**: registro local da autorização de GDScript como engine/script de jogo; contém contexto, decisão, alternativas, consequências, cláusula de não reabertura do pilar, critérios de reavaliação, autoridade de aprovação e rastreabilidade.
- **ADR-002 (licença MIT-only do Núcleo)**: registro local da posição de licenciamento único para o código do ciclo; contém a exclusão de cláusula alternativa e de AGPL, a regra de não cópia de código copyleft, a ressalva sobre a licença do motor, a autoridade de aprovação e a rastreabilidade.
- **Registro de rastreabilidade**: associação explícita entre cada desvio de intenção, o ADR que o resolve e a decisão R1, permitindo verificar cobertura sem consultar fontes externas.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% (2 de 2) dos registros de decisão previstos existem no ciclo e contêm todos os campos mínimos definidos em FR-009.
- **SC-002**: 100% (3 de 3) dos desvios de linguagem, exceção sem registro e licença possuem decisão local correspondente; baseline 0 de 3.
- **SC-003**: Zero alterações na decisão Godot-first e em suas passagens vinculantes; baseline 0, meta 0, verificável por comparação textual.
- **SC-004**: 100% (2 de 2) dos ADRs contêm declaração explícita de não reabertura do pilar.
- **SC-005**: Posições conflitantes sobre a licença do Núcleo reduzem de 2 para 1: a decisão local MIT-only passa a existir e a pendência de mudança documental permanece registrada como dependente de aprovação.
- **SC-006**: 100% (2 de 2) dos ADRs registram a aprovação humana como autoridade e rotulam como `[NV]` todo status remoto.
- **SC-007**: Zero ocorrências de código, compilação, teste, gate de build, pipeline de integração contínua, serviço externo ou emenda da norma externa nos dois ADRs.
- **SC-008**: 100% (4 de 4) das referências exigidas — desvio de linguagem, desvio de exceção sem registro, desvio de licença e decisão R1 — aparecem no registro de rastreabilidade.

## Assumptions

- O repositório é documental: não há código, runtime, build ou gate executado neste ciclo, e nenhum critério de sucesso depende de execução.
- A decisão Godot-first é premissa intocável; nenhum ADR a reabre, revisa ou reverte.
- A autoridade de aprovação das decisões locais é o maintainer do projeto; a emenda da norma externa depende de autoridade superior e fica fora de escopo.
- A decisão R1 de 2026-11-19 é a origem autoritativa desta feature; as fontes consultadas não nomeiam o proponente original, e isso não bloqueia os registros.
- A licença MIT vigente é preservada e nenhum código sob licença copyleft entra no ciclo.
- Uma decisão local é suficiente para o ciclo; a eventual mudança documental da exigência anterior de licença para o servidor depende de aprovação explícita e não é pré-requisito da feature.
- A questão sobre o arquivo de licenças de terceiros fica fora do escopo do ciclo.
- Todo status remoto — aprovação registrada, integração contínua, merge e proteção de branch — permanece `[NV]` até confirmação ao vivo.

## Clarifications

### Session 2026-11-19

- Q: O que significa "MIT-only" para o código deste ciclo? → A: MIT para todo o código produzido no ciclo (Núcleo e servidor headless), sem alternativa "ou GPL" e sem AGPL; a licença MIT vigente é preservada e nenhum código copyleft é copiado.
- Q: A decisão de licenciamento altera a licença do motor adotado ou resolve a questão do arquivo de licenças de terceiros? → A: Não em nenhum dos dois casos: a ressalva de que o motor não é o único MIT permanece, e a questão do arquivo de terceiros fica fora do escopo do ciclo.
- Q: Uma decisão local basta para a exceção de linguagem, ou é exigida emenda da norma externa? → A: Neste ciclo basta o ADR-001 local com aprovação humana; a emenda normativa fica como contingência futura, fora de escopo e sem reabrir o pilar.
- Q: Quem é o autor ou proponente original da ideia, e isso afeta a validade dos registros? → A: Resolvido por premissa: as fontes consultadas (decisão R1, plano e avaliação de stack do repositório) não nomeiam o proponente original; a autoria é tratada como não identificada e irrelevante para a validade dos ADRs, não bloqueando os registros.

0 perguntas ao usuário; ambiguidades resolvidas por evidência local (decisão R1, licença vigente, plano e avaliação de stack do próprio repositório).
R3 aplicado uma pergunta por vez: cada esclarecimento foi formulado e resolvido isoladamente, sem perguntas simultâneas ao usuário; nenhum marcador `[NEEDS CLARIFICATION]` permaneceu em aberto.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.
>
> Esta feature é estritamente documental e não produz manifests de orçamento de contexto nem saídas com handles de contexto; nenhum artefato dessa natureza é exigido no ciclo 001.
