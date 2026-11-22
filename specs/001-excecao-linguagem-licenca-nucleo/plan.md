# Implementation Plan: Exceção de linguagem e licença MIT-only do Núcleo

**Branch**: `001-excecao-linguagem-licenca-nucleo` | **Date**: 2026-11-19 | **Spec**: [spec.md](./spec.md)

**Input**: especificação esclarecida em `specs/001-excecao-linguagem-licenca-nucleo/spec.md` — decisão R1 de 2026-11-19 (veredito `go`, opção A), quatro respostas em `## Clarifications` e zero marcadores pendentes.

**Fase**: 8 (plan) do contrato SDD (`docs/governance/sdd-phase-contracts.md:17`). Antecede checklist (9) e tasks (11). Nenhum comando, gate, build ou teste é executado nesta fase.

## Summary

Produzir dois registros de decisão locais versionados — ADR-001 (exceção de linguagem para GDScript) e ADR-002 (licença MIT-only do Núcleo) — com campos mínimos, autoridade de aprovação e rastreabilidade explícita para os três desvios de origem e para a decisão R1, sem reabrir o pilar Godot-first e sem produzir código. A abordagem técnica é estritamente documental: artefatos Markdown versionados, um contrato de campos/IDs/SemVer/rollback, uma matriz de rastreabilidade e um procedimento de verificação por leitura. A origem autoritativa é a spec; as decisões de desenho estão em [research.md](./research.md), as entidades em [data-model.md](./data-model.md), o contrato em [contracts/adr-contract.md](./contracts/adr-contract.md) e a verificação em [quickstart.md](./quickstart.md).

## Technical Context

| Campo | Valor | Fundamento |
|---|---|---|
| Language/Version | **Não aplicável** — nenhuma linguagem de implementação é usada neste ciclo; GDScript é o *objeto* da decisão registrada no ADR-001, não a linguagem do ciclo. | spec FR-010, SC-007, Assumptions |
| Primary Dependencies | **Não aplicável** — nenhuma engine, SDK, biblioteca ou serviço é instalado ou invocado; os artefatos são arquivos Markdown versionados. | spec FR-010, SC-007 |
| Storage | **Não aplicável** como persistência de runtime — o repositório Git local com arquivos Markdown é o único suporte; não há banco, fila, índice ou cache. | spec Assumptions, AGENTS.md:3-5 |
| Testing | **Não aplicável** como execução automatizada — a verificação é leitura cruzada e comparação textual; nenhum teste, build ou gate é rodado na fase de planejamento; R4 e R7 são portarias do ciclo completo. | spec US3 (Independent Test), SC-003, FR-010 |
| Target Platform | **Não aplicável** — não há binário, runtime, device ou deploy; o destino é o repositório documental do alvo. | AGENTS.md:3-5, spec Assumptions |
| Project Type | Documental — registro local de decisões de arquitetura (ADRs) e rastreabilidade de governança. | spec FR-001, FR-009 |
| Performance Goals | **Não aplicável** — nenhum sistema em execução produz latência ou vazão; o sucesso é medido por contagens e comparação textual. | spec SC-001–SC-008 |
| Constraints | Escopo estritamente documental (FR-010); exatamente dois ADRs (FR-001); Godot-first intocável (FR-003, SC-003); MIT sem alternativa e sem AGPL para o código do ciclo (FR-005, FR-006); aprovação humana necessária (FR-011); status remoto, CI, merge e proteção de branch `[NV]` (SC-006); nenhuma emenda da norma externa neste ciclo (FR-010). | spec Requirements e Success Criteria |
| Scale/Scope | 2 ADRs; 3 desvios cobertos (SC-002); 4 referências de rastreabilidade (SC-008); 13 requisitos funcionais; 8 critérios de sucesso. | spec FR-001–FR-013, SC-001–SC-008 |
| Versionamento dos registros | SemVer `1.0.0` na materialização inicial de cada ADR; regras de changelog e rollback documentadas no contrato. | contracts/adr-contract.md §4–§5; docs/governance/sdd-phase-contracts.md:27 |
| Meio de verificação | Leitura local dos artefatos, sem rede, sem ambiente de execução e sem alteração de arquivos. | quickstart.md |

Nenhum campo acima permanece em branco: os campos sem aplicação a um ciclo documental trazem a declaração "**Não aplicável**" com o respectivo motivo.

## Constitution Check

*Gate reavaliado após a Phase 1 (data model, contratos e quickstart), conforme `.specify/memory/constitution.md` e o contrato de fase. A reposição abaixo cobre integralmente os princípios I–VI.*

