# Evidência — Roteiro de Testes 045 (execução autônoma)

**Data/hora**: 2026-11-23T06:33–06:34Z. **Sessão**: `20261123T062500Z-ponte-provisoria` · `insurgencia - ponte provisoria deterministica`. **Worktree**: `.specify/queue/worktrees/ponte-provisoria-045/`, branch `045-ponte-provisoria`, base `fa33052`. **Capturas**: `/tmp/tmp.goCurCLQlD/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `fa33052`, branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | 45 ticks/+15→0 | `ponte_apos_45_ticks_debita_15`: `exit=0`, 1 passed | PASS | `caso-45-ticks.txt` |
| 3 | Saldo 14 recusado | `saldo_14_nao_construi_ponte`: `exit=0`, 1 passed | PASS | `saldo-14.txt` |
| 4 | 60 PV | `ponte_60_pv_destruida`: `exit=0`, 1 passed | PASS | `pv-60.txt` |
| 5 | 1350 ticks/expiração | `ponte_expira_apos_1350_ticks`: `exit=0`, 1 passed | PASS | `expiracao.txt` |
| 6 | Comando único | `ponte_apos_um_unico_comando`: `exit=0`, 1 passed | PASS | `comando-unico.txt` |
| 7 | Hash/determinismo | `hash_reflete_pontes`, `hash_deterministico`, `determinismo_de_duas_simulacoes_identicas`: `exit=0`, 1 passed cada | PASS | `hash-determinismo.txt` |
| 8 | Tecla P/HUD | `KeyCode::KeyP` presente; HUD com `Pontes` | PASS (inspeção) | `fronteira.txt` |
| 9 | Fronteira | contratos/protocolo/Cargo `0`; `allow`/`unwrap`/`expect`/rede `0`; mudanças restritas a `feature.json`, `STATE.md` e 3 arquivos de código | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2, R1/R4/R7, R6 N/A; scorecard **108/625** em 18 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (06:33Z)**: `./verify.sh` **ALL STAGES OK**, **86 testes run: 86 passed** (77 anteriores + 9 novos), `VERIFY_EXIT=0`. Um RED determinístico de `clippy::similar_names` (`ponte` vs `ponto`) foi corrigido na fonte antes do verde. `cargo fmt --all` aplicado uma vez.

**Limitação**: atalho de 8 s entre ilhas e tempo de construção canônico não foram implementados — o GDD não os fixa como valores de Núcleo; 45 ticks é escolha de protótipo registrada na spec.
