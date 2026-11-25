# Feature Specification: Ato de Vandalismo do Agitador — BF-002c (ciclo 061)

**Feature ID**: 061

**Status**: Especificado

**Branch**: `feature/061-vandalismo` · **Created**: 2026-11-25 · **Âncora**: `origin/dev = 52138c3` · **R1**: `go` · **Score**: 288/3125 (`4×4×2×3×3`) · **Sessão**: `insurgencia - Ato de Vandalismo do Agitador`.

**Input**: decisão 80/20 registrada em `.specify/memory/STATE.md` (2026-11-25T05:10:03Z). Consumidor nomeado: jogador da Função Agitadora no cliente Bevy e automação headless via `insurgencia-servidor`.

Todos os números desta spec — custo 20 de Fôlego, alcance 2 m (`ALCANCE_CONTATO_M`), dano 120, silêncio 45 ticks (1,5 s × 30 Hz) e recarga 210 ticks (7 s) — são **`[HIPÓTESE]`/protótipo** fixados nesta sessão. Nenhum deles é regra canônica: o GDD é lido, nunca reescrito, e nenhuma regra nova é inventada.

## User Scenarios

### US1 — Ação `vandalismo` reconhecida pelo contrato local (P1)

O núcleo determinístico passa a aceitar `vandalismo` como uma ação com destino. O token entra como 15º elemento de `ACOES_PERMITIDAS` em `crates/sim/src/lib.rs`, e `acao_exige_destino("vandalismo")` passa a devolver `true`, mantendo-se `false` apenas para `sinalizar_sincronia` e `sinalizar_alavanca`. A CLI `insurgencia-servidor` continua lendo linhas de 4 ou 6 tokens e publicando `<tick>\t<sha256>` antes de cada `avancar_tick`.

**Why this priority**: sem o token e a aridade correta, a ação é recusada como `AcaoDesconhecida` antes de qualquer validação ou efeito; é a condição de possibilidade de todos os demais cenários.

**Independent Test**: inspecionar `ACOES_PERMITIDAS` (15 entradas, `vandalismo` presente) e `acao_exige_destino` em `crates/sim/src/lib.rs`, e conferir em `crates/servidor/tests/cli.rs` que uma linha `peer vandalismo seq request_id x y` é aceita com a saída inalterada.

**Acceptance Scenarios**:

1. **Given** o contrato local da sim, **When** `ACOES_PERMITIDAS` é lido, **Then** contém 15 tokens e o último é `vandalismo`.
2. **Given** `acao_exige_destino`, **When** recebe `vandalismo`, **Then** devolve `true`; **When** recebe `sinalizar_sincronia` ou `sinalizar_alavanca`, **Then** devolve `false`.
3. **Given** a CLI headless, **When** recebe uma linha `vandalismo` com destino, **Then** mantém o formato `<tick>\t<sha256>` e não introduz parser local nem whitelist paralela.

### US2 — Validação determinística antes do efeito (P1)

`ValidacaoComandos::validar` passa a tratar `vandalismo` na mesma ordem das demais ações com alvo: par autorizado → `seq`/`request_id` estritamente crescentes → whitelist → nível → destino/alvo → alcance → Fôlego → recarga. O alvo precisa existir no ponto informado, o agressor precisa estar a no máximo 2 m (`ALCANCE_CONTATO_M`), ter pelo menos 20 de Fôlego e recarga zerada (a recarga dura 210 ticks após o uso). Qualquer recusa preserva o estado e os monotônicos.

**Why this priority**: a validação é a fronteira autoritativa compartilhada por sim, cliente e CLI; sem ela o efeito poderia ser aplicado fora de alcance, sem recurso ou em recarga.

**Independent Test**: chamar `ValidacaoComandos::validar` com alvo ausente, alvo a 2,01 m, 19 de Fôlego e recarga 210, conferindo um `CodigoRecusa` tipado em cada caso e nenhuma alteração em `seq`/`request_id`, `folego` ou estado do alvo.

**Acceptance Scenarios**:

1. **Given** alvo no destino, agressor a ≤ 2 m, Fôlego ≥ 20 e recarga 0, **When** a validação roda, **Then** devolve `Ok(())`.
2. **Given** alvo ausente ou a mais de 2 m, **When** a validação roda, **Then** devolve recusa de alvo/destino, sem dano e sem avanço de monotônicos.
3. **Given** Fôlego 19 ou recarga 210, **When** a validação roda, **Then** devolve `FolegoInsuficiente` ou recusa de recarga, preservando recurso e monotônicos.

### US3 — Efeito observável e determinístico no sim (P1)

O ramo de `vandalismo` em `fase_skills` aplica 120 de dano ao alvo pela rota única de dano a jogador (respeitando absorção de escudo e piso zero), impõe 45 ticks de silêncio e cancela a canalização em curso do alvo. O custo de 20 Fôlego e a recarga de 210 ticks são marcados no agressor somente no comando aceito; `seq`/`request_id` avançam unicamente no aceite. `hash_estado` passa a cobrir o novo estado, e decrescentes de recarga/silêncio acontecem uma vez por tick em ordem estável.

