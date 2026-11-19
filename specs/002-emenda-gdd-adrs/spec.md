# Feature Specification: Emenda pontual do GDD a partir dos ADRs 001/002

**Feature Branch**: `002-emenda-gdd-adrs`

**Created**: 2026-11-19

**Status**: Draft

**Input**: Decisão R1 de 2026-11-19 (veredito `go`, opção A) — emenda pontual e auditável de sete passagens do documento normativo do Núcleo, usando como autoridade local os registros ADR-001 (independência de CI e não reabertura Godot-first) e ADR-002 (licença MIT única, sem AGPL e sem cláusula alternativa), sem reabrir decisões fechadas.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Fechar N3 e a licença única no GDD (Priority: P1)

O mantenedor do Núcleo precisa que o documento normativo deixe de tratar a licença como pergunta aberta. Hoje seis passagens sustentam a alternativa "MIT ou GPL", a exigência AGPL-3.0 para o servidor, os artefatos `LICENSE`/`COPYING` e N3 como bloqueante, enquanto a autoridade local já decidiu MIT único para todo o código do ciclo. A emenda faz o leitor determinar a licença vigente a partir do próprio GDD, sem precisar cruzar fontes.

**Why this priority**: É o risco de maior severidade — a convivência entre a licença MIT vigente e a exigência AGPL gera divergência entre decisão e documento e risco jurídico para forks e operadores de servidores, além de manter a "Primeira publicação" bloqueada.

**Independent Test**: Basta ler as seis passagens de licenciamento do GDD para verificar que N3 não aparece mais como pergunta aberta ou bloqueante, que a alternativa "MIT ou GPL" e a AGPL para o código do projeto desapareceram e que cada passagem alterada cita o ADR-002; nenhuma outra passagem é necessária para entregar esse valor.

**Acceptance Scenarios**:

1. **Given** que o GDD ainda formula N3 como "MIT ou GPL?" e exige AGPL-3.0 para o servidor, **When** o leitor consulta as passagens de licenciamento após a emenda, **Then** encontra a licença MIT única para todo o código do ciclo, sem alternativa "ou GPL" e sem AGPL, com a referência ao ADR-002 no próprio GDD.
2. **Given** que N3 é hoje listado como pergunta não respondida e bloqueante da "Primeira publicação", **When** o leitor verifica a seção de perguntas abertas, **Then** N3 consta como decidida pelo ADR-002, não bloqueia a publicação e N4, N6 e N8 permanecem como estavam.
3. **Given** a ressalva sobre servidor AGPL de terceiro e a ressalva de que licença e contrato de contribuição não protegem o jogador de instância injusta, **When** essas passagens são lidas, **Then** elas deixam claro que a AGPL citada é de servidores de terceiros e não impõe licença ao código do projeto, preservando portabilidade e direito de saída.

---

### User Story 2 — Reconciliar a promessa de integração contínua com o DEFER (Priority: P1)

O mantenedor precisa que a tabela de riscos do GDD deixe de prometer "integração contínua desde o primeiro dia", promessa que contradiz a decisão local de que nenhum efeito remoto, CI, merge ou aprovação é pré-requisito, e o DEFER do gate remoto já registrado. A emenda registra a integração contínua como diferida neste ciclo, sem alterar o DEFER nem introduzir pipeline.

**Why this priority**: É o segundo risco ALTA registrado — a promessa não honrada continua sendo lida como compromisso e desautoriza a porta única do gate local.

**Independent Test**: Basta ler a passagem de sustentabilidade financeira após a emenda para confirmar que a promessa de CI dia 1 foi substituída pelo registro do DEFER, sem qualquer efeito remoto; isso entrega o valor independentemente das passagens de licenciamento.

**Acceptance Scenarios**:

1. **Given** que a mitigação de sustentabilidade promete "integração contínua desde o primeiro dia", **When** o leitor consulta essa passagem após a emenda, **Then** a promessa não aparece mais e o texto registra a integração contínua e o gate remoto como diferidos neste ciclo, com referência ao ADR-001.
2. **Given** a orientação local que sugeria converter o DEFER em "DEFER com gatilho" no primeiro commit de código, **When** a emenda é avaliada, **Then** essa reabertura não é adotada: nenhum gatilho, pipeline, build ou serviço externo é introduzido no GDD.

