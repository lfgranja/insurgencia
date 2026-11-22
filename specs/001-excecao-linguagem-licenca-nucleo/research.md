# Phase 0 Research: Exceção de linguagem e licença MIT-only do Núcleo

**Feature**: `001-excecao-linguagem-licenca-nucleo` | **Created**: 2026-11-19 | **Fase**: F0 (Phase 0 — Outline & Research)

**Input**: `spec.md` (origem autoritativa), `.specify/assessments/excecao-linguagem-licenca-nucleo/{intake,research,problem,concept,decision}.md`, `.specify/memory/constitution.md`, `AGENTS.md`, `docs/governance/sdd-phase-contracts.md` e os documentos locais citados em cada decisão.

**Política de fontes**: somente fontes locais do alvo, com caminho e linha quando aplicável. Nenhuma URL foi aberta, nenhuma fonte externa foi consultada e nenhuma referência externa foi inventada. A allowlist de hosts não foi acionada (`local-only`). Toda afirmação sobre estado remoto, CI, merge, aprovação registrada ou proteção de branch permanece `[NV]`.

## Metodologia

1. Extrair da spec e do Technical Context as decisões de desenho necessárias — nenhum marcador de esclarecimento pendente (spec `## Clarifications`: quatro respostas em 2026-11-19).
2. Triangular cada decisão com pelo menos duas fontes locais independentes quando disponíveis (spec + R1; spec + `LICENSE`; constitution + `AGENTS.md`).
3. Registrar alternativas consideradas e por que foram rejeitadas, sem antecipar tarefas, código, build ou gates.
4. Registrar explicitamente os campos sem aplicação a um ciclo documental, com motivo.

Formato adotado em cada decisão: **Decisão**, **Rationale**, **Alternativas consideradas**, **Fontes locais**.

## Decisões de pesquisa

### RQ-1 — Forma e local dos registros de decisão

- **Decisão**: dois registros locais versionados em Markdown, materializados como `docs/adr/001-excecao-linguagem-gdscript.md` e `docs/adr/002-licenca-mit-nucleo.md`; nenhum registro é criado nesta fase de planejamento.
- **Rationale**: o problema é documental e o repositório não tem código; o appetite da opção A é `small`, com precedente de decisões locais já registradas. A forma Markdown é legível por auditor sem runtime e versionável pelo Git, atendendo à exigência de "registros locais versionados" sem introduzir ferramenta.
- **Alternativas consideradas**: (a) emenda da norma MDMV — rejeitada por exigir autoridade externa e estar fora de escopo (opção B de `concept.md`); (b) registro apenas em documento externo — rejeitado porque a spec exige registros no ciclo e rastreabilidade consultável sem fonte externa (FR-001, FR-012); (c) base de dados ou ferramenta de ADR — rejeitada por adicionar dependência e storage, vedados por FR-010 e SC-007.
- **Fontes locais**: spec FR-001, FR-010, FR-012, SC-007; `.specify/assessments/excecao-linguagem-licenca-nucleo/concept.md:9-14,30-32`; `.specify/assessments/excecao-linguagem-licenca-nucleo/research.md:12,18`; `AGENTS.md:3-5`.

### RQ-2 — Numeração dos ADRs e divergência com documento externo

- **Decisão**: prevalece a numeração local de R1 — `ADR-001` = exceção de linguagem e `ADR-002` = licença MIT-only do Núcleo. O documento externo `docs/AVALIACAO-STACK-MDMV.md` usa `ADR-002` para integração contínua e `ADR-006` para licenciamento; essa divergência é registrada como dado local, sem alterar o documento externo e sem criar um terceiro ADR neste ciclo.
- **Rationale**: R1 (`go`, opção A) e a spec definem explicitamente dois registros com esses IDs e escopos. A constituição estabelece a precedência da governança do alvo em divergência. A contradição de numeração é uma divergência documento↔decisão e, pela regra de verificação honesta, deve ser registrada, não escondida.
- **Alternativas consideradas**: (a) adotar a numeração externa (`ADR-006` para licença) — rejeitada por contrariar R1 e a spec; (b) incluir um ADR de CI como `ADR-002` local — rejeitada por FR-001 (exatamente dois registros), FR-010 e SC-002 (três desvios, nenhum deles de CI); (c) renomear o documento externo — rejeitada por ser emenda documental fora de escopo.
- **Fontes locais**: spec FR-001, FR-010, SC-002; `.specify/assessments/excecao-linguagem-licenca-nucleo/decision.md:21-25,29-37`; `docs/AVALIACAO-STACK-MDMV.md:390,396`; `.specify/memory/constitution.md:25`; `AGENTS.md:10-12`.

