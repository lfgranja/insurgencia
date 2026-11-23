# Feature Specification: Ponte provisória determinística

**Branch**: `045-ponte-provisoria` (base `fa33052`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T062500Z-ponte-provisoria`, `insurgencia - ponte provisoria deterministica`.

**Âncora**: decisão 80/20 do ciclo 045 registrada em `.specify/memory/STATE.md` (2026-11-23T06:25Z): ponte provisória **3×3×3×4 = 108/625**, escolhida como próximo recurso canônico implementável após a defesa de barricada (256/625) entregue no ciclo 044, contra Súditos/Propaganda **4×2×2×3 = 48/625**, Combate/Fôlego **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. Regra canônica: GDD §9.3:475,483 fixa custo **15 Materiais** e recusa sem saldo **sem consumo parcial**; GDD §8.4:407 fixa **60 PV** e **45 s de duração** (1350 ticks a 1/30 s). O atalho de 8 s entre ilhas depende de mapa/ilhas e fica **fora de escopo**, registrado como limitação. Os **45 ticks de construção** são escolha de protótipo (espelham a barricada do ciclo 040), **não** valor canônico. **R6 é N/A** por constituição v1.2.2; R4 precede R7 `1.0.0-local`.

## User Scenarios

### US1 — Custo canônico e recusa sem consumo parcial (P1)

O núcleo determinístico aceita a nova ação `construir_ponte` somente com saldo de **pelo menos 15 Materiais** (GDD §9.3:475,483); sem saldo, o comando é recusado com `CodigoRecusa::SaldoInsuficiente`, **sem debitar nada** e sem iniciar canalização. A ponte construída nasce com **60 PV** e **45 s de duração** (GDD §8.4:407).

**Why this priority**: é a regra canônica de GDD §9.3 e §8.4 e a diferença entre economia material e magia (Princípio V); consumo parcial transformaria uma recusa em perda invisível e quebraria a honestidade do estado (Princípio VI).

**Independent Test**: submeter `construir_ponte` com 14 e com 15 Materiais no mesmo ponto; conferir recusa sem débito no primeiro caso e canalização iniciada no segundo, comparando duas execuções idênticas.

**Acceptance Scenarios**:
1. **Given** um jogador com 14 Materiais na posição do destino, **When** `construir_ponte` é validado, **Then** retorna `SaldoInsuficiente`, `materiais` continua 14 e nenhuma canalização é iniciada.
2. **Given** um jogador com 15 Materiais na posição do destino, **When** `construir_ponte` é validado, **Then** a validação passa e a canalização de 45 ticks começa sem débito antecipado.
3. **Given** a ponte concluída, **When** o estado é inspecionado, **Then** `pv == 60` e `ticks_restantes` reflete 45 s a 1/30 s.

### US2 — Construção determinística e criação da Ponte (P1)

A canalização exige `destino` **igual à posição corrente** do jogador (comparação exata por `ponto_igual`) e avança no máximo **1 tick por jogador por tick**. Ao completar `TICKS_CONSTRUCAO_PONTE = 45`, o débito de 15 Materiais é **atômico** e uma `Ponte` com `id` monotônico é inserida em `Mundo::pontes` (`BTreeMap<u64, Ponte>`).

**Why this priority**: sem validação de destino e débito atômico, comandos forjados poderiam criar pontes à distância ou debitar duas vezes; o padrão já existe para barricadas e deve ser reutilizado sem divergência (Princípios III/VI).

**Independent Test**: construir com `destino` fora do ponto (recusa), com destino correto em 45 ticks (criação) e interromper a canalização no tick 20 por mudança de posição (sem criação e sem débito).

**Acceptance Scenarios**:
1. **Given** `destino` diferente da posição por `ponto_igual`, **When** o comando é processado, **Then** retorna `DestinoInvalido`, sem canalização e sem débito.
2. **Given** destino igual à posição e saldo suficiente, **When** 44 ticks são processados, **Then** `construcao_ticks == 44`, nenhuma ponte existe e `materiais` não mudou.
3. **Given** o tick 45 com saldo ainda suficiente, **When** a conclusão é processada, **Then** `materiais` cai exatamente 15 uma única vez e uma `Ponte` com `pv = 60` e `ticks_restantes = 1350` serializado ao fim do tick é criada (internamente `TICKS_DURACAO_PONTE + 1 = 1351`; a unidade extra é consumida no mesmo tick por `fase_finish`).
4. **Given** a canalização em curso, **When** o jogador muda de posição ou o saldo fica abaixo de 15, **Then** a canalização zera sem criar ponte e sem debitar.

### US3 — Expiração determinística sem efeitos residuais (P1)

A ponte expira de forma determinística ao fim de **1350 ticks (45 s)**, com decremento e remoção em `fase_finish`, sem relógio externo, aleatoriedade ou ordem de iteração instável. A remoção não credita nem debita recursos e não afeta barricadas ou jogadores.

**Why this priority**: a duração de 45 s é canônica (GDD §8.4:407); expiração não determinística invalidaria a comparação de hashes entre cliente e servidor headless do ciclo 042.

**Independent Test**: criar uma ponte, avançar 1349 ticks (presente) e mais 1 tick (removida), conferindo o mesmo resultado em duas execuções idênticas.

**Acceptance Scenarios**:
1. **Given** uma ponte recém-criada, **When** 1349 ticks são processados, **Then** ela permanece em `Mundo::pontes` com `ticks_restantes` decrescente.
2. **Given** `ticks_restantes == 1`, **When** o tick final é processado em `fase_finish`, **Then** a ponte é removida e nenhum outro estado muda.
3. **Given** duas pontes criadas em ticks diferentes, **When** expiram, **Then** cada uma é removida no seu próprio prazo, em ordem estável de `id`.

### US4 — Determinismo, ordem estável e hash (P1)

O estado da ponte é refletido em `Mundo::hash_estado()` na ordem de `BTreeMap` por `id`; duas execuções com a mesma sequência de comandos produzem exatamente o mesmo hash a cada tick, sem `HashMap`, relógio, aleatoriedade ou iteração instável.

**Why this priority**: o servidor headless emite hashes por tick; qualquer campo novo fora do hash escaparia da detecção de divergência (Princípio VI).

**Independent Test**: executar duas vezes a mesma sequência (duas pontes do mesmo dono em pontos diferentes) e comparar `Mundo::pontes` e `hash_estado` tick a tick.

**Acceptance Scenarios**:
1. **Given** duas execuções com o mesmo input, **When** os hashes são comparados em cada tick, **Then** são idênticos.
2. **Given** a criação de uma ponte, **When** o hash é recalculado, **Then** ele difere do hash anterior.
3. **Given** uma ponte com `pv` alterado ou removida por expiração, **When** o hash é recalculado, **Then** ele difere do hash do tick anterior.

### US5 — HUD: contagem de Pontes e tecla `P` (P2)

O cliente passa a enviar `construir_ponte` com a tecla **`P`** (`KeyCode::KeyP` em `just_pressed`) usando a posição atual como destino, e exibe **Pontes** com a contagem de pontes ativas do jogador local, ao lado de **Barreiras**. Nenhum campo novo entra no protocolo e nenhuma tecla existente é removida.

**Why this priority**: sem a tecla a ação não é acessível e sem a contagem o jogador não percebe o recurso; a leitura é feita diretamente do estado já replicado, sem nova superfície de contrato.

**Independent Test**: apertar `P` no cliente e conferir o comando gerado e o incremento de `Pontes` no HUD após 45 ticks; verificar que `crates/contratos` e `PROTOCOL_VERSION` permanecem intactos.

**Acceptance Scenarios**:
1. **Given** o jogador na posição desejada, **When** `P` é pressionado, **Then** um comando `construir_ponte` com `destino` igual à posição é enfileirado.
2. **Given** uma ponte ativa do jogador local, **When** o HUD é renderizado, **Then** exibe `Pontes: 1` sem campo de protocolo novo.
3. **Given** o estado final do ciclo, **When** os arquivos de contrato são conferidos, **Then** `crates/contratos`, `PROTOCOL_VERSION` e `Cargo.toml` permanecem intactos.

### US6 — Fronteira de stack e governança (P2)

A implementação permanece no core determinístico e no cliente já existentes, sem Lightyear, async, sockets, rede TCP, janela, nova dependência ou alteração de contrato versionado; R6 é **N/A** por constituição v1.2.2 e R4 precede R7 `1.0.0-local`.

**Why this priority**: preserva os Princípios II/III/IV e impede que um recurso local reabra a rede bloqueada por ADR-004 ou promova hipóteses a requisito.

**Independent Test**: inspecionar os arquivos alterados e os artefatos SDD, conferindo ausência de mudança em contrato/protocolo/Cargo e presença explícita de R6 N/A e da ordem R4 → R7.

**Acceptance Scenarios**:
1. **Given** o estado final do ciclo, **When** a fronteira é revisada, **Then** somente os paths de código atribuídos na etapa 12 e os 4 artefatos SDD foram tocados.
2. **Given** R6, **When** o ciclo é revisado, **Then** consta como N/A (não executada, não simulada), com PARE se invocada e nenhuma issue criada.

### Edge Cases
- 14 Materiais: recusa com `SaldoInsuficiente` e nenhum débito; 15 Materiais: canalização sem débito antecipado; débito ocorre uma única vez no tick 45.
- `destino` ausente ou diferente da posição por `ponto_igual`: `DestinoInvalido`, sem canalização.
- Canalização interrompida por mudança de posição ou perda de saldo: zera sem criar ponte e sem consumir materiais.
- Tick 44 sem ponte; tick 45 cria com `pv = 60` e `ticks_restantes = 1350` serializado (criada internamente com 1351; a unidade extra é consumida no mesmo tick por `fase_finish`); tick 1349 ainda presente; tick 1350 remove.
- `mover` enviado exatamente no tick de conclusão: o auto-avanço conclui e cria a ponte no ponto original antes do processamento do comando (ordem já documentada para a barricada no ciclo 040); o deslocamento é aplicado em seguida, sem cancelamento retroativo — limitação registrada em Clarificações.
- Duas pontes do mesmo dono: `id` monotônicos distintos, hash em ordem de `BTreeMap`, sem limite arbitrário introduzido neste ciclo.
- Ponte com `pv <= 0` (dano fora de escopo): o campo existe e entra no hash; nenhum sistema de ataque é introduzido para reduzi-lo.
- Nenhuma ponte: `hash_estado` permanece determinístico e o HUD exibe `Pontes: 0`.
- Comando `construir_ponte` com `seq`/`request_id` em replay: recusado antes da validação de destino e saldo.

## Requirements

### Functional Requirements

- **FR-001 (regra canônica)**: a ação `construir_ponte` custa exatamente **15 Materiais** (GDD §9.3:475,483); sem saldo, a ação é **recusada sem consumo parcial** (`CodigoRecusa::SaldoInsuficiente`, nenhum débito e nenhuma canalização iniciada); a ponte nasce com **60 PV** e **45 s de duração = 1350 ticks** a 1/30 s (GDD §8.4:407). O atalho de 8 s entre ilhas depende de mapa/ilhas e fica **fora de escopo**, registrado como limitação; os **45 ticks de construção** são escolha de protótipo, não valor canônico.
- **FR-002 (contrato/estado)**: acrescentar `"construir_ponte"` a `ACOES_PERMITIDAS` **na sim**; struct pública `Ponte { id: u64, dono: u64, posicao: (f64, f64), pv: i64, ticks_restantes: i64 }` em `BTreeMap<u64, Ponte>` no campo `Mundo::pontes`, com `proxima_ponte_id: u64`; constantes públicas e documentadas `CUSTO_PONTE_MATERIAIS = 15`, `PV_PONTE = 60`, `TICKS_DURACAO_PONTE = 1350`, `TICKS_CONSTRUCAO_PONTE = 45`. **Nenhuma mudança** em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto ou CI; reutilizar `CodigoRecusa::DestinoInvalido`/`SaldoInsuficiente` sem novo código de protocolo; sem Lightyear, async, sockets, janela ou relógio externo.
- **FR-003 (comportamento/fase)**: validar `destino == posicao` via `ponto_igual` e saldo ≥ 15 antes de iniciar a canalização; avançar no máximo **1 tick por jogador por tick**, zerando a canalização quando o ponto muda ou o saldo cai abaixo de 15; ao completar 45 ticks, fazer **débito atômico** (reverificar saldo, subtrair 15 uma única vez) e criar a `Ponte` somente se o débito ocorreu, com `id` monotônico, `pv = PV_PONTE` e `ticks_restantes` internamente `TICKS_DURACAO_PONTE + 1` (1351), cujo valor serializado ao fim do tick de criação é **1350** porque `fase_finish` consome a unidade extra no mesmo tick; expirar por decremento e `retain` em `fase_finish`, em ordem de `BTreeMap` por `id`, sem relógio, aleatoriedade ou efeito colateral em barricadas/jogadores.
- **FR-004 (hash/determinismo)**: `hash_estado()` inclui cada `Ponte` (`id`, `dono`, `posicao`, `pv`, `ticks_restantes`) em ordem estável de `BTreeMap`, além dos campos já presentes; estados idênticos geram hash idêntico; criação, variação de `pv` e expiração alteram o hash; duas execuções com o mesmo input produzem o mesmo estado e hash, sem `HashMap`, relógio, aleatoriedade ou ordem instável.
- **FR-005 (cliente/HUD)**: a tecla **`P`** (`KeyCode::KeyP`, `just_pressed`) envia `construir_ponte` com `destino` na posição atual; o HUD exibe **Pontes** com a contagem de pontes ativas do jogador local (filtrando `Mundo::pontes` por `dono`) ao lado de **Barreiras**, sem novo campo de protocolo e sem remover teclas existentes; a exibição é somente leitura do estado já replicado.
- **FR-006 (governança/testes)**: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`, `///` em itens públicos; `#[expect(clippy::needless_pass_by_value, reason = "...")]` é atributo de lint permitido, não chamada `.expect()` (vide Clarificações); testes determinísticos cobrindo 15 Materiais, recusa sem consumo, destino inválido, 45 ticks, débito atômico, 60 PV/1350 ticks, expiração, ordem, hash, tecla `P` e HUD; R1 `go`, R6 **N/A** por v1.2.2, R4 antes de R7 `1.0.0-local`; commits em PT-BR com staging explícito; gates, build, testes, formatadores, commit/push/PR **[NV]** para este agente.

### Key Entities

- **Ponte**: struct pública em `crates/sim` com `id`, `dono`, `posicao`, `pv = 60` e `ticks_restantes = 1350`; identificador monotônico e iteração ordenada por `id`.
- **Mundo**: ganha `pontes: BTreeMap<u64, Ponte>` e `proxima_ponte_id: u64`, mantendo `barreiras`, `jogadores`, comandos e tick/epoch intactos.
- **Jogador**: mantém `materiais`, `posicao` e a canalização existente (`construcao_ticks`/`construcao_ponto`), agora também usada por `construir_ponte` sem débito antecipado.
- **Ação `construir_ponte`**: nova entrada em `ACOES_PERMITIDAS` da sim; recusa com `DestinoInvalido` ou `SaldoInsuficiente`, sem mudança em `crates/contratos` ou `PROTOCOL_VERSION`.
- **Constantes canônicas**: 15 Materiais, 60 PV, 1350 ticks (45 s a 30 Hz) e 45 ticks de construção (protótipo, não canônico).
- **Hash do estado**: assinatura SHA-256 de `Mundo`; passa a incluir as pontes em ordem de `BTreeMap`.
- **HUD `Pontes` / tecla `P`**: único acréscimo visível e de input; nenhum campo novo de protocolo.

## Clarificações

- GDD §9.3:475,483 e §8.4:407 são canônicas: 15 Materiais, recusa sem saldo sem consumo parcial, 60 PV e 45 s de duração. Nada disso é `[HIPÓTESE]`.
- 45 s = 1350 ticks a 1/30 s (`PASSO_SEGUNDOS`); os **45 ticks de construção** são escolha de protótipo que espelha a barricada do ciclo 040 — não são o valor 45 s canônico e não devem ser confundidos com ele.
- A ponte é serializada com `ticks_restantes = 1350` ao fim do tick 45: a criação usa internamente `TICKS_DURACAO_PONTE + 1` (1351) e `fase_finish` consome essa unidade extra no mesmo tick; nenhum valor 1351 persiste entre ticks.
- A vedação a `unwrap`/`expect`/`#[allow]` não atinge o atributo de lint `#[expect(clippy::needless_pass_by_value, reason = "...")]`: é atributo permitido, distinto da chamada de método `.expect()`; a vedação a `expect` refere-se a `.expect()`.
- Limitação de ordem no tick de conclusão: o auto-avanço da ponte roda no início de `fase_skills`, antes do loop de comandos (ordem já documentada para a barricada no ciclo 040). Um `mover` enviado exatamente no tick em que o 45.º tick de construção conclui pode ser processado **depois** do auto-avanço: a ponte é criada no ponto original e o deslocamento é aplicado em seguida, sem cancelamento retroativo.
- O atalho de 8 s entre ilhas (GDD §8.4:407) depende de mapa/ilhas e fica **fora de escopo**, registrado como limitação explícita; este ciclo entrega o recurso e o estado, não a topologia.
- `pv = 60` é atributo canônico refletido no estado e no hash; nenhum sistema de dano/ataque a pontes é introduzido — combate é candidato separado (48/625) e permanece fora de escopo.
- A ponte não credita pontos de autogestão, não modifica barricadas e não altera condições de vitória; o ciclo 044 permanece como está.
- Sem mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto ou CI; ADR-004 mantém a rede bloqueada até decisão da SE.
- **R6 é N/A** por constituição v1.2.2: não é dívida executada, nem regra ativa, nem simulação de issue; se invocada, PARE e escale ao SWE; a dívida de automação (`resolve-tasks.sh`/adaptador) fica registrada sem `gh` improvisado.
- Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0); rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança.

