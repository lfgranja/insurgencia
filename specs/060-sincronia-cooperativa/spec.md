# Feature Specification: Sincronia cooperativa — BF-015a (ciclo 060)

**Branch**: `feature/060-sincronia-cooperativa` (base `origin/dev = e0e8f38`) · **Created**: 2026-11-24 · **Status**: Draft · **R1**: `go` · **Sessão**: `insurgencia - sincronia cooperativa`.

**Âncora**: GDD §4.4:209 ("entrada sincronizada de três jogadores em cinco segundos"), §8 (tabela de balanceamento — "Janela de sincronia"), §12.2:921/937 ("Barra de espaço: registra a presença do jogador..." / "Três jogadores sinalizam com a barra de espaço em 5,0 s → o desafio sincronizado inicia") e §12.3:943-950 ("Janela de sincronia de 5 s"). A ação `sinalizar_sincronia` já é whitelistada (`crates/sim/src/lib.rs:219`, `nucleo/sim/validacao.gd:4`), dispensa `destino` (`crates/sim/src/lib.rs:253-257`, `crates/servidor/tests/cli.rs:240`) e passa no gate de nível 1, mas `fase_skills` (`crates/sim/src/lib.rs:875`) não possui ramo para ela: hoje o comando apenas avança `seq`/`request_id`. **150 ticks = 5 s × 30 Hz**, três peers distintos e o signal `sincronia_iniciada:<tick>` são **`[HIPÓTESE]`/protótipo da sessão**, nunca cânone; o GDD é lido, nunca reescrito. **BF-015a** fecha neste ciclo; **BF-015b** (`sinalizar_alavanca`) e **BF-002c** (`Ato de Vandalismo`) permanecem **abertos**.

**Rastreabilidade canônica (mapeamento confirmado por Assessment/Tasks/Analyze/Roadmap)**: **FR-001** = estado do `Mundo` + janela/hash no sim — `pub const TICKS_JANELA_SINCRONIA: u64 = 150`, campos públicos `sincronias_iniciadas`, `sinalizadores_sincronia: BTreeSet<u64>`, `sincronia_ticks_restantes` zerados/vazios em `Mundo::new`, ramo em `fase_skills`, decremento em `avancar_tick`, três peers distintos com signal/reset e `hash_estado` cobrindo o novo estado com IDs ordenados; **FR-002** = cliente — tecla `Espaço` agenda `sinalizar_sincronia` com IDs monotônicos, `configurar` semeia o terceiro peer local, HUD exibe `Sincronias: N`, automação via CLI `insurgencia-servidor`; **FR-003** = testes de regressão da sim — cobertura determinística de inicialização, abertura da janela, três peers, signal, reset e hash, sem reescrever testes anteriores nem afrouxar expectativas; **FR-004** = evidência/validação/release. Cada FR tem um SC correspondente (SC-001..SC-004, 1:1) e uma etapa explícita no `plan.md`. DoD-1..DoD-4: (1) comportamento — primeiro sinal abre a janela de 150 ticks; terceiro peer distinto dentro da janela incrementa `sincronias_iniciadas` e emite `sincronia_iniciada:<tick>`; sinal fora da janela não conta e não corrompe o estado; `hash_estado` muda; (2) consumidor — jogador no cliente Bevy (tecla `Espaço`) e automação via CLI `insurgencia-servidor`; (3) comandos que provam — `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo test --locked -p insurgencia-cliente --lib`, `cargo check --locked -p insurgencia-cliente --all-targets`, `./verify.sh` remoto (`gates`/`rust`); (4) backlog — **BF-015a passa a `fechado`**; BF-015b e BF-002c permanecem abertos. Consumidor: **jogador do cliente Bevy + automação CLI headless**. Score **360/3125** (`5×4×3×2×3`).

## User Scenarios

### US1 — Estado e janela determinística no sim (P1)

