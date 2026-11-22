# Evidência — Roteiro de Testes 037 (execução autônoma)

**Data/hora**: 2026-11-22T23:05–23:07Z · **Worktree**: `.specify/queue/worktrees/estado-escalonamento-037/`, branch `037-estado-escalonamento-45`, base `efb4d4b`. **Superfície**: documental/CLI; nenhuma janela gráfica, build ou gate agregado executado por este roteiro. **Capturas**: `/tmp/tmp.zVdxQqmNhY/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD origin/dev origin/main` | `efb4d4b` / `efb4d4b` / `c126b6e` — coerente com as notas | PASS | `carimbo.txt`, `heads.txt` |
| 2 | PR #45 via `gh pr view`/`gh pr checks` | `OPEN`, base `main`, head `efb4d4b`, `autoMergeRequest=null`, URL https://github.com/lfgranja/insurgencia/pull/45; `gates`/`rust` success | PASS | `pr45.txt` |
| 3 | Gates citados no assessment | `ALL STAGES OK`, 41/41, 5 gates Python — citados como observação prévia, não reexecutados aqui | PASS | `gates.txt` |
| 4 | `find`/`wc` de contagens | pré-037: 36 specs; 37 validations; 37 releases; 0 bugs; 65 assessments; 10 research. Correção das notas 23:05Z aplicada | PASS | `contagens.txt` |
| 5 | Constituição | `Version: 1.0.0` (o grep literal com `Version:` não casou porque o arquivo usa `**Version:**`); `R6_in_constitution=0`; ordem `analyze` antes de `tasks` confirmada por leitura direta | PASS (com ressalva de sintaxe do grep) | `constituicao.txt` |
| 6 | Divergência de governança | `decisão de governança`, `não dívida`, Bloco 0.a presentes em research/decision/spec; nenhum `taskstoissues` executado | PASS | `governanca.txt` |
| 7 | 80/20 | `375/625` escolhido vs `60/625` alternativa | PASS | `80-20.txt` |
| 8 | Artefatos e ordem | 5 assessment + 6 specs + 2 validations + 1 release = 14; `analyze.md` READ-ONLY antes de `tasks`; R7 `1.0.0` com R4 anterior; `converge.md` novo | PASS | `ordem.txt` |
| 9 | Notas vs HEAD/contagens | `diff_rc=0` entre snapshots; notas corrigidas com comandos e carimbos | PASS | `diff-notas.txt` |
| 10 | Não-adoção de R6 | `R6` aparece apenas em menções de não-adoção/risco; constituição v1.0.0 sem R6; nenhuma ratificação de v1.2.0 | PASS | `r6-check.txt` |

**Resultado**: 10/10 PASS. `evidence_status: passed`. Rollback do roteiro: `rm -rf /tmp/tmp.zVdxQqmNhY`; nenhum arquivo do repositório modificado pela execução do roteiro.

**Ressalva**: as contagens 238 assessments / 13 research do checkout compartilhado incluem 173 diretórios untracked históricos e 3 notas `2026-11-20-*` preservadas; a árvore versionada tem 65/10. As notas do ciclo registram ambas com fonte e carimbo.
