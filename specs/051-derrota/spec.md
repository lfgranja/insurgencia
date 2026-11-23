# Feature Specification: Derrota por timeout e reconquista no cliente standalone

**Branch**: `051-derrota` (base `3132f47`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T142500Z-derrota`, `insurgencia - derrota por timeout e reconquista`.

**Âncora**: decisão 80/20 do ciclo 051 registrada em `.specify/memory/STATE.md` (2026-11-23T14:25Z): **Derrota por timeout/reconquista = 4×3×4×4 = 192/625**, escolhida sobre Papéis W/E/R + níveis **4×2×2×3 = 48/625**, PVE/PNJs **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. A regra canônica é GDD §6.2:277 (a Comuna é dissolvida se o relógio de 20 min se esgotar sem a conversão ou se a Corporação reconquistar 3 ilhas já libertadas) e §9.4:489,501 (ilha sem Súdito volta a Latente e emite `ilha_perdida`), com a mensagem "Comuna dissolvida" do critério de aceitação de GDD §6.2:285. **Todos os números são `[HIPÓTESE]` no GDD**: 20 min, 3 reconquistas e 3 ilhas; a conversão para **36000 ticks** é derivação de `PASSO_SEGUNDOS = 1 / 30` (20 min × 30 Hz = 1200 s × 30) e o uso de `ilha_perdida` como proxies de reconquista é simplificação de protótipo sem mapa, sem Corporação autônoma e sem rede — ambos rotulados como não-canônicos. O ciclo 050 entregou `resultado: Option<ResultadoPartida>` com a variante `Vitoria`, `fase_vitoria`, `Mundo.ilhas` e o sinal `ilha_perdida`; falta o desfecho de derrota: sem relógio, sem contador de reconquistas e sem mensagem "Comuna dissolvida", a regra de GDD §6.2:277 não é observável e a partida nunca termina em derrota.

## User Scenarios

### US1 — Condição de derrota observável (P1)

O cliente passa a representar o desfecho de derrota de GDD §6.2:277 e §6.2:285: com `resultado = None`, se `tick >= 36000` (20 min derivados a 30 Hz) **ou** se pelo menos **3 ilhas distintas** tiverem emitido `ilha_perdida`, `resultado` passa a `Some(ResultadoPartida::Derrota)` e o HUD anuncia "Comuna dissolvida" na linha `Resultado` já existente. A vitória do ciclo 050 prevalece se ocorrer no mesmo tick: `fase_vitoria` corre antes de `fase_derrota` e o guarda `resultado.is_some()` congela o desfecho. Antes deste ciclo a sim já tem ilhas, Súditos, Nós, Resistência, sinais e `resultado = Some(Vitoria)`, mas nenhuma condição de derrota: relógio e reconquista são observáveis apenas no GDD. Nenhum relógio externo, servidor, comando, protocolo ou nova tecla é introduzido — o contador é o próprio `tick` do passo fixo de 30 Hz.

**Why this priority**: é a frente de maior pontuação do ciclo (192/625) e a última lacuna da cadeia derrota/vitória do módulo base; sem `Derrota`, nenhum critério de aceitação de GDD §6.2:277/§6.2:285 pode ser verificado no estado observável.

**Independent Test**: ler GDD §6.2:277, §6.2:285 e §9.4:489,501; inspecionar a spec e o código: `tick = 35999` mantém `resultado = None`, `tick = 36000` define `Derrota`; 2 ilhas perdidas mantêm `None`, a 3ª define `Derrota` com "Comuna dissolvida"; vitória no mesmo tick permanece `Vitoria`. Nenhum número é apresentado como cânone e nenhuma regra de vitória é alterada.

**Acceptance Scenarios**:
1. **Given** `Mundo` com `resultado = None`, ilhas sem perda e `tick = 35999`, **When** `avancar_tick` processa esse tick, **Then** `resultado` permanece `None` e nenhuma mensagem de derrota é enfileirada.
2. **Given** o mesmo mundo em `tick = 36000`, **When** `fase_derrota` roda, **Then** `resultado = Some(ResultadoPartida::Derrota)` e o HUD pode exibir "Comuna dissolvida"; ticks posteriores não repetem a mensagem nem trocam o desfecho.
3. **Given** 2 ids em `ilhas_reconquistadas`, **When** a 3ª ilha distinta emite `ilha_perdida:<id>` no mesmo tick, **Then** `resultado = Some(Derrota)` ainda naquele tick, com dedup por id.
4. **Given** `fase_vitoria` define `Some(Vitoria)` no tick em que o 36000º tick também seria atingido, **When** `fase_derrota` roda, **Then** o desfecho permanece `Vitoria` e nenhuma derrota é registrada.

### US2 — Modelo determinístico da derrota (P1)

`Mundo` passa a ter `ilhas_reconquistadas: BTreeSet<u64>`, inicializado vazio em `Mundo::new`. É o único estado novo: ele registra ids de ilhas cujo sinal `ilha_perdida` foi emitido, permitindo contar reconquistas distintas sem `HashMap`, sem histórico de sinais e sem campo de relógio — o tempo é o `tick: i64` existente. `ResultadoPartida` ganha a variante `Derrota` (uma única variante, sem `DerrotaPorTempo`/`DerrotaPorReconquista`: a mensagem de HUD é a mesma e o hash usa um token estável por desfecho). As constantes derivadas `TICKS_LIMITE_PARTIDA: i64 = 36000` e `MAX_RECONQUISTAS: usize = 3` são documentadas como `[HIPÓTESE]`/protótipo, nunca como cânone. `BTreeSet` preserva a ordem crescente por `id`, requisito do determinismo de `hash_estado`. Nenhum produtor de reconquista fora de `fase_controle_ilhas`, nenhum comando, nenhuma ação e nenhuma dependência nova.

**Why this priority**: é a menor superfície de estado capaz de tornar a regra verificável — sem o conjunto, sem a variante e sem as constantes rotuladas, não há onde acumular reconquistas nem como observar o limite de tempo de forma determinística.

**Independent Test**: inspecionar `Mundo.ilhas_reconquistadas`, `Mundo::new`, `ResultadoPartida`, `TICKS_LIMITE_PARTIDA` e `MAX_RECONQUISTAS`; confirmar estado inicial vazio, ordem `BTreeSet`, documentação `[HIPÓTESE]` e ausência de nova dependência.

**Acceptance Scenarios**:
1. **Given** um `Mundo` recém-criado, **When** o estado é inspecionado, **Then** `ilhas_reconquistadas` está vazio, `resultado = None`, `tick = 0`, e as duas constantes derivadas existem com os valores 36000 e 3.
2. **Given** ids `7`, `2` e `5` inseridos em ordem arbitrária, **When** o hash é calculado, **Then** a serialização segue a ordem crescente `2;5;7`, idêntica entre execuções com o mesmo input.
3. **Given** um replay idêntico, **When** os dois mundos avançam os mesmos ticks, **Then** o conjunto e o desfecho são idênticos, sem relógio do sistema, aleatoriedade ou I/O.

### US3 — Registro de reconquista a partir de `ilha_perdida` (P1)

Em `fase_controle_ilhas`, sempre que o código enfileira `ilha_perdida:<id>` — transição de ilha controlada ou com progresso para Latente por ausência de Súdito convertido — o mesmo `id` é inserido em `Mundo.ilhas_reconquistadas`. A inserção acontece no ponto de emissão do sinal, em ordem crescente de `id` (iteração do `BTreeMap<Id, Ilha>`), e o `BTreeSet` garante dedup por id: uma ilha que já entrou no conjunto não entra segunda vez, mesmo que volte a ser controlada e perdida novamente. Ilha nunca controlada, sem progresso, ou já Latente não emite sinal e não altera o conjunto. Nenhum sinal novo é criado, nenhum produtor de `ilha_perdida` é acrescentado e a semântica atual do sinal não muda; apenas o registro do id é aditivo.

**Why this priority**: é a ponte entre a regra de perda de ilha (§9.4) e a contagem de reconquistas (§6.2:277) sem introduzir simulação da Corporação, mapa ou reconversão de 20 s.

**Independent Test**: inspecionar `fase_controle_ilhas`, a emissão de `ilha_perdida:<id>` e a inserção em `ilhas_reconquistadas`; simular perda de ilha controlada, perda de progresso parcial, ilha Latente repetida e recaptura da mesma ilha.

**Acceptance Scenarios**:
1. **Given** a ilha `4` controlada e nenhum Súdito convertido, **When** o tick processa a perda, **Then** `ilha_perdida:4` é enfileirado e `ilhas_reconquistadas` contém `4` exatamente uma vez.
2. **Given** `ilhas_reconquistadas = {2, 5}`, **When** a ilha `2` volta a perder controle, **Then** nenhuma segunda entrada `2` é criada e o tamanho do conjunto permanece 2.
3. **Given** uma ilha Latente sem progresso, **When** vários ticks se passam sem Súdito, **Then** nenhum `ilha_perdida` é emitido e o conjunto não muda.
4. **Given** três ilhas perdidas em ordem `7`, `2`, `5`, **When** o conjunto é serializado no hash, **Then** a ordem é `2;5;7`, independentemente da ordem de emissão.

### US4 — `fase_derrota` no tick e precedência da vitória (P1)

`avancar_tick` passa a executar `fase_prepare` → `fase_skills` → `fase_defesa` → `fase_controle_ilhas` → `fase_vitoria` → **`fase_derrota`** → `fase_finish` → reversões → `tick += 1`. `fase_derrota` é pura e determinística, nesta ordem de guardas: (1) se `resultado.is_some()`, retorna sem efeito; (2) se `tick >= TICKS_LIMITE_PARTIDA`, define `Some(Derrota)`; (3) se `ilhas_reconquistadas.len() >= MAX_RECONQUISTAS`, define `Some(Derrota)`. Com `tick` começando em 0 e incrementado no fim de `avancar_tick`, o limite dispara no tick rotulado `36000`, isto é, após 36000 ticks processados (20 min a 30 Hz). A ordem das fases garante que a vitória no mesmo tick prevaleça; a guarda `resultado.is_some()` também torna a derrota idempotente — nenhum segundo sinal, nenhuma segunda mensagem e nenhuma mudança de `Derrota` para outro desfecho. Nenhuma outra fase muda de semântica e nenhum subsistema é interrompido antecipadamente.

**Why this priority**: sem a fase e a precedência explícita, a derrota poderia sobrescrever uma vitória canônica ou disparar no tick errado (35999 em vez de 36000), invalidando GDD §6.2:277 e a cadeia de rastreabilidade do ciclo 050.

**Independent Test**: inspecionar `avancar_tick` e `fase_derrota`; simular 35999/36000 ticks, 2/3 reconquistas, vitória no mesmo tick, ambos os gatilhos no mesmo tick e ticks posteriores ao desfecho.

**Acceptance Scenarios**:
1. **Given** `tick = 35999` e conjunto vazio, **When** `avancar_tick` roda, **Then** `resultado = None`; no tick seguinte, com `tick = 36000`, **Then** `resultado = Some(Derrota)` e `tick` passa a 36001.
2. **Given** `tick = 100` e três ids distintos no conjunto, **When** `fase_derrota` roda, **Then** `resultado = Some(Derrota)` sem esperar o relógio.
3. **Given** `fase_vitoria` acabou de definir `Some(Vitoria)` e o conjunto já tem 3 ids, **When** `fase_derrota` roda no mesmo tick, **Then** `resultado` continua `Vitoria`.
4. **Given** `resultado = Some(Derrota)`, **When** mais ticks são processados, inclusive com novas perdas de ilha, **Then** o desfecho não muda e nenhuma mensagem adicional é produzida.

### US5 — Hash, HUD e fronteira determinística (P1)

`hash_estado` passa a incluir `ilhas_reconquistadas` (ids em ordem crescente) e a estender o token de `resultado` com `"derrota"`, ao lado de `"vitoria"` e `"nenhum"`, preservando todos os segmentos existentes (tick, epoch, match_id, jogadores, comandos, barreiras, pontes, súditos, ilhas, sinais, nós, resistência etc.). No cliente, a linha `Resultado: {resultado}` de `atualizar_texto` já existe e passa a exibir **"Comuna dissolvida"** quando `Some(Derrota)`; "Autogestão estabelecida" e "em curso" permanecem para os outros estados. Nenhuma linha, tecla ou mensagem antiga é removida; nenhuma tecla nova é mapeada. Contratos, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI e rede permanecem intactos: o desfecho é derivado do estado local do mesmo tick, sem campo de protocolo, sem mensagem de rede e sem servidor adicional.

**Why this priority**: sem o hash as mudanças não são verificáveis de forma determinística e sem a linha no HUD o jogador não percebe a derrota — o sintoma que motiva este ciclo; a fronteira preserva a tese de determinismo (Princípios II/III/VI) e a rede bloqueada por ADR-004.

**Independent Test**: comparar `hash_estado` antes/depois de inserir um id no conjunto e de definir `Derrota`; inspecionar `atualizar_texto` e conferir a nova mensagem na linha existente, a conservação das demais linhas/teclas e a ausência de campo de protocolo; revisar `git status --short`.

**Acceptance Scenarios**:
1. **Given** dois replays com o mesmo input, **When** `hash_estado` é chamado, **Then** as strings são idênticas; inserir um id em `ilhas_reconquistadas` ou definir `Derrota` altera o hash de forma previsível pelos novos segmentos.
2. **Given** `resultado = Some(Derrota)`, **When** `atualizar_texto` é lido, **Then** a linha mostra `Resultado: Comuna dissolvida`, mantendo `Posicao`, `Ilha`, `Limites`, `Suprimentos`, `Materiais`, `Fôlego`, `Vida`, `Alvo`, `Autogestão`, `Barreiras`, `Pontes`, `Súditos`, `Ilhas`, `Sinais`, `Construindo`, `Coleta`, `Resistência`, `Nós` e todas as teclas `WASD`/`B`/`P`/`C`/`G`/`Q`.
3. **Given** o estado final do ciclo, **When** os arquivos alterados são listados, **Then** aparecem somente os 4 artefatos SDD e os paths de código atribuídos (sim, testes da sim e cliente), sem contratos, protocolo, Cargo, CI ou rede.

### US6 — Sem novas teclas e sem superfície nova (P2)

A derrota é passiva: nenhuma ação do jogador a dispara diretamente, nenhuma tecla é adicionada e nenhuma UI, janela ou surface de captura é criada. O jogador percebe o desfecho apenas pela linha `Resultado` do HUD e, se desejar, pela diferença no hash entre replays. A ausência de interface para "render-se" ou "negociar" é simplificação documentada: GDD §6.2:277 fala em relógio e reconquista, não em comando do jogador. Se a fiação do texto no alvo headless for complexa, a mensagem fica registrada no estado determinístico e documentada como protótipo, sem inventar janela ou sintetizar captura visual.

**Why this priority**: preserva o escopo de uma frente de regras e evita que a derrota vire porta de entrada para inputs, protocolo ou superfície gráfica fora da fronteira.

**Independent Test**: revisar o tratamento de `KeyCode` no cliente e a lista de ações da sim; confirmar que nenhuma tecla, comando ou ação nova aparece e que nenhuma captura visual é fabricada.

**Acceptance Scenarios**:
1. **Given** o cliente final, **When** o tratamento de teclado é inspecionado, **Then** as teclas são exatamente as anteriores e nenhuma delas altera `ilhas_reconquistadas`, `tick` ou `resultado` fora das regras existentes.
2. **Given** o alvo headless, **When** a verificação é conduzida, **Then** inspeção de código/terminal é marcada como automatizável e qualquer captura visual de HUD permanece **[NV]** até verificação humana.

### US7 — Governança e estilo (P2)

R1 é `go`; a sequência da constituição v1.2.2 coloca `tasks` antes de `analyze` (READ-ONLY) e R4 (validate) antes de R7 (release `1.0.0-local`); **R6 é N/A** — se invocada, PARE e escale ao SWE, sem issue simulada, sem `gh` improvisado e sem alegar execução. O código segue `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100` e `///` em itens públicos (o atributo de lint `#[expect(clippy::..., reason = "...")]`, já usado no cliente, não é chamada `.expect()` e permanece permitido). Nenhum número do protótipo é promovido a cânone: 36000 ticks derivam de 20 min × 30 Hz e o uso de `ilha_perdida` como reconquista é aproximação sem mapa nem Corporação. Derrota por causas adicionais, variante duas Comunas, Plano Mestre, papéis W/E/R, PVE/PNJs, níveis e rede permanecem fora de escopo.

**Why this priority**: preserva a cadeia de rastreabilidade do hub e impede que uma frente de derrota vire porta de entrada para dívida de automação, mudança de stack ou regra canônica inventada.

**Independent Test**: inspecionar os 4 artefatos e o diff de código, conferindo marcações de governança, ordem das portarias e atributos de lint.

**Acceptance Scenarios**:
1. **Given** os 4 artefatos, **When** a governança é revisada, **Then** R1 `go`, R6 N/A, `tasks` antes de `analyze` e R4 antes de R7 `1.0.0-local` aparecem explicitamente, com scorecard **192/625**.
2. **Given** o código, **When** o estilo é conferido, **Then** não há `unsafe`, `unwrap`/`.expect()`/`#[allow]`, e as linhas respeitam 100 colunas.
3. **Given** os números do protótipo, **When** a spec é lida, **Then** 20 min/36000 ticks, 3 reconquistas e o proxy `ilha_perdida` aparecem rotulados como `[HIPÓTESE]`/derivação, não como cânone.

## Edge Cases

- Tick 35999 com `resultado = None`: nenhuma derrota; tick 36000: `Some(Derrota)`. O limite é `tick >= 36000` avaliado **antes** de `tick += 1` no fim de `avancar_tick` — 36000 ticks processados correspondem a 20 min a 30 Hz.
- Duas ilhas perdidas: `resultado = None`; a 3ª ilha **distinta** define `Derrota`. A contagem é de ids únicos em `BTreeSet`: a mesma ilha perdida duas vezes conta uma vez, e perder 3 vezes a mesma ilha não encerra a partida.
- `ilha_perdida` por progresso parcial: o código atual emite o sinal também quando `controle_ticks > 0` sem ilha controlada; o protótipo registra esse id como reconquista. É uma aproximação determinística explícita (não cânone), sem simular a Corporação nem os 20 s de reconversão de GDD §9.4.
- Ilha já Latente, sem progresso: nenhum sinal, nenhuma inserção no conjunto. Recaptura e nova perda da mesma ilha: o `BTreeSet` dedup mantém um único id.
- Vitória e derrota no mesmo tick: `fase_vitoria` corre antes e define `resultado`; `fase_derrota` vê `Some(_)` e retorna. Vitória prevalece sempre (GDD §6.2:285: a derrota por reconquista é registrada mesmo com Resistência abaixo de 50, mas não pode apagar uma vitória já gravada no mesmo tick).
- Timeout e 3 reconquistas no mesmo tick: desfecho único `Derrota`; a ordem das guardas não altera o observável (mensagem e hash idênticos).
- Derrota já definida: ticks seguintes não repetem mensagem, não acrescentam sinais, não trocam `Derrota` por `Vitoria` e não zeram o conjunto.
- `Mundo.nos` vazio: `fase_vitoria` exige 3 Nós, então não há vitória; a derrota por tempo ou por reconquista ainda funciona.
- Replays idênticos: `BTreeSet` por `id`, `BTreeMap` de ilhas e `VecDeque` de sinais mantêm a ordem; `hash_estado` inclui os novos segmentos de forma estável.
- `ilha_perdida:<id>` com id já presente no conjunto: nenhuma duplicata, nenhum pânico, nenhuma mudança no hash além da já registrada.
- Sem Súdito convertido em nenhuma ilha: todas as ilhas controladas voltam a Latente em ordem de `id`, cada uma emitindo seu sinal e entrando no conjunto uma vez — 3 ilhas assim encerram a partida no mesmo tick.
- R6 invocada por engano: PARE, escale ao SWE, sem issue simulada e sem `gh` improvisado; dívida de automação registrada, não executada.
- Capturas de HUD/tela: não há surface automatizável no alvo headless; execução ao vivo, gates, build, testes e capturas visuais ficam **[NV]** até verificação pelo orquestrador.

## Requirements

### Functional Requirements

- **FR-001 (regra de derrota canônica e rotulação)**: em `crates/sim/src/lib.rs`, a condição de GDD §6.2:277 e §6.2:285 é implementada como dissolução da Comuna (`"Comuna dissolvida"`) quando o relógio de **20 min** se esgota sem conversão **ou** quando **3 ilhas já libertadas** são reconquistadas, usando a perda de controle sinalizada por `ilha_perdida` (GDD §9.4:489,501) como proxy determinístico. A vitória prevalece se ocorrer no mesmo tick e nenhuma regra de vitória do ciclo 050 é alterada. **Todos os números são `[HIPÓTESE]` no GDD**: 20 min, 3 reconquistas e 3 ilhas; **36000 ticks** são derivação de `PASSO_SEGUNDOS = 1 / 30` (20 min × 30 Hz) e o proxy por `ilha_perdida` é simplificação sem mapa, sem Corporação autônoma e sem os 20 s de reconversão — rotulados como protótipo, nunca promovidos a cânone.
- **FR-002 (modelo do desfecho de derrota)**: `pub enum ResultadoPartida` ganha a variante `Derrota` documentada com `///` (variantes separadas por causa são desnecessárias: mensagem e token de hash são únicos); `Mundo.ilhas_reconquistadas: BTreeSet<u64>` é inicializado vazio em `Mundo::new` e é o único campo novo, registrando ids de ilhas que emitiram `ilha_perdida`; o tempo é o `tick: i64` existente, sem campo de relógio. `TICKS_LIMITE_PARTIDA: i64 = 36000` e `MAX_RECONQUISTAS: usize = 3` são constantes derivadas/hipotéticas documentadas. A ordem `BTreeSet` por `id` é estável e usada pela serialização; nenhum outro produtor de reconquista, índice, crate ou dependência é acrescentado.
- **FR-003 (registro de reconquista no sinal de perda)**: `fase_controle_ilhas` continua enfileirando `ilha_perdida:<id>` exatamente nas transições atuais (ilha controlada ou com `controle_ticks > 0` → Latente por ausência de Súdito convertido, em ordem crescente de `id`) e, **no mesmo ponto de emissão**, insere `id` em `Mundo.ilhas_reconquistadas`. O `BTreeSet` deduplica por id: ilha já registrada não entra segunda vez, ilha Latente sem progresso não emite e não registra. Nenhum sinal novo, nenhuma mudança na semântica de `ilha_perdida`, nenhum comando/ação nova e nenhum produtor fora dessa fase.
- **FR-004 (fase de derrota e precedência da vitória)**: `avancar_tick` executa `fase_prepare` → `fase_skills` → `fase_defesa` → `fase_controle_ilhas` → `fase_vitoria` → **`fase_derrota`** → `fase_finish` → reversões → `tick += 1`. `fase_derrota` retorna sem efeito se `resultado.is_some()` (vitória prevalece, derrota idempotente); caso contrário define `Some(ResultadoPartida::Derrota)` se `tick >= TICKS_LIMITE_PARTIDA` **ou** se `ilhas_reconquistadas.len() >= MAX_RECONQUISTAS`, nessa ordem de guardas, antes do incremento de `tick`. Nenhuma outra fase muda de semântica, nenhuma mensagem é enfileirada duas vezes e nenhum subsistema é interrompido antecipadamente.
- **FR-005 (hash, HUD e fronteira determinística)**: `hash_estado` inclui `ilhas_reconquistadas` (ids em ordem crescente) e estende o token de `resultado` com `"derrota"` ao lado de `"vitoria"`/`"nenhum"`, preservando todos os segmentos existentes e produzindo o mesmo resultado em replays com o mesmo input. `atualizar_texto` mantém a linha `Resultado: {resultado}` e passa a exibir **"Comuna dissolvida"** para `Some(Derrota)`, preservando "Autogestão estabelecida", "em curso", todas as outras linhas e teclas (`WASD`/`B`/`P`/`C`/`G`/`Q`) — sem novas teclas. Nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `concurrency.jsonl` ou `STATE.md`.
- **FR-006 (governança/estilo)**: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100`, `///` em itens públicos (atributo de lint `#[expect(...)]` permitido); R1 `go`, R6 **N/A** por v1.2.2 (PARE se invocada, sem issue simulada e sem `gh` improvisado), `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`; commits em PT-BR com staging explícito; gates/build/testes/formatadores/commit/push/PR **[NV]** para este agente.

### Key Entities

- **`ResultadoPartida::Derrota`**: novo desfecho observável, idempotente, gravado por `fase_derrota`; prevalecido por `Vitoria` no mesmo tick; mapeado para "Comuna dissolvida" no HUD e para o token `derrota` no hash.
- **`Mundo.ilhas_reconquistadas`**: `BTreeSet<u64>` vazio em `Mundo::new`; ids de ilhas que emitiram `ilha_perdida`, deduplicados e ordenados; contagem de reconquistas distintas sem histórico de sinais.
- **`Mundo.tick`**: relógio determinístico existente; usado como cronômetro da partida (`tick >= 36000`), sem relógio externo nem `Instant`/`SystemTime`.
- **`TICKS_LIMITE_PARTIDA` / `MAX_RECONQUISTAS`**: constantes derivadas 36000 (20 min × 30 Hz) e 3, rotuladas `[HIPÓTESE]`/protótipo, nunca cânone.
- **`fase_derrota`**: nova fase entre `fase_vitoria` e `fase_finish`; guardas `resultado.is_some()` → tempo → reconquistas; sem efeitos colaterais fora de `resultado`.
- **`Mundo.sinais` / `ilha_perdida:<id>`**: fila FIFO existente; o sinal continua sendo emitido por `fase_controle_ilhas` e passa a alimentar o conjunto de reconquistas no mesmo ponto de emissão.
- **`hash_estado`**: serializador SHA-256 determinístico; passa a cobrir `ilhas_reconquistadas` e o token `derrota`.
- **`atualizar_texto`**: sistema do HUD; a linha `Resultado` existente ganha o valor "Comuna dissolvida", sem novas teclas ou linhas.

## Clarificações

- A **derrota por relógio de 20 min ou 3 reconquistas** e a mensagem **"Comuna dissolvida"** são cânone narrativo (GDD §6.2:277, §6.2:285); **todos os números** são `[HIPÓTESE]` no GDD e entram como protótipo. **36000 ticks** são aritmética com o passo fixo existente (`PASSO_SEGUNDOS = 1 / 30` = 20 min × 30 Hz), não um novo número canônico.
- O protótipo **não tem mapa, Corporação autônoma nem reconversão em 20 s**: a perda de controle sinalizada por `ilha_perdida` (GDD §9.4:489,501) é usada como proxy determinístico de "reconquistar ilha libertada", com dedup por id. Essa aproximação é rotulada como não-canônica nos 4 artefatos; nenhuma regra de reconversão é implementada.
- O limite de tempo é avaliado com `tick >= TICKS_LIMITE_PARTIDA` **antes** de `tick += 1`: com `tick` iniciando em 0, a derrota por tempo ocorre ao processar o tick rotulado 36000 (após 36000 ticks = 20 min a 30 Hz). Nenhum `Instant`, `SystemTime`, `sleep` ou relógio externo é usado.
- A **vitória prevalece** no mesmo tick porque `fase_vitoria` corre antes de `fase_derrota` e `fase_derrota` começa com `resultado.is_some()`. Isso preserva GDD §6.2:285 sem apagar uma vitória já gravada; quando não há vitória, a derrota por reconquista é registrada mesmo com Resistência abaixo de 50.
- **Uma variante única `Derrota`** é suficiente: a mensagem de HUD é a mesma para tempo e reconquistas e o hash usa um token estável por desfecho; variantes `DerrotaPorTempo`/`DerrotaPorReconquista` não agregam observável neste protótipo e foram descartadas para manter a superfície mínima.
- `ilha_perdida` continua sendo emitido também na perda de progresso parcial (`controle_ticks > 0` sem ilha controlada), conforme o código atual do ciclo 049; a spec **não altera** essa semântica, apenas registra o id no conjunto no ponto de emissão.
- O conjunto deduplica por **id de ilha**: "3 reconquistas" significam 3 ilhas distintas; perder a mesma ilha repetidamente não acumula. Isso é coerente com "3 ilhas já libertadas" e evita ganho por repetição acidental de sinal.
- **Nenhuma tecla, comando ou ação nova** é introduzida; a derrota é um efeito passivo do tick. A linha `Resultado` do HUD já existe e muda apenas de valor.
- **Derrota por outras causas**, variante duas Comunas, Plano Mestre, papéis W/E/R, PVE/PNJs, níveis e rede são **fora de escopo**, registrados para ciclos seguintes.
- A vedação a `unwrap`/`expect` refere-se a chamadas de método; o atributo de lint `#[expect(clippy::needless_pass_by_value, reason = "...")]`, já presente no cliente, permanece permitido.
- **R6 é N/A** por constituição v1.2.2: fora da sequência executável até `resolve-tasks.sh` e ao adaptador de issues existirem; se invocada, PARE e escale ao SWE, sem simular issue, sem `gh` improvisado e sem alegar execução.
- Execução de gates/build/testes/formatadores e capturas visuais não têm surface automatizável neste agente: ficam **[NV]** até o orquestrador. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0).