O Núcleo passa a dar efeito observável a `sinalizar_sincronia` sem inventar topologia de ilhas. `crates/sim/src/lib.rs` expõe `pub const TICKS_JANELA_SINCRONIA: u64 = 150` (5 s × 30 Hz) e `Mundo` passa a ter os campos públicos `sincronias_iniciadas: u64`, `sinalizadores_sincronia: BTreeSet<u64>` e `sincronia_ticks_restantes: u64`, todos zerados/vazios em `Mundo::new()`. O ramo em `fase_skills` registra o peer, abrindo a janela no primeiro sinal quando `sincronia_ticks_restantes == 0`; `avancar_tick` decrementa exatamente uma vez por tick. Três peers distintos na mesma janela incrementam `sincronias_iniciadas`, emitem `sincronia_iniciada:<tick>` em `sinais` (`VecDeque<String>`, `crates/sim/src/lib.rs:552`) e resetam janela/conjunto; sinal fora da janela não conta e não corrói o estado. `hash_estado` (`crates/sim/src/lib.rs:2181`) passa a cobrir contador, ticks restantes e IDs ordenados, preservando a determinística dos replays. Nenhum dano de gás, expulsão, barra circular, rotas ou spawning de 12 ilhas é acrescentado.

**Why this priority**: a whitelist já autorizava a ação e o nível 1 já a permitia, mas sem representação, janela e ramo em `fase_skills` o comando só avança monotônicos. É a condição de possibilidade da coordenação observável e do hash que a CLI compara entre replays.

**Independent Test**: inspecionar `crates/sim/src/lib.rs` e confirmar a constante rotulada `[HIPÓTESE]`, os três campos em `Mundo::new()` zerados/vazios, o ramo em `fase_skills`, o decremento em `avancar_tick`, o signal no terceiro peer distinto e os três novos termos na `entrada` do `hash_estado`; verificar que dois mundos com os mesmos sinalizadores em ordens de inserção distintas produzem o mesmo hash e que nenhum termo novo foi escrito em contrato, protocolo, Cargo, CI ou GDD.

**Acceptance Scenarios**:
1. **Given** `Mundo::new()`, **When** o estado inicial é lido, **Then** `sincronias_iniciadas == 0`, `sinalizadores_sincronia` está vazio e `sincronia_ticks_restantes == 0`.
2. **Given** `sincronia_ticks_restantes == 0`, **When** o primeiro `sinalizar_sincronia` é processado, **Then** a janela abre com `TICKS_JANELA_SINCRONIA = 150` e o peer entra no conjunto, sem incrementar o contador.
3. **Given** três peers distintos na janela, **When** o terceiro é processado, **Then** `sincronias_iniciadas` incrementa exatamente uma vez, `sincronia_iniciada:<tick>` entra em `sinais` e janela/conjunto são resetados; sinal repetido ou fora da janela não conta.
4. **Given** um mundo com uma sincronia iniciada, **When** `hash_estado()` é comparado à baseline e a sequências equivalentes com ordens de inserção distintas, **Then** o hash difere da baseline e é idêntico entre replays equivalentes, com todos os números rotulados `[HIPÓTESE]`, nunca cânone.

### US2 — Consumidor no cliente e automação CLI (P1)

No cliente, a tecla `Espaço` passa a agendar `sinalizar_sincronia` com `seq`/`request_id` monotônicos pelo mesmo caminho das demais ações (`agendar_comando`), sem exigir `destino`. `configurar` semeia um terceiro peer local para que a janela de 150 ticks seja fechável por um único jogador no protótipo, e `texto_hud` (`crates/cliente/src/main.rs:1372`) passa a exibir `Sincronias: N` ao lado de `Sinais: N`. Na automação headless, `crates/servidor/src/main.rs:275` e `crates/servidor/tests/cli.rs:19` já analisam `sinalizar_sincronia` sem destino; a CLI continua publicando o mesmo formato `<tick>\t<sha256>` e passa a observar o signal. Nenhum protocolo, `PROTOCOL_VERSION`, Cargo, CI ou GDD é alterado.

**Why this priority**: é o consumidor nomeado no score 360/3125 (`5×4×3×2×3`). Sem a tecla e o terceiro peer local, a janela determinística seria verificável apenas por teste interno da sim; sem a CLI, a automação headless não conseguiria provar o signal. A HUD torna a regra legível sem texto longo, em coerência com GDD §12.3.

**Independent Test**: inspecionar o despacho da tecla `Espaço`, a semeadura do terceiro peer em `configurar`, a linha `Sincronias:` em `texto_hud` e os testes de aridade da CLI; confirmar que os IDs são monotônicos, que nenhuma janela de recurso Bevy, contrato, protocolo ou configuração de CI/Cargo foi acrescentada e que o formato `<tick>\t<sha256>` permanece.

