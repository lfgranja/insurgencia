# Roteiro de Testes — Defesa de barricada e autogestão (ciclo 044)

**Status:** escrito antes do PR · **Execução:** pelo orquestrador/SWE · **Evidências aqui:** **[NV]** (nada executado neste ciclo pelo agente SDD)

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passos 2–6 — regra canônica GDD §7.1:306: **+1 ponto de autogestão a cada 10 s de barricada intacta = 300 ticks a 1/30 s**; herdam-se 200 PV e 360 ticks da barricada 040; dono no mesmo ponto como defensor, Guerreiro/Artesão como simplificação de escopo |
| FR-002 | passos 2, 7, 9 — estado previsto: `pontos_autogestao: i64` em `Jogador`, `defesa_ticks: i64` em `Barreira`, constantes `TICKS_AUTOGESTAO_POR_PONTO = 300` e `PONTOS_AUTOGESTAO_POR_DEFESA = 1`; nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto ou CI; sem Lightyear, async ou sockets |
| FR-003 | passos 2–6 — fase determinística em `fase_defesa` (entre `fase_skills` e `fase_finish`), antes do decremento de duração e da remoção da barreira em `fase_finish`, percorrendo `BTreeMap` por `id`: +1 tick por tick enquanto a barreira está intacta e o dono está no mesmo ponto; crédito de +1 e zero do contador ao completar 300; saída do dono, dono ausente, destruição ou expiração zeram o acúmulo e não creditam; guarda explícita `ticks_restantes <= 1` impede acúmulo e crédito no tick da remoção; `mover` sem mudança de posição não interrompe; sem dupla contagem da mesma barricada na janela |
| FR-004 | passo 7 — `hash_estado()` inclui `pontos_autogestao` e `defesa_ticks`, preservando ordem de `BTreeMap`; replay idêntico tick a tick |
| FR-005 | passo 8 — HUD mostra `Autogestão` com o valor do jogador local, sem placeholder |
| FR-006 | passos 9, 10 — `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100`; constituição v1.2.2, R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A** |
| Governança (fora dos FR) | passos 1, 10 — exatamente 5 artefatos na fronteira, `tasks` antes de `analyze` (READ-ONLY), `converge` append-only, rótulo **[NV]** em toda captura e rollback documentado |
| SC-001..SC-006 | passos 1–10 |

T-IDs relacionados: T001–T012 (`tasks` = T009, `analyze` READ-ONLY = T010, `converge`/R4 = T011, `release` R7 = T012). A implementação em `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs` e `crates/cliente/src/main.rs` é handoff ao SWE/orquestrador, fora da fronteira SDD deste ciclo e **[NV]** até execução.

## Pré-condições

