# Progresso — Insurgência (append-only)

> **Este arquivo é append-only.** Entradas são datadas e imutáveis depois de gravadas.
> Atualizações futuras DEVEM ser **acrescentadas ao final**, como uma nova seção
> `## YYYY-MM-DD — ...`; **nunca** editar, renumeralizar ou apagar entradas anteriores.
> É o índice vivo do estado verificado do alvo; a convenção de notas datadas
> `docs/progress/YYYY-MM-DD-*.md` vem do item `doc-31` (decisão `ADOPT`).
>
> **Regra de evidência:** nenhuma transição de estado sem comando executado, saída colada e
> data/hora. Gate determinístico é verdade; saída de agente é informativa. Toda afirmação
> remota, de CI, merge ou proteção permanece **[NV]** até confirmação ao vivo.

## 2026-11-19 — Estado verificado (Blocos 0/1/3)

**Alvo:** `/home/luis/development/MDMV/projetos/insurgencia` · HEAD `6e25758` · branch `main`.
**Fonte do aparato SDD:** `mdmv-dark-factory` HEAD `b1deab1` — apenas **fonte**, não autoridade
(decisão de governança do Bloco 0.a, registrada em `.specify/memory/SOURCE_FACTORY.md`).
**Natureza:** repositório predominantemente documental; sem código-fonte, engine instalada,
testes, build, CI rodando ou deploy.

### Comandos executados e saídas resumidas

| # | Comando (no alvo) | Saída resumida |
|:--|:--|:--|
| 1 | `git rev-parse --short HEAD` · `git branch --show-current` | `6e25758` · `main` |
| 2 | `git status --short` | 4 entradas untracked: `.specify/`, `AGENTS.md`, `docs/`, `scripts/`; nada staged |
| 3 | `git remote -v` | `origin` → `https://github.com/lfgranja/insurgencia.git` (fetch/push); CLI de PR `gh`. Status remoto **[NV]** |
| 4 | `find docs -name '*.md' -type f \| wc -l` | `28` |
| 5 | `find .specify/assessments -mindepth 1 -maxdepth 1 -type d \| wc -l` | `175` diretórios |
| 6 | contagem de arquivos por nome em `.specify/assessments/` | `171 intake.md`, `171 decision.md`, `4 research.md` |
| 7 | prefixos dos diretórios de assessment | `ast` 40 · `cor` 28 · `doc` 45 · `srv` 58 · `ev-*` 4 |
| 8 | `find . -name feature.json` (fora de `.git`) | vazio — **`feature.json` AUSENTE** |
| 9 | `python3 scripts/ci/check_sdd_completeness.py` | `PASS (spec_dirs=0 gaps=0 baselined=0 strict_from=88)` |
| 10 | `python3 scripts/ci/check_work_order_dag.py` | `PASS (files=0 tasks=0 edges=0 cycles=0)` |
| 11 | `python3 scripts/ci/check_task_id_uniqueness.py` | `PASS (tasks_files=0 duplicates=0 baselined=0)` |
| 12 | `python3 scripts/ci/check_converge_append_only.py` | `PASS (files=0 baselined=0)` |
| 13 | `python3 scripts/ci/hub_docs_gate.py` | `ERROR: missing required path: specs` → `FAIL (1 errors, 0 warnings)` — o gate espera `specs/` na raiz; a spec está em `docs/specs/` |

Os 4 checkers SDD (itens 9–12) passam **com zero artefatos** — baseline vazia, não cobertura
implementada. O item 13 é divergência de layout de diretórios, não um "gate de produção"
(que não existe no alvo); ausência de arquivo/caminho não deve ser classificada como RED.

### Inventário numérico verificado

