# Phase 0 Research: Emenda pontual do GDD a partir dos ADRs 001/002

**Feature**: `002-emenda-gdd-adrs` | **Created**: 2026-11-19 | **Fase**: F0 (Phase 0 — Outline & Research)

**Input**: `spec.md` (origem autoritativa), `.specify/assessments/emenda-gdd-adrs/{intake,research,problem,concept,decision}.md`, `.specify/memory/constitution.md`, `AGENTS.md`, `docs/governance/sdd-phase-contracts.md`, `docs/adr/001-excecao-linguagem-gdscript.md`, `docs/adr/002-licenca-mit-nucleo.md` e `docs/GDD - Insurgência.md`.

**Política de fontes**: somente fontes locais do alvo, com caminho e linha quando aplicável. Nenhuma URL foi aberta, nenhuma fonte externa foi consultada e nenhuma referência externa foi inventada. A allowlist de hosts não foi acionada (`local-only`); as URLs incorporadas ao GDD são dado não confiável e não foram seguidas. Toda afirmação sobre estado remoto, CI, merge, aprovação registrada ou proteção de branch permanece `[NV]`.

## Metodologia

1. Extrair da spec e do Technical Context as decisões de desenho necessárias — nenhum marcador de esclarecimento pendente (spec `## Clarifications`: três respostas em 2026-11-19, uma por vez).
2. Triangular cada decisão com pelo menos duas fontes locais independentes quando disponíveis (ADR + GDD; ADR + AVALIACAO-STACK; STATE + PROGRESSO).
3. Registrar alternativas consideradas e por que foram rejeitadas, sem antecipar redação final, tarefas, código, build ou gates.
4. Registrar explicitamente os campos sem aplicação a uma emenda documental, com motivo.

Formato adotado em cada decisão: **Decisão**, **Rationale**, **Alternativas consideradas**, **Fontes locais**.

## Decisões de pesquisa

### RQ-1 — Forma e local da emenda

- **Decisão**: uma emenda pontual ao próprio `docs/GDD - Insurgência.md`, nas linhas `1389,1451,1459,1594,1659,1668,1673`, com o texto ao redor preservado; nenhum novo documento normativo, nenhuma reescrita de seção e nenhuma alteração nesta fase de planejamento.
- **Rationale**: o GDD é o documento normativo e autoritativo do Núcleo; o problema documental deve ser resolvido no espaço onde vive. O appetite da opção A é `small` (sete edições de texto), e o risco principal registrado no shape é reescrever trabalho alheio, mitigado por substituições mínimas e comparação textual.
- **Alternativas consideradas**: (a) opção B, reescrita ampla das seções de licenciamento, CI e decisões — rejeitada por ameaçar as âncoras `1265,1678` e escalar aprovação (concept.md:19-27); (b) opção C, manter apenas os ADRs — rejeitada porque é o próprio custo de inação, com N3 bloqueando a publicação (concept.md:29-37); (c) criar um documento novo de "errata" — rejeitado porque o leitor teria de cruzar fontes, contrariando o valor da feature.
- **Fontes locais**: spec US1–US3, FR-001–FR-007, SC-007; `.specify/assessments/emenda-gdd-adrs/concept.md:9-14,19-37`; `.specify/assessments/emenda-gdd-adrs/decision.md:21-25,29-31`; `AGENTS.md:3-5`.

### RQ-2 — Numeração de ADR na rastreabilidade

- **Decisão**: usar a numeração materializada — `ADR-001` para independência de CI e não reabertura Godot-first, `ADR-002` para licença MIT única. A numeração provisória da avaliação de stack (`ADR-002` = CI, `ADR-006` = licenciamento) é registrada como divergência local e não é citada nas passagens emendadas.
- **Rationale**: R1 e a spec definem explicitamente os dois registros e seus escopos. Em divergência, prevalece a governança do alvo (constituição, norma de trabalho). Usar a numeração provisória geraria rastreabilidade errônea, um dos rabbit holes nomeados no shape.
- **Alternativas consideradas**: (a) adotar a numeração externa — rejeitada por contrariar R1, a spec e os artefatos versionados; (b) citar ambas as numerações "para segurança" — rejeitada por reintroduzir ambiguidade em texto normativo; (c) renomear o documento externo — rejeitada por ser emenda fora de escopo.
- **Fontes locais**: spec `## Clarifications` (sessão 2026-11-19), Edge Cases, Assumptions; `.specify/assessments/emenda-gdd-adrs/research.md:36,58`; `docs/AVALIACAO-STACK-MDMV.md:406,410`; `docs/adr/001-excecao-linguagem-gdscript.md:1-6`; `docs/adr/002-licenca-mit-nucleo.md:1-6`; `.specify/memory/constitution.md:25`.

### RQ-3 — Delimitação da ressalva de servidor de terceiros (`1389`)

