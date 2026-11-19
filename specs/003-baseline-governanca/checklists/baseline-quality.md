# Baseline Governance Requirements Quality Checklist: Baselines locais de governança

**Purpose**: validar a qualidade, completude e rastreabilidade dos requisitos que regem a materialização dos três baselines locais — não validar implementação, comportamento de gates ou resultado de execução.
**Created**: 2026-11-19
**Feature**: [spec.md](../spec.md) — `003-baseline-governanca`

**Note**: checklist de domínio gerado na fase 9, com apoio em `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/baseline-bootstrap-contract.md` e `quickstart.md`.
**Review Ownership**: artefato de revisão de qualidade de requisitos, de propriedade do revisor. Marcar um item como aprovado significa que o revisor confirmou que o critério de qualidade de requisitos está satisfeito; não significa implementação concluída.
**Marker Semantics**: neste arquivo de domínio os itens permanecem `[ ]`; a marcação de aprovado é decidida pelo revisor, nunca pelo gerador. A fase de implementação lê o estado dos checkboxes como gate e não altera marcadores.

## Completude dos requisitos

- [ ] CHK001 — Estão definidos com precisão os três baselines locais e seus caminhos canônicos, um por gate? [Spec §FR-001–FR-003; Contract §1; Completeness]
- [ ] CHK002 — A spec declara explicitamente que os baselines devem derivar do estado corrente do alvo e nunca da factory? [Spec §FR-005, §Assumptions; Research §RQ-5; Completeness]
- [ ] CHK003 — Estão especificadas as chaves exatas de cada baseline (`allow_missing`, `files`/`schema_version`/`enforce_unbaselined`, `allow_duplicates`) sem deixar formato implícito? [Spec §FR-001–FR-003; Contract §2; Data-model §Visão geral; Completeness]
- [ ] CHK004 — Há requisito explícito de proveniência local em cada baseline, com origem, método, data e não uso da factory? [Spec §FR-004; Data-model §Entidade Proveniência local; Research §RQ-6; Completeness]
- [ ] CHK005 — Estão definidos os critérios de PASS para os três gates, incluindo contadores e ausência de `stale`? [Spec §FR-006–FR-008, §SC-003; Contract §4.1; Completeness]
- [ ] CHK006 — Está previsto requisito para o caso de estado corrente sem entradas, sem inventar itens para inflar contagens? [Spec §FR-009, §SC-004; Research §RQ-7; Gap]
- [ ] CHK007 — Está especificado o critério que separa gap legítimo de defeito a corrigir antes de baselinar? [Spec §FR-010, §Clarifications Q3; Research §RQ-8; Completeness]
- [ ] CHK008 — Há requisito explícito de imutabilidade dos scripts, schemas, limiares, caminhos e regex? [Spec §FR-011, §SC-006; Plan §D7; Completeness]

## Clareza e inequivocidade

- [ ] CHK009 — É inequívoco o que significa "estado corrente" e por que ele prevalece sobre contagens históricas divergentes? [Spec §Clarifications Q1; Research §RQ-5; Ambiguity]
- [ ] CHK010 — A expressão "verde vazio" está caracterizada de modo a distinguir não-regressão de ausência de estado? [Spec §US1, §US3; Problem §Problem Statement; Clarity]
- [ ] CHK011 — É claro que a derivação para completude e unicidade usa `regressions` do `--json` sem baseline, e por que a ordem antes/depois da gravação é obrigatória? [Spec §FR-002, §FR-003; Contract §3.1; Research §RQ-4; Clarity]
- [ ] CHK012 — O termo `baselined` está definido por gate (`baselined_features`, `baselined_files`, `baselined_specs`) sem ambiguidade entre contadores? [Spec §FR-006–FR-008, §SC-004; Data-model §Visão geral; Clarity]
- [ ] CHK013 — Fica claro que `schema:"047.2.0"` e `schema:"087.1.0"` são do resumo do gate, não campos dos arquivos de completude e unicidade? [Data-model §Campos não aplicáveis; Contract §2.2, §2.3; Research §RQ-2, §RQ-3; Ambiguity]
- [ ] CHK014 — O significado de `stale_baseline` está explicitado como sinal de re-baselinação, e não como falha a suprimir? [Spec §FR-005–FR-008; Contract §4.2; Plan §D9; Clarity]
- [ ] CHK015 — A expressão `[NV]` está definida para CI, merge, publicação, aprovação e proteção de branch? [Spec §FR-012, §SC-007; Contract §6; Constitution VI; Clarity]
- [ ] CHK016 — É inequívoco o limite entre comandos descritos como procedimento e saída executada colada na portaria de evidência? [Spec §Contexto e Orçamento; Quickstart §Comandos de teste; Contract §6; Ambiguity]

