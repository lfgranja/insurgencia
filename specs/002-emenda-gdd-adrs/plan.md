# Implementation Plan: Emenda pontual do GDD a partir dos ADRs 001/002

**Branch**: `002-emenda-gdd-adrs` | **Date**: 2026-11-19 | **Spec**: [spec.md](./spec.md)

**Input**: spec esclarecida em `specs/002-emenda-gdd-adrs/spec.md` — decisão R1 de 2026-11-19 (veredito `go`, opção A), três respostas em `## Clarifications / ### Session 2026-11-19`, zero marcadores pendentes e `checklists/requirements.md` revalidado em 18/18.

**Fase**: 8 (plan) do contrato SDD (`docs/governance/sdd-phase-contracts.md:17`). Antecede checklist (9) e tasks (11). Nenhum comando, gate determinístico, build, teste ou efeito remoto é executado nesta fase: as verificações documentais executadas neste ciclo retornam resultado observável, e os gates determinísticos pertencem à portaria R4 (validate), não a tarefas desta feature.

## Summary

Emendar pontualmente sete passagens do documento normativo `docs/GDD - Insurgência.md` (`1389,1451,1459,1594,1659,1668,1673`) usando como autoridade local os ADRs materializados: ADR-002 (licença MIT única, sem alternativa "MIT ou GPL" e sem AGPL para o código do ciclo) em seis passagens e ADR-001 (independência de CI e não reabertura Godot-first) na passagem de sustentabilidade. Cada passagem alterada identifica o ADR que a decide no próprio GDD. As âncoras Godot-first (`1265,1678`) e o texto ao redor permanecem intocáveis; a verificação é feita por leitura cruzada e comparação textual. A origem autoritativa é a spec; as decisões de desenho estão em [research.md](./research.md), as entidades em [data-model.md](./data-model.md), o contrato das sete passagens em [contracts/gdd-amendment-contract.md](./contracts/gdd-amendment-contract.md) e o procedimento em [quickstart.md](./quickstart.md).

## Technical Context

| Campo | Valor | Fundamento |
|---|---|---|
| Language/Version | **Não aplicável** — nenhuma linguagem de implementação é usada; GDScript é o objeto da decisão registrada no ADR-001, não a linguagem desta emenda. | spec SC-007, Assumptions |
| Primary Dependencies | **Não aplicável** — nenhuma engine, SDK, biblioteca ou serviço é instalado ou invocado; o único artefato normativo de origem alterado é um arquivo Markdown versionado, e os artefatos de planejamento são apenas conferidos por leitura, sem alteração. | spec SC-007, AGENTS.md:3-5 |
| Storage | **Não aplicável** como persistência de runtime — o repositório Git local é o único suporte; não há banco, fila, índice ou cache. O arquivo alterado é `docs/GDD - Insurgência.md`. | spec Key Entities, AGENTS.md:3-5 |
| Testing | **Não aplicável** como execução automatizada — a verificação é leitura das sete passagens e comparação textual das duas âncoras, com resultado observável; nenhum teste, build ou gate determinístico é rodado nesta fase, pois os gates determinísticos pertencem à portaria R4 (validate), não a tarefas desta feature, e R7 é portaria posterior do ciclo completo. | spec US3, SC-003, SC-007 |
| Target Platform | **Não aplicável** — não há binário, runtime, device ou deploy; o destino é o documento normativo do repositório do alvo. | AGENTS.md:3-5, spec Assumptions |
| Project Type | Documental — emenda pontual e auditável de texto normativo já versionado. | spec FR-001–FR-007, SC-007 |
| Performance Goals | **Não aplicável** — nenhum sistema em execução produz latência ou vazão; o sucesso é medido por contagens (7/7, 0/2, 7/7) e comparação textual. | spec SC-001–SC-007 |
| Constraints | Escopo exatamente das sete passagens `1389,1451,1459,1594,1659,1668,1673`; âncoras `1265,1678` intocáveis; MIT único sem "ou GPL" e sem AGPL no código do projeto; CI e gate remoto registrados como diferidos, sem gatilho ou pipeline; licenças de terceiros fora da autoridade do ADR-002; rastreabilidade com a numeração materializada dos ADRs; aprovação remota, CI, merge e proteção `[NV]`; nenhuma emenda da norma externa. | spec Requirements, Success Criteria, Edge Cases, Clarifications |
| Scale/Scope | 7 passagens emendadas; 7 requisitos funcionais; 3 user stories (P1, P1, P2); 7 critérios de sucesso; 2 âncoras; 2 ADRs invocados; 6 passagens de licenciamento e 1 de CI. | spec FR-001–FR-007, US1–US3, SC-001–SC-007 |
| Versionamento da emenda | SemVer `1.0.0` no registro documental da emenda; changelog por IDs FR e `T###` e rollback documental por novo commit, definidos no contrato §5. O GDD não recebe campo de versão novo. | contracts/gdd-amendment-contract.md §5; sdd-phase-contracts.md:27 |
| Meio de verificação | Leitura local das sete passagens, das duas âncoras e das referências aos ADRs, com resultado observável; sem rede, sem ambiente de execução e sem alteração de arquivos nesta fase; os gates determinísticos pertencem à portaria R4 (validate), não a tarefas desta feature. | quickstart.md |
| Numeração de ADR | A materializada no repositório: `ADR-001` = independência de CI e não reabertura Godot-first; `ADR-002` = licença MIT única. A numeração provisória da avaliação de stack não é citada. | spec Clarifications; research.md RQ-2 |

