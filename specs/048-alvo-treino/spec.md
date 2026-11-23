# Feature Specification: Alvo de treino jogável no cliente standalone

**Branch**: `048-alvo-treino` (base `fdc6c08`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T081800Z-alvo-treino`, `insurgencia - alvo de treino jogavel`.

**Âncora**: decisão 80/20 do ciclo 048 registrada em `.specify/memory/STATE.md` (2026-11-23T08:18Z): Alvo de treino no cliente standalone **3×4×4×5 = 240/625**, escolhida sobre Ilhas/condição de vitória **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. A ACR do ciclo 047 apontou que a tecla `Q` gera comando válido no core, mas o cliente standalone só tem `PEER_ID = 1` e nenhum alvo no `destino`, deixando o ataque inerte e a dica "Q para atacar" enganosa. **Todo o conteúdo desta spec é scaffolding de protótipo explicitamente não-canônico**: o alvo de treino é um marcador local no espelho do cliente, sem nova regra de produto, sem número promovido a cânone e sem mudança de sim, contrato, protocolo, stack ou rede. **R6 é N/A** por constituição v1.2.2.

## User Scenarios

### US1 — Constantes e alvo registrado no espelho do cliente (P1)

O cliente standalone passa a conhecer um segundo participante, o alvo de treino: constantes documentadas `PEER_ALVO_TREINO = 2` e `ALVO_TREINO_POSICAO = (2.0, 0.0)` identificam um peer sintético que `configurar` registra no espelho local de `Mundo` (`sim.adicionar_jogador(PEER_ALVO_TREINO)`) e **posiciona explicitamente** em `ALVO_TREINO_POSICAO`, ao lado do já existente `PEER_ID = 1`. O peer 2 existe apenas nesse espelho: nenhum pacote, socket, servidor ou segundo jogador humano é introduzido. O avatar nasce em `POSICAO_INICIAL = (0.0, -236.0)` e `sincronizar_sim` espelha seus movimentos no peer 1; portanto `Q` só causa dano quando o jogador se aproxima a `ALCANCE_CONTATO_M = 2.0` do alvo (`hypot > 2.0` recusa). O alvo é encontrado por igualdade exata de posição (`ponto_igual`), daí a necessidade de fixar o peer 2 no destino. Todos os valores são escolhas de protótipo, não cânone.

**Why this priority**: é a causa raiz do achado da ACR 047 — sem alvo no espelho, `Q` gera comando sintaticamente válido que a sim recusa com `CodigoRecusa::AlvoInvalido`; registrar o peer 2 é a menor mudança que torna o ataque observável sem tocar na sim.

**Independent Test**: inspecionar `crates/cliente/src/main.rs` e conferir as duas constantes documentadas, a chamada `adicionar_jogador(PEER_ALVO_TREINO)` em `configurar` e a ausência de qualquer alteração em `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION` e `Cargo.toml`.

**Acceptance Scenarios**:
1. **Given** o cliente standalone inicializado, **When** `configurar` monta o espelho `Mundo`, **Then** ele contém os peers `1` e `2`; o peer 2 tem `posicao = (2.0, 0.0)` atribuída explicitamente após `adicionar_jogador`, ambos com os campos padrão da sim (`vida = 100`, `folego = 100`, contadores zerados) e o peer 1 reflete a posição local via `sincronizar_sim`.
2. **Given** o peer 2 posicionado em `ALVO_TREINO_POSICAO`, **When** `aplicar_ataque` procura o alvo por `ponto_igual`, **Then** ele é encontrado no `destino` do comando, sem mudança na sim.
3. **Given** o código final, **When** as dependências e contratos são revisados, **Then** nenhuma dependência, identificador de protocolo, crate ou arquivo de contrato foi acrescentado.

### US2 — Marcador visível simples com `AlvoTreino` (P1)

`configurar` spawna um marcador geométrico simples — um cubo ou cone gerado com as malhas já disponíveis — na posição lógica `ALVO_TREINO_POSICAO` (transformação `Transform::from_xyz(x, ALTURA_ILHA + 3.0, -y)`, convenção já usada pelo avatar), com um componente `AlvoTreino` que o identifica no código. Não há asset externo, textura, animação, física, colisão, barra de vida 3D nem sistema de combate visual: o marcador existe apenas para que o alvo seja percebido e para dar suporte à leitura do HUD. É protótipo explícito, não uma entidade canônica do GDD.

**Why this priority**: sem representação visível o jogador não consegue associar a nova linha do HUD a nada no mundo; o marcador fecha o circuito percepção → ação → feedback com o mínimo de superfície.

**Independent Test**: inspecionar o spawn em `configurar` e confirmar que o componente `AlvoTreino` existe, usa malha e material padrão já carregados e não adiciona dependência ou asset ao projeto.

**Acceptance Scenarios**:
1. **Given** a cena inicial do cliente, **When** `configurar` termina, **Then** existe uma entidade com `AlvoTreino` cuja transformação 3D corresponde a `(2.0, 0.0)` no plano lógico.
2. **Given** o marcador, **When** a implementação é revisada, **Then** ela usa apenas `Assets<Mesh>`/`Assets<StandardMaterial>` já presentes, sem `AssetServer` novo, textura, física ou colisão.
3. **Given** o componente `AlvoTreino`, **When** o resto do código é lido, **Then** ele não altera nenhuma regra da sim nem cria comportamento de combate novo.

### US3 — Ataque com `Q` endereçado ao alvo (P1)

`solicitar_ataque` continua detectando `KeyCode::KeyQ` em `just_pressed` e mantendo `seq`/`request_id` monotônicos, mas troca o `destino`: em vez da própria posição do jogador local (que fazia a sim procurar outro peer naquele ponto e não encontrar), passa `Some(ALVO_TREINO_POSICAO)`. Toda a validação — existência de alvo no destino, alcance de contato de `2.0 m` e Fôlego mínimo — permanece na sim, exatamente como implementada no ciclo 047; o cliente apenas fornece um destino válido. O avatar nasce em `(0.0, -236.0)`, a cerca de 236 m do alvo, e `sincronizar_sim` mantém a posição do peer 1 igual à do jogador local; até que o jogador se aproxime a no máximo `2.0 m` do alvo, a sim recusa com `CodigoRecusa::AlvoInvalido`, sem debitar Fôlego nem alterar vida. A dica do HUD existe exatamente para orientar essa aproximação; ao chegar ao alcance, o ataque volta a surtir efeito.

**Why this priority**: é a correção direta do ataque inerte relatado pela ACR 047, obtida apenas com a mudança do `destino`; reescrever a validação na sim seria mudança de regra e está fora de fronteira.

**Independent Test**: inspecionar `solicitar_ataque` e conferir `destino: Some(ALVO_TREINO_POSICAO)`; comparar com a validação existente em `crates/sim/src/lib.rs` (`aplicar_ataque`) sem editá-la; verificar que uma pressão de `Q` agenda um único comando por tick.

**Acceptance Scenarios**:
1. **Given** o jogador aproximado a `(1.0, 0.0)` (a `1.0 m` do alvo), **When** `Q` é pressionado uma vez, **Then** um `atacar_forte` é agendado com `peer_id = 1`, `destino = Some((2.0, 0.0))` e `seq`/`request_id` incrementados em um.
2. **Given** o mesmo tick com o jogador dentro de `2.0 m` do alvo e Fôlego suficiente, **When** a sim processa o comando, **Then** o alvo perde `10` de vida (`saturating_sub` já existente) e o jogador perde `30` de Fôlego, sem qualquer alteração na sim.
3. **Given** o jogador a mais de `2.0 m` do alvo, **When** `Q` é pressionado, **Then** a sim recusa com `AlvoInvalido`, sem dano, sem débito de Fôlego e sem pânico.

### US4 — HUD com vida do alvo e dica fazível (P1)

`atualizar_texto` passa a ler `simulacao.0.jogadores.get(&PEER_ALVO_TREINO)` e exibe a linha **`Alvo: N`** — saída renderizada em que `N` é a vida lida do espelho; o identificador local `vida_alvo` é detalhe de implementação, não parte do formato exibido — ao lado das linhas já existentes (`Posicao`, `Ilha`, `Suprimentos`, `Materiais`, `Fôlego`, `Vida`, `Barreiras`, `Pontes`, `Súditos`, `Construindo`, `Coleta`). A dica final, hoje "`… · Q para atacar`", passa a ser **"Aproxime-se do alvo e use Q para atacar"**, substituindo uma orientação enganosa por uma instrução compatível com a validação de alcance da sim. Nenhum campo de protocolo novo é lido: a vida do alvo vem do mesmo espelho `Mundo` que o HUD já consulta.

**Why this priority**: sem a leitura da vida do alvo o efeito do ataque continua invisível; sem a dica corrigida o jogador continua sendo levado a crer que `Q` funciona em qualquer posição — os dois sintomas relatados pela ACR 047.

**Independent Test**: inspecionar a string formatada em `atualizar_texto` e conferir que a saída renderizada é **`Alvo: N`**, com `N` igual à vida de `jogadores.get(&PEER_ALVO_TREINO)` — o nome da variável local (`vida_alvo`) é detalhe de implementação e não deve ser exigido como literal —, além da dica exata e de todas as linhas e teclas anteriores; verificar que o valor exibido vem do espelho `Mundo`.

**Acceptance Scenarios**:
1. **Given** o alvo com `vida = 100`, **When** o HUD é renderizado, **Then** exibe `Alvo: 100` com o valor lido do espelho, não literal fixo.
2. **Given** um ataque aceito, **When** o HUD é atualizado, **Then** a linha passa a exibir `Alvo: 90` (ou o valor saturable correspondente), preservando `Fôlego` e `Vida` do jogador local.
3. **Given** o texto final do painel, **When** a dica é lida, **Then** ela contém exatamente "Aproxime-se do alvo e use Q para atacar" e nenhuma das teclas `WASD`/`B`/`P`/`C`/`G` deixou de ser anunciada.

### US5 — Determinismo e fronteira preservados (P2)

O peer 2, o marcador e o destino do ataque vivem somente no cliente. `crates/sim`, `crates/sim/tests`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI e a configuração de rede permanecem intactos; a cobertura determinística existente da sim continua válida sem edição de testes. Não há `Lightyear`, `async`, sockets, relógio externo, aleatoriedade, nova janela ou nova dependência. A ordem `BTreeMap`, o `hash_estado`, o `BTreeSet<u64>` de deduplicação e a regeneração de Fôlego seguem exatamente como no ciclo 047.

**Why this priority**: garante que a correção de um sintoma de cliente não reabra a rede bloqueada por ADR-004 nem invalide o determinismo que é a tese do projeto (Princípios II/III/VI).

**Independent Test**: revisar o diff final e confirmar que só `crates/cliente/src/main.rs` entre os paths de código foi alterado; executar, pelo orquestrador, a suíte determinística existente sem modificar expectativas.

**Acceptance Scenarios**:
1. **Given** o estado final do ciclo, **When** os arquivos alterados são listados, **Then** aparecem somente os 4 artefatos SDD e o path de código atribuído na etapa 12.
2. **Given** a suíte determinística da sim, **When** o orquestrador a executa, **Then** ela passa sem que nenhum teste tenha sido editado.
3. **Given** `hash_estado`, **When** duas execuções com o mesmo input são comparadas, **Then** permanecem idênticas, sem contribuição de relógio, rede ou iteração instável.

### US6 — Superfície de verificação e capturas (P2)

O roteiro de verificação usa inspeção de código e, quando executado pelo orquestrador, o binário do cliente em modo `--minimizado`, com capturas de terminal e, se houver surface automatizável, do texto do HUD. Não existe automação de captura visual real deste HUD no alvo: qualquer captura de tela, leitura de pixels ou verificação visual ao vivo é **[NV]** até execução humana, e nenhuma é prometida como verde nesta spec.

**Why this priority**: mantém a honestidade dos rótulos — o cliente é headless por decisão de governança do alvo e a ausência de surface automatizável não pode virar evidência fabricada.

**Independent Test**: ler a coluna "Automatizável?" do roteiro de 10 passos no `plan.md` e conferir que cada passo está marcado como inspeção, execução com `--minimizado` ou **[NV]**.

**Acceptance Scenarios**:
1. **Given** o roteiro de 10 passos, **When** ele é lido, **Then** cada passo declara sua superfície e os passos sem automação estão marcados **[NV]**.
2. **Given** uma execução com `--minimizado`, **When** não houver saída de terminal para o HUD, **Then** o resultado é registrado como **[NV]**, nunca como captura visual real obtida.
3. **Given** o rollback do roteiro, **When** um passo falha, **Then** a fonte é corrigida, nunca a expectativa do teste.

### US7 — Governança e estilo (P2)

R1 é `go`; a sequência da constituição v1.2.2 coloca `tasks` antes de `analyze` (READ-ONLY) e R4 (validate) antes de R7 (release `1.0.0-local`); R6 é **N/A** — se invocada, PARE e escale ao SWE, sem issue simulada e sem `gh` improvisado. O código segue `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100` e `///` em itens públicos (o atributo de lint `#[expect(clippy::... , reason = "...")]`, já usado no cliente, não é chamada `.expect()` e permanece permitido). Nenhum número do protótipo é promovido a cânone.

**Why this priority**: preserva a cadeia de rastreabilidade do hub e impede que uma frente pequena de cliente vire porta de entrada para dívida de automação, mudança de stack ou regra canônica.

**Independent Test**: inspecionar os 4 artefatos e o diff de código, conferindo as marcações de governança, a ordem das portarias e os atributos de lint.

**Acceptance Scenarios**:
1. **Given** os 4 artefatos, **When** a governança é revisada, **Then** R1 `go`, R6 N/A, `tasks` antes de `analyze` e R4 antes de R7 `1.0.0-local` aparecem explicitamente.
2. **Given** o código do cliente, **When** o estilo é conferido, **Then** não há `unsafe`, `unwrap`/`.expect()` ou `#[allow]`, e linhas até 100 colunas.
3. **Given** `(2.0, 0.0)`, `PEER_ALVO_TREINO = 2` e as linhas do HUD, **When** a spec é lida, **Then** todos aparecem rotulados como protótipo não-canônico.

## Edge Cases

- Jogador no spawn `(0.0, -236.0)`: distância até `(2.0, 0.0)` de cerca de 236 m → `CodigoRecusa::AlvoInvalido`; é o estado inicial esperado e a dica "Aproxime-se…" é essencial. Aproximando-se a `(1.0, 0.0)` (ou qualquer ponto a no máximo `2.0 m`), o ataque é aceito; a sim usa `>` para recusar, então exatamente `2.0 m` também é aceito.
- Jogador em `(2.01, 0.0)`: distância maior que `2.0` → `CodigoRecusa::AlvoInvalido`, sem dano e sem débito de Fôlego. Nenhuma mensagem de erro nova é inventada no cliente.
- Alvo com `vida = 5`: ataque de `10` satura em `0`; com `vida = 0` o marcador permanece e novos ataques continuam sendo aceitos pela sim existente (que não remove alvo morto), com `saturating_sub(...).max(0)`. Esse comportamento é congelado: a sim não é alterada.
- Fôlego do jogador: `100 → 70 → 40 → 10` em três ataques válidos; o quarto é recusado com `FolegoInsuficiente` pela regra existente; a regeneração de `10/s` também permanece como no ciclo 047.
- Dois comandos `atacar_forte` no mesmo tick: o `BTreeSet<u64>` da sim deduplica por atacante; o cliente mantém `seq`/`request_id` monotônicos, um comando por pressão de `Q`.
- `destino` do comando de movemento/coleta/construção/propaganda: continua sendo a posição local; somente `solicitar_ataque` muda para `ALVO_TREINO_POSICAO`.
- Marcador e avatar na mesma vizinhança: sem física ou colisão nova; sobreposição visual é aceitável no protótipo.
- `--minimizado`: `configurar` já reduz a janela; a entidade de texto do HUD continua existindo na cena, mas não há surface automatizável para ler pixels ou abrir a janela neste alvo — capturas visuais ficam **[NV]**.
- Ausência do peer 2 no espelho (por regressão): `AlvoInvalido` volta a ocorrer; o roteiro (passo 3) detecta por inspeção antes de qualquer dica enganosa.
- Contrato, protocolo, Cargo, sim, rede: qualquer mudança nesses paths é fora de fronteira e deve ser corrigida, nunca aceita como parte do protótipo.

## Requirements

### Functional Requirements

- **FR-001 (constantes e rotulação não-canônica)**: em `crates/cliente/src/main.rs`, constantes documentadas com `///` **`PEER_ALVO_TREINO: u64 = 2`** e **`ALVO_TREINO_POSICAO: (f64, f64) = (2.0, 0.0)`** identificam o alvo de treino; ambas trazem a marcação explícita de que são escolhas de protótipo e **nunca cânone**. A posição lógica foi escolhida para ficar a exatamente `ALCANCE_CONTATO_M` da origem do mapa, mas o spawn do avatar permanece `POSICAO_INICIAL = (0.0, -236.0)`; nenhuma constante, regra ou número é acrescentado a `crates/sim`, `crates/contratos` ou a qualquer documento canônico.
- **FR-002 (peer 2 e marcador no espelho do cliente)**: `configurar` registra o peer 2 no espelho local `Mundo` com `sim.adicionar_jogador(PEER_ALVO_TREINO)`, atribui explicitamente `jogadores.get_mut(&PEER_ALVO_TREINO).posicao = ALVO_TREINO_POSICAO` — necessário porque `adicionar_jogador` cria peers na origem e `aplicar_ataque` exige igualdade exata com o `destino` — e preserva `PEER_ID = 1`, cuja posição é atualizada pelo `sincronizar_sim` já existente. O mesmo `configurar` spawna um marcador simples (cubo ou cone) com transformação `Transform::from_xyz(x, ALTURA_ILHA + 3.0, -y)` em `ALVO_TREINO_POSICAO`, com componente **`AlvoTreino`**, usando malhas e materiais já carregados; sem asset externo, textura, animação, física, colisão ou barra de vida 3D. O peer e o marcador existem **somente no cliente**, nunca na sim, na rede ou em protocolo: scaffolding explícito de protótipo, não entidade canônica.
- **FR-003 (ataque endereçado)**: `solicitar_ataque` mantém `KeyCode::KeyQ` em `just_pressed`, `seq`/`request_id` monotônicos e um comando por pressão, mas passa **`destino: Some(ALVO_TREINO_POSICAO)`**. Toda validação (alvo no destino por `ponto_igual`, alcance de `2.0 m`, Fôlego, deduplicação por tick, dano com `saturating_sub`, combate de 90 ticks) permanece em `crates/sim` sem alteração; recusas `AlvoInvalido`/`FolegoInsuficiente` e regeneração seguem o comportamento do ciclo 047. No spawn o jogador está a cerca de 236 m e o ataque é recusado; após a aproximação via `sincronizar_sim`, passa a surtir efeito.
- **FR-004 (HUD)**: `atualizar_texto` lê `simulacao.0.jogadores.get(&PEER_ALVO_TREINO)` e exibe **`Alvo: N`** (saída renderizada com a vida do espelho; `vida_alvo` é apenas o nome da variável local, detalhe de implementação), preservando todas as linhas e teclas atuais; a dica final passa a ser exatamente **"Aproxime-se do alvo e use Q para atacar"**. Nenhum campo novo de protocolo, nenhuma leitura de rede.
- **FR-005 (determinismo/fronteira)**: nenhuma mudança em `crates/sim`, `crates/sim/tests`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI ou configuração de rede; sem `Lightyear`, `async`, sockets, relógio externo, aleatoriedade, nova janela ou dependência. A cobertura determinística existente da sim continua válida sem edição de testes; `hash_estado`, `BTreeMap`, `BTreeSet` e regeneração permanecem idênticos.
- **FR-006 (governança/estilo)**: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100`, `///` em itens públicos (atributo de lint `#[expect(...)]` permitido); R1 `go`, R6 **N/A** por v1.2.2, `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`; commits em PT-BR com staging explícito; gates, build, testes, formatadores, commit/push/PR **[NV]** para este agente.

### Key Entities

- **`PEER_ALVO_TREINO` / `ALVO_TREINO_POSICAO`**: constantes documentadas no cliente; identificam o peer sintético `2` e sua posição `(2.0, 0.0)` — protótipo não-canônico.
- **Espelho `Mundo` (cliente)**: `Simulacao(Mundo)` já existente; passa a conter os peers `1` (posição local espelhada por `sincronizar_sim`) e `2` (fixo em `(2.0, 0.0)`), sem origem remota.
- **`AlvoTreino`**: componente Bevy que marca o cubo/cone do alvo na cena; apenas identificação visual/local.
- **`solicitar_ataque`**: sistema de `Q`; único comportamento alterado é o `destino` do comando `atacar_forte`.
- **`atualizar_texto`**: sistema do HUD; ganha a linha renderizada `Alvo: N` (`N` = vida do alvo; `vida_alvo` é o nome da variável local) e a dica "Aproxime-se do alvo e use Q para atacar".
- **`CodigoRecusa` e constantes de combate**: pertencem à sim e ficam intactos; `AlvoInvalido`, `FolegoInsuficiente`, `ALCANCE_CONTATO_M = 2.0`, custo `30`, dano `10` e `90 ticks` são reutilizados, não redefinidos.

## Clarificações

- O achado corrigido vem da ACR do ciclo 047: `Q` já gera comando válido no core, mas `destino` era a própria posição do jogador e o espelho só tinha `PEER_ID = 1`, logo não havia alvo e a dica "Q para atacar" prometia um efeito que não ocorria.
- O alvo de treino é **scaffolding de protótipo explicitamente não-canônico**: `PEER_ALVO_TREINO = 2` e `(2.0, 0.0)` são escolhas locais de implementação, não regra de produto, não novo personagem do GDD e não alteração de balanço.
- O avatar nasce em `POSICAO_INICIAL = (0.0, -236.0)` e `sincronizar_sim` propaga sua posição ao peer 1; o peer 2 é fixado em `ALVO_TREINO_POSICAO` porque a busca de alvo usa igualdade exata (`ponto_igual`). A sim recusa apenas com `>` (acima de `2.0 m`), então exatamente `2.0 m` é aceito — mas o jogador precisa aproximar-se antes de atacar; a dica existe para isso.
- Vida, Fôlego, dano, alcance, deduplicação e regeneração permanecem **na sim**, com os valores do ciclo 047 (todos `[HIPÓTESE]`/protótipo naquela spec, nenhum cânone); este ciclo apenas fornece um destino válido e um feedback visual.
- O marcador não tem lifecycle de combate: se a vida do alvo chegar a `0`, o efeito existente da sim satura em `0` e o marcador permanece; não há remoção, animação de morte ou ponto de vitória.
- `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto e CI não são reabertos; ADR-004 mantém a rede bloqueada até decisão da SE.
- A vedação a `unwrap`/`expect` refere-se a chamadas de método; o atributo de lint `#[expect(clippy::needless_pass_by_value, reason = "...")]`, já presente no cliente, permanece permitido.
- **R6 é N/A** por constituição v1.2.2: fora da sequência executável até `resolve-tasks.sh` e do adaptador de issues existirem; se invocada, PARE e escale ao SWE, sem simular issue, sem `gh` improvisado e sem alegar execução.
- Capturas de HUD/visual não têm surface automatizável no alvo headless; resultados de execução, gates e capturas visuais ficam **[NV]** até o orquestrador. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0).

