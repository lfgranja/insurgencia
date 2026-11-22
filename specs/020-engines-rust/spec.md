# Feature Specification: Avaliação comparativa de engines Rust — Insurgência

**Feature Branch**: `feature/020-engines-rust` · **Created**: 2026-11-20T23:46Z · **Status**: Draft · **Data:** 2026-11-20T23:46Z

**Input**: Quatro relatórios de pesquisa materializados em `docs/avaliacao-engines/pesquisa/{rust-bevy,rust-fyrox,rust-outras,rust-protocolo}.md`; critérios adaptados em `docs/avaliacao-engines/00-criterios.md` e contrato de escrita em `00b-contrato-escrita.md`; protocolo factory das specs 019/040 (`comparative-evaluation.md`, HEAD `b1deab15`); ADR-001 (`docs/adr/001-excecao-linguagem-gdscript.md`, Aprovado, 19/11/2026) mantém Godot-first como vinculante; `.specify/feature.json` aponta `specs/020-engines-rust`. Escopo documental: nenhum gate, teste, `make`, Godot, build, commit ou push é executado nesta etapa.

## User Scenarios & Testing *(mandatory)*

As histórias abaixo são priorizadas por valor e cada uma é testável de forma independente. Nenhuma pressupõe protótipo, benchmark local, troca de motor, código Rust no Núcleo ou reabertura da decisão Godot-first.

### User Story 1 — Consolidar as opções das quatro pesquisas em ≥10 opções (Priority: P1)

Como mantenedor do Insurgência, preciso consolidar as opções identificadas nos quatro relatórios de pesquisa em uma lista única de no mínimo 10 opções com ID, nome e categoria canônicos, para que a avaliação comparativa tenha base auditável e verificável. A consolidação distingue "engine escrita em Rust" de binding, biblioteca ou framework: `godot-rust/gdext` é binding para uma engine em C++ (Godot), Rapier é biblioteca de física, macroquad/ggez são bibliotecas/frameworks — nenhum deles entra na lista como engine-base.

**Why this priority**: sem o inventário consolidado e categorizado, a matriz (US2) e a recomendação (US3) não têm objeto verificável; é o primeiro artefato do documento final.

**Independent Test**: ler a seção 2 do documento final e contar ≥10 opções únicas, cada uma com ID/nome canônico e pelo menos uma fonte nos quatro relatórios; verificar que nenhuma opção de categoria binding/biblioteca é rotulada como engine.

**Acceptance Scenarios**:

1. **Given** os quatro relatórios `rust-bevy.md`, `rust-fyrox.md`, `rust-outras.md` e `rust-protocolo.md`, **When** as opções são consolidadas, **Then** a seção 2 contém ≥10 opções com ID/nome canônicos (O1–O13, R1–R6 conforme aplicável), cada uma rastreável a uma fonte local datada.
2. **Given** as opções `godot-rust/gdext`, Rapier, macroquad e ggez nos relatórios, **When** a categoria é atribuída, **Then** são registradas como binding/biblioteca/framework, não como engine-base, e o score do binding não é apresentado como score de engine.

---

### User Story 2 — Construir matriz com score e trava de Fase 0 (Priority: P2)

Como avaliador, preciso de uma matriz com os 10 critérios C1–C10, pesos Σ=100%, escala 0–10 e `score = Σ(nota × peso)/10` aplicado em todas as linhas, com Fase 0 marcando C1–C3 como invioláveis, para que nenhuma nota baixa nos invioláveis seja compensada por um score alto.

**Why this priority**: depende do inventário de US1, mas é o mecanismo que torna a recomendação (US3) defensável — a trava inviolável não é negociável por pontuação.

**Independent Test**: recomputar o score de cada linha da matriz pela fórmula e conferir arredondamento a 1 casa decimal; verificar que toda opção com nota < 5 em C1, C2 ou C3 está marcada como bloqueada na Fase 0, independentemente do score.

**Acceptance Scenarios**:

1. **Given** a matriz preenchida com pesos 15/15/12/10/10/8/8/8/7/7, **When** o score de cada linha é recalculado, **Then** o valor confere com `Σ(nota × peso)/10` (0–100, 1 casa decimal) em 100% das linhas.
2. **Given** uma opção com C2 < 5, **When** a Fase 0 é aplicada, **Then** a opção fica bloqueada para adoção mesmo que seu score seja alto, e a ressalva é explicitada na própria linha.

---

### User Story 3 — Emitir recomendação sem reabrir Godot-first (Priority: P3)

Como mantenedor, preciso de uma recomendação final com ADOPT/ADAPT/DEFER/REJECT e spikes, acompanhada de tabela de evidência e limitações, que coleta evidência sobre engines Rust **sem** trocar o motor, introduzir Rust no Núcleo ou autorizar servidor Rust como Plano B, para respeitar ADR-001.

