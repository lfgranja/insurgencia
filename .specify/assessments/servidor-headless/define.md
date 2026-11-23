# Define — Servidor headless determinístico

- **Slug**: `servidor-headless`
- **Created**: 2026-11-23T04:30Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (`Cargo.toml`, `crates/sim/src/lib.rs`, constituição v1.2.2)

## Problem Statement

O Núcleo já é determinístico: `Mundo` tem relógio de 30 Hz, fila `VecDeque` de comandos, `avancar_tick` por fases e `hash_estado` SHA-256 (`crates/sim/src/lib.rs`). Ainda assim, não existe uma forma executável e independente de janela de **exercitar essa determinação e produzir evidência comparável**. Hoje o hash é observado por testes internos e pelo exemplo `demo_deterministica.rs`; não há um binário com fronteira textual estável que leia uma sequência de comandos, avance N ticks e imprima um hash por tick. Sem isso, não é possível responder com um `diff` vazio à pergunta "duas execuções com a mesma entrada produzem o mesmo estado?", qualquer trabalho de rede (Lightyear) não tem uma verdade determinística para sincronizar, e a verificação do Núcleo continua dependente de código de teste acoplado em vez de um protocolo de saída auditável.

## Affected Users & Stakeholders

- **Users**: desenvolvedores e auditores que precisam reproduzir o determinismo do Núcleo com um comando e comparar saídas, sem abrir o cliente; mantenedores de captura/demo — precisam de uma fonte textual estável para registro posterior.
- **Stakeholders**: mantenedores do `insurgencia-sim` — precisam que o hash continue sendo a única fonte de verdade; mantenedores da workspace Rust — precisam de um crate isolado, sem dependências externas novas e sem mudança de protocolo; SWE/orquestrador — precisa de uma fatia verificável por `diff` e por contagem de linhas antes de qualquer netcode; designer — precisa que os valores e ações canônicos não sejam reabertos.

## Goals

- Criar o crate `crates/servidor` com o binário `insurgencia-servidor`, dependendo **somente** de `insurgencia-sim`: sem novas dependências externas, sem `async`, sem sockets e sem Lightyear; `Cargo.toml` da workspace ganha o membro e `Cargo.lock` é gerado pelo Cargo.
- Aceitar `--ticks N` com inteiro positivo e comandos pela stdin, um por linha, `peer_id acao seq request_id [x y]`, com coordenadas obrigatórias para `mover`, `coletar_materiais` e `construir_barreira` e proibidas para as demais ações conhecidas.
- Agendar os comandos em ordem antes do tick 0, preservando a ordem de leitura da stdin na fila do `Mundo`.
- Emitir exatamente uma linha por tick, `<tick>\t<hash sha256>`, começando em `0`, usando `Mundo::hash_estado()` como única fonte do hash.
- Responder a entrada malformada ou ação desconhecida com erro determinístico e código de saída `2`, sem pânico, usando `Result`/`ExitCode`.
- Seguir a disciplina Rust: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, documentação `///` em itens públicos.
- Cobrir com testes unitários do parser e teste de integração CLI: saída idêntica entre duas execuções, contagem de linhas igual a `N` e rejeição de entrada inválida.

## Non-Goals

- Netcode, Lightyear, sockets, `async`, transporte, lobby, matchmaking, predição ou reconciliação.
- Novas dependências externas, novo campo em `crates/contratos`, alteração em `PROTOCOL_VERSION` ou reimplementação do hash fora de `Mundo::hash_estado()`.
- Novas ações de simulação, renegociação de valores canônicos ou mudança nas regras já validadas de coleta e barricada.
- Janela, renderização, Bevy, plataforma de distribuição ou captura de vídeo.
- R6: N/A por constituição v1.2.2 — dívida de automação, nunca issue simulada, nunca `gh` improvisado.
- Executar gates, build, testes, formatadores, commit, push ou PR neste assessment.

## Success Metrics

- Com a mesma entrada, duas execuções do binário produzem **saída byte a byte idêntica** (baseline: nenhum binário existente — **[NV]** até verificação).
- Para `--ticks N` com `N > 0`, a saída tem **exatamente `N` linhas**, numeradas de `0` a `N-1`, uma por tick (baseline: **[NV]**).
- `--ticks 0`, valores negativos e valores não inteiros são recusados com erro determinístico antes de qualquer tick (baseline: **[NV]**).
- Comandos com coordenadas ausentes para `mover`, `coletar_materiais` ou `construir_barreira`, e com coordenadas presentes para qualquer outra ação conhecida, são rejeitados pelo parser com código `2` (baseline: **[NV]**).
- Linha malformada (contagem de campos errada ou inteiros inválidos) e ação desconhecida geram erro determinístico, código de saída `2` e nenhum pânico (baseline: **[NV]**).
- O campo `<hash sha256>` de cada linha é exatamente o retorno de `Mundo::hash_estado()` — nenhuma serialização paralela no servidor (baseline: **[NV]**).
- O crate compila sob `forbid(unsafe_code)`, `deny(clippy::pedantic)` e `max_width=100`, sem `unwrap`/`expect`/`#[allow]`, com itens públicos documentados (baseline: **[NV]**).
- Testes unitários do parser e teste de integração CLI cobrem determinismo, contagem de linhas e rejeição de entrada inválida (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, o determinismo do Núcleo permanece uma propriedade afirmada por testes internos, sem um protocolo executável que a exponha. Não há recibo comparável para auditar mudanças no `sim`, a captura/demo depende de código acoplado, e o próximo passo de rede seria construído sobre uma verdade que ainda não pode ser provada por `diff` — invertendo a ordem determinismo → sincronização que o GDD §16.3:1490 e a Constituição I/VI exigem. O custo é perder a forma mais barata de verificação honesta do estado autoritativo, justamente quando os ciclos 039–041 já fecharam regra, barreira e coleta no Núcleo.

## Open Questions

- Nenhuma clarificação de design: crate, dependências, formato de comando, regras de coordenadas, formato de saída, código de erro e testes estão fechados no contrato (R3 = 0 clarificações).
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** A inclusão do membro via `members = ["crates/*"]` satisfaz a redação do contrato, ou exige entrada explícita em `Cargo.toml`? Validar no `/speckit.specify`.
- **[NV]** O tick 0 imprime o hash antes ou depois de `avancar_tick`, e como o binário garante existência do `peer_id` (`adicionar_jogador`) antes de agendar? Validar na especificação.
- **[NV]** Os nomes definitivos de T001..T012 são os de `specs/042-servidor-headless/tasks.md`, ainda a produzir.

## Rastreabilidade (escopo do problema)

- FR-001 → crate `crates/servidor`, binário `insurgencia-servidor`, dependência única e membro da workspace; FR-002 → CLI `--ticks N` com inteiro positivo; FR-003 → parser de comandos e regra de coordenadas; FR-004 → agendamento em ordem antes do tick 0; FR-005 → saída `<tick>\t<hash sha256>` por tick a partir de 0; FR-006 → erro determinístico, código 2, sem pânico, e disciplina Rust/testes.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis especificados em `specs/042-servidor-headless/`, fora desta fronteira.
- R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A** por constituição v1.2.2.
