# Evidência — Roteiro de Testes 047 (execução autônoma)

**Data/hora**: 2026-11-23T07:54Z. **Sessão**: `20261123T073500Z-combate-folego` · `insurgencia - combate e folego`. **Worktree**: `.specify/queue/worktrees/combate-047/`, branch `047-combate-folego`, base `0ac4c9f`. **Capturas**: `/tmp/tmp.g1okcZI6FE/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `0ac4c9f`, branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | Custo/dano | `ataque_custa_30_e_causa_10_dano`: `exit=0`, 1 passed; vida com piso 0 | PASS | `dano-custo.txt` |
| 3 | Alcance/recusas | `ataque_fora_de_alcance_recusado`, `folego_insuficiente_recusa`: `exit=0`, 1 passed cada | PASS | `ataque_fora_de_alcance_recusado.txt` |
| 4 | Regen fora de combate | `regenera_10_por_segundo_fora_de_combate`: `exit=0`, 1 passed (+1/3 ticks) | PASS | `regen-fora.txt` |
| 5 | Regen em combate | `regenera_5_por_segundo_em_combate`: `exit=0`, 1 passed (+1/6 ticks) | PASS | `regen-combate.txt` |
| 6 | Janela de combate | `combate_ticks = 90` serializado no fim do tick do ataque (unidade extra consumida em `fase_finish`) | PASS | `dano-custo.txt` |
| 7 | Hash | `hash_reflete_folego_e_vida`: `exit=0`, 1 passed | PASS | `hash.txt` |
| 8 | Tecla Q/HUD | `KeyCode::KeyQ` presente; HUD com `Fôlego`/`Vida` | PASS (inspeção) | `fronteira.txt` |
| 9 | Fronteira | contratos/protocolo/Cargo `0`; `allow`/`unwrap`/`expect`/rede `0`; mudanças restritas a `feature.json`, `STATE.md` e 3 arquivos de código | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2, R1/R4/R7, R6 N/A; scorecard **48/625** em 24 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (07:54Z)**: `./verify.sh` **ALL STAGES OK**, **100 testes run: 100 passed** (93 anteriores + 7 novos), `VERIFY_EXIT=0`. Vários REDs determinísticos foram corrigidos na fonte antes do verde: sintaxe `concat!` no HUD, janela de combate serializada em 90, piso de vida 0 e movimento fora do limite no teste de dois atacantes. `cargo fmt --all` aplicado uma vez.

**Limitações**: todos os valores de combate são `[HIPÓTESE]`/escolha de protótipo conforme GDD §10.1 — nenhum é apresentado como cânone.
