# Roteiro de Testes — Mover validado no cliente (ciclo 059, BF-012)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador/SWE · **Evidências aqui**: **[NV]** (nenhum teste, gate, formatador, build, compilação, commit, push, PR, merge, deploy ou issue executado por este agente; os 14 passos abaixo são instruções, nunca resultados observados).

**Aviso de escopo**: decisão 80/20 **144/3125** (`4×3×3×2×2`, impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵) registrada em `.specify/memory/STATE.md` (2026-11-25T00:38:55Z), com consumidor = **jogador do cliente Bevy + validação da sim/CLI**. Contrato fixo, não rediscutido neste roteiro: `PIXELS_POR_METRO = 100.0` é constante de protótipo **`[HIPÓTESE]`** — 260 px/s ÷ 100 = **2,6 m/s ≈ 0,087 m/tick**, abaixo de `LIMITE_MOVIMENTO_M = 1.0`. Nenhuma regra canônica, GDD, `PROTOCOL_VERSION`, contrato, Cargo, CI, stack ou rede é alterada. `mover_jogador` monta `Comando { acao: "mover", destino: metros }`, chama `ValidacaoComandos::validar` **antes** de `agendar_comando` e avança `seq`/`request_id` **somente no aceite**; `sincronizar_sim` lê a posição do espelho `Mundo` e escreve `JogadorLocal` em pixels, sem injetar coordenadas locais no espelho; `sincronizar_avatar` roda **depois** de `sincronizar_sim` e `avancar_simulacao` **antes** dele; todos os helpers de destino/alvo operam em metros via `pixels_para_metros`.

## Escopo e rastreabilidade

| Requisito | Definição do ciclo | Passos |
|---|---|---|
| FR-001 | Conversão explícita pixels↔metros no cliente: `PIXELS_POR_METRO = 100.0` rotulado `[HIPÓTESE]`, `pixels_para_metros`/`metros_para_pixels`, 260 px/s → 2,6 m/s ≈ 0,087 m/tick < `LIMITE_MOVIMENTO_M = 1.0`; sem promoção a cânone e sem mudança de GDD | 2, 3, 5, 6 |
| FR-002 | `mover_jogador` em `crates/cliente/src/main.rs`: `Comando { acao: "mover", destino: metros }`, `ValidacaoComandos::validar` antes de `agendar_comando`, `ComandoLocal.seq`/`request_id` avançados só no aceite, recusa preserva posição e monotônicos, imobilização bloqueia | 3, 5, 6 |
| FR-003 | Ordem e direção da sincronização: `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`; espelho `Mundo` em metros é fonte, `JogadorLocal` em pixels é derivado; nenhuma injeção de pixels no espelho e nenhum helper de destino em pixels | 4, 5, 6, 8 |
| FR-004 | Evidência, validação e release: cinco gates Python, `./verify.sh`, tests/check/clippy/fmt, checks remotos `gates`/`rust`, capturas, **BF-012 fechado**, BF-003b e BF-013 abertos, R6 **N/A**, PR #76 intocável, rollback = revert do único commit de feature | 1, 7–14 |

T-IDs no escopo: **T001–T010** (cadeia SDD do ciclo 059 — constituição/branch, intake/research, define/shape, decide/clarify, specify, plan, checklist, revisão, tasks, analyze/implementação/converge/validate). R6/taskstoissues não pertence à cadeia: é **N/A** por constituição v1.2.2 — PARE se invocada, **nenhuma issue simulada** e nenhum `gh` improvisado. Promoção `dev → main` e deploy são exclusivos do SWE; o PR #76 pertence a outro owner com babysitter ativo e **não é tocado** neste roteiro.

## Pré-condições

1. Worktree `.specify/queue/worktrees/cycle-059/`, branch `feature/059-mover-validado`, base `origin/dev = 737b876` (`737b876142396f51d33b16c39a36ee5e61d6220a`). HEAD deve conferir com a base antes da aplicação do único commit de feature; eventuais alterações pré-existentes em `.specify/memory/STATE.md` pertencem ao registro da decisão e não são produzidas por este roteiro.
2. Toolchain Rust fixada em `rust-toolchain.toml`, `python3` e `gh` disponíveis. `TMP=$(mktemp -d)` fora do worktree; `mkdir -p "$TMP/saidas" "$TMP/saidas/gates"`.
3. Consumer = jogador do cliente Bevy e validação sim/CLI; a implementação de referência é handoff ao SWE/orquestrador, restrita ao cliente e, se necessário, a testes da sim. Contratos, `PROTOCOL_VERSION`, GDD, baseline, Cargo, CI, stack e rede não são fontes alteradas.
4. Nenhum teste, gate, build, formatador, commit, push, PR, merge, deploy ou issue executado por este agente; toda saída dos passos permanece **[NV]** até execução ao vivo pelo orquestrador/SWE.

## Superfície

