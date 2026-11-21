# Feature Specification: Cliente Low-Poly Jogável — 3D no `crates/cliente` (Bevy 0.20.1)

**Feature Branch**: `feature/026-cliente-lowpoly` · **Created**: 2026-11-21T02:04Z · **Status**: Draft · **Data:** 2026-11-21

**Input**: `.specify/feature.json` aponta `specs/026-cliente-lowpoly`. A SE redefiniu o **item 1 do entregável**: de "cliente greybox" para **cliente low-poly jogável (main scene, câmera, ilhas)** (`.specify/memory/STATE.md`, estado vivo 2026-11-21). Os itens 2–4 já estão entregues; os itens 5–6 permanecem **DEFER**. O cliente atual é 2D (`Camera2d` + `Sprite`), com lógica pura em `src/lib.rs` e espelho autoritativo em `insurgencia-sim`. Este ciclo **migra o crate existente de 2D para 3D**, sem criar novo crate e sem download de assets. ADR-004 fixa Bevy 0.20.1 e toolchain 1.99.1. Gate Rust de referência: `verify.sh` (`verify.sh: ALL STAGES OK`). Escopo documental: nenhuma janela aberta, build, teste, `cargo` ou formatador é executado nesta etapa; o não observado permanece **[NV]**.

## User Scenarios & Testing *(mandatory)*

As quatro histórias abaixo são ordenadas por dependência estrutural e valor. Cada uma é testável de forma independente por leitura dos artefatos; nenhuma pressupõe toolchain instalada, dependência baixada, execução de `cargo`, build, janela aberta ou alegação de execução observada nesta etapa.

### User Story 1 — Cena principal 3D com `Camera3d` e iluminação (Priority: P1)

Como desenvolvedor do Insurgência, preciso converter a cena principal do cliente de 2D para 3D — substituindo `Camera2d` por uma câmera perspectiva `Camera3d` e acrescentando iluminação de ambiente e direcional — para que as ilhas e o avatar low-poly tenham volume legível e um enquadramento único a partir do qual toda a cena seja visível.

**Why this priority**: é o pré-requisito estrutural da migração. Sem `Camera3d` e sem luz, nenhuma malha 3D (US2), avatar (US3) ou HUD (US4) pode ser demonstrado; é o ponto de entrada da tese de "cliente low-poly jogável".

**Independent Test**: ler `crates/cliente/src/main.rs` e confirmar a ausência de `Camera2d`/`Sprite`, a presença de exatamente uma `Camera3d` com projeção perspectiva posicionada acima e inclinada sobre a origem, e a existência de luz de ambiente e de uma fonte direcional; nenhum comando é executado nesta etapa.

**Acceptance Scenarios**:

1. **Given** o crate `crates/cliente` já materializado e 2D, **When** a cena principal é migrada, **Then** `DefaultPlugins` continuam registrados, um único sistema de `Startup` monta a cena e nenhuma câmera 2D ou sprite permanece ativo.
2. **Given** a cena 3D montada, **When** o primeiro quadro é enquadrado, **Then** exatamente uma `Camera3d` perspectiva cobre simultaneamente as quatro ilhas e o avatar, com iluminação de ambiente e direcional que revela silhuetas e cores sem textura.

---

### User Story 2 — Quatro ilhas low-poly reconhecíveis por silhueta e cor (Priority: P1)

Como jogador/avaliador da tese, preciso ver as quatro ilhas — Topo, Meio, Inferior e Central — como malhas 3D de baixa poligonalidade com silhuetas e cores distintas, para que cada ilha seja reconhecível **apenas** por contorno e cor, sem letreiro (GDD §13.2, linha 1045), mantendo o protagonismo da mecânica de cooperação.

**Why this priority**: é a prova visual do MVP e o critério de aceitação da direção visual do GDD; junto com US1, entrega uma cena 3D identificável mesmo antes do input (US3).

**Independent Test**: ler o sistema de cena e confirmar quatro entidades com malhas 3D (`Mesh3d`) e nomes `Topo`, `Meio`, `Inferior` e `Central`, silhuetas geometricamente distintas, posições separadas no plano XZ e quatro cores sólidas literalmente diferentes entre si, sem textura, sprite, modelo externo ou letreiro.

**Acceptance Scenarios**:

1. **Given** a cena 3D de US1, **When** as ilhas são criadas por malhas procedurais, **Then** existem 4 de 4 entidades com silhuetas distintas e 4 de 4 cores distintas, com 0 ilhas omitidas, duplicadas ou dependentes de texto para identificação.
2. **Given** a câmera de US1 e a iluminação, **When** a cena é observada, **Then** as quatro ilhas aparecem simultaneamente em posições separadas — Central ao centro e as três de rota ao redor — reconhecíveis por silhueta+cor conforme GDD:1045.