**Why this priority**: é o comportamento final do DoD-1 e a prova de que o protótipo tem efeito material, sem depender de regra canônica ou de topologia de ilhas.

**Independent Test**: em `crates/sim/tests/simulacao.rs`, aplicar um `vandalismo` válido e conferir dano exato, `silencio_ticks == 45`, campos de canalização zerados, Fôlego −20, recarga 210, monotônicos atualizados e `hash_estado` diferente da baseline mas igual entre replays equivalentes; repetir com comandos recusados e conferir estado inalterado.

**Acceptance Scenarios**:

1. **Given** um alvo com canalização ativa e escudo neutro, **When** um `vandalismo` válido é processado, **Then** perde 120 de vida, fica com 45 ticks de silêncio e tem a canalização cancelada.
2. **Given** um comando recusado, **When** o tick avança, **Then** nenhum dano, silêncio, custo, recarga ou monotônico muda; `ultima_recusa` registra a recusa e participa do hash.
3. **Given** dois mundos com a mesma sequência de comandos, **When** os hashes são comparados, **Then** são iguais; **Given** um mundo sem o vandalismo, **Then** o hash difere da baseline.

### US4 — Consumidor no cliente e HUD (P2)

No cliente Bevy, a tecla protótipo `H` agenda `vandalismo` com `destino` no ponto do alvo e `seq`/`request_id` monotônicos, pelo mesmo caminho das demais ações (`ValidacaoComandos::validar` antes de `agendar_comando`). `Q` permanece reservado a `atacar_forte`. O painel em `texto_hud` passa a exibir uma linha de estado legível para o efeito (silêncio/recarga do jogador local), sem segunda verdade e sem duplicar a sim.

**Why this priority**: sem o despacho e a dica na HUD, o efeito seria verificável apenas por teste interno da sim e o jogador da Função Agitadora não teria como exercitar o corte.

**Independent Test**: inspecionar em `crates/cliente/src/main.rs` o despacho de `KeyCode::KeyH`, a validação antes do agendamento, os monotônicos e a linha acrescentada em `texto_hud`; conferir que nenhuma janela Bevy, contrato, `PROTOCOL_VERSION` ou configuração de CI/Cargo foi acrescentada.

**Acceptance Scenarios**:

1. **Given** o jogador no cliente, **When** `H` é pressionado com alvo válido, **Then** um `vandalismo` com destino é agendado com IDs crescentes, sem alterar `Q`/`atacar_forte`.
2. **Given** um `H` recusado pela validação, **When** o comando é processado, **Then** os monotônicos locais não avançam e a posição permanece.
3. **Given** a HUD, **When** o estado muda, **Then** a linha exibida reflete somente o estado autoritativo da sim.

## Requirements

### Functional Requirements

- **FR-001 (contrato)**: `crates/sim/src/lib.rs` passa a ter `vandalismo` como 15º token de `ACOES_PERMITIDAS`; `acao_exige_destino("vandalismo")` devolve `true`, mantendo `false` apenas para `sinalizar_sincronia` e `sinalizar_alavanca`. A CLI `insurgencia-servidor` aceita a linha de 6 tokens e preserva `<tick>\t<sha256>`. **[HIPÓTESE]**, sem novo protocolo.
- **FR-002 (validação)**: `ValidacaoComandos::validar` exige alvo no destino, agressor a no máximo 2 m (`ALCANCE_CONTATO_M`), Fôlego ≥ 20 e recarga zerada (210 ticks após o uso). Recusa é tipada em `CodigoRecusa` e preserva estado e monotônicos.
- **FR-003 (efeito)**: em `fase_skills`, um `vandalismo` aceito aplica 120 de dano ao alvo pela rota única, impõe 45 ticks de silêncio, cancela a canalização em curso do alvo, consome 20 de Fôlego e marca 210 ticks de recarga no agressor. `seq`/`request_id` avançam apenas no aceite; `hash_estado` cobre o novo estado e os decrescentes ocorrem uma vez por tick.
- **FR-004 (cliente/HUD)**: `crates/cliente/src/main.rs` despacha a tecla protótipo `H` para `vandalismo` com destino e IDs monotônicos, preservando `Q` para `atacar_forte`; `texto_hud` exibe uma linha de estado derivada da sim, sem segunda verdade.

### Key Entities

- **`vandalismo`**: 15º token de `ACOES_PERMITIDAS`, com destino; único acréscimo ao contrato local neste corte.
- **`Jogador`**: passa a carregar marcas de recarga de `vandalismo` e o alvo passa a carregar ticks de silêncio e o cancelamento da canalização; campos cobertos por `hash_estado`, zerados em `Mundo::new`.
- **`Comando { acao: "vandalismo", destino: Some((x, y)) }`**: validado antes de `agendar_comando`; recusado não atualiza `seq`/`request_id`.
- **`CodigoRecusa`**: estende as recusas de alvo/alcance, Fôlego e recarga sem criar validação paralela no cliente.
- **`JogadorLocal` / `ComandoLocal`**: estado de tela e monotônicos do cliente; a tecla `H` usa a mesma fila de comandos das demais ações.
- **BF-002c / BF-015b**: recorte que fecha (`Ato de Vandalismo`) vs. pendência que permanece aberta (`sinalizar_alavanca`).