**Acceptance Scenarios**:
1. **Given** o jogador no cliente Bevy, **When** `Espaço` é pressionado, **Then** um `sinalizar_sincronia` é agendado com `seq`/`request_id` crescentes e sem `destino`.
2. **Given** `configurar`, **When** o mundo local é montado, **Then** três peers locais existem e o jogador consegue fechar a janela sozinho no protótipo, sem duplicar sinalização do mesmo peer.
3. **Given** a HUD, **When** uma sincronia é iniciada, **Then** `Sincronias: N` reflete `sincronias_iniciadas`, sem segunda verdade e sem alterar o restante do painel.
4. **Given** a CLI `insurgencia-servidor`, **When** uma linha `sinalizar_sincronia` é processada, **Then** o formato `<tick>\t<sha256>` permanece e o signal é observável no estado, sem parser local nem whitelist nova.

### US3 — Testes de regressão da sim (P1)

`crates/sim/tests/simulacao.rs` ganha um teste determinístico que cobre o estado inicial zerado, a abertura da janela pelo primeiro sinal, o incremento no terceiro peer distinto, o signal `sincronia_iniciada:<tick>`, o reset de janela/conjunto, a deduplicação por `BTreeSet`, o desconto de sinal fora da janela e a mudança do `hash_estado`, incluindo a igualdade entre replays com IDs inseridos em ordens distintas. Os testes existentes — inclusive os de `sinalizar_sincronia` (`crates/sim/tests/simulacao.rs:3052`) e da CLI — permanecem inalterados, sem expectativa afrouxada e sem `unwrap()`/`.expect()`/`#[allow]` improvisados.

**Why this priority**: é a prova executável de FR-001 e do consumidor de FR-002. Sem o teste de regressão, a janela poderia ser removida ou o hash omitido sem detecção, e a automação headless não teria paridade verificável com a sim.

**Independent Test**: inspecionar o teste novo e confirmar que ele falharia se o incremento, o signal, o reset ou a cobertura do hash fossem removidos; confirmar que nenhum teste anterior foi reescrito e que a suíte é verificável por `cargo test --locked -p insurgencia-sim --test simulacao` (execução **[NV]** até o orquestrador).

**Acceptance Scenarios**:
1. **Given** `Mundo::new()`, **When** o teste roda, **Then** ele verifica contador zero, conjunto vazio e janela zerada antes de qualquer sinal.
2. **Given** uma sequência de três peers distintos e um repetido, **When** o teste avança a janela, **Then** exatamente uma sincronia é contada, um signal é emitido e o peer repetido não conta duas vezes.
3. **Given** uma sincronia iniciada, **When** o hash é comparado, **Then** ele difere da baseline e é igual entre replays equivalentes, com ordens de inserção distintas dos IDs.
4. **Given** a suíte existente, **When** a regressão é revisada, **Then** nenhum teste anterior foi alterado e nenhuma expectativa foi relaxada.

### US4 — Evidência, validação e release (P2)

Os artefatos do ciclo registram DoD-1..DoD-4, score **360/3125** (`5×4×3×2×3`), rastreabilidade FR-001..FR-004 ↔ SC-001..SC-004 ↔ etapas/T, roteiro de verificação escrito antes do PR, `converge.md` append-only, validação condicionada (R4) e release `1.0.0-local` (R7). BF-015a é marcado como **fechado**; BF-015b e BF-002c permanecem **abertos**. Gate de destino `./verify.sh`; CI = jobs `gates` e `rust` do GitHub Actions; CLI de PR = `gh`; `dev→main` e deploy são exclusivos da SE. PR #76 (`dev→main`) está `OPEN`, `mergeable=false/dirty`, checks `gates`/`rust` pass no run `45525589489`; pertence a outro owner ativo e não é tocado.

**Why this priority**: sem evidência e sem a distinção de backlog, o corte poderia ser confundido com a janela canônica de dano de gás de GDD §8 ou com o mapa de 12 ilhas. A governança v1.2.2 exige R4 antes de R7 e mantém R6 fora da cadeia executável.

**Independent Test**: inspecionar os artefatos e confirmar DoD-1..DoD-4 idênticos, FR/SC 1:1, **R3 = 0** em `plan.md` (sem arquivo `clarify.md`), ordem `tasks → analyze → implement → converge → validate → release`, R6 N/A, e ausência de build/gate/teste/formatador/commit/push/PR executados por este agente de documentação.