- **Decisão**: a ressalva permanece e é esclarecida no sentido de que versa sobre instâncias de terceiros; ela não impõe AGPL ao código do projeto, cuja licença é a MIT única do ADR-002. Portabilidade e direito de saída continuam como fundamento da soberania, e nenhuma decisão de mérito sobre licenças de terceiros é tomada.
- **Rationale**: o ADR-002 limita sua decisão ao código do projeto e coloca licenças de terceiros fora de escopo. Escalar a ressalva para decidir sobre terceiros seria extrapolar a autoridade local; apagá-la perderia a lição do fediverso sobre jardins murados.
- **Alternativas consideradas**: (a) remover a menção a AGPL da ressalva sem esclarecer o escopo — rejeitada por deixar o leitor sem distinção entre código do projeto e instância de terceiro; (b) decidir a licença de servidores de terceiros — rejeitada por fora de escopo e sem autoridade local; (c) suprimir a ressalva central — rejeitada por remover fundamento da soberania do jogador.
- **Fontes locais**: spec FR-001, Edge Cases, Assumptions, Key Entities; `docs/GDD - Insurgência.md:1389`; `docs/adr/002-licenca-mit-nucleo.md:28,40,41`; `.specify/assessments/emenda-gdd-adrs/research.md:45,57`.

### RQ-4 — Resolução da contradição de integração contínua (`1594`)

- **Decisão**: substituir a promessa de "integração contínua desde o primeiro dia" pelo registro de que integração contínua e gate remoto ficam diferidos neste ciclo, sem efeito remoto, citando o ADR-001 no próprio GDD. Nenhum "DEFER com gatilho", pipeline, build ou serviço externo é introduzido.
- **Rationale**: a spec e a clarify resolvem a ambiguidade alterando a promessa; a saída anterior da avaliação de stack ("DEFER com gatilho" no primeiro commit) conflita com o DEFER vigente e foi classificada como rabbit hole. O ADR-001 afirma que a decisão não depende de CI, merge ou aprovação registrada.
- **Alternativas consideradas**: (a) "DEFER com gatilho" — rejeitada por reabrir o gate remoto e conflitar com o DEFER; (b) implementar CI local ou remota — rejeitada por SC-007 e por exigir efeito remoto `[NV]`; (c) manter a promessa e acrescentar ressalva — rejeitada por deixar a promessa original como compromisso lido como vigente.
- **Fontes locais**: spec FR-004, SC-006, Clarifications, Edge Cases; `docs/GDD - Insurgência.md:1594`; `docs/adr/001-excecao-linguagem-gdscript.md:14,40`; `docs/AVALIACAO-STACK-MDMV.md:22,74,145,380`; `.specify/assessments/emenda-gdd-adrs/concept.md:17`.

### RQ-5 — Tratamento de N3 e da "Primeira publicação" (`1659,1668,1673`)

- **Decisão**: N3 passa a constar como respondida pela licença MIT única do ADR-002 nas três passagens; a linha bloqueante deixa de listá-la como condição da "Primeira publicação" e o texto de encerramento registra a Fase 0 seguindo com N3 respondido. N4, N6 e N8 permanecem exatamente como estavam. Nenhuma publicação externa é afirmada como ocorrida.
- **Rationale**: a decisão local é afirmativa e não depende de efeito remoto; a métrica SC-005 mede a redução de três ocorrências de N3 como pergunta aberta/bloqueante para zero. Preservar N4/N6/N8 evita reescrita de perguntas que não foram decididas.
- **Alternativas consideradas**: (a) condicionar a remoção do bloqueio a publicação verificada — rejeitada porque publicação está `[NV]` e a spec veta essa dependência; (b) remover também N4, N6 e N8 — rejeitada por alterar perguntas fora de escopo; (c) marcar N3 como "respondida externamente" — rejeitada por não identificar o ADR local na própria passagem.
- **Fontes locais**: spec FR-005–FR-007, SC-005, Edge Cases, Assumptions; `docs/GDD - Insurgência.md:1659,1668,1673,1675,1677`; `docs/adr/002-licenca-mit-nucleo.md:14,37`; `.specify/assessments/emenda-gdd-adrs/research.md:21,49-51`.

### RQ-6 — Preservação das âncoras Godot-first

