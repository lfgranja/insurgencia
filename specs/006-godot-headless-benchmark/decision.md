# Decision: Godot headless benchmark

- **Slug**: godot-headless-benchmark
- **Decided**: 2026-11-20
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | intake/problem/research: `AGENTS.md:3-5` registra ausência de engine e de medição no alvo; o pedido do mantenedor pede prova de proveniência e linha de base, e a carga sintética já versionada não é aproveitada como evidência. O dano é a incapacidade de auditar a origem do binário e de detectar regressões de custo por tick. |
| Evidence strength | strong | research cita comando `gh api repos/godotengine/godot/releases/tags/4.7.2-stable`, `CHECKSUM_OK` em 2026-11-20T00:43:52Z, binário `4.7.2.stable.official.ed1daf0bf` em 00:44:11Z e mediana de 37,48 µs/tick em 7 amostras em 00:44:48Z, todos com carimbo; a natureza sintética da carga vem de `tests/benchmark/headless_tick.gd:3-19`, lido localmente. Nenhum critério depende de `ASSUMPTION` ou de fato remoto ainda não observado. |
| Value vs. inaction | strong | problema: sem verificação e sem medição, a instalação repousa em confiança informal e o desempenho da simulação fica sem referência; com elas, obtêm-se cadeia de integridade e linha de base comparável por custo baixo. |
| Feasibility / appetite | strong | concept: Opção A, appetite small; comandos já executados e carga já versionada, sem desenvolvimento de produto e sem dependência de serviço externo no caminho crítico. |
| Strategic fit | strong | constituição I (determinismo), III (fluxo ordenado), IV (sem dependência proprietária no caminho crítico) e VI (verificação honesta); `AGENTS.md` define o padrão de evidência com comando, saída e data/hora. |
| Risk posture | adequate | riscos nomeados em `research.md §Evidence Against the Idea`: carga sintética mal citada como partida real → rotulagem obrigatória; dispersão entre amostras → mediana, mínimo, máximo e contagem registrados, sem SLA; checksum não prova identidade de mantenedor → origem citada e conteúdo externo tratado como dado; evidência do orquestrador não reexecutada na etapa → distinção explícita entre citação e verificação ao vivo. |

R1 verificado: `problem.md` presente; `concept.md` presente; validade `strong` (≥adequate); evidência `strong` (nunca `weak`/`unknown`); opção recomendada nomeada (Opção A). Nenhum `unknown` é glossado: a não reexecução nesta etapa e a natureza sintética da carga são reconhecidas e transformadas em requisitos de registro, não em afirmação de resultado de jogo.

## Verdict & Rationale

**go** para a Opção A. O problema é real, circunscrito e sustentado por evidência com comando e carimbo; a medição abaixo da referência de 16,7 ms/tick é um dado de comparação, não um triunfo a ser supervalorizado. A decisão registra explicitamente que a carga é simulação headless sintética de 600 ticks × 64 entidades e que nenhum artefato deve convertê-la em benchmark 5v5 real. A Opção B é preterida porque abriria mão da cadeia de integridade, e a Opção C não produz linha de base. O handoff abaixo limita-se ao que `/speckit.specify` precisa; nenhuma implementação, reexecução ou portaria posterior a tasks é antecipada aqui.

## If needs-clarification

Não aplicável — nenhuma pergunta bloqueante restou. As três ambiguidades de `problem.md` foram resolvidas uma por vez por evidência local, pelo pedido autorizado e pela política de rotulagem, e registradas em `spec.md` / `## Clarifications` / `### Session 2026-11-20`, respeitando R3 (≤3 esclarecimentos).

## If go — Handoff to `/speckit.specify`

- **Problem**: o alvo não tem engine instalada nem medição de custo por tick, e carece de prova de que o binário usado vem de release oficial íntegra.
- **Chosen approach**: Opção A — release `4.7.2-stable` identificada, checksum esperado/calculado idêntico, binário `4.7.2.stable.official.ed1daf0bf` reportado, carga sintética de 600 ticks × 64 entidades medida e rotulada como tal.
- **In scope / out of scope**: dentro — identidade da release, par de checksums, identidade do binário, medição com estatísticas, referência de 16,7 ms/tick, rotulagem sintética, evidência com data/hora, dez artefatos rastreáveis; fora — reclassificação como 5v5 real, medição de partida/renderização/rede, SLA, gates, stage/commit/PR/merge nesta etapa, portarias posteriores a tasks.
- **Success metrics**: 1 release nomeada; 1 par de checksums idêntico com `CHECKSUM_OK`; 1 string de versão registrada; 1 execução de 600×64; mediana 37,48 µs/tick em 7 amostras com mín 22,71 e máx 42,95; 0,03748 ms/tick abaixo de 16,7 ms/tick; 100% de rotulagem sintética e 0 afirmações de 5v5 real; 10 de 10 artefatos sem seção vazia nem marcador de pendência.
- **Carried-forward open questions**: nenhuma; as três respostas estão em `spec.md` / `## Clarifications`. A instalação, a reexecução do benchmark e as portarias R4/R7 dependem de fases posteriores e não são afirmadas como concluídas por esta decisão.