| Princípio | Exigência | Avaliação pré-projeto | Reavaliação pós-projeto | Resultado |
|---|---|---|---|---|
| I — Determinismo primeiro | Trabalho computável por script/gate/template deve sê-lo; saída de LLM nunca roteia estado nem finaliza validação. | A spec fixa verificação por leitura e comparação textual (SC-003); nenhum estado é decidido por texto gerado. | O contrato define IDs, campos, SemVer e rollback como regras determinísticas; o quickstart é somente leitura; nenhum gate é executado ou relatado como verde. | **Atendido** |
| II — Adaptadores e portas | Nenhum código de domínio chama diretamente serviço concreto; integrações por trait/interface e adaptador. | Não há código no ciclo; FR-010 veta serviço externo. | Nenhum módulo, chamada ou adaptador é introduzido; a restrição permanece registrada como limite para ciclos futuros, sem implementação aqui. | **Atendido por não aplicabilidade declarada** — motivo: o ciclo não produz código; a regra é preservada como restrição, não como artefato. |
| III — Fluxo SDD ordenado | Ordem integral das portarias; R1=`go`; R4 precede R7; `converge.md` append-only. | intake→research→define→shape→decide→specify→clarify concluídos; R1=`go` (decision.md:5). | Plan é a fase 8; checklist (9), tasks (11), validate R4 e release R7 permanecem posteriores; nenhuma portaria antecipada e nenhum artefato de fase futura alterado. | **Atendido** |
| IV — Soberania do jogador | Regras, servidores e conteúdo inspecionáveis, modificáveis e redistribuíveis; sem dependência proprietária ou serviço externo no caminho crítico. | MIT-only (FR-005) preserva inspeção, modificação e redistribuição; FR-008 protege a licença do motor. | O contrato registra a ressalva de que o motor não recebe licença exclusiva e veta cópia de código copyleft (FR-007, FR-008). | **Atendido** |
| V — Materialismo e ação coletiva | Sem magia ou campeão solitário; habilidades são ações materiais e a vitória exige coordenação entre funções. | O ciclo não define mecânicas, habilidades ou regras de jogo (FR-010). | Nenhum conteúdo de magia, campeão ou vitória é introduzido; a aprovação é ato humano registrado (FR-011), não uma decisão automática. | **Atendido por não aplicabilidade declarada** — motivo: o objeto do ciclo é governança documental, não design de jogo; nenhuma regra do pilar é alterada. |
| VI — Verificação honesta | Nada é verde sem comando executado e saída colada; remoto/CI/merge/proteções `[NV]`; divergências registradas. | Toda evidência é local (`local-only`); status remoto `[NV]` (spec SC-006). | O quickstart não afirma execução; a divergência de numeração entre ADR-002 local e ADR-002 do documento externo está registrada em research.md (RQ-2); o contrato mantém rollback e changelog, sem conclusão de release. | **Atendido** |

**Complexity Tracking**: nenhuma violação de constituição foi identificada; a tabela de exceções é **não aplicável** nesta feature, porque o desenho introduz apenas registros documentais e nenhum módulo, adaptador, dependência ou exceção de arquitetura que precisasse de justificativa.

## Project Structure

### Documentation (this feature)

```text
specs/001-excecao-linguagem-licenca-nucleo/
├── spec.md                          # origem autoritativa (fase specify/clarify)
├── plan.md                          # este arquivo (fase plan)
├── research.md                      # decisões de pesquisa — F0 (fase plan)
├── data-model.md                    # entidades ADR-001, ADR-002 e rastreabilidade — F1
├── contracts/
│   └── adr-contract.md              # campos, IDs FR/T, SemVer e rollback — F1
├── quickstart.md                    # verificação local somente leitura — F1
├── checklists/
│   ├── requirements.md              # revalidado na clarify; não alterado nesta fase
│   └── adr-quality.md               # checklist de qualidade dos requisitos (fase 9)
└── tasks.md                         # saída da fase 11 — não criada por /speckit.plan
```

### Source Code (repository root)

**Não aplicável** — nenhum diretório de código, engine, teste, build ou migração é criado neste ciclo (spec FR-010 e SC-007; AGENTS.md:3-5). Os ADRs materializados na fase 12 são arquivos Markdown nos caminhos canônicos `docs/adr/001-excecao-linguagem-gdscript.md` e `docs/adr/002-licenca-mit-nucleo.md`, conforme `contracts/adr-contract.md §1`.

**Structure Decision**: adotar apenas a árvore documental acima. O contrato e o modelo de dados são artefatos de desenho; os registros ADR são criados na fase de implementação e não são antecipados nesta fase.

