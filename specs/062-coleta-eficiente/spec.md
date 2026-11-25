# Feature Specification: Coleta Eficiente do Camponês — BF-002k (ciclo 062)

**Feature ID**: 062

**Status**: Especificado

**Branch**: `feature/062-coleta-eficiente` · **Created**: 2026-11-25 · **Âncora**: `origin/dev = d3be560` · **R1**: `go` (registrado em `.specify/memory/STATE.md`) · **Score**: 1024/3125 (`4×4×4×4×4`) · **Sessão**: `insurgencia - Coleta Eficiente do Camponês`.

**Input**: decisão 80/20 registrada em `.specify/memory/STATE.md` (2026-11-25T06:40:58Z). Consumidor nomeado: jogador do cliente Bevy (tecla protótipo `K`) e automação headless via `insurgencia-servidor`.

Todos os números desta spec — custo 15 de Fôlego, buff de 150 ticks (5 s × 30 Hz) que reduz a conclusão da coleta de 180 para 90 ticks, e recarga de 300 ticks (10 s) — são **`[HIPÓTESE]`/protótipo** fixados nesta sessão a partir de `GDD:646`, lido e nunca reescrito. Nenhum deles é regra canônica; nenhuma regra nova é inventada.

## User Scenarios

### US1 — Ação `coleta_eficiente` reconhecida pelo contrato local (P1)

O núcleo determinístico passa a aceitar `coleta_eficiente` como uma ação com destino. A whitelist passa a ter 16 tokens no total, com `coleta_eficiente` presente, em `crates/sim/src/lib.rs`, e `acao_exige_destino("coleta_eficiente")` passa a devolver `true`, mantendo-se `false` apenas para `sinalizar_sincronia` e `sinalizar_alavanca`. A CLI `insurgencia-servidor` continua lendo linhas de 4 ou 6 tokens e publicando `<tick>\t<sha256>` antes de cada `avancar_tick`.

**Why this priority**: sem o token e a aridade corretos, a ação é recusada como `AcaoDesconhecida` antes de qualquer validação ou efeito; é a condição de possibilidade de todos os demais cenários.

**Independent Test**: inspecionar `ACOES_PERMITIDAS` (16 entradas, `coleta_eficiente` presente) e `acao_exige_destino` em `crates/sim/src/lib.rs`, e conferir em `crates/servidor/tests/cli.rs` que uma linha `peer coleta_eficiente seq request_id x y` é aceita com a saída inalterada.

**Acceptance Scenarios**:

1. **Given** o contrato local da sim, **When** `ACOES_PERMITIDAS` é lido, **Then** contém 16 tokens no total, com `coleta_eficiente` presente.
2. **Given** `acao_exige_destino`, **When** recebe `coleta_eficiente`, **Then** devolve `true`; **When** recebe `sinalizar_sincronia` ou `sinalizar_alavanca`, **Then** devolve `false`.
3. **Given** a CLI headless, **When** recebe uma linha `coleta_eficiente` com destino, **Then** mantém o formato `<tick>\t<sha256>` e não introduz parser local nem whitelist paralela.

### US2 — Validação determinística antes do efeito (P1)

`ValidacaoComandos::validar` passa a tratar `coleta_eficiente` na mesma ordem das demais ações com alvo: par autorizado → `seq`/`request_id` estritamente crescentes → whitelist → nível → destino → Fôlego → recarga → ausência de buff ativo. O destino deve ser exatamente a posição corrente do próprio jogador, ele precisa ter pelo menos 15 de Fôlego, recarga zerada e nenhum buff de coleta em curso — novas aplicações não acumulam (sem *stacking*). Qualquer recusa preserva Fôlego, monotônicos, coleta em curso e hash.

**Why this priority**: a validação é a fronteira autoritativa compartilhada por sim, cliente e CLI; sem ela o buff poderia ser aplicado à distância, sem recurso, em recarga ou empilhado sobre um buff ativo.

**Independent Test**: chamar `ValidacaoComandos::validar` com destino distante da posição, 14 de Fôlego, recarga 300 e buff 150 ativo, conferindo um `CodigoRecusa` tipado em cada caso e nenhuma alteração em `seq`/`request_id`, `folego`, campos de buff/recarga ou hash.

**Acceptance Scenarios**:

1. **Given** destino igual à posição do jogador, Fôlego ≥ 15, recarga 0 e nenhum buff ativo, **When** a validação roda, **Then** devolve `Ok(())`.
2. **Given** destino diferente da posição, **When** a validação roda, **Then** devolve recusa de destino, sem custo, sem buff e sem avanço de monotônicos.
3. **Given** Fôlego 14, recarga 300 ou buff 150 ativo, **When** a validação roda, **Then** devolve `CodigoRecusa` tipado, preservando Fôlego, monotônicos e o buff em curso (sem acumulação).

