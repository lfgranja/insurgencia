# Decision — Sincronia cooperativa observável

- **Slug**: `sincronia-cooperativa`
- **Decided**: 2026-11-25
- **Ciclo**: 060 · **Pendência**: BF-015a
- **Score**: **360/3125** (`5×4×3×2×3`)
- **R1**: `go` · **R3**: 0 clarificações · **R6/taskstoissues**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | `sinalizar_sincronia` está na whitelist (`lib.rs:217-231`), dispensa `destino` (`:258,262`) e passa no nível 1 (`:242-248`), mas `fase_skills` (`:875-966`) não tem ramo: o comando só avança monotônicos. A dica da HUD (`main.rs:1362`) nem menciona `Espaço`. |
| Força da evidência | strong | Código lido no checkout (whitelist, validador `:2414-2475`, `fase_skills`, dica), GDD §8.1/§8.3/§12.2 lido (`349-416`, `921`, `937`, `943-950`) e `STATE.md:594-604` com score e DoD; sem suposição de design novo. |
| Valor vs. inação | strong | **360/3125** contra 288/3125 (BF-002c) e 144/3125 (BF-015b); inação mantém a regra de Ação Coletiva sem recipiente no protótipo e BF-015a aberto. |
| Viabilidade / appetite | strong | `small`/`medium`: estado + janela + hash na sim, tecla/par local/HUD no cliente e testes; nenhuma mudança em GDD, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede. |
| Alinhamento estratégico | strong | Respeita a intenção de Ação Coletiva do GDD sem promover 150 ticks/3 peers a cânone e sem decidir topologia de ilhas; preserva R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY e `converge` append-only. |
| Postura de risco | strong | Dedup por `BTreeSet<u64>`, janela single-flight, sinal fora da janela ignorado e `hash_estado` cobrindo os campos novos; fronteira intacta; **R6/taskstoissues N/A**, sem issue simulada nem `gh` improvisado; **PR #76 não é tocado**. |

## 80/20 (scorecard único do ciclo)

| Candidato | Impacto × Confiança × Fechamento × Esforço⁻¹ × Risco⁻¹ ÷ 3125 | Resultado |
|:--|:--|:--|
| `sinalizar_sincronia` (BF-015a, escolhido) | 5×4×3×2×3 = **360/3125** | **360/3125** |
| Ato de Vandalismo (BF-002c) | 4×4×2×3×3 = 288/3125 | 288/3125 |
| `sinalizar_alavanca` (BF-015b) | 4×3×3×2×2 = 144/3125 | 144/3125 |

Critério do contrato — impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, escala 1–5, denominador **5⁵ = 3125**; o quarto e o quinto fatores são inversos de esforço e de risco (5 = menor esforço/risco, 1 = maior). Filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo. Impacto **5** (Ação Coletiva observável pela primeira vez), confiança **4** (autoridade lida no código; janela de 150 ticks é `[HIPÓTESE]`), fechamento **3** (consumidor e comandos de prova nomeados; spec e testes por escrever), esforço⁻¹ **2** (sim + cliente + CLI + hash), risco⁻¹ **3** (fronteira preservada, mas novos campos públicos e hash exigem determinismo). Rejeitados sem pontuação paralela: BF-003b (topologia canônica), BF-001/006/007/009/010 (autorização), BF-004/005 (execução externa), BF-013/014 (diagnóstico). Este é o único scorecard do assessment; os mesmos valores aparecem em `intake.md`, `research.md` e `shape.md`, sem pontuação paralela.

## DoD do ciclo (referência única; repetido nos cinco artefatos)

- **DoD-1 (janela e estado)**: primeiro `sinalizar_sincronia` com `sincronia_ticks_restantes == 0` abre a janela de `TICKS_JANELA_SINCRONIA = 150` ticks (5 s × 30 Hz); o terceiro `peer_id` distinto na janela incrementa `sincronias_iniciadas`, empurra `sincronia_iniciada:<tick>` em `sinais` e reinicia janela/conjunto; sinal fora da janela não conta e não corrompe estado; `hash_estado` cobre contador, ticks restantes e IDs ordenados.
- **DoD-2 (consumidor)**: jogador do cliente Bevy (tecla `Espaço`, `seq`/`request_id` monotônicos e terceiro par local semeado em `configurar`) e automação headless via CLI `insurgencia-servidor`.
- **DoD-3 (prova)**: `cargo test --locked -p insurgencia-sim --test simulacao`; `cargo test --locked -p insurgencia-servidor --test cli`; `cargo test --locked -p insurgencia-cliente --lib`; `cargo check --locked -p insurgencia-cliente --all-targets`; `./verify.sh` remoto (`gates`/`rust`).
- **DoD-4 (fronteira/backlog)**: sem topologia de ilhas, dano de gás, promoção de número a cânone ou mudança em GDD/`crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`/CI/stack/rede; **BF-015a passa a `fechado`**; BF-015b e BF-002c permanecem abertos.

