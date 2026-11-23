# Intake — Estado verificado pós-v1.2.2

**Data**: 2026-11-23T01:29Z · **Slug**: `estado-pos-v122` · **Sessão**: `session_id=20261123T012952Z-estado-pos-v122`, `session_title=insurgencia - estado verificado pos-v1.2.2`.

## Ideia
Registrar, de forma rastreável e sem tocar em código, o **estado verificado** do repositório no pós-ratificação da constituição **v1.2.2** (observado às 01:29–01:31Z) e a **deriva documental** remanescente: `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` e `STATE.md` ainda ancoram snapshots antigos (`efb4d4b`/PR #45) em trechos finais, enquanto o estado corrente é `e010363`/`8b0b044`/PR #48 e as contagens versionadas corretas são 37 specs, 65 assessments, 37 validations, 37 releases, 0 bugs, 10 research. Ciclo **documental**: consolida âncoras, gates, PR, contagens e governança; registra a suspensão de R6 por decisão ratificada; não promove `main`, não edita os documentos derivados (fora da fronteira) e não simula issues.

## 80/20 (decisão de escopo)
| Opção | Impacto × Confiança × Facilidade × Segurança ÷ 625 | Resultado |
|---|---|---|
| Documentação pós-v1.2.2 (escolhida) | 3×5×5×5 = 375/625 | **375/625** |
| Coletar materiais | 5×4×3×3 = 180/625 | 180/625 |

Escolhe-se a documentação pós-v1.2.2: captura o estado verificado e a deriva com confiança, facilidade e segurança máximas, sem risco de código. "Coletar materiais" tem impacto maior (5), mas confiança/facilidade/segurança menores (4/3/3) e foge da fronteira.

## Evidências de entrada (observadas às 01:29–01:31Z; não reexecutadas)
- Worktree isolado `.specify/queue/worktrees/doc-drift-038/`, branch `038-estado-pos-v122`, base/HEAD `e010363`. HEAD/`dev`=`e010363`, `main`=`8b0b044`.
- **0 PRs para `dev`**; PR **#48** `dev → main` OPEN, green, ACR sem blockers (comentário `7089052846`), na fila SWE — **não mesclar `main`**: promoção/deploy pertence ao SWE.
- **Constituição v1.2.2 ratificada** (`.specify/memory/constitution.md`, ratificada 2026-11-19, última emenda 2026-11-23): merge autônomo restrito a `dev`; promoção/deploy ao SWE; sequência executável `00 → 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09 → revisão → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release`, com `analyze` READ-ONLY **depois** de `tasks`. **R6 ratificada como intenção, mas fora da sequência executável** enquanto `resolve-tasks.sh`/adaptador de issues não existem; se invocada, **PARE e escale ao SWE**, nunca simular issue.
- Gates Python **PASS**; **41/41** testes observados; **deploy ausente no workflow** (dívida já escalada, aberta); factory `b1deab1` (`origin/dev 05b977f`) apenas fonte; **1 worktree, 0 stashes, 24 untracked preservados**.
- Deriva documental: `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` e `STATE.md` ancoram `efb4d4b`/PR #45 em trechos finais. Contagens versionadas corretas: **37 specs, 65 assessments, 37 validations, 37 releases, 0 bugs, 10 research**. O checkout compartilhado mostra **238 assessments / 13 research** por incluir 173 diretórios históricos untracked e 3 notas untracked (`2026-11-20-*`), preservados e não versionados.

## Restrição de execução
Agente escreve **somente** os 14 artefatos SDD na fronteira exata. Não executa gates/build/formatador/teste, não faz commit/push/PR, não mescla `main`, não edita os cinco documentos com deriva nem qualquer `converge.md` existente, não usa `taskstoissues`/`resolve-tasks.sh`, não simula issues e não toca paths fora da fronteira (incluindo as alterações preexistentes em `.specify/feature.json` e `docs/PROGRESSO.md`). Tudo o que não foi executado permanece **[NV]**.