Nenhum campo acima permanece em branco: os campos sem aplicação a uma emenda documental trazem a declaração "**Não aplicável**" com o respectivo motivo.

## Constitution Check

*Gate reavaliado após a Phase 1 (modelo de dados, contrato e quickstart), conforme `.specify/memory/constitution.md` e o contrato de fase. A reposição abaixo cobre integralmente os princípios I–VI.*

| Princípio | Exigência | Avaliação pré-projeto | Reavaliação pós-projeto | Resultado |
|---|---|---|---|---|
| I — Determinismo primeiro | Trabalho computável por script/gate/template deve sê-lo; saída de LLM nunca roteia estado nem finaliza validação. | A spec fixa contagens e comparação textual como verificação (SC-001–SC-004); nenhum estado é decidido por texto gerado. | O contrato define IDs, mapa de passagens, regras de redação e rollback como regras determinísticas; o quickstart é somente leitura; nenhum gate é executado ou relatado como verde nesta feature, pois os gates determinísticos pertencem à portaria R4 (validate), não a tarefas. | **Atendido** |
| II — Adaptadores e portas | Nenhum código de domínio chama diretamente serviço concreto; integrações por trait/interface e adaptador. | Não há código no ciclo; SC-007 veta serviço externo e efeito remoto. | Nenhum módulo, chamada ou adaptador é introduzido; a restrição permanece registrada como limite, não como artefato. | **Atendido por não aplicabilidade declarada** — motivo: a emenda é de texto e não produz código; a regra é preservada como restrição. |
| III — Fluxo SDD ordenado | Ordem integral das portarias; R1=`go`; R4 precede R7; `converge.md` append-only. | intake→research→define→shape→decide→specify→clarify concluídos; R1=`go` (decision.md:5). | Plan é a fase 8; checklist (9), tasks (11) e as portarias posteriores permanecem seguintes; nenhuma portaria antecipada e nenhum artefato de fase futura alterado. | **Atendido** |
| IV — Soberania do jogador | Regras, servidores e conteúdo inspecionáveis, modificáveis e redistribuíveis; sem dependência proprietária ou serviço externo no caminho crítico. | MIT único (FR-001–FR-003, FR-005–FR-007) preserva inspeção, modificação e redistribuição; a ressalva de instância de terceiro é esclarecida, não decidida. | O contrato preserva portabilidade e direito de saída como fundamento da soberania e mantém licenças de terceiros fora de escopo; nenhum serviço externo entra no texto. | **Atendido** |
| V — Materialismo e ação coletiva | Sem magia ou campeão solitário; habilidades são ações materiais e a vitória exige coordenação entre funções. | A emenda não define mecânicas, habilidades ou regras de jogo. | Nenhum conteúdo de magia, campeão ou vitória é introduzido; a decisão permanece ato humano registrado na governança local, não automático. | **Atendido por não aplicabilidade declarada** — motivo: o objeto é redação normativa de governança, não design de jogo; nenhuma regra do pilar é alterada. |
| VI — Verificação honesta | Nada é verde sem comando executado e saída colada; remoto/CI/merge/proteções `[NV]`; divergências registradas. | Toda evidência é local; status remoto `[NV]` (SC-003) e a ressalva de aprovação pendente dos ADRs é registrada na spec. | O quickstart não afirma execução; a numeração provisória divergente e a pendência de aprovação humana são registradas em research.md (RQ-2, RQ-5); o contrato mantém rollback e changelog, sem afirmar release. | **Atendido** |

