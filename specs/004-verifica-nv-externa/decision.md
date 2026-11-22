# Decision: Substituir marcadores `[NV]` de pesquisa por fatos verificados ao vivo

- **Slug**: verifica-nv-externa
- **Decided**: 2026-11-19
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | intake/problem: rótulos `[NV]` sobre fatos já verificados em três arquivos de evidência; dívida explícita na resolução de 19/11/2026 (`nv-resolution-2026-11-19.md:15`). |
| Evidence strength | strong | research: 12 fatos, cada um com ≥2 fontes primárias com título/URL/data; nenhum repousa em fonte única (confirmed nos três arquivos de evidência). |
| Value vs. inaction | adequate | problema: sem a aplicação, leitores cruzam arquivos de evidência e o risco de alegação empírica injustificada permanece; benefício é auditabilidade, não ganho funcional. |
| Feasibility / appetite | adequate | concept: opção A, appetite medium, 12 fatos, duas modalidades de escrita, sem código, sem novas buscas, sem gates. |
| Strategic fit | strong | constituição Princípio VI (verificação honesta) e contrato de resolução de `[NV]`; nenhuma portaria pulada (intake→research→define→shape→decide). |
| Risk posture | adequate | riscos nomeados e mitigados: ressalva obrigatória, classificação A/B/C, notas de superação append-only, proibição de benchmark e de recálculo de scores; risco residual de redação involuntária em snapshot é coberto por checklist e tasks de salvaguarda. |

R1 verificado: `problem.md` presente; `concept.md` presente; validade `strong` (≥adequate); evidência `strong` (nunca weak/unknown); opção recomendada nomeada (A). Veredito `go` legítimo.

## Verdict & Rationale

**go** para a opção A. A evidência é primária, datada e duplicada por fato; o problema é real e circunscrito a documentos; a constituição exige exatamente esse tipo de honestidade documental. A opção B é rejeitada por reescrever histórico e recalcular scores, e a C por deixar os marcadores no corpo, contrariando o objeto da feature. Nenhum `unknown` é glossado: o único ponto de atenção é a distinção entre 12 fatos locais e a contagem canônica A=11/B=92/C=157, tratada como registro datado que esta feature não altera (ver `## If go — Handoff`).

## If needs-clarification

Não aplicável — nenhuma pergunta bloqueante restou. As três ambiguidades de `problem.md` foram resolvidas por evidência local e registradas em `spec.md` / `## Clarifications` / `### Session 2026-11-19`, uma por vez, respeitando R3 (≤3).

## If go — Handoff to `/speckit.specify`

- **Problem**: marcadores `[NV]` sobre fatos já verificados ao vivo permanecem nos documentos integrados do hub, e uma substituição sem ressalva geraria alegações empíricas não sustentadas.
- **Chosen approach**: opção A — substituição cirúrgica com título/URL/data, ≥2 fontes e ressalvas; snapshots históricos recebem nota de superação append-only.
- **In scope / out of scope**: dentro — 12 fatos NV-A01–NV-A12 nos documentos listados no pedido, com preservação integral de classes B/C; fora — benchmarks/execução, reescrita de snapshots e de governança, recálculo de notas, novas URLs, gates, stage/commit.
- **Success metrics**: 12/12 fatos resolvidos; 0 fatos sem 2 fontes; 0 alegações de benchmark; 100% de marcadores B/C preservados; 100% de snapshots com nota de superação e 0 linhas reescritas; 0 notas/índices/recomendações alterados; 10/10 artefatos sem seção vazia nem TBD.
- **Carried-forward open questions**: nenhuma; as três respostas estão em `spec.md` / `## Clarifications`.
