# Shape — Derrota por timeout e reconquista

- **Slug**: `derrota`
- **Created**: 2026-11-23T14:25Z
- **Recommended option**: Opção A — Derrota por timeout/reconquista no core, reaproveitando `ilha_perdida`

## Options

### Option A — Derrota por timeout/reconquista (recomendada)

- **Sketch**: em `crates/sim/src/lib.rs`, estender `ResultadoPartida` com `Derrota` (ou `DerrotaPorTempo`/`DerrotaPorReconquista`), acrescentar `Mundo.ilhas_reconquistadas: BTreeSet<u64>` inicializado vazio em `new()` e declarar `TICKS_LIMITE_PARTIDA = 36000` e `MAX_RECONQUISTAS = 3` como `[HIPÓTESE]`/protótipo (20 min a 30 Hz, derivado de `PASSO_SEGUNDOS = 1/30`). Em `fase_controle_ilhas`, no mesmo ramo que já faz `sinais.push_back(format!("ilha_perdida:{id}"))`, registrar `ilhas_reconquistadas.insert(id)` — dedup por id, sem reimplementar a perda. Inserir `fase_derrota` imediatamente após `fase_vitoria` em `avancar_tick`: se `resultado.is_some()`, retornar; se `tick >= TICKS_LIMITE_PARTIDA` ou `ilhas_reconquistadas.len() >= MAX_RECONQUISTAS`, gravar `resultado = Some(Derrota)`. Atualizar `hash_estado` (o `match` sobre `ResultadoPartida` é exaustivo) com o conjunto e a variante em ordem estável, preservando a ordem das fases e o `tick += 1`. No cliente, um braço em `atualizar_texto` passa a exibir "Comuna dissolvida" no campo `Resultado` — sem novas teclas.
- **Appetite**: `small` (um a poucos dias) — um `enum`, um campo, duas constantes, uma fase, um `insert`, um braço de hash e um braço de HUD; sem mapa, geometria, IA, contrato, protocolo, Cargo, stack ou rede.
- **Trade-offs**: ganha o desfecho perdedor determinístico que fecha o ciclo iniciado pela vitória 050, reaproveitando o sinal `ilha_perdida` e o contador de ilhas — ao custo de um estado novo no hash e de uma fase a mais no tick. Impacto **4** (completa a condição final da partida, mas reusa infraestrutura existente em vez de abrir regra nova), confiança **3** (a regra está no GDD e o sinal já é emitido, porém todos os números são `[HIPÓTESE]` e a HUD permanece **[NV]** sem surface automatizável), facilidade **4** (mudança pequena e localizada, sem contrato novo) e risco invertido **4** (determinismo preservado, dedup por id e vitória prevalecendo no mesmo tick; não 5 porque a ordem das fases e o `hash_estado` são pontos sensíveis de replay e os rótulos de protótipo precisam ser mantidos).
- **Rabbit holes**: transformar a derrota em simulação da Corporação (ataque inimigo, tropas, geometria de reconquista); reabrir Nós/Resistência/pulso do ciclo 050; contar reconquistas pela fila `sinais` em vez do `BTreeSet` (a fila é FIFO, não histórico); trocar `tick` por relógio de parede (`Instant`/`SystemTime`) e quebrar o determinismo; esquecer o `match` exaustivo de `hash_estado`; promover 36000 ticks/3 reconquistas a cânone; prometer tela de fim de partida, animação ou nova tecla.

### Option B — Papéis W/E/R + níveis

- **Sketch**: introduzir os papéis Artesão–Guerreiro–Intelectual com progressão por níveis e habilidades associadas.
- **Appetite**: `large` (semanas) — exige economia de recursos, progressão, rebalanceamento e superfície de HUD/input.
- **Trade-offs**: impacto 4, mas confiança 2 e facilidade 2 (papéis, níveis e custos ainda `[HIPÓTESE]`, sem fechamento de regras), risco invertido 3 → **48/625**. Não fecha o ciclo de partida nem reaproveita o sinal `ilha_perdida`.
- **Rabbit holes**: progressão, árvore de habilidades, custos de promoção, sinergia com Fôlego e combate, reequilíbrio do ciclo 047.

### Option C — PVE/PNJs

- **Sketch**: povoar o mundo com PNJs e inimigos controlados pela sim para dar pressão sobre as ilhas.
- **Appetite**: `large` (semanas) — IA determinística, spawn, comportamento e verificação de longo prazo; GDD §16.3 prefere desafios de ilha a "combate puro".
- **Trade-offs**: impacto 4, confiança 2, facilidade 2, risco invertido 3 → **48/625**. Mesmo valor de B, com escopo ainda maior e sem endereçar o desfecho de derrota.
- **Rabbit holes**: IA, pathfinding, spawn remoto, ciclos de ataque, equilíbrio de dificuldade, replay com aleatoriedade.

### Option D — Rede / Lightyear primeira

