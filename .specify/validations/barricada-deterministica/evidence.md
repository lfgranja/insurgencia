# Evidência — Roteiro de Testes 040 (execução autônoma)

**Data/hora**: 2026-11-23T03:29–03:30Z. **Sessão**: `20261123T024300Z-barricada-deterministica` · `insurgencia - barricada deterministica de materiais`. **Worktree**: `.specify/queue/worktrees/barricada-040/`, branch `040-barricada-deterministica`, base `6b913f0`. **Capturas**: `/tmp/tmp.rGFcIo20wu/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `6b913f0`, branch correto, **14 artefatos**, `STATE_diff=1` (nota de decisão da sessão; ledger intacto) | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | 45 ticks → +20→0 | `barricada_apos_45_ticks_debita_20`: `exit=0`, 1 passed | PASS | `caso-45-ticks.txt` |
| 3 | Saldo 19 e destino inválido | `saldo_insuficiente_recusa_sem_debito` + `construcao_fora_do_ponto_recusada`: `exit=0`, 1 passed cada; sem débito/canalização/barreira | PASS | `saldo-insuficiente.txt` |
| 4 | 200 PV | `barricada_200_pv_destruida_ao_200_dano`: `exit=0`, 1 passed; remoção só em `pv=0` | PASS | `pv-200.txt` |
| 5 | 360 ticks | `barricada_expira_apos_360_ticks`: `exit=0`, 1 passed; 1 no tick 359, removida no 360 | PASS | `duracao-360.txt` |
| 6 | 3.ª barricada | `terceira_barricada_remove_a_mais_antiga`: `exit=0`, 1 passed; máx. 2 por dono | PASS | `tres-barricadas.txt` |
| 7 | Hash/determinismo | `hash_reflete_barreiras_e_construcao` + `determinismo_com_barreiras`: `exit=0`, 1 passed cada; SHA-256 reflete campos novos e ordem `BTreeMap` | PASS | `hash-determinismo.txt` |
| 8 | Dupla construção/mover | `dupla_construcao_no_mesmo_tick_nao_acelera` + `movimento_cancela_construcao`: `exit=0`, 1 passed cada | PASS | `dupla-construcao.txt` |
| 9 | Fronteira e cliente | `contratos_diff=0`, `protocol_diff=0`, `cargo_diff=0`, `allow=0`, `unwrap_expect=0`, `client_B=1`, `client_tick=1`, `hud=1`; alterações restritas a `feature.json`, `STATE.md` e 3 arquivos de código | PASS (inspeção; acionamento visual da tecla **[NV]**) | `fronteira.txt` |
| 10 | Governança/R6/rollback | v1.2.2 com `10 tasks → 11 analyze`, R1/R4/R7, R6 fora do fluxo; scorecard **400/625** em 4 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (03:29:39Z)**: `./verify.sh` **ALL STAGES OK**, **58 testes run: 58 passed** (48 anteriores + 10 novos), `VERIFY_EXIT=0`. Correções determinísticas aplicadas antes do verde: compile `format!(concat!(...))`, expiração off-by-one, guarda de saldo no recebimento do comando, `too_many_lines` via helpers privados e `cargo fmt --all`. Nenhum `#[allow]` introduzido.

**Cliente**: o cliente agora consome `comandos_pendentes` com `avancar_simulacao` a 30 Hz (teto 4 ticks/frame), de modo que a tecla `B` e o HUD deixam de ser inertes. Confirmação por inspeção de código e compilação; captura visual da tecla **não executada** neste roteiro.

### Revalidação após ACR — 2026-11-23T03:39Z

- **Blocker P1 corrigido** (comentário `7090866575`): a tecla `B` agenda um único comando, mas a canalização exigia comando por tick. `fase_skills` agora tem pré-passo `avancar_construcoes_automaticas` que avança 1 tick por jogador enquanto o jogador permanece no ponto com saldo ≥ 20; o comando no mesmo tick não conta duas vezes. Novo teste `construcao_continua_apos_um_unico_comando` cobre 45 ticks sem novos comandos.
- **P3 corrigido**: `aplicar_dano_a_barreira` com `dano <= 0` é no-op, sem elevar PV acima de 200.
- **P2 documental corrigido**: tabela de rastreabilidade de `test-roadmap.md` realinhada com FR-001..FR-006 da spec/checklist; governança ficou fora dos FR.
- **Gate reexecutado (03:38:31Z)**: `verify.sh` `ALL STAGES OK`, **59 testes run: 59 passed** (58 anteriores + 1 novo). Roteiro reexecutado (03:39Z): 11 testes nomeados, todos `exit=0`; 10/10 passos mantidos. Cinco gates locais PASS. O verde anterior de 58 testes foi descartado; a decisão usa o head corrigido.
- **[NV] mantido**: acionamento visual da tecla `B` em janela real; verificação por compilação, testes e inspeção.
