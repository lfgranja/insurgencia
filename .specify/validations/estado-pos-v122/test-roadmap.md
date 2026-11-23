# Roteiro de Testes — Estado verificado pós-v1.2.2 (ciclo 038)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador/SWE · **Evidências aqui**: **[NV]** (nada executado neste ciclo pelo agente SDD)

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passos 1, 2, 9 — HEADs, PR #48, decisão de não mesclar |
| FR-002 | passos 3, 9 — 41/41, gates Python, deploy ausente, inventário |
| FR-003 | passos 4, 5, 9 — deriva dos cinco documentos e contagens versionadas vs. checkout |
| FR-004 | passos 6, 8 — constituição v1.2.2, ordem `tasks` antes de `analyze`, R4 antes de R7 |
| FR-005 | passos 8, 10 — R6 suspensa por decisão ratificada, PARE se invocada, sem issues simuladas |
| FR-006 | passo 7 — 80/20 375/625 vs. 180/625 |
| FR-007 | passo 8 — 14 artefatos, fronteira, converge novo, alterações preexistentes intactas |
| FR-008 | passos 1–10 e plano de capturas abaixo |
| FR-009 | passos 9–10 — não-execução, rollback, factory somente fonte |
| SC-001..SC-006 | passos 1–10 |

T-IDs relacionados: T001–T015 (SDD), T016 (converge), T017 (R4), T018 (R7) e T019 (escalonamento/promoção, **[NV]**).

## Pré-condições

1. Worktree `.specify/queue/worktrees/doc-drift-038/`, branch `038-estado-pos-v122`, base `e010363`; HEAD/`dev` `e010363`, `main` `8b0b044`. PR #48 `dev → main` OPEN na fila SWE — nada a promover neste roteiro.
2. Os 14 artefatos SDD presentes na fronteira exata; `tasks.md` (T012) antes de `analyze.md` (T013); nenhum `converge.md` existente alterado; cinco documentos com deriva e alterações preexistentes intactos.
3. `TMP=$(mktemp -d)` para capturas de terminal e arquivos de apoio, fora do worktree; `mkdir -p "$TMP/saidas"`.
4. Nenhum gate, build, commit, push, PR, merge ou deploy executado por este agente — os passos abaixo são instruções ao orquestrador/SWE.

