# Define — Vitória local observável no espelho do cliente

- **Slug**: `mirror-vitoria-local`
- **Created**: 2026-11-24
- **Score**: **270/3125** (`5×3×3×2×3`: impacto 5 × confiança 3 × fechamento 3 × esforço⁻¹ 2 × risco⁻¹ 3)
- **Inputs used**: `intake.md` | `research.md` | `crates/cliente/src/main.rs:528-605` | `crates/sim/src/lib.rs:719-746,2158-2206` | `crates/sim/tests/simulacao.rs:2404-2407,2558-2593` | GDD:275 | Constituição v1.2.2

## Problem Statement

No cliente standalone, as ilhas existem **apenas na cena Bevy**: `configurar` as desenha com `criar_ilha`, mas o `Mundo` da sim recebe somente jogadores e PNJs (`crates/cliente/src/main.rs:528-605`). Não há ilha, súdito nem Nó no espelho, então a HUD informa `Nós: 0/3` e `propaganda_no` nunca encontra alvo — a vitória, embora implementada e testada em `crates/sim/tests/simulacao.rs:2558-2593`, é **inalcançável localmente**. O recorte resolve isso com uma semente estática prototípica injetada no espelho por um helper público da sim, sem tocar em regra canônica, CI, protocolo, Cargo, stack ou rede. É o corte BF-003a do BF-003; **BF-003b (mapa completo de 12 ilhas) fica aberto**.

## DoD do ciclo (referência única; idêntico nos cinco artefatos)

- **DoD-1**: espelho `Mundo` do cliente semeado com **4 ilhas (ids 1..=4)**, **1 súdito (id 1 em `(0.0, 236.0)`)** e **3 nós**.
- **DoD-2**: vitória local alcançável convertendo os **3 nós via `propaganda_no`** — mesma rota do teste `tres_nos_geram_vitoria`.
- **DoD-3**: `hash_estado` **muda** ao semear o espelho (baseline: sem ilhas/súditos/nós).
- **DoD-4**: **BF-003a fechado**; **BF-003b** (mapa completo de 12 ilhas) **permanece aberto**.

## Affected Users & Stakeholders

- **Users**: jogador do cliente Bevy — precisa ver `Nós: 3/3` e poder converter os três nós por `propaganda_no` até `ResultadoPartida::Vitoria`, em vez de um contador travado em `0/3`; quem lê a HUD — precisa de um estado que corresponda ao que a sim já sabe produzir.
- **Stakeholders**: mantenedores da sim — precisam de um helper público único (`semear_espelho_local`) que reutilize `adicionar_ilha`/`adicionar_no` (`lib.rs:719-746`) e preserve os testes determinísticos; mantenedores do cliente — precisam de uma única chamada em `configurar`, sem reimplementar verdade de estado; SWE/orquestrador — precisa de uma fatia verificável por inspeção de código + `./verify.sh` (jobs `gates`/`rust`) sem que o agente execute gates; designer — precisa que os números do GDD:275 continuem `[HIPÓTESE]`, não cânone. **PR #76 não é tocado** (outro owner ativo).

## Goals

- Expor na sim um helper público determinístico que semeie o espelho com **4 ilhas (ids 1..=4), 1 súdito (id 1 em `(0.0, 236.0)`) e 3 nós**, reaproveitando `adicionar_ilha`/`adicionar_no` e a API de súditos existente — **DoD-1**.
- Fazer o cliente chamar esse helper em `configurar`, imediatamente após registrar jogadores e PNJs, para que `propaganda_no` nos 3 nós leve a `Vitoria` como em `crates/sim/tests/simulacao.rs:2558-2593` — **DoD-2**.
- Garantir que `hash_estado` passe a refletir as ilhas/súditos/nós semeados (a serialização em `lib.rs:2158-2206` já inclui nós, resistência e resultado) — **DoD-3**.
- Fechar **BF-003a** e manter **BF-003b** aberto, sem inventar mapa de 12 ilhas — **DoD-4**.
- Preservar a fronteira: nenhuma mudança em regra canônica/GDD, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede; valores rotulados `[HIPÓTESE]` (GDD:275).
- Registrar rastros **FR-001..FR-004 no mapeamento canônico da spec** (helper/semente, chamada no cliente, teste de integração, evidência/validação/release) / SC-001..SC-006 / T001..T012 e constituição v1.2.2 (R1 `go`, R4 antes de R7 `1.0.0-local`, **R6/taskstoissues N/A**).

## Non-Goals