- Terminal local: `git`, `cargo test`, `cargo check`, `cargo clippy`, `cargo fmt`, `./verify.sh`, cinco scripts `python3 scripts/ci/*.py`.
- Remoto **[NV]**: GitHub Actions (`gates`, `rust`) no head do PR, via `gh`/fonte autorizada.
- Gráfica **[NV]**: janela do cliente Bevy apenas no passo 13 — não automatizável localmente (justificativa abaixo).

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Superfície | Automatizável? | Captura |
|:--|:--|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD origin/dev`, `git branch --show-current` e `date -u`; listar os artefatos do ciclo e conferir a decisão 144/3125 e a rastreabilidade FR-001..FR-004 ↔ SC ↔ T001..T010 no `STATE.md` | HEAD/base = `737b876`, branch = `feature/059-mover-validado`, único commit de feature pendente; BF-012 ainda aberto antes da implementação, BF-003b/BF-013 abertos, PR #76 intocável; nenhum passo executado por este agente | terminal / git | sim | `$TMP/saidas/ancora.txt` **[NV]** |
| 2 | Inspecionar a validação da sim: `grep -nE 'LIMITE_MOVIMENTO_M\|ValidacaoComandos\|fn validar\|MovimentoForaDoLimite\|DestinoInvalido\|SeqOuRequestReplay' crates/sim/src/lib.rs` | `LIMITE_MOVIMENTO_M = 1.0`; `ValidacaoComandos::validar(&Mundo, peer_id, acao, seq, request_id, destino)` recusa `mover` com `MovimentoForaDoLimite` acima de 1,0 m, `DestinoInvalido` sem destino e `SeqOuRequestReplay` em monotônicos; origem/destino em metros; nenhuma regra nova introduzida | inspeção estática / sim | sim | `$TMP/saidas/sim-validacao.txt` **[NV]** |
| 3 | Inspecionar o cliente: `grep -nE 'PIXELS_POR_METRO\|pixels_para_metros\|metros_para_pixels\|mover_jogador\|Comando\|validar\|agendar_comando\|seq\|request_id' crates/cliente/src/lib.rs crates/cliente/src/main.rs` | `PIXELS_POR_METRO = 100.0` rotulado `[HIPÓTESE]`; `mover_jogador` monta `Comando { acao: "mover", destino: metros }`, chama `ValidacaoComandos::validar` **antes** de `agendar_comando`, avança `seq`/`request_id` só no aceite; recusa preserva posição e monotônicos; `imobilizado_ticks > 0` bloqueia; todo helper de destino/alvo converte via `pixels_para_metros` | inspeção estática / cliente | sim (comportamento dinâmico nos passos 5–6) | `$TMP/saidas/cliente-mover.txt` **[NV]** |
| 4 | Conferir a ordem dos sistemas e a direção da sincronização em `crates/cliente/src/main.rs` (`avancar_simulacao`, `sincronizar_sim`, `sincronizar_avatar`, `texto_hud`) | `avancar_simulacao` roda **antes** de `sincronizar_sim`; `sincronizar_avatar` roda **depois**; `sincronizar_sim` lê `Simulacao.0.jogadores[PEER_ID].posicao` em metros e escreve `JogadorLocal` em pixels; **nenhuma** coordenada pixel-local é injetada no espelho; HUD deriva da posição local sincronizada | inspeção estática / cliente | sim (visual real **[NV]** no passo 13) | `$TMP/saidas/ordem-sincronizacao.txt` **[NV]** |
| 5 | `cargo test --locked -p insurgencia-cliente --lib` | Suíte verde, `0 failed`; helpers de conversão e estado do jogador do cliente sem regressão; nenhum teste relaxado para acomodar a fonte | cargo test (lib do cliente) | sim | `$TMP/saidas/cliente-lib.txt` **[NV]** |
| 6 | `cargo test --locked -p insurgencia-sim --test simulacao` | Suíte verde, `0 failed`; testes existentes de movimento, limite, cota e determinismo (`movimento_aplica_destino`, `limite_de_movimento`, `comando_invalido_nao_suprime_ataque_valido_no_mesmo_tick`) preservados; aceite/recusa de `mover` e hash determinísticos | cargo test (sim) | sim | `$TMP/saidas/simulacao.txt` **[NV]** |
| 7 | `cargo test --locked -p insurgencia-servidor --test cli` | Suíte verde (10 testes de integração), validação do CLI espelhando a sim; contrato e protocolo inalterados | cargo test (servidor) | sim | `$TMP/saidas/cli.txt` **[NV]** |
| 8 | `cargo check --locked -p insurgencia-cliente --all-targets` | EXIT `0`; binário, lib e demais targets do cliente compilam; sem novos `#[allow]`, `unwrap()` ou `.expect()` | cargo check | sim | `$TMP/saidas/check-cliente.txt` **[NV]** |
| 9 | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | EXIT `0`, nenhum aviso no workspace; convenções `#![deny(clippy::pedantic)]` preservadas, sem `allow` novo | cargo clippy | sim | `$TMP/saidas/clippy.txt` **[NV]** |
| 10 | `cargo fmt --all -- --check` | EXIT `0`, nenhum diff de formatação | cargo fmt | sim | `$TMP/saidas/fmt.txt` **[NV]** |
| 11 | Gates locais: `./verify.sh`; `python3 scripts/ci/check_sdd_completeness.py`; `python3 scripts/ci/check_work_order_dag.py`; `python3 scripts/ci/check_task_id_uniqueness.py`; `python3 scripts/ci/check_converge_append_only.py`; `python3 scripts/ci/hub_docs_gate.py` | `ALL STAGES OK`; os cinco scripts Python encerram com exit `0`: SDD sem lacunas em FR-001..FR-004/T001..T010 e BF-012 fechado, DAG sem ciclo/auto-aresta/órfã, T001–T010 sem duplicatas, `converge.md` append-only, hub-docs sem deriva | terminal / verify.sh + python3 / gates | sim | `$TMP/saidas/verify.txt` e `$TMP/saidas/gates/{sdd,dag,task-id,converge,hub-docs}.txt` **[NV]** |
| 12 | **Checks remotos [NV]**: no head do PR, conferir via `gh pr checks`/`gh run view` (fonte autorizada) os jobs `gates` e `rust` no SHA final | Ambos `gates` e `rust` green no SHA do head; `gates` executando os cinco scripts Python, `rust` com clippy `-D warnings` e as suítes de teste; ACR sem blockers; registrar run ID e carimbo | GitHub Actions / gh (remoto) | parcial (consulta automatizada; execução depende do CI) — **[NV]** | `$TMP/saidas/remote-checks.txt` **[NV]** |
| 13 | **Captura visual do HUD Bevy [NV]**: iniciar o cliente em máquina com X11/GPU (pelo SWE; este agente não abre janela) e observar avatar/HUD durante WASD, recusa e imobilização | Avatar e linha `Posicao` avançam somente quando o núcleo aceita `mover`; posição em pixels coerente com o espelho em metros; recusa e imobilização preservam posição e monotônicos; screenshot + carimbo anexados a `.specify/validations/mover-validado/evidence.md` — permanece **[NV]** | GUI / cliente Bevy | **não (local)** — **[NV]** | `$TMP/saidas/hud-visual.md` + screenshot **[NV]** |
| 14 | **Rollback [NV]**: reverter o único commit de feature do worktree, devolvendo HEAD a `737b876` | HEAD = base `737b876`, testes e gates no baseline; nenhum artefato do ciclo removido à força; rollback documentado, **não executado** por este agente | terminal / git | sim (instrução; não executada por este agente) | `$TMP/saidas/rollback.txt` **[NV]** |

