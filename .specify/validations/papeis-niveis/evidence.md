# Evidência — Roteiro de Testes 052 (execução autônoma)

**Data/hora**: 2026-11-23T15:17–15:18Z. **Sessão**: `20261123T150800Z-papeis-niveis` · `insurgencia - papeis e niveis`. **Worktree**: `.specify/queue/worktrees/papeis-052/`, branch `052-papeis-niveis`, base `4f3966e`. **Capturas**: `/tmp/tmp.bN4cG4g2OD/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `4f3966e`, branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | Estado/constantes | `Funcao`, `nivel`, `nivel_ticks`, `TICKS_POR_NIVEL=900`, `NIVEL_MAXIMO=12` presentes | PASS | `estado-papeis.txt` |
| 3 | Progressão | `avancar_niveis()` chamado em `fase_finish` | PASS | `fase-nivel.txt` |
| 4 | 900 ticks | `nivel_sobe_apos_900_ticks`: `exit=0`, 1 passed | PASS | `nivel-900.txt` |
| 5 | Teto 12 | `nivel_teto_12`: `exit=0`, 1 passed | PASS | `teto-12.txt` |
| 6 | Bloqueio | `acao_bloqueada_por_nivel`: `exit=0`, 1 passed | PASS | `bloqueio.txt` |
| 7 | Desbloqueios | `desbloqueio_q_w_e_r`: `exit=0`, 1 passed | PASS | `desbloqueio.txt` |
| 8 | Hash/HUD | `hash_reflete_funcao_e_nivel`: `exit=0`, 1 passed; `Função:`/`Nível:` no HUD; renderização real **[NV]** | PASS | `hash.txt`, `hud-papeis.txt` |
| 9 | Fronteira/Rust | contratos/protocolo/Cargo `0`; `allow`/`unwrap`/`expect`/rede `0` | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2, R1/R4/R7, R6 N/A; scorecard **48/625** em 34 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (15:17Z)**: `./verify.sh` **ALL STAGES OK**, **122 testes run: 122 passed** (117 anteriores + 5 novos), `VERIFY_EXIT=0`. Um RED de `clippy::match_same_arms` foi corrigido na fonte antes do verde.

**Limitações**: papéis/níveis/Q1-W2-E3-R6 são `[HIPÓTESE]`/protótipo; novas habilidades W/E/R e níveis acima do desbloqueio de ações existentes ficam para ciclos posteriores; captura visual real permanece **[NV]**.
