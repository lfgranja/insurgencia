# Evidência — Roteiro de Testes 041 (execução autônoma)

**Data/hora**: 2026-11-23T04:02–04:04Z. **Sessão**: `20261123T035400Z-coleta-jogavel` · `insurgencia - coleta jogavel no cliente`. **Worktree**: `.specify/queue/worktrees/coleta-cliente-041/`, branch `041-coleta-jogavel`, base `bf05cfa`. **Capturas**: `/tmp/tmp.zrt6yU5vYp/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `bf05cfa`, branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | 180 ticks | `coleta_apos_180_ticks`: `exit=0`, 1 passed; auto-avanço 1/tick | PASS | `caso-180-ticks.txt` |
| 3 | +3/+2 atômico | Mesmo teste cobre crédito no tick 180 e zeração de ticks/ponto | PASS | `caso-credito.txt` |
| 4 | Saída antes | `saida_antes_zera`: `exit=0`, 1 passed | PASS | `saida-antes.txt` |
| 5 | Dupla coleta no mesmo tick | `dupla_coleta_no_mesmo_tick_nao_acelera_credito`: `exit=0`, 1 passed | PASS | `dupla-coleta.txt` |
| 6 | Comando único | `coleta_continua_apos_um_unico_comando`: `exit=0`, 1 passed | PASS | `comando-unico.txt` |
| 7 | Hash/determinismo | `hash_deterministico`, `determinismo_de_duas_simulacoes_identicas`, `hash_saldos_divergentes`, `determinismo_coleta`: `exit=0`, 1 passed cada | PASS | `hash-determinismo.txt` |
| 8 | Tecla C/HUD/doc comment | `KeyCode::KeyC` presente; HUD com `Suprimentos`/`Coleta`; doc comment de `destino` cita `mover`, `coletar_materiais` e `construir_barreira` | PASS (inspeção; tecla real **[NV]**) | `cliente-c-hud.txt` |
| 9 | Fronteira | `contratos_diff=0`, `protocol_diff=0`, `cargo_diff=0`, `allow=0`, `unwrap_expect=0`; mudanças restritas a `feature.json`, `STATE.md` e 3 arquivos de código | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2 com `tasks`→`analyze`, R1/R4/R7, R6 N/A; scorecard **500/625** em 12 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (04:02Z)**: `./verify.sh` **ALL STAGES OK**, **60 testes run: 60 passed** (59 anteriores + 1 novo), `VERIFY_EXIT=0`. `cargo fmt --all` aplicado uma vez antes do gate. Nenhum `#[allow]` introduzido.

**Cliente**: tecla `C` agenda coleta com `seq`/`request_id` monotônicos; o HUD mostra Suprimentos e progresso de Coleta. Confirmação por compilação, testes e inspeção de código; acionamento visual da tecla não executado neste roteiro.
