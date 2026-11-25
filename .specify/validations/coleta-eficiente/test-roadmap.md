# Roteiro de Testes — Coleta Eficiente do Camponês (ciclo 062)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador/SWE · **Evidências aqui**: **[NV]** (nada executado neste ciclo pelo agente SDD) · **Data**: 2026-11-25.
**Branch**: `feature/062-coleta-eficiente` · **Worktree**: `.specify/queue/worktrees/cycle-062-coleta-eficiente/` · **Âncora**: `origin/dev = d3be560` (HEAD da feature, pai do commit a ser revertido).
Todos os números do protótipo — 15 Fôlego, buff 150 ticks, 90 vs 180 ticks, recarga 300 ticks e tecla `K` — são **[HIPÓTESE]** lidas em `GDD:646`; nenhuma é regra canônica.

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passo 1 — token `coleta_eficiente`, whitelist com 16 tokens, 14 com destino, CLI de 6 tokens |
| FR-002 | passos 2, 4 — destino = posição própria, Fôlego ≥ 15, recarga 0, sem *stacking*, recusa tipada sem efeito |
| FR-003 | passos 3, 5, 6 — 90 vs 180 ticks com recompensa igual, decrescentes exatos, reativação, hash determinístico |
| FR-004 | passos 2, 5, 7 — consumidor pelo mesmo caminho validado, inspeção de `KeyCode::KeyK`/`texto_hud` e checagem do cliente (`cargo check --all-targets`); nenhuma janela UI |
| SC-001..SC-004 | passos 1–8 |
| Governança (fora dos FR) | passos 7, 8 — constituição v1.2.2, R4 antes de R7 `1.0.0-local`, **R6 N/A**, rollback por `git revert` |

T-IDs relacionados: T001–T010 (`specs/062-coleta-eficiente/tasks.md`). A implementação em `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs`, `crates/servidor/tests/cli.rs` e `crates/cliente/src/main.rs` é handoff ao orquestrador/SWE e permanece **[NV]** até execução.

## Pré-condições

1. Worktree `.specify/queue/worktrees/cycle-062-coleta-eficiente/`, branch `feature/062-coleta-eficiente`, base/âncora `origin/dev = d3be560`; identidade em `.specify/feature.json` (`specs/062-coleta-eficiente`), nunca pelo nome do branch.
2. Feature implementada no head pelos T003–T009: whitelist de 16 tokens, `acao_exige_destino` true para `coleta_eficiente`, constantes `[HIPÓTESE]`, campos de buff/recarga cobertos por `serializar_jogadores`/`hash_estado`, ramo em `fase_skills`, decrescentes em `avancar_tick`, tecla `K` e linha em `texto_hud`. Nenhum parser ou whitelist paralela na CLI.
3. `TMP=$(mktemp -d)` fora do worktree para capturas de terminal e JSON; `mkdir -p "$TMP/saidas"`. Nenhum build, gate, teste, formatador, commit, push, PR, merge ou deploy executado por este agente — os passos abaixo são instruções ao orquestrador/SWE.
4. Superfície restrita a terminal/CLI: sem janela Bevy, sem browser, sem credenciais e sem rede; o cliente é verificado apenas por inspeção de código e `cargo check --locked -p insurgencia-cliente --all-targets`.

## Superfície CLI/terminal (sem UI)

