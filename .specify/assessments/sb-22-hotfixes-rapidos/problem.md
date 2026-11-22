# Problem Definition: Hotfixes rápidos

- **Slug**: sb-22-hotfixes-rapidos
- **Created**: 2026-11-21
- **Inputs used**: intake.md + research.md (steam-gtm, alternativas, fit-local)

## Problem Statement

O relato de "correções rápidas após o lançamento, na sequência de uma fila de avaliações" sugere velocidade como prática. A Insurgência já tem os meios: `verify.sh`, CI com gates e merge autônomo via `gh`. O que não existe é **disciplina de rastreabilidade** — uma correção pode entrar sem tag, sem notas e sem saída executada do gate, transformando "rápido" em "cego". Além disso, ler e responder toda review não escala (1.598 reviews no caso; a própria Valve recomenda limitar o tempo de leitura e não responder todas). O problema concreto é garantir que correções pequenas sejam publicadas com proveniência e verificação honesta, sem prometer SLA cego nem confundir "review score" com verificação (79% é instantâneo de terceiro, não comando executado).

## Affected Users & Stakeholders

- **Users**: jogadores que encontram um bug e precisam ver que foi corrigido, com nota datada — [source: steam-gtm achado 8]
- **Users**: engenheiros sem estúdio (GDD §3.2), que não podem pagar o custo de uma regressão não detectada — [source: fit-local]
- **Stakeholders**: autoridade do gate, que não pode chamar "verde" o que não foi executado (constituição VI) — [source: fit-local achado 6]
- **Stakeholders**: SE, que faz o merge autônomo quando o CI está verde e precisa de critério de triagem — [source: fit-local CI/CD]

## Goals

- Publicar cada correção como patch pequeno, com tag semver, notas curtas em CHANGELOG/GitHub Releases e saída executada de `verify.sh`.
- Triagem por impacto: bloqueante > regra > cosmético; responder poucas reviews, com moderação e limite de tempo.
- Manter o merge autônomo condicionado a CI verde, sem atalhos.
- Separar "prática do caso" (9 updates/3 dias) de promessa de SLA que o projeto não pode cumprir.

## Non-Goals

- Prometer hotfix em tempo fixo, "corrigir tudo que aparecer" ou responder toda review.
- Publicar sem gate verde executado, sem notas ou sem tag.
- Usar vendas, score de reviews ou pico de jogadores de terceiros como prova de sucesso local.
- Confundir Update Visibility Rounds (5 por produto, para grandes updates) com cadência de hotfix.
- Alterar `verify.sh`, CI, gates ou a constituição; criar Steam page/App ID.

## Success Metrics

- Cada correção com tag + notas + saída de `verify.sh` colada antes do merge (baseline: sem disciplina registrada).
- Tempo de triagem registrado por severidade, sem meta de SLA cego (baseline: não medido).
- Nenhuma regressão reintroduzida por hotfix detectada pelo gate (baseline: não medido; `verify.sh` cobre `--all-targets`).
- Número de reviews respondidas limitado e explícito (baseline: prática do dev de ler todas, não escalável).
- Zero afirmações de "verde" sem comando executado (baseline: regra vigente; manter).

## Cost of Inaction

Sem tag/notas/gate, uma correção rápida vira dívida de proveniência: ninguém sabe o que mudou, quando, nem se passou no gate; a próxima regressão custa mais caro do que o bug original. A leitura integral de reviews consome o tempo da SE sem evidência de retorno. O custo de adotar a disciplina é baixo (documentação + comando já existentes), enquanto o custo de não adotar é perder a única vantagem que o projeto tem sobre o outlier — verificabilidade.

## Open Questions

- [NEEDS CLARIFICATION: quem assina a tag e escreve as notas em cada correção — Main ou SE?]
- [NEEDS CLARIFICATION: qual limite de tempo/quantidade para leitura e resposta de reviews?]
- [NEEDS CLARIFICATION: CHANGELOG único do repo ou notas por release no GitHub Releases (ou ambos)?]