## Plano de capturas

- `$TMP/saidas/ancora.txt` (passo 1); `sim-validacao.txt`, `cliente-mover.txt` e `ordem-sincronizacao.txt` (passos 2–4).
- `$TMP/saidas/{cliente-lib,simulacao,cli,check-cliente,clippy,fmt}.txt` (passos 5–10); `$TMP/saidas/verify.txt` e `$TMP/saidas/gates/{sdd,dag,task-id,converge,hub-docs}.txt` (passo 11).
- `$TMP/saidas/remote-checks.txt` (passo 12), `$TMP/saidas/hud-visual.md` + screenshot (passo 13) e `$TMP/saidas/rollback.txt` (passo 14) — **[NV]**.
- Toda captura permanece rotulada **[NV]** até o orquestrador/SWE executar o passo correspondente; nenhuma captura existe neste ciclo.

## Itens [NV] e escalonamento

- **Passo 12 — checks remotos [NV]**: `gates`/`rust` rodam no GitHub Actions sobre o head do PR; somente o SWE confirma o SHA final e o estado do run. Não reexecutar localmente nem afirmar verde sem leitura remota.
- **Passo 13 — captura visual do HUD Bevy [NV]**: **não automatizável localmente**. Justificativa: este ambiente não dispõe de janela gráfica automatizável para inspeção e há precedente documentado no ciclo 058 de compilação do cliente Bevy excedendo **900 s (15 min)** em tentativa local; a verificação local é estática e indireta (passos 3–6 e 8). **Escalonamento**: o SWE/orquestrador executa o cliente em máquina com X11/GPU, anexa screenshot e carimbo a `.specify/validations/mover-validado/evidence.md`; até lá o item permanece **[NV]** e a evidência visual de FR-002/FR-003 fica pendente.
- **Promoção e backlog [NV]**: `dev → main`, deploy e o PR #76 (outro owner) são SWE-exclusivos; BF-012 só é fechado nos artefatos após a execução validada deste roteiro.

## Rollback do roteiro

`rm -rf "$TMP"` remove capturas e arquivos de apoio fora do worktree. Se algum passo falhar, registrar a saída obtida e corrigir a **fonte** (código/artefato do ciclo), nunca a expectativa do teste — em especial unidade promovida a cânone (passo 2), `seq`/`request_id` avançados sem aceite (passo 3), injeção de pixels no espelho ou ordem de sistemas trocada (passo 4). Para reverter a mudança, **reverter o único commit de feature** (`git revert <sha-do-commit-de-feature>`), devolvendo o worktree a `737b876`. Não executar `git clean`, `git reset --hard`, force-push, remoção de artefatos do ciclo, merge de PR, deploy, abertura de issue ou reexecução de gates por este agente; `dev → main` e deploy permanecem SWE-exclusivos.
