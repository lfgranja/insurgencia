# Evidência — Roteiro de Testes 039 (execução autônoma)

**Data/hora**: 2026-11-23T02:04–02:06Z. **Sessão**: `insurgencia - coleta deterministica de materiais` (`session_id=20261123T015356Z-coleta-materiais`). **Worktree**: `.specify/queue/worktrees/coleta-materiais-039/`, branch `039-coleta-materiais-deterministica`, base `990ab06`. **Capturas**: `/tmp/tmp.ADNoP5fSgj/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD` + inventário | `990ab06`; branch correta; 14 artefatos; `STATE_diff=0` | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | `coleta_apos_180_ticks` | `exit=0`, 1 passed | PASS | `caso-180.txt`, `coleta_apos_180_ticks.txt` |
| 3 | `saida_antes_zera` | `exit=0`, 1 passed | PASS | `saida-antes.txt`, `saida_antes_zera.txt` |
| 4 | `recusa_sem_destino_e_fora_do_ponto` | `exit=0`, 1 passed | PASS | `recusa-destino.txt`, `recusa_sem_destino_e_fora_do_ponto.txt` |
| 5 | `hash_saldos_divergentes` | `exit=0`, 1 passed | PASS | `hash-divergente.txt`, `hash_saldos_divergentes.txt` |
| 6 | `ausencia_consumo_parcial` | `exit=0`, 1 passed | PASS | `sem-consumo.txt`, `ausencia_consumo_parcial.txt` |
| 7 | `determinismo_coleta` | `exit=0`, 1 passed | PASS | `determinismo.txt`, `determinismo_coleta.txt` |
| 8 | Fronteira/protocolo | `contratos_diff=0`, `cargo_diff=0`, `protocol_diff=0`; alterações apenas `.specify/feature.json` e dois arquivos do `sim` | PASS | `fronteira.txt` |
| 9 | Governança/80/20 | Constituição v1.2.2 com `10 tasks → 11 analyze`, R1/R4, R6 fora do fluxo; `180/625` documentado | PASS | `governanca.txt` |
| 10 | Gates e rollback | `./verify.sh` ALL STAGES OK, 47/47 testes; SDD `39/0`, DAG `406/104/0`, task-id `0`, hub-docs `0`; converge pendente só do bootstrap; rollback documentado e não executado | PASS | `gates.txt` |

**Resultado**: 10/10 PASS. `evidence_status: passed`. Rollback do roteiro: `rm -rf /tmp/tmp.ADNoP5fSgj`.

**Correções reais**: dois REDs determinísticos de clippy (`explicit_counter_loop`, `range_plus_one`) corrigidos em `crates/sim/tests/simulacao.rs` sem `#[allow]`; `cargo fmt --all` aplicado uma vez. LSP diagnostics indisponível (`rust-analyzer` ausente); gate Rust do CI é a autoridade.
