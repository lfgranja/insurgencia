# Evidência — Roteiro de Testes 042 (execução autônoma)

**Data/hora**: 2026-11-23T04:37–04:38Z. **Sessão**: `20261123T043000Z-servidor-headless` · `insurgencia - servidor headless deterministico`. **Worktree**: `.specify/queue/worktrees/servidor-headless-042/`, branch `042-servidor-headless`, base `b099093`. **Capturas**: `/tmp/tmp.4vL6foNQAo/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `b099093`, branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | CLI `--ticks` | `--ticks 1` → 1 linha, `--ticks 3` → 3 linhas; `0`, `-1`, `abc` e ausência → `exit=2`, 0 linhas, mensagem determinística | PASS | `cli-ticks.txt` |
| 3 | Parser | 6 testes unitários + 4 de integração, todos `ok` (`exit=0`) | PASS | `parser.txt` |
| 4 | Agendamento antes do tick 0 | Duas linhas (`mover`, `coletar_materiais`) geraram 2 ticks com hashes distintos e contínuos | PASS | `agendamento.txt` |
| 5 | Contagem de ticks | `--ticks 3` → exatamente 3 linhas, ticks `0,1,2` | PASS | `ticks.txt` |
| 6 | Hash por tick | Formato `<tick>\t<hash>`; hashes SHA-256 de 64 hex (`95746704…`, `162de773…`, `673dcc23…`) | PASS | `hash.txt` |
| 7 | Determinismo | Duas execuções idênticas: `cmp_exit=0` | PASS | `determinismo.txt` |
| 8 | Rejeição | 5 casos malformados (token faltando/extra, ação desconhecida, `peer_id` inválido, `NaN`) → `exit=2`, 0 linhas, erro determinístico, sem pânico | PASS | `rejeicao.txt` |
| 9 | Fronteira | nova dependência externa `0`; `async`/socket/`Lightyear`/`tokio` `0`; `allow`/`unwrap`/`expect` `0`; `forbid(unsafe_code)` e `deny(clippy::pedantic)` presentes. O `grep unsafe=1` é a própria diretiva `forbid`, não código unsafe | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2 com `tasks`→`analyze`, R1/R4/R7, R6 N/A; scorecard **256/625** em 19 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (04:37Z)**: `./verify.sh` **ALL STAGES OK**, **71 testes run: 71 passed** (61 anteriores + 6 unitários + 4 de integração do servidor), `VERIFY_EXIT=0`. `Cargo.lock` foi gerado pelo Cargo via `cargo check --offline` (sem rede); nenhuma dependência externa acrescentada.

**Servidor**: binário `insurgencia-servidor` em `crates/servidor`, membro da workspace, sem sockets/async/rede. A execução é determinística por construção: stdin completa validada antes do primeiro tick, comandos agendados em ordem e hash emitido após cada `avancar_tick`.
