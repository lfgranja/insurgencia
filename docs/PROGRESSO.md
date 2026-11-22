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

## 2026-11-19T22:46:15Z — Resolução dos [NV] de aprovação, merge e CI

- **Autorização**: o usuário autorizou explicitamente os merges e pediu "Efetuar o necessário para resolver todos os [NV]". Nenhum código-fonte, gate ou arquivo de `converge` foi alterado por esta resolução.
- **PRs (verificado ao vivo via `gh`)**: #1 `MERGED` 2026-11-19T22:45:33Z (commit `5f18526`); #2 `MERGED` 2026-11-19T22:45:53Z (commit `741f4f1`); #3 `MERGED` 2026-11-19T22:46:15Z (commit `3e0a213`). `mergedBy=lfgranja` nos três; `gh pr list --state open` = `[]` (zero PRs abertos).
- **Branch `dev`**: HEAD `3e0a213`, igual ao commit do PR #3.
- **CI**: `statusCheckRollup=[]` nos PRs #1–#3 — nenhum check de CI configurado/reportado (verificado ao vivo; não é mais [NV]).
- **ADRs 001/002**: `Estado: Aprovado`, versão 1.0.1 — aprovação registrada por autorização explícita do usuário + `mergedBy=lfgranja` nos PRs #2/#3; nenhum review `APPROVED` emitido na plataforma e nenhum check exigido. Corpo das decisões inalterado; entrada de changelog `### [1.0.1] — 2026-11-19` acrescentada.
- **Proteção de branch (limitação conhecida)**: `gh api repos/lfgranja/insurgencia/branches/dev/protection` → HTTP 403 "Upgrade to GitHub Pro or make this repository public". Não consultável por limitação de plano — permanece **[NV]**, não registrada como verde.
- **Permanece [NV]**: publicação/release/deploy externo (nenhuma evidência ao vivo) e proteção de branch (403 por plano). Os marcadores metodológicos legítimos de classe B (execução externa) e C (salvaguarda metodológica) não foram alterados.
- **Histórico**: as seções anteriores permanecem imutáveis. Os estados `OPEN` dos PRs #2/#3 registrados acima são históricos e foram superados pelos merges; `.specify/memory/STATE.md` passou a registrar o estado vivo, com o antigo preservado sob "Anteriores".

## 2026-11-19T23:37:11Z — Ciclo 003 (baseline-governanca) implementado

- Branch `feature/003-baseline-governanca` a partir de `dev`=`a5e4131`; NNN=003 resolvido por `create-new-feature.sh`, não pelo nome do branch.
- Três baselines LOCAIS materializados em `docs/governance/`: `sdd-completeness-baseline.json`, `converge-append-only-baseline.json` e `task-id-uniqueness-baseline.json`, com `_provenance` origem `alvo-local`, head `a5e4131`, `factory_utilizada=false`. Nenhum baseline da factory foi copiado.
- Mapas vazios registrados honestamente: nenhuma lacuna histórica legítima, nenhum `converge.md` e nenhuma duplicata de `T###`; ratchet permanece ativo para o futuro.
- 12 FRs, 8 SCs, 36 tarefas `[X]`; RG-1/RG-5 condicionais não executados (estado atual com mapas vazios); RG-2/RG-3/RG-4 executados como prova de ratchet.
- Analyze READ-ONLY: 6 achados corrigidos em 3 rodadas; 0 achados pendentes na revalidação final.
- Gates locais: `sdd-completeness PASS (3/0/0)`, `work_order_dag PASS (97 tarefas/3 features)`, `task-id-uniqueness PASS`, `converge-append-only PASS`, `hub_docs_gate PASS (0 erros)`. `pytest -q` (exit 5) e `check_production_gate.py` (exit 2) permanecem N/A por decisão do Bloco 0.a.
- Ressalva: `baselined=0` permanece nos gates cujos mapas estão vazios; a prova é arquivo presente + saída colada em R4 + cenários de ratchet recusados, não contagem de entradas.

## 2026-11-20T00:09:52Z — Consolidação final (todos os branches mesclados)

- PRs #5 (ciclo 003, `f5a6b32`) e #6 (ciclo 004, `f8c98e6`) mesclados em `dev`; PR #4 (resolução [NV], `a5e4131`) mesclado às 22:57:32Z. Todos os branches locais têm 0 commits não mesclados; `gh pr list --state open` = `[]`.
- Ciclo 004: 12 fatos NV-A01–A12 verificados com 2 fontes primárias e ressalvas; classes B/C preservadas; notas de superação append-only; nenhum benchmark reclassificado. R4 aprovado, R7 1.0.0.
- Gates finais em `dev`: `sdd-completeness PASS (4/0/0)`, `work_order_dag PASS (133 tarefas/4 features, 0 ciclos)`, `task-id-uniqueness PASS`, `converge-append-only PASS`, `hub_docs_gate PASS (0 erros)`. `pytest -q`/`check_production_gate.py` permanecem N/A por Bloco 0.a.
- `[NV]` remanescentes dependem de autorização/execução externa: instalar Godot 4.7.2 e benchmarks, CI remoto/Postgres/Zitadel/Coolify, publicação/deploy, proteção de branch (API 403 por plano). Marcadores metodológicos legítimos permanecem por política.
- Fonte do aparato: factory HEAD `b1deab15f482dd85871cd6c6a857ad710b89e2a6`, lida como fonte, nunca modificada. Nenhum merge automático, force-push ou deleção de branch de terceiros.

## 2026-11-20T00:40:18Z — Ciclo 005: CI remoto, repo público e proteção de dev

- Autorização do usuário executada: `gh repo edit --visibility public` (confirmado `visibility=PUBLIC` às 00:26:47Z).
- Ciclo SDD 005 concluído (R1 go, 14 FRs, 27 tarefas, analyze sem CRITICAL, R4 aprovado, R7 1.0.0); PR #8 mesclado em `dev` (`6df9092`).
- Workflow `.github/workflows/ci-gates.yml` executa exatamente os 5 gates do alvo em PR/push para `dev`/`main`, `permissions: contents: read`, sem deploy/segredos/Forgejo/gates da factory.
- CI ao vivo: 2 execuções `completed/success` (`44647526469` push dev; `44647520124` PR) e check `gates` `completed/success` em `dev`.
- Proteção de `dev` aplicada por API (`PUT .../branches/dev/protection`) e verificada: `required_checks=["gates"]`, `strict=true`, `enforce_admins=false`, `allow_force_pushes=false`, `allow_deletions=false`. Nenhum auto-merge habilitado.
- Ressalva: publicação/release/deploy externo segue **não autorizado** e `[NV]`; install do Godot e Postgres/Zitadel/Coolify foram autorizados pelo usuário mas ainda não executados.

## 2026-11-20T01:14:03Z — Ciclos 006 e 007 + limite seguro das autorizações

