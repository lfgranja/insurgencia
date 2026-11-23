# Feature Specification: Defesa de barricada → pontos de autogestão

**Branch**: `044-defesa-barricada` (base `b6d2f36`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T054700Z-defesa-barricada`, `insurgencia - defesa de barricada`.

**Âncora**: decisão 80/20 do ciclo 044 registrada em `.specify/memory/STATE.md` (2026-11-23T05:47Z): defesa de barricada gerando **+1 ponto de autogestão a cada 10 s de barricada intacta** (GDD §7.1:306, tabela «4. Defender») — **4×4×4×4 = 256/625** — contra ponte provisória **3×3×3×4 = 108/625**, Súditos/Propaganda (maioria `[HIPÓTESE]`) **4×2×2×3 = 48/625** e rede **bloqueada por ADR-004 até decisão da SE** **5×2×1×1 = 10/625**; critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. A barricada entregue no ciclo 040 tem **200 PV** e **12 s = 360 ticks**; o núcleo roda a **30 Hz (1 tick = 1/30 s)**, logo **10 s = 300 ticks** e uma barricada pode creditar **no máximo 1 ponto** (360 < 600). Protótipo: **o dono no mesmo ponto é o defensor**; os papéis Guerreiro/Artesão de GDD §7.1:306 ficam registrados como **simplificação de escopo**. Constituição **v1.2.2 do alvo**: **R6 é N/A** (fora da sequência executável até existir `resolve-tasks.sh` e adaptador de issues; se invocada, **PARE e escale ao SWE**, nunca simular issue, nunca `gh` improvisado; dívida de automação registrada). R1 `go`; R4 antes de R7 `1.0.0-local`.

## User Scenarios

### US1 — Crédito canônico por defesa contínua (P1)

O núcleo determinístico credita exatamente **+1 ponto de autogestão** ao jogador que mantém a barricada intacta e permanece no mesmo ponto por **300 ticks consecutivos (10 s a 1/30 s)**; o contador da barricada volta a zero no crédito.

**Why this priority**: é a regra canônica de GDD §7.1:306 e a única recompensa nova deste ciclo; sem ela a barricada do ciclo 040 permanece sem efeito de jogo e o loop de defesa não fecha (Princípios III/VI).

**Independent Test**: construir uma barricada, manter o dono no ponto e avançar 299 ticks (sem crédito) e mais 1 tick (crédito de +1 e contador zerado), comparando duas execuções idênticas.

**Acceptance Scenarios**:

1. **Given** uma `Barreira` com `pv > 0` e o dono em `posicao` igual à da barricada via `ponto_igual`, **When** 299 ticks são processados, **Then** `defesa_ticks == 299` e `pontos_autogestao` permanece inalterado.
2. **Given** o mesmo estado com `defesa_ticks == 299`, **When** o tick 300 é processado com a barricada ainda intacta e o dono no ponto, **Then** `pontos_autogestao` aumenta de 1 e `defesa_ticks` volta a 0.

### US2 — Interrupção por saída do dono (P1)

Se o dono sair do ponto da barricada — inclusive por um comando `mover` que altera a posição — o acúmulo zera e **não credita**; retornar ao ponto reinicia a contagem do zero.

**Why this priority**: «segar o ponto ou perseguir a emboscada» é a decisão do jogador em GDD §7.1:306; creditar progresso interrompido transformaria a regra em renda passiva e quebraria o contrato canônico.

**Independent Test**: acumular 200 ticks, mover o dono para outro ponto, conferir `defesa_ticks == 0` e nenhum crédito; retornar e confirmar que o acúmulo recomeça em 1.

**Acceptance Scenarios**:

1. **Given** `defesa_ticks == 200` e o dono no ponto, **When** `mover` altera a posição do dono, **Then** `defesa_ticks` zera e `pontos_autogestao` não muda.
2. **Given** o dono fora do ponto, **When** novos ticks são processados, **Then** nenhum acúmulo ocorre; ao retornar, a contagem começa do zero.
3. **Given** um `mover` cujo `destino` é igual à posição atual por `ponto_igual`, **When** o comando é processado, **Then** a defesa **não** é interrompida.

### US3 — Fim da barricada sem crédito parcial (P1)

Barricada destruída (`pv <= 0`) ou expirada (`ticks_restantes` a zero) antes dos 300 ticks **não credita**; o contador é descartado junto com a barricada, mas os pontos já creditados em barricadas anteriores são permanentes.

**Why this priority**: a regra canônica exige barricada **intacta**; aceitar progresso de uma barreira morta creditaria defesa que não aconteceu.

**Independent Test**: acumular 299 ticks e remover a barricada por PV zero ( ou por expiração) antes do tick 300; conferir nenhum crédito e nenhum contador remanescente.

**Acceptance Scenarios**:

1. **Given** `defesa_ticks == 299`, **When** a barricada é destruída ou expira antes do tick 300, **Then** nenhum ponto é creditado e a `Barreira` sai de `Mundo::barreiras`.
2. **Given** uma barricada que expira exatamente no tick corrente, **When** a fase de defesa é processada, **Then** esse tick não acumula nem credita (a barreira não está mais intacta).
3. **Given** o jogador já recebeu +1 em uma barricada anterior removida, **When** a remoção ocorre, **Then** `pontos_autogestao` permanece com o valor creditado.

### US4 — Determinismo e ordem estável (P1)

O acúmulo é um passo determinístico do núcleo: no máximo **um incremento por barricada por tick**, com iteração em ordem de `id` (`BTreeMap`), sem relógio, aleatoriedade ou ordem de iteração instável; duas execuções com o mesmo input produzem exatamente o mesmo resultado.

**Why this priority**: o servidor headless do ciclo 042 emite hashes por tick; qualquer não-determinismo aqui invalida a comparação entre cliente e servidor (Princípios III/VI).

**Independent Test**: executar duas vezes a mesma sequência de comandos (duas barricadas do mesmo dono no mesmo ponto) e comparar `pontos_autogestao`, `defesa_ticks` e `hash_estado` tick a tick.

**Acceptance Scenarios**:

1. **Given** duas barricadas do mesmo dono no mesmo ponto, **When** um tick é processado, **Then** cada uma avança no máximo 1 tick, na ordem de `id` ascendente de `BTreeMap`, sem crédito duplicado em uma mesma barricada.
2. **Given** a mesma sequência de comandos em duas instâncias, **When** os hashes são comparados em cada tick, **Then** são idênticos.

### US5 — Estado replicável no hash (P2)

`Mundo::hash_estado()` passa a refletir `pontos_autogestao` de cada jogador e `defesa_ticks` de cada barricada, em ordem estável, tornando visível tanto o crédito quanto o progresso parcial.

**Why this priority**: é o mecanismo de verificação honesta do núcleo; um crédito que muda o estado mas não o hash escaparia da detecção de divergência.

**Independent Test**: comparar o hash antes e depois de um tick que incrementa `defesa_ticks`, e antes/depois do tick que credita +1.

**Acceptance Scenarios**:

1. **Given** um tick que incrementa `defesa_ticks` de 10 para 11, **When** o hash é calculado, **Then** ele difere do hash anterior.
2. **Given** duas execuções deterministicamente idênticas, **When** os hashes são calculados no mesmo tick, **Then** são iguais, com iteração de jogadores e barreiras na ordem de `BTreeMap`.

### US6 — Feedback no HUD e limites de escopo (P2)

O cliente exibe **Autogestão** com os pontos acumulados, sem exigir nova tecla nem novo campo de protocolo; a implementação respeita o estilo Rust obrigatório e não altera stack, dependências ou contrato.

**Why this priority**: sem o valor visível, o jogador não percebe a recompensa; sem os limites de escopo, o ciclo correria o risco de reabrir rede ou promover hipóteses.

**Independent Test**: inspecionar o HUD do cliente e os manifestos/contratos após a implementação, conferindo a presença de `Autogestão` e a ausência de mudança em `crates/contratos`, `PROTOCOL_VERSION` e `Cargo.toml`.

**Acceptance Scenarios**:

1. **Given** um jogador com `pontos_autogestao > 0`, **When** o HUD é renderizado, **Then** exibe `Autogestão` com o valor corrente, sem nova ação de input.
2. **Given** o estado final do ciclo, **When** os arquivos de contrato/manifesto são conferidos, **Then** `crates/contratos`, `PROTOCOL_VERSION` e `Cargo.toml` permanecem intactos e não há Lightyear, async ou sockets.

### Edge Cases
- 299 ticks: nenhum crédito; 300 ticks exatos: +1 e contador zerado; 600 ticks em uma mesma barricada não ocorrem (duração de 360 ticks).
- `mover` sem alterar posição (`ponto_igual`): não interrompe a defesa.
- Dono ausente de `Mundo::jogadores`: contador zera, sem crédito (tratado sem `unwrap`/`expect`).
- Barricada com `pv <= 0` ou que expire no tick corrente: fora da condição «intacta», não acumula.
- Duas barricadas do mesmo dono no mesmo ponto: contadores independentes por `Barreira`, avanço em ordem de `id`; cada uma credita no máximo 1 ponto.
- Barricadas de donos diferentes no mesmo ponto: cada dono só defende as suas (comparação por `dono`).
- Pontos já creditados não são perdidos quando a barricada é removida; somente o progresso parcial é descartado.
- zero barricadas: `pontos_autogestao` permanece inalterado; nenhuma fase inventa crédito.

## Requirements

### Functional Requirements

- **FR-001 (regra canônica)**: creditar **+1 ponto de autogestão a cada 10 s de barricada intacta** com o dono no mesmo ponto, conforme GDD §7.1:306; 10 s = **300 ticks** a 1/30 s; o contador é **por barricada** e zera no crédito; progresso parcial (299 ticks) não credita; com 200 PV/360 ticks, uma barricada credita no máximo 1 ponto.
- **FR-002 (contrato/estado)**: acrescentar `pontos_autogestao: i64` a `Jogador` e `defesa_ticks: i64` a `Barreira`, com constantes públicas e documentadas `TICKS_AUTOGESTAO_POR_PONTO = 300` e `PONTOS_AUTOGESTAO_POR_DEFESA = 1`; **nenhuma mudança** em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto ou CI; sem Lightyear, async, sockets, janela ou relógio externo.
- **FR-003 (comportamento/fase)**: passo determinístico em `fase_defesa` (entre `fase_skills` e `fase_finish`), executado **antes** do decremento de `ticks_restantes` e do `retain` em `fase_finish` que expira barricadas; iterar `barreiras` em ordem de `BTreeMap` por `id`; para cada barreira intacta (`pv > 0`) cujo `dono` esteja presente e em `posicao` igual via `ponto_igual`, incrementar `defesa_ticks` em 1 por tick (no máximo 1 por barricada por tick) e, ao atingir 300, somar 1 a `pontos_autogestao` e zerar o contador; zerar `defesa_ticks` **sem crédito** quando o dono sai do ponto, está ausente, ou a barricada é destruída/expirada.
- **FR-004 (hash/determinismo)**: `hash_estado()` inclui `pontos_autogestao` de cada jogador e `defesa_ticks` de cada barreira, na ordem estável de `BTreeMap`, além dos campos já presentes; estados idênticos geram hash idêntico e qualquer acúmulo parcial ou crédito altera o hash; duas execuções com o mesmo input produzem o mesmo estado, sem relógio, aleatoriedade ou ordem não determinística.
- **FR-005 (cliente/HUD)**: o HUD exibe **Autogestão** com `pontos_autogestao` do jogador local; nenhuma tecla nova é adicionada e nenhum campo novo entra no protocolo; a exibição é somente leitura do estado já replicado.
- **FR-006 (governança/testes)**: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, `///` em itens públicos; testes determinísticos cobrindo crédito em 300, não-crédito em 299, interrupção, expiração/destruição, ordem e hash; R1 `go`, R6 **N/A** por v1.2.2, R4 antes de R7 `1.0.0-local`; commits em PT-BR com staging explícito; gates, build, testes, formatadores, commit/push/PR **[NV]** para este agente.

### Key Entities

- **Jogador**: estado do defensor; ganha `pontos_autogestao: i64` (acumulado entre barricadas, persistente) e mantém `posicao`, `materiais` e os campos de canalização/coleta existentes.
- **Barreira**: parede do ciclo 040 (`id`, `dono`, `posicao`, `pv = 200`, `ticks_restantes = 360`); ganha `defesa_ticks: i64` (progresso de defesa **por barricada**, descartado na remoção).
- **Passo de defesa**: parte determinística de `fase_defesa` (entre `fase_skills` e `fase_finish`), antes do decremento/expiração de duração em `fase_finish`; compara `dono` e posição com `ponto_igual`, avança 1/tick e credita ao completar 300 ticks.
- **Constante canônica**: 10 s = 300 ticks (30 Hz); 1 ponto por intervalo completo; 360 ticks de duração da barricada limitam o ciclo a 1 crédito por barricada.
- **Hash do estado**: assinatura SHA-256 de `Mundo`; passa a incluir pontos de autogestão e defesa parcial em ordem de `BTreeMap`.
- **HUD `Autogestão`**: único acréscimo visível ao jogador; sem input ou protocolo novo.

## Clarificações
- A regra é **GDD §7.1:306** (tabela do loop de 30 s): «+1 ponto de autogestão a cada 10 s de barricada intacta»; o «tempo alvo 8 s» da mesma linha é a meta da fase, **não** o intervalo de crédito — o intervalo canônico de recompensa é 10 s = 300 ticks.
- «Intacta» significa `pv > 0` **e** barricada ainda presente após o decremento/expiração de duração no tick corrente; o tick da criação consome a unidade extra de `ticks_restantes` conforme o ciclo 040.
- Protótipo usa o **dono no mesmo ponto** como defensor («Guerreiro na barricada, Artesão reforça» é registrado como **simplificação de escopo**): não há sistema de papéis, reforço ou terceiros defendendo a barreira.
- O pulso de autogestão de 30 s (GDD §6.2/:299, `[HIPÓTESE]`), a conversão de Nós/ilhas e as condições de vitória permanecem **fora de escopo**; este ciclo implementa somente a recompensa local da tarefa «Defender Barricada».
- A condição de GDD :396 («Sabotar Maquinário só avança se o Guerreiro mantiver a barricada por 10 s») não é habilitada: sabotagem/Desafio da Greve ficam fora de escopo.
- «Zera e não credita» vale para saída do dono, dono ausente, destruição e expiração; somente o contador da barricada é perdido — os pontos já creditados permanecem.
- Sem mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, manifesto ou CI; sem Lightyear, async, sockets, rede TCP ou janela (ADR-004 mantém a rede bloqueada até decisão da SE).
- **R6 é N/A** por constituição v1.2.2: não é dívida executada, nem regra ativa, nem simulação de issue; se invocada, PARE e escale ao SWE; a dívida de automação (`resolve-tasks.sh`/adaptador) fica registrada sem `gh` improvisado.
- Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0); rastreabilidade FR-001..FR-006 acordada com os artefatos de governança.