### US3 — Efeito observável e determinístico no sim (P1)

No ramo aceito de `fase_skills`, o comando consome 15 de Fôlego, marca 150 ticks de buff e 300 ticks de recarga no agressor. Enquanto o buff está ativo, a coleta de materiais conclui em 90 ticks em vez dos 180 ticks usuais (`TICKS_COLETA`), sem acelerar tick a tick nem alterar a recompensa. `seq`/`request_id` avançam unicamente no aceite; os decrescentes de buff e recarga ocorrem exatamente uma vez por tick em ordem estável, e `hash_estado`/`serializar_jogadores` passam a cobrir os campos novos.

**Why this priority**: é o comportamento final do DoD-1 e a prova de que o protótipo tem efeito material sobre a coleta, sem depender de regra canônica, Ouro/XP ou topologia de ilhas.

**Independent Test**: em `crates/sim/tests/simulacao.rs`, aplicar um `coleta_eficiente` válido e conferir Fôlego −15, buff 150, recarga 300, conclusão da coleta em 90 ticks contra 180 na baseline, monotônicos atualizados só no aceite e `hash_estado` diferente da baseline mas igual entre replays equivalentes; repetir com comandos recusados e com reaplicação durante o buff, conferindo estado inalterado e sem acumulação.

**Acceptance Scenarios**:

1. **Given** um jogador com 15 de Fôlego, recarga 0 e sem buff, **When** um `coleta_eficiente` válido é processado, **Then** fica com Fôlego −15, 150 ticks de buff e 300 ticks de recarga.
2. **Given** coleta iniciada sob buff, **When** 90 ticks se passam no mesmo ponto, **Then** a coleta conclui; **Given** coleta sem buff, **Then** conclui em 180 ticks.
3. **Given** um comando recusado ou uma reaplicação durante o buff, **When** o tick avança, **Then** nenhum custo, buff extra, recarga nova ou monotônico muda; `ultima_recusa` preserva o valor anterior e participa do hash.
4. **Given** dois mundos com a mesma sequência de comandos, **When** os hashes são comparados, **Then** são iguais; **Given** um mundo sem o ativo, **Then** o hash difere da baseline.

### US4 — Consumidor no cliente e HUD (P2)

No cliente Bevy, a tecla protótipo `K` (livre) agenda `coleta_eficiente` com `destino` igual à posição do jogador local e `seq`/`request_id` monotônicos, pelo mesmo caminho das demais ações (`ValidacaoComandos::validar` antes de `agendar_comando`). O painel em `texto_hud` passa a exibir uma linha de estado legível (buff/recarga/Fôlego do jogador local), derivada do espelho autoritativo da sim, sem segunda verdade.

**Why this priority**: sem o despacho e a dica na HUD, o efeito seria verificável apenas por teste interno da sim e o jogador do Camponês não teria como exercitar o corte.

**Independent Test**: inspecionar em `crates/cliente/src/main.rs` o despacho de `KeyCode::KeyK`, a validação antes do agendamento, os monotônicos e a linha acrescentada em `texto_hud`; conferir que nenhuma janela Bevy, contrato, `PROTOCOL_VERSION` ou configuração de CI/Cargo foi acrescentada.

**Acceptance Scenarios**:

1. **Given** o jogador no cliente, **When** `K` é pressionado com Fôlego e recarga válidos, **Then** um `coleta_eficiente` com destino na própria posição é agendado com IDs crescentes.
2. **Given** um `K` recusado pela validação, **When** o comando é processado, **Then** os monotônicos locais não avançam, o Fôlego é preservado e a posição permanece.
3. **Given** a HUD, **When** o buff ou a recarga muda, **Then** a linha exibida reflete somente o estado autoritativo da sim.

### Edge Cases

- `destino` a qualquer distância diferente da posição corrente: recusa de destino, sem consumo de 15 Fôlego e sem avanço de monotônicos.
- Fôlego 14 (abaixo do custo 15), recarga 300 ou buff 150 ainda ativo: recusa tipada; reaplicação nunca acumula duração nem reduz a recarga.
- Jogador silenciado, imobilizado ou abaixo do nível mínimo: continua valendo a regra existente de bloqueio; nenhuma exceção específica é criada para o ativo.
- Mudança de posição durante a coleta: o progresso existente é reiniciado conforme a regra atual de `coleta_ponto`, preservando a comparação determinística por `f64::total_cmp`.
- Replay equivalente com o mesmo estado inicial e a mesma sequência de comandos: hashes iguais; inclusão dos campos novos altera a baseline uma única vez, atualizada junto com o código.