---

### User Story 3 — Auditar a emenda, as âncoras e a rastreabilidade (Priority: P2)

O mantenedor e a comunidade precisam conferir que as sete passagens divergentes foram todas corrigidas, que as âncoras da decisão Godot-first permaneceram intactas e que cada alteração identifica o ADR que a autoriza — sem transformar o ciclo documental em entrega de código ou efeito remoto.

**Why this priority**: Sem essa verificação por leitura, a emenda vira intenção não comprovada e corre o risco de escorrer para as âncoras intocáveis ou de reescrever trabalho alheio.

**Independent Test**: A verificação é feita apenas pela leitura cruzada das sete passagens e das duas âncoras: contagem de passagens emendadas, comparação textual das âncoras e presença das referências aos ADRs, sem executar nenhum artefato.

**Acceptance Scenarios**:

1. **Given** o mapa de sete passagens e as duas âncoras, **When** um auditor verifica a cobertura, **Then** encontra 7 de 7 passagens reconciliadas com a autoridade local e 0 alterações nas âncoras da decisão Godot-first.
2. **Given** uma passagem alterada qualquer, **When** o auditor procura sua origem, **Then** encontra no próprio GDD a referência ao ADR que a decide, sem depender de fonte externa.
3. **Given** status remoto `[NV]`, **When** o auditor lê o resultado da emenda, **Then** nenhuma aprovação em plataforma, execução de CI, merge ou proteção de branch é afirmada como obtida, e nenhum código, gate de build, pipeline ou serviço externo aparece como resultado da feature.

---

### Edge Cases

- Se a passagem sobre servidor AGPL de terceiros for lida como decisão sobre o código do projeto, a emenda DEVE esclarecer que ela versa sobre instâncias de terceiros: a autoridade do ADR-002 não cobre licenças de terceiros e não as decide implicitamente.
- Se a numeração provisória da avaliação aparecer na documentação (ADR-002 para CI e ADR-006 para licenciamento), a rastreabilidade DEVE usar a numeração materializada — ADR-001 para independência de CI e ADR-002 para licença MIT única —, sem citar a provisória.
- Se a sugestão de "DEFER com gatilho" ressurgir, ela DEVE ser reconhecida como fora de escopo: a contradição resolve-se alterando a promessa no GDD, não reativando o gate remoto.
- Se a aprovação humana dos ADRs ainda estiver registrada como pendente, a emenda documental DEVE apoiar-se na autorização explícita do pedido atual e na decisão R1 `go`, sem afirmar aprovação final remota, que permanece `[NV]`.
- Se N3 deixar de bloquear a "Primeira publicação", isso NÃO DEVE depender de publicação externa já verificada; publicação e efeitos remotos permanecem `[NV]`.
- Se a emenda ameaçar as passagens da decisão Godot-first, ela DEVE ser contida nas sete passagens mapeadas; as duas âncoras permanecem intocáveis e nenhuma seção vizinha é reescrita.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O GDD DEVE manter a ressalva de que a licença de um servidor de terceiros não garante justiça de instância, deixar explícito que ela não impõe AGPL ao código do projeto — cuja licença é a MIT única do ADR-002 — e preservar portabilidade e direito de saída como fundamento da soberania, citando o ADR-002 no próprio GDD.
- **FR-002**: A linha da tabela do MVP no GDD DEVE declarar licença MIT única para todo o código do ciclo, incluindo Núcleo e servidor, sem AGPL-3.0 e sem o artefato `COPYING`, mantendo a declaração/licença MIT e citando o ADR-002 como autoridade no próprio GDD.
- **FR-003**: A ressalva do MVP no GDD DEVE substituir o par "MIT/AGPL" pela licença MIT única, preservando a afirmação de que licença e contrato de contribuição protegem o código e a autoria, mas não o jogador de instância injusta, com a referência ao ADR-002 no próprio GDD.
- **FR-004**: A passagem de sustentabilidade financeira no GDD DEVE deixar de prometer "integração contínua desde o primeiro dia" e DEVE registrar a integração contínua e o gate remoto como diferidos neste ciclo, sem efeito remoto, em coerência com o ADR-001 e com o DEFER vigente, citando o ADR-001 no próprio GDD.
- **FR-005**: A nota N3 no GDD DEVE constar como respondida pela licença MIT única do ADR-002, sem a alternativa "MIT ou GPL" e sem reabertura da licença do motor, preservando a ressalva de que o motor adotado não é o único motor MIT, com a referência ao ADR-002 no próprio GDD.
- **FR-006**: A tabela de perguntas abertas bloqueantes no GDD DEVE deixar de listar N3 como bloqueante da "Primeira publicação", registrando-a como decidida pelo ADR-002, sem condicionar a emenda a publicação externa, com a referência ao ADR-002 no próprio GDD.
- **FR-007**: O texto que lista perguntas não respondidas no GDD DEVE remover N3 da lista, preservando N4, N6 e N8, e DEVE registrar que a Fase 0 segue com N3 respondido, sem alterar as âncoras da decisão Godot-first e com a referência ao ADR-002 no próprio GDD.

