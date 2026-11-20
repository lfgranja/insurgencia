# Release (R7) — ci-protecao-dev

- **Versão**: 1.0.0 (SemVer)
- **Data**: 2026-11-20
- **Feature**: `specs/005-ci-protecao-dev` — `feature/005-ci-protecao-dev`
- **R4**: registrado em `.specify/validations/ci-protecao-dev/validate.md` antes deste release
- **Branch**: `feature/005-ci-protecao-dev` — sem merge, sem auto-merge, sem force-push e sem deleção de branch
- **Remote/CI/PR/proteção**: `[NV]` até confirmação ao vivo via `gh` (visualização pública registrada em 2026-11-20T00:26:47Z e leitura via `gh api` em 2026-11-20T00:29Z)

## Changelog (1.0.0)

| ID | Mudança | Origem |
|---|---|---|
| FR-001 | Workflow `.github/workflows/ci-gates.yml` aplicável a PR com base `dev`/`main` e push para `dev`/`main` — 4 de 4 gatilhos; execução ao vivo `[NV]` | `spec.md` §FR-001; `plan.md` §Matriz de gatilhos |
| FR-002 | Cinco gates determinísticos do alvo executados na ordem de `AGENTS.md:16-22` — `check_sdd_completeness.py`, `check_work_order_dag.py`, `check_task_id_uniqueness.py`, `check_converge_append_only.py`, `hub_docs_gate.py` | `spec.md` §FR-002; `plan.md` §Inventário de gates |
| FR-003 | Falha propagada: código diferente de zero em qualquer gate falha a verificação; sem `continue-on-error` ou verde artificial | `spec.md` §FR-003; `plan.md` §D4 |
| FR-004 | Nome do status check a obter de execução ao vivo com comando, saída e data/hora, antes de qualquer proteção; nenhum nome presumido — `[NV]` até observação | `spec.md` §FR-004; `plan.md` §D5 |
| FR-005 | Proteção de `dev` deve exigir exatamente o nome observado; nenhuma outra branch alterada — ato separado e autorizado, `[NV]` | `spec.md` §FR-005; `AGENTS.md:7-9` |
| FR-006 | Leitura da proteção via API com resposta colada e data/hora; HTTP 403 registrado como limitação, nunca sucesso — `[NV]` | `spec.md` §FR-006; `plan.md` §D7 |
| FR-007 | Toda afirmação de CI, check, merge ou proteção permanece `[NV]` até comando com saída colada e data/hora | `spec.md` §FR-007; R4 §Ressalvas |
| FR-008 | Nenhuma etapa de publicação, release, deploy, pacote/imagem ou transferência de artefato externo no workflow | `spec.md` §FR-008; `plan.md` §D8 |
| FR-009 | Nenhum `.forgejo/**`, `fj`, `check_production_gate.py --check`, `pytest -q`, `verify.sh` ou `nextest` adotado como gate do alvo | `spec.md` §FR-009; `AGENTS.md` |
| FR-010 | Nenhum script de gate, baseline de ratchet, ADR ou documento de governança alterado para produzir verde; falha registrada, não suprimida | `spec.md` §FR-010; `plan.md` §D9 |
| FR-011 | Permissões mínimas (`contents: read`) e nenhum segredo, token ou credencial no workflow ou artefatos | `spec.md` §FR-011; `plan.md` §D8 |
| FR-012 | Escrita restrita ao workflow e aos artefatos derivados; nenhum stage, commit, PR ou merge realizado como portaria desta feature | `spec.md` §FR-012; `plan.md` §D10 |
| FR-013 | Evidência de execução ao vivo reservada com comando, saída, data/hora e requisito, em R4 `## Evidência final` | `spec.md` §FR-013; `docs/governance/sdd-phase-contracts.md` |
| FR-014 | Dez artefatos de planejamento presentes, sem seção vazia nem marcador pendente, com rastreabilidade | `spec.md` §FR-014; R4 SC-009 |
| T001–T027 | 27 tarefas de setup, gates, observação do check, proteção, auditoria e acabamento concluídas (`[X]`) | `specs/005-ci-protecao-dev/tasks.md` |
| SC-001–SC-010 | Conformidade registrada em R4; itens dependentes de execução ao vivo mantidos `[NV]` | `.specify/validations/ci-protecao-dev/validate.md` |
| CHK001–CHK### | Rastreabilidade FR↔SC↔CHK nos checklists `requirements.md` e `ci-quality.md` | `specs/005-ci-protecao-dev/checklists/` |
| DEC-R1 | Veredito `go` da Opção A — workflow único com job de gates e proteção pós-observação | `.specify/assessments/ci-protecao-dev/decision.md` |
| BUG-### | Nenhum bug reportado neste ciclo | — (nenhum registro `BUG-###` no ciclo) |