- Inspeção: `grep`, `sed`, `cat`, `find`, `diff`, `date -u`, `git status --short`, leitura das fontes e dos artefatos SDD.
- Comandos de prova: `cargo test --offline --locked -p insurgencia-sim --test simulacao`, `cargo test --offline --locked -p insurgencia-servidor --test cli`, `cargo check --locked -p insurgencia-cliente --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`, cinco scripts em `scripts/ci/` e `./verify.sh`.
- Capturas: saídas redirecionadas para `$TMP/saidas/*.txt` e resumos estruturados em `$TMP/saidas/*.json` (contagens de tokens/testes, carimbos, estados dos jobs). Toda captura permanece **[NV]** até o orquestrador anexar a saída com origem e carimbo.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | automatizável? |
|:--|:--|:--|:--|
| 1 | **Whitelist 16/14**: inspecionar `ACOES_PERMITIDAS`/`acao_exige_destino` em `crates/sim/src/lib.rs` e rodar `printf '1 coleta_eficiente 1 1 0.0 0.0\n' \| cargo run --locked -p insurgencia-servidor -- --ticks 1`; salvar `$TMP/saidas/whitelist.txt` e `whitelist.json` (total, com destino, sem destino) | `ACOES_PERMITIDAS.len() == 16` com `coleta_eficiente` presente; `acao_exige_destino == true` para **14** tokens (inclusive o novo) e `false` apenas para `sinalizar_sincronia`/`sinalizar_alavanca`; a CLI aceita a linha de 6 tokens e preserva `<tick>\t<sha256>`, sem `AcaoDesconhecida`, parser local ou whitelist paralela | sim |
| 2 | **Destino/fôlego/recarga**: `cargo test --offline --locked -p insurgencia-sim --test simulacao` e `cargo test --offline --locked -p insurgencia-servidor --test cli`, cobrindo `ValidacaoComandos::validar` com destino distante, Fôlego 14, recarga 300 e buff ativo; capturar `$TMP/saidas/validacao.txt` | Destino igual à posição com Fôlego ≥ 15, recarga 0 e buff 0 é aceito; cada caso inválido devolve `CodigoRecusa` tipado sem custo, sem buff, sem avanço de `seq`/`request_id`, sem mudança de progresso de coleta ou hash; `ultima_recusa` permanece sticky e participa do hash | sim |
| 3 | **90 vs 180 ticks e recompensa igual**: na mesma suíte da sim, comparar coleta iniciada sob buff com a baseline sem buff; capturar `$TMP/saidas/coleta-90-180.txt` | Sob buff a coleta conclui exatamente no tick **90**; sem buff, no tick **180** (`TICKS_COLETA`); a recompensa é idêntica nos dois casos, sem aceleração tick a tick e sem crédito antecipado | sim |
| 4 | **Sem acumulação (*stacking*)**: reaplicar `coleta_eficiente` enquanto o buff 150 e a recarga 300 estão ativos; capturar `$TMP/saidas/sem-stacking.txt` | A reaplicação é recusada com `CodigoRecusa` tipado: Fôlego, buff, recarga, monotônicos, progresso e hash permanecem inalterados; nenhuma duração é somada e a recarga não é renovada | sim |
| 5 | **Decrescentes e reativação**: avançar ticks sobre o buff/recarga ativos, conferir os campos a cada tick e, após expirarem, reaplicar com Fôlego suficiente; capturar `$TMP/saidas/decremento-reativacao.txt` | Buff e recarga decrescem **exatamente uma vez por tick**, em ordem estável de `peer_id`, sem salto ou dupla contagem; ao zerarem, um novo `coleta_eficiente` válido é aceito e restabelece 150/300; os campos são visíveis em `serializar_jogadores`/`hash_estado` | sim |
| 6 | **Hash determinístico**: executar duas vezes a mesma sequência de comandos em `Mundo` independentes (mesmo `epoch`/`match_id`), comparar `hash_estado()` a cada tick e contrastar com baseline sem o ativo e com comando recusado; capturar `$TMP/saidas/hash.txt` | Resumos SHA-256 iguais entre replays equivalentes, diferentes da baseline e inalterados após recusa; **zero literais fixos de 64 hex** na sim; nenhuma expectativa afrouxada para obter verde | sim |
| 7 | **Gates Python + clippy/fmt**: executar os cinco `python3 scripts/ci/*.py`, `cargo check --locked -p insurgencia-cliente --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings` e `cargo fmt --all -- --check`; capturar `$TMP/saidas/gates.txt` e `gates.json` | Cinco gates `PASS`, `clippy` e `fmt` com saída limpa e `exit=0`, cliente compilado sem janelas; todo `const` em função de teste declarado antes do primeiro statement (`items_after_statements`); inspeção estática de `KeyCode::KeyK`/`texto_hud` sem segunda verdade | sim |
| 8 | **`verify.sh` remoto**: rodar `./verify.sh` e observar os jobs `gates`/`rust` em `https://github.com/lfgranja/insurgencia/actions/workflows/ci-gates.yml` no head da feature; capturar `$TMP/saidas/verify.txt` e `ci.json` (run, jobs, carimbo) | `verify.sh: ALL STAGES OK` (check → clippy → nextest/test → fmt) e ambos os jobs verdes; até a saída ser colada, tudo permanece **[NV]** — nenhum PASS é afirmado para CI remoto | parcial (depende de observação do CI) |

## Plano de capturas (terminal/JSON)

- `$TMP/saidas/whitelist.txt`/`whitelist.json`, `validacao.txt`, `coleta-90-180.txt`, `sem-stacking.txt`, `decremento-reativacao.txt`, `hash.txt`, `gates.txt`/`gates.json`, `verify.txt`/`ci.json` — saídas dos passos 1–8.
- Contagens agregadas (testes, gates, jobs) só são coladas com comando, carimbo e fonte; nenhuma captura existe neste ciclo pelo agente SDD.

## Rollback do roteiro

`rm -rf "$TMP"` remove capturas e arquivos de apoio; encerrar qualquer inspeção sem persistir nada fora do worktree. Se um passo falhar, registrar a saída obtida e corrigir a **fonte** (código/artefato), nunca a expectativa do teste — em especial os casos de *stacking* (passo 4), decrescentes/reativação (passo 5) e hash (passo 6). Não executar `git clean`, `git reset`, force-push, reescrita de história, remoção de artefatos do ciclo, merge/PR, deploy nem reexecução de gates por este agente. Para reverter a feature, aplicar **`git revert <hash do commit da feature>`** — o commit cujo pai é a âncora `d3be560` — preservando o histórico; o rollback está documentado em `release.md` e **não foi executado neste ciclo**.
