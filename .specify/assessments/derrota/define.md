# Define — Derrota por timeout e reconquista

- **Slug**: `derrota`
- **Created**: 2026-11-23T14:25Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §6.2:277 e §9.4:489,501; `crates/sim/src/lib.rs`; `crates/cliente/src/main.rs`; STATE.md `2026-11-23T14:25Z`; assessment/release 050; Constituição v1.2.2)

- **Scorecard do ciclo** (único; fonte `.specify/memory/STATE.md`, `2026-11-23T14:25Z`): derrota **192/625** (4×3×4×4), papéis W/E/R 48/625, PVE/PNJs 48/625, rede 10/625; critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625** (5⁴).

## Problem Statement

O GDD descreve a derrota desde o início do projeto — §6.2:277, como `[HIPÓTESE]`, a Comuna é dissolvida se o relógio de 20 min se esgotar sem a conversão ou se a Corporação reconquistar 3 ilhas já libertadas, e §9.4:489,501 já garante o sinal `ilha_perdida` quando a ilha volta a Latente — mas o protótipo ainda não tem esse desfecho. `ResultadoPartida` só contém `Vitoria`, `Mundo` não acumula reconquistas, `avancar_tick` não tem fase de derrota, `hash_estado` só codifica `"vitoria"`/`"nenhum"` e a HUD, embora já tenha um campo `Resultado`, só sabe dizer "Autogestão estabelecida" ou "em curso". O ciclo 050 entregou a vitória como estado determinístico e deixou a derrota explicitamente fora de escopo; sem ela, a partida não termina no lado perdedor e a mensagem "Comuna dissolvida" do §6.2 permanece letra morta. O recorte fecha esse ciclo **reaproveitando o sinal e o contador de ilhas existentes**, com um único ponto de decisão e nenhuma frente de mapa, regra de combate, contrato, protocolo, Cargo, stack, rede ou nova tecla. Os números (20 min = 36000 ticks a 30 Hz; 3 reconquistas) são **`[HIPÓTESE]`/escolha de protótipo** e devem ser rotulados como tal em spec, HUD e comentários — nunca promovidos a cânone.

## Affected Users & Stakeholders

- **Users**: jogadores numa partida — precisam que o tempo esgotado ou a terceira reconquista encerre a partida com "Comuna dissolvida", em vez de uma simulação que continua indefinidamente após a derrota; quem lê a HUD — precisa ver o desfecho no campo `Resultado` já existente, sem nova tecla ou tela.
- **Stakeholders**: mantenedores da sim — precisam de um `enum`, um `BTreeSet`, duas constantes, uma fase, um gancho no sinal `ilha_perdida` e um braço no hash, preservando a ordem das fases e o incremento de `tick`; mantenedores do cliente — precisam de apenas um braço de texto na HUD, reaproveitando `Resultado`; designer — precisa que os números `[HIPÓTESE]` continuem rotulados como protótipo e que a regra de vitória do ciclo 050 não seja reaberta; SWE/orquestrador — precisa de uma fatia verificável por inspeção de código e execução determinística, com R1 `go`, R4 antes de R7 `1.0.0-local` e R6 N/A.

## Goals

