# Evidência — Roteiro de Testes 046 (execução autônoma)

**Data/hora**: 2026-11-23T07:08–07:09Z. **Sessão**: `20261123T065800Z-suditos` · `insurgencia - suditos e propaganda`. **Worktree**: `.specify/queue/worktrees/suditos-046/`, branch `046-suditos`, base `d410bd6`. **Capturas**: `/tmp/tmp.7oMqvFWSKs/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `d410bd6`, branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | 120 ticks de exposição | `propaganda_converte_apos_120_ticks`: `exit=0`, 1 passed | PASS | `exposicao-120.txt` |
| 3 | Dano → 90 ticks de reversão | `dano_reverte_em_90_ticks`: `exit=0`, 1 passed | PASS | `reversao-90.txt` |
| 4 | Injeção determinística | API `adicionar_sudito` sem spawn/hipóteses; testes individualmente verdes | PASS | `propaganda_apos_um_unico_comando.txt` |
| 5 | Auto-avanço/`BTreeSet` | `propaganda_apos_um_unico_comando` e `dupla_propaganda_no_mesmo_tick_nao_acelera`: `exit=0`, 1 passed cada | PASS | `propaganda_apos_um_unico_comando.txt` |
| 6 | Atacar/panfletar | Estado `convertido` verificado nos testes de conversão/reversão | PASS | `exposicao-120.txt` |
| 7 | Hash/determinismo | `hash_reflete_suditos`, `hash_deterministico`, `determinismo_de_duas_simulacoes_identicas`: `exit=0`, 1 passed cada | PASS | `hash-determinismo.txt` |
| 8 | Tecla G/HUD | `KeyCode::KeyG` presente; HUD com `Súditos` | PASS (inspeção) | `fronteira.txt` |
| 9 | Fronteira | contratos/protocolo/Cargo `0`; `allow`/`unwrap`/`expect`/rede `0`; mudanças restritas a `feature.json`, `STATE.md` e 3 arquivos de código | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2, R1/R4/R7, R6 N/A; scorecard **48/625** em 32 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (07:08Z)**: `./verify.sh` **ALL STAGES OK**, **92 testes run: 92 passed** (86 anteriores + 6 novos), `VERIFY_EXIT=0`. `cargo fmt --all` aplicado uma vez antes do gate. Nenhum `#[allow]` introduzido.

**Limitações**: spawn por pulso, raio de 6 m e funções trabalhador/propagandista/miliciano permanecem `[HIPÓTESE]` e não foram implementados; súditos são injetados deterministicamente pela API de simulação.