### Rastreabilidade FR ↔ SC

| FR | SC | DoD |
|:--|:--|:--|
| FR-001 contrato (`vandalismo` 15º, com destino) | SC-001 token e aridade observáveis | DoD-1, DoD-2 |
| FR-002 validação (alvo, 2 m, 20 Fôlego, 210 ticks) | SC-002 aceite e recusa determinísticos | DoD-1 |
| FR-003 efeito (120 dano, 45 silêncio, cancela canalização, monotônicos só no aceite, hash) | SC-003 efeito exato e hash coerente | DoD-1, DoD-3 |
| FR-004 cliente (`H` + HUD) | SC-004 consumidor exercita o fluxo | DoD-2, DoD-3, DoD-4 |

## Success Criteria

- **SC-001 (contrato mensurável)**: `ACOES_PERMITIDAS.len() == 15` com `vandalismo` presente; `acao_exige_destino("vandalismo") == true` e a CLI aceita a linha de 6 tokens sem alterar `<tick>\t<sha256>` (**[NV]** até execução).
- **SC-002 (validação mensurável)**: alvo no destino a ≤ 2,0 m com Fôlego ≥ 20 e recarga 0 é aceito; alvo ausente, distância 2,01 m, Fôlego 19 ou recarga 210 são recusados com `CodigoRecusa` tipado, e nenhum desses casos altera `seq`, `request_id`, Fôlego ou estado do alvo (**[NV]** até execução).
- **SC-003 (efeito mensurável)**: em comando aceito, o alvo perde exatamente 120 de vida pela rota única (com absorção de escudo e piso zero), fica com 45 ticks de silêncio e com a canalização zerada; o agressor fica com Fôlego −20 e recarga 210; `hash_estado` difere da baseline e é idêntico entre replays equivalentes. Em comando recusado, todas essas magnitudes permanecem inalteradas (**[NV]** até execução).
- **SC-004 (consumidor e governança mensurável)**: `KeyCode::KeyH` agenda `vandalismo` com destino e IDs crescentes, `Q` continua em `atacar_forte`, e `texto_hud` exibe a linha de estado; `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo test --locked -p insurgencia-cliente --lib` e `./verify.sh` (jobs `gates`/`rust`) são executados pelo orquestrador; BF-002c fecha e BF-015b permanece aberto, R6 N/A (**[NV]** nesta etapa documental).

## Clarifications

### Session 2026-11-25

- Q: A tecla H é protótipo? → A: Sim, `[HIPÓTESE]`; Q já pertence a `atacar_forte`.
- R3: uma pergunta, respondida nesta sessão; nenhuma ambiguidade bloqueante permanece e nenhum arquivo `clarify.md` é criado.

## Assumptions

- O worktree é `.specify/queue/worktrees/cycle-061-vandalismo` (branch `feature/061-vandalismo`, base `origin/dev = 52138c3`); a identidade vem de `.specify/feature.json`, nunca do nome do branch.
- `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs` e a CLI já oferecem as rotas reaproveitadas (`ACOES_PERMITIDAS`, `acao_exige_destino`, `ValidacaoComandos::validar`, `fase_skills`, `aplicar_dano_a_jogador`, `hash_estado`, despacho por tecla e `texto_hud`); nenhum crate, contrato de rede ou protocolo novo é necessário.
- A execução de testes, gates, formatadores, build, captura visual, commit, push, PR e deploy pertence ao orquestrador/SWE; toda saída permanece **[NV]** até ser colada com origem e carimbo.
- Venue para compilação pesada: GitHub Actions (`gates` e `rust`); não se rebaixa timeout local para RED nem se afirma verde sem saída observada.

## Out of scope

GDD e qualquer regra canônica (números do protótipo não são promovidos), CI e configuração de gates, contrato de rede/`PROTOCOL_VERSION`/Lightyear, qualquer forma de multiplayer real ou 5v5, mapa completo de 12 ilhas com topologia/rotas/spawning, `sinalizar_alavanca` (BF-015b), R6/`taskstoissues` (fora da sequência executável sob a constituição v1.2.2 — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado), promoção `dev→main`, deploy, commits, push, PRs (inclusive PR #76), merge e emissão de issues. Nenhum arquivo fora dos quatro artefatos deste ciclo é escrito nesta etapa.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê. Nenhum manifesto de contexto é criado neste ciclo (fora da fronteira). Honestidade de rótulos e limites de autoridade permanecem inalterados.

**80/20**: `vandalismo-agitador` (BF-002c) **4×4×2×3×3 = 288/3125**, contra `sinalizar_alavanca` (BF-015b) **144/3125** (`4×3×3×2×2`); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵ = 3125. Rejeitados: BF-003b (topologia canônica no GDD), BF-001/006/007/009/010 (autorização), BF-004/005 (execução externa), BF-013/014 (diagnóstico). Consumidor: jogador da Função Agitadora no cliente Bevy + automação via CLI `insurgencia-servidor`.
