# Evidência — Roteiro de Testes 035 (execução autônoma)

**Data/hora**: 2026-11-22T18:59:00Z → 18:59:03Z · **Sessão**: `insurgencia - fechamento de advisories ACR e rastreabilidade` (`session_id=20261122T183153Z-acr-advisories-trace`) · **Worktree**: `.specify/queue/worktrees/acr-trace-035/`, branch `035-advisories-acr-rastreabilidade`, base `f9d87e0`. **Superfície**: CLI + documentos; nenhuma UI de produto utilizável nesta sessão. **Capturas**: `/tmp/tmp.FAhaahvF3D/capturas/` (fora do git).

**Pré-condição**: `cargo build -q -p insurgencia-sim --example verificar_manifesto` → exit 0. Hash canônico das regras temporárias: `89335a931ed716f97d3a88827b42ad8ea2b0ae08eea58f13a07b1fc79f3fc0b0`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Comparar nova seção de `STATE.md` com `find`/`feature.json` | "34 specs, 237 assessments, 34 validations, 34 releases, 0 bugs, 13 research"; âncora `specs/034` rotulada como base do ciclo; nota 33/236/33/33 preservada e explicitamente **superada** como snapshot pré-034, conforme append-only | PASS | saída de terminal colada acima |
| 2 | Ler `evidence.md` do ciclo 034 | "cinco grupos de captura do passo 1"; `17/17` com baseline aplicado; `quatro execu=0`, `pendente apenas do bootstrap=0` | PASS | `grep` colado acima |
| 3 | `.gitignore` e convergência 034 | `.gitignore:6` contém `/.specify/queue/`; `specs/034/converge.md:26` registra a correção append-only da afirmação histórica | PASS | `grep` colado acima |
| 4 | Manifesto com `content_hash` em **maiúsculas** | `COMPAT_OK module_id=insurgencia.exemplo content_hash=89335a…0b0` — **minúsculo calculado**, não as maiúsculas da entrada; stderr vazia, exit **0** | PASS | `maiusculo-*` |
| 5 | Manifesto com hash minúsculo | mesma saída canônica do passo 4, exit **0** — sem regressão | PASS | `minusculo-*` |
| 6 | Divergências controladas | `VERSAO_PROTOCOLO_DIVERGENTE`, `MODULO_AUSENTE`, `REGRAS_AUSENTES`, `HASH_MODULO_DIVERGENTE`, `MANIFEST_INVALID` — cada uma exit **1**, stdout vazio, ordem do contrato preservada | PASS | `versao-*`, `modulo-*`, `regrasausentes-*`, `hash-*`, `vazio-*` |
| 7 | 3 execuções do caso maiúsculo + `diff` | três stdout idênticos, exits `0/0/0`; `diff12_rc=0`, `diff13_rc=0` | PASS | `aceito-1|2|3-*`, `diff-12.txt`, `diff-13.txt` |
| 8 | IDs de ACR e PR #40 | comentários `7082836571`/`7082907820`/`7082916777` rastreados em `research.md`/`converge.md`; PR #40 `OPEN`, base `main`, head `f9d87e0` — nenhum merge de `main` | PASS | saída de `gh` colada acima |
| 9 | 14 artefatos e ordem normativa | assess=5, specs=6, validate=2, release=1; `analyze.md` existe e vem antes de `tasks.md`; R7 `1.0.0` com R4 anterior | PASS | saída de `find`/`grep` colada acima |
| 10 | Inspeção estática e fronteira | único casamento proibido é `#![forbid(unsafe_code)]`; `forbid=1`, `deny(clippy::pedantic)=1`; 0 linhas >100; diff restrito aos paths do ciclo; nenhum commit/push/PR executado | PASS | saída de `grep`/`awk`/`git status` colada acima |

**Resultado**: 10/10 PASS. `evidence_status: passed`. Nenhum passo exigiu intervenção humana. Rollback do roteiro: `rm -rf /tmp/tmp.FAhaahvF3D`.

**Gates da unidade**: `./verify.sh` (18:55:09Z) → `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`; SDD `35/0`, DAG `332 tarefas/34 arestas/0 ciclos`, task-id `0 duplicatas`, hub-docs `0 erros`. `converge-append-only` aguardava apenas o bootstrap do novo `converge.md`, aplicado nesta fase. Uma tentativa anterior de `verify.sh` (18:39:50Z) estourou 900 s por contenção de `cargo` — **não** classificada como RED. `lsp diagnostics` falhou por ausência de `rust-analyzer` no ambiente; clippy do `verify.sh` é a autoridade determinística do alvo.
