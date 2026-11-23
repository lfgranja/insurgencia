# Feature Specification: Papéis materiais, níveis e desbloqueio de ações existentes

**Branch**: `052-papeis-niveis` (base `4f3966e`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T150800Z-papeis-niveis`, `insurgencia - papeis e niveis`.

**Âncora**: decisão 80/20 do ciclo 052 registrada em `.specify/memory/STATE.md` (2026-11-23T15:08Z): **Papéis + níveis + desbloqueio Q/W/E/R = 4×2×2×3 = 48/625**, escolhida sobre PVE/PNJs **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. O desempate com PVE/PNJs (ambas 48/625) cabe a esta frente: GDD §10.1 já fixa a progressão e o protótipo só precisa gates sobre comandos **existentes**, sem PNJs, mapa ou stack nova. A regra canônica é GDD §10.1:515-528: funções são **papéis materiais na insurgência, não classes mágicas** (§10:513), com teto de nível **12**, desbloqueio **Q no nível 1, W no 2, E no 3, R no 6** e papéis **Artesão, Guerreiro, Intelectual, Camponês, Agitador** (§10.2–10.6:536-709, resumo em §10.8:744-759). **Todos esses valores são `[HIPÓTESE]` no GDD** e entram como escolha de protótipo, nunca como cânone operacional. **36000 ticks** pertencem ao ciclo 051 e não são reabertos; aqui **900 ticks = 30 s a 30 Hz** é uma derivação de `PASSO_SEGUNDOS = 1 / 30`, também rotulada como protótipo.

## User Scenarios

### US1 — Papel material observável (P1)

O jogador passa a ter um papel material explícito no estado autoritativo e no HUD: `Funcao` enum com exatamente cinco variantes (`Artesao`, `Guerreiro`, `Intelectual`, `Campones`, `Agitador`), `Jogador.funcao` preenchida em `adicionar_jogador` e a linha `Função: <nome>` no painel já existente. Os nomes exibidos são `Artesão`, `Guerreiro`, `Intelectual`, `Camponês` e `Agitador` — papéis materiais (P2), sem magia, sem "classes mágicas" e sem promoção a regra canônica. Não há seleção de papel por tecla, menu, comando ou protocolo neste ciclo: o valor inicial é fixo e determinístico (`Funcao::Artesao`) para todo jogador adicionado, e o papel **não altera** o efeito de nenhuma habilidade existente — ele é identificador e rótulo, não multiplicador, bônus ou restrição. A ausência de escolha é simplificação documentada: GDD §10.1 fala em conjunto base de funções, mas o protótipo ainda não tem fluxo de seleção, e criar um seria uma frente nova.

**Why this priority**: é a menor superfície que torna o papel observável sem inventar habilidade, input ou contrato; sem o enum, o campo e a linha no HUD, nenhum critério de GDD §10.1 sobre funções materiais pode ser inspecionado no estado determinístico.

**Independent Test**: inspecionar `crates/sim/src/lib.rs` (`Funcao`, `Jogador.funcao`, `Mundo::adicionar_jogador`) e `crates/cliente/src/main.rs` (`texto_hud`); confirmar cinco variantes, valor inicial determinístico, nome exibido e nenhuma tecla, comando ou efeito de papel novo.

**Acceptance Scenarios**:
1. **Given** um `Mundo` recém-criado, **When** `adicionar_jogador(1)` é chamado, **Then** `jogadores[&1].funcao = Funcao::Artesao` e o hash muda em relação ao mundo sem esse jogador, sem campo de protocolo novo.
2. **Given** `jogadores[&1].funcao` em cada uma das cinco variantes, **When** `texto_hud` é montado, **Then** a linha `Função:` exibe, respectivamente, `Artesão`, `Guerreiro`, `Intelectual`, `Camponês` e `Agitador`, sem acentuação perdida e sem alterar as demais linhas.
3. **Given** dois jogadores com papéis diferentes, **When** executam a mesma ação existente no mesmo tick, **Then** o efeito determinístico é idêntico: papel não gera dano, custo, alcance, recarga ou gate adicional neste ciclo.
4. **Given** o cliente final, **When** o tratamento de `KeyCode` é inspecionado, **Then** não há tecla de seleção de papel e nenhuma tecla existente passa a mudar `Jogador.funcao`.

### US2 — Progressão determinística por nível (P1)

`Jogador.nivel` começa em **1** e sobe **+1 a cada 900 ticks** (`TICKS_POR_NIVEL: i64 = 900`, derivado de 30 s × 30 Hz), saturando em **12** (`NIVEL_MAXIMO: i64 = 12`, `[HIPÓTESE]`/protótipo de GDD §10.1:527-528). A atualização acontece em `fase_prepare`, antes da aplicação dos comandos do tick, percorrendo `Mundo.jogadores` em ordem de `peer_id` (`BTreeMap`); com `tick = 900` o nível passa a 2, com `tick = 1800` passa a 3, e com `tick >= 10800` permanece 12. O relógio é o `tick: i64` existente: nenhum `Instant`, `SystemTime`, `sleep`, aleatoriedade, I/O ou contador por jogador é acrescentado. O nível entra em `serializar_jogadores`, de modo que `hash_estado` distingue replays com progressão diferente e permanece idêntico entre replays com o mesmo input.

**Why this priority**: sem o campo, as constantes rotuladas e o incremento no tick, o desbloqueio de US3 não tem fonte determinística e a curta curva de progressão de GDD §10.1 não é verificável; é a base de todas as outras histórias.

**Independent Test**: inspecionar `TICKS_POR_NIVEL`, `NIVEL_MAXIMO`, `Jogador.nivel`, `adicionar_jogador`, `fase_prepare` e `serializar_jogadores`; simular `tick = 0/899/900/1800/10800/11000` e comparar hashes de replays idênticos.

**Acceptance Scenarios**:
1. **Given** um jogador recém-adicionado em `tick = 0`, **When** o estado é inspecionado, **Then** `nivel = 1`; após 899 chamadas de `avancar_tick` continua 1, e na 900ª passa a 2.
2. **Given** `nivel = 2` em `tick = 900`, **When** mais 900 ticks são processados, **Then** `nivel = 3` em `tick = 1800`; a cada múltiplo positivo de 900 há exatamente um incremento.
3. **Given** `nivel = 12` em `tick = 10800`, **When** mil ticks adicionais são processados, **Then** o nível permanece 12 (saturação em `NIVEL_MAXIMO`, sem estouro e sem voltar a 1).
4. **Given** dois replays com o mesmo input, **When** `hash_estado` é comparado, **Then** as strings são idênticas; mudar apenas o nível de um jogador altera o hash de forma previsível, preservando todos os segmentos existentes.

### US3 — Desbloqueio das ações já existentes (P1)

O nível passa a ser pré-requisito das ações que **já existem** no contrato e na sim, segundo a tabela de GDD §10.1:527: `atacar_forte` (**Q**) no nível 1; `construir_barreira` e `construir_ponte` (**W**) no nível 2; `propaganda` e `propaganda_no` (**E**) no nível 3; `atacar_no` (**R**) no nível 6. As quatro ações sem slot de habilidade (`mover`, `sinalizar_sincronia`, `sinalizar_alavanca`, `coletar_materiais`) permanecem disponíveis desde o nível 1: nada na GDD as condiciona e travá-las quebraria a partida. Nenhuma habilidade Q/W/E/R é criada, nenhuma `acao` nova entra em `ACOES_PERMITIDAS` e nenhuma tecla é adicionada ao cliente — W/E/R são apenas os nomes de design dos comandos existentes mapeados no protótipo.

**Why this priority**: é o valor jogável do ciclo — transforma a curva de nível em acesso progressivo sem custo de implementação de habilidades, mantendo o escopo em uma frente de regras.

**Independent Test**: inspecionar a tabela de mapeamento usada por `ValidacaoComandos::validar`, `ACOES_PERMITIDAS` e o tratamento de teclado; validar cada ação nos níveis 1, 2, 3, 5 e 6 com alvo/destino e recursos válidos.

**Acceptance Scenarios**:
1. **Given** `nivel = 1`, com alvo, destino, Fôlego e materiais válidos, **When** `atacar_forte` é validado, **Then** é aceito; `construir_barreira`, `construir_ponte`, `propaganda`, `propaganda_no` e `atacar_no` são recusados com `AcaoBloqueada`.
2. **Given** `nivel = 2`, **When** `construir_barreira` e `construir_ponte` são validados com saldo suficiente, **Then** ambos são aceitos, enquanto `propaganda`, `propaganda_no` e `atacar_no` continuam bloqueados.
3. **Given** `nivel = 3`, **When** `propaganda` e `propaganda_no` são validados com Súdito/Nó no ponto, **Then** ambos são aceitos e `atacar_no` permanece bloqueado; com `nivel = 6`, `atacar_no` passa a ser aceito em Nó ativo.
4. **Given** qualquer nível de 1 a 12, **When** `mover`, `sinalizar_sincronia`, `sinalizar_alavanca` ou `coletar_materiais` é validado, **Then** a ação não sofre gate de nível e segue apenas as regras já existentes.

### US4 — Recusa determinística de ação bloqueada (P1)

`CodigoRecusa` ganha a variante `AcaoBloqueada` (documentada com `///`, mensagem determinística), e `ValidacaoComandos::validar` passa a verificar o nível da ação **depois** de par, replay e `AcaoDesconhecida` e **antes** das verificações específicas de destino, alvo, Fôlego, saldo e estado do Nó. Assim, uma ação fora do nível sempre devolve `AcaoBloqueada`, mesmo que o destino ou o saldo também sejam inválidos; uma ação desconhecida continua devolvendo `AcaoDesconhecida`, e um peer inexistente continua devolvendo `PeerNaoAutorizado`. `validar` permanece puro: nenhuma mutação de `Mundo`, nenhum sinal, nenhum comando agendado e nenhuma exceção.

**Why this priority**: sem um código de recusa explícito e uma ordem de guardas estável, o bloqueio seria indistinguível de destino inválido ou saldo insuficiente, quebrando a verificabilidade determinística e a experiência do jogador.

**Independent Test**: inspecionar `CodigoRecusa`, a ordem das guardas em `ValidacaoComandos::validar` e os testes de recusa; combinar nível bloqueado com destino ausente, Fôlego zero, saldo zero e Nó inativo.

**Acceptance Scenarios**:
1. **Given** `nivel = 1` e `destino = None`, **When** `construir_barreira` é validado, **Then** o resultado é `Err(CodigoRecusa::AcaoBloqueada)`, não `DestinoInvalido`.
2. **Given** `nivel = 1`, `folego = 0` e alvo válido, **When** `atacar_no` é validado, **Then** o resultado é `AcaoBloqueada`; com `nivel = 6` e o mesmo estado, a validação prossegue e pode recusar por destino/Nó inativo, mas nunca por Fôlego (a ação não consome Fôlego).
3. **Given** `peer_id = 99`, **When** qualquer ação é validada, **Then** `PeerNaoAutorizado` prevalece; com `acao = "voar"`, `AcaoDesconhecida` prevalece sobre `AcaoBloqueada`.
4. **Given** qualquer combinação de nível e pré-requisitos, **When** `validar` é chamado várias vezes, **Then** o mundo permanece idêntico e a mesma entrada produz o mesmo `Result`, sem pânico.

### US5 — HUD, hash e fronteira determinística (P1)

`texto_hud` em `crates/cliente/src/main.rs` passa a incluir `Função: <nome>` e `Nível: N/12`, lendo `funcao` e `nivel` do espelho autoritativo `simulacao.0.jogadores.get(&PEER_ID)`; o denominador deriva de `NIVEL_MAXIMO` (12), e nenhuma linha, dica de tecla ou valor existente é removido. `serializar_jogadores` inclui `funcao` e `nivel` logo após os campos atuais do jogador, mantendo a ordem por `peer_id` e todos os segmentos de `hash_estado` (tick, epoch, match_id, jogadores, estruturas, nós, resistência, resultado). Nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, configuração de rede, `AGENTS.md`, `concurrency.jsonl` ou `STATE.md` acontece nesta etapa.

**Why this priority**: sem o HUD o jogador não percebe papel nem progressão; sem o hash as mudanças não são comparáveis em replays; sem a fronteira explícita, uma frente de regras viraria porta de entrada para protocolo e stack novos.

**Independent Test**: inspecionar `texto_hud`, `atualizar_texto`, `serializar_jogadores` e `hash_estado`, comparar hashes antes/depois de mudar papel e nível, conferir a conservação das linhas e teclas `WASD`/`B`/`P`/`C`/`G`/`R`/`F`/`Q` e revisar `git status --short`.

**Acceptance Scenarios**:
1. **Given** `funcao = Funcao::Agitador` e `nivel = 6`, **When** `texto_hud` é montado, **Then** aparecem `Função: Agitador` e `Nível: 6/12`, ao lado de `Posicao`, `Ilha`, `Limites`, `Suprimentos`, `Materiais`, `Fôlego`, `Vida`, `Alvo`, `Autogestão`, `Barreiras`, `Pontes`, `Súditos`, `Ilhas`, `Sinais`, `Resistência`, `Nós`, `Resultado`, `Construindo`, `Coleta` e da dica de teclas existente.
2. **Given** dois replays com o mesmo input, **When** `hash_estado` é calculado, **Then** as strings são idênticas; alterar apenas `funcao` ou `nivel` muda o hash de forma previsível e nenhum segmento existente desaparece.
3. **Given** o estado final do ciclo, **When** os arquivos alterados são listados, **Then** aparecem somente os 4 artefatos SDD e os paths de código atribuídos (sim, testes da sim e `crates/cliente/src/main.rs`), sem contratos, protocolo, Cargo, CI ou rede.

### US6 — Governança, rotulação e estilo (P2)

R1 é `go`; a sequência da constituição v1.2.2 coloca `tasks` antes de `analyze` (READ-ONLY) e R4 (validate) antes de R7 (release `1.0.0-local`); **R6 é N/A** — se invocada, PARE e escale ao SWE, sem issue simulada, sem `gh` improvisado e sem alegar execução. O código segue `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100` e `///` em itens públicos (o atributo de lint `#[expect(clippy::..., reason = "...")]`, já usado no cliente, não é chamada de método e permanece permitido). Nenhum número do protótipo é promovido a cânone: nível 1–12, 900 ticks por nível e os limiares Q1/W2/E3/R6 são `[HIPÓTESE]`/derivação. Execução de gates, build, testes, formatadores, commits, push, PRs e capturas visuais ficam **[NV]** até verificação pelo orquestrador.

**Why this priority**: preserva a cadeia de rastreabilidade do hub e impede que uma frente de progressão vire dívida de automação, mudança de stack ou regra canônica inventada.

**Independent Test**: inspecionar os 4 artefatos, a ordem das portarias e o diff de código, conferindo rótulos `[HIPÓTESE]`, R6 N/A e os atributos de lint.

**Acceptance Scenarios**:
1. **Given** os 4 artefatos, **When** a governança é revisada, **Then** R1 `go`, R6 N/A, `tasks` antes de `analyze` e R4 antes de R7 `1.0.0-local` aparecem explicitamente, com scorecard **48/625**.
2. **Given** o código, **When** o estilo é conferido, **Then** não há `unsafe`, `unwrap`/`.expect()`/`#[allow]`, e as linhas respeitam 100 colunas.
3. **Given** os números do protótipo, **When** a spec é lida, **Then** 1–12, 900 ticks e Q1/W2/E3/R6 aparecem rotulados como `[HIPÓTESE]`/derivação, e PVE/PNJs, rede e novas habilidades permanecem fora de escopo.

## Edge Cases

- `tick = 899`: nível permanece 1. Na transição para `tick = 900`, `fase_prepare` eleva o nível a 2 **antes** dos comandos daquele tick: um `construir_barreira` agendado no tick 900 já é aceito.
- `tick = 10800` (12 × 900): nível atinge 12; ticks posteriores não incrementam nem decrementam. Iniciar com `nivel` diferente de 1 em um jogador adicionado não é fluxo suportado — `adicionar_jogador` sempre zera em 1.
- Jogador adicionado depois de `tick = 0`: começa no nível 1 e só sobe no próximo múltiplo positivo de 900. A progressão usa o `tick` global do mundo (sem contador por jogador); essa é uma simplificação de protótipo, documentada e não-canônica, coerente com o cliente standalone, que adiciona jogadores na inicialização.
- Reversão de Papel/nível: `remover_jogador` segue apenas removendo a entrada do `BTreeMap`; não há histórico, persistência, serialização extra nem restauração de nível.
- Ação bloqueada com pré-requisitos inválidos: `AcaoBloqueada` vem primeiro. Uma ação desconhecida nunca recebe `AcaoBloqueada`, e um peer inexistente nunca passa da primeira guarda.
- `nivel = 6` com Nó inativo: `atacar_no` passa do gate de nível e pode ser recusado por `DestinoInvalido`, conforme a regra atual; o ciclo não altera a semântica do Nó.
- `Q` em `nivel = 1` com Fôlego insuficiente: o gate de nível passa e a validação existente devolve `FolegoInsuficiente`; a recusa de nível não mascara requisitos materiais de ações desbloqueadas.
- Hash e replay: `funcao` e `nivel` entram em `serializar_jogadores` na ordem de `peer_id`; a mesma entrada produz o mesmo hash, e nenhuma mudança de papel/nível é invisível ao hash.
- HUD sem surface automatizável: o alvo é headless; a checagem automatizável é inspeção de `texto_hud` e comparação de hash. Captura visual real da linha `Função:`/`Nível:` fica **[NV]** até verificação humana.
- R6 invocada por engano: PARE, escale ao SWE, sem issue simulada e sem `gh` improvisado; dívida de automação registrada, não executada.

## Requirements

### Functional Requirements

- **FR-001 (papel material e rotulação)**: em `crates/sim/src/lib.rs`, `pub enum Funcao` expõe exatamente `Artesao`, `Guerreiro`, `Intelectual`, `Campones` e `Agitador`, documentadas com `///` como papéis materiais de GDD §10.1/§10.8:744-759, não classes mágicas; `Jogador.funcao: Funcao` é preenchida com `Funcao::Artesao` em `Mundo::adicionar_jogador`, de forma fixa e determinística, pois seleção de papel está fora de escopo. O papel é identificador e rótulo: não altera dano, custo, alcance, recarga, Fôlego ou gate de nenhuma ação existente neste ciclo, e nenhuma variante para `Aliança Mercantil` é acrescentada. **Todos os valores são `[HIPÓTESE]` no GDD** e aparecem rotulados como protótipo nos 4 artefatos, nunca como cânone.
- **FR-002 (progressão determinística)**: `pub const TICKS_POR_NIVEL: i64 = 900` (30 s a 30 Hz, derivação de `PASSO_SEGUNDOS`, rotulada `[HIPÓTESE]`/protótipo) e `pub const NIVEL_MAXIMO: i64 = 12` (`[HIPÓTESE]` de GDD §10.1:527-528) são documentados; `Jogador.nivel: i64` começa em 1 em `adicionar_jogador` e ganha +1 a cada 900 ticks em `fase_prepare`, antes dos comandos do tick, percorrendo `jogadores` em ordem de `peer_id` e saturando em `NIVEL_MAXIMO`. O único relógio é o `tick: i64` existente: sem `Instant`, `SystemTime`, `sleep`, aleatoriedade, I/O, contador adicional por jogador ou campo novo fora de `funcao`/`nivel`.
- **FR-003 (mapeamento de desbloqueio das ações existentes)**: `ValidacaoComandos::validar` aplica o mapa derivado de GDD §10.1:527 — `atacar_forte` (Q) ≥ 1, `construir_barreira`/`construir_ponte` (W) ≥ 2, `propaganda`/`propaganda_no` (E) ≥ 3, `atacar_no` (R) ≥ 6. `mover`, `sinalizar_sincronia`, `sinalizar_alavanca` e `coletar_materiais` nunca são gateados por nível. `ACOES_PERMITIDAS` não ganha entrada nova; nenhuma habilidade, tecla, comando, sinal ou efeito Q/W/E/R é implementado — W/E/R são apenas rótulos de design para comandos já presentes, e novas habilidades ficam para ciclo posterior.
- **FR-004 (recusa `AcaoBloqueada`)**: `pub enum CodigoRecusa` ganha a variante documentada `AcaoBloqueada`; `ValidacaoComandos::validar` verifica o nível logo após par, replay e `AcaoDesconhecida` e antes de destino, alvo/alcance, Fôlego, saldo e estado do Nó. Ação bloqueada devolve `Err(CodigoRecusa::AcaoBloqueada)` mesmo com destino, alvo, Fôlego ou saldo inválidos; ações desbloqueadas seguem integralmente as regras atuais. `validar` permanece puro e determinístico, sem mutar `Mundo` nem agendar comandos.
- **FR-005 (HUD, hash e fronteira determinística)**: `texto_hud` em `crates/cliente/src/main.rs` acrescenta `Função: <nome>` e `Nível: N/12`, lendo o espelho autoritativo `simulacao.0.jogadores.get(&PEER_ID)` e derivando o denominador de `NIVEL_MAXIMO`; todas as linhas e teclas existentes (`WASD`/`B`/`P`/`C`/`G`/`R`/`F`/`Q`) são preservadas e nenhuma tecla nova é introduzida. `serializar_jogadores` inclui `funcao` e `nivel` mantendo a ordem por `peer_id` e todos os segmentos de `hash_estado`, que continua idêntico entre replays com o mesmo input. Nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `concurrency.jsonl` ou `STATE.md`.
- **FR-006 (governança e estilo)**: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100`, `///` em itens públicos (atributo de lint `#[expect(...)]` permitido); R1 `go`, R6 **N/A** por v1.2.2 (PARE se invocada, sem issue simulada e sem `gh` improvisado), `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`; commits em PT-BR com staging explícito; gates/build/testes/formatadores/commit/push/PR **[NV]** para este agente.

### Key Entities

- **`Funcao`**: enum de cinco papéis materiais (`Artesao`, `Guerreiro`, `Intelectual`, `Campones`, `Agitador`), rotulada `[HIPÓTESE]`/protótipo, sem efeito mecânico neste ciclo.
- **`Jogador.funcao` / `Jogador.nivel`**: campos novos do estado determinístico; papel fixo na criação e nível 1 → 12 por `tick`, ambos incluídos no hash.
- **`TICKS_POR_NIVEL` / `NIVEL_MAXIMO`**: constantes derivadas 900 (30 s a 30 Hz) e 12, documentadas como protótipo, nunca cânone.
- **`fase_prepare`**: ponto determinístico de incremento do nível, antes dos comandos do tick; usa apenas `tick` e a ordem de `BTreeMap` por `peer_id`.
- **`CodigoRecusa::AcaoBloqueada`**: recusa explícita e documentada para ação fora do nível, precedendo recusas de destino, alvo, Fôlego, saldo e Nó.
- **`ValidacaoComandos::validar`**: guardião puro que aplica a tabela Q1/W2/E3/R6 às ações já existentes, sem criar ações novas.
- **`serializar_jogadores` / `hash_estado`**: serialização estável que passa a cobrir papel e nível, preservando todos os segmentos e a comparabilidade entre replays.
- **`texto_hud`**: sistema do HUD que acrescenta `Função: <nome>` e `Nível: N/12` sem novas teclas nem captura visual automatizável.

## Success Criteria

- **SC-001 (papel material e rotulação)**: `Funcao` tem exatamente cinco variantes e `Jogador.funcao` é preenchido com `Funcao::Artesao` em `adicionar_jogador`; o HUD exibe `Função: <nome>` com os cinco nomes acentuados corretamente, sem tecla de seleção, efeito por papel ou variante nova. Nível 1–12, papéis e limiares Q1/W2/E3/R6 aparecem rotulados `[HIPÓTESE]`/protótipo nos 4 artefatos, sem promoção a cânone.
- **SC-002 (progressão)**: `Jogador.nivel = 1` na criação, `TICKS_POR_NIVEL = 900`, `NIVEL_MAXIMO = 12`, +1 em `fase_prepare` antes dos comandos do tick e saturação em 12; 899 mantém 1, 900 eleva a 2, 1800 eleva a 3 e 10800 ou mais permanece 12. O `tick` é o único relógio, sem contador por jogador ou dependência nova.
- **SC-003 (desbloqueio)**: com alvo, destino e recursos válidos, `atacar_forte` é aceito no nível 1, construções no 2, `propaganda`/`propaganda_no` no 3 e `atacar_no` no 6; `mover`, sinais e coleta nunca são gateados. `ACOES_PERMITIDAS`, as ações e as teclas existentes permanecem intactas — nenhuma habilidade nova é introduzida.
- **SC-004 (recusa)**: uma ação fora do nível devolve `Err(CodigoRecusa::AcaoBloqueada)` mesmo com destino, Fôlego, saldo ou Nó inválidos, enquanto `PeerNaoAutorizado`, `SeqOuRequestReplay` e `AcaoDesconhecida` precedem o bloqueio; `validar` continua puro, determinístico e sem mutar o mundo.
- **SC-005 (HUD/hash/fronteira)**: `texto_hud` acrescenta `Função: <nome>` e `Nível: N/12` a partir do espelho autoritativo, com denominador derivado de `NIVEL_MAXIMO`, preservando todas as linhas e teclas; `serializar_jogadores` cobre papel e nível, `hash_estado` é idêntico entre replays com o mesmo input e muda de forma previsível quando apenas papel ou nível mudam; contratos, protocolo, Cargo, manifesto, CI e redes permanecem intactos.
- **SC-006 (governança/estilo)**: R1 `go`, R6 N/A, `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`; execução, capturas visuais e publicação **[NV]** até o orquestrador.

## Clarificações

- **Papéis materiais e os números de progressão** vêm de GDD §10.1:515-528 e §10.8:744, mas **todos são `[HIPÓTESE]`**: nível 1–12, teto 12 e desbloqueio Q1/W2/E3/R6. O protótipo os adota como regras de simulação, sem promovê-los a cânone e sem reescrever o GDD.
- **900 ticks** são aritmética com o passo fixo existente (`PASSO_SEGUNDOS = 1 / 30`): 30 s × 30 Hz. É derivação rotulada, não novo número canônico.
- **A progressão usa o `tick` global do mundo**, sem contador por jogador. Jogadores adicionados após o início começam no nível 1 e sobem no próximo múltiplo de 900; isso é simplificação explícita, aceita porque o cliente standalone adiciona jogadores na inicialização e o servidor não faz parte desta frente.
- **W/E/R não são habilidades novas**: o mapa incide sobre `construir_barreira`/`construir_ponte`, `propaganda`/`propaganda_no` e `atacar_no`, comandos que já existem e já são enviados pelo cliente. Nenhum efeito, custo, alcance, recarga, ícone, tecla ou sinal é acrescentado; implementar as habilidades reais de GDD §10.2–10.6 fica para ciclo posterior.
- **Papel não gera vantagem**: atribuir `Funcao::Artesao` por padrão é escolha determinística de protótipo, não balanceamento. Seleção de papel, Aliança Mercantil e sinergias entre funções ficam fora de escopo.
- **Quatro ações permanecem livres de nível** (`mover`, `sinalizar_sincronia`, `sinalizar_alavanca`, `coletar_materiais`), pois nenhuma delas ocupa slot Q/W/E/R na GDD e travá-las impediria a partida.
- **`AcaoBloqueada` tem prioridade sobre recusas específicas**, mas nunca sobre `PeerNaoAutorizado`, `SeqOuRequestReplay` ou `AcaoDesconhecida`: a ordem das guardas é parte do contrato observável.
- **R6 é N/A** por constituição v1.2.2: fora da sequência executável até `resolve-tasks.sh`/adaptador existirem; se invocada, PARE e escale ao SWE, sem simular issue, sem `gh` improvisado e sem alegar execução.
- **R3 = 0** clarificações em aberto: o GDD fornece o conjunto e os limiares, e as lacunas (seleção de papel, habilidades reais) são explicitamente diferidas, não ambíguas.
- Execução de gates/build/testes/formatadores e capturas visuais não têm surface automatizável neste agente: ficam **[NV]** até o orquestrador.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs` e `crates/cliente/src/main.rs` na etapa 12; o orquestrador executa gates, build, testes e verificações finais.
- A sim já tem `PASSO_SEGUNDOS = 1 / 30`, `Mundo.jogadores: BTreeMap<u64, Jogador>`, `adicionar_jogador`, `fase_prepare`, `avancar_tick`, `ACOES_PERMITIDAS`, `ValidacaoComandos::validar`, `CodigoRecusa`, `serializar_jogadores`, `hash_estado` e as ações `atacar_forte`, `construir_barreira`, `construir_ponte`, `propaganda`, `propaganda_no`, `atacar_no`, além de `mover`, `sinalizar_sincronia`, `sinalizar_alavanca` e `coletar_materiais`; o cliente já tem `texto_hud`, `atualizar_texto`, o espelho `Mundo`, `PEER_ID` e as teclas atuais. Nenhum crate é necessário.
- Ambiente headless: nenhuma janela gráfica utilizável, rede, credencial ou comando `gh` é usado na redação destes artefatos; capturas visuais e execução ao vivo ficam **[NV]**.
- O scorecard **48/625** é a escolha do ciclo 052: a frente de derrota (192/625) foi fechada no ciclo 051; PVE/PNJs empatam em 48/625 e ficam para o próximo ciclo, e a rede (10/625) segue bloqueada por ADR-004 até decisão da SE.

## Out of scope

Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md` e documentos derivados; `Lightyear`, `async`, sockets, TCP, servidor, segundo jogador humano; seleção de papel por input/menu, Aliança Mercantil, sinergias entre funções, criação de habilidades Q/W/E/R reais (GDD §10.2–10.6), custos/alcances/recargas/efeitos novos, novas teclas, comandos, sinais ou mensagens de protocolo; PVE/PNJs, mapa, rotas, coordenadas de ilha, itens, equipamento, inventário, economia nova, Plano Mestre, regras de vitória/derrota dos ciclos 049–051; promoção de qualquer número a cânone; execução de gates, build, testes, formatadores, commits, push, PRs, merge ou deploy; alterações preexistentes fora da fronteira. O que não foi executado permanece **[NV]** até verificação pelo orquestrador.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: Papéis + níveis + desbloqueio Q/W/E/R **4×2×2×3 = 48/625**, contra PVE/PNJs **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. Registro em `.specify/memory/STATE.md` (2026-11-23T15:08Z). Nenhum manifesto é criado e nenhum número do protótipo é promovido a cânone.
