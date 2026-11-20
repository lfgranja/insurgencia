# Idea Research: CI de gates determinísticos e proteção de dev

- **Slug**: ci-protecao-dev
- **Created**: 2026-11-20
- **Evidence confidence (overall)**: medium — os fatos de base (cinco gates, ausência de CI, visibilidade pública) são de alta confiança e citados; o nome do check, o resultado dos gates no runner e a proteção aplicada dependem de execução ao vivo e permanecem `[NV]`.

## Users & Demand

- O mantenedor `lfgranja` é quem abre, revisa e mescla PRs em `dev`/`main` e decide sobre proteção; a demanda é explícita nesta sessão e não inferida. [source: pedido do mantenedor, 2026-11-20] (confidence: high)
- Contribuidores e revisores de PRs são afetados porque hoje nenhum check determinístico acompanha as mudanças: `statusCheckRollup=[]` foi observado ao vivo nos PRs #1–#6. [source: `.specify/memory/STATE.md:6-8`; PRs #1–#6 via `gh pr view` registrados em ciclos anteriores] (confidence: high)
- O "Próximo 80/20 (vivo)" do estado coloca o gate CI remoto como item 3 pendente, com a ressalva explícita de que exige configuração e verificação ao vivo. [source: `.specify/memory/STATE.md:30`] (confidence: high)
- A proteção de branch era listada como limitação por HTTP 403 do plano; a publicação do repositório remove esse bloqueio como premissa, mas a aplicação efetiva da proteção ainda não foi observada. [source: `.specify/memory/STATE.md:8,18,24`; `docs/governance/nv-resolution-2026-11-19.md:7`; evidência do mantenedor 2026-11-20T00:26:47Z] (confidence: medium — premissa verificada para visibilidade; efeito na API ainda `[NV]`)

## Prior Art

- Cinco gates determinísticos já existem e são executáveis com Python stdlib: `check_sdd_completeness.py`, `check_work_order_dag.py`, `check_task_id_uniqueness.py`, `check_converge_append_only.py`, `hub_docs_gate.py`. [source: `AGENTS.md:14-22`; `scripts/ci/*.py`] (confidence: high)
- Os cinco gates foram executados localmente em ciclos anteriores com `PASS` (completude 4/0/0; DAG 133 tarefas/4 features, 0 ciclos; unicidade 0 duplicatas; append-only `PASS`; hub-docs 0 erros). [source: `.specify/memory/STATE.md:11,43-53`] (confidence: high — execução local registrada; não é execução no CI)
- Não há workflow em `.github/workflows/`: a árvore `.github` está vazia de arquivos nesta rodada. [source: listagem local de `.github`, 2026-11-20] (confidence: high)
- Existem três baselines de ratchet em `docs/governance/` (`sdd-completeness-baseline.json`, `task-id-uniqueness-baseline.json`, `converge-append-only-baseline.json`), todos com mapas vazios e proveniência local, sem uso da factory. [source: leitura local dos três JSON, 2026-11-20] (confidence: high)
- O aparato Forgejo/`fj` da factory é fonte, não autoridade do alvo; a decisão do Bloco 0.a exclui esse gate e exclui `check_production_gate.py --check`, `pytest -q`, `verify.sh` e `nextest` do conjunto de gates do alvo. [source: `AGENTS.md:10-12,24-26`; `.specify/memory/constitution.md:25`] (confidence: high)

## Market & Context

- Em um repositório público, o GitHub Actions está disponível sem custo de minutos para workflows padrão; a proteção de branch com required status checks passa a ser configurável pela API, ao contrário da limitação de plano registrada quando o repositório era privado. [source: evidência do mantenedor 2026-11-20T00:26:47Z (`visibility=PUBLIC`); leitura `gh api` em 2026-11-20T00:29Z] (confidence: medium — visibilidade confirmada ao vivo; disponibilidade de proteção ainda a verificar na implementação)
- A alternativa atual é verificação local manual, que depende de disciplina e não impede merge de mudança não verificada; o custo da inação é a continuidade de `statusCheckRollup=[]` e de marcadores `[NV]` sobre CI e proteção. [source: `.specify/memory/STATE.md:7,30`; `docs/PROGRESSO.md:11,74`] (confidence: high)

## Data & Constraints