---

### User Story 3 — Avatar low-poly com WASD e detecção de região (Priority: P2)

Como jogador local, preciso controlar um avatar low-poly com WASD/setas e ter a região sobre a qual ele está detectada automaticamente, para que o cliente deixe de ser estático e demonstre, com input real e lógica pura preservada, que a migração 3D continua jogável.

**Why this priority**: converte a cena em protótipo jogável; depende de US1 e US2 para existir, mas é testável isoladamente pela leitura dos sistemas de input, movimento e detecção, e pela integridade da lógica pura em `lib.rs`.

**Independent Test**: ler os sistemas de `Update` e confirmar a leitura de `ButtonInput<KeyCode>` para WASD/setas, a normalização da direção no plano XZ com `Time` delta e a detecção de região por proximidade usando a função pura `ilha_em` de `lib.rs`; confirmar que `lib.rs` permanece sem dependência da Bevy e com seus testes intactos. Nenhuma rede ou autoridade de servidor é envolvida.

**Acceptance Scenarios**:

1. **Given** o avatar low-poly na cena, **When** `W`/`A`/`S`/`D` ou uma seta é pressionada, **Then** ele se desloca no plano XZ a velocidade constante, com movimento diagonal normalizado, sem sair da área enquadrada pela câmera e preservando o comportamento da função pura `deslocar`.
2. **Given** o avatar dentro do raio de uma ilha, **When** ele entra na região, **Then** a ilha é detectada de forma determinística pela função pura `ilha_em` e o HUD passa a indicar posição e ilha — somente no estado local da sessão.
3. **Given** o avatar fora do raio de qualquer ilha, **When** ele se move, **Then** nenhuma ilha é reportada, nenhum erro ou pânico ocorre e o último estado válido do HUD permanece coerente.

---

### User Story 4 — HUD com posição e ilha atual (Priority: P2)

Como avaliador da tese, preciso que o HUD exiba a posição do avatar e a ilha detectada, para que a detecção de região seja verificável na tela e a migração para 3D seja reconhecível como o mesmo jogo jogável, sem exigir servidor, lobby ou publicação.

**Why this priority**: é o critério de observabilidade da US3 e o fechamento do "cliente jogável"; depende de US1–US3, mas é auditável de forma isolada pela leitura do sistema de HUD e da mark-up de texto.

**Independent Test**: ler o sistema de HUD e confirmar um texto atualizado com as coordenadas do avatar e o nome/cor da ilha detectada, alimentado pela detecção pura de `lib.rs`; confirmar que o único elemento de interface é esse painel de texto, sem UI decorativa.

**Acceptance Scenarios**:

1. **Given** o avatar em movimento, **When** um quadro é renderizado, **Then** o painel de texto mostra a posição atual do avatar no plano XZ e o nome da ilha detectada (ou indica que nenhuma ilha está sob o avatar).
2. **Given** o avatar sobre uma ilha, **When** a detecção muda de região, **Then** o texto passa a exibir o nome/cor da nova ilha, coerente com US3, sem conexão de rede e sem gravar estado no servidor.

---

### Edge Cases

