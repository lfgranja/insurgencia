# Roteiro de Testes — Servidor headless determinístico (ciclo 042)

**Status:** escrito antes do PR · **Execução:** pelo orquestrador/SWE · **Evidências aqui:** **[NV]** (nada executado neste ciclo pelo agente SDD)

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passo 9 — crate `crates/servidor` com binário `insurgencia-servidor`, dependendo apenas de `insurgencia-sim`; `Cargo.toml` da workspace com o membro e `Cargo.lock` gerado pelo Cargo; sem novas dependências externas, async, sockets ou Lightyear |
| FR-002 | passos 2, 5 — `--ticks N` com inteiro positivo; ausência, zero, negativo e não-numérico rejeitados; uma linha por tick |
| FR-003 | passos 3, 4 — parser `peer_id acao seq request_id [x y]`, com coordenadas obrigatórias para `mover`, `coletar_materiais`, `construir_barreira` e proibidas para `sinalizar_sincronia`, `sinalizar_alavanca`, `atacar_forte`; comandos agendados em ordem antes do tick 0 |
| FR-004 | passos 5, 6 — saída `<tick>\t<hash sha256>`, iniciando em `0`, com hash canônico de `Mundo::hash_estado()` (64 hex) e sequência contínua de ticks |
| FR-005 | passos 2, 8 — erro determinístico e código de saída **2** para argumento inválido, entrada malformada e ação desconhecida, sem pânico, via `Result`/`ExitCode` |
| FR-006 | passos 7, 9, 10 — duas execuções com mesma entrada produzem saída idêntica byte a byte; `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, `///` em itens públicos; constituição v1.2.2, R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A** |
| Governança (fora dos FR) | passos 1, 10 — exatamente 5 artefatos na fronteira, `tasks` antes de `analyze` (READ-ONLY), `converge` append-only, rótulo **[NV]** em toda captura e rollback documentado |
| SC-001..SC-006 | passos 1–10 |

T-IDs relacionados: T001–T012 (SDD, com `tasks` antes de `analyze` READ-ONLY e os artefatos de validação/release no mesmo intervalo). A implementação em `crates/servidor/` e a alteração do `Cargo.toml` da workspace são handoff ao SWE/orquestrador, fora da fronteira SDD deste ciclo e **[NV]** até execução.

## Pré-condições

- Worktree `.specify/queue/worktrees/servidor-headless-042/`, branch `042-servidor-headless`, base `b099093`; sessão `20261123T043000Z-servidor-headless`. Artefatos SDD presentes na fronteira exata, com `tasks` antes de `analyze.md` (READ-ONLY).
- Implementação de referência pronta pelo SWE: `crates/servidor` com o binário `insurgencia-servidor`, membro da workspace, `Cargo.lock` gerado pelo Cargo, parser e testes unitários/integração. Nenhuma dependência externa nova, nenhum async, socket ou Lightyear; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, `///` em itens públicos.
- `TMP=$(mktemp -d)` fora do worktree para capturas; `mkdir -p "$TMP/saidas"`. Nenhum build, gate, formatador, teste, commit, push, PR, merge ou deploy executado por este agente — os passos abaixo são instruções ao orquestrador/SWE.

## Superfície documental/CLI

