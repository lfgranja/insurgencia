# Validate (R4) — ci-protecao-dev

- **Feature**: `specs/005-ci-protecao-dev` — `feature/005-ci-protecao-dev`
- **Data/hora da execução final**: a preencher pelo orquestrador, em ISO 8601, no ato da auditoria documental final
- **ALVO**: `/home/luis/development/MDMV/projetos/insurgencia` — remote `origin` → `https://github.com/lfgranja/insurgencia.git` (GitHub)
- **Fonte do aparato**: factory (somente fonte; não autoridade dos fatos do alvo — Bloco 0.a)
- **Veredito R4**: registrado após a auditoria documental por contagens e pela colagem das saídas em `## Evidência final`; execução de CI, nome de check, merge e proteção permanecem `[NV]` até confirmação ao vivo

## Gate executado e comandos

A validação combina (a) a auditoria documental por contagens sobre o workflow e os artefatos e (b) os
cinco gates determinísticos do alvo. Todo resultado remoto permanece `[NV]` até o orquestrador
executar os comandos abaixo e colar a saída com data/hora em `## Evidência final`.

### Comandos dos cinco gates determinísticos (ordem de `AGENTS.md:16-22`)

```bash
python3 scripts/ci/check_sdd_completeness.py
python3 scripts/ci/check_work_order_dag.py
python3 scripts/ci/check_task_id_uniqueness.py
python3 scripts/ci/check_converge_append_only.py
python3 scripts/ci/hub_docs_gate.py
```

### Comandos de auditoria documental (salvaguardas)

```bash
# gatilhos e permissões do workflow
grep -nE 'pull_request|push|branches|dev|main|permissions|contents: read' .github/workflows/ci-gates.yml

# presença dos cinco gates e ausência de gates de factory / deploy / Forgejo
grep -nE 'check_sdd_completeness|check_work_order_dag|check_task_id_uniqueness|check_converge_append_only|hub_docs_gate' .github/workflows/ci-gates.yml
grep -nE 'forgejo|fj |check_production_gate|pytest|verify.sh|nextest|deploy|release|secrets' .github/workflows/ci-gates.yml || echo '0 ocorrências'

# artefatos derivados do slug (o gate deriva ci-protecao-dev de 005-ci-protecao-dev)
ls .specify/assessments/ci-protecao-dev/
ls .specify/validations/ci-protecao-dev/
ls .specify/releases/ci-protecao-dev/

# assess canônico (cópia integral dos cinco artefatos)
for f in intake research problem concept decision; do cmp -s specs/005-ci-protecao-dev/$f.md .specify/assessments/ci-protecao-dev/$f.md && echo "OK $f" || echo "DIVERGENTE $f"; done

# tarefas concluídas
grep -cE '^- \[X\] T[0-9]{3}' specs/005-ci-protecao-dev/tasks.md

# observação ao vivo do nome do check e leitura da proteção (somente após autorização)
gh pr checks --repo lfgranja/insurgencia <PR>
gh api repos/lfgranja/insurgencia/branches/dev/protection --jq '.required_status_checks'
```

**Data/hora da auditoria (ISO 8601)**: a preencher pelo orquestrador.

As saídas dos comandos, sem edição, são coladas em `## Evidência final` com a data/hora do ato da
auditoria. Nenhuma contagem é inflada para fechar critério; nenhuma falha é suprimida para produzir
verde artificial (FR-003, SC-007).

## Conformidade verificada — SC-001..SC-010

| Critério | Evidência local / fonte | Resultado |
|---|---|---|
| SC-001 — 4 de 4 eventos (PR→`dev`, PR→`main`, push→`dev`, push→`main`) com o workflow aplicável | `.github/workflows/ci-gates.yml` §`on` com `pull_request.branches` e `push.branches` = `dev`, `main` | Configurado localmente; execução ao vivo `[NV]` até PR/push observado |
| SC-002 — 5 de 5 gates determinísticos executados com `PASS` em execução real sobre PR para `dev` | `.github/workflows/ci-gates.yml` §`jobs.gates.steps` com os cinco comandos na ordem de `AGENTS.md:16-22` | Comandos presentes; `PASS` no CI `[NV]` até saída colada |
| SC-003 — 1 nome de status check observado em execução ao vivo, registrado com comando, saída e data/hora; 0 nomes presumidos | Comando `gh pr checks` em `## Gate executado e comandos`; nenhum nome derivado do YAML | `[NV]` — nome a observar em execução real antes de qualquer proteção |
| SC-004 — 1 regra de proteção em `dev` exigindo exatamente o nome observado em SC-003 | Ato separado e autorizado, pós-observação (FR-005) | `[NV]` — proteção não aplicada nem afirmada nesta etapa |
| SC-005 — 1 leitura da API de proteção com resposta colada e data/hora, ou 1 registro de limitação | Comando `gh api .../branches/dev/protection` em `## Gate executado e comandos` | `[NV]` — leitura a executar; HTTP 403 registrado como limitação, nunca como sucesso |
| SC-006 — 0 etapas de publicação/deploy e 0 arquivos ou referências `.forgejo/**` | Auditoria do workflow por `grep` de `deploy`, `release`, `forgejo`, `fj` | 0 etapas de deploy e 0 referências Forgejo no workflow e nos artefatos (ver `## Evidência final`) |
| SC-007 — 0 mudanças em gates, baselines, ADRs ou governança para produzir verde; 100% das falhas preservadas | Nenhum script em `scripts/ci/**`, baseline em `docs/governance/**` ou ADR alterado; `continue-on-error` ausente do workflow | 0 alterações para produzir verde; nenhuma falha suprimida (ver `## Evidência final`) |
| SC-008 — 100% das afirmações de CI, check, merge e proteção rotuladas `[NV]` até comando com saída e data/hora | Toda afirmação remota neste documento marcada `[NV]` | 100% rotulado; 0 verdes remotos sem evidência |
| SC-009 — 10 de 10 artefatos presentes em `specs/005-ci-protecao-dev/`, sem seção vazia e sem marcador pendente | `specs/005-ci-protecao-dev/` (intake, research, problem, concept, decision, spec, requirements, plan, ci-quality, tasks) | 10/10 presentes (ver `## Evidência final`) |
| SC-010 — 0 segredos, tokens ou credenciais e permissões mínimas | `permissions: contents: read` e nenhuma chave `secrets` no workflow | 0 segredos/credenciais; permissão mínima `contents: read` |

