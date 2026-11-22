# Problem Definition: CI de gates determinísticos e proteção de dev

- **Slug**: ci-protecao-dev
- **Created**: 2026-11-20
- **Inputs used**: intake.md | research.md | pedido do mantenedor 2026-11-20

## Problem Statement

As mudanças destinadas a `dev` e `main` no repositório Insurgência são mescladas sem verificação automática dos cinco gates determinísticos do alvo: nenhum check é reportado nos PRs (`statusCheckRollup=[]`) e a branch `dev` não tem required status check. Com o repositório agora público, o bloqueio anterior da API de proteção deixa de ser uma barreira de plano, mas a ausência de execução e de evidência ao vivo mantém CI, nome do check e proteção como `[NV]` — e qualquer merge pode acontecer sem prova de que os gates passaram.

## Affected Users & Stakeholders

- **Users**: mantenedor que abre/revisa PRs (`lfgranja`) — hoje precisa executar os gates localmente e não tem sinal de verificação na página do PR; contribuidores de PRs para `dev`/`main` — não recebem feedback determinístico antes do merge.
- **Stakeholders**: comunidade do jogo, que depende da integridade histórica dos artefatos SDD; auditores, que não conseguem provar que um merge foi precedido de gates PASS; o próprio processo de governança, cuja constituição exige determinismo e verificação honesta.

## Goals

- Ter os cinco gates determinísticos executados automaticamente em PR e push para `dev`/`main`, com resultado explícito de sucesso ou falha.
- Conhecer o nome real do status check a partir de uma execução ao vivo, sem presumi-lo.
- Fazer da branch `dev` uma branch protegida por required status check usando exatamente o nome observado.
- Registrar cada afirmação remota com comando, saída e data/hora, mantendo `[NV]` até confirmação ao vivo.
- Entregar o planejamento completo em dez artefatos rastreáveis, com FRs testáveis e SCs mensuráveis.

## Non-Goals

- Publicação, release ou deploy de qualquer artefato (não autorizado).
- Importação de `.forgejo/**`, uso do gate Forgejo/`fj` ou adoção de `check_production_gate.py --check`, `pytest -q`, `verify.sh` e `nextest` como gates do alvo.
- Alteração do código dos cinco gates, das baselines de ratchet, de ADRs ou de documentos de governança para forçar um resultado verde.
- Supressão de falha de gate, aceitação de check "sempre verde" ou exigência de um nome de check inventado.
- Stage, commit, abertura de PR, merge, reescrita de histórico ou mutação remota nesta etapa de planejamento.
- Definição de arquitetura de produto, engine, runtime de jogo, telemetria ou banco.

## Success Metrics

- Cobertura de gatilhos: 4 de 4 eventos (PR→`dev`, PR→`main`, push→`dev`, push→`main`) com workflow aplicável (baseline: 0 de 4; verificar na implementação, `[NV]` até execução).
- Gates no CI: 5 de 5 gates executados e com `PASS` em uma execução ao vivo sobre um PR para `dev` (baseline: 0 de 5 observados no CI; `[NV]` até execução).
- Nome do check: 1 nome real observado em execução ao vivo e registrado com comando, saída e data/hora (baseline: nenhum; `[NV]` até execução).
- Proteção: required status check configurado em `dev` com o nome observado e confirmado por leitura da API (baseline: nenhuma proteção confirmada; `[NV]` até execução).
- Honestidade: 100% das afirmações de CI, check, merge e proteção rotuladas `[NV]` até observação; 0 nomes de check presumidos (baseline: prática anterior; meta 0).
- Salvaguardas: 0 etapas de publicação/deploy, 0 arquivos `.forgejo/**` importados e 0 baselines/gates alterados para mascarar falha (baseline: 0; meta 0).
- Artefatos: 10 de 10 artefatos presentes, sem seção vazia e sem marcador pendente (baseline: 1 template de spec; meta 10 de 10).

## Cost of Inaction

PRs continuam a ser mesclados sem verificação determinística, o item 3 do 80/20 permanece aberto, e a honestidade documental exige que CI, nome do check e proteção sigam rotulados `[NV]` indefinidamente. O custo é a perda de auditabilidade de cada merge e a repetição manual dos cinco gates por quem lembrar de executá-los — com risco concreto de regressões nos artefatos SDD que os gates protegem.

## Open Questions

- Pergunta 1 — como capturar o nome real do check antes de configurar a proteção? Respondida em `spec.md` / `## Clarifications` / Q1: por observação de execução ao vivo, nunca por inferência do YAML.
- Pergunta 2 — como tratar a lacuna dos diretórios derivados da spec 005 no gate de completude? Respondida em `spec.md` / `## Clarifications` / Q2: falha honesta e materialização nas fases seguintes, sem alterar baselines.
- Pergunta 3 — quem autoriza a proteção e como provar que ela foi aplicada? Respondida em `spec.md` / `## Clarifications` / Q3: ato separado, após observação do check, verificado por leitura da API e registrado com data/hora; permanece `[NV]` até então.
