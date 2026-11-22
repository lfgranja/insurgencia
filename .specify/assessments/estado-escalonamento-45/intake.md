# Intake — Estado verificado e escalonamento 45

**Data**: 2026-11-22T22:58Z · **Slug**: `estado-escalonamento-45` · **Sessão**: `session_id=20261122T225813Z-estado-escalonamento`, `session_title=insurgencia - estado verificado e escalonamento 45`.

## Ideia
Registrar, de forma rastreável e sem tocar em código, o **estado verificado** do repositório no fechamento do ciclo 036 e o **escalonamento** do PR #45 (`dev → main`) que está pronto para o SWE.O ciclo é documental: consolida as evidências observadas entre 22:58–23:01Z — HEADs, gates, contagens, PR #44 mesclado, PR #45 green com ACR sem blockers — e registra a decisão de governança que faz prevalecer a constituição v1.0.0 do alvo sobre a v1.2.0 citada pelo prompt v4.6. Nenhuma promoção de `main` é executada.

## 80/20 (decisão de escopo)
| Opção | Impacto × Confiança × Facilidade × Segurança ÷ 625 | Resultado |
|---|---|---|
| Documentação/escalonamento (escolhida) | 3×5×5×5 = 375/625 | **375/625** |
| Paridade `construir_barreira` | 5×3×2×2 = 60/625 | 60/625 |

Escolhe-se a documentação/escalonamento: captura o valor do estado verificado e destrava o SWE sem risco de mexer em código.

## Evidências de entrada (observadas às 22:58–23:01Z)
- Worktree isolado `.specify/queue/worktrees/estado-escalonamento-037/`, branch `037-estado-escalonamento-45`, base/HEAD `efb4d4b`.
- `origin/dev=efb4d4b`, `origin/main=c126b6e`; **0 PRs para dev**; PR **#45** `dev → main` OPEN, green, ACR sem blockers (comentário `7086327685`), na fila SWE — **não mesclar main**.
- PR #44 mesclado em dev às **21:44:38Z** (`efb4d4b`): encerra o ciclo 036 (HUD acentuado, Noto Sans OFL 1.1, blocker P1 `BEVY_ASSET_ROOT` corrigido em `01ff4d0`, captura final sem env, evidência 11/11).
- Gates: `./verify.sh` **ALL STAGES OK 41/41**; 5 gates Python **PASS**; contagens observadas na sessão: **36 specs, 238 assessments, 36 validations, 36 releases, 0 bugs, 13 research**. `find` somente leitura neste worktree confirmou 36 specs (excluindo o stub 037), 36 validations, 36 releases e 0 bugs.
- **Governança**: prompt v4.6 cita constituição v1.2.0 (`tasks` antes de `analyze`, R6 obrigatório), mas o alvo tem constituição **v1.0.0 ratificada** (`analyze` antes de `tasks`, sem R6); o comando `speckit.github.taskstoissues.md` exige `resolve-tasks.sh` ausente. Bloco 0.a: vale a governança do alvo. Divergência registrada como **decisão de governança, não dívida**; R6 **não** adotado neste ciclo.
- Factory `b1deab1` (`origin/dev b3b0c9f4`) apenas fonte — nada a transplantar.

## Restrição de execução
Agente escreve **somente** os 14 artefatos SDD na fronteira exata. Não executa gates/build/formatador/teste, não faz commit/push/PR, não mescla `main`, não edita `converge.md` existente e não toca paths fora da fronteira. Tudo o que não foi executado permanece **[NV]**.