## Consistência e não contradição

- [ ] CHK017 — Os três formatos exigidos pela spec são consistentes com os formatos lidos pelos scripts e com o contrato? [Spec §FR-001–FR-003; Contract §2; Research §RQ-1–RQ-3; Consistency]
- [ ] CHK018 — A política de não inventar entradas é coerente entre FR-003, FR-009, SC-002 e SC-004? [Spec §FR-003, §FR-009, §SC-002, §SC-004; Research §RQ-7; Conflict]
- [ ] CHK019 — O critério de gap legítimo é coerente entre a clarificação Q3, os casos-limite e o critério de aceite de `release`/`validate` da feature 003? [Spec §Clarifications Q3, §Edge Cases, §SC-004; Contract §3.3; Consistency]
- [ ] CHK020 — As contagens esperadas (uma entrada em completude, nenhuma nas outras duas) são coerentes entre spec, research, data-model, contrato e quickstart? [Spec §SC-004; Research §RQ-5; Data-model §Entidade Baseline de completude; Contract §3.3; Quickstart §Etapa 6; Consistency]
- [ ] CHK021 — A exigência de imutabilidade dos scripts é coerente com o uso de campos extras de proveniência ignorados pelos carregadores? [Spec §FR-004, §FR-011; Research §RQ-6, §RQ-9; Plan §D3; Conflict]
- [ ] CHK022 — A hierarquia de prioridades das três histórias é coerente com o risco e com a dependência de proveniência? [Spec §US1, §US2, §US3; Contract §1; Consistency]
- [ ] CHK023 — A posição de que a factory é fonte, não autoridade, é coerente entre spec, assessment e constituição? [Spec §FR-005; Decision §Verdict; Constitution IV; Conflict]

## Rastreabilidade e identificadores

- [ ] CHK024 — Cada baseline possui identificador estável (`BL-CONV`, `BL-COMP`, `BL-UNIQ`) e caminho canônico correspondente? [Contract §1; Spec §FR-001–FR-003; Traceability]
- [ ] CHK025 — A associação FR → gate → cenário de regressão está completa para os FRs com comportamento de gate (FR-001, FR-002, FR-003, FR-006, FR-007, FR-008), sendo os demais (FR-004, FR-005, FR-009, FR-010, FR-011, FR-012) verificados por leitura/comparação documental, sem associação a gate? [Spec §FR-001–FR-003, §FR-006–FR-008; Contract §4.3, §8; Traceability]
- [ ] CHK026 — Os critérios de sucesso SC-001–SC-008 rastreiam para requisitos funcionais sem lacuna? [Spec §SC-001–SC-008 ↔ §FR-001–FR-012; Checklists/requirements.md CHK015; Traceability]
- [ ] CHK027 — Os cenários `RG-1`…`RG-5` possuem assinatura de recusa inequívoca e ID estável no contrato e no quickstart? [Contract §4.3; Quickstart §Cenários de verificação; Spec §SC-008; Traceability]
- [ ] CHK028 — A distinção entre autoridade local (`DEC-R1`), dívida (`DIV-1`) e achado (`BUG-###`) está documentada? [Contract §1, §5.2, §5.3; Spec §Input; Clarity]
- [ ] CHK029 — A rastreabilidade das três clarificações para FRs e decisões de pesquisa está estabelecida? [Spec §Clarifications; Research §RQ-5, §RQ-7, §RQ-8; Traceability]
- [ ] CHK030 — A numeração do contrato (§1–§8) é referenciada de forma consistente por data-model, quickstart e plano? [Contract §1–§8; Data-model §Visão geral; Quickstart §Etapas; Plan §Phase 0 and Phase 1 Outputs; Traceability]

## Cobertura de cenários e casos-limite

- [ ] CHK031 — Está coberta a hipótese de o estado corrente estar limpo para um gate, com mapa vazio e `baselined=0`? [Spec §Edge Cases, §FR-009; Research §RQ-7; Coverage]
- [ ] CHK032 — Está coberta a hipótese de chave da factory ou chave órfã aparecer em algum baseline? [Spec §Edge Cases, §FR-005; Contract §1, §5.3; Gap]
- [ ] CHK033 — Está coberta a hipótese de contagens históricas divergirem no ato da materialização? [Spec §Edge Cases, §Clarifications Q1; Research §RQ-5; Coverage]
- [ ] CHK034 — Está coberta a hipótese de um gap de portaria futura ser confundido com defeito de artefato já exigido? [Spec §Clarifications Q3, §Edge Cases; Research §RQ-8; Coverage]
- [ ] CHK035 — Está coberta a hipótese de apêndice legítimo em `converge.md` preservar o prefixo baselinado? [Spec §Edge Cases; Contract §2.1; Data-model §Entidade Baseline de convergência; Coverage]
- [ ] CHK036 — Está coberta a hipótese de um item baselinado deixar de ser necessário e virar `stale`? [Spec §Edge Cases; Contract §4.2; Research §RQ-9; Gap]