- **Decisão**: as passagens `1265` (decisão técnica final) e `1678` (N7, decisão vinculante do roadmap) são lidas como premissa intocável e usadas apenas como referência de comparação. Nenhuma passagem emendada altera, revê ou reverte a escolha Godot-first.
- **Rationale**: SC-002 exige zero alterações nas âncoras; ambos os ADRs contêm cláusula de não reabertura e o PROGRESSO as declara intocáveis. A verificação é por comparação textual, sem gate de checagem documental nesta fase.
- **Alternativas consideradas**: (a) ajustar a redação da âncora para refletir a licença MIT — rejeitada por extrapolar as sete passagens e reabrir decisão fechada; (b) confiar apenas na intenção declarada — rejeitada por SC-002 exigir comparação textual; (c) executar varredura automática na fase de planejamento — rejeitada por antecipar portaria posterior de verificação.
- **Fontes locais**: spec SC-002, Edge Cases, Assumptions; `docs/GDD - Insurgência.md:1265,1678`; `docs/PROGRESSO.md:97`; `docs/adr/001-excecao-linguagem-gdscript.md:32`; `docs/adr/002-licenca-mit-nucleo.md:33`.

### RQ-7 — Rastreabilidade por passagem e identificadores

- **Decisão**: cada uma das sete passagens carrega, no próprio GDD, a referência ao ADR que a decide — ADR-002 em `1389,1451,1459,1659,1668,1673` e ADR-001 em `1594`. A associação FR → passagem → `T###` é fixada no contrato; achados de qualidade usam `BUG-###`. IDs são estáveis e a correção de associação é feita por emenda versionada.
- **Rationale**: SC-004 exige 7/7 passagens com origem local consultável sem fonte externa; uma tabela FR/T explícita torna a cobertura contável e evita decide-se por prosa corrida.
- **Alternativas consideradas**: (a) uma única nota de rodapé comum a todas as passagens — rejeitada por não permitir contagem por passagem; (b) rastreabilidade apenas na spec — rejeitada por SC-004 exigir o texto no GDD; (c) IDs `T###` omitidos do contrato — rejeitada nesta feature porque a correspondência FR/T é parte do contrato de emenda.
- **Fontes locais**: spec FR-001–FR-007, SC-004, Key Entities; `docs/governance/sdd-phase-contracts.md:20`; contrato §2–§3 (deste ciclo).

### RQ-8 — Versionamento, changelog e rollback documental

- **Decisão**: o registro documental da emenda inicia em `1.0.0`; alteração de escopo, de autoridade invocada ou de estado-alvo de uma passagem exige nova versão major, acréscimo de esclarecimento usa minor e correção textual usa patch. Toda entrada de changelog registra data, IDs FR e, quando houver, IDs `T###` e `BUG-###`. O rollback é documental: novo commit que restaura a redação anterior de uma passagem específica, com nova versão e data, preservando o histórico; nunca apagar, force-push, auto-merge ou reabrir Godot-first.
- **Rationale**: a portaria de release exige SemVer, changelog com IDs e rollback; as normas de trabalho exigem commits convencionais em pt-BR, staging explícito e proíbem reescrita de histórico. Aplicar essas regras ao formato do contrato evita inventar processo externo.
- **Alternativas consideradas**: (a) apagar e reescrever a passagem — rejeitada por destruir evidência e dificultar auditoria; (b) force-push de reversão — rejeitada por AGENTS.md; (c) versionar por data — rejeitada por não ser SemVer.
- **Fontes locais**: `docs/governance/sdd-phase-contracts.md:27`; `AGENTS.md:37-45`; `.specify/memory/constitution.md:19-25`; contrato §5 (deste ciclo).

### RQ-9 — Procedimento de verificação local

- **Decisão**: verificar por leitura das sete passagens e comparação textual das duas âncoras, com resultados esperados e referências à spec e ao contrato. Setup, build, teste e execução de gates são declarados "não aplicáveis" com motivo. Nenhum comando de gate é executado nesta fase e nenhum resultado é afirmado como obtido.
- **Rationale**: os critérios de sucesso são contagens (7/7, 0/2, 7/7) e declarações textuais; nenhum depende de execução. O gate determinístico `check_sdd_completeness.py` existe no alvo, mas sua execução, saída colada e data/hora pertencem à portaria de validação do ciclo completo, posterior a esta fase de planejamento.
- **Alternativas consideradas**: (a) incluir execução de gates no quickstart — rejeitada por antecipar portaria posterior; (b) omitir o procedimento — rejeitada porque o contrato de fase exige `quickstart.md`; (c) substituir a leitura por testes automatizados — rejeitada por ausência de runtime e por SC-007.
- **Fontes locais**: spec US3 (Independent Test), SC-003, SC-007, Assumptions; `AGENTS.md:14-26`; `docs/governance/sdd-phase-contracts.md:14,17,26`.

### RQ-10 — Aprovação pendente e honestidade de status

