# Feature Specification: Combate e Fôlego determinísticos

**Branch**: `047-combate-folego` (base `0ac4c9f`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T073500Z-combate-folego`, `insurgencia - combate e folego`.

**Âncora**: decisão 80/20 do ciclo 047 registrada em `.specify/memory/STATE.md` (2026-11-23T07:35Z): Combate/Fôlego **4×2×2×3 = 48/625**, escolhida como última frente de produto sem mudança de stack contra rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. **Todos os valores de combate desta spec são `[HIPÓTESE]` ou escolha de protótipo e nenhum deve ser lido como cânone**: GDD §10.1 fixa Fôlego máximo **100**, regeneração **10/s fora de combate** e **5/s em combate**, alcance contato **2 m** — todos marcados `[HIPÓTESE]` na fonte; custo de ataque **30**, dano **10** e duração de combate **90 ticks (3 s a 1/30 s)** são escolhas de protótipo deste ciclo, não dogmas canônicos. A conversão de 10/s em +1 a cada **3 ticks** e de 5/s em +1 a cada **6 ticks** é apenas a discretização dessas hipóteses a 30 Hz.

## User Scenarios

### US1 — Fôlego, custo de ataque e dano (P1)

O núcleo determinístico dá efeito à ação `atacar_forte`, já presente em `ACOES_PERMITIDAS`: o atacante paga **30** de Fôlego (`[HIPÓTESE]` de protótipo) e causa **10** de dano (`[HIPÓTESE]` de protótipo) ao alvo, aplicado com `saturating_sub` para que a vida nunca fique negativa. O ataque entra em combate por **90 ticks** (`[HIPÓTESE]` de protótipo). Nenhum desses números é apresentado como cânone.

**Why this priority**: é o efeito verificável do ciclo e o que transforma `atacar_forte` de uma entrada inerte do contrato em comportamento observável; sem custo, dano e estado de combate discretizados não há como comparar duas execuções (Princípios V/VI).

**Independent Test**: injetar dois jogadores no mesmo ponto, aplicar um `atacar_forte` e conferir que o atacante perde 30 de Fôlego, o alvo perde 10 de vida (com `saturating_sub` no piso) e ambos passam a ter `combate_ticks = 90`.

**Acceptance Scenarios**:
1. **Given** um jogador com `folego = 100`, **When** um `atacar_forte` válido é aceito, **Then** seu `folego` passa a `70` e `combate_ticks` a `90` (valores `[HIPÓTESE]`/protótipo).
2. **Given** um alvo com `vida = 5`, **When** recebe dano de ataque `10`, **Then** `vida` vira `0` por `saturating_sub`, sem pânico nem valor negativo.
3. **Given** o mesmo input repetido em duas execuções, **When** os ticks são comparados, **Then** Fôlego, vida e contagens de combate são idênticos.

### US2 — Estado do Jogador e constantes documentadas (P1)

`Jogador` passa a expor `folego`, `folego_ticks` e `combate_ticks` (todos `i64`, documentados com `///`), refletidos no hash e na criação inicial com Fôlego cheio. Constantes públicas e documentadas registram as hipóteses: `FOLEGO_MAXIMO`, `CUSTO_ATAQUE_FORTE`, `DANO_ATAQUE_FORTE`, `TICKS_COMBATE`, `ALCANCE_CONTATO_M`, `FOLEGO_REGENERACAO_FORA` e `FOLEGO_REGENERACAO_COMBATE`.

**Why this priority**: sem estado explícito e constantes nomeadas os valores ficam espalhados em literais, a regeneração não tem acumulador e nenhuma regra pode ser exercitada de forma reproduzível; o padrão segue o já usado por coleta, construção e Propaganda (Princípio III).

**Independent Test**: criar um `Mundo`, inspecionar um jogador recém-inserido (`folego = 100`, `folego_ticks = 0`, `combate_ticks = 0`) e confirmar que as sete constantes existem com documentação e que os campos aparecem no hash.

**Acceptance Scenarios**:
1. **Given** um `Mundo` novo, **When** um jogador é inserido, **Then** `folego == FOLEGO_MAXIMO`, `folego_ticks == 0` e `combate_ticks == 0`.
2. **Given** os itens públicos novos, **When** o código é revisado, **Then** todos têm `///` e as constantes trazem a marcação de que os valores são `[HIPÓTESE]`/protótipo.
3. **Given** `atacar_forte`, **When** `ACOES_PERMITIDAS` é consultada, **Then** a ação já consta na sim, sem acrescentar entrada nova em `crates/contratos`.

### US3 — Validação de alvo, alcance, Fôlego e deduplicação (P1)

`atacar_forte` é recusado quando não há jogador no `destino` informado ou quando o alvo está além do alcance contato (`[HIPÓTESE]` de **2 m**) — `CodigoRecusa::AlvoInvalido` — e quando o atacante tem Fôlego abaixo do custo (`[HIPÓTESE]` de **30**) — `CodigoRecusa::FolegoInsuficiente`. O `peer_id` entra no `BTreeSet<u64>` de atacantes atendidos no tick corrente **somente depois** de todas as validações (destino/alvo, alcance e Fôlego), garantindo **no máximo um ataque por jogador por tick** e que uma recusa não consuma a cota de um comando válido no mesmo tick; a validação do alvo usa o mesmo critério de ponto exato já adotado para coleta e construção.

**Why this priority**: sem deduplicação por atacante, comandos repetidos drenariam Fôlego e causariam dano múltiplo no mesmo tick, quebrando o determinismo; sem validação de alvo/alcance/Fôlego o ataque agiria à distância e sem custo (Princípio VI).

**Independent Test**: enviar dois `atacar_forte` do mesmo jogador no mesmo tick e conferir dano e custo de apenas um; enviar com alvo a 3 m (fora do alcance hipotético de 2 m) e com `folego = 20` e conferir `AlvoInvalido` e `FolegoInsuficiente`, respectivamente, sem mutar estado.

**Acceptance Scenarios**:
1. **Given** dois comandos `atacar_forte` do mesmo `peer_id` no mesmo tick, **When** o tick é processado, **Then** o alvo sofre dano uma vez e o atacante paga o custo uma vez (`BTreeSet<u64>`).
2. **Given** um jogador a mais de **2 m** (`[HIPÓTESE]`) do destino informado, **When** o ataque é validado, **Then** retorna `CodigoRecusa::AlvoInvalido` e nada muda.
3. **Given** `folego = 20` (abaixo do custo hipotético 30), **When** o ataque é validado, **Then** retorna `CodigoRecusa::FolegoInsuficiente`, sem dano e sem pânico.

### US4 — Regeneração acumulada em `fase_finish` (P1)

A regeneração acontece em `fase_finish` com `folego_ticks` como acumulador: fora de combate, **10/s** (`[HIPÓTESE]`) viram **+1 Fôlego a cada 3 ticks**; em combate, **5/s** (`[HIPÓTESE]`) viram **+1 a cada 6 ticks**. Enquanto `combate_ticks > 0` vale a taxa de combate e a contagem decresce de 1 em 1 por tick; ao zerar, volta a valer a taxa fora de combate. O Fôlego nunca ultrapassa `FOLEGO_MAXIMO`.

**Why this priority**: é a disciplina de tick que garante que as hipóteses de 10/s e 5/s sejam observáveis em 30 Hz sem relógio externo, e que impediria que um jogador com Fôlego cheio "guardasse" acumulado para estourar o teto.

**Independent Test**: conferir que 2 ticks fora de combate não regeneram, o 3.º regenera +1, e que em combate apenas o 6.º tick regenera; verificar que o acumulador não passa do teto e que `combate_ticks` cai de 90 para 0 em 90 ticks.

**Acceptance Scenarios**:
1. **Given** `folego = 50`, `folego_ticks = 0`, `combate_ticks = 0`, **When** 2 ticks de `fase_finish` passam, **Then** `folego` continua `50` e `folego_ticks = 2`; no 3.º, `folego = 51` e `folego_ticks = 0`.
2. **Given** um jogador em combate (`combate_ticks > 0`), **When** 5 ticks passam, **Then** não há regeneração; no 6.º, `+1` e o acumulador zera.
3. **Given** `folego = FOLEGO_MAXIMO`, **When** vários ticks de regeneração passam, **Then** o valor permanece no teto, sem estouro.

### US5 — Determinismo, ordem estável e hash (P1)

`hash_estado()` passa a incluir `folego`, `folego_ticks` e `combate_ticks` de cada jogador, em ordem estável de `BTreeMap` por `peer_id`; duas execuções com a mesma sequência de comandos produzem exatamente o mesmo estado e hash a cada tick, sem `HashMap`, relógio, aleatoriedade ou iteração instável.

**Why this priority**: o servidor headless emite hashes por tick; qualquer campo novo fora do hash escaparia da detecção de divergência (Princípio VI).

**Independent Test**: executar duas vezes a mesma sequência (dois jogadores, ataque, regeneração, fim de combate) e comparar `Mundo::jogadores` e `hash_estado` tick a tick; atacar em uma delas e conferir divergência do hash.

**Acceptance Scenarios**:
1. **Given** duas execuções com o mesmo input, **When** os hashes são comparados em cada tick, **Then** são idênticos.
2. **Given** a mutação de `folego`, `folego_ticks` ou `combate_ticks`, **When** o hash é recalculado, **Then** ele difere do hash anterior.
3. **Given** dois jogadores com `peer_id` distintos, **When** o hash é montado, **Then** a contribuição segue a ordem de `BTreeMap`, não a ordem de inserção.

### US6 — Cliente/HUD: tecla `Q` e leitura de `Fôlego`/`Vida` (P2)

O cliente envia `atacar_forte` com a tecla **`Q`** (`KeyCode::KeyQ` em `just_pressed`) e passa a exibir **`Fôlego`** e **`Vida`** do jogador local no HUD, ao lado das informações já existentes. Nenhum campo novo entra no protocolo, nenhuma tecla existente é removida e a exibição é somente leitura do estado já replicado.

**Why this priority**: sem a tecla a ação não é acessível e sem os dois rótulos o jogador não percebe custo, exaustão ou dano sofrido; a leitura é feita do estado já presente, sem nova superfície de contrato.

**Independent Test**: apertar `Q` no cliente e conferir o comando gerado; alterar `folego`/`vida` na sim e verificar a atualização dos rótulos no HUD; confirmar que `crates/contratos` e `PROTOCOL_VERSION` permanecem intactos.

**Acceptance Scenarios**:
1. **Given** o cliente em execução, **When** `Q` é pressionado, **Then** um comando `atacar_forte` é enfileirado sem remover `B`/`P`/`C`/`G`.
2. **Given** um jogador com `folego = 70` e `vida = 90`, **When** o HUD é renderizado, **Then** exibe `Fôlego: 70` e `Vida: 90` sem campo de protocolo novo.
3. **Given** o estado final do ciclo, **When** os arquivos de contrato são conferidos, **Then** `crates/contratos`, `PROTOCOL_VERSION` e `Cargo.toml` permanecem intactos.

### US7 — Fronteira de stack e governança (P2)

A implementação permanece no core determinístico e no cliente já existentes, sem Lightyear, async, sockets, rede TCP, janela, nova dependência ou alteração de contrato versionado; R6 é **N/A** por constituição v1.2.2 e R4 precede R7 `1.0.0-local`. Nenhum valor de combate é promovido a cânone.

**Why this priority**: preserva os Princípios II/III/IV, impede que um recurso local reabra a rede bloqueada por ADR-004 e impede que hipóteses de design virem requisito canônico.

**Independent Test**: inspecionar os arquivos alterados e os artefatos SDD, conferindo ausência de mudança em contrato/protocolo/Cargo e presença explícita de R6 N/A, da ordem R4 → R7 e da rotulagem `[HIPÓTESE]` de todos os valores.

**Acceptance Scenarios**:
1. **Given** o estado final do ciclo, **When** a fronteira é revisada, **Then** somente os paths de código atribuídos na etapa 12 e os 4 artefatos SDD foram tocados.
2. **Given** R6, **When** o ciclo é revisado, **Then** consta como N/A (não executada, não simulada), com PARE se invocada e nenhuma issue criada.
3. **Given** cada número de combate, **When** a spec é lida, **Then** ele aparece marcado como `[HIPÓTESE]` ou escolha de protótipo, nunca como cânone.

### Edge Cases
- `folego = 30` exatamente: ataque aceito, Fôlego zera; `folego = 29`: `FolegoInsuficiente`.
- `folego = 10`, dano de ataque: vida vai a `0` por `saturating_sub`; novos ataques ao mesmo alvo não produzem valor negativo.
- Dois comandos `atacar_forte` do mesmo atacante no mesmo tick: custo e dano aplicados uma única vez (`BTreeSet<u64>` de peer_ids).
- Dois atacantes distintos no mesmo tick, cada um com seu alvo: ambos atacam (dedup é por atacante, não global).
- Comando `atacar_forte` inválido (destino/alvo, alcance ou Fôlego) seguido de um comando válido do mesmo `peer_id` no mesmo tick: a recusa não registra o atacante no `BTreeSet<u64>`; o comando válido aplica custo e dano normalmente — a cota de ataque do tick não é consumida por comando inválido.
- Cliente standalone (`PEER_ID = 1` em `crates/cliente/src/main.rs`): sem segundo jogador/spawn não há alvo no `destino`, logo a tecla `Q` gera o comando validado pelo core, mas o HUD não mostra dano; a verificação do cliente é estrutural, não visual.
- Alvo no `destino` mas a mais de **2 m** (`[HIPÓTESE]`) do atacante: `AlvoInvalido`, sem custo de Fôlego.
- `destino` sem jogador correspondente: `AlvoInvalido`, sem pânico.
- `combate_ticks = 1`: o último tick decrementa para 0; a regeneração seguinte já usa a taxa fora de combate.
- `folego_ticks = 2` fora de combate e entrada em combate no tick seguinte: o acumulador é preservado e, sob a taxa de combate, passa a exigir 6 ticks para o próximo `+1` — comportamento congelado nesta spec.
- `folego = FOLEGO_MAXIMO`: acumulador não guarda crédito para estourar o teto.
- Nenhum jogador com Fôlego para atacar: sim segue determinística e o HUD exibe `Fôlego: 100` / `Vida: 100` iniciais.
- Comando `atacar_forte` com `seq`/`request_id` em replay: recusado antes do custo e do dano.
- Mudança de stack, protocolo ou contrato (`crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`): fora de fronteira, nenhum arquivo desses é alterado.

## Requirements

### Functional Requirements

- **FR-001 (regra de Fôlego e combate)**: a ação `atacar_forte` — já listada em `ACOES_PERMITIDAS` na sim — passa a custar **30** de Fôlego (`[HIPÓTESE]` de protótipo), causar **10** de dano (`[HIPÓTESE]` de protótipo) aplicado com `saturating_sub` e colocar atacante e alvo em combate por **90 ticks** (`[HIPÓTESE]` de protótipo) a 1/30 s (`PASSO_SEGUNDOS`). A regeneração é **10/s** fora de combate e **5/s** em combate (GDD §10.1, ambos `[HIPÓTESE]`), discretizada como **+1 a cada 3 ticks** e **+1 a cada 6 ticks**, respectivamente; o alcance contato é **2 m** (`[HIPÓTESE]`). **Nenhum desses valores é cânone**: todos são hipóteses de design ou escolhas de protótipo registradas como limitação.
- **FR-002 (contrato/estado)**: dar efeito a `atacar_forte` **somente na sim**, sem acrescentar entrada a `crates/contratos`; `Jogador` ganha `folego: i64`, `folego_ticks: i64` e `combate_ticks: i64`, documentados com `///` e inicializados com `FOLEGO_MAXIMO`/`0`/`0`; constantes públicas documentadas `FOLEGO_MAXIMO = 100`, `CUSTO_ATAQUE_FORTE = 30`, `DANO_ATAQUE_FORTE = 10`, `TICKS_COMBATE = 90`, `ALCANCE_CONTATO_M = 2.0`, `FOLEGO_REGENERACAO_FORA = 10`, `FOLEGO_REGENERACAO_COMBATE = 5`; variantes `CodigoRecusa::FolegoInsuficiente` e `CodigoRecusa::AlvoInvalido` quando necessárias. **Nenhuma mudança** em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto ou CI; nenhum novo código de protocolo; sem Lightyear, async, sockets, janela ou relógio externo.
- **FR-003 (comportamento/fase)**: validar o alvo no `destino` informado por ponto exato (`ponto_igual`) e por distância ≤ `ALCANCE_CONTATO_M` (`[HIPÓTESE]` de 2 m), e o Fôlego do atacante ≥ `CUSTO_ATAQUE_FORTE`; falha de alvo/alcance retorna `AlvoInvalido` e falha de Fôlego retorna `FolegoInsuficiente`, ambas sem mutar estado. Um `BTreeSet<u64>` de peer_ids de atacantes, por tick, limita a **um ataque por jogador por tick**; dano com `saturating_sub`; `combate_ticks = 90` em atacante e alvo; processamento em ordem estável de `BTreeMap`, sem relógio, aleatoriedade ou efeito colateral em barricadas, pontes, súditos, coleta, construção ou autogestão.
- **FR-004 (hash/determinismo)**: `hash_estado()` inclui `folego`, `folego_ticks` e `combate_ticks` de cada `Jogador` em ordem estável de `BTreeMap`, além dos campos já presentes; estados idênticos geram hash idêntico; ataque, dano, regeneração e fim de combate alteram o hash; duas execuções com o mesmo input produzem o mesmo estado e hash, sem `HashMap`, relógio, aleatoriedade ou ordem instável.
- **FR-005 (cliente/HUD)**: a tecla **`Q`** (`KeyCode::KeyQ`, `just_pressed`) envia `atacar_forte`; o HUD exibe **`Fôlego`** e **`Vida`** do jogador local ao lado das informações já existentes, sem novo campo de protocolo e sem remover teclas existentes (`B`/`P`/`C`/`G`/WASD); a exibição é somente leitura do estado já replicado.
- **FR-006 (governança/testes)**: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`, `///` em itens públicos; testes determinísticos cobrindo custo 30, dano com `saturating_sub` no piso, 90 ticks de combate, regeneração em 3/6 ticks, teto de Fôlego, deduplicação por `BTreeSet`, alvo inválido, Fôlego insuficiente, replay, ordem, hash, tecla `Q` e HUD; R1 `go`, R6 **N/A** por v1.2.2, R4 antes de R7 `1.0.0-local`; commits em PT-BR com staging explícito; gates, build, testes, formatadores, commit/push/PR **[NV]** para este agente.

### Key Entities

- **Jogador**: struct pública em `crates/sim` que ganha `folego`, `folego_ticks` e `combate_ticks` mantendo `vida`, `posicao`, `seq`, `request_id` e os campos de coleta/construção/pontes/propaganda intactos.
- **`atacar_forte`**: ação já presente em `ACOES_PERMITIDAS` na sim; ganha efeito (custo, dano, combate) sem mudança em `crates/contratos` ou `PROTOCOL_VERSION`.
- **Constantes hipotéticas**: `FOLEGO_MAXIMO = 100`, `CUSTO_ATAQUE_FORTE = 30`, `DANO_ATAQUE_FORTE = 10`, `TICKS_COMBATE = 90`, `ALCANCE_CONTATO_M = 2.0`, `FOLEGO_REGENERACAO_FORA = 10`, `FOLEGO_REGENERACAO_COMBATE = 5` — todas `[HIPÓTESE]` ou escolha de protótipo, não cânone.
- **`fase_finish`**: ponto de regeneração com acumulador `folego_ticks` (+1/3 ticks fora de combate, +1/6 em combate) e decaimento de `combate_ticks`.
- **`CodigoRecusa`**: ganha `FolegoInsuficiente` e `AlvoInvalido` para as recusas específicas do ataque.
- **Hash do estado**: assinatura SHA-256 de `Mundo`; passa a incluir os três campos novos de cada jogador em ordem de `BTreeMap`.
- **HUD `Fôlego`/`Vida` e tecla `Q`**: único acréscimo visível e de input; nenhum campo novo de protocolo.

## Clarificações

- GDD §10.1 é fonte das hipóteses, **não de cânone neste ciclo**: Fôlego máximo 100, regeneração 10/s fora de combate e 5/s em combate, alcance contato 2 m — todos já marcados `[HIPÓTESE]` na fonte. Custo 30, dano 10 e combate de 90 ticks são escolhas de protótipo do ciclo 047. Nenhum valor pode ser apresentado como cânone.
- A conversão 10/s → +1 a cada 3 ticks e 5/s → +1 a cada 6 ticks decorre de `PASSO_SEGUNDOS = 1/30`; é discretização de hipótese, não regra canônica.
- O combate é representado por `combate_ticks` no estado, no hash e na taxa de regeneração; **nenhum sistema de física, projétil, animação ou habilidade completa é simulado** — apenas o ataque forte determinístico.
- Dano é aplicado no alvo por `saturating_sub` sobre `vida`; a API de dano de Súditos do ciclo 046 permanece como está e não é reaberta.
- Alvo é identificado pelo `destino` do comando: exige jogador nesse ponto (`ponto_igual`) e distância ≤ 2 m (`[HIPÓTESE]`) do atacante. Sem alvo ou fora de alcance, `AlvoInvalido`.
- Deduplicação por `BTreeSet<u64>` de **peer_ids dos atacantes**: no máximo um ataque por jogador por tick; atacantes distintos não se bloqueiam. O registro ocorre após todas as validações, então uma recusa não consome a cota do tick.
- A vedação a `unwrap`/`expect`/`#[allow]` não atinge o atributo de lint `#[expect(clippy::needless_pass_by_value, reason = "...")]`: é atributo permitido, distinto da chamada de método `.expect()`; a vedação a `expect` refere-se a `.expect()`.
- As hipóteses de progressão do GDD (desbloqueio Q/W/E/R, teto de nível 12, alcances curto 6 m/médio 12 m/ilha 25 m, funções e ultimate) ficam **fora de escopo**; este ciclo usa apenas `Q` como atalho do ataque forte.
- Súditos/Propaganda (ciclo 046), barricada (044), ponte provisória (045) e autogestão permanecem como estão; nenhum efeito colateral nesses subsistemas.
- Sem mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto ou CI; ADR-004 mantém a rede bloqueada até decisão da SE.
- **R6 é N/A** por constituição v1.2.2: não é dívida executada, nem regra ativa, nem simulação de issue; se invocada, PARE e escale ao SWE; a dívida de automação (`resolve-tasks.sh`/adaptador) fica registrada sem `gh` improvisado.
- Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0); rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança.

