# Evidência — Roteiro de Testes 053 (execução autônoma, pré-ACR)

**Data/hora**: 2026-11-23T16:01–16:11Z. **Sessão**: `20261123T155300Z-pnj-defensores` · `insurgencia - pnjs defensores`. **Worktree**: `.specify/queue/worktrees/pnj-053/`, branch `053-pnj-defensores`, base `4975fd4`. **Capturas**: `/tmp/tmp.6SlMPonkhP/saidas/`. **Escopo**: registro histórico **pré-ACR**; não cobre a correção do PR #65 (distância de `atacar_pnj`, cota por atacante e escolha do menor `peer_id`).

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `4975fd4`, branch correto, **14 artefatos** na fronteira; scorecard 48/625 (4×2×2×3) | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | Estado/constantes | `pub struct Pnj`, `Mundo.pnjs`, `adicionar_pnj`, `aplicar_dano_a_pnj`, `atacar_pnj`, `fase_ataque_pnjs` presentes em `crates/sim/src/lib.rs` | PASS | `estado-pnj.txt` |
| 3 | Dano e letalidade | `dano_mata_pnj`: `exit=0`, 1 passed; transição 24 → 14 → 0 com piso zero e `ativo = false` | PASS | `dano.txt` |
| 4 | Ação `atacar_pnj` | `atacar_pnj_respeita_alcance`: `exit=0`, 1 passed; token local ao sim, sem contrato/protocolo/Cargo/habilidade nova | PASS | `atacar_pnj_respeita_alcance.txt`, `fronteira.txt` |
| 5 | Ataque do PNJ | `pnj_ataca_jogador_em_alcance`: `exit=0`, 1 passed; 8 dmg a cada 90 ticks em 2 m | PASS | `ataque-pnj.txt` |
| 6 | Hash | `hash_reflete_pnjs`: `exit=0`, 1 passed; segmento de PNJs em ordem estável | PASS | `hash.txt` |
| 7 | HUD | `PNJs:` presente em `texto_hud`; `texto_progresso` extraído para manter `texto_hud` sob o limite de clippy; captura visual real **[NV]** | PASS | `hud-pnj.txt` |
| 8 | Tecla `T` | `KeyCode::KeyT` com `just_pressed` presente, seq/request_id incrementados só com alvo | PASS | `hud-pnj.txt` |
| 9 | Fronteira/Rust | contratos/protocolo/Cargo `0`; `allow`/`unwrap`/`expect`/rede `0` nos dois arquivos | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2, R1/R4/R7, R6 N/A; scorecard **48/625** em **35 arquivos**; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado (pré-ACR)**: **10/10 PASS**. `evidence_status: passed` **no estado anterior à correção ACR**. Estado pós-ACR: três testes novos da sim e reexecução dos gates pelo orquestrador pendentes; nenhum resultado, verde ou contagem pós-ACR é afirmado (`evidence_status` pós-ACR: **[NV]/pending**).

## Gate agregado (16:11Z, pré-ACR)

`./verify.sh` **ALL STAGES OK** — check, clippy, `128 testes run: 128 passed, 0 skipped`, fmt; `VERIFY_EXIT=0`. Cinco gates locais: SDD completeness `PASS (spec_dirs=52 gaps=0 baselined=0 strict_from=88)`, work-order DAG `PASS (files=52 tasks=564 edges=249 cycles=0)`, task-ID uniqueness `PASS (tasks_files=52 duplicates=0 baselined=0)`, converge append-only `FAIL` antes do bootstrap do baseline da nova `converge.md` (corrigido pelo orquestrador após a captura), hub docs `PASS (0 errors, 0 warnings)`.

## Correções na fonte durante a execução

1. `16:02Z` — `clippy::too_many_lines` em `texto_hud` (103 linhas): extraído `texto_progresso(&Simulacao) -> (String, String)` puro; HUD inalterado.
2. `16:09Z` — `saturating_sub` satura em `i64::MIN`, não em zero; `14i64.saturating_sub(24) = -10`. Corrigido para `.saturating_sub(dano).max(0)` em `aplicar_dano_a_pnj`, com transição em `pv == 0`.
3. `16:11Z` — asserção invertida em `dano_mata_pnj`: a transição para inativo retorna `true` por contrato; corrigida para `assert!(...)`.

Nenhuma expectativa foi afrouxada para mascarar defeito; os três RED foram corrigidos na implementação/asserção correta e o verde final da execução **pré-ACR** usa o head corrigido daquele momento. Após a correção ACR o head mudou: o verde deve ser revalidado pelo orquestrador, sem reaproveitar esta captura como prova do estado corrigido.

## Limitações

PNJs defensores, 24 PV, dano 10 do jogador, 8 dmg/90 ticks em 2 m, `atacar_pnj` nível 1 com `destino` na posição do PNJ e distância `<= ALCANCE_CONTATO_M` (2 m), cota por atacante/tick (não por PNJ), contra-ataque ao menor `peer_id` em alcance (não o mais próximo), e tecla `T` são `[HIPÓTESE]`/protótipo. Sem mapa, patrulha, rede, lobby, 5v5, habilidades W/E/R novas ou janela normal. Captura visual real **[NV]**; CI remoto e merge **[NV]** até execução pelo orquestrador.

## Correção ACR pós-PR #65 — reexecução pendente **[NV]**

- Contrato corrigido: `atacar_pnj` exige distância euclidiana `<= ALCANCE_CONTATO_M` (2 m) entre jogador e PNJ, com `destino` ainda na posição do PNJ; cota de um ataque por atacante por tick (não por PNJ); contra-ataque escolhe o menor `peer_id` entre jogadores em alcance (não o mais próximo).
- Três testes novos da sim, a executar pelo orquestrador: (1) aceitação em 2 m exatos e recusa além de 2 m em `atacar_pnj`; (2) no máximo um ataque por atacante no mesmo tick, sem cota por PNJ; (3) em dois jogadores em alcance, apenas o de menor `peer_id` sofre os 8 de dano no tick de ataque.
- Nenhuma captura, resultado ou verde pós-ACR foi produzido por este agente. O orquestrador deve reexecutar `cargo test -p sim`, `./verify.sh`, os gates e o roteiro de 10 passos sobre o estado corrigido, registrando a saída em `$TMP/saidas/acr-correcao.txt`. R6 permanece **N/A**, o scorecard segue **48/625** e a versão `1.0.0-local` não é promovida.

## Correção ACR — reexecução pelo orquestrador (16:29–16:30Z)

- Três testes novos: `atacar_pnj_a_1_5m_sem_colocalizacao`, `cota_de_ataque_pnj_e_por_atacante` e `pnj_ataca_menor_peer_id_em_alcance` — **3/3 PASS** (`exit=0`, 1 passed cada).
- `./verify.sh` **ALL STAGES OK**, **131 testes run: 131 passed, 0 skipped** (128 pré-ACR + 3 novos), `VERIFY_EXIT=0` (`16:29Z`).
- Cinco gates locais PASS (`16:27Z`): SDD completeness, work-order DAG, task-ID uniqueness, converge append-only (`files=35 baselined=35`) e hub docs.
- Um RED de `clippy::assertions_on_constants` surgiu nas asserções de sanity dos testes novos; corrigido removendo as duas asserções constantes, sem afrouxar nenhum comportamento observável.
- Captura: `/tmp/tmp.6SlMPonkhP/saidas/acr-correcao.txt`. R6 permanece **N/A**; versão `1.0.0-local` não promovida; CI remoto/merge **[NV]** até verificação ao vivo.
