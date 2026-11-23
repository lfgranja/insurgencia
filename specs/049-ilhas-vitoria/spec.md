# Feature Specification: Controle de ilha por Súdito no cliente standalone

**Branch**: `049-ilhas-vitoria` (base `42b5f7b`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T111200Z-controle-ilha`, `insurgencia - controle de ilha por sudito`.

**Âncora**: decisão 80/20 do ciclo 049 registrada em `.specify/memory/STATE.md` (2026-11-23T11:15Z): **Controle de ilha por Súdito (10 s contínuos) = 4×3×3×3 = 108/625**, escolhida sobre Nós de Controle + vitória completa **4×2×2×3 = 48/625**, Papéis W/E/R + níveis **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. A regra canônica é GDD §9.4:489,501: controlar uma ilha exige superar o Desafio de Ilha **e** manter ao menos 1 Súdito da Comuna na ilha por **10 s contínuos**; ao atingir os 300 ticks, a ilha emite **`ilha_convertida:<id>`** uma única vez; sem o Súdito, volta a **Latente** e emite **`ilha_perdida:<id>`**, inclusive quando há progresso parcial perdido. O repertório de sinais é o da tabela §9.1 (`ilha_bloqueada`, `terreno_preparado`, `desafio_iniciado`, `contestacao`, `ilha_convertida`, `ilha_perdida`); o sufixo `:<id>` identifica a ilha na fila determinística e é convenção do protótipo. **Protótipo sem mapa**: na ausência de posições por ilha, a presença de **qualquer Súdito convertido** é tratada como **global**; esta simplificação é registrada explicitamente e não é promovida a cânone. Nós de Controle, Resistência Ideológica, papéis W/E/R, níveis e vitória completa ficam para ciclos seguintes.

## User Scenarios

### US1 — Regra canônica de 10 s e simplificação rotulada (P1)

O cliente passa a representar a regra canônica de GDD §9.4:489,501 com o passo fixo já existente da sim: `PASSO_SEGUNDOS = 1 / 30` implica **300 ticks contínuos** de presença de Súdito da Comuna para controlar uma ilha; ao atingir 300 ticks, a ilha emite **`ilha_convertida:<id>`** uma única vez; a ausência devolve a ilha à condição **Latente** e emite **`ilha_perdida:<id>`**, inclusive quando se perde progresso parcial (sinais definidos na tabela de §9.1, com sufixo `:<id>` de protótipo). Como o protótipo não tem mapa nem posições por ilha, a presença considerada é a de **qualquer Súdito com `convertido == true`** no `Mundo`, tratada como global; nenhum raio, coordenada, rota, desafio de ilha ou reconversão da Corporação é introduzido. A regra de 10 s é cânone; o mapeamento para 300 ticks e a presença global são **simplificações de protótipo explicitamente registradas**, nunca regra de produto nova.

**Why this priority**: é o comportamento canônico ainda ausente — os ciclos 044–047 cobriram Defesa, Ponte, Coleta, Súditos e Combate/Fôlego, mas a sim não possui nenhum campo de ilha nem progresso de controle; sem a regra de 10 s não existe condição de vitória observável.

**Independent Test**: ler GDD §9.4:489,501 e a tabela de §9.1, depois inspecionar a spec e o código: os números 10 s/300 ticks e a presença global aparecem rotulados como protótipo; nenhuma regra canônica é alterada e nenhum marcador de mapa é criado.

**Acceptance Scenarios**:
1. **Given** um Súdito convertido presente por 299 ticks contínuos, **When** o tick 299 é processado, **Then** a ilha ainda não está controlada e nenhum sinal `ilha_convertida:<id>` ou `ilha_perdida:<id>` foi enfileirado.
2. **Given** o mesmo Súdito convertido no tick 300, **When** o tick é processado, **Then** a ilha passa a `controlada = true` — equivalente aos 10 s contínuos de §9.4 — e `ilha_convertida:<id>` é enfileirado uma única vez.
3. **Given** uma ilha controlada (ou com progresso parcial), **When** nenhum Súdito convertido está presente em um tick, **Then** ela retorna a Latente (`controlada = false`, `controle_ticks = 0`) e `ilha_perdida:<id>` é enfileirado uma vez.

### US2 — Modelo `Ilha` e registro determinístico (P1)

`Mundo` passa a ter `ilhas: BTreeMap<u64, Ilha>`, com `Ilha { id: u64, controlada: bool, controle_ticks: i64 }`, e o método `adicionar_ilha(id)` insere uma ilha zerada (`controlada = false`, `controle_ticks = 0`). `BTreeMap` preserva a ordem estável por `id`, requisito do determinismo do `hash_estado`; `adicionar_ilha` é a única forma de criar ilhas e não depende de relógio, rede, aleatoriedade ou mapa. Duas ilhas evoluem de forma independente, com progressos próprios, sob a mesma presença global de Súdito convertido.

**Why this priority**: é a menor superfície de estado que torna a regra verificável — sem `Ilha`, `Mundo.ilhas` e `adicionar_ilha`, a fase de controle não tem onde acumular progresso nem como ser serializada no hash.

**Independent Test**: inspecionar a definição de `Ilha`, o campo `Mundo.ilhas`, o construtor `Mundo::new` e `adicionar_ilha(id)`; confirmar estado inicial zerado, ordem `BTreeMap` e ausência de nova dependência.

**Acceptance Scenarios**:
1. **Given** um `Mundo` recém-criado, **When** `adicionar_ilha(7)` é chamado, **Then** `ilhas[&7]` existe com `controlada = false` e `controle_ticks = 0`.
2. **Given** as ilhas `2` e `7`, **When** o hash é calculado, **Then** a serialização segue a ordem crescente de `id`, igual entre execuções com o mesmo input.
3. **Given** `adicionar_ilha(7)` repetido, **When** o segundo chamado é processado, **Then** a entrada é reiniciada para o estado zerado, sem duplicata e sem pânico.

### US3 — Fila de sinais determinística (P1)

`Mundo` passa a ter `sinais: VecDeque<String>` (ou equivalente determinístico de fila FIFO). A fila é o canal determinístico pelo qual `ilha_convertida:<id>` (uma única vez, ao atingir 300 ticks) e `ilha_perdida:<id>` (na transição controlada → Latente e também na perda de progresso parcial) são publicados, em ordem de iteração de `BTreeMap` por `id`. Não há relógio externo, socket, callback assíncrono, logger global ou ordenação dependente de `HashMap`. A fila entra no `hash_estado` e na leitura do HUD. Os nomes `ilha_convertida` e `ilha_perdida` vêm do repertório de §9.1; o sufixo `:<id>` identifica a ilha e é convenção do protótipo. Nenhum outro sinal de §9.1 ganha produtor neste ciclo (por exemplo, `contestacao` permanece sem produtor).

**Why this priority**: a perda precisa ser observável sem efeito colateral externo; uma fila determinística é a única forma de publicar o sinal sem reabrir rede, protocolo ou log dependente de tempo.

**Independent Test**: inspecionar `Mundo.sinais`, os pontos exatos de `push_back` de `ilha_convertida:<id>` e `ilha_perdida:<id>` e a inclusão da fila em `hash_estado`; verificar que não há `Instant`, `SystemTime`, `rand`, I/O ou dependência nova.

**Acceptance Scenarios**:
1. **Given** uma ilha que acabou de perder o controle ou progresso parcial, **When** o tick é concluído, **Then** `sinais.back()` contém `ilha_perdida:<id>` exatamente uma vez para aquela perda.
2. **Given** uma ilha em Latente sem progresso (`controle_ticks = 0`), **When** vários ticks sem Súdito convertido se passam, **Then** nenhum `ilha_perdida:<id>` adicional é enfileirado.
3. **Given** duas ilhas perdidas no mesmo tick, **When** a fila é lida, **Then** os sinais aparecem em ordem crescente de `id`, estável entre execuções.

### US4 — Fase `fase_controle_ilhas` no tick (P1)

`avancar_tick` passa a executar `fase_prepare` → `fase_skills` → `fase_defesa` → **`fase_controle_ilhas`** → `fase_finish` → reversões → incremento de `tick`. A nova fase fica **entre `fase_defesa` e `fase_finish`**, preservando a ordem das fases existentes. Para cada ilha em ordem de `id`: com pelo menos um Súdito convertido, `controle_ticks` avança uma unidade por tick (saturando em 300) e, ao atingir 300, `controlada` vira `true` e `ilha_convertida:<id>` é enfileirado uma única vez; sem Súdito convertido, `controle_ticks` volta a zero e `controlada` vira `false`, com `ilha_perdida:<id>` enfileirado se o estado anterior era `true` **ou** havia progresso parcial (`controle_ticks > 0`). A contagem é **contínua**: qualquer tick de ausência zera o progresso, sem acumulação parcial.

**Why this priority**: é a tradução operacional da regra canônica; posicionar a fase depois de `fase_defesa` garante que o estado de defesa do tick já foi aplicado antes de avaliar a posse, sem alterar `fase_skills` nem `fase_finish`.

**Independent Test**: inspecionar `avancar_tick` e `fase_controle_ilhas`; simular sequências de 299/300 ticks, interrupção no meio e perda após controle, comparando `controlada`, `controle_ticks` e a fila de sinais.

**Acceptance Scenarios**:
1. **Given** 300 ticks ininterruptos com Súdito convertido, **When** a fase roda em cada tick, **Then** `controle_ticks` vai de 0 a 300 e `controlada` só muda para `true` no último tick.
2. **Given** 150 ticks com presença, 1 tick sem presença e depois 150 ticks com presença, **When** a fase roda, **Then** a ilha nunca atinge 300 contínuos: o progresso foi reiniciado no tick de ausência.
3. **Given** a ilha controlada (ou com progresso parcial), **When** a presença falta, **Then** `controle_ticks = 0`, `controlada = false`, um `ilha_perdida:<id>` na fila e nenhuma alteração em jogadores, barricadas, pontes ou súditos.

### US5 — Hash e HUD (P1)

`hash_estado` passa a incluir `ilhas` (por `id`, com `controlada` e `controle_ticks`) e `sinais` (em ordem FIFO), mantendo a concatenação determinística já usada para jogadores, barricadas, pontes e súditos. O HUD de `crates/cliente/src/main.rs` passa a exibir **`Ilhas: N`** (número de ilhas com `controlada == true`) e **`Sinais: N`** (comprimento da fila de sinais), preservando todas as linhas e teclas atuais (`Posicao`, `Ilha`, `Limites`, `Suprimentos`, `Materiais`, `Fôlego`, `Vida`, `Alvo`, `Autogestão`, `Barreiras`, `Pontes`, `Súditos`, `Construindo`, `Coleta` e `WASD`/`B`/`P`/`C`/`G`/`Q`). A linha singular `Ilha: {ilha}` existente permanece intacta; as novas linhas são aditivas. Nenhum campo de protocolo é lido: os valores vêm do espelho `Mundo` que o HUD já consulta.

**Why this priority**: sem o hash as mudanças de estado não são verificáveis de forma determinística; sem as duas linhas do HUD o jogador não percebe progresso nem perda — os dois sintomas que motivam este ciclo.

**Independent Test**: comparar `hash_estado` antes/depois de `adicionar_ilha`, de um `ilha_convertida:<id>` e de um `ilha_perdida:<id>`; inspecionar a string formatada em `atualizar_texto` e conferir as duas linhas aditivas e a conservação das demais.

**Acceptance Scenarios**:
1. **Given** o mesmo input replayado, **When** `hash_estado` é chamado duas vezes, **Then** as strings são idênticas; inserir uma ilha ou um sinal altera o hash de forma previsível pela inclusão dos novos segmentos.
2. **Given** duas ilhas, uma controlada, **When** o HUD é renderizado, **Then** `Ilhas: 1` é exibido com o valor lido de `Mundo.ilhas`.
3. **Given** um `ilha_perdida:<id>` enfileirado, **When** o HUD é atualizado, **Then** `Sinais: 1` aparece e nenhuma linha ou tecla anterior deixa de ser anunciada.

### US6 — Determinismo e fronteira preservados (P2)

As mudanças ficam em `crates/sim/src/lib.rs` (estado, fase, hash), com a leitura aditiva em `crates/cliente/src/main.rs` (HUD) e cobertura determinística em `crates/sim/tests`. `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, configuração de rede, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md` e documentos derivados permanecem intactos nesta etapa. Não há `Lightyear`, `async`, sockets, relógio externo, aleatoriedade, nova janela, novo crate ou dependência. A ordem de `BTreeMap`, o `BTreeSet<u64>` de deduplicação e a regeneração de Fôlego seguem exatamente como nos ciclos anteriores; um replay com o mesmo input produz o mesmo hash.

**Why this priority**: o determinismo é a tese do projeto (Princípios II/III/VI) e a rede continua bloqueada por ADR-004; uma frente de vitória não pode reabrir stack, protocolo ou fronteira de agente.

**Independent Test**: revisar `git status --short` e o diff final, confirmando só os 4 artefatos SDD e os paths de código atribuídos; executar, pelo orquestrador, a suíte determinística e comparar hashes de replay.

**Acceptance Scenarios**:
1. **Given** o estado final do ciclo, **When** os arquivos alterados são listados, **Then** aparecem somente os 4 artefatos desta fronteira e os paths de código da etapa 12.
2. **Given** a suíte determinística, **When** o orquestrador a executa, **Then** ela passa com a cobertura nova e a existente, sem dependência de tempo ou rede.
3. **Given** dois replays com o mesmo input, **When** os hashes são comparados, **Then** são idênticos, inclusive com ilhas e sinais presentes.

### US7 — Governança e estilo (P2)

R1 é `go`; a sequência da constituição v1.2.2 coloca `tasks` antes de `analyze` (READ-ONLY) e R4 (validate) antes de R7 (release `1.0.0-local`); R6 é **N/A** — se invocada, PARE e escale ao SWE, sem issue simulada, sem `gh` improvisado e sem alegar execução. O código segue `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100` e `///` em itens públicos (o atributo de lint `#[expect(clippy::..., reason = "...")]`, já usado no cliente, não é chamada `.expect()` e permanece permitido). Nenhum número do protótipo é promovido a cânone: 300 ticks derivam de 10 s + 30 Hz, e a presença global é simplificação de mapa.

**Why this priority**: preserva a cadeia de rastreabilidade do hub e impede que uma frente de vitória vire porta de entrada para dívida de automação, mudança de stack ou regra canônica inventada.

**Independent Test**: inspecionar os 4 artefatos e o diff de código, conferindo marcações de governança, ordem das portarias e atributos de lint.

**Acceptance Scenarios**:
1. **Given** os 4 artefatos, **When** a governança é revisada, **Then** R1 `go`, R6 N/A, `tasks` antes de `analyze` e R4 antes de R7 `1.0.0-local` aparecem explicitamente.
2. **Given** o código, **When** o estilo é conferido, **Then** não há `unsafe`, `unwrap`/`.expect()`/`#[allow]`, e as linhas respeitam 100 colunas.
3. **Given** 300 ticks e a presença global, **When** a spec é lida, **Then** ambos aparecem rotulados como derivação/simplificação de protótipo, não cânone.

## Edge Cases

- Tick 299 com presença contínua: `controle_ticks = 299`, `controlada = false`; tick 300 com a mesma presença: `controlada = true`. O limite é exatamente 300 (`==`, não `>=` após incremento saturado).
- Interrupção no meio: 150 ticks com presença + 1 tick sem presença + 150 ticks com presença totalizam 300 ticks de presença, mas **não** 300 contínuos — a contagem volta a zero no tick de ausência.
- Ilha sem progresso e sem Súdito convertido: `controle_ticks` permanece 0, `controlada = false` e **nenhum** `ilha_perdida:<id>` é enfileirado. Já a perda de **progresso parcial** (`0 < controle_ticks < 300`) enfileira `ilha_perdida:<id>` uma vez, ainda que a ilha não tenha chegado a ser controlada.
- Vários Súditos convertidos: a presença é booleana ("qualquer um"), não somada; 2 ou 10 Súditos convertidos não aceleram o progresso.
- Súdito convertido que sofre dano e entra em reversão (`reversao_ticks > 0`) mas ainda `convertido = true`: continua contando como presença até a conversão efetivamente terminar; a sim existente decide o valor de `convertido`, a fase de controle apenas o lê.
- Duas ilhas perdidas no mesmo tick (controle ou progresso parcial): `ilha_perdida:<id>` é enfileirado uma vez por ilha, em ordem crescente de `id` (`BTreeMap`), sem duplicata; a conversão de duas ilhas no mesmo tick enfileira `ilha_convertida:<id>` uma vez por ilha, na mesma ordem.
- `adicionar_ilha(id)` com `id` já existente: reinicia a entrada (`insert`), zerando progresso; sem pânico e sem alterar as outras ilhas.
- Fila de sinais com múltiplas entradas: leitura FIFO; o comprimento exibido no HUD é o número de sinais pendentes, não um total histórico.
- Reconversão da Corporação em 20 s ([HIPÓTESE] de GDD §9.4) e prazo de 60 s de conversão de Súdito (§9.2): fora de escopo; nenhum dos dois é implementado neste ciclo.
- Desafio de Ilha ainda não implementado: o protótipo não verifica preparação/contestação; a regra de 10 s é exercitada sobre o estado de presença, e essa ausência fica registrada como simplificação.
- R6 invocada por engano: PARE, escale ao SWE, sem issue simulada e sem `gh` improvisado; dívida de automação registrada, não executada.
- Capturas de HUD/tela: não há surface automatizável no alvo headless; execução ao vivo, gates, build, testes e capturas visuais ficam **[NV]** até verificação pelo orquestrador.

## Requirements

### Functional Requirements

- **FR-001 (regra canônica e simplificação rotulada)**: em `crates/sim/src/lib.rs`, a regra de GDD §9.4:489,501 é implementada com **300 ticks contínuos** de presença de Súdito da Comuna (`PASSO_SEGUNDOS = 1 / 30` × 10 s) para controlar uma ilha; ao atingir 300 ticks emite **`ilha_convertida:<id>`** uma única vez; a ausência devolve a ilha a **Latente** e emite **`ilha_perdida:<id>`**, também quando se perde progresso parcial — sinais do repertório de §9.1 com sufixo `:<id>` de protótipo. Na ausência de mapa, a presença de **qualquer Súdito com `convertido == true`** é tratada como **global**; os números 10 s/300 ticks e a presença global são registrados como **derivação/simplificação de protótipo, nunca cânone**. Nenhuma regra canônica é alterada: Desafio de Ilha, rotas, Nós de Controle, vitória completa, papéis W/E/R e reconversão da Corporação permanecem fora de escopo.
- **FR-002 (modelo `Ilha` e `Mundo.ilhas`)**: `pub struct Ilha { pub id: u64, pub controlada: bool, pub controle_ticks: i64 }` documentada com `///`, com `Mundo.ilhas: BTreeMap<u64, Ilha>` inicializado vazio em `Mundo::new` e `pub fn adicionar_ilha(&mut self, id: u64)` inserindo `controlada = false` e `controle_ticks = 0`. A ordem `BTreeMap` por `id` é estável e usada pela serialização; nenhum outro produto de ilha, índice ou crate é acrescentado.
- **FR-003 (fila `sinais`)**: `Mundo.sinais: VecDeque<String>` (ou equivalente determinístico de fila FIFO), documentado e inicializado vazio; `ilha_convertida:<id>` é enfileirado **uma única vez** quando `controle_ticks` atinge 300, e `ilha_perdida:<id>` é enfileirado **uma vez por perda** — transição de `controlada = true` para `false` **ou** perda de progresso parcial (`controle_ticks > 0`) — ambos em ordem de `id`. Não há relógio externo, I/O, rede, aleatoriedade ou `HashMap` na publicação; nenhum outro sinal de §9.1 ganha produtor novo neste ciclo. A fila participa do `hash_estado` e da leitura do HUD.
- **FR-004 (`fase_controle_ilhas`)**: `avancar_tick` executa `fase_prepare` → `fase_skills` → `fase_defesa` → **`fase_controle_ilhas`** → `fase_finish` → reversões → `tick += 1`. A nova fase itera `Mundo.ilhas` por `id`: com Súdito convertido presente, `controle_ticks` incrementa uma unidade por tick, satura em 300 e, ao atingir 300, marca `controlada = true` e enfileira `ilha_convertida:<id>` uma única vez; sem presença, zera `controle_ticks`, marca `controlada = false` e enfileira `ilha_perdida:<id>` se o estado anterior era `true` ou se havia progresso parcial. A contagem é contínua, não acumulativa; nenhuma outra fase, jogador, barricada, ponte ou súdito é alterado pela nova fase.
- **FR-005 (hash/HUD/fronteira determinística)**: `hash_estado` inclui `ilhas` (`id`, `controlada`, `controle_ticks`, em ordem de `id`) e `sinais` (ordem FIFO), preservando os segmentos existentes. `atualizar_texto` em `crates/cliente/src/main.rs` acrescenta **`Ilhas: N`** (ilhas com `controlada == true`) e **`Sinais: N`** (comprimento da fila), lidos do espelho `Mundo`, sem tocar na linha singular `Ilha: {ilha}` nem nas linhas/teclas atuais e sem campo de protocolo novo. Nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `concurrency.jsonl` ou `STATE.md`; replays com o mesmo input produzem o mesmo hash.
- **FR-006 (governança/estilo)**: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100`, `///` em itens públicos (atributo de lint `#[expect(...)]` permitido); R1 `go`, R6 **N/A** por v1.2.2, `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`; commits em PT-BR com staging explícito; gates, build, testes, formatadores, commit/push/PR **[NV]** para este agente.

### Key Entities

- **`Ilha`**: estado por ilha — `id`, `controlada`, `controle_ticks`; zerada por `adicionar_ilha(id)`, evoluída pela `fase_controle_ilhas`.
- **`Mundo.ilhas`**: `BTreeMap<u64, Ilha>`, ordem estável por `id`, incluída no `hash_estado`.
- **`Mundo.sinais`**: `VecDeque<String>` FIFO; transporta `ilha_convertida:<id>` e `ilha_perdida:<id>` sem I/O externo, em ordem determinística de `id`, incluída no hash e contada no HUD.
- **`Sudito.convertido`**: campo já existente na sim; única entrada da fase de controle (presença global, protótipo sem mapa).
- **`fase_controle_ilhas`**: nova fase entre `fase_defesa` e `fase_finish` em `avancar_tick`; implementa 300 ticks contínuos, a transição para Latente e o enfileiramento determinístico de `ilha_convertida:<id>`/`ilha_perdida:<id>`.
- **`TICKS_CONTROLE_ILHA`**: constante derivada de 10 s × 30 Hz = 300 (rotulada como derivação, não cânone).
- **`hash_estado`**: serializador SHA-256 determinístico; passa a cobrir ilhas e sinais.
- **`atualizar_texto`**: sistema do HUD; ganha as linhas aditivas `Ilhas: N` e `Sinais: N`.

## Clarificações

- A regra **10 s contínuos de Súdito da Comuna** e os sinais **`ilha_convertida`**/**`ilha_perdida`** são cânone (GDD §9.4:489,501; sinais em §9.1); o sufixo `:<id>` que identifica a ilha na fila é convenção do protótipo. A conversão para **300 ticks** é aritmética com o passo fixo existente (`PASSO_SEGUNDOS = 1 / 30`), não um novo número canônico.
- Sem mapa, **presença global de qualquer Súdito convertido** é simplificação registrada: não há posição por ilha, raio, rota ou Desafio de Ilha neste ciclo.
- `ilha_perdida:<id>` é emitido na transição controlada → Latente **e também quando progresso parcial (`controle_ticks > 0`) é perdido**; uma ilha sem progresso não emite sinal, e a fila não repete o sinal a cada tick sem presença (a zeração ocorre no tick da ausência). `ilha_convertida:<id>` é emitido uma única vez ao atingir 300 ticks.
- O repertório de sinais de §9.1 é referenciado como contrato de nomes; este ciclo introduz produtores para `ilha_convertida` e `ilha_perdida` (com sufixo `:<id>`), sem inventar `contestacao` ou outros nomes.
- Reconversão da Corporação (20 s, [HIPÓTESE] em §9.4), Resistência Ideológica, Nós de Controle, vitória completa e papéis W/E/R + níveis são **fora de escopo**, registrados para ciclos seguintes.
- `adicionar_ilha` é o único produtor de ilhas; não há spawn automático ligado a jogador, súdito ou tick.
- A vedação a `unwrap`/`expect` refere-se a chamadas de método; o atributo de lint `#[expect(clippy::needless_pass_by_value, reason = "...")]`, já presente no cliente, permanece permitido.
- **R6 é N/A** por constituição v1.2.2: fora da sequência executável até `resolve-tasks.sh` e ao adaptador de issues existirem; se invocada, PARE e escale ao SWE, sem simular issue, sem `gh` improvisado e sem alegar execução.
- Execução de gates/build/testes/formatadores e capturas visuais não têm surface automatizável neste agente: ficam **[NV]** até o orquestrador. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0).

## Success Criteria

- **SC-001 (regra e rotulação)**: 300 ticks contínuos de presença de Súdito convertido controlam a ilha e emitem `ilha_convertida:<id>` uma única vez; a ausência retorna a Latente e emite `ilha_perdida:<id>` (também com progresso parcial); 10 s/300 ticks e a presença global aparecem rotulados como derivação/simplificação de protótipo, e nenhum valor foi promovido a regra canônica nova.
- **SC-002 (modelo)**: `Ilha { id, controlada, controle_ticks }`, `Mundo.ilhas: BTreeMap<u64, Ilha>` e `adicionar_ilha(id)` existem documentados; ilhas novas nascem com `controlada = false` e `controle_ticks = 0`; a ordem por `id` é estável e nenhum produtor/crate/dependência novo aparece.
- **SC-003 (sinais)**: `Mundo.sinais: VecDeque<String>` (ou equivalente FIFO determinístico) registra `ilha_convertida:<id>` uma única vez na conversão e `ilha_perdida:<id>` uma vez por perda (transição controlada → Latente ou progresso parcial), em ordem de `id`, sem relógio, I/O, rede ou aleatoriedade; ilhas sem progresso não emitem sinal adicional.
- **SC-004 (fase)**: `fase_controle_ilhas` roda entre `fase_defesa` e `fase_finish`; 299 ticks não controlam, o 300º controla e emite `ilha_convertida:<id>` uma única vez; uma ausência isolada zera o progresso; a perda de uma ilha controlada ou com progresso parcial zera o contador, marca Latente e enfileira `ilha_perdida:<id>`, sem alterar outros subsistemas.
- **SC-005 (hash/HUD/fronteira)**: `hash_estado` inclui ilhas e sinais e permanece idêntico entre replays com o mesmo input; o HUD exibe `Ilhas: N` e `Sinais: N` aditivamente, preservando todas as linhas e teclas anteriores; contratos, protocolo, Cargo, manifesto, CI e rede permanecem intactos.
- **SC-006 (governança/estilo)**: R1 `go`, R6 N/A, `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`; execução, capturas visuais e publicação **[NV]** até o orquestrador.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs` e `crates/sim/tests` na etapa 12; o orquestrador executa gates, build, testes e verificações finais.
- A sim já tem `PASSO_SEGUNDOS = 1 / 30`, `avancar_tick`, `fase_prepare`, `fase_skills`, `fase_defesa`, `fase_finish`, `hash_estado`, `BTreeMap`, `VecDeque`, `Sudito.convertido` e `adicionar_sudito`; o cliente já tem `atualizar_texto` e o espelho `Mundo`. Nenhum crate é necessário.
- Ambiente headless: nenhuma janela gráfica utilizável, rede, credencial ou comando `gh` é usado na redação destes artefatos; capturas visuais e execução ao vivo ficam **[NV]**.
- O scorecard **108/625** é a escolha do ciclo 049: frente canônica de controle de ilha, de maior Pontuação entre as frentes de produto avaliadas; Nós/vitória e papéis (48/625 cada) ficam para o próximo ciclo e a rede (10/625) segue bloqueada por ADR-004 até decisão da SE.

## Out of scope

Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md` e documentos derivados; `Lightyear`, `async`, sockets, TCP, servidor, segundo jogador humano; mapa, posições por ilha, raios, rotas, Desafio de Ilha, preparação/contestação, Nós de Controle, Resistência Ideológica, reconversão da Corporação em 20 s, papéis W/E/R, níveis e vitória completa; reabertura de ponte (045), súditos (046), combate (047) ou alvo de treino (048); promoção de qualquer número a cânone; execução de gates, build, testes, formatadores, commits, push, PRs, merge ou deploy; alterações preexistentes fora da fronteira. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: Controle de ilha por Súdito **4×3×3×3 = 108/625**, contra Nós de Controle + vitória completa **4×2×2×3 = 48/625**, Papéis W/E/R + níveis **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. Registro em `.specify/memory/STATE.md` (2026-11-23T11:15Z). Nenhum manifesto é criado e nenhum número do protótipo é promovido a cânone.