- Foco da janela: sem foco, `ButtonInput<KeyCode>` não registra teclas e o avatar permanece parado — nenhum pânico e nenhum estado corrompido.
- Movimento diagonal: a direção é normalizada antes de aplicar a velocidade, evitando deslocamento $\sqrt{2}$ maior na diagonal; o avatar é limitado (clamp) à área coberta pela câmera.
- Detecção de região: fora do raio de qualquer ilha, a detecção é um no-op determinístico; em empate, a ilha mais próxima vence, conforme a função pura `ilha_em`.
- Silhueta repetida: nenhuma das quatro ilhas pode ter a mesma silhueta ou a mesma cor das demais — a identificação não depende de letreiro (GDD:1045).
- Orçamento de triângulos: exceder 10 mil triângulos por personagem (GDD:1127) viola a direção low-poly; as malhas DEVEM ser primitivas e contadas por leitura, sem asset externo para esconder o excesso.
- Ausência de textura: cores sólidas substituem texturas; nenhuma captura de tela do MVP depende de textura ou de modelo do pacote Paragon.
- Regra da janela: a SE solicita execução **normal** somente quando pedir explicitamente; inspeção, debug, testes e validação de agente iniciam com `--minimizado` — a flag é preservada na migração.
- `cargo nextest` indisponível → fallback documentado para `cargo test --locked --workspace`; o fallback não autoriza `--fast` a substituir o gate completo.
- Alegação de `cargo check`/`clippy`/`nextest`/`fmt` ou de janela aberta executada nesta etapa → não observado; permanece **[NV]** (SC-001, SC-002).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O crate `crates/cliente` DEVE permanecer como o mesmo pacote binário `insurgencia-cliente`, membro do workspace `crates/*`, com alvo padrão em `src/main.rs` e `bevy = "=0.20.1"` fixado em **default features** (render/janela) — `default-features = false` é rejeitado. A migração 2D→3D NÃO cria novo crate, NÃO troca a engine e NÃO adiciona `lightyear`, UDP, HTTP ou qualquer dependência de rede; a dependência de `insurgencia-sim` (espelho autoritativo, ADR-004) é preservada. A opção `--minimizado` DEVE continuar disponível, aplicando a regra da janela (Q4).
- **FR-002**: O aplicativo DEVE manter o `App` Bevy com `DefaultPlugins` e um único sistema de `Startup` que monta a **cena principal 3D** — câmera, iluminação, quatro ilhas, avatar e HUD — antes do primeiro quadro. `Camera2d` e `Sprite` DEVEM ser removidos; a cena permanece inteiramente programática, sem `.tscn`, `project.godot` ou cena Godot (US1).
- **FR-003**: A cena principal DEVE conter exatamente uma câmera 3D com projeção perspectiva (componente `Camera3d`), posicionada acima e inclinada sobre a origem, enquadrando simultaneamente as quatro ilhas e o avatar, com **iluminação de ambiente e uma fonte direcional** que revele silhueta e cor. O fundo DEVE ser cor neutra e NÃO pode haver segunda câmera ativa (US1, Q2, SC-002).
- **FR-004**: Todas as malhas da cena DEVEM ser **procedurais**, construídas em código com componentes `Mesh3d` — nenhum carregamento de `.gltf`/`.obj`/imagem, nenhum download e nenhum arquivo de asset externo neste ciclo. A fonte primária Kenney Nature Kit 3D (CC0, kenney.nl) fica registrada como referência futura, mas nenhum asset é baixado agora (Q1).
- **FR-005**: As quatro ilhas DEVEM ser entidades `Mesh3d` com silhuetas geometricamente distintas, posições separadas no plano XZ e cores sólidas literalmente distintas: `Topo` verde (0,10 / 0,80 / 0,20), `Meio` amarelo (0,95 / 0,80 / 0,10), `Inferior` azul (0,10 / 0,45 / 0,95) e `Central` vermelho (0,90 / 0,12 / 0,12), com a Central na origem e as três de rota ao redor. Cada ilha DEVE ser reconhecível **apenas** por silhueta e cor, sem letreiro (GDD:1045), sem textura, sprite, modelo externo ou arte de produção (US2, Q2, SC-003).
- **FR-006**: O avatar DEVE ser uma entidade low-poly distinta das ilhas, com cor sólida cinza-claro (0,90 / 0,90 / 0,90), iniciando visível no enquadramento e separada do centro da Ilha Central. **100% dos personagens jogáveis do MVP usam formas geométricas sem textura aplicada** e cada personagem DEVE respeitar o orçamento de **≤ 10 mil triângulos** (GDD:1127); nenhum modelo do pacote Paragon aparece em captura de tela do MVP (Q2, Q3).
- **FR-007**: Um sistema de `Update` DEVE ler `ButtonInput<KeyCode>` para WASD e setas e mover o avatar no plano XZ a velocidade constante alvo, usando `Time` (delta), normalizando a direção diagonal e limitando (clamp) a posição à área visível — sem relógio de parede e sem aleatoriedade. O deslocamento DEVE continuar delegado à função pura `deslocar` em `src/lib.rs`, que permanece sem dependência da Bevy e com seus 5 testes preservados. Um sistema de detecção e HUD DEVE usar a função pura `ilha_em` de `lib.rs` para identificar a ilha sob o avatar (raio alvo de 1,5 unidade; ilha mais próxima em empate) e atualizar um único painel de texto com a **posição do avatar e a ilha atual**; fora do raio a operação é no-op determinístico. Nenhum evento de rede, snapshot, persistência ou autoridade externa é envolvido — o estado existe somente na sessão local (US3, US4, Q1, Q2, SC-004).
- **FR-008**: Todo código Rust do crate DEVE declarar `unsafe_code = "forbid"`, `clippy::pedantic = "deny"` e `unwrap_used`/`expect_used` em `deny`, tratando falhas com `Result<T, E>`, `?` e `thiserror` onde aplicável, sem desativar lint em `Cargo.toml`/`clippy.toml` (única exceção: `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado). `verify.sh` DEVE permanecer a única fonte de verdade do gate local, com quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — terminando com `verify.sh: ALL STAGES OK`. Os 5 gates determinísticos alvo e os 20 testes da lógica pura (5 em `cliente/src/lib.rs`, 8 em `sim`, 7 em `contratos`) DEVEM permanecer intactos, assim como `Makefile`, baselines e `evidence.csv` (SC-001, SC-004, SC-005).

### Key Entities *(include if feature involves data)*

- **Crate `insurgencia-cliente`**: o mesmo pacote binário de `crates/cliente`, migrado de 2D para 3D; depende de `bevy = "=0.20.1"` em default features e de `insurgencia-sim`; sem rede.
- **Cena principal 3D**: `App` com `DefaultPlugins` e um sistema de `Startup` que monta `Camera3d`, iluminação, ilhas, avatar e HUD; ponto de entrada `src/main.rs`.
- **`Camera3d`**: componente de câmera perspectiva único ativo, responsável pelo enquadramento das quatro ilhas e do avatar.
- **Ilha low-poly**: entidade `Mesh3d` com nome (`Topo`, `Meio`, `Inferior`, `Central`), silhueta distinta, posição no plano XZ e cor sólida distinta; alvo da detecção de região.
- **Avatar low-poly**: entidade `Mesh3d` controlável, sem textura e dentro do orçamento de 10 mil triângulos; nunca carrega autoridade de jogo.
- **Lógica pura (`lib.rs`)**: módulo sem dependência da Bevy que concentra `Forma`, constantes de layout, `deslocar`, `estado_inicial` e `ilha_em`; é a fonte da verdade de movimento e detecção, com 5 testes preservados.
- **Detecção de região / HUD**: par (raio de proximidade, função pura `ilha_em`) que atualiza localmente o painel de texto com posição e ilha; sem servidor, sem snapshot e sem persistência.

## Success Criteria *(mandatory)*

Critérios a aferir na implementação e em R4. Nenhum comando de `cargo`, gate, teste, build ou formatador foi executado nesta etapa de especificação; as metas abaixo são alvos, não resultados observados aqui.

### Measurable Outcomes

- **SC-001**: Na implementação/R4, `verify.sh` termina com `verify.sh: ALL STAGES OK` e 4 de 4 estágios OK, na ordem `cargo check --locked --workspace --all-targets` → `cargo clippy --workspace --all-targets --all-features -- -D warnings` → `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) → `cargo fmt --all -- --check`, com 0 estágios vermelhos e 0 lints desativados. (baseline: `verify.sh` já ALL STAGES OK com o cliente 2D; meta: 4 de 4 após a migração; nesta etapa: **[NV]**)
- **SC-002**: A cena principal usa exatamente 1 `Camera3d` perspectiva ativa e malhas `Mesh3d` para ilhas e avatar, com 0 `Camera2d`, 0 sprites e iluminação de ambiente + direcional presentes; o binário inicia e responde a WASD/setas com 0 pânicos e 0 conexões de rede. (baseline: `Camera2d` + `Sprite` no cliente atual; meta: 1 de 1 `Camera3d` e 100% da cena em `Mesh3d`; nesta etapa: **[NV]**)
- **SC-003**: 4 de 4 ilhas (`Topo`, `Meio`, `Inferior`, `Central`) ficam visíveis simultaneamente no primeiro quadro, com 4 de 4 silhuetas distintas e 4 de 4 cores distintas, reconhecíveis sem letreiro conforme GDD:1045, e 0 ilhas ausentes ou repetidas. (baseline: 4 ilhas 2D; meta: 4 de 4 reconhecíveis por silhueta+cor em 3D)
- **SC-004**: Os 20 testes da lógica pura permanecem PASS e inalterados — 5 em `crates/cliente/src/lib.rs`, 8 em `crates/sim` e 7 em `crates/contratos` — com 0 testes removidos, 0 asserções enfraquecidas e `lib.rs` ainda sem dependência da Bevy. (baseline: 20 testes do núcleo determinístico; meta: 20 de 20 preservados; nesta etapa: **[NV]**)
- **SC-005**: 5 de 5 gates determinísticos alvo permanecem PASS e inalterados — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e `Makefile`, baselines e `evidence.csv` não são modificados. (baseline: 5 de 5 registrados em `.specify/memory/STATE.md`; meta: 5 de 5; nesta etapa: **[NV]**)