### RQ-3 — Conteúdo e limites da posição MIT-only

- **Decisão**: MIT para todo o código produzido no ciclo (Núcleo e servidor headless), sem cláusula alternativa "ou GPL" e sem AGPL para esse código; a licença MIT vigente é preservada, nenhum código copyleft é copiado, contribuições de terceiros são identificadas e atribuídas, a licença do motor não é alterada ou declarada exclusiva, e a questão do arquivo de licenças de terceiros fica fora do escopo.
- **Rationale**: é a resposta das clarificações da spec e de R1; não há servidor AGPL no ciclo, e a ressalva sobre o motor evita atribuição indevida de licença exclusiva. A pendência de mudança documental da exigência anterior permanece registrada como dependente de aprovação explícita.
- **Alternativas consideradas**: (a) "MIT ou GPL" — rejeitada por ambiguidade e risco copyleft (FR-006); (b) AGPL no servidor — rejeitada porque nenhum servidor sob essa licença existe no ciclo e FR-006 a exclui; (c) decidir o arquivo de licenças de terceiros neste ciclo — rejeitada por FR-013 (fora de escopo).
- **Fontes locais**: spec `## Clarifications` (sessão 2026-11-19), FR-005–FR-008, FR-013, SC-005; `LICENSE:1-3`; `docs/AVALIACAO-STACK-MDMV.md:381,396,410`; `docs/plans/2026-11-19-ciclo-80-20-omoba.md:12-13`.

### RQ-4 — Não reabertura de Godot-first e verificação

- **Decisão**: cada ADR contém declaração textual explícita de que a decisão Godot-first (`GDD:1265,1678`) permanece vinculante e não é reaberta, revista ou revertida; a conferência é feita por comparação textual dos registros, sem gate de checagem documental.
- **Rationale**: SC-003 exige zero alterações nas passagens vinculantes e SC-004 exige a declaração nos dois ADRs; a declaração escrita é a métrica qualitativa definida, e FR-010 veta gates na fase de planejamento, com R4 e R7 como portarias do ciclo completo. Uma proposta futura de segunda linguagem ou troca de engine é encaminhada a aprovação formal com evidência prévia (FR-004).
- **Alternativas consideradas**: (a) gate determinístico de varredura textual — rejeitado na fase de planejamento por FR-010 e SC-007; (b) registrar a não reabertura apenas na spec — rejeitado por SC-004 exigir os dois ADRs; (c) emendar o GDD para acomodar a exceção — rejeitado como fora de escopo e risco de reabertura involuntária.
- **Fontes locais**: spec FR-003, FR-004, FR-010, SC-003, SC-004, Edge Cases; `.specify/assessments/excecao-linguagem-licenca-nucleo/concept.md:14,32,36`; `docs/GDD - Insurgência.md:1265,1678`; `docs/AVALIACAO-STACK-MDMV.md:443`.

### RQ-5 — Autoridade de aprovação e honestidade de status

- **Decisão**: o maintainer do projeto é a autoridade de aprovação das decisões locais, e a aprovação humana aparece como necessária nos dois ADRs. Aprovação efetivamente registrada, execução de CI, merge e proteção de branch são mantidos como `[NV]`.
- **Rationale**: a evidência local aponta o maintainer como autoridade de decisões locais, enquanto a emenda normativa depende de autoridade superior e fica fora de escopo. A constituição impede afirmar verde sem confirmação ao vivo.
- **Alternativas consideradas**: (a) atribuir a aprovação ao Strategic HQ — rejeitada porque isso diz respeito à norma externa, não aos registros locais; (b) marcar aprovação como obtida — rejeitada por FR-011, SC-006 e constituição VI; (c) condicionar o sucesso a CI remota — rejeitada pelos Assumptions da spec.
- **Fontes locais**: spec FR-011, SC-006, Assumptions; `.specify/assessments/excecao-linguagem-licenca-nucleo/problem.md:14,39`; `.specify/assessments/excecao-linguagem-licenca-nucleo/research.md:35-36`; `.specify/memory/constitution.md:17`; `AGENTS.md:45`.