## Success Criteria

- **SC-001 (constantes e rotulação)**: `PEER_ALVO_TREINO = 2` e `ALVO_TREINO_POSICAO = (2.0, 0.0)` existem documentados em `crates/cliente/src/main.rs`, rotulados como protótipo não-canônico; a posição lógica está a exatamente `ALCANCE_CONTATO_M` da origem do mapa, enquanto o spawn do avatar permanece `(0.0, -236.0)`; nenhum valor foi promovido a regra de produto.
- **SC-002 (peer 2 e marcador)**: `configurar` registra o peer 2 no espelho `Mundo` ao lado do peer 1 e atribui explicitamente sua posição a `ALVO_TREINO_POSICAO` (necessário para a igualdade exata de `aplicar_ataque`), sem tocar na sim; spawna entidade simples com componente `AlvoTreino` e transformação `(x, ALTURA_ILHA + 3.0, -y)` usando apenas malhas/materiais disponíveis; nenhum asset externo, dependência, física, colisão, efeito canônico ou participante remoto é acrescentado.
- **SC-003 (ataque)**: `Q` agenda um único `atacar_forte` com `destino = Some((2.0, 0.0))` e `seq`/`request_id` monotônicos; no spawn a cerca de 236 m a sim recusa com `AlvoInvalido`, sem dano, sem débito e sem pânico; após aproximação a no máximo `2.0 m` (posição espelhada por `sincronizar_sim`), o alvo perde vida e o jogador perde Fôlego pela regra existente da sim, sem edição de nenhuma regra.
- **SC-004 (HUD)**: o painel exibe a linha renderizada **`Alvo: N`**, com `N` lido do peer 2 no espelho (o identificador `vida_alvo` no código é detalhe de implementação), e a dica exata "Aproxime-se do alvo e use Q para atacar"; todas as linhas e teclas anteriores permanecem e nenhum campo de protocolo é introduzido.
- **SC-005 (determinismo/fronteira)**: `crates/sim`, `crates/sim/tests`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI e rede permanecem intactos; a cobertura determinística existente passa sem edição; `hash_estado` e a ordem estável seguem idênticos entre execuções com o mesmo input.
- **SC-006 (governança/estilo)**: R1 `go`, R6 N/A, `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`; todos os valores do protótipo rotulados como não-canônicos; execução, capturas visuais e publicação **[NV]** até o orquestrador.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam `crates/cliente/src/main.rs` na etapa 12; o orquestrador executa gates, build, testes e verificações finais.
- Ambiente headless: nenhuma janela gráfica utilizável, rede, credencial ou comando `gh` é usado na redação destes artefatos; `--minimizado` é o modo previsto para eventuais execuções do binário pelo orquestrador.
- O cliente já tem `Simulacao(Mundo)`, `configurar`, `solicitar_ataque`, `atualizar_texto`, `Assets<Mesh>`/`Assets<StandardMaterial>` e o sistema de teclas com `Q` (ciclo 047), e a sim já tem `adicionar_jogador`, `ALCANCE_CONTATO_M`, `CodigoRecusa::AlvoInvalido` e `FolegoInsuficiente`; nenhum novo crate é necessário.
- O scorecard **240/625** é a escolha do ciclo 048: frente de cliente local, de maior Pontuação e menor risco de mudança de stack; ilhas/vitória (48/625) não são reabertas e a rede (10/625) segue bloqueada por ADR-004 até decisão da SE.

## Out of scope

Mudanças em `crates/sim`, `crates/sim/tests`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI ou rede; `Lightyear`, `async`, sockets, TCP, servidor, segundo jogador humano; física, projéteis, animação, efeitos visuais de combate, barra de vida 3D, mortalidade do alvo ou tela de vitória; progressão, níveis, habilidades W/E/R, alcances curto/médio/ilha; reabertura de barricada (044), ponte (045), súditos (046) ou Fôlego/combate (047); promoção de qualquer número a cânone; execução de gates, build, testes, formatadores, commits, push, PRs, merge ou deploy; alterações preexistentes fora da fronteira. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: Alvo de treino no cliente standalone **3×4×4×5 = 240/625**, escolhido sobre Ilhas/condição de vitória **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. Registro em `.specify/memory/STATE.md` (2026-11-23T08:18Z). Nenhum manifesto é criado e nenhum valor do protótipo é promovido a cânone.
