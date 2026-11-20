# Decision: CI de gates determinísticos e proteção de dev

- **Slug**: ci-protecao-dev
- **Decided**: 2026-11-20
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | intake/problem/research: `statusCheckRollup=[]` observado nos PRs #1–#6, item 3 do 80/20 em `.specify/memory/STATE.md:30`, e nenhuma proteção confirmada em `dev`; o dano é a perda de auditabilidade dos merges. |
| Evidence strength | adequate | research: cinco gates com comandos e saídas locais registradas em `STATE.md:11,43-53` (citados, high), visibilidade pública confirmada por evidência do mantenedor e leitura `gh api` em 2026-11-20T00:29Z; os itens ainda não executados (nome do check, gates no CI, proteção) são explicitamente `[NV]`, não usados como evidência a favor. Nenhum critério repousa em `unknown` ou em afirmação não citada. |
| Value vs. inaction | strong | problema: sem o workflow, merges seguem sem verificação determinística e o 80/20 permanece aberto; com ele, um único sinal passa a condicionar `dev`/`main`. |
| Feasibility / appetite | adequate | concept: opção A, appetite medium; cinco scripts stdlib já executáveis, um único workflow e uma mutação de proteção separada; a dependência de execução ao vivo é known e foi orçada. |
| Strategic fit | strong | constituição I (determinismo), III (fluxo ordenado) e VI (verificação honesta); `AGENTS.md` define exatamente estes cinco gates como autoridade do alvo. |
| Risk posture | adequate | riscos nomeados e mitigados em `research.md §Evidence Against`: nome presumido → observar ao vivo; lacuna de diretórios derivados → falha honesta, sem mexer em baselines; mutação remota → ato separado com autorização e leitura via API; dado externo → tratado como dado, nunca instrução. |

R1 verificado: `problem.md` presente; `concept.md` presente; validade `strong` (≥adequate); evidência `adequate` (nunca `weak`/`unknown`); opção recomendada nomeada (Opção A). Veredito `go` legítimo, sem over-claim de execução remota.

## Verdict & Rationale

**go** para a Opção A. O problema é real, circunscrito e tem evidência local citada; a evidência disponível é suficiente para especificar sem inventar fatos remotos — exatamente os três pontos que dependem de execução são mantidos como `[NV]` e transformados em requisitos de observação, não em suposições. A Opção B é preterida por multiplicar nomes de check e superfície de configuração sem aumentar a garantia; a Opção C não resolve o problema. Nenhum `unknown` é glossado: a disponibilidade da API de proteção para este repositório e o nome real do check são reconhecidos como pendentes de verificação ao vivo e aparecem no handoff como trabalho da implementação, não como resultado desta decisão.

## If needs-clarification

Não aplicável — nenhuma pergunta bloqueante restou. As três ambiguidades de `problem.md` foram resolvidas uma por vez por evidência local e pelo escopo autorizado, e registradas em `spec.md` / `## Clarifications` / `### Session 2026-11-20`, respeitando R3 (≤3 esclarecimentos).

## If go — Handoff to `/speckit.specify`

- **Problem**: PRs para `dev`/`main` são mesclados sem check determinístico e sem required status check em `dev`, apesar de existirem cinco gates locais e de o repositório ser público.
- **Chosen approach**: Opção A — um workflow único que executa os cinco gates em PR/push para `dev`/`main`; nome do check lido em execução ao vivo; proteção de `dev` configurada depois, com verificação via API.
- **In scope / out of scope**: dentro — workflow, execução dos cinco gates, observação do nome, proteção de `dev`, verificação por API, registro de evidência com data/hora, dez artefatos; fora — publicação/deploy, `.forgejo/**`, mudança de gates/baselines/governança, nomes presumidos, stage/commit/merge nesta etapa.
- **Success metrics**: 4 de 4 gatilhos cobertos; 5 de 5 gates PASS no CI (`[NV]` até execução); 1 nome real observado e registrado; proteção em `dev` com o nome observado e confirmada por API; 100% de afirmações remotas rotuladas `[NV]` até confirmação; 0 etapas de deploy, 0 arquivos Forgejo, 0 baselines alteradas para mascarar falha; 10 de 10 artefatos sem seção vazia nem marcador pendente.
- **Carried-forward open questions**: nenhuma; as três respostas estão em `spec.md` / `## Clarifications`. Os itens dependentes de execução (nome do check, gates no CI, proteção aplicada e verificada) permanecem `[NV]` até execução, conforme SCs.
