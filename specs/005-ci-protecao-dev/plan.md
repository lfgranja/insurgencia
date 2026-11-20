# Implementation Plan: CI de gates determinísticos e proteção de dev

**Feature**: `005-ci-protecao-dev` — executar os cinco gates determinísticos do alvo em PR/push para `dev`/`main`, observar ao vivo o nome do status check e conduzir a proteção de `dev` com verificação por API.
**Input**: [spec.md](spec.md), [research.md](research.md), [decision.md](decision.md) (R1 `go`, Opção A), `AGENTS.md`, `.specify/memory/constitution.md`, `docs/governance/sdd-phase-contracts.md`.

## Technical Context

| Campo | Valor |
|---|---|
| Tipo de feature | Configuração de automação de verificação + governança de branch; sem código de produto |
| Plataforma de execução | GitHub Actions em repositório público (`visibility=PUBLIC`, 2026-11-20T00:26:47Z; leitura via `gh api` em 2026-11-20T00:29Z) |
| Linguagem/runtime dos gates | Python 3 com stdlib; `hub_docs_gate.py` usa PyYAML apenas se disponível |
| Gates autoritativos | Os cinco de `AGENTS.md:16-22`, nenhum a mais e nenhum da factory |
| Gatilhos | Pull request com base `dev` ou `main`; push para `dev` ou `main` |
| Unidade de evidência | Comando executado + saída colada + data/hora + requisito atendido; remoto permanece `[NV]` até execução |
| Proteção | Ato separado e autorizado, aplicado à branch `dev` somente após observação do nome real do check |
| Fora de escopo | Publicação/deploy, `.forgejo/**`, mudança de gates/baselines/governança, stage/commit/merge nesta etapa |
| Artefatos de design | Consolidados neste `plan.md`; `data-model.md`, `contracts/` e `quickstart.md` não se aplicam a uma feature sem API, dados persistidos ou comandos de produto |
| Escrita nesta etapa | Restrita a `specs/005-ci-protecao-dev/`; nenhum gate é rodado, nenhum workflow é criado aqui |

Esclarecimentos pendentes no contexto: nenhum. As três ambiguidades foram resolvidas em `spec.md` / `## Clarifications` / `### Session 2026-11-20`, antes deste plano (R3 respeitado).

## Constitution Check

| Princípio / norma | Reavaliação | Status |
|---|---|---|
| I — Determinismo primeiro | Os cinco gates já são scripts determinísticos; o workflow apenas os executa e registra saída (FR-002, FR-003) | PASS |
| II — Adaptadores e portas | Não há código de domínio nem integração de serviço; acesso remoto via CLI/API em etapa separada, sem acoplar gates a provedor | PASS |
| III — Fluxo SDD ordenado | intake→research→define→shape→decide(`go`)→specify→clarify→plan→checklist→tasks, nesta ordem, sem portaria posterior a tasks | PASS |
| IV — Soberania do jogador | Nenhuma dependência proprietária introduzida; gates usam stdlib e arquivos versionados | PASS |
| V — Materialismo e ação coletiva | Fora do escopo de produto; nenhuma alteração de regras do jogo | PASS |
| VI — Verificação honesta | Nome do check, gates no CI e proteção são `[NV]` até comando com saída e data/hora; falha não é suprimida (FR-003–FR-007) | PASS |
| Norma — gates do alvo | Apenas os cinco de `AGENTS.md`; factory é fonte, não autoridade (FR-002, FR-009) | PASS |
| Norma — commits/branch/PR | Nenhum stage, commit, PR ou merge nesta etapa; mutação remota depende de autorização separada (FR-012) | PASS |
| Contratos de fase (§checklist/tasks) | Checklists com `- [ ] CHK###` e rastreabilidade ≥80%; tasks com `- [ ] T### [P?] [USn?]` e caminho | PASS |

Nenhuma violação injustificada. Os gates não são executados nesta feature de planejamento.

## Decisões de design