- Ciclo 006: Godot 4.7.2 instalada da release oficial (`SHA512-SUMS.txt` 128-hex idêntico ao cálculo local, `CHECKSUM_OK`), binário `4.7.2.stable.official.ed1daf0bf`; benchmark headless sintético 600×64 com mediana 37,48 µs/tick (7 amostras, 22,71–42,95) — explicitamente não é 5v5 real. PR #10 mesclado (`e9b1f34`).
- Ciclo 007: Postgres local descartável `postgres:16-alpine` (16.15) com migrações, JSONB/GIN, GRANTs mínimos e teste negativo de UPDATE (`permission denied`); container removido com 0 remanescentes. PR #11 mesclado (`05d38b8`).
- PRs #1–#11 todos `MERGED`; zero PRs abertos; proteção de `dev` exigindo check `gates` operando em todos os merges recentes.
- Bloqueio real: publicação/deploy não autorizado; Postgres/Coolify remotos e Zitadel dependem de credenciais/instância fora do alvo (`MDMV/.tokens/`, Gru) — nenhuma credencial foi lida ou copiada nesta sessão. `pgmq` segue `[NV]`.
- Factory HEAD `b1deab15f482dd85871cd6c6a857ad710b89e2a6`, lida como fonte, nunca modificada.

## 2026-11-20T01:48:40Z — Auditoria de progresso com pesquisa externa e atualização documental