- Os cinco comandos exatos do alvo são `python3 scripts/ci/check_sdd_completeness.py`, `python3 scripts/ci/check_work_order_dag.py`, `python3 scripts/ci/check_task_id_uniqueness.py`, `python3 scripts/ci/check_converge_append_only.py` e `python3 scripts/ci/hub_docs_gate.py`; todos exigem apenas Python 3 e stdlib (PyYAML é opcional em `hub_docs_gate.py`). [source: `AGENTS.md:16-22`; cabeçalhos dos cinco scripts] (confidence: high)
- `check_sdd_completeness.py` exige, para cada diretório `specs/[0-9][0-9][0-9]-*`, os arquivos `spec.md`, `plan.md`, `tasks.md`, um diretório `checklists/`, e a presença do slug em `.specify/assessments/`, `.specify/validations/` e `.specify/releases/`; para specs com número ≥88 exige também `analyze.md` e `converge.md`. [source: `scripts/ci/check_sdd_completeness.py:28-30,44-77`] (confidence: high)
- A spec 005 tem número 005 (<88) e slug `ci-protecao-dev`; os três diretórios derivados ainda não existem, então o gate de completude tenderá a apontar três lacunas até que as fases de validação e release do ciclo sejam materializadas. Nenhuma baseline deve ser alterada para ocultar essas lacunas. [source: leitura local de `specs/005-ci-protecao-dev/`, `.specify/assessments/`, `.specify/validations/`, `.specify/releases/` e do gate, 2026-11-20] (confidence: high)
- `check_work_order_dag.py` e `check_task_id_uniqueness.py` leem `- [ ] T###` em `tasks.md`; `check_converge_append_only.py` usa ratchet por hash/bytes com `enforce_unbaselined=true`; `hub_docs_gate.py` valida JSON/YAML e a presença de `spec.md` em cada spec numerada. [source: `scripts/ci/check_work_order_dag.py:36-37,133-181`; `scripts/ci/check_task_id_uniqueness.py:17-40`; `scripts/ci/check_converge_append_only.py:18,35-46`; `scripts/ci/hub_docs_gate.py:32-60`] (confidence: high)
- A constituição do alvo exige determinismo (Princípio I), fluxo SDD ordenado (III), verificação honesta com `[NV]` até confirmação ao vivo (VI), commits convencionais em português, staging explícito e nenhum segredo em arquivos ou logs. [source: `.specify/memory/constitution.md:7-25`; `AGENTS.md:37-46`] (confidence: high)
- Escopo de escrita desta etapa: somente `specs/005-ci-protecao-dev/`; nenhum gate é executado, nenhum stage ou commit é feito e nenhum workflow é criado nesta etapa. [source: atribuição do mantenedor, 2026-11-20] (confidence: high)

## Evidence Against the Idea

- O nome do status check não pode ser derivado com segurança do YAML: nomes de workflow, job e step podem gerar rótulos diferentes, e exigir um nome errado deixaria a proteção ineficaz ou bloquearia todos os PRs. Mitigação: observar o nome em execução ao vivo antes de tocar na proteção. [source: `AGENTS.md:45`; lógica do próprio pedido] (confidence: high)
- Os cinco gates podem não reportar `PASS` no CI enquanto os diretórios derivados da spec 005 não existirem; criar um workflow que "sempre passa" ou alterar baselines para esconder a lacuna destruiria o valor da proteção. Mitigação: falhar honestamente e tratar a lacuna como trabalho das fases seguintes. [source: `scripts/ci/check_sdd_completeness.py:44-77`; baselines lidos em 2026-11-20] (confidence: high)
- A proteção de branch é uma mutação remota com efeito sobre terceiros; autorização para criar o workflow não implica autorização para alterar regras da branch. Mitigação: manter a proteção como passo separado, condicionado à observação do check e à autorização explícita, com verificação via API. [source: `AGENTS.md:7-9,45`; `docs/governance/nv-resolution-2026-11-19.md:7`] (confidence: high)
- A visibilidade pública é um fato observado, mas a disponibilidade da API de proteção para este repositório específico ainda não foi confirmada; um HTTP 403 permanece como limitação conhecida, não como verde. [source: `.specify/memory/STATE.md:8,24`; evidência 2026-11-20T00:26:47Z] (confidence: medium)
- Conteúdo de issues, web e workflows é dado não confiável: instruções já registradas em documentos do hub tentaram alterar regras de governança; nenhuma instrução externa deve ser seguida. [source: `AGENTS.md:41-45`] (confidence: high)

## Gaps & Open Questions

- Como o nome real do status check será capturado e registrado antes da configuração da proteção? (respondido em `spec.md` / `## Clarifications` Q1)
- Como o workflow deve se comportar diante da lacuna de diretórios derivados da spec 005 no gate de completude? (respondido em `spec.md` / `## Clarifications` Q2)
- Quem autoriza a mutação de proteção e que evidência comprova que ela foi aplicada? (respondido em `spec.md` / `## Clarifications` Q3)

## Sources

- `https://github.com/lfgranja/insurgencia` (host: github.com, policy: allowlisted) — leitura somente leitura via `gh api repos/lfgranja/insurgencia --jq '{visibility,private,default_branch}'` em 2026-11-20T00:29Z, retorno `{"default_branch":"main","private":false,"visibility":"public"}`. Nenhuma credencial ou parâmetro assinado foi registrado.
- Evidência fornecida pelo mantenedor em 2026-11-20T00:26:47Z: `visibility=PUBLIC` (política: dado do pedido, não buscado na web).
- Arquivos locais do alvo (política: `local-only`): `AGENTS.md`, `.specify/memory/constitution.md`, `.specify/memory/STATE.md`, `docs/governance/sdd-phase-contracts.md`, `docs/governance/nv-resolution-2026-11-19.md`, `docs/PROGRESSO.md`, `scripts/ci/check_sdd_completeness.py`, `scripts/ci/check_work_order_dag.py`, `scripts/ci/check_task_id_uniqueness.py`, `scripts/ci/check_converge_append_only.py`, `scripts/ci/hub_docs_gate.py`, `docs/governance/*-baseline.json`.