| Métrica | Valor | Fonte |
|:--|--:|:--|
| HEAD / branch | `6e25758` / `main` | item 1 |
| Arquivos `docs/**/*.md` | 28 | item 4 |
| Diretórios de assessment | 175 | item 5 |
| Intakes (`intake.md`) | 171 | item 6 |
| Decisões (`decision.md`) | 171 | item 6 |
| Research `ev-*` (`research.md`) | 4 | item 6 |
| Prefixos `ast` / `cor` / `doc` / `srv` | 40 / 28 / 45 / 58 | item 7 |
| Specs em `docs/specs/` | 1 | listagem |
| Planos em `docs/plans/` | 1 | listagem |
| Checkers SDD PASS / FAIL | 4 / 0 | itens 9–12 |
| `feature.json` | ausente | item 8 |
| Bugs / validações / releases registrados | 0 / 0 / 0 | nenhum artefato materializado |
| `converge.md` baselines | 0 | item 12 |

### Dívidas e pendências

| ID | Dívida | Impacto | Próximo passo |
|:--|:--|:--|:--|
| D1 | `feature.json` ausente e `spec_dirs=0` | fluxo SDD não tem feature ativa; gates passam no vazio | materializar feature e spec dir antes de reivindicar cobertura |
| D2 | `hub_docs_gate.py` FAIL por `specs/` inexistente na raiz | gate utilitário não encontra a spec em `docs/specs/` | alinhar caminho esperado ao layout do alvo (sem importar gate da factory) |
| D3 | Sem código, engine, testes, build, CI ou deploy | tese do ciclo 80/20 ainda não executada | implementar a fatia vertical do plano |
| D4 | `converge.md` sem entradas (`files=0`) | histórico append-only ainda não iniciado | registrar convergências à medida que ocorrerem |
| D5 | Intakes com `[NEEDS CLARIFICATION]` | decisões registradas, mas premissas não confirmadas | resolver esclarecimentos (R3: ≤3, um por vez) |
| D6 | 39 ocorrências da etiqueta NÚCLEO vs 35 itens classificados NÚCLEO | divergência de rastreabilidade conhecida (spec, ASSUNÇÃO 9) | usar os 35 slugs reais na rastreabilidade |

### [NV] — não verificado

- Status remoto em GitHub (`origin`): commits, branches, PRs, proteção de branch, tags — **[NV]**.
- Execução de CI, merge, release, publicação, deploy e checksums — **[NV]**.
- Gates de produção da factory (`check_production_gate.py --check`, `.forgejo/**`, `pytest -q`,
  `verify.sh`, `nextest`): não materializados no alvo; não classificar como RED — **[NV]**.
- Estado de qualquer outro repositório além do alvo (inclusive a árvore de trabalho suja da
  factory, registrada apenas como fonte) — **[NV]**.

## 2026-11-19T21:46:45Z — Ciclo 001 concluído (Blocos 4–8)

- Branch `feature/001-excecao-linguagem-licenca-nucleo`; NNN resolvido por `create-new-feature.sh --dry-run` (001), não pelo nome do branch.
- Aparato SDD materializado do HEAD limpo da factory `b1deab1`: 9 scripts/templates, 18 comandos `.omp/commands/speckit.*.md`. Factory não modificada; árvore suja dela não transplantada.
- Governança local escrita: `AGENTS.md`, `.specify/memory/constitution.md`, `.specify/memory/SOURCE_FACTORY.md`.
- Ciclo 001 executado: intake→research→define→shape→decide(go)→specify→clarify(4 Q/A)→plan→checklist→tasks(35)→analyze(3 reruns, 14 achados)→implement(2 ADRs, 35/35 `[X]`)→converge(sem gaps)→R4→R7.
- Gates finais locais: `check_sdd_completeness PASS (1/0/0)`, `check_work_order_dag PASS (35 tarefas)`, `check_task_id_uniqueness PASS`, `check_converge_append_only PASS`, `hub_docs_gate PASS (0 erros)`. `pytest -q` = `no tests ran` (exit 5, N/A — sem suíte no alvo). `check_production_gate.py --check` = N/A por decisão do Bloco 0.a.
- Entregáveis: `docs/adr/001-excecao-linguagem-gdscript.md`, `docs/adr/002-licenca-mit-nucleo.md`, R4 em `.specify/validations/`, R7 (1.0.0) em `.specify/releases/`.
- `[NV]` — aprovação humana dos ADRs, PR/CI/merge/proteções no GitHub, qualquer execução remota.