## Technical Decisions

| ID | Decisão | Rationale | Origem |
|---|---|---|---|
| D1 | Registros locais versionados em Markdown; nenhuma emenda da norma externa neste ciclo. | Resolve os desvios no espaço onde vivem (documentação) com appetite `small`; a emenda normativa é contingência futura. | spec FR-001, FR-010; R1 (decision.md:21-25) |
| D2 | Identidades fixas `ADR-001` e `ADR-002`, com a numeração local prevalecendo sobre a numeração de documento externo. | R1 e a spec definem explicitamente os dois registros; a divergência externa é registrada, não silenciada. | spec FR-001; R1 (decision.md:29-31); research.md RQ-2 |
| D3 | Esquema único de campos mínimos para os dois ADRs, acrescido dos campos específicos de cada decisão. | FR-009 exige título, data, contexto, decisão, alternativas, consequências, autoridade e rastreabilidade; FR-002–FR-008 e FR-013 acrescentam o conteúdo próprio. | spec FR-002–FR-009, FR-013; data-model.md |
| D4 | Matriz de rastreabilidade com quatro referências (`DEV-LING-01`, `DEV-EXC-01`, `DEV-LIC-01`, `DEC-R1`), associando cada desvio a um ADR e ambos os ADRs a R1. | Garante 3/3 desvios cobertos e 4/4 referências presentes, sem consulta a fonte externa. | spec FR-012, SC-002, SC-008; data-model.md §Registro de rastreabilidade |
| D5 | Não reabertura do pilar expressa por declaração textual verificável por comparação; nenhum gate de checagem documental. | SC-003 exige zero alterações e SC-004 exige a declaração nos dois ADRs; FR-010 veta gates na fase de planejamento, enquanto R4 e R7 são portarias do ciclo completo. | spec FR-003, FR-010, SC-003, SC-004 |
| D6 | Aprovação humana como autoridade necessária; todo status remoto rotulado `[NV]`. | Constituição VI e FR-011/SC-006 impedem afirmar aprovação, CI, merge ou proteção sem confirmação ao vivo. | spec FR-011, SC-006; constitution.md:17 |
| D7 | Versionamento SemVer `1.0.0` com changelog por IDs FR/T e rollback documental via novo commit; nunca force-push. | Padroniza a evolução dos registros e preserva histórico, em conformidade com as normas de trabalho e a fase release posterior. | contracts/adr-contract.md §4–§5; AGENTS.md:37-45; sdd-phase-contracts.md:27 |
| D8 | Verificação somente leitura; comandos de setup/teste/build classificados como não aplicáveis. | O sucesso não depende de execução (spec Assumptions) e FR-010/SC-007 vedam código, compilação, testes e gates nos ADRs. | spec US3, FR-010, SC-007; quickstart.md |

## Phase 0 and Phase 1 Outputs

| Artefato | Fase | Conteúdo |
|---|---|---|
| [research.md](./research.md) | F0 — Outline & Research | Decisões de pesquisa com justificativa, alternativas e fontes exclusivamente locais; nenhuma fonte externa consultada. |
| [data-model.md](./data-model.md) | F1 — Design | Entidades ADR-001, ADR-002 e Registro de rastreabilidade, com campos, validações, estados e campos não aplicáveis. |
| [contracts/adr-contract.md](./contracts/adr-contract.md) | F1 — Contracts | IDs, campos obrigatórios, rastreabilidade FR/T, SemVer, changelog, rollback e critérios de conformidade. |
| [quickstart.md](./quickstart.md) | F1 — Validation guide | Procedimento de verificação local por leitura, resultados esperados e declarações de não aplicabilidade. |

## Readiness Gate

- Spec esclarecida: `## Clarifications / ### Session 2026-11-19` com quatro respostas (a 4ª sobre autoria, resolvida por premissa conforme R3, uma por vez); zero marcadores pendentes; `checklists/requirements.md` revalidado em 16/16 na fase clarify — sem alteração nesta fase.
- Constitution Check I–VI reavaliado após o desenho: sem violações e sem exceções a justificar.
- Este plano não cria, antecipa ou modifica artefatos de fases posteriores; a existência de `checklists/adr-quality.md` e de `tasks.md` no diretório da feature decorre das portarias 9 e 11 executadas depois deste plano, não deste documento.
- Próxima portaria: fase 9 (checklist), com `checklists/adr-quality.md` append-only e itens `- [ ] CHK###`.
- Estado remoto, aprovação humana, CI, merge e proteção de branch permanecem `[NV]` até confirmação ao vivo.
