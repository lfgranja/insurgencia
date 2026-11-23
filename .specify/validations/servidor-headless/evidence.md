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

**Servidor**: binário `insurgencia-servidor` em `crates/servidor`, membro da workspace, sem sockets/async/rede. A execução é determinística por construção: stdin completa validada antes do primeiro tick, comandos agendados em ordem e hash emitido no início de cada tick, antes de `avancar_tick()` (linha `0` = estado inicial).

**Correção ACR #55 (comentário `7091811530`)**: o snapshot de estado é feito no início de cada tick, antes de `avancar_tick()`. As capturas acima registram a execução original; a linha `0` passa a conter o hash do estado inicial.

### Reexecução final — 2026-11-23T04:45Z

- Roteiro reexecutado após a correção do snapshot: CLI aceita `1`/`3` e rejeita `0`/`-1`/`abc`/ausência com exit 2; 6+5 testes do crate PASS; stdin com dois comandos gera 2 ticks; `--ticks 3` emite exatamente 3 linhas com ticks `0,1,2`; `cmp_exit=0`; 5 casos malformados rejeitados com exit 2; nenhuma dependência/`allow`/`unwrap`/`expect`/async/socket; 5 gates locais PASS. Capturas: `/tmp/tmp.9zdb7J9z25/saidas/`.
- **Linha 0 corrigida**: `c4d3c5cf874bd14bd4da189756830e80cb6000ac8b6e3f53dcfa93d599ba8747` = `Mundo::new().hash_estado()` (estado inicial), seguida por `95746704…` e `162de773…` nos ticks 1 e 2. O teste de integração `primeira_linha_com_stdin_vazia_e_o_estado_inicial` cobre esse contrato.
- **Gate final (04:44:39Z)**: `verify.sh` **ALL STAGES OK**, **72 testes run: 72 passed** (71 anteriores + 1 novo teste de integração). O verde anterior de 71 testes foi descartado; a decisão usa o head corrigido.