## Success Criteria

- **SC-001 (regra e rotulação)**: `resultado = Some(Derrota)` e "Comuna dissolvida" ocorrem após 36000 ticks sem conversão **ou** ao 3º id distinto em `ilhas_reconquistadas`; 2 reconquistas não encerram; vitória no mesmo tick prevalece e derrota é idempotente; 20 min/36000 ticks, 3 reconquistas e o proxy `ilha_perdida` aparecem rotulados `[HIPÓTESE]`/protótipo, sem promoção a cânone.
- **SC-002 (modelo)**: `ResultadoPartida::Derrota`, `Mundo.ilhas_reconquistadas: BTreeSet<u64>` vazio em `Mundo::new`, `TICKS_LIMITE_PARTIDA = 36000` e `MAX_RECONQUISTAS = 3` existem documentados; `tick` é o único relógio; ordem estável por `id`, sem produtor/crate/dependência novo.
- **SC-003 (registro de reconquista)**: cada `ilha_perdida:<id>` insere o id uma vez no conjunto, na ordem de `id`; ilha Latente não emite nem registra; recaptura e nova perda do mesmo id não duplicam; nenhum sinal, comando ou ação nova e nenhuma mudança na semântica do sinal.
- **SC-004 (fase e precedência)**: `fase_derrota` roda entre `fase_vitoria` e `fase_finish` com as guardas `resultado.is_some()` → `tick >= 36000` → `len() >= 3`; 35999 não dispara e 36000 dispara; 2 não disparam e 3 disparam; vitória no mesmo tick prevalece; nenhuma segunda mensagem em ticks posteriores.
- **SC-005 (hash/HUD/fronteira)**: `hash_estado` inclui o conjunto ordenado e o token `derrota` e permanece idêntico entre replays com o mesmo input; a linha `Resultado` exibe "Comuna dissolvida" sem remover linhas ou teclas, sem novas teclas; contratos, protocolo, Cargo, manifesto, CI e rede permanecem intactos.
- **SC-006 (governança/estilo)**: R1 `go`, R6 N/A, `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`; execução, capturas visuais e publicação **[NV]** até o orquestrador.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs` e `crates/cliente/src/main.rs` na etapa 12; o orquestrador executa gates, build, testes e verificações finais.
- A sim já tem `PASSO_SEGUNDOS = 1 / 30`, `Mundo` com `BTreeMap`/`BTreeSet`/`VecDeque`, `tick` como contador de passos, `avancar_tick` com `fase_prepare`/`fase_skills`/`fase_defesa`/`fase_controle_ilhas`/`fase_vitoria`/`fase_finish`, `Ilha`, `Mundo.sinais` com `ilha_perdida:<id>`, `ResultadoPartida::Vitoria`, `hash_estado` e o padrão de emissão de sinais em ordem de `id`; o cliente já tem `atualizar_texto`, o espelho `Mundo` e a linha `Resultado`. Nenhum crate é necessário.
- Ambiente headless: nenhuma janela gráfica utilizável, rede, credencial ou comando `gh` é usado na redação destes artefatos; capturas visuais e execução ao vivo ficam **[NV]**.
- O scorecard **192/625** é a escolha do ciclo 051: frente de derrota do GDD, agora factível após Nós/vitória (050) e controle de ilha (049); Papéis W/E/R + níveis e PVE/PNJs (48/625 cada) ficam para ciclos seguintes e a rede (10/625) segue bloqueada por ADR-004 até decisão da SE.

## Out of scope

Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md` e documentos derivados; `Lightyear`, `async`, sockets, TCP, servidor, segundo jogador humano; mapa real, rotas, coordenadas de ilha, simulação da Corporação, reconversão em 20 s, Desafio de Ilha, Plano Mestre, Nós/Resistência novos, variante duas Comunas, papéis W/E/R, níveis, PVE/PNJs, novas teclas, novos comandos, novos sinais; promoção de qualquer número a cânone; execução de gates, build, testes, formatadores, commits, push, PRs, merge ou deploy; alterações preexistentes fora da fronteira. O que não foi executado permanece **[NV]** até verificação pelo orquestrador.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: Derrota por timeout/reconquista **4×3×4×4 = 192/625**, contra Papéis W/E/R + níveis **4×2×2×3 = 48/625**, PVE/PNJs **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. Registro em `.specify/memory/STATE.md` (2026-11-23T14:25Z). Nenhum manifesto é criado e nenhum número do protótipo é promovido a cânone.