- **D1 — Um workflow, um sinal**: Opção A de `concept.md`. Um único workflow e um único status check representam a verificação completa dos cinco gates; evita cinco nomes a observar e proteção por subconjunto. Justificativa: FR-002, SC-002, SC-003.
- **D2 — Cobertura dos quatro eventos**: o mesmo conjunto de gates é acionado por PR→`dev`, PR→`main`, push→`dev` e push→`main`. Justificativa: FR-001, SC-001.
- **D3 — Ordem dos gates**: os cinco comandos são executados na ordem em que aparecem em `AGENTS.md:16-22`, facilitando a comparação entre execução local e CI. Justificativa: FR-002; `STATE.md:11`.
- **D4 — Falha propagada**: saída diferente de zero em qualquer gate falha a verificação; nenhum `continue-on-error`, tolerância ou resultado verde artificial. Justificativa: FR-003, SC-007, constituição VI.
- **D5 — Nome observado, nunca inferido**: o nome do check é lido de execução real e registrado antes da proteção; se não aparecer, a proteção não é aplicada. Justificativa: FR-004, SC-003; `research.md §Evidence Against`.
- **D6 — Proteção como ato separado**: criar o workflow e proteger `dev` são ações distintas; a segunda depende de autorização explícita e de leitura da API. Justificativa: FR-005, FR-006; `AGENTS.md:7-9`.
- **D7 — Prova por API**: a configuração de proteção é lida após aplicação, com saída colada; HTTP 403 ou ausência de resposta é limitação conhecida, nunca verde. Justificativa: FR-006, SC-005; `nv-resolution-2026-11-19.md:7`.
- **D8 — Salvaguardas de escopo**: sem deploy, sem `.forgejo/**`, sem alteração de gates/baselines/governança, sem segredos e com permissões mínimas. Justificativa: FR-008–FR-011, SC-006, SC-010.
- **D9 — Lacuna de diretórios derivados**: o slug `ci-protecao-dev` ainda não existe em `.specify/assessments/`, `.specify/validations/` e `.specify/releases/`, e `check_sdd_completeness.py:64-69` exige os três. A falha resultante é honesta e registrada; a materialização cabe às fases R4/R7 do ciclo, fora do escopo de escrita desta etapa. Nenhuma baseline é ampliada para esconder a lacuna. Justificativa: FR-003, FR-010, SC-007; Clarificação Q2.
- **D10 — Escopo de escrita da etapa**: somente `specs/005-ci-protecao-dev/`; nenhum gate, stage, commit ou workflow nesta etapa. Justificativa: FR-012, FR-014, pedido do mantenedor.

## Matriz de gatilhos

| Evento | Branch/ref | Cobertura | Verificação observável |
|---|---|---|---|
| Pull request aberto, sincronizado ou reapresentado | base `dev` | FR-001 / SC-001 | Uma execução com os cinco gates aparece no PR |
| Pull request aberto, sincronizado ou reapresentado | base `main` | FR-001 / SC-001 | Mesma execução, sem etapa de publicação |
| Push | `dev` | FR-001 / SC-001 | Execução com os cinco gates no ref |
| Push | `main` | FR-001 / SC-001 | Execução com os cinco gates no ref |

## Inventário de gates

| # | Comando autoritativo | Critério de sucesso | Rastreio |
|---|---|---|---|
| 1 | `python3 scripts/ci/check_sdd_completeness.py` | saída zero e `PASS` | `AGENTS.md:17`; FR-002 |
| 2 | `python3 scripts/ci/check_work_order_dag.py` | saída zero e `PASS` | `AGENTS.md:18`; FR-002 |
| 3 | `python3 scripts/ci/check_task_id_uniqueness.py` | saída zero e `PASS` | `AGENTS.md:19`; FR-002 |
| 4 | `python3 scripts/ci/check_converge_append_only.py` | saída zero e `PASS` | `AGENTS.md:20`; FR-002 |
| 5 | `python3 scripts/ci/hub_docs_gate.py` | saída zero e `PASS` | `AGENTS.md:21`; FR-002 |

Comandos fora do conjunto — `check_production_gate.py --check`, `pytest -q`, `verify.sh`, `nextest` e qualquer gate `fj`/Forgejo — não entram no workflow (FR-009). O registro local de `PASS` em `STATE.md:43-53` é precedente, não substituto da execução no CI (SC-002).

## Sequência de verificação ao vivo (execução na fase de implementação)

1. Aplicar o workflow e abrir/atualizar um PR para `dev`; confirmar que uma execução aparece (`[NV]` antes disso).
2. Executar os cinco gates no runner e colar a saída de cada um com data/hora; qualquer falha é registrada, não suprimida (SC-002, FR-003).
3. Listar os checks do PR e capturar o nome exato exibido, com comando e saída (SC-003, FR-004).
4. Registrar o nome e, em ato separado autorizado, configurar a proteção da branch `dev` exigindo aquele nome (SC-004, FR-005).
5. Ler a proteção pela API e colar a resposta com data/hora; se recusada, registrar limitação e manter `[NV]` (SC-005, FR-006).
6. Auditar as salvaguardas: 0 etapas de deploy, 0 `.forgejo/**`, 0 mudanças em gates/baselines/governança, 0 segredos e `[NV]` em toda afirmação remota sem evidência (SC-006–SC-010).
7. Na fase de validação do ciclo, materializar os diretórios derivados e registrar R4; na fase de release, registrar R7 com SemVer, changelog e rollback. Nenhuma dessas fases é executada nesta etapa de planejamento.

## Abordagem de verificação documental desta etapa

1. Contar artefatos: dez presentes, sem seção vazia e sem marcador pendente (SC-009).
2. Conferir rastreabilidade: cada FR mapeado para pelo menos um cenário de aceitação e um SC; cada SC com baseline e meta.
3. Conferir R1 e R3 na decisão e nas clarificações: `go` legítimo e três respostas, uma por vez.
4. Conferir salvaguardas no texto: nenhuma instrução de deploy/Forgejo/segredo e nenhum verde remoto afirmado sem `[NV]`.
5. Registrar divergências encontradas em `research.md`/`spec.md`; nenhuma é suprimida para produzir aparência de conformidade.

Nenhum comando de gate, build, teste, stage, commit ou efeito remoto é executado nesta feature de planejamento.