## Assumptions

- A ADR-004 (Accepted, 20/11/2026) é a autoridade para linguagem/engine: Bevy 0.20.1 e toolchain 1.99.1; nenhuma troca adicional de engine é permitida sem novo ADR. A migração é **in-place** no mesmo `crates/cliente`; os crates já mesclados (`contratos`, `sim`, `metricas`, `exportacao`) permanecem intactos.
- A lógica pura em `src/lib.rs` é a fonte da verdade de geometria, deslocamento e detecção e NÃO recebe dependência da Bevy; a migração troca apenas a camada de renderização em `src/main.rs`.
- Low-poly aqui significa malhas procedurais de baixa contagem, cores sólidas e ausência de textura — não significa importar assets. A referência Kenney Nature Kit 3D (CC0) está registrada como fonte primária, mas nenhum download ocorre neste ciclo (Q1).
- Single-player local: um jogador, teclado, sem servidor de jogo, lobby, matchmaking, multiplayer, snapshot ou rede neste ciclo (Q1, Q2). Os itens 5–6 da SE permanecem **DEFER**.
- Regra operacional da janela (SE, 2026-11-21): execução **normal** somente quando a SE solicitar; inspeção, debug, testes e validação de agente iniciam com `--minimizado` (Q4).
- Convenção de escrita: português brasileiro, termos estrangeiros explicados na 1ª ocorrência, sem TBD/TODO/placeholder; saídas não observadas permanecem **[NV]**; cada agente escreve somente seus próprios arquivos.
- Nenhum `cargo`, gate, teste, build, formatador, `make`, Godot, stage, commit ou push é executado nesta etapa; registro com data/hora pertence a R4, e versão, changelog com FR/T e rollback a R7; R4 precede R7. Nenhum código Rust é escrito nesta etapa.