## Success Criteria

- **SC-001 (regra de Fôlego/combate)**: o ataque custa 30, causa 10 de dano com `saturating_sub` e instaura 90 ticks de combate; a regeneração observada é de +1 a cada 3 ticks fora de combate e +1 a cada 6 ticks em combate, com teto de 100 e alcance de 2 m — cada número marcado como `[HIPÓTESE]`/escolha de protótipo e **nenhum apresentado como cânone**.
- **SC-002 (contrato/estado)**: `atacar_forte` segue em `ACOES_PERMITIDAS` sem mudança em `crates/contratos`; `Jogador` tem `folego`, `folego_ticks` e `combate_ticks` documentados; as sete constantes e as variantes `FolegoInsuficiente`/`AlvoInvalido` existem; `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto e CI permanecem intactos; nenhuma dependência, Lightyear, async, socket ou janela é introduzida.
- **SC-003 (comportamento/fase)**: alvo validado no ponto e no alcance hipotético de 2 m; Fôlego insuficiente recusado; no máximo um ataque por jogador por tick (`BTreeSet<u64>`); dano satura no piso; combate de 90 ticks decai um por tick; regeneração ocorre exatamente nos ticks 3/6, em ordem estável de `BTreeMap`, sem relógio/aleatoriedade e sem afetar outros subsistemas.
- **SC-004 (hash/determinismo)**: `hash_estado` muda com ataque, dano, regeneração e fim de combate, é idêntico entre duas execuções com o mesmo input e inclui os três campos em ordem de `BTreeMap`.
- **SC-005 (cliente/HUD)**: a tecla `Q` gera `atacar_forte` e o HUD exibe `Fôlego` e `Vida`, sem novo campo de protocolo e sem remover teclas existentes.
- **SC-006 (governança/testes)**: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]` (atributo de lint `#[expect(...)]` permitido), `max_width = 100`, testes determinísticos cobrindo os casos limítrofes; R1 `go`, R6 N/A, R4 antes de R7 `1.0.0-local`; gates, build, testes, formatadores e publicação ficam **[NV]** até execução pelo orquestrador.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam o código Rust na etapa 12; o orquestrador executa gates, build e verificações finais.
- Ambiente headless: nenhuma janela, rede, credencial ou comando `gh` é usado na redação destes artefatos.
- A discretização a 30 Hz (`PASSO_SEGUNDOS = 1/30`) é estável para o alvo; hipóteses não são promovidas a requisito e nenhum número é tratado como cânone.
- O core já expõe `Mundo`, `Jogador`, `ACOES_PERMITIDAS` (com `atacar_forte`), `fase_finish`, `hash_estado`, `BTreeMap`, `BTreeSet`, `ponto_igual` e `CodigoRecusa`, e o cliente já lê o estado e trata teclas em `crates/cliente/src/main.rs`; nenhum novo crate é necessário.
- O scorecard **48/625** é a escolha do ciclo 047 porque a única alternativa de produto restante é a rede (10/625), bloqueada por ADR-004 até decisão da SE; as frentes 256/625 (ciclo 044), 108/625 (045) e 48/625 (046) já foram entregues.

## Out of scope

Progressão de nível, desbloqueio Q/W/E/R, alcances curto/médio/ilha, habilidades das funções, ultimate, física, projéteis, animação, efeitos visuais de combate, sistema de dano a Súditos (ciclo 046), barricada (044), ponte provisória (045), autogestão, rede (TCP/Lightyear/async/sockets), mudança de stack/dependências/protocolo/contrato (`crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`), janelas, deploy, commits/push/PRs e alterações preexistentes fora da fronteira. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: Combate/Fôlego **4×2×2×3 = 48/625**, escolhida sobre rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. As frentes 256/625 (ciclo 044), 108/625 (045) e 48/625 (046) já foram entregues, logo não há reabertura das decisões anteriores. Nenhum manifesto é criado e nenhum valor de combate é promovido a cânone.