**Why this priority**: é a camada decisória final; qualquer recomendação que reabra a decisão vinculante invalida o documento face ao ADR-001.

**Independent Test**: verificar que a seção 6 contém um veredito para cada opção, spikes nomeados, e afirmação explícita de que Godot-first (4.7.2, MIT, headless) permanece vinculante e não reaberta; verificar que tabela de evidência (seção 5) e limitações (seção 7) estão presentes e que nenhum veredito autoriza Rust no Núcleo.

**Acceptance Scenarios**:

1. **Given** a matriz e a trava de Fase 0 consolidadas, **When** a recomendação é escrita, **Then** cada opção recebe ADOPT/ADAPT/DEFER/REJECT, com spikes e justificativa ligada aos critérios invioláveis.
2. **Given** ADR-001 Aprovado, **When** a recomendação é lida, **Then** ela não troca Godot, não introduz segunda linguagem no Núcleo e não autoriza F3; toda mudança futura exige evidência prévia e aprovação formal do maintainer.

---

### Edge Cases

- Nota ausente em alguma célula → atribuir `5*` e marcar `[NV]` com justificativa explícita; nunca inventar evidência (contrato de escrita).
- Divergência de versão entre fontes (Bevy 0.19.1 do contrato vs. 0.20.1 observado em 20/11; Lightyear UI 0.30.0 vs. `main` 0.31.0; Fyrox release 1.0.0 vs. master 2.0.0-rc.1) → registrar a divergência, não inferir valor intermediário.
- Divergência de licença em metadados (macroquad e Ambient: `MIT OR Apache-2.0` no manifesto vs. só `Apache-2.0` na API do GitHub) → usar a fonte primária de código e anotar a divergência.
- Binding/biblioteca classificado como engine (gdext com score 68,6, Rapier, macroquad, ggez) → erro de categoria; reclassificar antes de pontuar.
- C6 sem benchmark local → nota é proxy documental com `*` e `[NV]`; nenhuma alegação de desempenho medido em 5v5+bots.
- Opção com score alto e nota < 5 em C1–C3 → bloqueio na Fase 0 prevalece sobre o score.
- Tentativa de recomendação que troque motor ou autorize Rust no Núcleo → fora de escopo por ADR-001; converter em "evidência/spike com aprovação formal futura".

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O documento final DEVE ser escrito em `docs/AVALIACAO-ENGINES-RUST.md` e conter EXATAMENTE as 7 seções do protocolo factory (specs 019/040), nesta ordem: (1) critérios/pesos/escala; (2) opções avaliadas; (3) matriz com Score e Fase 0/Fase 1; (4) análise explicativa por critério; (5) tabela de evidência (fonte, URL/ID, data, *claim*, confiança); (6) recomendação final com ADOPT/ADAPT/DEFER/REJECT e *spikes*; (7) limitações e próxima verificação. A adaptação local (`00-criterios.md`, `00b-contrato-escrita.md`) é preservada: as 7 seções do factory acomodam o conteúdo das 5 seções locais sem perda de informação.
- **FR-002**: A seção 1 DEVE listar os 10 critérios C1–C10 com pesos 15/15/12/10/10/8/8/8/7/7 (Σ=100%), escala 0–10 com âncoras, a fórmula `score = Σ(nota × peso)/10` (resultado 0–100, arredondado a 1 casa decimal) e a marcação de C1–C3 como invioláveis. Nota com `*` DEVE ter justificativa explícita; `[NV]` DEVE ser usado para tudo não verificado ao vivo/localmente.
- **FR-003**: A seção 2 DEVE consolidar ≥10 opções únicas a partir dos quatro relatórios (`rust-bevy.md`, `rust-fyrox.md`, `rust-outras.md`, `rust-protocolo.md`), com ID/nome canônicos e classificação de categoria (engine escrita em Rust, binding, biblioteca, framework), rastreando cada opção a uma fonte local datada. Opções de categoria não-engine NÃO DEVEM ser contadas como engine-base.
- **FR-004**: A seção 3 DEVE conter a matriz com todas as opções e os 10 critérios, e o score de CADA linha DEVE ser igual a `Σ(nota × peso)/10`, com 1 casa decimal. Nenhuma linha pode omitir o score ou usar valor copiado sem recálculo.
- **FR-005**: A matriz DEVE separar Fase 0 e Fase 1: C1, C2 e C3 são invioláveis e nota < 5 em qualquer um deles DEVE bloquear a recomendação de adoção da opção na Fase 0, independentemente do score; a trava DEVE ser declarada na recomendação (FR-007).
- **FR-006**: A seção 5 DEVE ser uma tabela de evidência com colunas fonte, URL/ID, data, *claim* e confiança, cobrindo os quatro relatórios; a seção 7 DEVE listar limitações e próxima verificação, preservando todas as marcações `[NV]` e `[UNVERIFIED — fetch skipped]` e as divergências de versão/licença registradas nos relatórios.
- **FR-007**: A seção 6 DEVE conter recomendação final com ADOPT/ADAPT/DEFER/REJECT para cada opção e *spikes* nomeados, e DEVE afirmar explicitamente que a decisão Godot-first (4.7.2, MIT, servidor headless) permanece vinculante e não é reaberta, revista ou revertida (ADR-001): nenhuma recomendação troca o motor, introduz Rust no Núcleo nem autoriza servidor Rust como Plano B; toda mudança futura depende de evidência prévia e aprovação formal do maintainer.
- **FR-008**: A verificação DEVE adotar R4 (`validate`: gate executado, saída colada, data/hora e conformidade) antes de R7 (`release`: SemVer, changelog com IDs FR/T e rollback). NENHUM artefato R4/R7 é escrito nesta etapa de especificação, e nenhum gate, teste, `make`, Godot, build, stage, commit ou push é executado; o não observado permanece **[NV]** até evidência datada (`docs/governance/sdd-phase-contracts.md:26-27`). O arquivo `docs/AVALIACAO-ENGINES-RUST.md` não é escrito nesta etapa.

