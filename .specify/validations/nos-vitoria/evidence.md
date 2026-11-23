# Evidência — Roteiro de Testes 050 (execução autônoma)

**Data/hora**: 2026-11-23T13:04–13:05Z. **Sessão**: `20261123T125000Z-nos-vitoria` · `insurgencia - nos de controle e vitoria`. **Worktree**: `.specify/queue/worktrees/nos-vitoria-050/`, branch `050-nos-vitoria`, base `9b4f5b4`. **Capturas**: `/tmp/tmp.jhY3z4dsaY/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `9b4f5b4`, branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | Estado dos Nós | `NoControle`, `Mundo.nos`, `resistencia`, `resultado`, `no_alvo` e ações presentes | PASS | `estado-nos.txt` |
| 3 | Fase no pipeline | `prepare → skills → defesa → controle_ilhas → vitoria → finish` | PASS | `fase-vitoria.txt` |
| 4 | 600 ticks | `no_convertido_apos_600_ticks`: `exit=0`, 1 passed | PASS | `propaganda-600.txt` |
| 5 | Dano sem conversão | `dano_nao_converte_no_e_respeita_teto`: `exit=0`, 1 passed; `atacar_no_inativo_recusado`: `exit=0`, 1 passed | PASS | `dano.txt` |
| 6 | Vitória | `tres_nos_geram_vitoria`: `exit=0`, 1 passed | PASS | `vitoria.txt` |
| 7 | Hash/replay | `hash_reflete_nos_e_resistencia`: `exit=0`, 1 passed | PASS | `hash.txt` |
| 8 | HUD/teclas | `Resistência:`, `Nós:`, `propaganda_no`/`R` e `atacar_no`/`F` presentes; renderização real **[NV]** | PASS (inspeção) | `hud-nos.txt` |
| 9 | Fronteira/Rust | contratos/protocolo/Cargo `0`; `allow`/`unwrap`/`expect`/rede `0` | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2, R1/R4/R7, R6 N/A; scorecard **60/625** em 25 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (13:04Z)**: `./verify.sh` **ALL STAGES OK**, **110 testes run: 110 passed** (105 anteriores + 5 novos), `VERIFY_EXIT=0`. Um RED de `clippy::too_many_lines` em `atualizar_texto` foi corrigido via helper puro `texto_hud`, preservando o HUD.

**Limitações**: todos os valores numéricos são `[HIPÓTESE]`/protótipo conforme GDD §6.2/§8.5; derrota por timeout/reconquista e variante duas Comunas ficam fora de escopo; captura visual real permanece **[NV]**.