### RQ-6 — Rastreabilidade mínima e cobertura

- **Decisão**: quatro referências obrigatórias — `DEV-LING-01` (desvio de linguagem, avaliação #1), `DEV-EXC-01` (exceção sem registro, avaliação #3), `DEV-LIC-01` (desvio de licença, avaliação #10) e `DEC-R1` (decisão R1). Os dois primeiros e `DEC-R1` associam-se ao ADR-001; `DEV-LIC-01` e `DEC-R1` ao ADR-002.
- **Rationale**: FR-012 exige que nenhum desvio previsto fique sem decisão local correspondente; SC-002 mede 3/3 desvios e SC-008 mede 4/4 referências. A associação de ambos os ADRs a R1 torna a verificação possível sem fonte externa.
- **Alternativas consideradas**: (a) rastrear apenas R1 — rejeitado por perder a cobertura individual dos desvios; (b) incluir o desvio de CI (#4) — rejeitado por não pertencer aos três desvios da spec (SC-002) e por FR-001; (c) rastreabilidade implícita por prosa corrida — rejeitada por impedir contagem objetiva.
- **Fontes locais**: spec FR-009, FR-012, SC-002, SC-008, Key Entities; `.specify/assessments/excecao-linguagem-licenca-nucleo/decision.md:39`; `docs/AVALIACAO-STACK-MDMV.md:387,389,396`.

### RQ-7 — Versionamento, changelog e rollback de registros documentais

- **Decisão**: cada ADR inicia em `1.0.0`; mudanças que alteram a decisão, a cláusula de não reabertura ou o escopo exigem nova versão major, esclarecimentos sem mudança de decisão usam minor e correções textuais/atribuição usam patch. Toda entrada de changelog registra data, IDs FR de origem e, quando existirem, IDs `T###` da fase tasks. O rollback é documental: novo commit que revoga ou emenda o registro preservando o histórico, nunca force-push, nunca merge automático; rollback de ambiente remoto permanece `[NV]`.
- **Rationale**: a fase release exige SemVer, changelog com IDs e rollback; as normas de trabalho do alvo exigem commits convencionais em pt-BR, staging explícito e proíbem force-push. Aplicar essas regras ao formato do contrato evita inventar processo externo.
- **Alternativas consideradas**: (a) apagar e reescrever o ADR — rejeitado por destruir a evidência e conflitar com append-only de governança; (b) force-push de reversão — rejeitado por AGENTS.md; (c) versionar por datas — rejeitado por não ser SemVer exigido em R7.
- **Fontes locais**: `docs/governance/sdd-phase-contracts.md:27`; `AGENTS.md:37-45`; `.specify/memory/constitution.md:19-25`; contrato `contracts/adr-contract.md §4–§5` (deste ciclo).

### RQ-8 — Procedimento de verificação local

- **Decisão**: a verificação é feita por leitura dos artefatos e comparação textual, com resultados esperados e referências à spec/contrato; setup, build, teste e execução de gates são declarados "não aplicáveis" com motivo. Nenhum comando de gate é executado nesta fase.
- **Rationale**: os critérios de sucesso são contagens e declarações textuais (SC-001–SC-008); nenhum depende de execução, e FR-010/SC-007 vedam código, compilação, testes e gates no conteúdo dos ADRs. O comando determinístico `check_sdd_completeness.py` existe no alvo, mas sua execução pertence à fase validate (R4), portaria do ciclo completo posterior a esta fase de planejamento, assim como a fase release (R7).
- **Alternativas consideradas**: (a) incluir execução de gates no quickstart — rejeitada por antecipar a fase validate e por FR-010; (b) omitir o procedimento — rejeitada porque o contrato de fase exige `quickstart.md`; (c) substituir a leitura por testes automatizados — rejeitada por ausência de runtime.
- **Fontes locais**: spec US3 (Independent Test), FR-010, SC-003, SC-007, Assumptions; `AGENTS.md:14-26`; `docs/governance/sdd-phase-contracts.md:14,17`.

### RQ-9 — Divergência R1 ↔ spec sobre aprovação dos ADRs

- **Decisão**: R1 fala em "ADRs aprovados no ciclo", enquanto a spec veta aprovação verde. A reconciliação é: entregável = ADRs materializados e revisados localmente; a aprovação humana efetiva permanece `[NV]`. A métrica do gate `check_sdd_completeness PASS` é evidência local de completude documental, não afirmação de merge ou CI.
- **Rationale**: a constituição (VI) e SC-006 impedem afirmar aprovação, CI ou merge sem confirmação ao vivo; registrar a divergência preserva a verificação honesta sem bloquear a entrega documental, conforme C2 do contrato de remediação.
- **Alternativas consideradas**: (a) registrar os ADRs como aprovados — rejeitada por contrariar a spec e a constituição VI; (b) omitir a divergência — rejeitada por esconder contradição documento↔decisão; (c) condicionar o entregável a CI remota — rejeitada pelos Assumptions da spec.
- **Fontes locais**: spec FR-011, SC-006, Assumptions; `.specify/assessments/excecao-linguagem-licenca-nucleo/decision.md:5` (R1); `docs/governance/analyze-remediation-2026-11-19.md:8-11` (C2); `docs/governance/sdd-phase-contracts.md:14`.

## Evidência contrária e tratamento local

| Risco apontado pela evidência local | Tratamento neste desenho |
|---|---|
| Um ADR local pode ser insuficiente sem emenda da norma (saída B). | Registrado como contingência futura no ADR-001, sem execução neste ciclo e sem reabertura do pilar (RQ-1, RQ-4). |
| Fechar N3 como MIT-only contraria literalmente a exigência anterior de AGPL no servidor. | Nenhum servidor sob essa licença existe no ciclo; a mudança documental depende de aprovação explícita e fica pendente, não silenciosa (RQ-3). |
| Fechar a licença pode gerar nova divergência documento↔decisão. | A divergência de numeração e a pendência documental são registradas em research.md e no contrato (RQ-2, RQ-3). |
| Reabertura involuntária do GDD por ADRs de processo. | Declaração textual nos dois ADRs e verificação por comparação textual (RQ-4). |
| Ausência de runtime impede verificação empírica. | Verificação estritamente documental; nenhum resultado é afirmado como executado (RQ-8). |

## Fontes consultadas (somente locais)

| Fonte local | Uso |
|---|---|
| `specs/001-excecao-linguagem-licenca-nucleo/spec.md` | Requisitos, clarificações, critérios de sucesso, premissas. |
| `.specify/assessments/excecao-linguagem-licenca-nucleo/{intake,research,problem,concept,decision}.md` | Evidência de origem, opções, veredito R1 e mapeamento dos desvios. |
| `.specify/memory/constitution.md` | Princípios I–VI, precedência da governança local e verificação honesta. |
| `AGENTS.md` | Natureza documental do alvo, gates disponíveis, normas de commit e rollback. |
| `docs/governance/sdd-phase-contracts.md` | Ordem de portarias e requisitos das fases plan, checklist, validate e release. |
| `docs/AVALIACAO-STACK-MDMV.md:381,387,389,390,396,410,443` | Desvios #1/#3/#10, divergência de numeração e riscos registrados. |
| `docs/GDD - Insurgência.md:1265,1451,1659,1678` | Premissa Godot-first, posições documentais divergentes sobre licença. |
| `LICENSE:1-3` | Licença MIT vigente. |
| `docs/specs/2026-11-19-ciclo-80-20-omoba.md:5-6,40-43,55` e `docs/plans/2026-11-19-ciclo-80-20-omoba.md:12-13` | Godot-first operacional, MIT no Núcleo, ausência de execução. |

## Campos não aplicáveis a esta pesquisa (com motivo)

| Campo | Declaração | Motivo |
|---|---|---|
| Benchmarks externos de adoção ou mercado | Não aplicável | Nenhuma fonte externa foi consultada e o alvo não tem runtime para medir; registrar números seria inventar evidência. |
| URL, host ou política de allowlist acionada | Não aplicável | Nenhuma URL foi fornecida ou aberta; a política adotada foi `local-only`. |
| Resultado de gate, build, teste, lint ou compilação | Não aplicável | Nenhum comando foi executado nesta fase; status de execução pertence à fase validate (R4). |
| Proponente/nomeação externa da ideia | Não aplicável | As fontes locais nomeiam os ADRs e desvios, não o proponente; a spec registra que isso não bloqueia os registros. |