- Declarar `ResultadoPartida::Derrota` (ou as variantes `DerrotaPorTempo`/`DerrotaPorReconquista`) ao lado de `Vitoria` em `crates/sim/src/lib.rs`, documentando os números envolvidos como **`[HIPÓTESE]`/protótipo**.
- Acrescentar `Mundo.ilhas_reconquistadas: BTreeSet<u64>` inicializado vazio em `new()`, com dedup por id de ilha.
- Declarar `TICKS_LIMITE_PARTIDA = 36000` (20 min × 30 Hz, derivação `[HIPÓTESE]` de `PASSO_SEGUNDOS = 1/30`) e `MAX_RECONQUISTAS = 3` como constantes de protótipo.
- registrar, no ponto em que `fase_controle_ilhas` emite `ilha_perdida:<id>`, o id em `ilhas_reconquistadas` — sem reimplementar a perda de ilha e sem depender da fila `sinais`, que é FIFO e não histórico.
- Inserir `fase_derrota` imediatamente após `fase_vitoria` em `avancar_tick`: se `resultado.is_some()`, nada; se `tick >= TICKS_LIMITE_PARTIDA`, gravar Derrota; se `ilhas_reconquistadas.len() >= MAX_RECONQUISTAS`, gravar Derrota. Vitória prevalece se ocorrer no mesmo tick.
- Incluir `ilhas_reconquistadas` e a nova variante em `hash_estado`, em ordem estável, atualizando o `match` hoje exaustivo sobre `ResultadoPartida` e preservando a ordem das fases e o `tick += 1`.
- Fazer a HUD exibir "Comuna dissolvida" no campo `Resultado` quando houver derrota, mantendo "Autogestão estabelecida" para vitória e "em curso" para `None`; **sem novas teclas, eventos ou telas**.
- Manter a fronteira: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack, rede ou mapa; rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012 e constituição v1.2.2 (R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**).

## Non-Goals

- Reabrir a vitória do ciclo 050: Nós, Resistência, −30/Nó, pulso −10, teto −20 com piso 10, 600 ticks de Propaganda/inatividade e a mensagem "Autogestão estabelecida" permanecem `[HIPÓTESE]`/escolha de protótipo e intocados.
- Promover 20 min/36000 ticks, 3 reconquistas ou os 20 s de reconversão de §9.4 a cânone; alterar as regras de perda de ilha, o tempo de defesa de 10 s ou a fila `sinais`.
- Mapa, geometria, rotas, Plano Mestre, posições de ilhas ou simulação da Corporação como agente: a reconquista é inferida do sinal `ilha_perdida`, não de um novo ataque inimigo.
- Papéis W/E/R + níveis, PVE/PNJs, IA, spawn remoto, combate novo ou qualquer frente que ficou em 48/625 no scorecard.
- Netcode, lobby, matchmaking, predição/reconciliação, `async`, sockets ou Lightyear (ADR-004); rede ficou em 10/625 no scorecard.
- Tela de fim de partida, animação, som, picking ou novas teclas — a HUD `Resultado` já existe e basta um braço de texto.
- Executar gates, build, testes, formatadores, commit, push ou PR neste assessment; adotar R6 ou simular emissão de issues.

## Success Metrics