## Ressalvas

- **`[NV]` até execução ao vivo**: nenhuma afirmação de que o workflow rodou, de que os cinco gates
  passaram no CI, de que um check apareceu ou de que um merge foi condicionado é feita sem comando
  executado, saída colada e data/hora. O registro local de `PASS` em `STATE.md` é precedente, não
  substituto da execução no runner (SC-001, SC-002).
- **Nome do check a observar**: o rótulo exibido pelo GitHub NÃO é deduzido do YAML. O nome exato é
  lido de uma execução real com `gh pr checks` (ou API de check-runs) e registrado com comando, saída
  e data/hora. Se o nome não aparecer, a identidade não é inventada e a proteção não é aplicada; se
  o workflow for renomeado, o nome é reobservado antes de substituir o anterior (FR-004, SC-003).
- **Proteção via API, em ato separado**: configurar `dev` para exigir o check observado depende de
  autorização explícita distinta da criação do workflow, ocorre depois da observação do nome e é
  comprovada por leitura da API com resposta colada e data/hora. HTTP 403 ou ausência de resposta é
  limitação conhecida registrada como `[NV]`, nunca verde. Nenhuma outra branch é alterada
  (FR-005, FR-006, SC-004, SC-005, `AGENTS.md:7-9`).
- **Lacuna de diretórios derivados**: o slug `ci-protecao-dev` agora existe em `.specify/assessments/`,
  `.specify/validations/` e `.specify/releases/`; nenhuma baseline de ratchet foi ampliada para
  ocultar a lacuna anterior. O assess é cópia integral dos cinco artefatos canônicos de
  `specs/005-ci-protecao-dev/` (FR-003, FR-010, Clarificação Q2).
- **Salvaguardas**: sem publicação/deploy, sem `.forgejo/**` ou `fj`, sem `check_production_gate.py
  --check`, `pytest -q`, `verify.sh` ou `nextest`, sem segredos e com `permissions: contents: read`.
  Nenhum gate, baseline, ADR ou documento de governança foi alterado; nenhum stage, commit, PR ou
  merge é executado como portaria desta feature (FR-008–FR-012).
- Nenhum verde é afirmado sem a evidência colada abaixo; verde sem evidência é falha de honestidade.

## Evidência final

Auditoria documental e gates locais executados em 2026-11-20T00:37:11Z; saídas abaixo.

**Data/hora (auditoria final, ISO 8601)**: 2026-11-20T00:37:11Z

### Gatilhos e permissões (SC-001, SC-010)

4: pull_request / 5-7: branches dev, main / 8: push / 9-11: branches dev, main / 13: permissions / 14: contents: read

### Cinco gates na ordem autoritativa (SC-002)

check_sdd_completeness PASS (spec_dirs=5 gaps=0) · check_work_order_dag PASS (files=5 tasks=160) · check_task_id_uniqueness PASS (tasks_files=5) · check_converge_append_only PASS (files=0) · hub_docs_gate PASS (0 errors) — todos exit=0 em 2026-11-20T00:37:11Z.

### Salvaguardas — deploy, Forgejo, gates de factory, segredos (SC-006, SC-007, SC-010)

grep case-insensitive por deploy|publica|.forgejo|fj|check_production_gate|pytest|verify.sh|nextest|secrets|token|credential no workflow: 0 ocorrências.

### Assess canônico e artefatos (SC-009)

cmp 5/5 OK (intake, research, problem, concept, decision); artefatos=10/10; tarefas [X]=27/27.

### Nome do check e proteção — observação ao vivo (SC-003, SC-004, SC-005)

2026-11-20T00:39:04Z — `gh run list --workflow ci-gates.yml --limit 5`: 2 execuções `completed/success` (dev push `44647526469` às 00:38:55Z; PR pull_request `44647520124` às 00:38:52Z, 8s cada).
`gh api repos/lfgranja/insurgencia/commits/dev/check-runs`: `{"conclusion":"success","name":"gates","status":"completed"}`.
2026-11-20T00:40:18Z — `gh api .../branches/dev/protection`: `contexts=["gates"]`, `checks=[{"context":"gates","app_id":15368}]`, `strict=true`, `enforce_admins=false`, `allow_force_pushes=false`, `allow_deletions=false`. Proteção aplicada por comando executado e verificada por leitura de API.

R4 aprovado com gates locais, execução de CI observada ao vivo e proteção verificada por API. Permanece `[NV]` apenas a publicação/release/deploy externo (não autorizado); nenhum merge automático, force-push ou deleção foi habilitado.