## Qualidade dos critérios de aceite

- [ ] CHK037 — SC-001 explicita baseline e meta para os três arquivos de baseline presentes? [Spec §SC-001; Measurability]
- [ ] CHK038 — SC-003 e SC-004 definem objetivamente o que conta como PASS, `baselined>0` e `stale=0`? [Spec §SC-003, §SC-004; Contract §4.1; Acceptance Criteria]
- [ ] CHK039 — SC-005 torna mensurável a ausência de chaves da factory e de entradas órfãs? [Spec §SC-005; Contract §8 item 2; Measurability]
- [ ] CHK040 — SC-006 é verificável por comparação textual dos três scripts? [Spec §SC-006; Quickstart §Critérios de encerramento; Measurability]
- [ ] CHK041 — SC-008 define a recusa de regressão controlada como resultado observável, com assinaturas por gate? [Spec §SC-008; Contract §4.3; Acceptance Criteria]

## Premissas, dependências e limites documentais

- [ ] CHK042 — As premissas sobre ausência de runtime, banco, CI e serviço externo estão registradas e apoiadas em fontes locais? [Spec §Assumptions; Plan §Technical Context; Assumption]
- [ ] CHK043 — A dependência da feature em relação à decisão R1 e à dívida 1 está documentada sem afirmar aprovação remota? [Spec §Input, §Assumptions; Decision §Verdict; Contract §1; Assumption]
- [ ] CHK044 — O limite "até confirmação ao vivo" está formulado como restrição de requisito e não como promessa de verde? [Spec §FR-012, §SC-007; Contract §6; Constitution VI; Assumption]
- [ ] CHK045 — A precedência da governança do alvo sobre a factory está declarada como premissa? [Spec §FR-005; Decision §Scorecard/Strategic fit; Constitution §Normas de trabalho; Dependency]
- [ ] CHK046 — A ressalva de que nenhum segredo, token ou credencial é necessário está registrada? [Spec §Assumptions; Contract §7; Constitution §Normas de trabalho; Assumption]

## Qualidade do contrato, do modelo, da pesquisa e do guia

- [ ] CHK047 — O contrato especifica formatos, comandos, validações, ratchet, rollback e critérios de conformidade sem deixar seção vazia? [Plan §Phase 0 and Phase 1 Outputs; Contract §1–§8; Completeness]
- [ ] CHK048 — As transições de estado dos três baselines impedem reescrita silenciosa e exigem re-baselinação explícita? [Data-model §Entidade Baseline de convergência, §Entidade Baseline de completude, §Entidade Baseline de unicidade; Contract §4.2; Coverage]
- [ ] CHK049 — O guia declara como não aplicáveis setup de instalação, build e serviços, com motivo, sem suprimir os comandos de bootstrap? [Quickstart §Comandos de setup, §Comandos de teste; Plan §D1; Gap]
- [ ] CHK050 — As decisões de pesquisa registram alternativas rejeitadas e evidência contrária sem inventar fontes externas? [Research §Decisões de pesquisa, §Evidência contrária; Spec §Assumptions; Completeness]

## Notes

- 50 itens, todos com pelo menos uma referência a spec, plano, pesquisa, contrato, modelo, guia, decisão, assessment, constituição ou checklist (100% rastreáveis; mínimo contratual 80%).
- Nenhum item foi marcado como aprovado; a avaliação pertence ao revisor de requisitos.
- Anexo somente ao final da categoria adequada, continuando a numeração em CHK051, sem editar, renumerar ou remover itens existentes (append-only).
- `checklists/requirements.md` mantém ciclo próprio de `/speckit.specify` e `/speckit.clarify`; este arquivo não o substitui nem o altera.
- Itens que apontarem lacuna devem ser resolvidos na spec, no plano ou no contrato antes da fase de implementação.
- Namespace de agregação (sem renumerar itens existentes): os IDs deste arquivo são referidos como `BQ-CHK001`…`BQ-CHK050` quando agregados em `tasks.md`, rastreabilidade ou validação.
