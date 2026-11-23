# Evidência — Roteiro de Testes 038 (execução autônoma)

**Data/hora**: 2026-11-23T01:35–01:36Z · **Sessão**: `insurgencia - estado verificado pos-v1.2.2` (`session_id=20261123T012952Z-estado-pos-v122`) · **Worktree**: `.specify/queue/worktrees/doc-drift-038/`, branch `038-estado-pos-v122`, base `e010363`. **Superfície**: documental/CLI; nenhuma janela gráfica, build ou gate agregado executado por este roteiro. **Capturas**: `/tmp/tmp.3SqaqPmxWP/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD origin/dev origin/main` | `e010363` / `e010363` / `8b0b044` — coerente com as notas | PASS | `carimbo.txt`, `heads.txt` |
| 2 | PR #48 via `gh` | `OPEN`, base `main`, head `e010363`, `autoMergeRequest=null`, URL https://github.com/lfgranja/insurgencia/pull/48; `gates`/`rust` success | PASS | `pr48.txt` |
| 3 | Gates citados e deploy | `Gates Python PASS; 41/41 testes`; `deploy_count=0` no workflow — citados como observação prévia, não reexecutados aqui | PASS | `gates.txt` |
| 4 | Deriva documental | `spec.md`/`research.md` citam os cinco documentos e a âncora antiga `efb4d4b`/PR #45; `STATE.md` do worktree permanece idêntico ao HEAD (`git diff` vazio) | PASS | `deriva.txt` |
| 5 | Contagens no worktree | 38 specs, 66 assessments, 38 validations, 38 releases, 0 bugs, 10 research — inclui o ciclo 038 em curso e o assessment novo; as contagens versionadas pré-038 (37/65/37/37/0/10) permanecem corretas como snapshot | PASS | `contagens.txt` |
| 6 | Constituição | `Version: 1.2.2`; sequência com `10 tasks → 11 analyze → 12 implement`; R4 antes de R7; R6 fora do fluxo com PARE se invocada | PASS | `constituicao.txt` |
| 7 | 80/20 | `375/625` escolhido vs `180/625` para `coletar_materiais` | PASS | `80-20.txt` |
| 8 | Artefatos e ordem | 5 assessment + 6 specs + 2 validations + 1 release = 14; `analyze.md` READ-ONLY; `tasks` T012 antes de `analyze` T013; R7 `1.0.0` com R4 anterior | PASS | `ordem.txt` |
| 9 | Notas vs HEAD/contagens | `diff_rc=0`; nenhuma divergência entre snapshots | PASS | `diff-notas.txt` |
| 10 | R6 fora do fluxo | apenas menções de suspensão, PARE e proibição de simular issues; nenhuma issue criada; nenhum `taskstoissues` executado | PASS | `r6-check.txt` |

**Resultado**: 10/10 PASS. `evidence_status: passed`. Rollback do roteiro: `rm -rf /tmp/tmp.3SqaqPmxWP`.

**Ressalva**: `STATE.md` não foi versionado nem alterado neste ciclo, conforme decisão do SWE de manter a memória local no checkout compartilhado. `.specify/feature.json` é identidade da feature, não trabalho alheio.
