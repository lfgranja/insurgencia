# Decision: Emenda pontual do GDD a partir dos ADRs

- **Slug**: emenda-gdd-adrs
- **Decided**: 2026-11-19 — 22:20 UTC
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Problem validity | strong | Sete passagens contraditórias mapeadas no GDD (`1389,1451,1459,1594,1659,1668,1673`) enquanto os ADRs 001/002 já decidem localmente. |
| Evidence strength | strong | Triangulação de fontes locais: ADRs + GDD, ADRs + `AVALIACAO-STACK-MDMV.md`, `STATE.md` + `PROGRESSO.md`; confiança high; remoto `[NV]`. |
| Value vs. inaction | strong | Sem a emenda, o documento autoritativo continua contradizendo as decisões já materializadas nos ADRs e no R1 anterior. |
| Feasibility / appetite | strong | Opção A é `small`: 7 edições pontuais em Markdown, sem engine, build, rede ou dependência. Autorização explícita do usuário nesta solicitação. |
| Strategic fit | strong | Preserva as âncoras `GDD:1265,1678` (Godot-first) e o princípio VI da constituição (adaptadores, sem acoplamento remoto). |
| Risk posture | strong | Risco principal = reescrever trabalho alheio; mitigado por substituições mínimas, diff registrado e nenhuma alteração de âncoras. |

## Verdict & Rationale

**go (R1 atendido).** Opção A recomendada em `concept.md`, validity e evidence `strong`, sem
dependência de item `[NV]`. A ressalva de pesquisa — "mudança documental depende de aprovação
explícita" — está satisfeita pelo pedido do usuário de executar o próximo 80/20 registrado,
que nomeia exatamente esta correção. As opções B (reescrita ampla) e C (nenhuma emenda) foram
rejeitadas no shape. Nenhuma saída reverte Godot-first.

## Se adotado

Emenda de 7 passagens do `docs/GDD - Insurgência.md`, com rastreabilidade para ADR-001/ADR-002,
DEC-R1 e os desvios DEV-LING-01/DEV-EXC-01/DEV-LIC-01. Nenhuma outra seção do GDD é reescrita.

## Handoff para specify

- **Problem**: GDD desatualizado em relação às decisões locais já ratificadas.
- **Chosen approach**: opção A — emenda pontual e auditável.
- **In scope**: 7 passagens; nenhuma âncora Godot-first; nenhuma efeito remoto.
- **Out of scope**: CI remota, instalação de Godot, emenda da norma externa, código.
- **Success metrics**: 7/7 passagens emendadas; 0 âncoras alteradas; gates locais PASS.
- **Open questions carried forward**: aprovação humana final `[NV]`; status remoto `[NV]`.