## Veredito e justificativa

**Veredito: go** — Opção A (`sinalizar_sincronia` com janela determinística, BF-015a), scorecard 6/6 strong, **360/3125** (`5×4×3×2×3`), R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, **R6/taskstoissues N/A**. **Consumidor: jogador do cliente Bevy (tecla `Espaço`) + automação headless via CLI `insurgencia-servidor`.** O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas, os rastros **FR-001..FR-004** e o **DoD-1..DoD-4** reproduzido acima. **PR #76 (`dev→main`) pertence a outro owner ativo, está verde mas `dirty`, e não é tocado**; `dev→main` e deploy são exclusivos da SE. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. **Constante e campos**: `pub const TICKS_JANELA_SINCRONIA: u64 = 150` e os campos públicos `sincronias_iniciadas: u64`, `sinalizadores_sincronia: BTreeSet<u64>` e `sincronia_ticks_restantes: u64` em `Mundo`, zerados/vazios em `Mundo::new`; valores são `[HIPÓTESE]` do GDD §8/§12, não cânone.
2. **Janela e contagem**: `avancar_tick` decrementa a janela uma vez por tick; o primeiro sinal com janela zerada a abre; três `peer_id` distintos na mesma janela incrementam `sincronias_iniciadas`, empurram `sincronia_iniciada:<tick>` em `sinais` e reiniciam janela/conjunto; sinal fora da janela não conta e não corrompe estado.
3. **Determinismo**: `hash_estado` cobre `sincronias_iniciadas`, `sincronia_ticks_restantes` e os IDs dos sinalizadores em ordem ordenada, preservando replay entre cliente e CLI.
4. **Cliente**: `Espaço` agenda `sinalizar_sincronia` com `seq`/`request_id` monotônicos (um comando por pressão); `configurar` semeia o terceiro par local como scaffolding determinístico; a HUD exibe `Sincronias: N` lida de `Mundo.sincronias_iniciadas` e a dica de teclado passa a anunciar `Espaço`.
5. **Regressão**: testes cobrem abertura da janela, terceiro peer distinto, sinal fora da janela, signal emitido, `hash_estado` e o caminho CLI — **FR-003**.
6. **Fronteira/backlog**: nada em GDD/cânone, topologia de ilhas, gás/expulsão, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede; **BF-015a passa a `fechado`**, BF-015b e BF-002c permanecem abertos; estilo Rust `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.
7. **Constituição v1.2.2**: R1 `go`, R3 = 0 clarificações, R4 antes de R7 (`1.0.0-local`); **R6/taskstoissues N/A** — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado. **PR #76 (`dev→main`) pertence a outro owner ativo e não é tocado**; `dev→main` e deploy são exclusivos da SE.

## Riscos registrados

- **Janela/dupla contagem**: mitigadas por janela single-flight, `BTreeSet<u64>` para peers distintos e reset ao atingir três; o comportamento exato no tick 150 fica **[NV]** até verificação.
- **Sinal fora da janela**: deve ser ignorado sem corromper contador, conjunto ou `sinais`; nenhuma tolerância extra é acrescentada.
- **Determinismo/hash**: novos campos entram em `hash_estado` com IDs ordenados; sem isso, replay e comparação cliente/CLI divergiriam. Cobertura por testes (FR-003), **[NV]** até execução.
- **Canonização indevida**: 150 ticks / 3 peers vêm de GDD §8/§12 como `[HIPÓTESE]`; nenhum número é promovido a cânone e nenhuma topologia de ilha é decidida. Gás, expulsão e barra "0/3 → 3/3" permanecem intenção de design, fora do ciclo.
- **Dívida R6 / PR #76**: R6 é N/A sob v1.2.2 (sem adaptador de issues); nenhuma issue simulada nem `gh` improvisado. PR #76 está verde, mas `dirty`, é de outro owner e é intocável; promoção `dev→main` e deploy são SWE-exclusivos.
- **Verificação**: `./verify.sh` (jobs `gates`/`rust`), `cargo test`/`cargo check` e captura visual da HUD permanecem **[NV]** — nenhum foi executado por este agente. Âncora do ciclo: `origin/dev = e0e8f38`, cinco gates PASS no início.

## Handoff — `/speckit.specify`

- **Problem**: `sinalizar_sincronia` é aceito pelo validador, mas descartado em `fase_skills`; o cliente não tem tecla nem contador, então "três em cinco segundos" não é observável no protótipo.
- **Chosen approach**: Opção A — janela determinística de 150 ticks no `Mundo` com contador, conjunto de sinalizadores, signal FIFO e hash; cliente com `Espaço`, ids monotônicos, terceiro par local e HUD `Sincronias: N`. Scaffolding **não-canônico**.
- **In scope**: `TICKS_JANELA_SINCRONIA`, campos de `Mundo`, decremento em `avancar_tick`, emissão de `sincronia_iniciada:<tick>`, `hash_estado`, tecla/par/HUD no cliente, testes de regressão e rastros. **Out of scope**: ilhas/topologia/gás/expulsão, BF-002c, BF-015b, BF-003b, rede/ADR-004, contratos/protocolo/Cargo/CI/stack, promoção `dev→main`/deploy, **R6 (N/A)**, PR #76 e qualquer execução neste assessment.
- **Consumidor**: jogador do cliente Bevy (tecla `Espaço`) + automação headless via CLI `insurgencia-servidor`.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Comandos de prova** (`STATE.md:602`, não executados aqui): `cargo test --locked -p insurgencia-sim --test simulacao`; `cargo test --locked -p insurgencia-servidor --test cli`; `cargo test --locked -p insurgencia-cliente --lib`; `cargo check --locked -p insurgencia-cliente --all-targets`; `./verify.sh` remoto (`gates`/`rust`).
- **Carried-forward open questions**: fronteira exata do tick 150, deduplicação do terceiro peer, formato do comando CLI para três peers e efeito da HUD com janela minimizada — validar na especificação; PR #76 permanece intocável.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | sim: `TICKS_JANELA_SINCRONIA = 150`, campos `sincronias_iniciadas`/`sinalizadores_sincronia`/`sincronia_ticks_restantes`, decremento em `avancar_tick`, `sincronia_iniciada:<tick>` em `sinais` e cobertura do `hash_estado` | contrato da sessão; `crates/sim/src/lib.rs:875-966`; `intake.md` · `shape.md` |
| FR-002 | cliente: tecla `Espaço` agendando `sinalizar_sincronia` com ids monotônicos, terceiro par local em `configurar` e HUD `Sincronias: N` | `crates/cliente/src/main.rs:1362`; contrato; `define.md` |
| FR-003 | testes de regressão: abertura da janela, terceiro peer distinto, sinal fora da janela, signal, hash e CLI | contrato; `STATE.md:602`; `shape.md` · Assumptions |
| FR-004 | evidência/validação/release; comandos de prova e **[NV]** neste assessment | `STATE.md:602`; `research.md` · `decision.md` |
| SC-001 | primeiro sinal abre janela de 150 ticks; cada `avancar_tick` decrementa 1 | `define.md` · Success Metrics 1 |
| SC-002 | três peers distintos na janela incrementam contador, emitem signal e reiniciam janela/conjunto | `define.md` · Success Metrics 2 |
| SC-003 | sinal fora da janela não conta e não corrompe estado | `define.md` · Success Metrics 3 |
| SC-004 | `hash_estado` muda com contador/ticks/IDs ordenados | `define.md` · Success Metrics 4 |
| SC-005 | cliente: `Espaço` com ids monotônicos, terceiro par local e HUD `Sincronias: N` | `define.md` · Success Metrics 5 |
| SC-006 | fronteira intacta, estilo Rust e cobertura determinística preservada | `define.md` · Success Metrics 6 · **[NV]** |
| T001 | constante `TICKS_JANELA_SINCRONIA = 150` documentada como `[HIPÓTESE]` | `shape.md` · Opção A |
| T002 | campos públicos de `Mundo` zerados/vazios em `Mundo::new` | `shape.md` · Opção A |
| T003 | decremento da janela em `avancar_tick` | `shape.md` · Opção A |
| T004 | abertura da janela no primeiro sinal e dedup de peers no conjunto | `shape.md` · Assumptions |
| T005 | terceiro peer distinto incrementa contador e emite `sincronia_iniciada:<tick>` | `shape.md` · Opção A |
| T006 | reset de janela/conjunto após o terceiro sinal | `shape.md` · Opção A |
| T007 | `hash_estado` cobrindo contador, ticks e IDs ordenados | `shape.md` · Assumptions |
| T008 | tecla `Espaço` no cliente com ids monotônicos e terceiro par local em `configurar` | `shape.md` · Opção A |
| T009 | HUD `Sincronias: N` e dica atualizada | `shape.md` · Assumptions |
| T010 | testes de regressão sim/CLI/cliente e rastros/governança: R1 `go`, R3 = 0, R4 antes de R7, **R6 N/A**, PR #76 intocável, não-execução | contrato; `intake.md` |

Os nomes definitivos de T001..T010 são os da spec 060; a coluna "Eco" apenas rastreia a cobertura deste assessment. **R6/taskstoissues é N/A** por constituição v1.2.2 e não recebe tarefa; PR #76 não é tocado. Nada aqui é especificação, plano, tarefa, implementação ou evidência de execução.

Veredito: go