1. Worktree `/home/luis/development/MDMV/projetos/insurgencia/.specify/queue/worktrees/defesa-barricada-044/`, branch `044-defesa-barricada`, base `b6d2f36`; sessão `20261123T054700Z-defesa-barricada`. Artefatos SDD na fronteira exata, com `tasks` antes de `analyze.md` (READ-ONLY).
2. Implementação de referência pronta pelo SWE: `pontos_autogestao: i64` em `Jogador`, `defesa_ticks: i64` em `Barreira`, fase determinística em `fase_defesa` (entre `fase_skills` e `fase_finish`), `hash_estado()` refletivo e HUD `Autogestão`. Barricada de referência com 200 PV e 360 ticks (ciclo 040). Nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto ou CI; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100`.
3. `TMP=$(mktemp -d)` fora do worktree para capturas; `mkdir -p "$TMP/saidas"`. Nenhum build, gate, formatador, teste, commit, push, PR, merge ou deploy executado por este agente — os passos abaixo são instruções ao orquestrador/SWE.

## Superfície documental/CLI

- Inspeção: `git rev-parse`/`git status --short`, `find`, `grep`, `diff`, `cat`, `date -u`, leitura de fontes (GDD §7.1:306, constituição v1.2.2, `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs`, `crates/cliente/src/main.rs`).
- Testes: `cargo test -p sim` e, se aplicável, `cargo test -p cliente` (comando canônico do hub), conduzidos **pelo orquestrador**, com saída redirecionada para `$TMP/saidas`; nenhuma janela gráfica, rede, credencial ou socket.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD` e `date -u > "$TMP/saidas/carimbo.txt"`; listar os 5 artefatos do ciclo e conferir `tasks` antes de `analyze` > `"$TMP/saidas/inventario.txt"` | base `b6d2f36`, branch `044-defesa-barricada`, 5 artefatos na fronteira; `.specify/memory/STATE.md`, `AGENTS.md`, `concurrency.jsonl` e paths alheios intactos | sim |
| 2 | **Caso 300 ticks e +1 autogestão**: construir uma barricada, manter o dono exatamente no ponto da barreira e avançar 299, 300 e 301 ticks, registrando `defesa_ticks` e `pontos_autogestao` a cada tick em `"$TMP/saidas/caso-300-ticks.txt"` | nenhum crédito nos ticks 1–299 (`defesa_ticks` cresce de 1 em 1); no tick 300, `pontos_autogestao` aumenta de **exatamente +1** e `defesa_ticks` volta a **0**; no tick 301 o contador recomeça em 1, sem novo crédito; a barreira nasce com 200 PV e 360 ticks | sim |
| 3 | **Caso dono fora do ponto**: com defesa parcial acumulada, emitir `mover` que altera a posição do dono, avançar mais de 300 ticks e depois devolvê-lo ao ponto da barreira; em casos separados, testar dono ausente e `mover` sem mudança de posição; capturar em `"$TMP/saidas/dono-fora.txt"` | `defesa_ticks` zera no tick em que o dono sai; nenhum crédito enquanto estiver fora, mesmo após 300 ticks; ao retornar, a contagem recomeça do zero — são necessários 300 ticks **contínuos** de presença para o próximo +1; dono ausente zera sem crédito; `mover` sem mudança de posição não interrompe; a barreira permanece intacta | sim |
| 4 | **Caso destruição**: com defesa parcial acumulada, aplicar dano registrado até zerar os 200 PV da barreira e avançar ticks seguintes; capturar em `"$TMP/saidas/destruicao.txt"` | a barreira sai de `barreiras` quando `pv` chega a 0; `defesa_ticks` zera; nenhum crédito no tick da destruição nem nos seguintes, mesmo que o dono permaneça no ponto | sim |
| 5 | **Caso expiração**: construir, acompanhar `ticks_restantes` nos ticks 299, 300, 359 e 360 e registrar `defesa_ticks`/`pontos_autogestao` em `"$TMP/saidas/expiracao.txt"` | duração de **360 ticks** preservada; a barreira some no tick 360 (não antes); o +1 ocorre apenas no tick 300; os 60 ticks seguintes não creditam; no tick de remoção (`ticks_restantes <= 1`) a guarda impede acúmulo e crédito, e `defesa_ticks` zera sem crédito adicional | sim |
| 6 | **Caso janela de crédito por barricada**: manter o dono no ponto até o fim da vida da barreira e, em caso separado, colocar duas barreiras do mesmo dono no mesmo ponto; capturar em `"$TMP/saidas/janela-unica.txt"` | cada barricada credita **+1 por janela de 300 ticks**; duas barricadas sobrepostas do mesmo dono podem creditar **+2** no total (um crédito por barricada), mas a mesma barricada nunca credita duas vezes na mesma janela; cada incremento de defesa é no máximo um por barricada por tick | sim |
| 7 | **Caso hash divergente e replay**: comparar `hash_estado()` de dois mundos idênticos exceto por `pontos_autogestao`/`defesa_ticks`, e executar a mesma sequência de comandos em dois `Mundo` independentes (mesmo `epoch`/`match_id`) comparando o resumo a cada tick; capturar em `"$TMP/saidas/hash-determinismo.txt"` | resumos SHA-256 diferentes no primeiro par e idênticos tick a tick no segundo; ambos os campos novos entram na serialização de jogadores/barreiras, em ordem estável de `BTreeMap` | sim |
| 8 | **Caso HUD**: inspecionar o texto produzido por `atualizar_texto` com o jogador local em diferentes valores de `pontos_autogestao`; capturar em `"$TMP/saidas/hud.txt"` | linha `Autogestão` presente com o valor numérico igual a `pontos_autogestao` do jogador local; campos anteriores (Materiais, Barreiras, Construindo, Coleta) preservados; nenhum placeholder ou rótulo improvisado | sim |
| 9 | **Fronteira de contrato/Cargo e regras Rust**: `git status --short` e `grep` por `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, `Lightyear`, `async`, `socket`, `#[allow]`, `unwrap()`/`.expect()`, `unsafe` > `"$TMP/saidas/fronteira.txt"`; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e `max_width = 100` | nenhum desses caminhos ou padrões alterado; mudança restrita a `lib.rs`, `simulacao.rs` e `main.rs`; sem nova dependência, protocolo, rede ou papel de Guerreiro/Artesão implementado | sim |
| 10 | **Governança, R6 N/A, rollback e [NV]**: `grep -nE 'tasks|analyze|R1|R4|R6|1.0.0-local'` na constituição e nos artefatos > `"$TMP/saidas/governanca.txt"`; conferir rótulo **[NV]** em toda captura e aplicar o rollback documentado | `tasks` antes de `analyze` (READ-ONLY); R4 **condicionado à execução deste roteiro** e antes de R7 `1.0.0-local`; `converge` append-only; **R6 N/A** — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado, dívida de automação registrada; nenhum build/gate/commit executado pelo agente SDD e rollback documentado em `release.md`, nunca executado | parcial (revisão humana) |

**Nota de correção pós-ACR (2026-11-23):** o passo 6 passou a descrever a regra real da janela de crédito — uma barricada credita +1 a cada 300 ticks, duas barricadas sobrepostas do mesmo dono podem somar +2, e a mesma barricada nunca credita duas vezes na janela; a guarda de expiração `ticks_restantes <= 1` foi registrada no passo 5 e no FR-003 de `requirements.md`.

## Plano de capturas (terminal)

- `$TMP/saidas/carimbo.txt`, `inventario.txt`, `caso-300-ticks.txt`, `dono-fora.txt`, `destruicao.txt`, `expiracao.txt`, `janela-unica.txt`, `hash-determinismo.txt`, `hud.txt`, `fronteira.txt`, `governanca.txt` — saídas dos passos 1–10.
- Toda captura é rotulada **[NV]** até o orquestrador executar o passo correspondente; nenhuma captura existe neste ciclo pelo agente SDD.

## Rollback do roteiro

`rm -rf "$TMP"` remove capturas e arquivos de apoio; encerrar qualquer inspeção sem persistir nada fora do worktree. Se um passo falhar, registrar a saída obtida e corrigir a **fonte** (código/artefato), nunca a expectativa do teste — em especial crédito antecipado antes de 300 ticks (passo 2), acúmulo após saída do dono (passo 3), destruição (passo 4), acúmulo ou crédito no tick de expiração (passo 5), crédito duplo da mesma barricada na janela (passo 6), hash omisso (passo 7) e HUD omisso (passo 8) — e revalidar o histórico append-only em `converge.md`. Não executar `git clean`, `git reset`, force-push, remoção de artefatos do ciclo, merge/PR, deploy nem reexecução de gates por este agente. Para reverter o ciclo, seguir `release.md` (rollback documentado, nunca executado neste ciclo).
