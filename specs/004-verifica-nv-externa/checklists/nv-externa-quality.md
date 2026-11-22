# Verificação NV Externa — Requirements Quality Checklist: Verifica NV externa

**Purpose**: validar a qualidade, completude e rastreabilidade dos requisitos que regem a substituição de marcadores `[NV]` por fatos verificados ao vivo — não validar a implementação da substituição nem o resultado da edição dos documentos.
**Created**: 2026-11-19
**Feature**: [spec.md](../spec.md) — `004-verifica-nv-externa`

**Note**: checklist de domínio gerado na fase 9, com apoio em `spec.md`, `plan.md`, `research.md` e nos três arquivos de `research-evidence/`.
**Review Ownership**: artefato de revisão de qualidade de requisitos, de propriedade do revisor. `[x]` significa que o revisor confirmou que o critério de qualidade de requisitos está satisfeito; não significa implementação concluída.
**Marker Semantics**: os itens permanecem `[ ]`; a aprovação é decidida pelo revisor, nunca pelo gerador. A fase de implementação lê o estado dos checkboxes como gate e não altera marcadores.

## Completude dos requisitos

- [ ] CHK001 — Estão definidos com precisão os 12 fatos NV-A01–NV-A12 e seus destinos arquivo/linha, sem deixar o inventário implícito? [Spec §FR-001, §FR-013; Plan §Inventory; Completeness]
- [ ] CHK002 — É explícito que cada substituição exige título, URL, data e pelo menos duas fontes independentes no próprio destino? [Spec §FR-002, §SC-002; Plan §D2; Completeness]
- [ ] CHK003 — Estão enumeradas as ressalvas de escopo de cada fato, incluindo as quatro hipóteses automáticas da marca Godot e a regra do fork comercial? [Spec §FR-003, §SC-008; Research §NV-A01; Completeness]
- [ ] CHK004 — Há requisito explícito proibindo afirmação de benchmark, build, protótipo, custo por tick ou falha empírica de engine? [Spec §FR-004, §SC-003; Constitution VI; Completeness]
- [ ] CHK005 — A ressalva de natureza documental da issue Stride #1666 está especificada de modo a impedir leitura de teste empírico? [Spec §Clarifications Q1, §Edge Cases; Research §NV-A03; Gap]
- [ ] CHK006 — Estão listados, um a um, os marcadores de classe C que devem permanecer, incluindo reconexão, hash, snapshot, RCE e lógica de PNJs? [Spec §FR-005; Research §Data & Constraints; Completeness]
- [ ] CHK007 — Estão listados os marcadores de classe B que devem permanecer, incluindo CI, merge, publicação, aprovação remota e proteção de branch? [Spec §FR-006; Constitution VI; Completeness]
- [ ] CHK008 — A distinção entre documento integrado (substituição no corpo) e snapshot histórico (nota de superação) está definida por caminho, não por adjetivo? [Spec §FR-007, §FR-008, §Clarifications Q2; Plan §D5; Completeness]
- [ ] CHK009 — Está especificado que notas, vetores, índices e recomendações não são recalculados pela substituição? [Spec §FR-009, §SC-007; Plan §D6; Completeness]
- [ ] CHK010 — Há requisito de não alteração de governança, ADRs, histórico datado e `converge.md`, com a contagem canônica preservada? [Spec §FR-010; nv-resolution §Fronteiras; Completeness]
- [ ] CHK011 — Está definido o destino de afirmações não cobertas pelas três evidências, inclusive termos de serviços proprietários? [Spec §FR-011, §Edge Cases; Research §NV-A12; Gap]

## Clareza e inequivocidade