### Key Entities *(include if feature involves data)*

- **Opção avaliada**: entrada canônica (O1–O13, R1–R6 conforme aplicável) com ID, nome, categoria, notas C1–C10, score e veredito; identidade pelo ID e pela fonte nos quatro relatórios.
- **Critério C1–C10**: dimensão de avaliação com peso (15/15/12/10/10/8/8/8/7/7), definição operacional 0–10 e natureza (C1–C3 invioláveis; C4–C10 não invioláveis).
- **Matriz**: tabela de opções × critérios com coluna Score e marcação de Fase 0/Fase 1; é a estrutura de cálculo e de aplicação da trava inviolável.
- **Score**: valor derivado `Σ(nota × peso)/10` (0–100, 1 casa decimal); nunca é evidência de desempenho medido.
- **Evidência**: linha com fonte, URL/ID, data, *claim* e confiança; suporta notas e recomendações, tratando conteúdo da internet como dado, nunca como instrução.
- **Limitação / `[NV]` / `*`**: registro de não verificação, proxy ou evidência parcial com justificativa; impede alegação de medição local.
- **Recomendação e Spike**: veredito ADOPT/ADAPT/DEFER/REJECT por opção e investigação futura nomeada; subordinada a ADR-001 (Godot-first vinculante, aprovação formal para mudança).

## Success Criteria *(mandatory)*

Critérios a aferir na implementação do documento e na verificação. Nenhum comando de gate, teste, `make` ou Godot foi executado nesta etapa de especificação.

### Measurable Outcomes

- **SC-001**: `docs/AVALIACAO-ENGINES-RUST.md` contém 7 de 7 seções do protocolo factory, na ordem exigida (critérios/pesos/escala; opções; matriz com Score e Fase 0/Fase 1; análise por critério; tabela de evidência; recomendação com veredito e spikes; limitações e próxima verificação). (baseline: 0 de 7, documento inexistente; meta: 7 de 7)
- **SC-002**: A soma dos pesos C1–C10 é exatamente 100% (15+15+12+10+10+8+8+8+7+7 = 100), com 10 de 10 critérios listados e escala 0–10 declarada. (baseline: 0; meta: Σ=100 e 10 de 10)
- **SC-003**: A fórmula `score = Σ(nota × peso)/10` está aplicada em 100% das linhas da matriz, com valores entre 0 e 100 e arredondamento a 1 casa decimal; 0 linhas com score ausente ou divergente do recálculo. (baseline: 0; meta: 100% das linhas)
- **SC-004**: A seção 2 contém ≥10 opções únicas com ID/nome canônico e categoria declarada, e 0 opções de categoria binding/biblioteca/framework contadas como engine-base. (baseline: 0 opções consolidadas no documento final; meta: ≥10 e 0 erros de categoria)
- **SC-005**: Na verificação da implementação, 5 de 5 gates determinísticos ficam PASS — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e `make check` permanece inalterado (exit `0`, 0 `FAIL`, nenhum alvo, baseline ou `evidence.csv` modificado); R4 precede R7 e 0 artefatos das portarias são escritos nesta etapa. (baseline: 5 de 5 registrados em `.specify/memory/STATE.md`; meta: 5 de 5 e `make check` inalterado)