**Acceptance Scenarios**:
1. **Given** o estado final do ciclo, **When** a evidência é revisada, **Then** artefatos de assessment/validação/release existem, o roteiro cobre sim/janela/hash, cliente/CLI, testes e governança, com captura visual **[NV]**.
2. **Given** o backlog, **When** o fechamento é registrado, **Then** BF-015a aparece fechado, BF-015b e BF-002c permanecem abertos, sem promoção de valor prototípico a cânone e sem topologia de 12 ilhas decidida.
3. **Given** a constituição v1.2.2, **When** a cadeia é conferida, **Then** `tasks` precede `analyze` (READ-ONLY), `converge` é append-only, R4 vem antes de R7 `1.0.0-local`, R6/taskstoissues é N/A (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado) e promoção/deploy permanecem SWE-exclusivos.

### Edge Cases

- **Janela zerada**: o primeiro sinal abre a janela em `TICKS_JANELA_SINCRONIA`; sinal com janela já aberta apenas acrescenta o peer ao conjunto, sem reiniciar o contador.
- **Peer repetido**: reinserir o mesmo `peer_id` no conjunto não conta como novo participante; `BTreeSet<u64>` deduplica por construção.
- **Terceiro sinal fora da janela**: se `sincronia_ticks_restantes` chegou a zero, a janela expirou; o sinal não incrementa `sincronias_iniciadas` e não corrompe o estado — ele abre uma nova janela como primeiro sinal.
- **Reset atômico**: no fechamento da janela, contador de sinalizadores, ticks restantes e `sinais` são atualizados na mesma passagem de `fase_skills`, sem estado parcial observável pelo hash.
- **Ordem de inserção**: os IDs são serializados ordenados no hash; dois replays com a mesma sequência de peers, mas ordens de chegada diferentes dentro da janela, produzem o mesmo hash.
- **Valores prototípicos**: `150` = 5 s × 30 Hz, `3` peers e o signal `sincronia_iniciada:<tick>` são `[HIPÓTESE]` de GDD §8/§12, nunca cânone; GDD não é reescrito e `TICKS_JANELA_SINCRONIA` não é promovido a regra canônica.
- **Fora de escopo do GDD**: dano de gás de 8 pontos/s, expulsão após 3 s, barra circular e tolerância de rede de 0,2 s permanecem fora deste corte; nenhuma topologia de 12 ilhas, rotas ou spawning é decidido.
- **Convenção Rust**: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap()`/`.expect()`/`#[allow]` improvisados; o ramo de `fase_skills` reaproveita o despacho por `acao` e não cria validação paralela.
- **Fronteira**: nenhuma mudança em `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, CI, GDD, manifesto, `AGENTS.md`, `concurrency.jsonl`, `STATE.md`, PR #76 ou código de outro ciclo nesta fronteira documental.
- **R6 invocada por engano**: **PARE** e escale ao SWE; nenhuma issue simulada, nenhum `taskstoissues`/`gh` improvisado. `dev→main` e deploy são SWE-exclusivos.

## Requirements

### Functional Requirements

- **FR-001 estado do `Mundo` + janela/hash no sim**: `pub const TICKS_JANELA_SINCRONIA: u64 = 150` e campos públicos `sincronias_iniciadas`, `sinalizadores_sincronia: BTreeSet<u64>`, `sincronia_ticks_restantes` zerados/vazios em `Mundo::new`; ramo de `sinalizar_sincronia` em `fase_skills` registra o peer, `avancar_tick` decrementa uma vez por tick, primeiro sinal com janela zerada a abre, três peers distintos incrementam `sincronias_iniciadas`, emitem `sincronia_iniciada:<tick>` em `sinais` e resetam janela/conjunto, sinal fora da janela não conta; `hash_estado` cobre contador, ticks restantes e IDs ordenados; sem topologia/gás/cânone — `[HIPÓTESE]` de GDD §8/§12.
- **FR-002 cliente**: tecla `Espaço` agenda `sinalizar_sincronia` com IDs monotônicos, `configurar` semeia um terceiro peer local, HUD exibe `Sincronias: N`; automação via CLI `insurgencia-servidor` sem alterar protocolo/CI/Cargo/GDD.
- **FR-003 testes de regressão da sim**: `crates/sim/tests/simulacao.rs` cobre inicialização zerada, abertura da janela, três peers distintos, signal, reset, deduplicação, sinal fora da janela e hash determinístico com IDs ordenados, sem reescrever testes anteriores nem afrouxar expectativas.
- **FR-004 evidência/validação/release**: BF-015a fechado, BF-002c/BF-015b abertos, provas nomeadas, `./verify.sh` (`gates`/`rust`), R4 antes de R7 `1.0.0-local`, R6 N/A, nenhuma issue simulada, PR #76 intocável.

### Key Entities

- **`TICKS_JANELA_SINCRONIA`**: constante pública na sim, `150` ticks (5 s × 30 Hz), `[HIPÓTESE]`/protótipo da sessão; nunca cânone.
- **`Mundo.sincronias_iniciadas` / `Mundo.sincronia_ticks_restantes` / `Mundo.sinalizadores_sincronia`**: estado novo e determinístico, zerado/vazio em `Mundo::new`, atualizado em `fase_skills`, decrementado em `avancar_tick` e coberto por `hash_estado` com IDs ordenados.
- **`sincronia_iniciada:<tick>`**: signal empurrado em `sinais` (`VecDeque<String>`) no fechamento da janela; observável pela CLI e pelo hash.
- **`Comando { acao: "sinalizar_sincronia" }`**: já whitelistado e sem destino; o ramo em `fase_skills` é a única mudança de comportamento da ação.
- **`JogadorLocal` / `ComandoLocal`**: estado de tela e monotônicos do cliente; a tecla `Espaço` usa a mesma fila de comandos das demais ações.
- **BF-015a / BF-015b / BF-002c**: recorte fechado (efeito observável da sincronia) vs. pendências abertas (`sinalizar_alavanca`, `Ato de Vandalismo`).

### Rastreabilidade FR ↔ SC

| FR | SC | DoD |
|:--|:--|:--|
| FR-001 estado do `Mundo` + janela/hash no sim | SC-001 estado e janela determinísticos, efeito observável e hash cobrindo o estado, sem cânone novo | DoD-1, DoD-3 |
| FR-002 cliente | SC-002 consumidor nomeado exercita o fluxo sem alterar protocolo/CI/Cargo/GDD | DoD-2 |
| FR-003 testes de regressão da sim | SC-003 cobertura determinística, verificável por `--test simulacao`, sem expectativa afrouxada | DoD-1, DoD-3 |
| FR-004 evidência/validação/release | SC-004 prova registrada, governança preservada, nenhuma issue simulada | DoD-3, DoD-4 |

## Success Criteria

- **SC-001 (estado e janela determinísticos no sim)**: `TICKS_JANELA_SINCRONIA = 150` e os três campos de `Mundo` são zerados/vazios em `Mundo::new`; o ramo em `fase_skills` registra o peer, `avancar_tick` decrementa uma vez por tick, o primeiro sinal abre a janela e três peers distintos nela incrementam `sincronias_iniciadas`, emitem `sincronia_iniciada:<tick>` em `sinais` e resetam janela/conjunto, sem contar sinal fora da janela; `hash_estado` cobre contador, ticks restantes e IDs ordenados, difere da baseline e é idêntico entre replays equivalentes. Nenhum dano de gás, topologia de 12 ilhas ou número promovido a cânone. **[FR-001]**
- **SC-002 (consumidor cliente/CLI)**: no cliente Bevy, a tecla `Espaço` agenda `sinalizar_sincronia` com IDs monotônicos e sem `destino`, `configurar` semeia o terceiro peer local e a HUD mostra `Sincronias: N`; na CLI `insurgencia-servidor`, a linha mantém aridade, parser sem whitelist local e saída `<tick>\t<sha256>`, com o signal observável. Nenhuma alteração de protocolo, CI, Cargo ou GDD. **[FR-002]**
- **SC-003 (regressão determinística)**: o teste em `crates/sim/tests/simulacao.rs` cobre estado inicial zerado, abertura da janela, três peers distintos, signal, reset, deduplicação, sinal fora da janela e hash com IDs ordenados; falharia se qualquer um desses comportamentos fosse removido, e é executável por `cargo test --locked -p insurgencia-sim --test simulacao` (**[NV]** até o orquestrador). Testes anteriores permanecem inalterados e nenhuma expectativa é afrouxada. **[FR-003]**
- **SC-004 (prova e governança)**: artefatos registram DoD-1..DoD-4 e score **360/3125** (`5×4×3×2×3` — BF-015a contra BF-002c `288/3125` (`4×4×2×3×3`) e BF-015b `144/3125` (`4×3×3×2×2`)); roteiro de verificação escrito antes do PR, `./verify.sh` com jobs `gates`/`rust`, R4 antes de R7 `1.0.0-local`, `converge` append-only, **R3 = 0** em `plan.md` (sem `clarify.md`), **R6 N/A** (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado), BF-015a fechado e BF-015b/BF-002c abertos, promoção/deploy SWE-exclusivos e PR #76 intocável. **[FR-004]**

## Clarificações

- **R3 = 0**: nenhuma clarificação em aberto; a captura visual da janela Bevy, a execução externa de gates e o comportamento de rede além do protótipo são **limitações de escopo [NV]**, não ambiguidades bloqueantes. **Não há arquivo `clarify.md`**: o registro canônico de R3 fica em `plan.md`, etapa 07, e nesta seção.
- `150` ticks (5 s × 30 Hz), `3` peers distintos e o formato do signal são escolha de protótipo fixada no contrato a partir de GDD §8/§12, não conteúdo canônico promovido; o GDD é lido, nunca reescrito.
- A janela é puramente determinística e local ao `Mundo`: não há sincronização de rede, tolerância de 0,2 s, barra circular, dano de gás ou expulsão neste corte. A semeadura do terceiro peer no cliente é artifício de protótipo para tornar a janela fechável localmente, não uma decisão de topologia.
- O signal pertence à sim e a CLI apenas o observa; a HUD é leitura do estado da sim, sem segunda verdade. Nenhuma regra de validação paralela é introduzida no cliente.
- Captura visual da HUD, `./verify.sh`, testes e jobs remotos são **[NV]**: nenhum build, gate, teste, formatador, commit, push ou PR é executado por este agente; a execução pertence ao orquestrador/SWE.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira (`spec.md`, `plan.md`, `checklists/requirements.md` e `.specify/feature.json`) no worktree `.specify/queue/worktrees/cycle-060/` (branch `feature/060-sincronia-cooperativa`, base `origin/dev = e0e8f38`). Não edita código, assessment, validação, release, `STATE.md`, `AGENTS.md` ou `concurrency.jsonl`.
- A sim já expõe a whitelist em `crates/sim/src/lib.rs:219`, `acao_exige_destino` em `:253-257`, `sinais` em `:552`, `Mundo::new` em `:608`, `fase_skills` em `:875` e `hash_estado` em `:2181`; o cliente já tem `texto_hud` em `crates/cliente/src/main.rs:1372`, `configurar` e o despacho de comandos; a CLI já analisa `sinalizar_sincronia` em `crates/servidor/src/main.rs:275` e `crates/servidor/tests/cli.rs:19`. Nenhum novo crate, contrato ou protocolo é necessário.
- O roteiro de verificação é escrito antes do PR em `.specify/validations/sincronia-cooperativa/`, com 14 passos; o gate de destino é `./verify.sh` e o CI é GitHub Actions com jobs `gates`/`rust`. Toda execução/captura é **[NV]** até o orquestrador.
- Score **360/3125** (`5×4×3×2×3`), escolhido sobre BF-002c **288/3125** (`4×4×2×3×3`) e BF-015b **144/3125** (`4×3×3×2×2`); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵ = 3125. Consumidor: jogador do cliente Bevy (tecla `Espaço`) + automação via CLI `insurgencia-servidor`.

## Out of scope

Mudanças em GDD, qualquer regra canônica, números promovidos a cânone, janela de gás lacrimogêneo com 8 pontos/s e expulsão após 3 s (GDD §8), barra circular/tolerância de 0,2 s (GDD §12.3), mapa completo de 12 ilhas, topologia/rotas/spawning, `sinalizar_alavanca` (BF-015b), `Ato de Vandalismo` (BF-002c), redes/Lightyear, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, CI, manifesto, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, baseline e documentos de outros ciclos **na fronteira do agente de documentação**; janelas normais, deploy, promoção `dev→main`, commits, push, PRs (inclusive PR #76), merge e emissão de issues (reais ou simuladas).

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. Nenhum manifesto de contexto é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade permanecem em `docs/governance/context-budget-manifest.md`.

**80/20**: `sincronia-cooperativa` (BF-015a) **5×4×3×2×3 = 360/3125**, contra BF-002c **4×4×2×3×3 = 288/3125** e BF-015b **4×3×3×2×2 = 144/3125**; critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, escala 1–5, denominador 5⁵ = 3125. Rejeitados: BF-003b (topologia canônica no GDD), BF-001/006/007/009/010 (autorização), BF-004/005 (execução externa), BF-013/014 (diagnóstico). Nenhum valor prototípico é promovido a cânone; BF-015b e BF-002c permanecem abertos.
