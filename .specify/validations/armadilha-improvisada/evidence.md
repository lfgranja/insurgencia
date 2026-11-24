# Evidência — Roteiro de Testes 057 (execução autônoma)

**Data/hora**: 2026-11-24T12:13Z → 12:55Z. **Sessão**: `20261124T121311Z-armadilha-improvisada` · `insurgencia - armadilha improvisada`. **Worktree**: `.specify/queue/worktrees/armadilha-057/`, branch `feature/057-armadilha-improvisada`, base `661890f`. **Capturas**: `/tmp/tmp-armadilha-057-saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `661890f`, 14 artefatos, scorecard 720/3125, DoD | PASS | roadmap.txt |
| 2 | Constantes/estrutura | 9 constantes públicas, `Armadilha`, `Mundo.armadilhas`, `Jogador.equipe/recarga/imobilizado` | PASS | inspeção |
| 3 | Ação/validação | `armadilha` nível 3, destino ≤3 m, 25 Fôlego, 5 Materiais, recarga 420 | PASS | teste CLI |
| 4 | Recursos/cota | débito 25+5, máx 3 por jogador, `SaldoInsuficiente`/`ArmadilhaEmRecarga` | PASS | testes sim |
| 5 | Arme/duração | 30 ticks para armar, 1800 ticks de duração, expiração determinística | PASS | testes sim |
| 6 | Imobilização | inimigo (`equipe` diferente) a ≤1 m fica 60 ticks imobilizado; aliado imune | PASS | testes sim |
| 7 | `mover` bloqueado | enquanto imobilizado, comando não altera posição nem monotônicos | PASS | testes sim |
| 8 | Hash | `hash_reflete_armadilha_e_imobilizacao`: PASS; campos e armadilhas serializados | PASS | testes sim |
| 9 | Cliente/HUD | `E` despacha armadilha para Artesão, análise para demais; HUD `Armadilhas: N`; `texto_hud` <100 linhas; `cargo check -p insurgencia-cliente` EXIT 0 | PASS | inspeção + check |
| 10 | Governança | R1/R4/R7, R6 N/A/bloqueado, promoção/deploy SWE-exclusivos | PASS | artefatos SDD |

**Resultado local**: **10/10 PASS** nos planos documental/CLI. `cargo fmt --all` OK; `cargo check -p insurgencia-sim --tests` EXIT 0; `cargo test -p insurgencia-sim --test simulacao` **102/102**; inventário **151**; `cargo check -p insurgencia-cliente` EXIT 0. `cargo test -p insurgencia-cliente --lib` excedeu o teto por contenção de CPU — **[NV]**, não RED. Sonda CLI: `1 armadilha 1 1 0 0` é aceita sintaticamente e recusada por nível (`linha 1: acao bloqueada por nivel`), conforme DoD.

Correções durante a execução:

- `serializar_estruturas` perdeu args nomeados do `format!(concat!(...))` ao inserir `{armadilhas}`;
- teste perdeu import de `TICKS_DURACAO_BARREIRA`;
- helper do cliente recebia `&Simulacao` mas chamadas passavam `&Mundo`.

Todas corrigidas na fonte; nenhuma expectativa afrouxada.

## Correções ACR aplicadas na fonte (pós-revisão)

A tabela acima registra a execução original (12:13–12:55Z). Após a revisão ACR, três correções foram aplicadas na fonte por agentes de implementação; **nenhum** build, gate, teste, formatador ou captura foi executado por este agente de documentação (resultados permanecem **[NV]** até o orquestrador):

| Correção ACR | Mudança na fonte | Evidência observável por inspeção |
|:--|:--|:--|
| Hash helpers | `serializar_estruturas` (`crates/sim/src/lib.rs`) decomposto em `serializar_barreiras`, `serializar_pontes`, `serializar_armadilhas`, `serializar_suditos`, `serializar_pnjs`, `serializar_ilhas`, `serializar_ilhas_reconquistadas` e `serializar_sinais`, todos puros (`#[must_use]`, `///`) | Segmentos e ordem preservados: jogadores por `peer_id`, armadilhas por `id`; `hash_estado`/`serializar_jogadores` seguem cobrindo os campos e cada armadilha |
| Contagens CLI | `crates/servidor/tests/cli.rs` renomeou o teste para `todas_as_quatorze_acoes_do_sim_sao_aceitas_pela_whitelist_do_cli` e atualizou `ACOES_PERMITIDAS.len()` de 13 para **14** e o total de ações com destino de 11 para **12** | Coerente com a inclusão do token `armadilha` desde o ciclo 056; whitelist permanece derivada de `ACOES_PERMITIDAS`, sem expectativa afrouxada |
| HUD/Y | `crates/cliente/src/main.rs`: `solicitar_analise_materialista` despacha com `KeyCode::KeyY` (E contextual: Artesão → armadilha, demais → análise; `Y` força a análise em qualquer papel, inclusive no simulador); `texto_hud` delega a coleta a `coletar_dados_hud`/`DadosHud` (puro, `#[must_use]`) e ficou com **49 linhas** | `Armadilhas: N` preservada, `DICA_TECLAS` registra `E` contextual e `Y`, `texto_hud` com 49 linhas (< 100) |

Escopo, scorecard **720/3125**, rótulos `[HIPÓTESE]`, R6 **N/A/bloqueado**, o resultado 10/10 PASS acima e a lista de correções originais permanecem inalterados.

**[NV]**: CI remoto, merge em `dev`, promoção/deploy, captura visual da janela Bevy. A superfície é CLI/headless; nenhuma janela foi aberta na área de trabalho do usuário.