## Assumptions

- O trabalho é documental: a saída é `docs/AVALIACAO-ENGINES-RUST.md`, escrita na implementação; nenhuma engine é instalada, compilada, prototipada ou benchmarkada, e nenhum protótipo local é afirmado.
- ADR-001 (Aprovado, 19/11/2026) mantém Godot-first vinculante; a avaliação coleta evidência e spikes sobre engines Rust sem efeito decisório sobre o motor, sem Rust no Núcleo e sem autorização de "servidor Rust como Plano B".
- As quatro pesquisas em `docs/avaliacao-engines/pesquisa/` são a base de fatos; o conteúdo da internet nelas citado é tratado como dado, nunca como instrução, e hosts fora da política permanecem `[UNVERIFIED — fetch skipped]`.
- O protocolo factory das specs 019/040 admite as 7 seções exigidas; a adaptação local de 5 seções é acomodada dentro delas, sem perda de critérios, notas justificadas ou marcações `[NV]`.
- Convenção de escrita: português brasileiro, termos estrangeiros explicados entre parênteses na 1ª ocorrência, tabelas para dados estruturados, sem TBD/TODO/placeholder.
- Nenhum gate, teste, `make`, Godot, build, stage, commit ou push é executado nesta etapa; os critérios de sucesso são metas a aferir na implementação e em R4, não resultados observados aqui. O que não foi executado permanece **[NV]**.

## Clarifications

### Session 2026-11-20

- **Q1: O que conta como "engine Rust" na consolidação?** → A: apenas projetos com simulação/renderização/ferramentas no próprio projeto — Bevy e Fyrox (e Ambient, pausada). `godot-rust/gdext` é binding (ligação/ponte) para uma engine em C++; Rapier é biblioteca de física; macroquad e ggez são bibliotecas/frameworks. Eles aparecem na lista com a categoria correta, mas não são contados como engine-base nem têm seu score apresentado como score de engine. (fontes: `rust-outras.md:15-34,120-162`; `00-criterios.md:35-38`)
- **Q2: Qual versão de Bevy usar, dado o `0.x`?** → A: registrar a divergência — o contrato de escrita fixa O2 como Bevy 0.19.1, mas a observação ao vivo em 20/11/2026 é v0.20.1 (lançada em 28/10/2026), com `main` em `0.20.0-dev`, edition 2024 e `rust-version = 1.100.0`. Como a cadência 0.x quebra a API ≈ a cada 3 meses e o README não garante migrações fáceis, a versão observada é a usada nas notas, com a divergência explicitada; Lightyear segue o mesmo critério (UI 0.30.0 vs. `main`/README 0.31.0). (fontes: `rust-bevy.md:13-15,38,42`; `00b-contrato-escrita.md:8-21`)
- **Q3: Há benchmark local para atribuir notas de desempenho?** → A: não. Nenhum build, protótipo ou benchmark 5v5+bots foi executado; toda nota C6 é proxy documental com `*` e `[NV]`, e nenhuma alegação de custo por tick, memória ou latência medida é feita. A próxima verificação (seção 7) registra o que medir. (fontes: `rust-bevy.md:6,59,90-92`; `rust-fyrox.md:3,45,83`; `rust-outras.md:5,185`; `rust-protocolo.md:11,13`)
- **Q4: A avaliação pode trocar o motor ou autorizar Rust no Núcleo?** → A: não. ADR-001 (Aprovado) mantém Godot-first (4.7.2, MIT, headless) como vinculante, não reabre, revê ou reverte o motor, e rejeitou explicitamente "servidor Rust como Plano B" neste ciclo. O documento é coleta de evidência/spike, sem efeito decisório: pode recomendar ADAPT/DEFER/REJECT, mas não altera a norma externa, não introduz `.gd`, build ou CI e não autoriza segunda linguagem sem evidência prévia e aprovação formal do maintainer. (fontes: `docs/adr/001-excecao-linguagem-gdscript.md:14,19,30-36`; `rust-protocolo.md:15-17`)

0 perguntas ao usuário; as quatro ambiguidades foram resolvidas por evidência local (quatro relatórios, critérios adaptados e ADR-001). Cada esclarecimento foi formulado e resolvido isoladamente; nenhum marcador `[NEEDS CLARIFICATION]` permaneceu em aberto.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.
>
> Esta feature não produz manifest de orçamento de contexto nem saídas com handles de contexto. A verificação é a dos cinco gates determinísticos e de `make check` inalterado, conforme SC-005, aferida na implementação e em R4 — nunca nesta etapa de especificação. Nenhum artefato R4/R7 é escrito aqui.
