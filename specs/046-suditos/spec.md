# Feature Specification: Súditos e Propaganda determinísticos

**Branch**: `046-suditos` (base `d410bd6`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T065800Z-suditos`, `insurgencia - suditos e propaganda`.

**Âncora**: decisão 80/20 do ciclo 046 registrada em `.specify/memory/STATE.md` (2026-11-23T06:58Z): Súditos/Propaganda **4×2×2×3 = 48/625**, escolhida como próximo recurso canônico implementável após a ponte provisória (108/625) entregue no ciclo 045 e a defesa de barricada (256/625) entregue no ciclo 044, contra Combate/Fôlego **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. Regra canônica: GDD §9.2:464 fixa que um Súdito inimigo exposto **4 s (120 ticks a 1/30 s)** à Propaganda para de atacar estruturas e passa a panfletar, e que, se levar dano de jogador, **reverte em 3 s (90 ticks)**. Spawn por pulso de 30 s, raio de 6 m e funções trabalhador/propagandista/miliciano são `[HIPÓTESE]` e ficam **fora de escopo**, registradas como limitações.

## User Scenarios

### US1 — Conversão por exposição e reversão por dano (P1)

O núcleo determinístico aplica a regra canônica de GDD §9.2:464: um Súdito inimigo acumula exposição à Propaganda até **4 s = 120 ticks** e, ao atingir esse limiar, deixa de atacar estruturas e passa a panfletar (representado pela transição de estado `convertido = true`). Se o Súdito levar dano de jogador, a conversão **reverte em 3 s = 90 ticks**, voltando à condição anterior.

**Why this priority**: é o critério de aceitação canônico da seção de Súditos do GDD e o efeito verificável do ciclo; sem os prazos exatos de 120/90 ticks a regra não é observável nem comparável entre execuções (Princípios V/VI).

**Independent Test**: injetar um Súdito, submeter 119 ticks de Propaganda (ainda não convertido) e o 120.º (convertido); em seguida aplicar dano e conferir 89 ticks ainda convertido e o 90.º revertido, comparando duas execuções idênticas.

**Acceptance Scenarios**:
1. **Given** um Súdito inimigo com `propaganda_ticks = 0`, **When** 119 ticks de exposição são processados, **Then** `convertido` permanece `false` e `propaganda_ticks == 119`.
2. **Given** `propaganda_ticks == 119`, **When** o 120.º tick de exposição é processado, **Then** `convertido` passa a `true` — o Súdito para de atacar estruturas e passa a panfletar no estado.
3. **Given** um Súdito convertido, **When** `aplicar_dano_a_sudito(id)` é chamado, **Then** `reversao_ticks` é fixado em **90** e o Súdito continua convertido até a contagem zerar.
4. **Given** `reversao_ticks == 1`, **When** o último tick de reversão é processado, **Then** `convertido` volta a `false`, `propaganda_ticks` zera e `reversao_ticks` zera.

### US2 — Estado do Súdito e injeção determinística (P1)

O `Mundo` passa a manter os Súditos em `BTreeMap<u64, Sudito>`, com struct pública `Sudito { id, posicao, convertido, propaganda_ticks, reversao_ticks }`. A API `adicionar_sudito(id, posicao)` permite injeção determinística em testes e demos, e `aplicar_dano_a_sudito(id)` simula o dano de jogador sem introduzir sistema de combate.

**Why this priority**: sem um estado explícito e uma via de injeção determinística a regra canônica não pode ser exercitada de forma reproduzível; o padrão `BTreeMap` espelha barricadas e pontes, evitando divergência de convenção (Princípio III).

**Independent Test**: injetar dois Súditos com IDs distintos, inspecionar o estado e confirmar iteração ordenada por `id`; chamar `aplicar_dano_a_sudito` em um deles e conferir que o outro permanece intacto.

**Acceptance Scenarios**:
1. **Given** um `Mundo` novo, **When** `adicionar_sudito(7, (x, y))` é chamado, **Then** `Mundo::suditos` contém a chave `7` com `convertido = false`, `propaganda_ticks = 0`, `reversao_ticks = 0` e a posição fornecida.
2. **Given** Súditos com IDs 7 e 2, **When** o estado é iterado, **Then** a ordem é `2, 7` por `BTreeMap`.
3. **Given** `aplicar_dano_a_sudito(99)` com ID inexistente, **When** a chamada é feita, **Then** nenhum estado muda e nenhum pânico ocorre.

### US3 — Auto-avanço determinístico por comando (P1)

A ação `propaganda` avança a exposição de cada Súdito alvejado em **no máximo 1 tick por Súdito por tick**. Um `BTreeSet<u64>` rastreia os IDs já avançados no tick corrente, de modo que comandos duplicados não acumulem exposição extra; a reversão avança no mesmo ritmo, em ordem estável de ID, sem relógio externo, aleatoriedade ou efeito colateral em outras entidades.

**Why this priority**: a disciplina de tick é o que garante que 4 s signifiquem 120 ticks em qualquer execução; deduplicar por `BTreeSet` impede que flood de comandos converta um Súdito antes do tempo canônico (Princípio VI).

**Independent Test**: enviar dois comandos `propaganda` para o mesmo Súdito no mesmo tick e conferir `propaganda_ticks == 1`, não `2`; repetir por 120 ticks e confirmar conversão exatamente no 120.º.

**Acceptance Scenarios**:
1. **Given** dois comandos `propaganda` para o mesmo ID no mesmo tick, **When** o tick é processado, **Then** `propaganda_ticks` incrementa exatamente **1**.
2. **Given** Súditos convertidos com `reversao_ticks > 0`, **When** um tick é processado, **Then** cada contagem decrementa exatamente 1, em ordem de `BTreeMap`, sem afetar Súditos não convertidos.
3. **Given** duas execuções com a mesma sequência de comandos, **When** os ticks são comparados, **Then** exposição, conversão e reversão são idênticas em todos os ticks.

### US4 — Determinismo, ordem estável e hash (P1)

O estado dos Súditos é refletido em `Mundo::hash_estado()` na ordem de `BTreeMap` por `id`; duas execuções com a mesma sequência de comandos produzem exatamente o mesmo hash a cada tick, sem `HashMap`, relógio, aleatoriedade ou iteração instável.

**Why this priority**: o servidor headless emite hashes por tick; qualquer campo novo fora do hash escaparia da detecção de divergência (Princípio VI).

**Independent Test**: executar duas vezes a mesma sequência (dois Súditos injetados, exposição, dano e reversão) e comparar `Mundo::suditos` e `hash_estado` tick a tick.

**Acceptance Scenarios**:
1. **Given** duas execuções com o mesmo input, **When** os hashes são comparados em cada tick, **Then** são idênticos.
2. **Given** a injeção de um Súdito, **When** o hash é recalculado, **Then** ele difere do hash anterior.
3. **Given** a conversão ou a reversão de um Súdito, **When** o hash é recalculado, **Then** ele difere do hash do tick anterior.

### US5 — HUD: contagem de convertidos e tecla `G` (P2)

O cliente passa a enviar `propaganda` com a tecla **`G`** (`KeyCode::KeyG` em `just_pressed`) e exibe **Convertidos** com a contagem de Súditos convertidos do estado replicado (filtro por `convertido` em `Mundo::suditos`), ao lado de **Pontes**. Nenhum campo novo entra no protocolo e nenhuma tecla existente é removida.

**Why this priority**: sem a tecla a ação não é acessível e sem a contagem o jogador não percebe o efeito da conversão; a leitura é feita diretamente do estado já replicado, sem nova superfície de contrato.

**Independent Test**: apertar `G` no cliente e conferir o comando gerado; converter um Súdito na sim e verificar o incremento de `Convertidos` no HUD; confirmar que `crates/contratos` e `PROTOCOL_VERSION` permanecem intactos.

**Acceptance Scenarios**:
1. **Given** o cliente em execução, **When** `G` é pressionado, **Then** um comando `propaganda` é enfileirado.
2. **Given** um Súdito convertido no estado, **When** o HUD é renderizado, **Then** exibe `Convertidos: 1` sem campo de protocolo novo.
3. **Given** o estado final do ciclo, **When** os arquivos de contrato são conferidos, **Then** `crates/contratos`, `PROTOCOL_VERSION` e `Cargo.toml` permanecem intactos.

### US6 — Fronteira de stack e governança (P2)

A implementação permanece no core determinístico e no cliente já existentes, sem Lightyear, async, sockets, rede TCP, janela, nova dependência ou alteração de contrato versionado; R6 é **N/A** por constituição v1.2.2 e R4 precede R7 `1.0.0-local`.

**Why this priority**: preserva os Princípios II/III/IV e impede que um recurso local reabra a rede bloqueada por ADR-004 ou promova `[HIPÓTESE]` a requisito.

**Independent Test**: inspecionar os arquivos alterados e os artefatos SDD, conferindo ausência de mudança em contrato/protocolo/Cargo e presença explícita de R6 N/A e da ordem R4 → R7.

**Acceptance Scenarios**:
1. **Given** o estado final do ciclo, **When** a fronteira é revisada, **Then** somente os paths de código atribuídos na etapa 12 e os 4 artefatos SDD foram tocados.
2. **Given** R6, **When** o ciclo é revisado, **Then** consta como N/A (não executada, não simulada), com PARE se invocada e nenhuma issue criada.

### Edge Cases
- `propaganda_ticks = 119`: ainda inimigo; tick 120 converte; ticks adicionais não acumulam além do limiar enquanto `convertido = true`.
- Dois comandos `propaganda` para o mesmo ID no mesmo tick: exposição incrementa 1 (`BTreeSet<u64>` deduplica por ID).
- Dano aplicado antes da conversão (`propaganda_ticks < 120`): `reversao_ticks` é registrado, mas não há conversão a desfazer; a exposição permanece e a conversão ocorre no prazo canônico se a Propaganda continuar.
- Dano aplicado a Súdito já convertido: `reversao_ticks = 90`; conversão persiste nos ticks 1..89 e cessa no tick 90.
- Novo dano durante a reversão: a contagem é reiniciada em 90 (monotônica por evento de dano, sem acumulação).
- `aplicar_dano_a_sudito` com ID inexistente: operação sem efeito, sem pânico.
- Nenhum Súdito injetado: `hash_estado` permanece determinístico e o HUD exibe `Convertidos: 0`.
- Dois Súditos: `id` explícitos distintos, hash em ordem de `BTreeMap`, sem limite arbitrário introduzido neste ciclo.
- Comando `propaganda` com `seq`/`request_id` em replay: recusado antes da contagem de exposição.
- Súdito convertido não gera renda, não credita autogestão e não altera barricadas, pontes ou vitória; renda de trabalhador (60 s) é `[HIPÓTESE]` fora de escopo.

## Requirements

### Functional Requirements

- **FR-001 (regra canônica)**: a exposição de um Súdito inimigo à Propaganda converte após **4 s = 120 ticks** a 1/30 s (GDD §9.2:464); convertido, ele **para de atacar estruturas e passa a panfletar**, representado pela transição de estado `convertido = true`; se levar **dano de jogador**, a conversão **reverte em 3 s = 90 ticks**. Spawn por pulso de 30 s, raio de 6 m, limite de 3 conversões, permanência de 60 s e funções trabalhador/propagandista/miliciano são `[HIPÓTESE]` e ficam **fora de escopo**, registradas como limitações.
- **FR-002 (contrato/estado)**: acrescentar `"propaganda"` a `ACOES_PERMITIDAS` **na sim**; struct pública `Sudito { id: u64, posicao: (f64, f64), convertido: bool, propaganda_ticks: i64, reversao_ticks: i64 }` em `BTreeMap<u64, Sudito>` no campo `Mundo::suditos`; APIs públicas e documentadas `adicionar_sudito(id, posicao)` (injeção determinística) e `aplicar_dano_a_sudito(id)` (dano de jogador); constantes públicas e documentadas `TICKS_PROPAGANDA = 120`, `TICKS_REVERSAO = 90`. **Nenhuma mudança** em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto ou CI; nenhum novo código de protocolo; sem Lightyear, async, sockets, janela ou relógio externo.
- **FR-003 (comportamento/fase)**: auto-avanço por comando com `BTreeSet<u64>` — cada Súdito alvejado incrementa `propaganda_ticks` no máximo **1 tick por Súdito por tick**, com deduplicação por ID; ao completar 120 ticks, `convertido = true`; `aplicar_dano_a_sudito(id)` fixa `reversao_ticks = 90` e, ao zerar, `convertido = false` e `propaganda_ticks = 0`; processamento em ordem estável de `BTreeMap`, sem relógio, aleatoriedade ou efeito colateral em jogadores, barricadas ou pontes.
- **FR-004 (hash/determinismo)**: `hash_estado()` inclui cada `Sudito` (`id`, `posicao`, `convertido`, `propaganda_ticks`, `reversao_ticks`) em ordem estável de `BTreeMap`, além dos campos já presentes; estados idênticos geram hash idêntico; injeção, conversão e reversão alteram o hash; duas execuções com o mesmo input produzem o mesmo estado e hash, sem `HashMap`, relógio, aleatoriedade ou ordem instável.
- **FR-005 (cliente/HUD)**: a tecla **`G`** (`KeyCode::KeyG`, `just_pressed`) envia `propaganda`; o HUD exibe **Convertidos** com a contagem de Súditos com `convertido = true` (filtro em `Mundo::suditos`) ao lado de **Pontes**, sem novo campo de protocolo e sem remover teclas existentes; a exibição é somente leitura do estado já replicado.
- **FR-006 (governança/testes)**: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`, `///` em itens públicos; testes determinísticos cobrindo 120 ticks, conversão no limiar, dano e 90 ticks de reversão, deduplicação por `BTreeSet`, ID inexistente, ordem, hash, tecla `G` e HUD; R1 `go`, R6 **N/A** por v1.2.2, R4 antes de R7 `1.0.0-local`; commits em PT-BR com staging explícito; gates, build, testes, formatadores, commit/push/PR **[NV]** para este agente.

### Key Entities

- **Sudito**: struct pública em `crates/sim` com `id`, `posicao`, `convertido`, `propaganda_ticks` e `reversao_ticks`; identificador fornecido na injeção, iteração ordenada por `id`.
- **Mundo**: ganha `suditos: BTreeMap<u64, Sudito>`, mantendo `pontes`, `barreiras`, `jogadores`, comandos e tick/epoch intactos.
- **Ação `propaganda`**: nova entrada em `ACOES_PERMITIDAS` da sim; sem mudança em `crates/contratos` ou `PROTOCOL_VERSION`.
- **APIs determinísticas**: `adicionar_sudito(id, posicao)` e `aplicar_dano_a_sudito(id)`, sem sistema de combate, física ou projétil.
- **Constantes canônicas**: `TICKS_PROPAGANDA = 120` (4 s) e `TICKS_REVERSAO = 90` (3 s), a 1/30 s (`PASSO_SEGUNDOS`).
- **Hash do estado**: assinatura SHA-256 de `Mundo`; passa a incluir os súditos em ordem de `BTreeMap`.
- **HUD `Convertidos` / tecla `G`**: único acréscimo visível e de input; nenhum campo novo de protocolo.

## Clarificações

- GDD §9.2:464 é canônica: **4 s = 120 ticks** e **3 s = 90 ticks** a 1/30 s (`PASSO_SEGUNDOS`). Nada disso é `[HIPÓTESE]`.
- O que é `[HIPÓTESE]` e fica fora de escopo: spawn a cada pulso de 30 s (2 por ilha convertida, 3 na Rota Central), raio de 6 m, conversão de até 3 Súditos por pulso, conservação da função por 60 s e as funções trabalhador/propagandista/miliciano.
- Conversão é representada no estado (`convertido = true`), no hash e no HUD; **nenhum sistema de ataque a estruturas nem de panfletar é simulado** neste ciclo — combate/Fôlego é candidato separado (48/625) e permanece fora de escopo. "Para de atacar e passa a panfletar" traduz-se na transição de estado, não em física ou projétil.
- Dano de jogador é exercitado pela API determinística `aplicar_dano_a_sudito(id)`; nenhum sistema de dano, projétil ou física é introduzido.
- Auto-avanço por um comando com `BTreeSet<u64>`: cada Súdito recebe no máximo 1 tick de exposição por tick, com IDs rastreados em `BTreeSet` e processados em ordem estável; comandos duplicados no mesmo tick não acumulam exposição extra.
- A vedação a `unwrap`/`expect`/`#[allow]` não atinge o atributo de lint `#[expect(clippy::needless_pass_by_value, reason = "...")]`: é atributo permitido, distinto da chamada de método `.expect()`; a vedação a `expect` refere-se a `.expect()`.
- Súdito convertido não credita pontos de autogestão, não modifica barricadas ou pontes e não altera condições de vitória; os ciclos 044 e 045 permanecem como estão.
- Sem mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto ou CI; ADR-004 mantém a rede bloqueada até decisão da SE.
- **R6 é N/A** por constituição v1.2.2: não é dívida executada, nem regra ativa, nem simulação de issue; se invocada, PARE e escale ao SWE; a dívida de automação (`resolve-tasks.sh`/adaptador) fica registrada sem `gh` improvisado.
- Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0); rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança.