- [ ] CHK012 — É inequívoco o significado de "verificado ao vivo" e por que a data 2026-11-19 é de acesso, não de publicação da fonte? [Spec §FR-012; Research §Síntese; Clarity]
- [ ] CHK013 — A contagem local de 12 fatos está claramente separada da contagem canônica A=11/B=92/C=157, sem sugerir que uma substitui a outra? [Spec §Edge Cases, §Assumptions; Plan §D1; Ambiguity]
- [ ] CHK014 — É claro que "sem correção na 4.4" significa estado aberto e ausência de registro de fix, e não falha observada? [Spec §Clarifications Q1; Research §NV-A03; Ambiguity]
- [ ] CHK015 — A expressão "4 versões por ciclo do Bevy" está quantificada e limitada ao ciclo 0.19, com a distinção entre publicações e séries menores explicitada? [Spec §Clarifications Q3; Research §NV-A11; Clarity]
- [ ] CHK016 — É inequívoco que `license: null` na API do GitHub para Dviglo não significa ausência de licença? [Spec §FR-003, §Edge Cases; Research §NV-A07; Clarity]
- [ ] CHK017 — O termo "nota de superação" está definido como apêndice append-only, distinto de correção do corpo datado? [Spec §FR-007; Plan §Templates; Ambiguity]
- [ ] CHK018 — Fica claro que a verificação é contagem documental, sem gates, build, stage ou commit? [Spec §FR-014, §SC-009; Plan §Verificação; Clarity]

## Consistência e não contradição

- [ ] CHK019 — A proibição de empirismo (FR-004) é coerente com a ressalva de Stride #1666 e com a ausência de alegações de versão/performance do Fyrox? [Spec §FR-004, §Clarifications Q1; Research §NV-A03, §NV-A05; Consistency]
- [ ] CHK020 — A exigência de duas fontes (FR-002/SC-002) é coerente com todas as linhas da síntese de 12 fatos em `research.md`? [Spec §FR-002, §SC-002; Research §Síntese; Consistency]
- [ ] CHK021 — A regra "nota em `pesquisa/`, substituição nos demais" é coerente entre spec, plano e matriz arquivo→ação? [Spec §FR-007, §FR-008; Plan §Matriz arquivo → ação; Consistency]
- [ ] CHK022 — A decisão `go` da opção A é coerente com a rejeição da reescrita integral e da errata centralizada? [Spec §Assumptions; decision.md §Scorecard, §Verdict; Conflict]
- [ ] CHK023 — Os critérios mensuráveis cobrem de forma coerente fatos, fontes, ressalvas, salvaguardas e integridade de artefatos, sem sobrepor contagens incompatíveis? [Spec §SC-001–SC-010; requirements.md CHK006–CHK010; Consistency]

## Cobertura de cenários e casos-limite

- [ ] CHK024 — Há cobertura para o caso de fato já citado no destino, exigindo acréscimo de ressalva em vez de remoção da citação? [Spec §Edge Cases 1; FR-003; Gap]
- [ ] CHK025 — A afirmação negativa "sem EULA" está quantificada como limitada às fontes primárias oficiais, e não como prova universal? [Spec §Edge Cases 2; Research §NV-A02; Gap]
- [ ] CHK026 — Os cenários de aceitação cobrem separadamente Godot, Stride/Fyrox/Dviglo/Defold, Lightyear/Bevy e salvaguardas, permitindo teste independente? [Spec §US1–US4; requirements.md CHK016; Coverage]
- [ ] CHK027 — Está previsto o caso de a substituição parecer exigir recálculo de nota, com regra explícita de não recalcular? [Spec §Edge Cases 8; FR-009; Edge Case]
- [ ] CHK028 — A divergência de versão do Fyrox entre corpus e evidência tem critério de resolução sem alterar vetor nem índice? [Spec §US2/AC4; Plan §D6; Coverage]

## Rastreabilidade e prontidão

- [ ] CHK029 — Existe esquema de IDs estável (NV-Axx, FR-xxx, SC-xxx, CHKxxx, Txxx) que permita rastrear cada fato do inventário ao requisito e à tarefa? [Spec §FR-013; Plan §Inventory; Traceability]
- [ ] CHK030 — As três clarificações estão registradas uma por vez, com 0 perguntas ao usuário e R3 respeitado? [Spec §Clarifications; requirements.md CHK017; Traceability]
- [ ] CHK031 — A spec evita antecipar tarefas, checklists ou código, mantendo o plano como único local de decisão técnica? [Spec §Contexto e Orçamento; Plan §Decisões de design; Traceability]

## Notes

- 31 itens de qualidade de requisitos, todos com referência rastreável (100% ≥ 80% exigido pelo contrato).
- Itens em `[ ]` aguardam avaliação do revisor; nenhum item foi marcado como aprovado pelo gerador.
- Esta checklist testa a qualidade do que está **escrito** na spec/plano/research; não verifica se a substituição foi aplicada aos documentos de destino.