- Data/hora: `date -u '+%Y-%m-%dT%H:%M:%SZ'` → `2026-11-20T01:48:40Z`. HEAD local `2af7537`, branch `docs/estado-autorizacoes-2026-11-20`.
- Gates locais reexecutados nesta sessão (saídas coladas): `check_sdd_completeness` PASS (`spec_dirs=7 gaps=0 baselined=0 strict_from=88`); `check_work_order_dag` PASS (`files=7 tasks=222 edges=0 cycles=0`); `check_task_id_uniqueness` PASS (`tasks_files=7 duplicates=0 baselined=0`); `check_converge_append_only` PASS (`files=0 baselined=0`); `hub_docs_gate` PASS (`0 errors, 0 warnings`). `python3 -m pytest -q` → `no tests ran` (exit 5, N/A). `command -v godot` → AUSENTE no PATH.
- Remoto verificado ao vivo via `gh` (somente leitura): `visibility=PUBLIC`, default branch `main`; 12 PRs `MERGED` (#1–#12), 0 abertos; 10/10 execuções `deterministic-gates` `completed/success`; proteção de `dev` com `strict=true` e `contexts=["gates"]`. Publicação/deploy externo permanece **[NV]**.
- Inventário interno (contagens verificadas): 7/7 specs com R1/R4/R7; 89 FRs; 222/222 tarefas `[X]`; 63 SCs; 7/7 `validate.md` e `release.md`; 182 diretórios de assessment; 46 arquivos `docs/**/*.md` após a pesquisa. Ciclo 80/20: 5 critérios CE / 28 subcritérios / 6 Tasks, 0 executados; 39 caminhos canônicos, 4 existentes; 0 `.gd` fora do benchmark; ausentes `Makefile`, `manifesto.json`, `docs/progress/evidence.csv`.
- Pesquisa externa (skills `research` / `source-driven-development` / `documentation-and-adrs`; `npx skills find/use`): três notas em `docs/research/` com fontes primárias e idiomas EN/PT-BR/ES/JA — `2026-11-20-godot-headless-oficial.md`, `2026-11-20-alternativas-netcode-oficial.md`, `2026-11-20-paisagem-foss-licencas.md`. Godot 4.7.2 confirmado por fonte primária (18/08/2026, commit `ed1daf0`); ADR-001 permanece válida, sem nova ADR; wire de alto nível do Godot é detalhe de implementação → envelope próprio; `--script` exige `extends SceneTree`; 30→20 Hz exige acumulador; federação e deploy seguem **[NV]**. Nenhuma tentativa de injeção de prompt detectada; nenhuma credencial copiada.
- Documentos atualizados: `OBJETIVO.md` (§§2, 2A, 2B, 3, 4, 5), `ROADMAP.md` (§§2, 2A, 3, 4, 5), `docs/research/index.md`, `.specify/memory/STATE.md`. Nenhuma transição de estado SDD; nenhum merge, commit, `git add` ou alteração de gate.
- Síntese: **funciona** = aparato SDD/CI/gates; **não implementado** = todo o produto (Núcleo, Módulos, ferramentas, testes, evidência); **[NV]** = deploy, serviços remotos, federação, engine executável nesta sessão.

## 2026-11-20T02:02:49Z — Ciclo 008 (CE-1/Task 1) concluído; PR #13 aberto

- Âncora resolvida por `create-new-feature.sh --dry-run` (NNN=008, nunca pelo nome do branch): `.specify/feature.json` → `specs/008-nucleo-contrato`; branch `feature/008-nucleo-contrato`; commit `f5d9634` (20 arquivos, 489 inserções).
- R1 `go` (`.specify/assessments/nucleo-contrato/decision.md`); spec/plano/tasks/checklist/analyze/convergência materializados; R4 aprovado (`.specify/validations/nucleo-contrato/validate.md`); R7 `1.0.0` (`.specify/releases/nucleo-contrato/release.md`).
- Diff de `project.godot` registrado antes da edição em `specs/008-nucleo-contrato/project-godot-diff.md` (regra de não sobrescrever sem diff explícito).
- Evidência local: Godot `4.7.2.stable.official.ed1daf0bf` executado (`--headless --version`, exit 0); `test_contrato.gd` → 16 `PASS` + `OK test_contrato` + exit 0; `sha256sum modulos/exemplo/regras.cfg` = `b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746` (218 bytes); `git diff --stat -- nucleo/` vazio; `grep -c '"assets": \[\]'` = 1.
- Analyze READ-ONLY encontrou 3 divergências de escopo (spec, plano e tasks prometiam 39/39 caminhos); corrigidas na fonte para 14/39 presentes e 25 pendências explícitas das Tasks 2–6. Nenhuma reivindicação de `OK_CAMINHOS` completo.
- Gates finais (02:02:49Z): `hub_docs_gate PASS (0 erros)`, `check_sdd_completeness PASS (spec_dirs=8 gaps=0)`, `check_converge_append_only PASS (files=0)`, `check_task_id_uniqueness PASS (duplicates=0)`, `work_order_dag PASS (tasks=222 cycles=0)`. `pytest -q` = exit 5 (N/A, sem suíte); `check_production_gate.py --check` = N/A por Bloco 0.a.
- Remoto verificado ao vivo: PR #13 `OPEN` (base `dev`), check `gates` `SUCCESS` (`44657956734`). **Sem merge, auto-merge, force-push ou deleção** — aprovador distinto e revisão deliberada permanecem **[NV]**. PRs #1–#12 `MERGED`, 0 abertos antes deste.
- Próximo 80/20: Task 2–3 (servidor headless + validação autoritativa, CE-2), agora desbloqueada pelo contrato. Requererá novo ciclo SDD e PR próprio.

## 2026-11-20T03:09:54Z — Ciclo 009 (CE-2/Tasks 2-3) concluído e mesclado

- Branch `feature/009-servidor-validacao` (NNN=009 por `create-new-feature.sh --dry-run`); commit `b918515` (20 arquivos, 579 inserções); PR #14 `MERGED` em `dev` às 03:09:54Z, check `gates` SUCCESS observado ao vivo. Autorização de merge fornecida pelo prompt de orquestração.
- R1 `go`; spec/plano/tasks/checklist; R4 aprovado (`.specify/validations/servidor-validacao/validate.md`); R7 `1.0.0`.
- Evidência local: `SERVIDOR_PRONTO porta=7777`; suites `test_contrato` 16 PASS, `test_servidor` 6, `test_loopback` 8 (20 snapshots/1 s), `test_validacao` 9 — total 39 PASS, 0 FAIL, exit 0. Godot `4.7.2.stable.official.ed1daf0bf`.
- Três ondas de agentes (artefatos SDD / código / mapeamento GDD-[NV]) e três corretivas determinísticas por erro real observado; nenhuma correção afirmada sem comando de verificação.
- Gates finais (03:08:52Z): 5/5 PASS (`spec_dirs=9`, `gaps=0`, `tasks=222`, `cycles=0`, `0 erros`). `pytest -q` = exit 5 (N/A); `check_production_gate.py` N/A por Bloco 0.a.
- GDD: `### 16.7 Checklist de implementação` inserida após a linha 1567, preservando §16.1–16.6 e §17+; marcação conservadora com **[NV]** em conexão UDP real, deploy e federação.
- **[NV]** remanescentes: conexão UDP real entre clientes, Ilha Meio (CE-3), manifesto (CE-4), evidência (CE-5), deploy/serviços remotos e aprovador distinto. Próximo 80/20: Task 4 (Ilha Meio).

## 2026-11-20T03:21:31Z — Fatia 80/20 concluída (ciclos 008–012 mesclados)

- Ciclos 010 (PR #15, 03:14:25Z), 011 (PR #16, 03:18:08Z) e 012 (PR #17, 03:21:19Z) mesclados em `dev`, cada um com check `gates` SUCCESS observado ao vivo. Autorização de merge fornecida pelo prompt de orquestração; retry aplicado no PR #15 após falha transitória do CLI.
- Task 4: IlhaSincronia + mapa + regra de produção; 56 PASS em 6 suites, `DELTA_EVENTO_MS 0`. Task 5: manifesto + gate local; 7 PASS, `COMPAT_OK`, divergência de hash recusada e arquivo restaurado; SC-004 corrigido para `make_exit=2` real. Task 6: `make check` exit 0 (5 `OK` + `COMPAT_OK`), CSV 9 colunas com 1 `NAO_VERIFICADO`, registro datado.
- Correções determinísticas ao plano registradas nos R4: `sha256_string()` → `HashingContext` (009/011), acumulador 30→20 Hz com tolerância (009), `Engine.get_main_loop().get_multiplayer()` (009), `make_exit=2` (011).
- Verificação final (03:21:31Z): 5 gates PASS (`spec_dirs=12`, `files=12`, `tasks=222`, `cycles=0`); `make check` exit 0; 17/17 PRs `MERGED`, 0 abertos; 12/12 execuções CI `success`; 12 specs / 12 validations / 12 releases / 47 docs Markdown.
- IANA CSV consultado sem entrada específica para porta 7777 — marcador permanece **[NV]** quanto a registro, sem afirmar indisponibilidade.
- GDD `16.7 Checklist de implementação`: CE-1..CE-5 marcados; Ilha Topo/Inferior, funções, vitória Central e 5v5 permanecem `[ ]`; deploy/serviços remotos/federação **[NV]**.

## 2026-11-20T10:05:42Z — MVP completo e [NV] de UDP real fechado

- Ciclos 013 (PR #18), 014 (PR #19), 015 (PR #20), 016 (PR #21) e 017 (PR #22) mesclados em `dev` com check `gates` SUCCESS observado ao vivo. Autorização de merge fornecida pelo prompt de orquestração.
- Entregas: Ilha Topo (16 `PASS`), Ilha Inferior/Materiais (16 `PASS`), Artesão/Guerreiro (18 `PASS`), Ilha Central/vitória (18 `PASS`) e conexão UDP real de 2 processos (`make test-rede-e2e` exit 0, envelope JSON próprio via `send_bytes`/`peer_packet` na porta 7780).
- Correções determinísticas registradas nos R4: preload de `IlhaSincronia` restaurado (013), inferência `: Mundo` (013), variável `i` e limite de loop de recarga (015), `Time.get_ticks_msec()` no lugar de `OS.get_ticks_msec()` (017).
- Verificação final: 39/39 caminhos canônicos; 5 gates PASS; `make check` exit 0; `make test-rede-e2e` exit 0; 131 `PASS` em 11 suítes, 0 `FAIL`; 22/22 PRs `MERGED`, 0 abertos; 15/15 CI `success`.
- GDD `16.7`: 18/18 critérios de aceitação do MVP marcados; CE-1..CE-5 marcados; escopo de produção (6 ilhas/20 s, 5v5) e federação permanecem fora / **[NV]**.
- Evidência `docs/progress/evidence.csv`: UDP real promovido a `PASS`; novo `EV-20261120-009` registra **[NV]** de NAT/clientes externos, preservando a regra de pelo menos uma linha não verificada.

## 2026-11-20T10:58Z — Diretrizes da SE: CI green, IAMAdapter e escopo solo

- **Regra de merge alterada:** CI/CD verde observado é autoridade suficiente para merge autônomo em `dev`/`main` via `gh`, sem aprovação humana. Registrada em `.specify/memory/constitution.md:22` e `AGENTS.md:60`. PRs #13–#22 aprovados retroativamente pela autorização da SE. Se o CI não inspirar confiança, o próximo 80/20 imediato é reforçá-lo.
- **Zitadel substituído** por IAMAdapter/IAMInterface: ADR-003 e contrato em `docs/architecture/iam-interface.md`. Registros históricos (ciclo 007, append-only) mantêm a menção antiga como contexto datado.
- **Escopo solo/single confirmado:** sem 5v5, sem lobby, sem testes de Internet/NAT nesta fase. Roteiro de validação para a SE em `docs/ROTEIRO-TESTE-SOLO.md`, com script `tests/roteiro_solo.sh`.
- Marcadores remanescentes continuam **[NV]** por exigirem ambiente, credencial ou decisão de produto; nenhum foi reclassificado como verde por inferência.

## 2026-11-20T11:34:04Z — Ciclo 018 (IAMAdapterLocal) concluído e mesclado

- Branch `feature/018-iam-adapter-local` (NNN=018 por `create-new-feature.sh --dry-run`); PR #24 `MERGED` em `dev`, check `gates` SUCCESS observado ao vivo, sob a regra de merge autônomo com CI verde. Total 24 PRs mesclados, 0 abertos.
- Todas as fases do SDD executadas em ordem, sem pular: intake, research, problem, concept, decision (`go`), specify com `## Clarifications` Q1–Q4, plan, checklist (25 itens de revisão), tasks T001..T010, analyze READ-ONLY (6 achados), implement, converge append-only, R4 aprovado e R7 `1.0.0`.
- Entrega: IAMInterface (4 métodos, 9 códigos) e IAMAdapterLocal determinístico em processo, sem servidor, segredo, rede, credencial persistida ou login obrigatório; 6 permissões locais concedidas, todo o resto negado como caminho normal.
- Evidência: `test_iam_adapter_local` 36 `PASS`, `OK`, exit 0; regressão 131 `PASS` em 11 suítes, 0 `FAIL`; `make check` exit 0; 5 gates PASS (`spec_dirs=18`, `converge files=1 baselined=1`).
- Quatro correções determinísticas ao longo da verificação: herança por `class_name` → caminho literal; anotações de tipo → constants de preload; `\t` literal → TAB; bootstrap do baseline de converge executado pelo próprio gate após RED legítimo `UNBASELINED`. Todas registradas nos R4/converge.
- GDD `16.7` recebe marcação de IAM local; provedor real, federação e deploy permanecem **[NV]**.

## 2026-11-20T11:52:26Z — Ciclo 019 (tela de resultado) concluído; MVP fechado

- Branch `feature/019-tela-resultado` (NNN=019 por `create-new-feature.sh --dry-run`); PR #25 `MERGED` em `dev`, check `gates` SUCCESS observado ao vivo, sob a regra de merge autônomo com CI verde. Total 25 PRs mesclados, 0 abertos.
- Todas as fases executadas em ordem, sem pular: intake, research, problem, concept, decision (`go`), specify com Q1–Q4, plan, checklist (16 itens), tasks T001..T010, analyze READ-ONLY, implement, converge append-only, R4 aprovado e R7 `1.0.0`.
- Entrega: `nucleo/apresentacao/tela_resultado.gd` (Control programático, sem .tscn/main scene; AGUARDANDO por padrão; VITORIA somente com evento do servidor) e `tests/test_tela_resultado.gd` (19 `PASS`).
- Correções determinísticas registradas no converge: divergência impacto/timestamp no `decision.md`, estado inicial da tela e `DirAccess.close()` inexistente no Godot 4.
- Evidência: 19 `PASS` tela + 167 `PASS` em 12 suítes, 0 `FAIL`; `make check` exit 0; 5 gates PASS (`spec_dirs=19`, `converge files=2 baselined=2`). Bootstrap do baseline de converge executado pelo próprio gate após RED legítimo `UNBASELINED`.
- Porta 7777: CSV oficial IANA consultado às 11:38:06Z sem entradas `7777/tcp` ou `7777/udp`; marcador informativo resolvido, sem afirmar disponibilidade em rede pública.
- GDD `16.7`: tela de resultado marcada; restam fora do MVP/NAT/lobby/5v5/deploy/serviços remotos/federação **[NV]**.

## 2026-11-20T23:52:56Z — Ciclo 020 (engines Rust) concluído e mesclado

- Branch `feature/020-engines-rust` (NNN=020 por `create-new-feature.sh --dry-run`); PR #26 `MERGED` em `dev`, check `gates` SUCCESS observado ao vivo. Total 26 PRs mesclados, 0 abertos.
- Protocolo factory 019/040 (HEAD `b1deab15`, lido como fonte): ≥10 critérios, Σ pesos=100%, escala 0–10, score ponderado, ≥10 opções, Fase 0 com C1–C3 invioláveis, evidência primária e `[NV]`.
- Entrega: `docs/AVALIACAO-ENGINES-RUST.md` com 10 opções (Godot baseline + 9 Rust/relacionadas), scores recalculados (82,6 / 77,1 / 76,4 / 68,6 / 66,8 / 65,2 / 60,4 / 57,0 / 54,8 / 54,5), 32 linhas de evidência e spikes A/B/C. Relatórios de pesquisa em `docs/avaliacao-engines/pesquisa/`.
- Conclusão: Godot-first (ADR-001) permanece; Bevy 0.20.1+Lightyear 0.31 é ADAPT apenas como padrões em GDScript, não como troca de engine. Fyrox/macroquad/ggez/Rapier travados em C2/C3; Ambient pausada; gdext é binding, não engine.
- Correções deterministicas: versões Bevy 0.19.1→0.20.1 e Lightyear 0.30→0.31; divergência Fyrox 62,6/65,2 resolvida no relatório dedicado; categoria de gdext corrigida.
- Gates: 5/5 PASS (`spec_dirs=20`, `converge files=3 baselined=3` após bootstrap do próprio gate). Sem código, build, benchmark ou migração de motor neste ciclo.

## 2026-11-21T00:22Z — Prioridade da SE redefinida: prova da tese

- Interjeição da SE substitui a fila anterior: entregável = **MVP jogável single-player** que valide o que de fato prova a tese do GDD. Ordem: (1) cliente greybox jogável com main scene, câmera e ilhas geométricas; (2) duas partidas consecutivas, uma com cliente modificado (GDD:1567); (3) instrumentação no Núcleo + 20 partidas com as métricas da §17; (4) exportação de conta/personagem, `COPYRIGHT.txt` e commits assinados (GDD §15.9).
- **DEFER explícito:** (5) dois servidores com Módulos diferentes e teste de queda do servidor de referência (GDD:50–53); (6) servidores comunitários e forks publicados — dependem de outras pessoas, não de código.
- Stack mantida: Bevy 0.20.1 + Lightyear 0.31 (ADR-004), rust-toolchain 1.99.1, crates `contratos` e `sim` mesclados (PRs #27 e #28). O ciclo 023 anterior (servidor Lightyear isolado) foi cancelado antes do commit; seus artefatos untracked foram removidos e os gates restaurados (5/5 PASS; `verify.sh` ALL STAGES OK).

## 2026-11-21T00:24Z — Ciclo 023 reaberto para cliente greybox

- Branch `feature/023-cliente-greybox`, NNN=023 por `create-new-feature.sh --dry-run`; `feature.json` → `specs/023-cliente-greybox`.
- Artefatos SDD e código Bevy do cliente entregues por agentes: `App` com `DefaultPlugins`, `Camera2d`, avatar com input WASD/setas, quatro ilhas geométricas (Topo/Meio/Inferior/Central) e painel de texto.
- Resolução de dependência: `thiserror` unificado em `=2.0.21` nos crates `contratos` e `sim` (pin exato) para atender à cadeia Bevy 0.20.1 → `bevy_shader` → `wesl` → `wgsl-parse` (`^2.0.12`); nenhum pacote foi substituído por nome similar.

## 2026-11-21T01:26Z — Entregável da SE concluído (itens 1–4)

- PR #29 (ciclo 023): cliente greybox Bevy jogável, janela minizada; 20 testes Rust. GPU Vulkan detectada e janela criada sem erro. Inspeção visual **[NV]**.
- PR #30 (ciclo 024): instrumentação no Núcleo, 20 partidas com métricas §17 e corredor de duas partidas com cliente modificado (GDD:1567); 28 testes. Relatório datado em `docs/evidencias/metricas-20-partidas-2026-11-21.md`.
- PR #31 (ciclo 025): exportação versionada, `COPYRIGHT.txt` e DCO; 34 testes. Assinatura GPG **[NV]** (sem chave no ambiente).
- GDD `16.7` atualizado com migração ADR-004 e os 4 itens; itens 5 e 6 permanecem DEFER por decisão da SE.
- Verificação final: 5/5 gates do alvo PASS, `verify.sh` ALL STAGES OK, 31/31 PRs mesclados, 0 abertos.

## 2026-11-21T01:57Z — Confirmação visual da SE sobre o cliente greybox

- Observação direta da SE: o jogo abre, o avatar se movimenta com WASD/setas e, ao passar sobre as ilhas, o painel identifica a região. Registrado como verdade observada, sem reexecução por agente.
- Resolve o marcador `[NV]` de inspeção visual do greybox constante do R4 do ciclo 023 e do GDD `16.7`. O limite remanescente e tecnico de captura (ponte Xwayland gera imagem preta), nao do jogo.

## 2026-11-21T02:28:07Z — Ciclo 026 (cliente low-poly 3D) mesclado

- PR #32 `MERGED` em `dev`, check `gates` SUCCESS; total 32 PRs mesclados, 0 abertos. NNN=026 por `create-new-feature.sh --dry-run`.
- Item 1 redefinido pela SE de "greybox" para "low-poly": `main.rs` migrado de `Camera2d`/`Sprite` para `Camera3d`, iluminação e malhas procedurais (cubos/prismas/avatar cone), sem textura; `lib.rs` puro e seus 5 testes preservados.
- GDD §13.2 e limite ≤10k triângulos (`:1127`) atendidos por construção; Kenney CC0 registrado como fonte futura, não baixado (host fora da allowlist).
- Gate Rust: `verify.sh` ALL STAGES OK, 34 testes; 5 gates do alvo PASS. Binário rodou minizado por 18s com Vulkan e janela criada sem erro. Inspeção visual da SE **[NV]**.
## 2026-11-21T02:55Z — Ciclo 027 (low-poly visual real) — PR #33 mesclado

- Correção do report da SE: o ciclo 026 entregou primitivas 3D (greybox), não low-poly visual. `main.rs` reescrito com malhas irregulares: terreno 16×10 (320 triângulos), ilhas com contorno de 8 vértices e taludo (32 triângulos), 3 árvores + 2 rochas por ilha e avatar com base/corpo/cabeça icosaédrica; 4 cores preservadas, sem textura.
- Bug encontrado e corrigido: B0004 do Bevy — pai do avatar sem `Visibility` tornava os 3 filhos invisíveis (conforme doc oficial). Corrigido com `Visibility::default()`; `Transform::IDENTITY` adicionado aos spawns de terreno/topo/taludo. Também corrigidos E0308 (`cor: [f32; 3]`) e lints de precisão do hash.
- `lib.rs` intacto (5 testes); `verify.sh` ALL STAGES OK, 34 testes nextest; 5/5 gates do alvo PASS; runtime minizado com 0 avisos/0 erros, Vulkan RADV RAVEN.
- Janela normal aberta para inspeção da SE (PID 696504); inspeção visual **[NV]** até confirmação. Regra da janela inalterada.

## 2026-11-21T04:23Z — Estado verificado (orquestração autônoma, Blocos 0–4)

- Âncora real: `feature.json` → `specs/027-lowpoly-real`; HEAD `2587647` em `dev`; factory fonte `b1deab15` (não transplantada). Nenhum PR aberto, nenhum worktree extra.
- Gate autoritativo do alvo: `verify.sh` ALL STAGES OK, 34/34 testes (4 estágios). Gate da factory (`check_production_gate.py`) ausente → N/A conforme Bloco 0.a. CI: GitHub Actions `ci-gates.yml`, CLI `gh` — divergência da factory (Forgejo/`fj`) registrada em `SOURCE_FACTORY.md`.
- 5/5 gates SDD PASS: sdd-completeness (27 specs, 0 gaps), work-order-dag (232 tasks, 0 ciclos), task-id-uniqueness (0 duplicatas), converge-append-only (10 arquivos), hub-docs-gate (0 erros). `pytest -q` = exit 5 (N/A — sem suíte Python).
- Contagens: 27 specs, 202 assessments, 27 validations, 27 releases, 0 bugs, 4 research. `docs/PROGRESSO.md` é append-only; nota preserva histórico.
- Dívidas documentais detectadas: `AGENTS.md` desatualizado (afirma "sem código/verify.sh" quando existem 5 crates Rust e gate verde); cabeçalho de `verify.sh` copiado do `casv-rust`. Não corrigidas nesta nota — candidatas ao 80/20.
- Status remoto/CI/merge/proteções: **[NV]** nesta auditoria (sem confirmação ao vivo via UI/API além do gate local e do `gh pr view` do ciclo anterior).

## 2026-11-21T04:35Z — Orquestração: confirmação visual, regra da janela e CI 028

- **Inspeção visual da SE confirmada por imagem anexada às 04:33Z**: cena low-poly visível — terreno irregular triangulado, 4 ilhas com taludos/cores (azul, verde, violeta, laranja), árvores e rochas em cada ilha, avatar com corpo/cabeça e HUD. Resolve o `[NV]` de inspeção visual do ciclo 027 (SC-005). Captura de agente via `spectacle -b -n -f` funciona em Wayland (`/tmp/test-spectacle.png`, PNG 1366×768) — o limite técnico anterior de "imagem preta via Xwayland" não se aplica.
- **Regra da janela reforçada pela SE às 04:33Z**: agente SEMPRE roda o cliente com `--minimizado` em desenvolvimento, depuração, inspeção, teste, validação e segundo plano. **NUNCA** abrir janela normal enquanto a SE trabalha no notebook. Janela normal apenas sob pedido explícito e momentâneo. Registrada em `AGENTS.md` §Development Commands e §Testing & QA; janela aberta pelo agente foi encerrada imediatamente.
- **CI do PR #34 (ciclo 028)**: job `gates` SUCCESS; job `rust` FAILURE em `Setup Rust` — `dtolnay/rust-toolchain@master` exige input `toolchain` e não lê `rust-toolchain.toml`. Corrigido com `toolchain: '1.99.1'` explícito, espelhando a fonte local. Artefatos SDD revisados (spec/plan/analyze C6/validate/release/converge append-only). Status remoto após correção: **[NV]** até check verde observado.

## 2026-11-21T04:43Z — Ciclo 028: segunda falha do CI corrigida

- CI run `44866958727` (após `toolchain: '1.99.1'`): job `gates` SUCCESS; job `rust` FAILURE em `verify.sh` stage 1 — `wayland-sys` panicked: `Package 'wayland-client' not found` no runner Ubuntu (Fedora local tem a lib). Corrigido com step `Install system libraries`: `pkg-config libwayland-dev libxkbcommon-dev libx11-dev libudev-dev`.
- Verificação local após correção: YAML parse (4 steps no job rust), `pkg-config --exists wayland-client` OK, `./verify.sh` ALL STAGES OK, 5/5 gates SDD PASS. Status remoto do novo check: **[NV]** até observação ao vivo.

## 2026-11-21T04:58Z — Ciclo 028 mesclado (PR #34)

- PR #34 `MERGED` em `dev`, merge commit `d2b23e4`; total 34 PRs mesclados, 0 abertos. Ambos os checks obrigatórios (`gates`, `rust`) SUCCESS observados ao vivo no run `44868073016` (04:56:22Z).
- Ciclo 028 completo: R1 go, R4 aprovado, R7 1.0.0. Três falhas reais do CI corrigidas: input `toolchain` ausente, `wayland-sys` sem `libwayland.pc`, `alsa-sys` sem `alsa.pc`. O job `rust` agora executa `verify.sh` com toolchain fixada e conjunto completo de libs Bevy no Ubuntu.
- Autoridade documental alinhada: `AGENTS.md` reescrito (stack Rust, 34 testes, regra da janela reforçada), `verify.sh:2` e GDD:1627 corrigidos. A factory foi usada apenas como fonte (HEAD `b1deab15`), nada transplantado.

## 2026-11-21T05:12Z — Orquestração concluída (Blocos 0–8)

- 4 agentes de pesquisa paralelos (core src, tests, configs/build, scripts/docs) sintetizaram um único `AGENTS.md` na raiz, com as 8 seções obrigatórias. Correção material: `Makefile` **existe** (alvos Godot legados) — a versão anterior afirmava o contrário; regra da janela reforçada preservada (agente sempre `--minimizado`, nunca janela normal). `hub_docs_gate.py` PASS.
- Ciclo 029 (HUD glyphs) mesclado via PR #35 (`970af4e`, 05:11:50Z): duas strings ASCII seguro, verificador isolado PASS, `verify.sh` ALL STAGES OK (34 testes), 5/5 gates. Ambos os checks SUCCESS observados ao vivo (run `44869231266`). Total 35 PRs mesclados, 0 abertos.
- Estado final: HEAD `970af4e` em `dev`; factory usada apenas como fonte (HEAD `b1deab15`), nada transplantado; trabalho alheio untracked (ast-*, cor-*, doc-*, ev-*, srv-*) preservado.
- **[NV]** remanescente: confirmação visual final da SE sobre o HUD ASCII; itens 5-6 do entregável, assinatura GPG, deploy/serviços remotos — todos exigem intervenção/autorização humana. Fila 80/20 esgotada para trabalho autônomo no alvo.


## 2026-11-21T13:09Z — Assess do vídeo Slotbound (28 propostas) + ciclo 80/20 `sb-16`

- Assess completo: 14 subagentes em segundo plano produziram 8 notas de evidência multilíngue (EN/PT/ES/ZH/JA/KO) + síntese em `docs/research/slotbound-2026-11-21-*.md` e 5 artefatos por proposta em `.specify/assessments/sb-*` (28/28). Vereditos: 3 ADOPT, 19 ADAPT, 4 DEFER, 2 AVOID; nenhuma proposta inteira REJECT (`sb-26` só tem subparte REJECT). Skills `research`, `competitive-brief`, `novelty-assessment`, `research-intake` e `kickstart-assess` lidas via `npx skills use`, nenhuma instalada.
- Correções primárias sobre o resumo/vídeo: pico de wishlists veio da **demo** (~600 → ~7.000 em 3 dias), não do Next Fest; Valve documenta "Wishlists (Mostly) Aren't a Factor"; voz IA, stills animados e "walk-back" são inferências de Garry Seto, não evidência primária (post original não tem marcador de edição); 68/93/79/77 por idioma `[NV]`, somente 79% global confirmado (1.598 reviews em 21/11).
- Ciclo 80/20 executado: `crates/sim/examples/demo_deterministica.rs` (novo) — simulação headless de 30 ticks com dois jogadores e hash por tick, sem Bevy, janela, relógio ou rede; coberto por `verify.sh --all-targets` quando o cargo estiver livre.
- Verificação observada: `rustc --edition 2021 -D warnings` PASS; `clippy-driver -D warnings -W clippy::pedantic` `CLIPPY_EXIT=0`; duas execuções idênticas (`DETERMINISTIC_OK`), `hash_final=6fcc2f9e652da1834e213f6afa2eb58d306bfbc5846060ec0513b67f68294f12`.
- **[NV]**: `verify.sh` completo e CI/merge remotos — tentativas de 180 s e 600 s estouraram por processos `cargo` concorrentes de outros repositórios na máquina; não houve afirmação de verde sem saída executada (constituição VI). Factory `mdmv-dark-factory` usada apenas como fonte de ideias/scripts; nada transplantado. Trabalho alheio untracked preservado.
- Fechamento do ciclo 030: artefatos SDD completos (spec/plan/tasks/checklist/converge + R4/R7) e assessment equivalente; baseline `docs/governance/converge-append-only-baseline.json` atualizado por bootstrap determinístico. Gates Python reexecutados: `sdd-completeness PASS (30/0)`, `converge-append-only PASS (13/13)`, `work-order-dag PASS (262/0)`, `task-id-uniqueness PASS (0 duplicatas)`, `hub_docs_gate PASS (0 erros)`. `verify.sh` completo segue **[NV]** — máquina continuamente ocupada por `cargo` de outros repositórios.

## 2026-11-21T13:23Z — Fechamento de rastreabilidade dos ciclos 028–030 (correções documentais)

- **Gate Rust observado (`2026-11-21T13:21Z`)**: `./verify.sh` → `stage 1 ok (62s)`, `stage 2 ok (160s)`, `34 tests run: 34 passed`, `stage 4 ok (5s)`, `ALL STAGES OK`, `VERIFY_EXIT=0` (4 estágios: check → clippy → nextest → fmt).
- **5 gates Python (`2026-11-21T13:18:27Z`)**: `check_sdd_completeness` PASS (`spec_dirs=30 gaps=0`); `check_work_order_dag` PASS (`262 tarefas/0 ciclos`); `check_task_id_uniqueness` PASS (`0 duplicatas`); `check_converge_append_only` PASS (`13/13`); `hub_docs_gate` PASS (`0 erros`). `check_production_gate.py` da factory é ausente no alvo → exit 2 = N/A (Bloco 0.a).
- **Remoto ao vivo via `gh` (`2026-11-21T13:19:29Z`)**: repo público, default `main`; **0 PRs abertos**; 5 runs `deterministic-gates` `success`; proteção de `dev` com required check `gates`, `enforce_admins=false`, sem reviews; `main` desprotegido (404). `git ls-remote origin refs/heads/dev` = `3709c56` = HEAD local.
- **Errata de redação (`2026-11-21T13:19Z`)**: `grep` em `verify.sh:5-15,70,75` mostra que **não há flag `--all-targets`** — o script a executa internamente. A expressão `verify.sh --all-targets` estava sintaticamente errada e foi corrigida para `./verify.sh` (que executa `--all-targets` internamente) em 8 arquivos: `specs/030-demo-deterministica-headless/{spec,plan,tasks}.md` (Analyze), `checklists/requirements.md`, `.specify/validations/demo-deterministica-headless/validate.md`, `.specify/releases/demo-deterministica-headless/release.md`, `.specify/memory/STATE.md`, `docs/research/slotbound-2026-11-21-sintese.md`. Errata registrada por append em `specs/030-demo-deterministica-headless/converge.md`.
- **Precisão de manifestação**: `grep -rni lightyear Cargo.toml Cargo.lock crates/ .github/` → nenhuma ocorrência; Lightyear 0.31 consta apenas em ADR-004/docs. `AGENTS.md` ajustado para "declarado em ADR-004; ausente de `Cargo.toml`/`Cargo.lock` (verificado 21/11)".
- **Tarefas herdadas fechadas com notas datadas**: `specs/029-hud-glyphs/tasks.md` T004–T008 `[x]` (PR #35 `MERGED` `970af4e`, CI `44869231266` `success`, confirmação visual da SE em `STATE` às 04:33Z); `specs/028-ci-rust-governanca/tasks.md` T011 `[x]` (PR #34 `MERGED` `d2b23e4`, CI `44868073016` `success`). Seções Analyze existentes preservadas.
- Factory fonte `mdmv-dark-factory` HEAD `b1deab1`, usada apenas como fonte; nada transplantado. Gate autoritativo do alvo = `./verify.sh` + 5 gates Python; CI GitHub Actions, CLI `gh`. Nenhum gate, build, commit ou PR executado por estas correções; nenhuma linha anterior de `converge.md`/`PROGRESSO.md` alterada.

## 2026-11-21T20:25Z — Merge do PR #36 e endurecimento da proteção de `dev` (autorização SE)

- **Re-leitura (20:24:58Z)**: PR #36 `OPEN`, `gh pr checks 36` → `gates pass 9s`, `rust pass 3m48s`; proteção de `dev` antes da mutação: `contexts=["gates"]`, `strict=true`, `enforce_admins=false`, `reviews=null`, `restrictions=null`. `HEAD local=fe82843`; factory fonte `b1deab1`.
- **Gates do alvo (20:25:17Z)**: `./verify.sh` → `ALL STAGES OK`, `34 tests run: 34 passed`, `VERIFY_EXIT=0`; 5 gates Python PASS (SDD `31/0`, DAG `274/0`, task-id `0 duplicatas`, converge `14/14`, hub-docs `0 erros`). Factory `check_production_gate.py` ausente = exit 2, N/A (Bloco 0.a).
- **Merge (20:25:32Z)**: `gh pr merge 36 --merge` → `MERGE_EXIT=0`; `gh pr view 36` → `state=MERGED`, `mergedAt=2026-11-21T20:25:35Z`, `mergeCommit=ce2d1f077b31fd0b6e16cc298ecf1406ca2fc9a2`; `git ls-remote origin refs/heads/dev` = `ce2d1f0...`; 0 PRs abertos. Autorização expressa do SE para merge com CI verde.
- **Endurecimento (20:25:48Z)**: `gh api -X PUT repos/lfgranja/insurgencia/branches/dev/protection --input -` com `{"required_status_checks":{"strict":true,"contexts":["gates","rust"]},"enforce_admins":false,"required_pull_request_reviews":null,"restrictions":null}` → resposta `contexts=["gates","rust"]`, `strict=true`, `enforce_admins=false`, `reviews=null`, `restrictions=null`, `PUT_EXIT=0`. GET de verificação confirmou `contexts=["gates","rust"]`, `strict=true`, `enforce_admins=false` em `https://api.github.com/repos/lfgranja/insurgencia/branches/dev/protection`. Autorização expressa do SE.
- **[NV] remanescentes**: confirmação visual final da SE, deploy/publicação (DEFER), assinatura GPG (DEFER). Nenhum outro remoto mutado; nenhum branch de terceiro deletado; auto-merge permanece desligado.

## 2026-11-21T21:31Z — Ciclo 033: validador de manifesto de compatibilidade em Rust

- Âncora: branch `033-manifesto-compatibilidade-rust` sobre `de3cfb3`, `feature.json → specs/033-...`; factory `b1deab1` apenas fonte. Contrato existente preservado: 6 campos de `manifesto.json`, códigos literais e ordem de `specs/011`/`nucleo/compatibilidade/manifesto.gd`. Nenhum esquema novo inventado.
- Artefatos SDD 033: assess `go`, spec/plan/tasks T-IDs/checklist/analyze READ-ONLY/converge append-only, R4 aprovado, R7 `1.0.0` com changelog FR/T e rollback. Código: `crates/contratos/src/manifesto.rs`, `pub mod manifesto;`, `crates/contratos/tests/manifesto.rs` (7 testes). Sem novas dependências.
- REDs reais corrigidos: `clippy::double_must_use` em `manifesto.rs:77,124` (removidos os `#[must_use]` redundantes) e `cargo fmt --all -- --check` (formatador aplicado). Ambos registrados por append no converge; baseline reatualizado por bootstrap.
- Verificação final (21:31:15Z): `./verify.sh` `ALL STAGES OK`, `41 tests run: 41 passed, 0 skipped`, `VERIFY_EXIT=0`. Gates Python (21:29:01Z): SDD `33/0`, DAG `296/0`, task-id `0 duplicatas`, converge `16/16`, hub-docs `0 erros`.
- **[NV]**: CI/merge do PR deste ciclo, confirmação visual final da SE, deploy/publicação e assinatura GPG (DEFER).

## 2026-11-22T16:59Z — Sessão "insurgencia - gate CLI de manifesto" (Blocos 0–5)

- Autoridade do alvo confirmada ao vivo: `./verify.sh` + 5 gates Python; GitHub Actions/`gh`; `dev` exige contexts `gates` + `rust`; `main` desprotegido; factory `b1deab1` (origin/dev `d5d796f`) apenas fonte. Divergência de provider/CLI/gate classificada como governança do alvo — nada importado.
- Drenagem 0.0: 0 PRs abertos, 1 worktree, 0 stashes, 193 paths alheios preservados; ledger `.specify/queue/concurrency.jsonl` criado e runtime ignorado via `.gitignore` (diff explícito). Nenhum trabalho herdável pendente.
- Gate autoritativo (16:54:17Z): `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0` após uma tentativa estourada por `cargo` concorrente (não classificada como RED); 5 gates PASS; CI remota `success` no head exato `7329c53`, run `44952738240`.
- SDD: 33 specs / 236 assessments / 33 validations / 33 releases / 0 bugs / 13 research; nenhuma divergência de fase; âncora `feature.json → specs/033`.
- 80/20: `impacto × confiança × facilidade × segurança / 625`; escolhido gate CLI de manifesto (320/625), à frente de proveniência (200/625) e `avancar_tick` (90/625). Gap de deploy em `main` registrado para escalonamento ao SWE (competência humana).

## 2026-11-22T18:34Z — Fechamento de advisories ACR: correção de deriva documental (ciclo 035)

- **Sessão**: `insurgencia - fechamento de advisories ACR e rastreabilidade` (`session_id=20261122T183153Z-acr-advisories-trace`); 80/20 = advisories ACR + deriva documental, 400/625. Worktree isolado, branch `035-advisories-acr-rastreabilidade`, base `f9d87e0`.
- **Contagens reais (auditoria 18:31–18:34Z)**: 34 specs, 237 assessments, 34 validations, 34 releases, 0 bugs, 13 research. A nota de 16:59Z (33/236/33/33, âncora `specs/033`) era snapshot pré-034 — preservada e superada.
- **Gates (18:32:17Z, observado pela orquestração via `gh`/`git`, não reexecutado)**: `./verify.sh` ALL STAGES OK, 41/41; SDD 34/0; DAG 314/17/0; task-id 0; converge 17/17; hub-docs 0 erros.
- **Âncora**: HEAD `f9d87e0` em `dev`; `feature.json → specs/034-gate-manifesto-cli`. Proteção de `dev` = `contexts=["gates","rust"]`, `strict=true`, `enforce_admins=false` — avisos antigos ("rust não obrigatório", `dev=3709c56`) superados.
- **PR #40** `dev→main`, head `f9d87e0`: `OPEN`, `gates pass` (run `45050452465`), `rust pass` (run `45050458892`); na fila do SWE, **sem merge de `main`** (deploy é competência do SWE). Estado posterior: **[NV]**.
- Nenhum gate/build/format/commit/push/PR executado por esta correção documental; histórico preservado por append.
- **[NV]/DEFER**: confirmação visual da SE, deploy/publicação, assinatura GPG.

## 2026-11-22T19:06Z — Escalonamento ao SWE (ciclo 035, PR #42 mesclado em dev)

- **Contexto**: sessão `insurgencia - fechamento de advisories ACR e rastreabilidade` (`session_id=20261122T183153Z-acr-advisories-trace`). O 80/20 escolhido (400/625) foi fechar 10 advisories de ACR e a deriva documental; o PR #42 foi mesclado em `dev` às 19:05:43Z, merge commit `87c8099`.
- **Evidência**: PR #42 — run `45058283735`, `gates pass` (6s) e `rust pass` (4m37s); ACR sem blockers (comentário `7084068822`); roteiro de 10 passos 10/10 PASS com capturas em `/tmp/tmp.FAhaahvF3D/capturas/` e relatório em `.specify/validations/advisories-acr-rastreabilidade/evidence.md`. No worktree: `./verify.sh` `ALL STAGES OK`, 41/41; gates Python SDD `35/0`, DAG `332 tarefas/34 arestas/0 ciclos`, task-id `0 duplicatas`, converge `18/18`, hub-docs `0 erros`.
- **O que resta ao SWE**: decidir a promoção `dev → main` no PR #40 e, se decidir, disparar o deploy em produção. Nenhum agente mesclou `main`, habilitou auto-merge, alterou proteção ou executou deploy.
- **Riscos, limitações e rollback**: rollback do ciclo 035 por reversão de `87c8099` em `dev`; rollback de eventual promoção por reversão do merge em `main`. **[NV]**: check final do PR #40 no head exato, deploy, efeitos de produção, confirmação visual da SE e assinatura GPG.
- **Issues/PRs**: Refs #39, #40, #41, #011; PR #42 mesclado em `dev`; PR #40 escalado.

## 2026-11-22T21:17Z — Ciclo 036: HUD com acento e fonte livre

- **Sessão/pedido**: `insurgencia - HUD com acento e fonte livre` (`session_id=20261122T211548Z-hud-acento-fonte`). Pedido explícito do SWE: restaurar `Ê`, `—` e `·` no HUD.
- **Âncora/80-20**: ciclo 036 em curso, branch `036-hud-acento-fonte-livre`, base `7c5c0f3`. 80/20 registrado — escolha do usuário tem prioridade; critério interno impacto 5 × confiança 4 × facilidade 3 × segurança 4 ÷ 625 = 240/625.
- **Superação (append-only)**: o paliativo ASCII do ciclo 029 (`INSURGENCIA - low-poly`, `/` no lugar de `·`) é superado por esta mudança; histórico do ciclo 029 intacto.
- **Fonte**: Noto Sans Regular, 621572 bytes, OFL 1.1, cobertura de U+00CA/U+2014/U+00B7 confirmada por `fc-match`; licença e proveniência em `assets/fonts/OFL.txt` e `docs/governance/font-provenance.md` (escritos pelo agente de código).
- **PR #40 mesclado pelo SWE** às 20:52:35Z, merge commit `c126b6e`; `origin/main=c126b6e`. Encerra o [NV] de promoção `dev → main` das sessões anteriores — fato observado, não pendência.
- **Evidência visual**: captura ASCII anterior (sem tofu) em `docs/evidencias/hud-confirmacao-visual-2026-11-22.png`; nova captura com acento a ser executada pelo orquestrador em Xvfb isolado, sem janela no desktop.
- **[NV] remanescentes**: CI/merge do ciclo 036, deploy, assinatura GPG e qualquer efeito de produção. Nota append-only; nenhum gate/build/format/commit/push/PR executado.