## Success Criteria

- **SC-001 (regra canônica)**: 119 ticks mantêm o Súdito inimigo e o 120.º converte; o convertido para de atacar estruturas e passa a panfletar no estado; dano de jogador fixa 90 ticks e o Súdito reverte exatamente no 90.º; spawn/raio/funções `[HIPÓTESE]` estão registrados como fora de escopo.
- **SC-002 (contrato/estado)**: `ACOES_PERMITIDAS` contém `propaganda`, `Sudito` e `Mundo::suditos` existem com `BTreeMap<u64, Sudito>`, as APIs `adicionar_sudito`/`aplicar_dano_a_sudito` e as constantes `TICKS_PROPAGANDA = 120`, `TICKS_REVERSAO = 90` estão documentadas; `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto e CI permanecem intactos; nenhuma dependência, Lightyear, async, socket ou janela é introduzida.
- **SC-003 (comportamento/fase)**: cada Súdito alvejado avança no máximo 1 tick por tick com deduplicação por `BTreeSet<u64>`; a conversão ocorre no tick 120; o dano reinicia a reversão em 90 ticks e a conversão cessa ao zerar, em ordem estável de `BTreeMap`, sem relógio/aleatoriedade e sem afetar outros estados.
- **SC-004 (hash/determinismo)**: `hash_estado` muda com injeção, conversão e reversão, é idêntico entre duas execuções com o mesmo input e inclui os súditos em ordem de `BTreeMap`.
- **SC-005 (cliente/HUD)**: a tecla `G` gera `propaganda` e o HUD exibe `Convertidos` com a contagem do estado, sem novo campo de protocolo e sem remover teclas existentes.
- **SC-006 (governança/testes)**: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]` (atributo de lint `#[expect(...)]` permitido), `max_width = 100`, testes determinísticos cobrindo os casos limítrofes; R1 `go`, R6 N/A, R4 antes de R7 `1.0.0-local`; gates, build, testes, formatadores e publicação ficam **[NV]** até execução pelo orquestrador.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam o código Rust na etapa 12; o orquestrador executa gates, build e verificações finais.
- Ambiente headless: nenhuma janela, rede, credencial ou comando `gh` é usado na redação destes artefatos.
- Os valores canônicos (4 s = 120 ticks, 3 s = 90 ticks, 30 Hz) são estáveis para o alvo; hipóteses não são promovidas a requisito.
- O core já expõe `Mundo`, `Jogador`, `ACOES_PERMITIDAS`, `fase_finish`, `hash_estado`, `BTreeMap` e `BTreeSet`, e o cliente já lê o estado e trata teclas em `crates/cliente/src/main.rs`; nenhum novo crate é necessário.
- O scorecard 48/625 é a escolha do ciclo 046 porque as opções 256/625 (defesa de barricada, ciclo 044) e 108/625 (ponte provisória, ciclo 045) já foram entregues; não há reabertura das decisões anteriores.

## Out of scope

Spawn por pulso de 30 s, raio de 6 m, limite de 3 conversões, permanência de 60 s e funções trabalhador/propagandista/miliciano (`[HIPÓTESE]`), ataque real a estruturas e panfletar simulado, renda de trabalhador convertido, combate/Fôlego, barricada e autogestão (ciclo 044), ponte provisória (ciclo 045), rede (TCP/Lightyear/async/sockets), mudança de stack/dependências/protocolo/contrato (`crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`), janelas, deploy, commits/push/PRs e alterações preexistentes fora da fronteira. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: Súditos/Propaganda **4×2×2×3 = 48/625**, escolhida sobre Combate/Fôlego **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, denominador 625. As opções 256/625 (ciclo 044) e 108/625 (ciclo 045) já foram entregues. Nenhum manifesto é criado.
