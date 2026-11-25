# Feature Specification: Mover validado no cliente — BF-012 (ciclo 059)

**Branch**: `feature/059-mover-validado` (base `origin/dev = 737b876`) · **Created**: 2026-11-24 · **Status**: Draft · **R1**: `go` · **Sessão**: `insurgencia - mover validado no cliente`.

**Âncora**: o corte é ancorado no código já implementado — `crates/cliente/src/lib.rs:21` (`VELOCIDADE = 260.0` unidades de tela), `crates/cliente/src/main.rs:618-657` (`mover_jogador` escreve `JogadorLocal` em pixels), `:1117-1129` (`sincronizar_sim` injeta pixels no espelho) e `crates/sim/src/lib.rs:212,2414-2445` (`LIMITE_MOVIMENTO_M = 1.0`, `ValidacaoComandos::validar`). A escala `PIXELS_POR_METRO = 100.0` é **`[HIPÓTESE]`/protótipo da sessão**: 260 px/s ÷ 100 = 2,6 m/s ≈ **0,087 m/tick** a 30 Hz, abaixo de `LIMITE_MOVIMENTO_M = 1.0`. Nenhum número é promovido a cânone e o GDD não é reescrito. **BF-012** (WASD → `mover` validado no espelho) fecha neste ciclo; **BF-003b** (mapa de 12 ilhas) e **BF-013** (`contratos` consumido por binários) permanecem **abertos**.

