# Roteiro de Testes — Estado verificado e escalonamento 45 (ciclo 037)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador/SWE · **Evidências aqui**: **[NV]** (nada executado neste ciclo pelo agente SDD)

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passos 1, 2, 9 — HEADs, PR #45, decisão de não mesclar |
| FR-002 | passos 1, 9 — PR #44 mesclado às 21:44:38Z (`efb4d4b`) |
| FR-003 | passos 3, 4, 9 — gates e contagens, `find` somente leitura |
| FR-004 | passos 5, 6, 10 — constituição v1.0.0, divergência não-dívida, não-adoção de R6 |
| FR-005 | passo 7 — 80/20 375/625 vs. 60/625 |
| FR-006 | passo 8 — ordem normativa do alvo, `analyze` antes de `tasks`, R4 antes de R7 |
| FR-007 | passo 8 — 14 artefatos, fronteira, converge novo |
| FR-008 | passos 1–10 e plano de capturas abaixo |
| FR-009 | passos 9–10 — não-execução, rollback, factory somente fonte |
| SC-001..SC-006 | passos 1–10 |

T-IDs relacionados: T001–T014 (SDD), T015 (converge), T016 (R4), T017 (R7) e T018 (escalonamento/promoção, **[NV]**).

## Pré-condições

1. Worktree `.specify/queue/worktrees/estado-escalonamento-037/`, branch `037-estado-escalonamento-45`, base `efb4d4b`. PR #44 mesclado às 21:44:38Z; PR #45 `dev → main` OPEN na fila SWE — nada a promover neste roteiro.
2. Os 14 artefatos SDD presentes na fronteira exata; `analyze.md` precede `tasks.md`; nenhum `converge.md` existente alterado.
3. `TMP=$(mktemp -d)` para capturas de terminal e arquivos de apoio, fora do worktree; `mkdir -p "$TMP/saidas"`.
4. Nenhum gate, build, commit, push, PR ou merge executado por este agente — os passos abaixo são instruções ao orquestrador/SWE.

## Superfície documental/CLI
- Inspetores: `git rev-parse`/`git status`, `find`, `wc -l`, `grep`, `sha256sum`-like checagem de conteúdo, `diff`, `cat`, `date -u`.
- Nenhuma janela gráfica nem build: tudo é leitura de artefatos e comandos de terminal com saída redirecionada para `$TMP/saidas`.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD origin/dev origin/main` e `date -u > "$TMP/saidas/carimbo.txt"`; comparar com `intake.md`/`research.md`/`converge.md` | HEAD=`efb4d4b`, `origin/dev=efb4d4b`, `origin/main=c126b6e`, conforme registrado com carimbo `22:58–23:01Z` | sim |
| 2 | `grep -nE '#45|não mesclar|fila SWE|7086327685' .specify/assessments/estado-escalonamento-45/{research,decision}.md specs/037-estado-escalonamento-45/spec.md > "$TMP/saidas/pr45.txt"`; conferir estado do PR #45 via API/fonte autorizada **[NV]** | PR #45 OPEN/green/ACR sem blockers, escalonado ao SWE, decisão explícita de **não mesclar `main`** | parcial (inspeção automatizada; estado ao vivo do PR depende do SWE) |
| 3 | `grep -nE '41/41|5 gates|ALL STAGES OK' .specify/assessments/estado-escalonamento-45/research.md > "$TMP/saidas/gates.txt"`; marcar **[NV]** qualquer reexecução | gates observados citados como fatos; nenhum afirmado como executado neste ciclo | sim |
| 4 | `find specs -mindepth 1 -maxdepth 1 -type d ! -name '037-*' | wc -l`; `find .specify/validations -mindepth 1 -maxdepth 1 -type d | wc -l`; `find .specify/releases -type f -name '*.md' | wc -l`; `find .specify -iname '*bug*' | wc -l`; salvar em `$TMP/saidas/contagens.txt`; comparar com `spec.md`/`research.md` | 36 specs (pré-037), 36 validations, 36 releases, 0 bugs; assessments/research com ressalva de snapshot — nenhuma contagem afirmada sem comando | sim |
| 5 | `grep -n 'Version: 1.0.0' .specify/memory/constitution.md`; `grep -nE 'R1|R4|R6' .specify/memory/constitution.md > "$TMP/saidas/constituicao.txt"` | constituição v1.0.0 ratificada; sem R6; `analyze` antes de `tasks`; R4 antes de R7 | sim |
| 6 | `grep -nE 'decisão de governança|não dívida|Bloco 0.a' .specify/assessments/estado-escalonamento-45/{research,decision}.md specs/037-estado-escalonamento-45/spec.md > "$TMP/saidas/governanca.txt"` | divergência registrada como decisão, não dívida; nenhum `taskstoissues`/`resolve-tasks.sh` usado | sim |
| 7 | `grep -nE '375/625|60/625' .specify/assessments/estado-escalonamento-45/{intake,concept,decision}.md > "$TMP/saidas/80-20.txt"` | 80/20 documentado: 375/625 escolhido sobre 60/625 | sim |
| 8 | Listar os 14 artefatos; `grep -n 'READ-ONLY' specs/037-*/analyze.md`; `grep -n '1.0.0' .specify/releases/estado-escalonamento-45/release.md`; `git status --short`; conferir que nenhum `converge.md` alheio aparece como modificado | 14 artefatos na fronteira; `analyze` (READ-ONLY) antes de `tasks`; R4 antes de R7 `1.0.0`; `converge.md` novo; nada fora da fronteira tocado | parcial (inspeção automatizada + revisão humana) |
| 9 | **Comparar notas vs HEAD/contagens ao vivo**: repetir passos 1 e 4 após qualquer atualização de `dev`; `diff "$TMP/saidas/contagens.txt" <(novo)`; registrar divergência | notas e HEAD/contagens coerentes, ou divergência registrada com data e fonte — nunca reescrita sem rastro (Princípio VI) | sim |
| 10 | **Confirmar que R6 não foi adotado sem ratificação**: `grep -ri 'R6' specs/037-estado-escalonamento-45 .specify/{assessments,validations,releases}/estado-escalonamento-45` (só menções de não-adoção); checar que `tasks.md` vem depois de `analyze.md` na DAG e que nenhuma ratificação de v1.2.0 existe | nenhum uso de R6 como regra ativa; ordem do alvo preservada; v1.2.0 não ratificada; rollback documentado em `release.md` permanece não executado | sim |

## Plano de capturas (terminal — sem execução de build/gate)
- `$TMP/saidas/carimbo.txt`, `pr45.txt`, `gates.txt`, `contagens.txt`, `constituicao.txt`, `governanca.txt`, `80-20.txt` — saídas dos passos 1–7.
- `$TMP/saidas/ordem.txt` e `$TMP/saidas/diff-notas.txt` — saídas dos passos 8–9; `$TMP/saidas/r6-check.txt` — saída do passo 10.
- Toda captura é rotulada **[NV]** até o orquestrador executar o passo correspondente; nenhuma captura existe neste ciclo.

## Rollback do roteiro
`rm -rf "$TMP"` remove capturas e arquivos de apoio; encerrar qualquer inspeção sem persistir nada fora do worktree. Se algum passo falhar, registrar a saída obtida e corrigir a **fonte** (nota/artefato), nunca a expectativa do teste. Não executar `git clean`, `git reset`, force-push, remoção de artefatos do ciclo, merge do PR #45 nem reexecução de gates por este agente. Para reverter o ciclo, seguir `release.md` (rollback documentado, nunca executado neste ciclo).