## Superfície documental/CLI
- Inspetores: `git rev-parse`/`git status`, `find`, `wc -l`, `grep`, `diff`, `cat`, `date -u`, leitura via API/fonte autorizada para o estado do PR.
- Nenhuma janela gráfica nem build: tudo é leitura de artefatos e comandos de terminal com saída redirecionada para `$TMP/saidas`.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD origin/dev origin/main` e `date -u > "$TMP/saidas/carimbo.txt"`; comparar com `intake.md`/`research.md`/`converge.md` | HEAD/`dev`=`e010363`, `main`=`8b0b044`, conforme registrado com carimbo `01:29–01:31Z` | sim |
| 2 | **Conferir PR #48 green**: estado via API/fonte autorizada e `grep -nE '#48|não mesclar|fila SWE|7089052846'` nos artefatos > `"$TMP/saidas/pr48.txt"` | PR #48 OPEN/green/ACR sem blockers (comentário `7089052846`), escalonado ao SWE, decisão explícita de **não mesclar `main`** | parcial (inspeção automatizada; estado ao vivo depende do SWE) |
| 3 | `grep -nE '41/41|gates Python|ALL STAGES' .specify/assessments/estado-pos-v122/research.md > "$TMP/saidas/gates.txt"`; marcar **[NV]** qualquer reexecução e a dívida de deploy ausente no workflow | gates observados citados como fatos; nenhum afirmado como executado neste ciclo; deploy ausente permanece dívida aberta | sim |
| 4 | `grep -nE 'OBJETIVO|ROADMAP|research/index|PROGRESSO|STATE|efb4d4b' specs/038-estado-pos-v122/spec.md .specify/assessments/estado-pos-v122/research.md > "$TMP/saidas/deriva.txt"`; confirmar que os cinco arquivos não aparecem como modificados em `git status --short` | deriva registrada com âncora antiga `efb4d4b`/PR #45, **sem edição** dos cinco documentos | parcial (inspeção automatizada + revisão humana de `git status`) |
| 5 | `find specs -mindepth 1 -maxdepth 1 -type d -name '[0-9][0-9][0-9]-*' \| wc -l`; mesmos padrões para assessments/validations/releases; `find docs/research -maxdepth 1 -type f -name '*.md' \| wc -l`; salvar em `$TMP/saidas/contagens.txt` | 37 specs, 65 assessments, 37 validations, 37 releases, 0 bugs, 10 research (versionadas); 238/13 do checkout compartilhado explicado por untracked históricos | sim |
| 6 | `grep -n 'Version: 1.2.2' .specify/memory/constitution.md`; `grep -nE '10 tasks|11 analyze|R1|R4|R6' .specify/memory/constitution.md > "$TMP/saidas/constituicao.txt"` | constituição v1.2.2 ratificada; `analyze` depois de `tasks`; R4 antes de R7; R6 fora da sequência executável | sim |
| 7 | `grep -nE '375/625|180/625' .specify/assessments/estado-pos-v122/{intake,concept,decision}.md > "$TMP/saidas/80-20.txt"` | 80/20 documentado: 375/625 escolhido sobre 180/625 | sim |
| 8 | Listar os 14 artefatos; `grep -n 'READ-ONLY' specs/038-estado-pos-v122/analyze.md`; conferir a DAG em `tasks.md` (T012 antes de T013); `grep -n '1.0.0' .specify/releases/estado-pos-v122/release.md`; `git status --short` | 14 artefatos na fronteira; `tasks` antes de `analyze` (READ-ONLY); R4 antes de R7 `1.0.0`; `converge.md` novo; nada fora da fronteira tocado, incluindo `feature.json`/`PROGRESSO.md` | parcial (inspeção automatizada + revisão humana) |
| 9 | **Comparar notas vs HEAD/contagens ao vivo**: repetir passos 1 e 5 após qualquer atualização de `dev`; `diff "$TMP/saidas/contagens.txt" <(novo)`; registrar divergência | notas e HEAD/contagens coerentes, ou divergência registrada com data e fonte — nunca reescrita sem rastro (Princípio VI) | sim |
| 10 | **Confirmar que R6 está fora do fluxo**: `grep -riE 'R6|taskstoissues|resolve-tasks|simular issue|PARE' specs/038-estado-pos-v122 .specify/{assessments,validations,releases}/estado-pos-v122 > "$TMP/saidas/r6-check.txt"`; checar que nenhuma issue foi criada e que a DAG não contém R6 | R6 suspensa por decisão ratificada; PARE se invocada; nenhuma issue simulada, nenhum `gh` improvisado; emenda ratificada é pré-requisito para inclusão | sim |

## Plano de capturas (terminal — sem execução de build/gate)
- `$TMP/saidas/carimbo.txt`, `pr48.txt`, `gates.txt`, `deriva.txt`, `contagens.txt`, `constituicao.txt`, `80-20.txt` — saídas dos passos 1–7.
- `$TMP/saidas/ordem.txt` e `$TMP/saidas/diff-notas.txt` — saídas dos passos 8–9; `$TMP/saidas/r6-check.txt` — saída do passo 10.
- Toda captura é rotulada **[NV]** até o orquestrador executar o passo correspondente; nenhuma captura existe neste ciclo.

## Rollback do roteiro
`rm -rf "$TMP"` remove capturas e arquivos de apoio; encerrar qualquer inspeção sem persistir nada fora do worktree. Se algum passo falhar, registrar a saída obtida e corrigir a **fonte** (nota/artefato), nunca a expectativa do teste. Não executar `git clean`, `git reset`, force-push, remoção de artefatos do ciclo, merge do PR #48, deploy nem reexecução de gates por este agente. Para reverter o ciclo, seguir `release.md` (rollback documentado, nunca executado neste ciclo).