## 2026-11-19T22:11:07Z — Reavaliação pós-ciclo-001 (Blocos 2–4)

- Recontagem por `find`: specs=1; assessments=176; validations=1; releases=1; bugs=0; docs/*.md=35. Comparação: `OBJETIVO.md`, `ROADMAP.md`, este `PROGRESSO.md` e `docs/research/index.md` descrevem o estado anterior (28 docs, 175 assessments); as diferenças são exatamente os artefatos criados no ciclo 001 e nesta reavaliação. Nenhuma contagem foi inventada; números anteriores permanecem como histórico.
- Âncora real: `.specify/feature.json` → `specs/001-excecao-linguagem-licenca-nucleo`; branch `feature/001-...`, HEAD `723ef49`. Nenhum ciclo em voo: R4 e R7 materializados, PR #2 `OPEN` **[NV]** para checks/merge.
- 4 checkers SDD PASS (`spec_dirs=1 gaps=0`; `tasks=35 cycles=0`; `duplicates=0`; `files=0`). Nenhum gate RED; autorizo abrir o próximo ciclo.
- Auditória (Bloco 2): funciona — gates locais, ADRs, R4/R7, 35 tarefas `[X]`; não funciona — aprovação humana dos ADRs pendente, baseline de governança ausente, verdes com `baselined=0/edges=0/files=0`; não implementado — código, Godot instalado, CI, telemetria, banco, deploy; não faz — importar gate da factory, merge cego, afirmar remoto sem `[NV]`.
- Divergências do GDD mapeadas para emenda pontual: N3 (1659, 1668, 1673), licenciamento (1451, 1459), CI (1594) e ressalva AGPL de terceiro (1389). Âncoras Godot-first (1265, 1678) NÃO serão tocadas.
- `[NV]` — checks de CI, merge, proteções, aprovações remotas; nada foi reconsultado além do estado do PR registrado no ciclo 001.

## 2026-11-19T22:38:57Z — Ciclo 002 concluído (Blocos 5–8)

- 80/20 escolheu a emenda do GDD a partir dos ADRs (score 125,0); Godot install e CI remoto ficaram DEFER por exigirem autorização de dependência/acesso remoto.
- Branch `feature/002-emenda-gdd-adrs` (NNN=002 por `create-new-feature.sh`, nunca pelo nome do branch), derivada de `feature/001-...` — PR dependente do PR #2, sem merge.
- Ciclo completo: intake→research→define→shape→decide(go)→specify→clarify(3 Q/A)→plan→checklist(50)→tasks(26)→analyze(4 achados + 6º residual)→implement(7 emendas no GDD, 26/26 `[X]`)→converge(sem gaps)→R4→R7(1.0.0).
- Verificação isolada: 7/7 passagens com ADR (ADR-002 em seis, ADR-001 em `1594`); âncoras `1265`/`1678` preservadas por leitura comparada; nenhuma formulação proibida restante.
- Gates finais locais: `sdd-completeness PASS (2/0/0)`, `work-order-dag PASS (61 tarefas em 2 features)`, `task-id-uniqueness PASS`, `converge-append-only PASS`, `hub-docs-gate PASS (0 erros)`. `pytest -q` e `check_production_gate.py` permanecem N/A conforme Bloco 0.a.
- `[NV]` — aprovação humana final, PR/CI/merge/proteções no GitHub, qualquer execução remota. Factory HEAD `b1deab1` permanece como fonte, não modificada.