- Inspeção: `git rev-parse`/`git status --short`, `find`, `grep`, `diff`, `cat`, `date -u`, leitura de fontes (constituição v1.2.2, `Cargo.toml`, `crates/sim/src/lib.rs` e `spec.md`).
- Testes: `cargo test -p insurgencia-servidor` e execução do binário com entrada pela stdin, conduzidas **pelo orquestrador**, com saída redirecionada para `$TMP/saidas`; nenhuma janela gráfica, rede, credencial ou socket. As ações são apenas linhas de texto; nenhuma conexão é aberta.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD` e `date -u > "$TMP/saidas/carimbo.txt"`; listar os 5 artefatos do ciclo e conferir `tasks` antes de `analyze` > `"$TMP/saidas/inventario.txt"` | base `b099093`, branch `042-servidor-headless`, 5 artefatos na fronteira; `.specify/memory/STATE.md`, `AGENTS.md`, `concurrency.jsonl` e paths alheios intactos | sim |
| 2 | **CLI `--ticks`**: executar `--ticks 1`, `--ticks 3` e, em casos separados, `--ticks 0`, `--ticks -1`, `--ticks abc` e sem `--ticks`; capturar em `"$TMP/saidas/cli-ticks.txt"` | `N` positivo aceito; ausência, zero, negativo e não-numérico produzem erro determinístico e **código de saída 2**, sem pânico e sem linhas de tick | sim |
| 3 | **Parser (testes unitários)**: alimentar linhas válidas e inválidas no formato `peer_id acao seq request_id [x y]`, incluindo `x y` presentes/ausentes nas seis ações conhecidas; capturar em `"$TMP/saidas/parser.txt"` | `peer_id`/`seq`/`request_id` inteiros; `x y` **obrigatórios** em `mover`, `coletar_materiais`, `construir_barreira` e **proibidos** em `sinalizar_sincronia`, `sinalizar_alavanca`, `atacar_forte`; token a mais, token faltando e não-numérico recusados; testes unitários do parser passam | sim |
| 4 | **Agendamento antes do tick 0**: enviar várias linhas válidas pela stdin em ordem e comparar a aplicação com o estado esperado do `Mundo` no tick 0; capturar em `"$TMP/saidas/agendamento.txt"` | toda a stdin é lida antes do primeiro tick e os comandos são agendados via `agendar_comando` **na ordem de entrada**; nenhum comando vaza para o tick 0 fora dessa ordem e nenhuma reordenação depende de mapas, tempo ou aleatoriedade | sim |
| 5 | **Ticks e contagem de linhas**: para `--ticks N` com `N` entre 1 e 10, contar as linhas de saída e verificar o primeiro campo; capturar em `"$TMP/saidas/ticks.txt"` | exatamente **N** linhas, com ticks `0, 1, …, N-1` em ordem crescente e intervalo fixo de 1 por linha; nenhuma linha extra, nenhum tick repetido e nenhum cabeçalho | sim |
| 6 | **Hash por tick**: conferir o formato de cada linha e o valor do hash contra `Mundo::hash_estado()` do estado correspondente; capturar em `"$TMP/saidas/hash.txt"` | linha no formato `<tick>\t<hash sha256>` (tabulação única entre os campos), hash com **64 dígitos hexadecimais**, começando no tick 0; estados distintos produzem hashes distintos e o campo é o hash canônico do núcleo, não um valor improvisado | sim |
| 7 | **Determinismo (teste de integração CLI)**: executar duas vezes o binário com a mesma entrada e o mesmo `--ticks`, comparando `cmp` byte a byte; capturar em `"$TMP/saidas/determinismo.txt"` | saídas **idênticas byte a byte** nas duas execuções; nenhuma dependência de relógio, tempo, aleatoriedade, endereço de socket ou iteração não determinística | sim |
| 8 | **Rejeição determinística**: enviar linha malformada (token faltando, extra ou não-numérico), coordenadas proibidas em ação que não as usa e ação desconhecida, cada caso em execução separada; capturar em `"$TMP/saidas/rejeicao.txt"` | cada caso termina com erro determinístico e **código de saída 2**, sem pânico, sem `unwrap`/`expect` e sem rastro de pilha; a ação desconhecida não é confundida com `ACOES_PERMITIDAS` de `crates/sim/src/lib.rs` | sim |
| 9 | **Fronteira de dependências e regras Rust**: `git status --short`, `grep` por `crates/servidor`, `insurgencia-sim`, `Cargo.toml`, `Cargo.lock`, `async`, `socket`, `Lightyear`, `tokio`, `#[allow]`, `unwrap`/`expect`, `unsafe`; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)`, `max_width = 100` e `///` em itens públicos > `"$TMP/saidas/fronteira.txt"` | o crate depende **somente** de `insurgencia-sim`; workspace com o novo membro e `Cargo.lock` gerado pelo Cargo; nenhuma dependência externa nova, async, socket ou Lightyear; convenções de lint, largura e documentação preservadas | sim |
| 10 | **Governança, R6 N/A, rollback e [NV]**: `grep -nE 'tasks|analyze|R1|R4|R6|1.0.0-local'` na constituição e nos artefatos > `"$TMP/saidas/governanca.txt"`; conferir rótulo **[NV]** em toda captura e aplicar o rollback documentado | `tasks` antes de `analyze` (READ-ONLY); R4 **condicionado à execução deste roteiro** e antes de R7 `1.0.0-local`; `converge` append-only; **R6 N/A** — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado, dívida de automação registrada; nenhum build/gate/commit executado pelo agente SDD e rollback documentado em `release.md`, nunca executado | parcial (revisão humana) |

## Plano de capturas (terminal)

- `$TMP/saidas/carimbo.txt`, `inventario.txt`, `cli-ticks.txt`, `parser.txt`, `agendamento.txt`, `ticks.txt`, `hash.txt`, `determinismo.txt`, `rejeicao.txt`, `fronteira.txt`, `governanca.txt` — saídas dos passos 1–10.
- Toda captura é rotulada **[NV]** até o orquestrador executar o passo correspondente; nenhuma captura existe neste ciclo pelo agente SDD.

## Rollback do roteiro

`rm -rf "$TMP"` remove capturas e arquivos de apoio; encerrar qualquer inspeção sem persistir nada fora do worktree. Se um passo falhar, registrar a saída obtida e corrigir a **fonte** (código/artefato), nunca a expectativa do teste — em especial CLI (passo 2), parser e coordenadas (passo 3), agendamento (passo 4), formato/hash (passos 5–6), determinismo (passo 7) e rejeição (passo 8) — e revalidar o histórico append-only em `converge.md`. Não executar `git clean`, `git reset`, force-push, remoção de artefatos do ciclo, merge/PR, deploy nem reexecução de gates por este agente. Para reverter o ciclo, seguir `release.md` (rollback documentado, nunca executado neste ciclo).
