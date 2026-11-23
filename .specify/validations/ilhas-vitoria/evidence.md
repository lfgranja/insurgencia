# Evidência — Roteiro de Testes 049 (execução autônoma)

**Data/hora**: 2026-11-23T12:25–12:26Z. **Sessão**: `20261123T111200Z-controle-ilha` · `insurgencia - controle de ilha por sudito`. **Worktree**: `.specify/queue/worktrees/ilhas-vitoria-049/`, branch `049-ilhas-vitoria`, base `42b5f7b`. **Capturas**: `/tmp/tmp.aDRjmpbB5q/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `42b5f7b`, branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | Estado `Ilha` | `pub struct Ilha`, `Mundo.ilhas`, `adicionar_ilha` presentes; `Sudito.convertido` é a entrada | PASS | `estado-ilha.txt` |
| 3 | Fase no pipeline | `avancar_tick`: `prepare → skills → defesa → controle_ilhas → finish → reversoes` | PASS | `fase-controle.txt` |
| 4 | 300 ticks | `ilha_controlada_apos_300_ticks`: `exit=0`, 1 passed | PASS | `controle-300.txt` |
| 5 | Perda/sinal | `sem_sudito_ilha_perdida_sinal`: `exit=0`, 1 passed | PASS | `perda-sinal.txt` |
| 6 | Fila de sinais | Sinais `ilha_convertida:<id>` / `ilha_perdida:<id>` em `VecDeque` determinística, incluídos no hash | PASS | `hash-replay.txt` |
| 7 | Hash/replay | `hash_reflete_ilhas_e_sinais` e `duas_ilhas_controle_deterministico`: `exit=0`, 1 passed cada | PASS | `hash-replay.txt` |
| 8 | HUD | `Ilhas:` e `Sinais:` presentes em `atualizar_texto`; renderização real **[NV]** | PASS (inspeção) | `hud-ilhas.txt` |
| 9 | Fronteira/Rust | contratos/protocolo/Cargo `0`; `allow`/`unwrap`/`expect`/rede `0`; execução minimizada sem pânico via gate | PASS | `fronteira-rust.txt` |
| 10 | Governança | v1.2.2, R1/R4/R7, R6 N/A; scorecard **108/625** em 22 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Gate agregado (12:25Z)**: `./verify.sh` **ALL STAGES OK**, **105 testes run: 105 passed** (101 anteriores + 4 novos), `VERIFY_EXIT=0`. Três REDs determinísticos foram corrigidos na fonte antes do verde: `uninlined_format_args` nos sinais, `too_many_lines` em `hash_estado` (extração de `serializar_jogadores`) e divergência documental dos sinais (reconciliada antes do PR).

**Limitações**: presença de Súdito convertido tratada como global (sem mapa/raio); Nós de Controle, Resistência Ideológica e condição de vitória completa ficam para o próximo ciclo; captura visual real permanece **[NV]**.

### Revalidação pós-ACR — 2026-11-23T12:34Z

- Quatro achados advisory do PR #61 (`7101193975`) fechados: HUD/release/roadmap passam a falar em **contagem de sinais pendentes**, assessments registram `ilha_perdida:<id>` também na perda de progresso parcial, `fase_controle_ilhas` não aloca `Vec<u64>` por tick e `Mundo.sinais` está documentado como fila pendente (não histórico).
- `./verify.sh` reexecutado: **ALL STAGES OK**, **105/105 testes**, cinco gates locais PASS. O verde anterior permanece como histórico; a decisão usa o head corrigido.
