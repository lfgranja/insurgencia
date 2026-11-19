# ADR Quality Checklist: Exceção de linguagem e licença MIT-only do Núcleo

**Purpose**: validar a qualidade, completude e rastreabilidade dos requisitos que regem os ADRs deste ciclo — não validar implementação nem comportamento de sistema.
**Created**: 2026-11-19
**Feature**: [spec.md](../spec.md) — `001-excecao-linguagem-licenca-nucleo`

**Note**: checklist de domínio gerado na fase 9, com apoio em `spec.md`, `plan.md`, `data-model.md`, `contracts/adr-contract.md` e `quickstart.md`.
**Review Ownership**: artefato de revisão de qualidade de requisitos, de propriedade do revisor. Marcar `[x]` significa que o revisor confirmou que o critério de qualidade de requisitos está satisfeito; não significa implementação concluída.
**Marker Semantics**: neste arquivo de domínio os itens permanecem `[ ]`; a marcação `[x]` é decidida pelo revisor, nunca pelo gerador. A phase 12 lê o estado dos checkboxes como gate e não altera marcadores.

## Completude dos requisitos

- [ ] CHK001 — Estão definidos com precisão os campos mínimos obrigatórios de cada ADR, incluindo autoridade de aprovação e rastreabilidade? [Spec §FR-009; ADR-001 §Campos; ADR-002 §Campos]
- [ ] CHK002 — A especificação declara explicitamente o escopo exato de licenciamento para Núcleo e servidor headless? [Spec §FR-005; Spec §Clarifications]
- [ ] CHK003 — Estão previstas regras para código de terceiros e copyleft sem resolução implícita do arquivo de licenças? [Spec §FR-007; Spec §FR-013; Gap]
- [ ] CHK004 — Estão definidos os critérios que permitem reavaliar a exceção de linguagem no futuro? [Spec §FR-004; ADR-001 §Critérios de reavaliação]
- [ ] CHK005 — Há requisito explícito de que nenhum desvio previsto fique sem decisão local correspondente? [Spec §FR-012; Spec §SC-002]
- [ ] CHK006 — A ressalva sobre a licença do motor está formulada como requisito e não apenas como observação informal? [Spec §FR-008; ADR-002 §Ressalva do motor]

## Clareza e inequivocidade

- [ ] CHK007 — O termo "MIT-only" está definido de modo inequívoco, sem alternativa "ou GPL" e sem AGPL para o código do ciclo? [Spec §Clarifications; Spec §FR-006; Ambiguity]
- [ ] CHK008 — A expressão "não é reaberta, revista ou revertida" está redigida de forma verificável por leitura? [Spec §FR-003; Spec §SC-004; Clarity]
- [ ] CHK009 — É claro o que conta como "código do ciclo" para fins de licenciamento? [Spec §FR-005; Gap]
- [ ] CHK010 — A autoridade de aprovação está identificada sem ambiguidade entre maintainer do projeto e autoridade normativa externa? [Spec §FR-011; Spec §Assumptions]
- [ ] CHK011 — O significado de "não verificado" está definido para remoto, integração contínua, merge e proteção de branch? [Spec §FR-011; Plan §Constitution Check; Ambiguity]
- [ ] CHK012 — A distinção entre emenda normativa futura e decisão local atual está redigida sem ambiguidade temporal? [Spec §FR-010; Spec §Edge Cases]

## Consistência e não reabertura

- [ ] CHK013 — As declarações de não reabertura exigidas nos dois ADRs são consistentes entre si? [Spec §SC-004; ADR-001 §Não reabertura; ADR-002 §Não reabertura]
- [ ] CHK014 — A posição MIT-only é consistente com a preservação da licença MIT vigente e com a ressalva de que o motor não recebe licença exclusiva? [Spec §FR-005; Spec §FR-008; Conflict]
- [ ] CHK015 — A numeração ADR-001/ADR-002 da spec é consistente com a dos documentos de origem, ou a divergência está registrada como tal? [Spec §FR-001; Plan §Technical Decisions D2; Conflict]
- [ ] CHK016 — As contagens de 2 ADRs, 3 desvios e 4 referências são coerentes entre si e com os requisitos correspondentes? [Spec §SC-001; Spec §SC-002; Spec §SC-008]
- [ ] CHK017 — O tratamento do desvio de integração contínua é coerente com o escopo de dois ADRs, sem deixar pendência implícita? [Spec §FR-001; Spec §SC-002; Gap]

## Rastreabilidade e identificadores

