# Decision: Baselines locais de governança

- **Slug**: baseline-governanca
- **Decided**: 2026-11-19
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Dívida 1 aberta em `.specify/memory/STATE.md:50`; `docs/PROGRESSO.md:96` classifica os verdes vazios como falha de auditoria; candidato 3 no 80/20 com score 30,0. |
| Evidence strength | strong | Todo o lastro é local e determinístico: três scripts com caminhos/schemas/baseline explícitos, saídas datadas dos gates em STATE/PROGRESSO e contrato de fase. Nenhuma alegação remota é usada como evidência — `[NV]`. |
| Value vs. inaction | adequate | Fecha dívida 1 e ativa ratchet/stale nos três gates; valor imediato é limitado enquanto o alvo segue documental (`gaps=0`, `files=0`, `duplicates=0`), conforme `Evidence Against` em research.md. |
| Feasibility / appetite | strong | Option A é small: três JSON em `docs/governance/`, formatos já conhecidos, `--bootstrap` nativo no gate de converge, zero dependência externa e zero alteração de código. |
| Strategic fit | strong | Respeita "factory é fonte, não autoridade" (Bloco 0.a), reforça a exigência de evidência de artefatos do R4 e o princípio de que gate determinístico é verdade. |
| Risk posture | adequate | Riscos nomeados e mitigáveis: baseline vazio parecer cobertura, baselinar dívida real como aceita e copiar da factory. Mitigações: derivar do alvo, registrar proveniência, não tocar nos scripts e colar saída em R4. |

## Verdict & Rationale

**go.** O gate R1 está satisfeito: `problem.md` e `concept.md` existem, a validade do problema é `strong`, a força da evidência é `strong` (nunca `weak`/`unknown`) e há opção recomendada (Option A — bootstrap local integral). O problema é real e rastreável em registros locais datados, e a solução proposta cabe em appetite small sem dependência remota — o que honra a ressalva "nunca copiar da factory". Os pontos `adequate` (valor e risco) são reconhecidos, não maquiados: com estado atual limpo, o ganho observável depende da disciplina de re-baselinar e de não transformar dívida em anuência. Isso é tratado como restrição de escopo na especificação, não como motivo para `needs-clarification`: as três perguntas em aberto são refináveis durante `clarify` (R3: ≤3, uma por vez), sem bloquear o início.

## If needs-clarification

- **Blocking questions**: nenhuma. As perguntas em `problem.md`/`research.md` não bloqueiam o `go` e seguem como carried-forward para `specify`/`clarify`.
- **Revisit stage**: não aplicável — veredito `go`.

## If go — Handoff to `/speckit.specify`

- **Problem**: os três gates de governança do alvo emitem PASS sem baseline local, produzindo "verdes vazios" que não distinguem não-regressão de ausência de estado baselinado.
- **Chosen approach**: Option A — derivar os três baselines do estado atual do alvo e gravá-los em `docs/governance/`, nos formatos que os scripts já leem, com proveniência local explícita e sem copiar da factory.
- **In scope / out of scope**: inclui os três JSON de baseline (`sdd-completeness-baseline.json`, `converge-append-only-baseline.json`, `task-id-uniqueness-baseline.json`), a base honesta derivada do estado corrente e a evidência de execução em R4. Exclui alteração de scripts/schemas, importação da factory, CI/merge/remoto `[NV]`, engine/código/telemetria/banco e reclassificação das dívidas 2–6 do STATE.
- **Success metrics**: 3 de 3 arquivos de baseline locais presentes; contadores de baselined deixarem de ser zero **ou** baseline vazio explicitamente justificado com proveniência e saída colada; zero chaves da factory; regressão controlada rejeitada por pelo menos um gate antes de R4.
- **Carried-forward open questions**:
  1. [NEEDS CLARIFICATION: estado exato do alvo no ato da materialização, ante a divergência histórica `spec_dirs=0/1/2`.]
  2. [NEEDS CLARIFICATION: baseline com mapas vazios satisfaz a dívida 1 ou exige ao menos uma entrada por gate?]
  3. [NEEDS CLARIFICATION: critério para separar gap legítimo (a baselinar) de defeito a corrigir antes de virar baseline?]