- Mapa completo de 12 ilhas, topologia de rotas, spawning por ilha, reconquista e demais critérios de **BF-003b** — explicitamente fora (DoD-4).
- Promover a `[HIPÓTESE]` do GDD:275 (3 nós, 100 de Resistência, −30 por nó, pulso de −10) a regra canônica; reabrir números de vitória.
- Duplicar a semeadura dentro do cliente; criar uma segunda verdade de estado que diverge dos testes da sim (alternativa rejeitada em `research.md`).
- Apenas exibir `hash_estado`/contadores na HUD sem semear o espelho — não satisfaz DoD-2.
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, jobs `gates`/`rust` do CI, `.github/workflows`, stack, Lightyear/ADR-004 ou rede.
- Executar gates, `./verify.sh`, build, testes, formatadores, commit, push ou PR neste assessment; adotar R6 ou simular emissão de issues; tocar PR #76; fazer `dev → main` ou deploy (exclusivo da SE).

## Success Metrics

- **SC-001 / DoD-1**: `Mundo` no cliente contém exatamente 4 ilhas com ids 1..=4, 1 súdito com id 1 na posição `(0.0, 236.0)` e 3 nós após `configurar` (baseline: zero de cada — **[NV]** até verificação).
- **SC-002 / DoD-2**: agendando `propaganda_no` nos 3 nós e avançando `TICKS_PROPAGANDA_NO` ticks, os três ficam `convertido`, `resistencia_ideologica == 0` e `resultado == Some(ResultadoPartida::Vitoria)`, estável nos ticks seguintes, como em `simulacao.rs:2558-2593` (baseline: vitória inalcançável no cliente — **[NV]**).
- **SC-003 / DoD-3**: `hash_estado()` do espelho semeado difere do `hash_estado()` do espelho sem semente, sem aleatoriedade na semente (baseline: igualdade por estado vazio — **[NV]**).
- **SC-004 / DoD-4**: BF-003a é marcado como fechado no handoff; BF-003b permanece aberto, sem implementação de 12 ilhas.
- **SC-005**: a HUD do cliente deixa de mostrar `Nós: 0/3` e passa a refletir os 3 nós do espelho (baseline: `Nós: 0/3` — **[NV]** até execução).
- **SC-006**: nenhuma mudança em regra canônica, `crates/contratos`, protocolo, Cargo, CI, stack ou rede; testes determinísticos existentes permanecem válidos; estilo Rust `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100` (**[NV]**).

## Cost of Inaction

Sem o recorte, o BF-003 continua com a mesma pontuação de 270/3125 do ciclo 057, mas sem entrega: o cliente segue com `Nós: 0/3`, a vitória determinística da sim não é observável por quem joga, e toda demonstração local de autogestão exigiria reconstruir o espelho à mão. Pior, a duplicação improvisada (opção B) passaria a ser o caminho de menor resistência, criando divergência entre sim e cliente. O custo é a perda da única fatia barata que fecha BF-003a sem abrir o mapa de 12 ilhas e sem mexer em regra, CI ou protocolo.

## Open Questions

- **[NV]** Confirmar na spec o nome exato da API de súditos em `Mundo` e a ordem de inserção ilhas → súdito → nós no helper.
- **[NV]** Confirmar que as posições dos 3 nós na semente são as de `posicoes_nos()` (`simulacao.rs:2404-2407`) ou outra escolha prototípica equivalente; ambas são `[HIPÓTESE]`, não cânone.
- **[NV]** Confirmar que `(0.0, 236.0)` está dentro dos limites do terreno gerado em `configurar` e que o súdito é serializado em `hash_estado`.
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** Nomes definitivos de SC-001..SC-006 / T001..T012 em `specs/058-mirror-vitoria-local/` (em construção); FR-001..FR-004 já têm mapeamento canônico fixado pela spec.

## Rastreabilidade (escopo do problema)

- FR-001 → helper público `semear_espelho_local` e `POSICOES_NOS_LOCAIS` na sim (4 ilhas ids 1..=4, 1 súdito id 1 em `(0.0, 236.0)`, 3 nós); FR-002 → chamada do helper no `configurar` do cliente; FR-003 → teste de integração `espelho_local_seed_viabiliza_vitoria_e_reflete_hash` (vitória por `propaganda_no` nos 3 nós e `hash_estado` alterado); FR-004 → evidência/validação/release (BF-003a fechado, BF-003b aberto, fronteira intacta e constituição v1.2.2).
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis em `specs/058-mirror-vitoria-local/`, fora desta fronteira.
- **Score: 270/3125** (5×3×3×2×3) — único do ciclo, repetido em `intake.md`, `research.md`, `shape.md` e `decision.md`. R1 `go`, R4 antes de R7 `1.0.0-local`, **R6/taskstoissues N/A** por constituição v1.2.2. Todo número de vitória citado é `[HIPÓTESE]` (GDD:275); nada aqui é cânone, spec, plano ou implementação.