**Complexity Tracking**: nenhuma violação de constituição foi identificada; a tabela de exceções é **não aplicável** nesta feature, porque o desenho altera apenas sete passagens de texto e não introduz módulo, adaptador, dependência, gate ou exceção de arquitetura que exigisse justificativa.

## Project Structure

### Documentation (this feature)

```text
specs/002-emenda-gdd-adrs/
├── spec.md                              # origem autoritativa (fase specify/clarify)
├── plan.md                              # este arquivo (fase plan)
├── research.md                          # decisões de pesquisa — F0 (fase plan)
├── data-model.md                        # GDD, 7 passagens, âncoras e rastreabilidade — F1
├── contracts/
│   └── gdd-amendment-contract.md        # mapa FR/T das 7 passagens, redação, SemVer e rollback — F1
├── quickstart.md                        # verificação por leitura, sem código — F1
├── checklists/
│   ├── requirements.md                  # revalidado na clarify; não alterado nesta fase
│   └── gdd-amendment.md                 # checklist de qualidade dos requisitos (fase 9)
└── tasks.md                             # saída da fase 11 — não criada por /speckit.plan
```

### Source Code (repository root)

**Não aplicável** — nenhum diretório de código, engine, teste, build ou migração é criado neste ciclo (spec SC-007; AGENTS.md:3-5). O único arquivo de origem no escopo da emenda é `docs/GDD - Insurgência.md`, nas linhas `1389,1451,1459,1594,1659,1668,1673`. Os artefatos de planejamento são apenas conferidos por leitura, sem alteração. Os ADRs invocados (`docs/adr/001-excecao-linguagem-gdscript.md` e `docs/adr/002-licenca-mit-nucleo.md`) são lidos como autoridade e não são alterados.

**Structure Decision**: adotar apenas a árvore documental acima. Contrato e modelo de dados são artefatos de desenho; a emenda das sete passagens pertence à fase de implementação e não é antecipada neste plano.

## Technical Decisions