### Key Entities *(include if feature involves data)*

- **GDD**: documento normativo e autoritativo do Núcleo, único artefato normativo de origem alterado pela feature; os artefatos de planejamento (contrato, modelo de dados, quickstart) são apenas conferidos por leitura, sem alteração; permanece como fonte de leitura para mantenedores, operadores de forks/servidores e governança, com o texto ao redor das passagens emendadas intacto.
- **ADR-001**: registro local que autoriza a exceção de linguagem, afirma que a decisão não depende de efeito remoto, CI, merge ou aprovação já registrada e veda a reabertura da decisão Godot-first; é a autoridade invocada pela passagem de integração contínua.
- **ADR-002**: registro local que fixa MIT como posição única para todo o código do ciclo, exclui a alternativa "MIT ou GPL" e a AGPL, preserva a licença MIT vigente e coloca licenças de terceiros fora de escopo; é a autoridade invocada pelas seis passagens de licenciamento.
- **Mapa de 7 passagens**: inventário rastreável, uma entrada por requisito funcional, com localização no GDD, conteúdo divergente verificado, ADR que decide e estado-alvo após a emenda:

  | IDs | Localização no GDD (seção; âncora de leitura) | Conteúdo divergente verificado | Autoridade | Estado-alvo |
  |---|---|---|---|---|
  | FR-001 | Ressalva central do fediverso; 1389 | "Servidor pode ser AGPL"; soberania depende de portabilidade, não da licença do servidor | ADR-002 (escopo limitado ao código do projeto) | Ressalva restrita a servidores de terceiros; sem AGPL imposta ao projeto |
  | FR-002 | Tabela do MVP; 1451 | "Licença MIT ... para o Núcleo; AGPL-3.0 para o servidor"; `LICENSE` e `COPYING` | ADR-002 | MIT único para Núcleo e servidor; sem `COPYING` |
  | FR-003 | Ressalva do MVP; 1459 | "Licença MIT/AGPL e contrato de contribuição protegem o código e a autoria" | ADR-002 | Par "MIT/AGPL" substituído por MIT único |
  | FR-004 | Tabela de riscos de sustentabilidade; 1594 | "integração contínua desde o primeiro dia" | ADR-001 + DEFER vigente | Promessa substituída por registro do DEFER, sem efeito remoto |
  | FR-005 | Notas para o designer; 1659 | "N3 — Licença do Núcleo: MIT ou GPL?" | ADR-002 | N3 respondida por MIT único, sem alternativa |
  | FR-006 | Perguntas abertas bloqueantes; 1668 | N3 bloqueia "Publicação do repositório e do catálogo de módulos" | ADR-002 | N3 removida dos bloqueantes, sem dependência externa |
  | FR-007 | Texto de encerramento das perguntas abertas; 1673 | "Enquanto N3, N4, N6 e N8 não são respondidos ..." | ADR-002 | N3 removida da lista; N4, N6 e N8 preservados |