**Rastreabilidade canônica**: FR-001 conversão pixels↔metros; FR-002 `mover_jogador` validado; FR-003 sincronização espelho→local e reordenação; FR-004 evidência/validação/release. Cada FR tem um SC correspondente (SC-001..SC-004, 1:1) e uma etapa explícita no `plan.md`. DoD-1..DoD-5: (1) `Comando { acao: "mover", destino: metros }` com `ValidacaoComandos::validar` **antes** de `agendar_comando`; (2) recusa preserva posição e `seq`/`request_id` — *no counter burn on rejection*; (3) `sincronizar_sim` lê `Mundo` e escreve `JogadorLocal` em pixels, sem injeção local, com `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`; (4) todos os destinos/alvos em metros via `pixels_para_metros`, escala `[HIPÓTESE]`, nenhuma regra canônica alterada; (5) consumidor = jogador do cliente Bevy + validação da sim/CLI, BF-012 fechado e BF-003b/BF-013 abertos. Score **144/3125** (`4×3×3×2×2`). R6/**taskstoissues** é **N/A** sob a constituição v1.2.2; PR #76 (`dev→main`) pertence a outro owner ativo — intocável.

## User Scenarios

### US1 — Conversão determinística pixels↔metros no cliente (P1)

O jogador do cliente standalone passa a ter uma única conversão explícita entre o estado local de tela (`JogadorLocal.posicao: (f32, f32)`, `VELOCIDADE = 260.0` px/s) e o estado autoritativo em metros (`Mundo.jogadores[PEER_ID].posicao`). `crates/cliente/src/lib.rs` expõe `pub const PIXELS_POR_METRO: f64 = 100.0` e os helpers puros documentados `pixels_para_metros`/`metros_para_pixels`. Todos os destinos e alvos das ações do cliente passam a ser construídos em metros; `aproximar_no` opera com `hypot` em metros, sem casts diretos de pixel. Nenhuma estrutura de estado, contrato, protocolo ou regra canônica nova é criada.

**Why this priority**: é a condição de possibilidade do `mover` — sem a conversão, um passo de ~8,67 unidades de tela por tick seria lido como ~8,67 m pelo validador e recusado por `MovimentoForaDoLimite`, ou o espelho seria contaminado por coordenadas de tela.

**Independent Test**: inspecionar `crates/cliente/src/lib.rs` e confirmar a constante rotulada `[HIPÓTESE]`, os helpers puros com `///`, a assinatura em metros de `aproximar_no` e os testes de `--lib` (`pixels_para_metros((260.0, 0.0)) == (2.6, 0.0)` e ida/volta dentro de `1e-6`); verificar por inspeção que nenhum destino de comando usa `f64::from(pixel)` diretamente.

**Acceptance Scenarios**:
1. **Given** a escala prototípica `PIXELS_POR_METRO = 100.0`, **When** `VELOCIDADE = 260.0` px/s é convertida a 30 Hz, **Then** o resultado é 2,6 m/s ≈ 0,087 m/tick, estritamente abaixo de `LIMITE_MOVIMENTO_M = 1.0`, sem alterar o limite ou a velocidade.
2. **Given** uma posição local em pixels, **When** `pixels_para_metros` e `metros_para_pixels` são aplicados em sequência, **Then** o valor original é recuperado dentro de `1e-6`, sem relógio, RNG, rede, `#[allow]` ou `unwrap()`.
3. **Given** os números 100,0 / 260,0 / 2,6 / 0,087, **When** o código é revisado, **Then** cada um aparece como `[HIPÓTESE]`/derivação de protótipo, nunca como cânone; o GDD é lido, nunca reescrito.

### US2 — `mover_jogador` gera comando validado (P1)

Em `crates/cliente/src/main.rs`, `mover_jogador` deixa de gravar `jogador.0 = deslocar(...)` em pixels. Com WASD/setas e `imobilizado_ticks > 0` preservando o bloqueio local, a intenção de movimento é convertida em `Comando { acao: "mover", destino: metros, seq, request_id }`; `ValidacaoComandos::validar(&simulacao.0, PEER_ID, "mover", seq + 1, request_id + 1, Some(destino_m))` é chamada **antes** de `simulacao.0.agendar_comando(...)`. Somente no ramo `Ok(_)` os contadores de `ComandoLocal` avançam; em `Err(_)` a função retorna sem mover `jogador` e sem consumir `seq`/`request_id`.

**Why this priority**: é o consumidor nomeado (jogador do cliente Bevy) e o fechamento do BF-012. Sem a validação prévia, o cliente continua burlando o limite de 1 m e a paridade com a validação da CLI (`crates/servidor/src/main.rs:146-175`).

**Independent Test**: inspecionar que a ordem no corpo da função é `validar` → `agendar_comando` → incremento, com o incremento apenas no aceite; confirmar que `MovimentoForaDoLimite`, `SeqOuRequestReplay` e imobilização não alteram posição nem monotônicos e que nenhuma nova regra de validação é introduzida na sim.

**Acceptance Scenarios**:
1. **Given** um passo de ≈ 0,087 m/tick a partir da posição do espelho, **When** o jogador pressiona WASD, **Then** `validar` aceita, `agendar_comando` registra `mover` e `seq`/`request_id` avançam exatamente uma vez.
2. **Given** um destino artificialmente além de 1 m, replay de `seq`/`request_id` ou jogador imobilizado, **When** `validar` recusa, **Then** nenhum comando é agendado, a posição local permanece a última derivada do espelho e os contadores não são consumidos.
3. **Given** a fronteira do ciclo, **When** o cliente é revisado, **Then** não há chamada direta a `deslocar` em `mover_jogador`, nem validação reimplementada no cliente, nem mudança em contratos, `PROTOCOL_VERSION`, Cargo, CI, stack ou rede.

### US3 — Sincronização autoritativa e reordenação dos sistemas (P1)

`sincronizar_sim` inverte a seta: lê `simulacao.0.jogadores[PEER_ID].posicao` em metros e grava `jogador.0.posicao = metros_para_pixels(...)`; a injeção direta `remoto.posicao = (f64::from(x), f64::from(y))` é removida, assim como o retorno antecipado por imobilização local. A cadeia `Update` passa a executar `mover_jogador` → todos os `solicitar_*` → `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar` → `atualizar_texto`, para que o avatar e a HUD desenhem o estado já aceito e avançado pelo Núcleo.

**Why this priority**: sem a inversão, o espelho continua sendo sobrescrito por pixels e o aceite do `mover` não é observável; sem a reordenação, o avatar desdesenha um tick atrasado e pode reaplicar movimento.

**Independent Test**: inspecionar `sincronizar_sim` e a `.chain()` do `Update`; confirmar que não há escrita em `remoto.posicao`, que `JogadorLocal` é o único destino da conversão e que `avancar_simulacao` precede `sincronizar_sim`, que por sua vez precede `sincronizar_avatar`.

**Acceptance Scenarios**:
1. **Given** um `mover` aceito e o tick avançado, **When** `sincronizar_sim` roda, **Then** `JogadorLocal.posicao` reflete a posição do `Mundo` convertida para pixels; avatar/HUD só avançam nesse caso.
2. **Given** um `mover` recusado, **When** a cadeia `Update` completa, **Then** a posição do espelho não mudou por aquele input e a posição local permanece idêntica — sem segunda verdade e sem dupla aplicação de movimento.
3. **Given** a cadeia anterior (`mover_jogador` → `sincronizar_avatar` → … → `sincronizar_sim` → `avancar_simulacao`), **When** a reordenação é conferida, **Then** a nova ordem está presente e nenhum plugin, janela, recurso Bevy ou sistema novo foi acrescentado.

### US4 — Evidência, validação e release (P2)

Os artefatos do ciclo registram a prova e a governança: `.specify/assessments/mover-validado/{intake,research,define,shape,decision}.md`, `.specify/validations/mover-validado/test-roadmap.md` (roteiro de 14 passos, escrito antes do PR), `converge.md` append-only, validação condicionada ao roteiro (R4) e release `1.0.0-local` (R7). BF-012 é marcado como **fechado**; BF-003b e BF-013 permanecem **abertos**. Gate de destino `./verify.sh`; CI = jobs `gates` e `rust` do GitHub Actions; CLI de PR = `gh`; `dev→main` e deploy são exclusivos da SE; PR #76 pertence a outro owner ativo e não é tocado.

**Why this priority**: sem evidência e sem a distinção de backlog, o corte poderia ser confundido com mapa completo, diagnóstico de contratos ou regra canônica; a governança v1.2.2 exige R4 antes de R7 e mantém R6 fora da cadeia executável.

**Independent Test**: inspecionar os artefatos e confirmar DoD-1..DoD-5 idênticos, FR-001..FR-004 ↔ SC-001..SC-004 1:1, R3 = 0 em `plan.md` (sem arquivo `clarify.md`), ordem `tasks → analyze → implement → converge → validate → release`, R6 N/A, e ausência de build/gate/teste/formatador/commit/push/PR executado por este agente de documentação.

**Acceptance Scenarios**:
1. **Given** o estado final do ciclo, **When** a evidência é revisada, **Then** os artefatos de assessment/validação/release existem, o roteiro de 14 passos cobre conversão, aceite, recusa, sincronização, `./verify.sh`, jobs `gates`/`rust`, captura visual **[NV]** e rollback documentado.
2. **Given** o backlog, **When** o fechamento é registrado, **Then** BF-012 aparece fechado, BF-003b e BF-013 permanecem abertos, sem promoção de valor prototípico a cânone.
3. **Given** a constituição v1.2.2, **When** a cadeia é conferida, **Then** `tasks` precede `analyze` (READ-ONLY), `converge` é append-only, R4 vem antes de R7 `1.0.0-local`, R6/taskstoissues é N/A (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado) e promoção/deploy permanecem SWE-exclusivos.

### Edge Cases

- **Recusa por limite**: um destino além de 1 m é recusado por `MovimentoForaDoLimite`; posição e monotônicos permanecem intactos; nenhum `agendar_comando`.
- **Replay**: avançar `seq`/`request_id` antes da validação faria o próprio comando ser recusado por `SeqOuRequestReplay`; o par só avança no aceite.
- **Imobilização**: `imobilizado_ticks > 0` bloqueia localmente e na sim; a recusa não consome contadores e não move o avatar.
- **Dupla aplicação**: com a posição local derivada do espelho, `mover_jogador` não pode acumular o passo sobre um estado já avançado; a ordem dos sistemas é parte do contrato, não detalhe de implementação.
- **Arredondamento na fronteira de 1 m**: o passo nominal ≈ 0,087 m é folgado; o comportamento exatamente na fronteira permanece **[NV]** até verificação em execução — nenhuma tolerância nova é inventada.
- **Escala 100**: `PIXELS_POR_METRO = 100.0` é `[HIPÓTESE]`; nenhuma regra canônica do GDD é reescrita e `LIMITE_MOVIMENTO_M` permanece `1.0`.
- **Destinos/alvos das demais ações**: propagação, ataque, escudo, coleta, armadilha e análise devem converter `jogador.0.posicao` via `pixels_para_metros`; nenhum `f64::from(pixel)` direto sobrevive como destino de comando.
- **Convenção Rust**: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap()`/`.expect()`/#[allow] improvisados; o cast `f64 → f32` em `metros_para_pixels` usa `#[expect(clippy::cast_possible_truncation, reason = "...")]` local.
- **Fronteira**: nenhuma mudança em `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, CI, GDD, manifesto, `AGENTS.md`, `concurrency.jsonl`, `STATE.md`, PR #76 ou código de outro ciclo nesta fronteira documental.
- **R6 invocada por engano**: **PARE** e escale ao SWE; nenhuma issue simulada, nenhum `taskstoissues`/`gh` improvisado. `dev→main` e deploy são SWE-exclusivos.

## Requirements

### Functional Requirements

- **FR-001 conversão determinística pixels↔metros**: `PIXELS_POR_METRO = 100.0` e helpers puros em `crates/cliente/src/lib.rs` (260 px/s → 2,6 m/s ≈ 0,087 m/tick < `LIMITE_MOVIMENTO_M = 1,0`; destinos/alvos de ações em metros)
- **FR-002 `mover_jogador`**: `Comando { acao: "mover", destino: metros }`, `ValidacaoComandos::validar` antes de `agendar_comando`, `seq`/`request_id` avançados só no aceite
- **FR-003 sincronização espelho→local**: `sincronizar_sim` lê a posição do espelho `Mundo` e grava `JogadorLocal` em pixels, sem injetar pixels no espelho; `avancar_simulacao` antes de `sincronizar_sim` e `sincronizar_avatar` depois
- **FR-004 evidência/validação/release**: BF-012 fechado, BF-003b/BF-013 abertos, `./verify.sh` (`gates`/`rust`), R4 antes de R7 `1.0.0-local`, R6 N/A

### Key Entities

- **`PIXELS_POR_METRO` / `pixels_para_metros` / `metros_para_pixels`**: API pública de `insurgencia_cliente`, único ponto de conversão; `[HIPÓTESE]`/protótipo, sem relógio, RNG ou rede.
- **`ComandoLocal { seq, request_id }`**: monotônicos do cliente; avançam somente no aceite do `mover`, preservando replay e não queimando contador na recusa.
- **`Comando { acao: "mover", destino: metros }`**: forma canônica da intenção de movimento; a decisão pertence a `ValidacaoComandos::validar` na sim.
- **`Simulacao(Mundo)`**: espelho autoritativo; `sincronizar_sim` lê dele, nunca escreve nele coordenadas locais.
- **`JogadorLocal`**: estado de tela do avatar; passa a ser derivado do espelho via `metros_para_pixels`.
- **BF-012 / BF-003b / BF-013**: recorte fechado (plumbing do `mover`) vs. pendências abertas (mapa completo de 12 ilhas; consumo de `crates/contratos`).

### Rastreabilidade FR ↔ SC

| FR | SC | DoD |
|:--|:--|:--|
| FR-001 conversão determinística pixels↔metros | SC-001 escala explícita, conversão verificável por `--lib`, sem cânone novo | DoD-4 |
| FR-002 `mover_jogador` validado | SC-002 WASD gera `mover` validado; recusa preserva posição e monotônicos; imobilização bloqueia | DoD-1, DoD-2 |
| FR-003 sincronização espelho→local e reordenação | SC-003 avatar/HUD refletem somente o estado autoritativo aceito, sem segunda verdade | DoD-3 |
| FR-004 evidência/validação/release | SC-004 prova registrada, governança preservada, nenhuma issue simulada | DoD-5 |

## Success Criteria

- **SC-001 (escala e conversão verificável)**: a escala `PIXELS_POR_METRO = 100.0` é protótipo rotulado `[HIPÓTESE]`; `260 px/s ÷ 100 = 2,6 m/s` e `PASSO_SEGUNDOS = 1/30` dão ≈ 0,087 m/tick, abaixo de `LIMITE_MOVIMENTO_M = 1.0`; helpers puros são exercitados por `cargo test --locked -p insurgencia-cliente --lib` e todos os destinos/alvos usam metros. Nenhum cânone novo e nenhuma regra canônica alterada. **[FR-001]**
- **SC-002 (`mover` validado, sem counter burn)**: WASD/setas geram `Comando { acao: "mover", destino: metros }`, `ValidacaoComandos::validar` roda antes de `agendar_comando` e `seq`/`request_id` avançam somente no aceite; em `MovimentoForaDoLimite`, replay ou imobilização, posição e monotônicos permanecem intactos. **[FR-002]**
- **SC-003 (observação autoritativa)**: `sincronizar_sim` lê `Mundo.jogadores[PEER_ID].posicao` e escreve `JogadorLocal` em pixels, sem escrita local no espelho; a ordem `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar` é verificável por inspeção, e avatar/HUD refletem somente o estado aceito, sem segunda verdade ou dupla aplicação. **[FR-003]**
- **SC-004 (prova e governança)**: artefatos registram DoD-1..DoD-5 e score **144/3125** (`4×3×3×2×2`), roteiro de 14 passos, `./verify.sh` com jobs `gates`/`rust`, R4 antes de R7 `1.0.0-local`, `converge` append-only, R3 = 0 em `plan.md` (sem `clarify.md`), R6 **N/A** (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado), BF-012 fechado e BF-003b/BF-013 abertos, promoção/deploy SWE-exclusivos e PR #76 intocável. **[FR-004]**

## Clarificações

- **R3 = 0**: nenhuma clarificação em aberto; as incógnitas registradas (captura visual da janela Bevy, execução externa de gates e comportamento exato na fronteira de 1 m) são **limitações de escopo [NV]**, não ambiguidades bloqueantes. **Não há arquivo `clarify.md`**: o registro canônico de R3 fica em `plan.md`, etapa 07, e nesta seção.
- A escala `PIXELS_POR_METRO = 100.0` e os valores derivados (2,6 m/s, ≈ 0,087 m/tick) são escolha de protótipo fixada no contrato, não conteúdo canônico; `LIMITE_MOVIMENTO_M = 1.0` e o validador permanecem como autoridade única da sim.
- A direção da sincronização é espelho → local; a validação pertence à sim e o cliente apenas a consulta antes de agendar. Semeadura duplicada de movimento (escrita local + comando) foi rejeitada no scorecard por dividir a verdade do estado.
- Captura visual do avatar/HUD, `./verify.sh`, testes e jobs remotos são **[NV]**: nenhum build, gate, teste, formatador, commit, push ou PR é executado por este agente; a execução pertence ao orquestrador/SWE.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira (`spec.md`, `plan.md`, `checklists/requirements.md` e `.specify/feature.json`) no worktree `.specify/queue/worktrees/cycle-059/` (branch `feature/059-mover-validado`, base `origin/dev = 737b876`). Não edita código, assessment, validação, release, `STATE.md`, `AGENTS.md` ou `concurrency.jsonl`.
- O cliente já expõe `VELOCIDADE`, `deslocar`, `aproximar_no`, `ComandoLocal`, `Simulacao`, `mover_jogador`, `sincronizar_sim`, `sincronizar_avatar`, `avancar_simulacao` e os helpers de `solicitar_*`; a sim já expõe `ValidacaoComandos::validar`, `agendar_comando`, `Comando` e `LIMITE_MOVIMENTO_M`. Nenhum novo crate, contrato ou protocolo é necessário.
- O roteiro de verificação em `.specify/validations/mover-validado/test-roadmap.md` tem 14 passos, escritos antes do PR; o gate de destino é `./verify.sh` e o CI é GitHub Actions com jobs `gates`/`rust`. Toda execução/captura é **[NV]** até o orquestrador.
- Score **144/3125** (`4×3×3×2×2`), escolhido sobre BF-003b (dependente de decisão canônica de topologia), BF-013 (diagnóstico, 100/3125) e BF-014 (exibir hash não muda o jogo, 250/3125); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵ = 3125. Consumidor: jogador do cliente Bevy + validação da sim/CLI.

## Out of scope

Mudanças em GDD, qualquer regra canônica, números promovidos a cânone, mapa completo de 12 ilhas (BF-003b), topologia/rotas/spawning, consumo de `crates/contratos` por binários (BF-013), HUD com `hash_estado` (BF-014), networking/ADR-004, `Lightyear`, sockets, predição, reconciliação cliente-servidor, lobby, novas habilidades; mudanças em `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, baseline e documentos de outros ciclos **na fronteira do agente de documentação**; janelas normais, deploy, promoção `dev→main`, commits, push, PRs (inclusive PR #76), merge e emissão de issues (reais ou simuladas).

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. Nenhum manifesto de contexto é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade permanecem em `docs/governance/context-budget-manifest.md`.

**80/20**: `mover-validado` (BF-012) **4×3×3×2×2 = 144/3125**, contra BF-003b (270/3125 histórico inclui o BF-003a fechado e depende de topologia canônica), BF-014 **250/3125** (eliminado: diagnóstico) e BF-013 **100/3125** (`5×2×5×1×2`, eliminado: diagnóstico); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, escala 1–5, denominador 5⁵ = 3125. Nenhum valor do protótipo é promovido a cânone; BF-003b e BF-013 permanecem abertos.