## Success Criteria

- **SC-001 (regra canônica)**: 14 Materiais recusam sem débito e 15 Materiais iniciam a canalização; a ponte concluída nasce com 60 PV e 1350 ticks; o atalho de 8 s e os 45 ticks de construção como protótipo estão registrados com suas fronteiras.
- **SC-002 (contrato/estado)**: `ACOES_PERMITIDAS` contém `construir_ponte`, `Ponte` e `Mundo::pontes` existem com `BTreeMap<u64, Ponte>` e as quatro constantes documentadas; `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto e CI permanecem intactos; nenhuma dependência, Lightyear, async, socket ou janela é introduzida.
- **SC-003 (comportamento/fase)**: `destino` fora do ponto e saldo abaixo de 15 recusam sem consumo; a canalização avança no máximo 1 tick por jogador por tick e zera sem criar quando interrompida; o débito é atômico e único no tick 45; a ponte expira em 1350 ticks sem afetar outros estados.
- **SC-004 (hash/determinismo)**: `hash_estado` muda com criação, `pv` e expiração, e é idêntico entre duas execuções com o mesmo input, incluindo as pontes em ordem de `BTreeMap`.
- **SC-005 (cliente/HUD)**: a tecla `P` gera `construir_ponte` na posição atual e o HUD exibe `Pontes` com a contagem do jogador local, sem novo campo de protocolo e sem remover teclas existentes.
- **SC-006 (governança/testes)**: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]` (atributo de lint `#[expect(...)]` permitido), `max_width = 100`, testes determinísticos cobrindo os casos limítrofes; R1 `go`, R6 N/A, R4 antes de R7 `1.0.0-local`; gates, build, testes, formatadores e publicação ficam **[NV]** até execução pelo orquestrador.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam o código Rust na etapa 12; o orquestrador executa gates, build e verificações finais.
- Ambiente headless: nenhuma janela, rede, credencial ou comando `gh` é usado na redação destes artefatos.
- Os valores canônicos (15 Materiais, 60 PV, 45 s = 1350 ticks, 30 Hz) são estáveis para o alvo; hipóteses não são promovidas a requisito.
- O core já expõe `Mundo`, `Jogador`, `Barreira`, `ACOES_PERMITIDAS`, `ValidacaoComandos`, `fase_finish`, `hash_estado` e `ponto_igual`, e o cliente já lê o estado e trata teclas em `crates/cliente/src/main.rs`; nenhum novo crate é necessário.
- O scorecard 108/625 é a escolha do ciclo 045 porque a opção 256/625 (defesa de barricada) já foi entregue no ciclo 044; não há reabertura da decisão anterior.

## Out of scope

Atalho de 8 s entre ilhas e qualquer mudança de mapa/topologia, dano/ataque a pontes e combate/Fôlego, Súditos/Propaganda, papéis Artesão/Guerreiro implementados, barricada e autogestão (ciclos 040/044), rede (TCP/Lightyear/async/sockets), mudança de stack/dependências/protocolo/contrato (`crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`), janelas, deploy, commits/push/PRs e alterações preexistentes fora da fronteira. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: ponte provisória **3×3×3×4 = 108/625**, escolhida sobre Súditos/Propaganda 48/625, Combate/Fôlego 48/625 e rede 10/625 (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, denominador 625. A opção 256/625 do ciclo 044 já foi entregue. Nenhum manifesto é criado.