## Clarifications

### Session 2026-11-21

- **Q1: A migração 3D usa assets externos ou malhas procedurais?** → A: **Malhas procedurais, sem assets externos neste ciclo**. Toda geometria é construída em código com `Mesh3d`; nenhum `.gltf`, `.obj`, imagem ou pacote é baixado ou carregado de disco. A Kenney Nature Kit 3D (CC0, kenney.nl) é a fonte primária confirmada para evolução futura, mas fica apenas registrada como referência — sem download agora. (fontes: STATE.md; GDD §13.2; kenney.nl)
- **Q2: As ilhas e o avatar usam textura ou letreiro para serem reconhecidos?** → A: **Nenhuma textura e nenhum letreiro**. 100% dos personagens do MVP usam formas geométricas sem textura aplicada; cada ilha é reconhecível apenas por silhueta e cor, com cores sólidas e distintas. Nenhum modelo do pacote de Paragon aparece em captura de tela do MVP. (fontes: GDD §13.2, critérios de aceitação da direção visual — linha 1045 e seguintes)
- **Q3: Qual é o orçamento de geometria por personagem?** → A: **≤ 10 mil triângulos por personagem**, meta de orçamento de assets do GDD; o avatar e qualquer personagem jogável low-poly DEVEM respeitar esse teto, medido por leitura das malhas primitivas. (fonte: GDD §14, linha 1127)
- **Q4: Como a janela do cliente deve ser aberta?** → A: **Normal somente quando a SE pedir; `--minimizado` durante inspeção, debug, testes e validação de agente**. A flag já existente no cliente 2D é preservada na migração para 3D; nenhuma execução de janela é afirmada nesta etapa. (fonte: `.specify/memory/STATE.md` — regra operacional da janela, SE, 2026-11-21; `AGENTS.md` §Development Commands e §Testing & QA)

0 perguntas ao usuário; as quatro ambiguidades foram resolvidas por evidência local (STATE.md, GDD, ADR-004 e kenney.nl). Cada esclarecimento foi formulado e resolvido isoladamente; nenhum marcador `[NEEDS CLARIFICATION]` permaneceu em aberto.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.
>
> Esta feature não produz manifest de orçamento de contexto nem saídas com handles de contexto. A verificação é a dos quatro estágios de `verify.sh`, dos 20 testes da lógica pura e dos cinco gates determinísticos alvo, conforme SC-001/SC-004/SC-005, aferida na implementação e em R4 — nunca nesta etapa de especificação. Nenhum artefato R4/R7 é escrito aqui.