## Requirements

### Functional Requirements

- **FR-001 (contrato)**: `crates/sim/src/lib.rs` passa a ter novo token `coleta_eficiente` (whitelist com 16 tokens) em `ACOES_PERMITIDAS`; `acao_exige_destino("coleta_eficiente")` devolve `true`, mantendo `false` apenas para `sinalizar_sincronia` e `sinalizar_alavanca`. A CLI `insurgencia-servidor` aceita a linha de 6 tokens e preserva `<tick>\t<sha256>`. **[HIPÓTESE]**, sem novo protocolo.
- **FR-002 (validação)**: `ValidacaoComandos::validar` exige destino exatamente igual à posição do próprio jogador, Fôlego ≥ 15, recarga zerada e nenhum buff de coleta ativo; reaplicação não acumula (sem *stacking*). Recusa é tipada em `CodigoRecusa` e preserva Fôlego, `seq`/`request_id`, progresso de coleta e hash. **[HIPÓTESE]**.
- **FR-003 (efeito)**: em `fase_skills`, um `coleta_eficiente` aceito consome 15 de Fôlego, marca 150 ticks de buff e 300 ticks de recarga no agressor; sob buff a coleta conclui em 90 ticks contra 180 sem buff, com a mesma recompensa. `seq`/`request_id` avançam apenas no aceite; decrescentes de buff/recarga ocorrem exatamente uma vez por tick em ordem estável, e `serializar_jogadores`/`hash_estado` cobrem os campos novos. **[HIPÓTESE]**.
- **FR-004 (cliente/HUD)**: `crates/cliente/src/main.rs` despacha a tecla protótipo `KeyCode::KeyK` para `coleta_eficiente` com destino na posição local e IDs monotônicos, validando antes de `agendar_comando`; `texto_hud` exibe uma linha de estado (buff, recarga, Fôlego) derivada da sim, sem segunda verdade. **[HIPÓTESE]**.

### Key Entities

- **`coleta_eficiente`**: novo token `coleta_eficiente` (whitelist com 16 tokens), com destino; único acréscimo ao contrato local neste corte.
- **`Jogador`**: passa a carregar ticks de buff de coleta e de recarga do ativo em `crates/sim/src/lib.rs`, inicializados em `Mundo::new` e cobertos por `serializar_jogadores`/`hash_estado` — sem `HashMap`, sem relógio externo.
- **`Comando { acao: "coleta_eficiente", destino: Some((x, y)) }`**: validado antes de `agendar_comando`; recusado não atualiza `seq`/`request_id` nem Fôlego.
- **`CodigoRecusa`**: estende as recusas de destino, Fôlego e recarga sem criar validação paralela no cliente.
- **`TICKS_COLETA` / meta de coleta**: 180 ticks na baseline e 90 ticks sob buff; a meta é derivada do estado autoritativo, não de uma regra de interface.
- **`JogadorLocal` / `ComandoLocal`**: estado de tela e monotônicos do cliente; a tecla `K` usa a mesma fila de comandos das demais ações.
- **BF-002k / passiva Ouro-XP**: recorte que fecha (`Coleta Eficiente`) vs. benefício passivo adiado por ausência de primitiva no núcleo.

### Rastreabilidade FR ↔ SC

| FR | SC | DoD |
|:--|:--|:--|
| FR-001 contrato (`coleta_eficiente` presente em whitelist de 16 tokens, com destino) | SC-001 token e aridade observáveis | DoD-1, DoD-2 |
| FR-002 validação (destino = posição, 15 Fôlego, recarga 300, sem *stacking*) | SC-002 aceite e recusa determinísticos | DoD-1 |
| FR-003 efeito (buff 150, 90 vs 180 ticks, monotônicos só no aceite, hash) | SC-003 efeito exato e hash coerente | DoD-1, DoD-3 |
| FR-004 cliente (`K` + HUD) | SC-004 consumidor exercita o fluxo | DoD-2, DoD-3, DoD-4 |

## Success Criteria

