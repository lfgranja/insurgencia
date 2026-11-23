# Evidência — Roteiro de Testes 044 (execução autônoma)

**Data/hora**: 2026-11-23T05:55–05:59Z. **Sessão**: `20261123T054700Z-defesa-barricada` · `insurgencia - defesa de barricada`. **Worktree**: `.specify/queue/worktrees/defesa-barricada-044/`, branch `044-defesa-barricada`, base `b6d2f36`. **Capturas**: `/tmp/tmp.CSn9QGXnU2/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `b6d2f36`, branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | 300 ticks/+1 | `defesa_gera_ponto_apos_300_ticks`: `exit=0`, 1 passed | PASS | `caso-300-ticks.txt` |
| 3 | Dono fora do ponto | `dono_fora_do_ponto_nao_pontua`: `exit=0`, 1 passed | PASS | `dono-fora.txt` |
| 4 | Destruição | `barricada_destruida_nao_pontua`: `exit=0`, 1 passed | PASS | `destruicao.txt` |
| 5 | Expiração | `barricada_expira_apos_360_ticks`: `exit=0`, 1 passed | PASS | `expiracao.txt` |
| 6 | Janela única/sem dupla contagem | `terceira_barricada_remove_a_mais_antiga`: `exit=0`, 1 passed | PASS | `janela-unica.txt` |
| 7 | Hash/determinismo | `hash_reflete_autogestao`, `hash_deterministico`, `determinismo_de_duas_simulacoes_identicas`: `exit=0`, 1 passed cada | PASS | `hash-determinismo.txt` |
| 8 | HUD | `Autogestão` presente em `atualizar_texto`; campos anteriores preservados | PASS (inspeção) | `hud.txt` |
| 9 | Fronteira | `contratos=0`, `protocol=0`, `cargo=0`, `allow=0`, `unwrap_expect=0`, `network=0`; mudanças restritas a `feature.json`, `STATE.md` e 3 arquivos de código | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2 com `tasks`→`analyze`, R1/R4/R7, R6 N/A; scorecard **256/625** em 16 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (05:55Z)**: `./verify.sh` **ALL STAGES OK**, **76 testes run: 76 passed** (72 anteriores + 4 novos), `VERIFY_EXIT=0`. `cargo fmt --all` aplicado uma vez antes do gate. Nenhum `#[allow]` introduzido.

**Nota de fase**: a documentação foi alinhada ao comportamento entregue: `fase_defesa` roda entre `fase_skills` e `fase_finish`; a expiração permanece em `fase_finish`.