- **Sketch**: postergar a verificação local até transporte, predição e reconciliação cliente-servidor estarem prontos.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de completude de partida para infraestrutura de rede.
- **Trade-offs**: impacto 5, mas facilidade 1 e risco invertido 1 → **10/625**; ADR-004 bloqueia a troca até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`). Sincronizaria um desfecho cuja verificação local ainda não foi fechada.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), predição de desfecho, resolução de concorrência de comandos, troca de engine sem novo ADR.

## Recommendation

Escolher **Opção A**. É a única que satisfaz as metas de `define.md` com fronteira mínima: impacto **4** (a partida passa a terminar no lado perdedor), confiança **3** (regra e sinal existem, números `[HIPÓTESE]` e HUD **[NV]**), facilidade **4** (um enum, um conjunto, duas constantes, uma fase, um `insert`, um braço de hash e um braço de HUD) e risco invertido **4** (determinismo, dedup e precedência preservados; atenção à ordem de fases e ao `hash_estado`). B e C (48/625 cada) são frentes de design maiores e não endereçam o desfecho; D (10/625) inverte a dependência verificabilidade → sincronização e está bloqueada por ADR-004. Scorecard único do ciclo, com fonte em `.specify/memory/STATE.md` (`2026-11-23T14:25Z`): **192/625** contra 48/625 (B), 48/625 (C) e 10/625 (D).

Critério do scorecard — `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625** (5⁴), quarto fator = risco invertido (5 = risco mínimo, 1 = risco máximo). Valores: derrota 192/625 (4×3×4×4), papéis 48/625, PVE 48/625, rede 10/625 — os mesmos de `intake.md` e `decision.md`, sem pontuação paralela.

A recomendação **não** reabre a vitória do ciclo 050 nem promove números a cânone: 20 min = 36000 ticks a 30 Hz, 3 reconquistas e os 20 s de §9.4 continuam `[HIPÓTESE]`/protótipo. A precedência vitória-sobre-derrota no mesmo tick é obtida pela ordem `fase_vitoria → fase_derrota` e pela guarda `resultado.is_some()`, não por nova regra.

## Out of Scope (for the recommended option)

- Reabertura da vitória 050 (Nós, Resistência, −30/−10/−20, 600 ticks, "Autogestão estabelecida") e promoção de qualquer número `[HIPÓTESE]` a cânone (GDD §6.2:277 e §9.4:501 permanecem rótulos de protótipo).
- Mapa, geometria, rotas, Plano Mestre, posições de ilhas, IA da Corporação, ataque inimigo, novas regras de perda de ilha, alteração da fila `sinais` ou dos 10 s/20 s de §9.4.
- Papéis W/E/R + níveis, PVE/PNJs e qualquer frente que ficou em 48/625; netcode, lobby, matchmaking, `async`, sockets e Lightyear (ADR-004, 10/625).
- Tela de fim de partida, animação, som, picking ou novas teclas — a HUD `Resultado` já existe e recebe apenas um braço de texto.
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`; **R6** (N/A por constituição v1.2.2), issues simuladas e `gh` improvisado; gates, build, testes e formatadores neste assessment.

## Assumptions to Validate

- `ResultadoPartida` admite a nova variante sem quebrar consumidores: o `match` exaustivo em `hash_estado` (`Some(Vitoria)`/`None`) precisa ser atualizado na mesma fatia; o cliente usa `matches!`, que não é exaustivo, e basta um braço de texto.
- `ilhas_reconquistadas.insert(id)` no ponto de emissão de `ilha_perdida:{id}` em `fase_controle_ilhas` é suficiente — a ilha pode perder o sinal em ticks diferentes; o `BTreeSet<u64>` garante dedup por id, e a fila `sinais` não é fonte do contador.
- `tick >= TICKS_LIMITE_PARTIDA` com `TICKS_LIMITE_PARTIDA = 36000` é a tradução correta de 20 min a 30 Hz (`PASSO_SEGUNDOS = 1/30`); [`[HIPÓTESE]`] e `tick` é `i64`, sem risco de estouro nesse limite.
- Inserir `fase_derrota` após `fase_vitoria` e antes de `fase_finish` não altera o replay: ordem das fases existentes e `tick += 1` permanecem, e a guarda `resultado.is_some()` faz a vitória prevalecer no mesmo tick.
- Codificar `ilhas_reconquistadas` e a nova variante em `hash_estado` em ordem estável mantém estados diferentes com hashes diferentes; omissão geraria colisão e divergência de replay.
- A HUD lê `resultado` do espelho `Mundo` e exibe "Comuna dissolvida" sem nova tecla; o efeito renderizado é **[NV]** sem surface automatizável de captura — não prometer screenshot real.
- Nenhuma mudança em contratos, protocolo, Cargo, stack, rede ou mapa; estilo Rust `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; B, C e D são as alternativas de `research.md` e não recebem rastro de implementação neste ciclo. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica prevista em `specs/051-derrota/` (template no momento da leitura). Todo número citado é `[HIPÓTESE]`/`escolha de protótipo` do GDD/ciclo 050; nada aqui é cânone. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa.