- **SC-001 (contrato mensurável)**: `ACOES_PERMITIDAS.len() == 16` com `coleta_eficiente` presente; `acao_exige_destino("coleta_eficiente") == true`, `false` apenas para `sinalizar_sincronia`/`sinalizar_alavanca`, e a CLI aceita a linha de 6 tokens sem alterar `<tick>\t<sha256>` (**[NV]** até execução).
- **SC-002 (validação mensurável)**: destino igual à posição com Fôlego ≥ 15, recarga 0 e sem buff é aceito; destino distante, Fôlego 14, recarga 300 ou buff 150 ativo são recusados com `CodigoRecusa` tipado, e nenhum desses casos altera `seq`, `request_id`, Fôlego, progresso de coleta ou hash (**[NV]** até execução).
- **SC-003 (efeito mensurável)**: em comando aceito, Fôlego cai exatamente 15, o buff marca 150 ticks e a recarga 300, a coleta conclui em 90 ticks sob buff contra 180 na baseline, com a mesma recompensa; os decrescentes ocorrem uma vez por tick e `hash_estado` difere da baseline e é idêntico entre replays equivalentes. Em comando recusado ou reaplicado sob buff, todas essas magnitudes permanecem inalteradas (**[NV]** até execução).
- **SC-004 (consumidor e governança mensurável)**: `KeyCode::KeyK` agenda `coleta_eficiente` com destino na posição local e IDs crescentes, e `texto_hud` exibe a linha de estado derivada da sim; `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo check --locked -p insurgencia-cliente --all-targets` e `./verify.sh` (jobs `gates`/`rust` no GitHub Actions) são executados pelo orquestrador; BF-002k passa a `fechado`, os demais backlog permanecem abertos, R6 N/A (**[NV]** nesta etapa documental).

## Clarifications

### Session 2026-11-25

- Q: A tecla `K` é protótipo e a passiva de Ouro/XP entra neste corte? → A: A tecla `K` é protótipo **`[HIPÓTESE]`** (livre no cliente). A passiva de Ouro/XP fica **fora de escopo** porque não existe primitiva de Ouro/XP no núcleo determinístico; criá-la seria inventar regra canônica sem lastro. R3: uma pergunta, respondida nesta sessão; nenhuma ambiguidade bloqueante permanece e nenhum arquivo `clarify.md` é criado.

## Assumptions

- O worktree é `.specify/queue/worktrees/cycle-062-coleta-eficiente` (branch `feature/062-coleta-eficiente`, base [EV] `origin/dev = d3be560`); a identidade vem de `.specify/feature.json`, nunca do nome do branch.
- `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs` e a CLI já oferecem as rotas reaproveitadas (`ACOES_PERMITIDAS`, `acao_exige_destino`, `ValidacaoComandos::validar`, `fase_skills`, `TICKS_COLETA`, `serializar_jogadores`, `hash_estado`, despacho por tecla e `texto_hud`); nenhum crate, contrato de rede ou protocolo novo é necessário.
- Os números de `GDD:646` são lidos como protótipo e rotulados `[HIPÓTESE]`; o GDD não é editado e nenhuma regra canônica é promovida.
- PR #76 (`dev→main`) é de outro dono, verde mas `CONFLICTING`, e permanece intocável; promoção `dev→main`, deploy, auto-merge, force-push e reescrita de história são SWE-exclusivos.
- A execução de testes, gates, formatadores, build, captura visual, commit, push, PR e deploy pertence ao orquestrador/SWE; toda saída permanece **[NV]** até ser colada com origem e carimbo.
- Venue para compilação pesada: GitHub Actions (jobs `gates` e `rust`); não se rebaixa timeout local para RED nem se afirma verde sem saída observada.

## Out of scope

GDD e qualquer regra canônica (números de `GDD:646` não são promovidos), CI e configuração de gates, contrato de rede/`PROTOCOL_VERSION`/Lightyear, qualquer forma de multiplayer real ou 5v5, mapa completo de 12 ilhas com topologia/rotas/spawning, passiva de Ouro/XP (sem primitiva no núcleo), R6/`taskstoissues` (fora da sequência executável sob a constituição v1.2.2 — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado), promoção `dev→main`, deploy, commits, push, PRs (inclusive PR #76), merge, auto-merge, force-push e emissão de issues. Nenhum arquivo fora dos quatro artefatos deste ciclo é escrito nesta etapa.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê. Nenhum manifesto de contexto é criado neste ciclo (fora da fronteira). Honestidade de rótulos e limites de autoridade permanecem inalterados.

**80/20**: `Coleta Eficiente do Camponês` (BF-002k) **4×4×4×4×4 = 1024/3125**, contra BF-002n `216/3125`, BF-015b `144/3125` (inelegível: ordem/topologia canônica não definida em `GDD:376,381,1502`), BF-003b `100/3125` (inelegível: topologia) e BF-014 `250/3125` (diagnóstico); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵ = 3125. Rejeitados: BF-001/006/007/009/010 (autorização/capacidade SWE), BF-004/005 (execução externa) e BF-002d..j/l..t (primitivas ausentes: invisibilidade, torres, canalização genérica, Ouro, topologia). Consumidor: jogador do Camponês no cliente Bevy (tecla `K`) + automação via CLI `insurgencia-servidor`.
