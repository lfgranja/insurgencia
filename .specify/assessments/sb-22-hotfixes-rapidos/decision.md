# Decision — Hotfixes rápidos

- **Slug**: sb-22-hotfixes-rapidos
- **Decided**: 2026-11-21
- **Veredito**: ADOPT
- **Artifacts reviewed**: intake.md; research.md; problem.md; concept.md

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Correções precisam de rastreabilidade; hoje nada garante tag, notas e gate antes do merge. |
| Força da evidência | strong | Nove updates em 3 dias documentados, orientação direta da Valve e canal CI→`gh` existente no repo. |
| Valor vs. inação | strong | Sem disciplina, "rápido" vira hotfix cego, regressão e perda de proveniência. |
| Viabilidade / Appetite | strong | `verify.sh`, CI e releases já existem; falta só disciplina documental. |
| Alinhamento estratégico | strong | Casa com CI verde → merge autônomo, proveniência aberta e "nada é verde sem comando" (I/VI). |
| Postura de risco | adequate | Risco de virar corrida por contagem de patches ou resposta integral a reviews; mitigado por triagem e limite de tempo. |

## Racional

ADOPT, com guardrail: adota-se a **cadência de patches pequenos com notas em CHANGELOG/GitHub Releases e gate verde executado**, não a promessa de hotfix cego. O caso estudado publicou nove updates em três dias, com notas públicas e origem nos relatos dos jogadores, e a própria Valve recomenda corrigir rápido, anunciar no hub e testar em branch — mas reserva "Update Visibility Rounds" (5 por produto) para grandes updates, não para cada patch. No repo, `verify.sh` (com `--all-targets`), CI e merge autônomo via `gh` já estão prontos: a mudança é documental. Triagem por impacto (bloqueante > regra > cosmético) e limite de tempo para leitura de reviews evitam a armadilha de responder 1.598 avaliações. Vendas, score de 79% e datas 12/11 vs 13/11 permanecem observações de terceiros/dualidade legítima, não critérios de verde.

## Condição/responsável

- Responsável: Main (tag, notas e saída de `verify.sh`) + SE (merge via CI verde e triagem de reviews).
- Condição de validade: nenhum patch mergeado sem `verify.sh` executado e saída colada, tag semver e notas curtas publicadas. Se o atalho "publicar e verificar depois" aparecer, a adoção converte em AVOID para aquele fluxo.

## Handoff

- **Problema**: correções rápidas sem tag, notas e gate executado geram proveniência frágil e risco de regressão.
- **Abordagem**: patch mínimo por correção; tag semver; notas curtas em CHANGELOG/GitHub Releases (o que mudou, por quê, como verificar); `verify.sh` executado com saída colada antes do merge autorizado por CI verde; triagem bloqueante > regra > cosmético; poucas reviews respondidas, com limite de tempo.
- **In**: tags e notas, saída executada do gate, registro de severidade, canal CI→`gh` existente.
- **Out**: promessa de SLA/hotfix cego, publicar sem gate, responder toda review, pedir avaliação dentro do jogo, usar vendas/score como prova local, Update Rounds em cada patch, alterações em gates/CI/constituição.
- **Métricas**: 100% dos patches com tag+notas+saída antes do merge (baseline: sem disciplina); nenhuma regressão detectada pelo gate após hotfix (baseline: não medido); tempo de triagem registrado sem meta cega; zero "verde" sem comando executado.
- **Perguntas**: quem assina tag/notas; qual limite de tempo/quantidade para reviews; CHANGELOG único ou notas por release.