| ID | Decisão | Rationale | Origem |
|---|---|---|---|
| D1 | Emenda pontual nas sete passagens mapeadas; nenhuma reescrita ampla de seção. | Opção A do shape (`small`); reduz 7 divergências a 0 sem reescrever trabalho alheio. | spec US3, SC-001; R1 (decision.md:21-25); concept.md:9-17 |
| D2 | ADR-002 autoriza as seis passagens de licenciamento; ADR-001 autoriza a passagem de CI. | É a divisão de autoridade definida na spec e no mapa de passagens; nenhuma passagem cite numeração provisória. | spec Key Entities; Clarifications; research.md RQ-2 |
| D3 | Referência ao ADR decide escrita no próprio GDD, uma por passagem alterada. | SC-004 exige 7/7 rastreabilidade consultável sem fonte externa. | spec FR-001–FR-007, SC-004 |
| D4 | As duas âncoras Godot-first (`1265,1678`) são premissa de comparação, nunca alvo de emenda. | SC-002 exige zero alterações; PROGRESSO e ambos os ADRs as declaram intocáveis. | spec SC-002, Edge Cases; ADR-001:32; ADR-002:33 |
| D5 | Escopo da ressalva AGPL restrito a servidores de terceiros, sem decidir licença de terceiros. | A autoridade do ADR-002 cobre apenas o código do projeto; a pergunta sobre terceiros fica fora de escopo. | spec FR-001, Edge Cases, Assumptions; ADR-002:28,41 |
| D6 | Contradição de CI resolvida alterando a promessa em `1594`, não registrando "DEFER com gatilho". | A spec e a clarify escolhem alterar a promessa; gatilho, pipeline e build são vetados e conflitam com o DEFER vigente. | spec FR-004, SC-006, Clarifications; research.md RQ-4 |
| D7 | N3 deixa de bloquear a "Primeira publicação" por decisão documental, sem publicação externa verificada. | A decisão local é afirmativa; efeito remoto permanece `[NV]` e não é pré-requisito. | spec FR-006, FR-007, Assumptions; ADR-002:14 |
| D8 | Versionamento SemVer `1.0.0`, changelog com IDs FR/`T###` e rollback documental por novo commit. | Padroniza a evolução da emenda e preserva histórico; sem force-push, sem merge automático e sem apagamento de redação anterior. | contracts/gdd-amendment-contract.md §5; AGENTS.md:37-45 |
| D9 | Verificação somente por leitura, com resultado observável; setup, build, teste e gate determinístico classificados como não aplicáveis com motivo. | Os critérios são contagens e comparações textuais; nenhum depende de execução, SC-007 veta código e gates no resultado, e os gates determinísticos pertencem à portaria R4 (validate), não a tarefas desta feature. | spec US3, SC-003, SC-007; quickstart.md |

## Phase 0 and Phase 1 Outputs

| Artefato | Fase | Conteúdo |
|---|---|---|
| [research.md](./research.md) | F0 — Outline & Research | Decisões de pesquisa com decisão, justificativa, alternativas e fontes exclusivamente locais; evidência contrária tratada; nenhuma URL aberta. |
| [data-model.md](./data-model.md) | F1 — Design | Entidades documentais GDD, Passagem de emenda, Âncora Godot-first, Rastreabilidade e Registro de emenda, com campos, validações, transições e campos não aplicáveis. |
| [contracts/gdd-amendment-contract.md](./contracts/gdd-amendment-contract.md) | F1 — Contracts | Mapa das 7 passagens com IDs FR e `T###`, regras de redação, rastreabilidade, SemVer, changelog, rollback documental e critérios de conformidade por leitura. |
| [quickstart.md](./quickstart.md) | F1 — Validation guide | Procedimento de verificação por leitura das sete passagens e das duas âncoras, resultados esperados e declarações de não aplicabilidade. |

## Readiness Gate

- Spec esclarecida: `## Clarifications / ### Session 2026-11-19` com três respostas (uma por vez, R3 respeitado); zero marcadores pendentes; `checklists/requirements.md` em 18/18 na fase clarify — sem alteração nesta fase.
- Constitution Check I–VI reavaliado após o desenho: sem violações e sem exceções a justificar.
- Este plano não cria, antecipa ou modifica artefatos de fases posteriores; a existência futura de `checklists/gdd-amendment.md` e de `tasks.md` decorre das portarias 9 e 11, não deste documento.
- Próxima portaria: fase 9 (checklist), com `checklists/gdd-amendment.md` append-only e itens `- [ ] CHK###`, sem marcação `[x]` pelo gerador.
- Estado remoto, aprovação humana, CI, merge e proteção de branch permanecem `[NV]` até confirmação ao vivo.
