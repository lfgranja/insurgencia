# Evidência — Roteiro de Testes 056 (execução autônoma)

**Data/hora**: 2026-11-24T10:40Z → 10:49Z. **Sessão**: `20261124T102734Z-servidor-validacao` · `insurgencia - validacao do servidor headless`. **Worktree**: `.specify/queue/worktrees/servidor-056/`, branch `feature/056-servidor-validacao`, base `0bc2c25`. **Capturas**: `/tmp/tmp-servidor-056-saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `0bc2c25`, branch correto, 16 artefatos, scorecard 1280/3125, DoD | PASS | `roadmap.txt` |
| 2 | API do sim | `ACOES_PERMITIDAS` pública; `acao_exige_destino` com 11/2; `cargo check -p insurgencia-sim --tests` EXIT 0 | PASS | staged checks |
| 3 | Parser/whitelist | 13 ações aceitas; 11 exigem 6 campos; sinalizar_* 4; teste `todas_as_treze...` PASS | PASS | `cargo test -p insurgencia-servidor --test cli` |
| 4 | Validação antes de agendar | recusa `linha 1: ação desconhecida: voar`, exit 2, stdout 0 bytes; replay `linha 2: seq ou request_id em replay`, exit 2 | PASS | `invalid.out/err`, `replay.out/err` |
| 5 | Formato/hash | lote válido imprime `0\t7b134fe2...`; `comandos_validados_preservam_formato_tick_hash_e_determinismo` PASS | PASS | `roadmap.txt` |
| 6 | Hash inicial | `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial` PASS | PASS | CLI test |
| 7 | Fixture e compatibilidade | `coletar_materiais 1 1 0 0`; parser `mover (3.5,-4)` preservado; 10 testes do CLI PASS | PASS | CLI test |
| 8 | Coordenadas/finitude | `acoes_com_destino_exigem_coordenadas_finitas_e_rejeitam_coordenadas_extras` PASS | PASS | CLI test |
| 9 | Fronteira/Rust | contrato/protocolo/Cargo/CI/stack/rede intactos; sem `unsafe`/`unwrap`/`expect`/`#[allow]` | PASS | inspeção |
| 10 | Governança | R1/R4/R7, R6 N/A/bloqueado, promoção/deploy SWE-exclusivos, rollback documentado | PASS | artefatos SDD |

**Resultado local**: **10/10 PASS**. `cargo fmt --all` OK; `cargo check -p insurgencia-sim --tests` EXIT 0; `cargo test -p insurgencia-sim --test simulacao` **97/97**; `cargo check -p insurgencia-servidor --tests` EXIT 0; `cargo test -p insurgencia-servidor --test cli` **10/10**; inventário **146 testes** (141 anteriores + 5 novos). Cinco gates locais PASS; `converge-append-only` dependeu do bootstrap da nova entrada.

**Correção durante a execução**: dois placeholders inválidos em mensagens de falha (`{campos[1]}`) corrigidos por `FixServerFormat056` para argumentos posicionais; nenhum comportamento afrouxado. Duas mensagens históricas GDScript (`concept.md`/`problem.md`) removidas indevidamente pelo agente de assessment foram restauradas byte a byte, preservando o histórico.

**[NV]**: CI remoto `gates`+`rust` no head do PR; merge em `dev`; promoção/deploy; captura visual real. Nota: superfície é CLI/headless, sem browser-use.