- **Âncoras Godot-first**: passagens da decisão técnica final e da decisão vinculante do roadmap; são premissa intocável, usada apenas como referência de comparação na auditoria, nunca como alvo de emenda.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% (7 de 7) das passagens do mapa deixam de contradizer os ADRs 001/002; baseline 7 divergências, meta 0, verificável por leitura das sete passagens.
- **SC-002**: 0 alterações nas duas âncoras da decisão Godot-first; baseline 0, meta 0, verificável por comparação textual.
- **SC-003**: 100% das verificações documentais executadas neste ciclo retornam resultado observável; os gates determinísticos locais pertencem à portaria R4 (validate), não a tarefas desta feature; nenhum critério depende de status remoto, que permanece `[NV]` até confirmação ao vivo.
- **SC-004**: 100% (7 de 7) das passagens alteradas identificam no próprio GDD o ADR que as decide; baseline 0 (rastreabilidade só fora do GDD), meta 7.
- **SC-005**: N3 deixa de constar como pergunta aberta ou bloqueante no GDD; baseline 3 ocorrências, meta 0.
- **SC-006**: A promessa de "integração contínua desde o primeiro dia" sem reconciliação com o DEFER reduz de 1 para 0; nenhum gatilho, pipeline ou build remoto é introduzido.
- **SC-007**: Zero ocorrências de código, compilação, teste, gate de build, pipeline de integração contínua, serviço externo ou emenda da norma externa no resultado da feature; baseline 0, meta 0.

## Assumptions

- O repositório é documental: a emenda é estritamente de texto e nenhum critério de sucesso depende de runtime, build ou execução de código neste ciclo.
- A decisão Godot-first e suas duas âncoras são premissa intocável; nenhuma passagem a reabre, revisa ou reverte.
- A autorização explícita do usuário neste pedido, registrada na decisão R1 `go`, satisfaz a ressalva de aprovação para a emenda documental; a aprovação final em plataforma remota permanece `[NV]`.
- A rastreabilidade usa a numeração materializada dos ADRs (ADR-001 = independência de CI e não reabertura do pilar; ADR-002 = licença MIT única), não a numeração provisória da avaliação de stack.
- A cobertura é exatamente das sete passagens mapeadas, incluindo a ressalva de servidor de terceiros e a ressalva do MVP; nenhuma seção vizinha é reescrita.
- A contradição de CI resolve-se alterando a promessa no GDD, não registrando "DEFER com gatilho", que conflita com o DEFER vigente.
- N3 deixar de bloquear a "Primeira publicação" não depende de publicação externa; publicação, aprovação em plataforma, CI, merge e proteção de branch permanecem `[NV]`.
- Licenças de terceiros ficam fora da autoridade do ADR-002: a emenda esclarece o escopo da ressalva, sem decidir sobre essas licenças.
- O texto ao redor das sete passagens é preservado, de modo que a emenda seja pontual, auditável e revertível por comparação textual.

## Clarifications

### Session 2026-11-19

- Q: A emenda depende da aprovação humana dos ADRs, hoje registrada como pendente? → A: Não depende para esta emenda documental: a autorização explícita do usuário neste pedido e o veredito `go` da decisão R1 autorizam a correção; a aprovação final em plataforma remota permanece `[NV]`.
- Q: Qual numeração de ADR deve orientar a rastreabilidade da emenda? → A: A materializada no repositório — ADR-001 para independência de CI e não reabertura Godot-first, ADR-002 para licença MIT única; a numeração provisória da avaliação de stack não é usada.
- Q: A contradição de CI se resolve com "DEFER com gatilho" ou alterando a promessa no GDD? → A: Alterando a promessa no GDD, registrando CI e gate remoto como diferidos neste ciclo e sem efeito remoto, em coerência com o ADR-001 e o DEFER vigente; nenhum gatilho ou pipeline é reintroduzido.

0 perguntas ao usuário; as três ambiguidades foram resolvidas por evidência local (decisão R1, ADR-001, ADR-002, avaliação de stack e estado do repositório), uma por vez. Nenhum marcador `[NEEDS CLARIFICATION]` permaneceu em aberto e R3 foi respeitado (≤3 esclarecimentos, formulados e respondidos isoladamente).

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.
>
> Esta feature é estritamente documental e altera apenas texto normativo já versionado; não produz manifests de orçamento de contexto nem saídas com handles de contexto. Nenhum comando, caminho de implementação ou efeito remoto faz parte da spec; a verificação é feita por leitura das sete passagens e das duas âncoras, com resultado observável. Gates determinísticos locais pertencem à portaria R4 (validate), não a esta feature.