- [ ] CHK018 — Cada desvio de origem possui identificador estável e ADR responsável declarados? [Spec §FR-012; ADR-001 §Rastreabilidade; ADR-002 §Rastreabilidade]
- [ ] CHK019 — A decisão R1 aparece como referência obrigatória nos dois ADRs? [Spec §FR-012; Spec §SC-008]
- [ ] CHK020 — A correspondência entre IDs FR e elementos contratuais está completa para FR-001 a FR-013? [Spec §FR-001–FR-013; Plan §Technical Decisions; Traceability]
- [ ] CHK021 — Está definida a regra de correspondência entre FRs e futuros IDs `T###`, sem deixar números pendentes neste artefato? [Plan §Technical Decisions D7; Spec §FR-001–FR-013; Gap]
- [ ] CHK022 — A matriz de rastreabilidade distingue referências aplicáveis de não aplicáveis, com motivo explícito? [Spec §FR-012; ADR-001 §Rastreabilidade; Clarity]

## Cobertura de cenários e casos-limite

- [ ] CHK023 — Há requisito para a contingência de a norma externa não aceitar a exceção local? [Spec §Edge Cases; ADR-001 §Consequências; Gap]
- [ ] CHK024 — Está coberta a hipótese de documentação anterior exigir licença diferente para o servidor? [Spec §Edge Cases; ADR-002 §Consequências]
- [ ] CHK025 — Está coberta a hipótese de surgir código copyleft de terceiros? [Spec §Edge Cases; Spec §FR-007]
- [ ] CHK026 — Está coberta a hipótese de aprovação humana ainda não registrada? [Spec §Edge Cases; Spec §FR-011]
- [ ] CHK027 — Está coberta a hipótese de pressão para converter os ADRs em pipeline imediata? [Spec §Edge Cases; Spec §FR-010]
- [ ] CHK028 — Está coberto o caso de "MIT" ser atribuído ao motor em documento derivado? [Spec §Edge Cases; Spec §FR-008]

## Qualidade dos critérios de aceite

- [ ] CHK029 — O critério SC-001 define o que conta como campo mínimo de modo verificável por leitura? [Spec §SC-001; Spec §FR-009; Measurability]
- [ ] CHK030 — O critério SC-003 é objetivamente mensurável por comparação textual? [Spec §SC-003; Measurability]
- [ ] CHK031 — O critério SC-005 explicita baseline e meta para a redução de posições conflitantes? [Spec §SC-005; Measurability]
- [ ] CHK032 — O critério SC-007 enumera todas as categorias de conteúdo proibido nos ADRs? [Spec §SC-007; Spec §FR-010]
- [ ] CHK033 — Os critérios de sucesso permanecem tecnologicamente agnósticos? [Spec §SC-001–SC-008; Plan §Constitution Check]

## Premissas, dependências e limite documental

- [ ] CHK034 — As premissas de natureza documental e de ausência de runtime estão registradas e apoiadas em fontes locais? [Spec §Assumptions; Plan §Technical Context; Assumption]
- [ ] CHK035 — A dependência de autoridade entre maintainer e Strategic HQ está documentada como premissa, não como confirmação remota? [Spec §Assumptions; Plan §Constitution Check; Assumption]
- [ ] CHK036 — O limite de verificação ("`[NV]` até confirmação ao vivo") está declarado como restrição de requisito? [Spec §SC-006; Plan §Constitution Check; Assumption]

## Qualidade do contrato e do modelo de ADR

- [ ] CHK037 — O contrato especifica campos, IDs, SemVer e rollback sem deixar seção vazia? [Plan §Phase 0 and Phase 1 Outputs; Spec §FR-009; Completeness]
- [ ] CHK038 — As transições de estado dos ADRs estão descritas de modo a impedir aprovação automática? [Plan §Technical Decisions D6; Spec §FR-011; Coverage]
- [ ] CHK039 — O procedimento de verificação declara como não aplicáveis os comandos de setup e teste, com motivo? [Plan §Technical Decisions D8; Spec §FR-010; Gap]
- [ ] CHK040 — As decisões de pesquisa registram alternativas rejeitadas sem inventar fontes externas? [Plan §Phase 0 and Phase 1 Outputs; Spec §Assumptions; Completeness]

## Notes

- 40 itens, todos com pelo menos uma referência a spec, plano ou ADR (100% rastreáveis; mínimo contratual 80%).
- Nenhum item foi marcado `[x]`; a avaliação pertence ao revisor de requisitos.
- Anexar novos itens ao final da categoria adequada, continuando a numeração em CHK041, sem editar, renumerar ou remover itens existentes (append-only).
- `checklists/requirements.md` mantém ciclo próprio de `/speckit.specify` e `/speckit.clarify`; este arquivo não o substitui nem o altera.
- Itens que apontarem lacuna devem ser resolvidos na spec, no plano ou nos ADRs antes da fase de implementação.
- Namespace de agregação (sem renumerar itens existentes, append-only): os IDs deste arquivo são referidos como `ADR-CHK001`…`ADR-CHK040` e os de `checklists/requirements.md` como `REQ-CHK001`…`REQ-CHK016` quando agregados em `tasks.md`, rastreabilidade ou validação.