- Com `resultado == None`, `avancar_tick` repetido até `tick >= 36000` (`[HIPÓTESE]`: 20 min a 30 Hz) grava `resultado = Some(Derrota)`; antes disso, nenhuma derrota é gravada (baseline: `resultado` nunca muda por tempo — **[NV]** até verificação).
- Cada `ilha_perdida:<id>` emitido em `fase_controle_ilhas` acrescenta `id` a `ilhas_reconquistadas` **uma única vez**; com 3 ids distintos (`[HIPÓTESE]`), `avancar_tick` grava Derrota mesmo que a Resistência da Ilha Central esteja abaixo de 50, conforme §6.2 (baseline: ids descartados — **[NV]**).
- Se vitória e derrota ocorrerem no mesmo tick, `resultado` permanece `Vitoria`; a guarda `resultado.is_some()` em `fase_derrota` e a ordem `fase_vitoria → fase_derrota` são a prova estrutural (baseline: sem fase de derrota — **[NV]**).
- O mesmo id perdido em ticks diferentes não aumenta o contador (`BTreeSet<u64>`), e nenhuma reconquista é contada sem o sinal `ilha_perdida` correspondente (baseline: sem conjunto — **[NV]**).
- `hash_estado` inclui `ilhas_reconquistadas` e a variante de derrota em ordem estável: estados que diferem só no desfecho ou no conjunto produzem hashes diferentes; replay determinístico existente permanece válido (baseline: `match` só cobre `Vitoria`/`None` — **[NV]**).
- A HUD exibe `Resultado: Comuna dissolvida` na derrota, `Autogestão estabelecida` na vitória e `em curso` em `None`, sem nova tecla; nenhuma mudança em contratos, protocolo, Cargo, stack ou rede (baseline: só vitória/"em curso" — **[NV]**).
- Estilo Rust preservado: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100` (baseline: **[NV]** até verificação).

## Cost of Inaction

Sem o recorte, a partida iniciada com a vitória do ciclo 050 permanece incompleta: o relógio e as reconquistas não têm efeito, a mensagem "Comuna dissolvida" do §6.2 nunca aparece e o `Resultado` da HUD fica preso em "em curso" para qualquer sessão que não converta os 3 Nós. Qualquer ciclo futuro que queira sessões com limite de tempo, defesa de ilhas ou replay de partidas longas teria de reconstruir o contador, a fase e o braço de hash — além de reintroduzir a decisão de precedência entre vitória e derrota. O custo é a perda da única maneira barata de fechar o ciclo de partida sem mapa, IA, contrato, protocolo, stack ou rede, e com o sinal `ilha_perdida` já emitido e determinístico.

## Open Questions

- Nenhuma clarificação de design pendente no contrato: a regra está no GDD e o contrato fixa o desenho (R3 = 0 clarificações).
- **[NV]** Escolher entre variante única `Derrota` e variantes `DerrotaPorTempo`/`DerrotaPorReconquista`: o contrato admite ambas; a decisão afeta o texto do hash e da HUD, não a regra de precedência.
- **[NV]** Confirmar que `BTreeSet<u64>` dedup por id é a semântica desejada quando a mesma ilha é perdida, reconquistada e perdida de novo — o contrato pede dedup por id, portanto uma ilha conta uma vez.
- **[NV]** Confirmar a precedência "vitória prevalece no mesmo tick" com o gancho escolhido (`fase_derrota` após `fase_vitoria`, guarda `resultado.is_some()`) no roteiro determinístico.
- **[NV]** Confirmar a codificação do novo estado em `hash_estado` (ordem estável, `match` exaustivo atualizado) e a ausência de divergência de replay.
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** Os nomes definitivos de T001..T012 são os de `specs/051-derrota/tasks.md`, ainda em construção pelo `/speckit.specify`.

## Rastreabilidade (escopo do problema)

- FR-001 → variante `ResultadoPartida::Derrota` (ou `DerrotaPorTempo`/`DerrotaPorReconquista`) e constantes `TICKS_LIMITE_PARTIDA = 36000` / `MAX_RECONQUISTAS = 3`, rotuladas `[HIPÓTESE]`; FR-002 → `Mundo.ilhas_reconquistadas: BTreeSet<u64>` inicializado vazio e registro do id em `fase_controle_ilhas` no ponto do `ilha_perdida:<id>` (dedup por id); FR-003 → `fase_derrota` após `fase_vitoria` com guarda `resultado.is_some()`, relógio `tick >= 36000` e contagem `len() >= 3`, vitória prevalecendo no mesmo tick; FR-004 → precedência e ausência de dupla contagem, reaproveitando sinal e contador de ilhas sem reabrir §9.4; FR-005 → `hash_estado` com o novo conjunto e a nova variante em ordem estável e HUD `Resultado` com "Comuna dissolvida", sem novas teclas; FR-006 → fronteira (contratos/protocolo/Cargo/stack/rede/mapa intactos), rastros e constituição v1.2.2.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis especificados em `specs/051-derrota/`, fora desta fronteira.
- R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A** por constituição v1.2.2. Todo número citado (20 min = 36000 ticks a 30 Hz, 3 reconquistas, 20 s de §9.4 e os números da vitória 050) é `[HIPÓTESE]`/`escolha de protótipo`; nada aqui é cânone.
