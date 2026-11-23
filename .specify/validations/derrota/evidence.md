# Evidência — Roteiro de Testes 051 (execução autônoma)

**Data/hora**: 2026-11-23T14:38–14:39Z. **Sessão**: `20261123T142500Z-derrota` · `insurgencia - derrota por timeout e reconquista`. **Worktree**: `.specify/queue/worktrees/derrota-051/`, branch `051-derrota`, base `3132f47`. **Capturas**: `/tmp/tmp.UFgDHR1Oef/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `3132f47`, branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | Estado/constantes | `TICKS_LIMITE_PARTIDA=36000`, `MAX_RECONQUISTAS=3`, variantes de derrota e `ilhas_reconquistadas` presentes | PASS | `estado-derrota.txt` |
| 3 | Fase no pipeline | `fase_derrota()` após `tick += 1`, preservando precedência da vitória | PASS | `fase-derrota.txt` |
| 4 | Timeout | `derrota_por_timeout`: `exit=0`, 1 passed; fronteira 35999→36000 | PASS | `timeout.txt` |
| 5 | Reconquistas | `derrota_apos_tres_reconquistas` e `reconquista_duplicada_nao_conta`: `exit=0`, 1 passed cada | PASS | `reconquistas.txt` |
| 6 | Precedência | `vitoria_precede_derrota`: `exit=0`, 1 passed | PASS | `precedencia.txt` |
| 7 | Hash | `hash_reflete_derrota`: `exit=0`, 1 passed | PASS | `hash.txt` |
| 8 | HUD | "Comuna dissolvida" presente para ambas as variantes; renderização real **[NV]** | PASS (inspeção) | `hud-derrota.txt` |
| 9 | Fronteira/Rust | contratos/protocolo/Cargo `0`; `allow`/`unwrap`/`expect`/rede `0` | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2, R1/R4/R7, R6 N/A; scorecard **192/625** em 23 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (14:38Z)**: `./verify.sh` **ALL STAGES OK**, **117 testes run: 117 passed** (112 anteriores + 5 novos), `VERIFY_EXIT=0`. Quatro REDs determinísticos foram corrigidos na fonte antes do verde: `too_many_lines`/`redundant_closure`/`uninlined_format_args` no hash e off-by-one da fase de derrota (`tick` avançava depois da avaliação).

**Limitações**: valores de timeout/reconquista são `[HIPÓTESE]`/protótipo; captura visual real permanece **[NV]**.

### Revalidação pós-ACR — 2026-11-23T14:50Z

- Quatro achados advisory do PR #63 (`7104447713`) fechados: HUD descreve `Resultado: Comuna dissolvida — <motivo>`, sufixo de reconquistas derivado de `MAX_RECONQUISTAS`, variantes/tokens documentados e pipeline com `fase_derrota` após `tick += 1`. Um RED de `too_many_lines` no HUD foi corrigido ao extrair `texto_resultado`.
- `./verify.sh` reexecutado: **ALL STAGES OK**, **117 testes run: 117 passed**, cinco gates locais PASS. A decisão usa o head corrigido.