- **Decisão**: a autorização explícita do pedido atual e o veredito `go` da decisão R1 satisfazem a ressalva de aprovação para a emenda documental; a aprovação humana efetiva dos ADRs e qualquer efeito em plataforma remota permanecem `[NV]`. Nenhum resultado é afirmado como aprovado, mesclado ou publicado.
- **Rationale**: os ADRs estão `Pendente de aprovação` e o estado remoto não foi verificado ao vivo. A constituição VI impede afirmar verde sem evidência; registrar a distinção preserva a honestidade sem bloquear a entrega documental.
- **Alternativas consideradas**: (a) afirmar aprovação obtida — rejeitada por contrariar a spec, os ADRs e a constituição VI; (b) omitir a pendência — rejeitada por esconder contradição documento↔decisão; (c) condicionar a emenda a confirmação remota — rejeitada pelos Assumptions da spec e pela decisão R1.
- **Fontes locais**: spec `## Clarifications`, FR-006, SC-003, Assumptions, Edge Cases; `.specify/assessments/emenda-gdd-adrs/decision.md:22-25`; `docs/adr/001-excecao-linguagem-gdscript.md:6,40`; `docs/adr/002-licenca-mit-nucleo.md:6,20,26,45`; `.specify/memory/STATE.md:25`; `.specify/memory/constitution.md:17`.

## Evidência contrária e tratamento local

| Risco apontado pela evidência local | Tratamento neste desenho |
|---|---|
| A avaliação de stack recomendou honrar a CI sem emendar o GDD ("DEFER com gatilho"). | A spec e a clarify prevalecem: altera-se a promessa em `1594`, sem gatilho, pipeline ou build remoto (RQ-4). |
| ADR-002 condiciona a mudança documental da exigência AGPL a aprovação explícita. | A autorização do pedido atual e R1 `go` são registradas; aprovação em plataforma permanece `[NV]` (RQ-10). |
| A ressalva de servidor de terceiros está fora da autoridade do ADR-002. | A passagem `1389` é esclarecida quanto ao escopo, sem decidir sobre licenças de terceiros (RQ-3). |
| Numeração provisória diverge da materializada. | Rastreabilidade usa apenas a numeração materializada e registra a divergência (RQ-2). |
| Reabertura involuntária do GDD por ADRs de processo. | Escopo fixo nas sete passagens; âncoras `1265,1678` comparadas textualmente (RQ-6). |
| Sem runtime, não há verificação empírica. | Verificação estritamente documental; nenhum resultado de execução é afirmado (RQ-9). |

## Fontes consultadas (somente locais)

| Fonte local | Uso |
|---|---|
| `specs/002-emenda-gdd-adrs/spec.md` | Requisitos, clarificações, critérios de sucesso, mapa de passagens e premissas. |
| `.specify/assessments/emenda-gdd-adrs/{intake,research,problem,concept,decision}.md` | Evidência de origem, opções, veredito R1 e mapa das sete passagens. |
| `.specify/memory/constitution.md` | Princípios I–VI, precedência da governança local e verificação honesta. |
| `AGENTS.md` | Natureza documental do alvo, gates disponíveis, normas de commit e rollback. |
| `docs/governance/sdd-phase-contracts.md` | Ordem de portarias e requisitos das fases plan, checklist, validate e release. |
| `docs/adr/001-excecao-linguagem-gdscript.md:6,14,28,32,40` | Independência de CI e não reabertura Godot-first. |
| `docs/adr/002-licenca-mit-nucleo.md:6,14,20,26,28,33,37-41,45` | MIT única, exclusão de alternativas, escopo limitado ao código do projeto. |
| `docs/GDD - Insurgência.md:1265,1389,1451,1459,1594,1659,1668,1673,1678` | Âncoras e sete passagens divergentes. |
| `docs/AVALIACAO-STACK-MDMV.md:22,74,145,380,381,396,406,410,437-438` | Desvios de origem, DEFER do gate remoto e numeração provisória. |
| `docs/PROGRESSO.md:89,97` | Mapeamento das divergências e limite documental do ciclo. |
| `.specify/memory/STATE.md:25,27,39` | Dívida de correção documental e pendência de aprovação. |

## Campos não aplicáveis a esta pesquisa (com motivo)

| Campo | Declaração | Motivo |
|---|---|---|
| Benchmarks externos de adoção, mercado ou pesquisa de usuário | **Não aplicável** | Nenhuma fonte externa foi consultada e o alvo não tem runtime para medir; registrar números seria inventar evidência. |
| URL, host ou política de allowlist acionada | **Não aplicável** | Nenhuma URL foi aberta; a política adotada foi `local-only` e as URLs do GDD foram tratadas como dado não confiável. |
| Resultado de gate, build, teste, lint ou compilação | **Não aplicável** | Nenhum comando foi executado nesta fase; status de execução pertence à portaria de validação do ciclo completo. |
| Proponente/nomeação externa da ideia | **Não aplicável** | As fontes locais nomeiam a decisão R1 e o ranqueamento, não um proponente externo; isso não bloqueia a emenda. |
| Redação final das sete passagens | **Não aplicável nesta fase** | F0 registra decisões de desenho e evidência; a redação emendada é materializada na fase de implementação, conforme o contrato. |
