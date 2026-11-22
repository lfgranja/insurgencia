# Evidência — Roteiro de Testes 034 (execução autônoma)

**Data/hora**: 2026-11-22T17:21:57Z → 17:21:58Z · **Session title**: `insurgencia - gate CLI de manifesto` · **Ambiente**: worktree `.specify/queue/worktrees/manifesto-cli/`, branch `034-gate-manifesto-cli`, head `7329c53`. **Superfície**: CLI headless; sem UI de produto, sem `browser-use`. **Capturas**: `/tmp/tmp.t9J9ly2h8z/capturas/` (fora do git, removidas no rollback do roteiro).

**Pré-condição**: `cargo build -q -p insurgencia-sim --example verificar_manifesto` → exit 0. `BIN=target/debug/examples/verificar_manifesto`. Hash das regras temporárias: `8b6c19070d629bce5ed01620c0a66d41c0fd5ae8be94cc4d15981856bbbc1f1f` (`sha256sum`).

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | `$BIN` sem args / 1 arg / 5 args / `versao_local=abc` | cinco grupos de captura do passo 1, exit **2**, stderr de uso ou `versao_local deve ser um inteiro entre 0 e 255`, nenhuma linha `COMPAT_*` | PASS | `uso0-*`, `uso1-*`, `uso1b-*`, `uso5-*`, `usoabc-*` |
| 2 | `$BIN` com `{}` e com `nao-e-json`, regras válidas | `COMPAT_FALHA MANIFEST_INVALID` na stderr, exit **1**, stdout vazio (ambos) | PASS | `vazio-*`, `ruido-*` |
| 3 | manifesto aceito gerado com hash real do arquivo de regras | `COMPAT_OK module_id=insurgencia.exemplo content_hash=8b6c…1f1f` em stdout, stderr vazia, exit **0** | PASS | `aceito-*`, `manifesto-aceito.json` |
| 4 | `protocol_version=2` | `COMPAT_FALHA VERSAO_PROTOCOLO_DIVERGENTE`, exit **1** | PASS | `versao-*` |
| 5 | `module_id=outro.modulo` | `COMPAT_FALHA MODULO_AUSENTE`, exit **1** | PASS | `modulo-*` |
| 6 | arquivo de regras inexistente | `COMPAT_FALHA REGRAS_AUSENTES`, exit **1** | PASS | `regrasausentes-*` |
| 7 | `content_hash` = 64 zeros | `COMPAT_FALHA HASH_MODULO_DIVERGENTE`, exit **1** | PASS | `hash-*` |
| 8 | caso aceito repetido 3× | três stdout idênticos, exits `0/0/0`; `diff -u` 1↔2 e 1↔3 com rc **0** e arquivos vazios | PASS | `aceito-1-*`, `aceito-2-*`, `aceito-3-*`, `diff-12.txt`, `diff-13.txt` |
| 9 | args explícitos; manifesto inexistente | explícitos: igual ao passo 3, exit **0**; inexistente: `erro ao ler manifesto '…': No such file or directory (os error 2)`, exit **2** | PASS | `explicito-*`, `manifestoausente-*` |
| 10 | inspeção estática do novo `.rs` | único casamento de `unwrap\|expect\|unsafe\|async` é a própria linha `#![forbid(unsafe_code)]`; `forbid=1`, `deny(clippy::pedantic)=1`; 0 linhas >100 colunas; `git diff --name-only` = `.specify/feature.json` (identidade da feature) — nenhum `Cargo.toml`, biblioteca ou `manifesto.json` alterado | PASS | saída de terminal colada acima; legibilidade revisada nesta sessão |

**Resultado**: 10/10 PASS. `evidence_status: passed`. Nenhum passo exigiu intervenção humana; nenhuma captura de UI (superfície inexistente). Rollback do roteiro: `rm -rf /tmp/tmp.t9J9ly2h8z` — nenhum arquivo do repositório modificado pela execução.

**Gates da unidade**: `./verify.sh` (17:12:43Z) → `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`; um RED real anterior (`unused import OsString`) foi corrigido na fonte e registrado no converge. Gates Python (17:06:29Z): SDD `34/0`, DAG `314 tarefas/17 arestas/0 ciclos`, task-id `0 duplicatas`, hub-docs `0 erros`; baseline aplicado e gate `converge-append-only` PASSOU com `17/17` às 21:23:25Z de 22/11.

**Nota (ciclo 035)**: o caso de `content_hash` em maiúsculo passa a ser coberto pelo roteiro do ciclo 035; a linha de sucesso do CLI imprime agora o SHA-256 calculado em minúsculo (`hash_das_regras`), canônico e byte a byte idêntico para manifestos com a mesma hash em caixas diferentes.
