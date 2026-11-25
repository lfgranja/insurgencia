# Feature Specification: Espelho local de vitória — BF-003a (ciclo 058)

**Branch**: `feature/058-mirror-vitoria-local` (base `origin/dev = 1855e5c`) · **Created**: 2026-11-24 · **Status**: Draft · **R1**: `go` · **Sessão**: `insurgencia - mirror vitoria local`.

**Âncora**: GDD:275 — "A Ilha Central possui 3 Nós de Controle — produção, comunicação e segurança — e 100 pontos de Resistência Ideológica. Cada Nó convertido reduz 30 pontos; o pulso de autogestão seguinte reduz os 10 restantes". O parágrafo é precedido de `[HIPÓTESE]`: **3 nós, 100, −30, −10 e `TICKS_PROPAGANDA_NO = 600` (20 s × 30 Hz) são protótipo, nunca cânone**. Nenhum número é promovido a regra canônica e o GDD não é reescrito. **BF-003a** (vitória local observável no espelho do cliente) fecha neste ciclo; **BF-003b** (mapa completo de 12 ilhas) permanece **aberto** — topologia, spawning e rotas não são decididas aqui.

**Rastreabilidade canônica**: FR-001 helper público `semear_espelho_local` e constante `POSICOES_NOS_LOCAIS` na sim; FR-002 chamada do helper em `configurar` no cliente; FR-003 teste de regressão determinístico na sim; FR-004 artefatos de evidência, validação e release. Cada FR tem um SC correspondente (SC-001..SC-004, 1:1) e uma etapa explícita no `plan.md`. DoD-1..DoD-4: (1) espelho com **4 ilhas (ids 1..=4)**, **1 súdito (id 1 em `(0.0, 236.0)`)** e **3 nós**; (2) vitória local por conversão dos **3 nós via `propaganda_no`**, mesma rota de `tres_nos_geram_vitoria`; (3) `hash_estado` **muda** ao semear; (4) BF-003a fechado e BF-003b aberto. Consumidor: **jogador do cliente Bevy + testes de integração da sim**. Score **270/3125** (`5×3×3×2×3`). R6/**taskstoissues é N/A** sob a constituição v1.2.2 do alvo: nunca simular issue, nunca `gh` improvisado.

## User Scenarios

### US1 — Helper de semente e constantes na sim (P1)

O jogador do cliente standalone passa a ter no espelho `Mundo` o estado mínimo que a sim já sabe converter: `insurgencia_sim::semear_espelho_local(&mut Mundo)` insere **4 ilhas** com ids `1..=4`, **1 súdito** com id `1` na posição derivada `(0.0, 236.0)` e **3 nós** nas posições de `POSICOES_NOS_LOCAIS` — `(1, (0.5, 0.0))`, `(2, (1.5, 0.0))`, `(3, (2.5, 0.0))`. O helper é público, puro em relação a relógio, RNG e rede, reaproveita `Mundo::adicionar_ilha`, `adicionar_sudito` e `adicionar_no` (APIs já existentes) e usa apenas `BTreeMap` determinístico. Nenhuma estrutura de estado, regra de vitória, contrato ou protocolo novo é criado.

**Why this priority**: é o elo ausente identificado em `research.md` — a vitória já está implementada e testada na sim, mas o espelho do cliente está vazio de ilhas/súditos/nós e a HUD reporta `Nós: 0/3`. Sem a semente, nenhuma outra história é observável.

**Independent Test**: criar `Mundo::new()`, confirmar `ilhas`, `suditos` e `nos` vazios e guardar `hash_estado()`; chamar `semear_espelho_local`; confirmar `ilhas.len() == 4` com ids `1..=4`, `suditos.len() == 1` com `id == 1` e `posicao == (0.0, 236.0)`, `nos.len() == 3` com os ids/posições de `POSICOES_NOS_LOCAIS` e `hash_estado()` diferente do inicial.

**Acceptance Scenarios**:
1. **Given** um `Mundo::new()` sem semente, **When** `semear_espelho_local` é chamado, **Then** o mundo contém exatamente 4 ilhas (`1..=4`), 1 súdito (`id 1`, `(0.0, 236.0)`) e 3 nós (`POSICOES_NOS_LOCAIS`), todos com estado inicial determinístico (`controlada: false`/`controle_ticks: 0`, `convertido: false`/`propaganda_ticks: 0`/`inativo_ticks: 0`).
2. **Given** a semente aplicada, **When** `hash_estado()` é recalculado, **Then** ele difere do hash do mundo sem semente, permanecendo idêntico entre duas execuções da mesma sequência (sem `HashMap`, RNG, `Instant` ou ordem de inserção).
3. **Given** os números da semente e de `POSICOES_NOS_LOCAIS`, **When** o código é revisado, **Then** cada um aparece como `[HIPÓTESE]`/derivação de protótipo de GDD:275 e `PASSO_SEGUNDOS = 1/30`; nenhum é apresentado como cânone e nenhuma regra canônica é alterada.

### US2 — Chamada do helper em `configurar` no cliente (P1)

Em `crates/cliente/src/main.rs`, a função `configurar` — que já cria câmera, luzes, terreno, ilhas na cena Bevy, avatar e alvo de treino — passa a chamar `insurgencia_sim::semear_espelho_local(&mut sim)` logo após `Mundo::new()` e os `adicionar_jogador`/`adicionar_pnj`, antes de `commands.insert_resource(Simulacao(sim))`. O recurso `Simulacao` passa a conter a semente já no tick 0, sem nova janela, campo de protocolo, comando de teclado ou mudança na HUD.

**Why this priority**: é a ligação entre a sim e o consumidor nomeado (jogador do cliente Bevy). Sem a chamada, o helper existe mas o espelho permanece vazio e a vitória continua inalcançável no cliente.

**Independent Test**: inspecionar `configurar` e confirmar a ordem `Mundo::new()` → jogadores/PNJs → `semear_espelho_local(&mut sim)` → inserção de `Simulacao`; confirmar que nenhuma outra chamada de semeadura aparece no cliente e que a lógica de renderização (`criar_ilha`) permanece intacta.

**Acceptance Scenarios**:
1. **Given** a inicialização do cliente, **When** `configurar` termina, **Then** `Simulacao.0` contém 4 ilhas, 1 súdito e 3 nós, com `hash_estado()` já alterado em relação ao `Mundo::new()`.
2. **Given** a cena Bevy existente, **When** a semeadura é acrescentada, **Then** câmera, luzes, terreno, ilhas renderizadas, avatar, alvo de treino, HUD, `JogadorLocal`, `ComandoLocal` e `AccumuladorTick` permanecem como antes — a mudança é somente no estado da sim.
3. **Given** a fronteira do ciclo, **When** o cliente é revisado, **Then** não há jogo duplicado da semente no cliente, nenhum `HashMap`/RNG/relógio externo, nenhuma janela normal e nenhuma alteração em contratos, `PROTOCOL_VERSION`, Cargo, CI, stack ou rede.

### US3 — Teste de regressão determinístico na sim (P1)

`crates/sim/tests/simulacao.rs` ganha `espelho_local_seed_viabiliza_vitoria_e_reflete_hash`: começa com `Mundo::new()` (sem nós), guarda o hash, chama `semear_espelho_local`, verifica `nos.len() == 3`, `ilhas.len() == 4`, `suditos.len() == 1` e `hash_estado()` diferente do inicial; registra três jogadores nas posições de `POSICOES_NOS_LOCAIS`, agenda `propaganda_no` para os três nós e avança `TICKS_PROPAGANDA_NO + 1` ticks; confirma `resultado == Some(ResultadoPartida::Vitoria)` e `resistencia_ideologica == 0`. A rota é a mesma de `tres_nos_geram_vitoria`, preservada sem alteração.

**Why this priority**: é a prova automática de DoD-2 e DoD-3 e o consumidor "testes de integração da sim" nomeado no contrato; sem ela, a ligação sim↔cliente poderia divergir silenciosamente.

**Independent Test**: localizar o teste pelo nome exato e executá-lo com `--exact` pelo orquestrador; inspecionar que os três nós vêm de `POSICOES_NOS_LOCAIS`, que o agendamento usa `propaganda_no`, que o avanço é `TICKS_PROPAGANDA_NO + 1` e que `tres_nos_geram_vitoria` permanece intacto.

**Acceptance Scenarios**:
1. **Given** o teste `espelho_local_seed_viabiliza_vitoria_e_reflete_hash`, **When** executado, **Then** passa com 1 teste, confirmando 4 ilhas, 1 súdito, 3 nós, hash alterado, `resultado == Some(Vitoria)` e resistência zero.
2. **Given** a suíte existente, **When** o novo teste é acrescentado, **Then** nenhum teste anterior é modificado, nenhuma expectativa é afrouxada e a convenção Rust (`forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap()`/`.expect()`/`#[allow]`) é respeitada.
3. **Given** a vitória local no cliente, **When** ela é verificada por inspeção, **Then** a fonte autoritativa da rota é a sim e o helper — não uma reimplementação no cliente; a captura visual da HUD permanece **[NV]** até verificação humana.

### US4 — Evidência, validação e release (P2)

Os artefatos do ciclo registram a prova e a governança: `.specify/assessments/mirror-vitoria-local/{intake,research}.md`, `.specify/validations/mirror-vitoria-local/test-roadmap.md` (roteiro com 14 passos, escrito antes do PR), `converge.md` append-only, validação condicionada ao roteiro (R4) e release `1.0.0-local` (R7). BF-003a é marcado como **fechado**; BF-003b (12 ilhas) fica **aberto**. Gate de destino `./verify.sh`; CI = jobs `gates` e `rust` do GitHub Actions; CLI de PR = `gh`; `dev→main` e deploy são exclusivos da SE; PR #76 pertence a outro owner ativo e não é tocado.

**Why this priority**: sem evidência e sem a distinção BF-003a/BF-003b, o corte poderia ser confundido com o mapa completo ou com regra canônica; a governança v1.2.2 exige R4 antes de R7 e mantém R6 fora da cadeia executável.

**Independent Test**: inspecionar os artefatos e confirmar DoD-1..DoD-4 idênticos, FR-001..FR-004/SC-001..SC-004 1:1, R3 = 0 em `plan.md` (sem arquivo `clarify.md`), ordem `tasks → analyze → implement → converge → validate → release`, R6 N/A, e ausência de build/gate/teste/formatador/commit/push/PR executados por este agente de documentação.

**Acceptance Scenarios**:
1. **Given** o estado final do ciclo, **When** a evidência é revisada, **Then** os artefatos de assessment/validação/release existem, o roteiro de 14 passos cobre semente, vitória, hash, `./verify.sh`, gates `gates`+`rust`, HUD **[NV]** e rollback documentado.
2. **Given** o backlog, **When** o fechamento é registrado, **Then** BF-003a aparece fechado e BF-003b aparece aberto, sem reabertura de ciclos anteriores e sem promoção de valor prototípico a cânone.
3. **Given** a constituição v1.2.2, **When** a cadeia é conferida, **Then** `tasks` precede `analyze` (READ-ONLY), `converge` é append-only, R4 vem antes de R7 `1.0.0-local`, R6/taskstoissues é N/A (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado) e promoção/deploy permanecem SWE-exclusivos.

### Edge Cases

- `Mundo::new()` sem semente: `ilhas`/`suditos`/`nos` vazios, `hash_estado()` é a baseline de comparação; a vitória não é observável — é exatamente o problema do ciclo.
- Idempotência: `semear_espelho_local` é chamado uma vez por `configurar`; reaplicações não são parte do contrato e não devem ser assumidas como deduplicadas — o helper reaproveita `adicionar_*`, que insere/substitui por `id`.
- Súdito em `(0.0, 236.0)`: posição é escolha de protótipo do contrato, `[HIPÓTESE]`; nenhuma rota, spawn ou grid de 12 ilhas é inferido dela.
- `POSICOES_NOS_LOCAIS`: `(0.5, 0.0)`, `(1.5, 0.0)`, `(2.5, 0.0)` coincidem com `posicoes_nos()` dos testes para que a rota de `propaganda_no` seja a mesma da sim; nenhuma posição é cânone.
- `propaganda_no` exige Nó no ponto do jogador: sem os nós semeados, o comando não converte nada e a HUD mostra `Nós: 0/3`; a semente é condição necessária, não uma nova regra de validação.
- `TICKS_PROPAGANDA_NO = 600` (20 s a 30 Hz): o teste avança 601 ticks; nenhum `Instant`/relógio de parede, RNG ou arredondamento é introduzido.
- `RESISTENCIA_INICIAL = 100`, `CONTRIBUICAO_NO = 30`, `PISO_DANO_COM_NOS = 10`, `TOTAL_NOS = 3`: já existentes e rotulados `[HIPÓTESE]`; nenhum é reaberto ou reescrito.
- Hash: semear altera a entrada SHA-256 (nós, resistência e resultado); dois replays idênticos devem produzir o mesmo hash — a mudança exigida é em relação ao mundo sem semente, não aleatoriedade entre execuções.
- HUD: `Nós: 3/3` e "Autogestão estabelecida" são observáveis na execução real **[NV]**; a verificação automatizável é inspeção do estado da sim e do teste determinístico, não captura de janela.
- Fronteira: nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, CI, GDD, manifesto, `AGENTS.md`, `concurrency.jsonl`, `STATE.md`, PR #76 ou código de outro ciclo nesta fronteira documental.
- R6 invocada por engano: **PARE** e escale ao SWE; nenhuma issue simulada, nenhum `taskstoissues`/`gh` improvisado. `dev→main` e deploy são SWE-exclusivos.

## Requirements

### Functional Requirements

- **FR-001 (helper do sim e constantes determinísticas)**: `crates/sim/src/lib.rs` expõe `pub fn semear_espelho_local(mundo: &mut Mundo)` e `pub const POSICOES_NOS_LOCAIS: [(u64, (f64, f64)); 3]`, ambos documentados com `///` e rotulados `[HIPÓTESE]`/protótipo de GDD:275. O helper semeia **4 ilhas (ids `1..=4`)** via `adicionar_ilha`, **1 súdito (id `1` em `(0.0, 236.0)`)** via `adicionar_sudito` e **3 nós** nas posições de `POSICOES_NOS_LOCAIS` via `adicionar_no`. Nenhuma estrutura de estado, regra canônica, contrato ou protocolo é acrescentado; todos os números da semente são protótipo, nunca cânone.
- **FR-002 (chamada do helper em `configurar`)**: `crates/cliente/src/main.rs` chama `insurgencia_sim::semear_espelho_local(&mut sim)` em `configurar`, após `Mundo::new()` e os `adicionar_jogador`/`adicionar_pnj` e antes de `commands.insert_resource(Simulacao(sim))`. O espelho passa a conter a semente no tick 0; a renderização Bevy, a HUD e os demais recursos permanecem intactos, sem duplicação da semeadura no cliente e sem janela/protocolo/rede novos.
- **FR-003 (teste de regressão da sim)**: `crates/sim/tests/simulacao.rs` contém `espelho_local_seed_viabiliza_vitoria_e_reflete_hash`, que verifica 4 ilhas, 1 súdito, 3 nós e `hash_estado` alterado após a semente, registra três jogadores nas posições dos nós, agenda `propaganda_no` para os três e avança `TICKS_PROPAGANDA_NO + 1` ticks até `resultado == Some(ResultadoPartida::Vitoria)` com `resistencia_ideologica == 0`. `tres_nos_geram_vitoria` e os demais testes permanecem inalterados; convenções Rust v1.2.2 respeitadas.
- **FR-004 (evidência, validação e release)**: os artefatos do ciclo registram DoD-1..DoD-4, o score 270/3125, a rastreabilidade FR-001..FR-004 ↔ SC-001..SC-004, o roteiro de verificação (14 passos), `converge.md` append-only, a validação condicionada (R4) e o release `1.0.0-local` (R7). BF-003a é fechado e BF-003b permanece aberto; gate `./verify.sh`, CI `gates`/`rust`, PR via `gh`, promoção `dev→main`/deploy SWE-exclusivos, PR #76 intocável. R6/taskstoissues é **N/A** sob v1.2.2: PARE se invocada, nenhuma issue simulada.

### Key Entities

- **`semear_espelho_local`**: helper público em `insurgencia_sim`, único ponto da semente; reaproveita `adicionar_ilha`/`adicionar_sudito`/`adicionar_no` e é chamado pelo cliente e pelo teste de regressão.
- **`POSICOES_NOS_LOCAIS`**: `[(1, (0.5, 0.0)), (2, (1.5, 0.0)), (3, (2.5, 0.0))]`, constante `[HIPÓTESE]` que coincide com `posicoes_nos()` da sim para manter a rota de vitória idêntica.
- **`Simulacao(Mundo)`**: recurso Bevy que passa a conter a semente já no tick 0; não é um espelho oculto nem uma segunda fonte de verdade.
- **`espelho_local_seed_viabiliza_vitoria_e_reflete_hash`**: teste de regressão que liga semente, `hash_estado` e vitória por `propaganda_no`.
- **`ResultadoPartida::Vitoria` / `resistencia_ideologica == 0`**: desfecho já implementado na sim, agora alcançável no cliente sem mudar a regra de GDD:275.
- **BF-003a / BF-003b**: recorte fechado (semente local) vs. pendência aberta (mapa completo de 12 ilhas).

### Rastreabilidade FR ↔ SC

| FR | SC | DoD |
|:--|:--|:--|
| FR-001 helper do sim e constantes | SC-001 espelho semeado (4 ilhas, 1 súdito, 3 nós) e hash alterado | DoD-1, DoD-3 |
| FR-002 chamada em `configurar` | SC-002 cliente semeado na inicialização | DoD-1 |
| FR-003 teste de regressão da sim | SC-003 vitória por conversão dos 3 nós via `propaganda_no` | DoD-2, DoD-3 |
| FR-004 evidência/validação/release | SC-004 BF-003a fechado, BF-003b aberto, `./verify.sh`, R6 N/A | DoD-4 |

## Success Criteria

- **SC-001 (espelho semeado e hash alterado)**: `semear_espelho_local` produz exatamente 4 ilhas (ids `1..=4`), 1 súdito (id `1` em `(0.0, 236.0)`) e 3 nós (`POSICOES_NOS_LOCAIS`), com estado inicial determinístico; `hash_estado()` difere do `Mundo` sem semente e é idêntico entre replays com a mesma sequência. Todos os números aparecem como `[HIPÓTESE]`/protótipo, nunca cânone. **[FR-001]**
- **SC-002 (cliente semeado na inicialização)**: `configurar` chama `semear_espelho_local(&mut sim)` e `Simulacao.0` contém a semente antes do primeiro tick; cena, renderização, HUD e demais recursos permanecem intactos, sem duplicação no cliente, janela, protocolo, CI, Cargo, stack ou rede novos. **[FR-002]**
- **SC-003 (vitória local alcançável)**: converter os 3 nós via `propaganda_no` leva a `resultado == Some(ResultadoPartida::Vitoria)` com `resistencia_ideologica == 0` no teste `espelho_local_seed_viabiliza_vitoria_e_reflete_hash`, pela mesma rota de `tres_nos_geram_vitoria`, sem modificar testes anteriores nem expectativas. **[FR-003]**
- **SC-004 (evidência, validação e release)**: artefatos registram DoD-1..DoD-4, score **270/3125** (`5×3×3×2×3`), roteiro de 14 passos, `./verify.sh` com jobs `gates`/`rust`, R4 antes de R7 `1.0.0-local`, `converge` append-only, R3 = 0 em `plan.md` (sem `clarify.md`), R6 **N/A** (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado), BF-003a fechado e BF-003b aberto, promoção/deploy SWE-exclusivos e PR #76 intocável. **[FR-004]**

## Clarificações

- GDD:275 é fonte de hipóteses, **não de cânone neste ciclo**: 3 nós, 100 de Resistência, −30 por nó e o pulso final de −10 estão marcados `[HIPÓTESE]` na fonte; o GDD é lido, nunca reescrito, e nenhuma regra de vitória é alterada.
- A semente — ids `1..=4`, súdito `1` em `(0.0, 236.0)` e as três posições de `POSICOES_NOS_LOCAIS` — é escolha de protótipo fixada no contrato, não conteúdo canônico; nenhuma delas é inferida como topologia de 12 ilhas.
- `R3 = 0` clarificações em aberto: as incógnitas de `intake.md` (posições, efeito visual da HUD, execução externa) são limitações de escopo registradas, não ambiguidades bloqueantes. Não há arquivo `clarify.md`; o registro canônico fica em `plan.md`, etapa 07.
- A única fonte da semente é a sim; o cliente apenas chama o helper. Semeadura duplicada no cliente foi rejeitada no scorecard (120/3125) por dividir a verdade do estado entre dois lugares.
- Captura visual da HUD e verificação remota de `gates`/`rust` são **[NV]**: nenhum build, gate, teste, formatador, commit, push ou PR é executado por este agente; a execução pertence ao orquestrador/SWE.
- **R6/taskstoissues é N/A/bloqueado** por constituição v1.2.2: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE** e escale ao SWE — nunca simular issue, nunca `gh` improvisado. `dev→main` e deploy são SWE-exclusivos; PR #76 não é tocado.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira (`spec.md`, `plan.md`, `checklists/requirements.md` e `.specify/feature.json`) no worktree `.specify/queue/worktrees/impl-058-mirror-vitoria/` (branch `feature/058-mirror-vitoria-local`, base `origin/dev = 1855e5c`). Não edita código, assessment, validação, release, `STATE.md`, `AGENTS.md` ou `concurrency.jsonl`.
- A sim já expõe `Mundo::new`, `adicionar_ilha`, `adicionar_sudito`, `adicionar_no`, `avancar_tick`, `propaganda_no`, `hash_estado`, `TICKS_PROPAGANDA_NO`, `ResultadoPartida::Vitoria` e as constantes `[HIPÓTESE]` de vitória; o cliente já tem `configurar`, `Simulacao`, criação de terreno/ilhas e HUD. Nenhum novo crate, contrato ou protocolo é necessário.
- O roteiro de verificação em `.specify/validations/mirror-vitoria-local/test-roadmap.md` tem 14 passos, escritos antes do PR; o gate de destino é `./verify.sh` e o CI é GitHub Actions com jobs `gates`/`rust`. Toda execução/captura é **[NV]** até o orquestrador.
- Score **270/3125** (`5×3×3×2×3`), escolhido sobre semeadura duplicada no cliente (**120/3125**) e HUD/`hash_estado` sem semear (**75/3125**); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵ = 3125. Consumidor: jogador do cliente Bevy + testes de integração da sim.

## Out of scope

Mudanças em GDD, qualquer regra canônica de vitória, números promovidos a cânone, mapa completo de 12 ilhas (BF-003b), topologia/spawning/rotas, novos pytest de jogo, habilidades, dano, inventário, progressão; mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, baseline e documentos de outros ciclos **na fronteira do agente de documentação** (o orquestrador atualizou `STATE.md` e o baseline de converge como subproduto de governança do ciclo, não como feature); `Lightyear`, `async`, sockets, rede, servidor remoto, segundo jogador humano; janelas normais, deploy, promoção `dev→main`, commits, push, PRs (inclusive PR #76), merge e emissão de issues. Reabertura dos ciclos 044–057. Nenhuma execução de gate/build/teste/formatador por este agente de documentação; verificações foram executadas pelo orquestrador fora desta fronteira.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. Nenhum manifesto de contexto é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade permanecem em `docs/governance/context-budget-manifest.md`.

**80/20**: `mirror-vitoria-local` (BF-003a) **5×3×3×2×3 = 270/3125**, contra semeadura duplicada no cliente **120/3125** (`5×3×2×2×2`) e HUD/`hash_estado` sem semear **75/3125** (`1×3×1×5×5`); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, escala 1–5, denominador 5⁵ = 3125. Nenhum valor do protótipo é promovido a cânone e BF-003b permanece aberto.