## Success Criteria

- **SC-001 (regra canônica)**: com barricada intacta e dono no mesmo ponto, 299 ticks não creditam e 300 ticks creditam exatamente +1 com `defesa_ticks` zerado; uma barricada nunca credita mais de 1 ponto.
- **SC-002 (contrato/estado)**: `Jogador.pontos_autogestao` e `Barreira.defesa_ticks` existem com as constantes documentadas; `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto e CI permanecem intactos; nenhuma dependência, Lightyear, async, socket ou janela é introduzida.
- **SC-003 (comportamento/fase)**: saída do dono (inclusive `mover` com posição alterada), dono ausente, destruição e expiração zeram `defesa_ticks` sem crédito; `mover` sem mudança de posição não interrompe; no máximo 1 incremento por barricada por tick, em ordem de `BTreeMap`.
- **SC-004 (hash/determinismo)**: `hash_estado` muda com progresso parcial e com crédito e é idêntico entre duas execuções com o mesmo input, incluindo os novos campos em ordem de `BTreeMap`.
- **SC-005 (cliente/HUD)**: o HUD exibe `Autogestão` com o valor corrente, sem nova tecla e sem campo novo de protocolo.
- **SC-006 (governança/testes)**: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, testes determinísticos cobrindo os casos limítrofes; R1 `go`, R6 N/A, R4 antes de R7 `1.0.0-local`; gates, build, testes, formatadores e publicação ficam **[NV]** até execução pelo orquestrador.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam o código Rust na etapa 12; o orquestrador executa gates, build e verificações finais.
- Ambiente headless: nenhuma janela, rede, credencial ou comando `gh` é usado na redação destes artefatos.
- Os valores canônicos (10 s/300 ticks, 200 PV, 360 ticks, 30 Hz) são estáveis para o alvo; hipóteses não são promovidas a requisito.
- O core já expõe `Mundo`, `Jogador`, `Barreira`, `fase_skills`, `fase_defesa`, `fase_finish`, `hash_estado`, `ponto_igual` e `BTreeMap` de barreiras, conforme lido em `crates/sim/src/lib.rs`; nenhum novo crate é necessário.

## Out of scope

Guerreiro/Artesão como papéis implementados, reforço e defesa por terceiros, sabotagem/Desafio da Greve, pulso de 30 s, conversão de Nós/ilhas e condição de vitória, dano/combate contra barricadas, ponte provisória, rede (TCP/Lightyear/async/sockets), mudança de stack/dependências/protocolo/contrato, janelas, deploy, commits/push/PRs e alterações preexistentes fora da fronteira. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: defesa de barricada → +1 autogestão/10 s **4×4×4×4 = 256/625**, escolhida sobre ponte provisória 108/625, Súditos/Propaganda 48/625 e rede 10/625 (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, denominador 625. Nenhum manifesto é criado.