## Entregáveis versionados

- `.github/workflows/ci-gates.yml` — workflow `deterministic-gates`: gatilhos PR/push para `dev`/`main`, `permissions: contents: read`, job `gates` em `ubuntu-latest` com checkout, setup-python e os cinco gates na ordem autoritativa.
- `.specify/assessments/ci-protecao-dev/` — assess canônico: cópia integral de `intake.md`, `research.md`, `problem.md`, `concept.md` e `decision.md` (o gate deriva o slug `ci-protecao-dev` de `005-ci-protecao-dev`).
- `.specify/validations/ci-protecao-dev/validate.md` — R4 com `## Evidência final` para o orquestrador colar comandos, saídas e data/hora.
- `.specify/releases/ci-protecao-dev/release.md` — este documento (R7).
- `specs/005-ci-protecao-dev/tasks.md` — 27 de 27 tarefas marcadas `[X]`, IDs preservados.

## Plano de rollback (documental)

Procedimento aplicável à configuração introduzida: o workflow e a regra de proteção. A feature não
produz código de produto, deploy, banco, migração, binário ou serviço externo.

1. Registrar o achado com IDs FR e, quando existirem, `T###` e `BUG-###` (gatilhos: gate suprimido,
   ordem dos gates alterada, permissão ampliada, segredo introduzido, etapa de deploy/Forgejo
   adicionada, nome de check presumido ou proteção aplicada sem observação e leitura por API).
2. **Workflow**: para reverter, remover `.github/workflows/ci-gates.yml` em novo ciclo documentado,
   com nova versão SemVer e data — nunca apagar ou reescrever histórico. A remoção restaura a condição
   `statusCheckRollup=[]` anterior e é registrada como reversão, não como correção silenciosa.
3. **Proteção de `dev`**: para reverter, remover o required status check da branch `dev` em ato
   separado e autorizado, somente após ler a configuração atual pela API e colar a resposta com
   data/hora; nenhuma outra branch é tocada. Se a API recusar (HTTP 403), registrar a limitação e
   manter `[NV]` — não afirmar reversão aplicada.
4. Reconfirmar que nenhum script de gate, baseline, ADR ou documento de governança foi alterado e que
   nenhuma baseline foi ampliada para ocultar a falha de completude anterior.
5. Reexecutar a auditoria documental por contagens e conferir as saídas coladas no R4; registrar a
   reversão no changelog com versão, data e IDs FR/`T###`/`BUG-###`.

**Proibido**: apagar histórico, `force-push`, merge ou auto-merge, reescrever governança, afirmar
CI/merge/aprovação/proteção obtidos sem comando executado, saída colada e data/hora.

## Restrições

- Nenhum merge, auto-merge, force-push ou deleção de branch executada.
- Nenhuma afirmação de CI/check verde remoto ou de proteção aplicada; `[NV]` até confirmação ao vivo.
- Nenhuma portaria posterior a tasks (implement, converge, validate, release) executada como gate,
  stage ou commit por esta feature.
- Nenhum código, dependência, engine, serviço externo, segredo ou emenda da norma externa introduzido.
- Publicação/deploy, `.forgejo/**` e gates da factory permanecem fora de escopo.
