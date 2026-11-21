# Feature Specification: Cliente Greybox Jogável — Bevy 0.20.1 (`crates/cliente`)

**Feature Branch**: `feature/023-cliente-greybox` · **Created**: 2026-11-21T00:24Z · **Status**: Draft · **Data:** 2026-11-21

**Input**: `.specify/feature.json` aponta `specs/023-cliente-greybox`. A SE redefiniu a prioridade para um MVP jogável **single-player** que prova a tese (`.specify/memory/STATE.md`, estado vivo 2026-11-21T00:22Z), cujo item 1 é o cliente greybox: cena principal, câmera e ilhas em formas geométricas. A ADR-004 (Accepted, 20/11/2026) fixa Bevy 0.20.1 e a toolchain 1.99.1; os crates `insurgencia-contratos` (`crates/contratos`) e `insurgencia-sim` (`crates/sim`) já estão mesclados (specs 021 e 022). Este ciclo acrescenta um **novo crate binário** `crates/cliente` ao workspace existente (`members = ["crates/*"]`), com `bevy = "0.20.1"` em *default features* (render/janela). Gate Rust de referência: `verify.sh` (`verify.sh: ALL STAGES OK`). Escopo exclusivamente documental: nenhum `cargo`, gate, teste, build, formatador, código Rust, stage, commit ou push nesta etapa de escrita.

## User Scenarios & Testing *(mandatory)*

As quatro histórias abaixo são ordenadas por dependência estrutural e valor. Cada uma é testável de forma independente por leitura dos artefatos; nenhuma pressupõe toolchain instalada, dependência baixada, build executado, janela aberta ou alegação de execução observada nesta etapa.

### User Story 1 — App Bevy com cena principal e câmera (Priority: P1)

Como desenvolvedor do Insurgência, preciso de um aplicativo Bevy 0.20.1 em `crates/cliente` cuja cena principal registre os plugins padrão e configure uma câmera 3D, para que exista um ponto de entrada único e um enquadramento a partir do qual o restante da cena greybox seja visível.

**Why this priority**: é o pré-requisito estrutural — sem `App`/`DefaultPlugins` e sem câmera, nenhuma ilha (US2), avatar (US3) ou binário (US4) pode ser demonstrado.

**Independent Test**: ler `crates/cliente/src/main.rs` e confirmar a construção do `App` com `DefaultPlugins`, um sistema `Startup` de cena principal e a presença de uma câmera 3D com projeção perspectiva; nenhum comando é executado nesta etapa.

**Acceptance Scenarios**:

1. **Given** o crate `crates/cliente` materializado, **When** o aplicativo é construído, **Then** `DefaultPlugins` são registrados e um único sistema de `Startup` monta a cena principal (câmera, ilhas, avatar e texto de HUD).
2. **Given** a cena principal montada, **When** o primeiro quadro é renderizado, **Then** exatamente uma câmera 3D perspectiva está ativa, posicionada acima e inclinada sobre as quatro ilhas e o avatar.

---

### User Story 2 — Quatro ilhas geométricas com cores distintas (Priority: P1)

Como jogador/avaliador da tese, preciso ver quatro ilhas — Topo, Meio, Inferior e Central — representadas por geometria primitiva e cores sólidas distintas, para que a topologia do protótipo seja reconhecível sem nenhum arte de produção.

**Why this priority**: é a prova visual da tese do MVP; junto com US1, já entrega uma cena greybox identificável mesmo antes do input (US3).

**Independent Test**: ler o sistema de cena e confirmar quatro entidades de ilha com nomes `Topo`, `Meio`, `Inferior` e `Central`, geometria primitiva (caixa/cilindro), posições distintas no plano XZ e quatro valores de cor diferentes entre si, sem texturas ou modelos.

**Acceptance Scenarios**:

1. **Given** a cena principal, **When** as ilhas são criadas, **Then** existem 4 de 4 entidades (`Topo` verde, `Meio` amarelo, `Inferior` azul, `Central` vermelho) com cores literalmente distintas e 0 ilhas omitidas ou duplicadas.
2. **Given** a câmera de US1, **When** a cena é enquadrada, **Then** as quatro ilhas aparecem simultaneamente em posições separadas, com a Central no centro e as três de rota ao redor.

---

### User Story 3 — Input do jogador move o avatar e interage com as ilhas (Priority: P2)

Como jogador local, preciso mover um avatar com o teclado (WASD/setas) e interagir com uma ilha ao pressionar `E`/`Espaço` quando estiver próximo dela, para que a greybox deixe de ser estática e demonstre, com input real, que o cliente é jogável.

**Why this priority**: converte a cena em protótipo jogável; depende de US1 e US2 para existir, mas é testável isoladamente pela leitura dos sistemas de input, movimento e interação.

**Independent Test**: ler os sistemas de `Update` e confirmar a leitura de `ButtonInput<KeyCode>` para WASD/setas, a normalização do vetor de direção com `Time` delta e a reação à tecla de interação dentro de um raio de proximidade, atualizando o HUD e a cor do avatar; nenhuma rede ou autoridade de servidor é envolvida.

**Acceptance Scenarios**:

1. **Given** o avatar na cena, **When** `W`/`A`/`S`/`D` ou uma seta é pressionada, **Then** o avatar se desloca no plano XZ a velocidade constante, com movimento diagonal normalizado e sem sair da área enquadrada pela câmera.
2. **Given** o avatar dentro do raio de proximidade de uma ilha, **When** `E` ou `Espaço` é pressionado, **Then** o texto do HUD passa a indicar o nome da ilha e o avatar adota a cor daquela ilha — somente no estado local da sessão.
3. **Given** o avatar fora do raio de qualquer ilha, **When** `E`/`Espaço` é pressionado, **Then** nada muda no HUD nem na cor do avatar e nenhum erro ou pânico ocorre.

---

### User Story 4 — Executa como binário jogável (Priority: P2)

Como avaliador da tese, preciso que `crates/cliente` seja um crate binário executável no workspace, para que a greybox possa ser aberta como um jogo real com um único comando, sem servidor, servidor de jogo, lobby ou etapa de publicação.

**Why this priority**: é o critério de "MVP jogavel" da SE; depende de US1–US3 para ter o que executar, mas é auditável de forma isolada pela leitura do `Cargo.toml` e do ponto de entrada `src/main.rs`.

**Independent Test**: ler `crates/cliente/Cargo.toml` e confirmar o pacote `insurgencia-cliente` com alvo binário padrão (`src/main.rs`), `bevy = "0.20.1"` fixado em *default features*, e ausência de `lightyear`, UDP ou qualquer dependência de rede; a execução efetiva pertence à implementação/R4 e permanece **[NV]** nesta etapa.

**Acceptance Scenarios**:

1. **Given** o workspace com `members = ["crates/*"]`, **When** o crate `crates/cliente` é materializado, **Then** ele é membro do workspace e expõe o binário `insurgencia-cliente` a partir de `src/main.rs`.
2. **Given** o binário iniciado, **When** a janela abre, **Then** a cena principal com câmera, quatro ilhas e avatar aparece e responde ao input de US3 — sem conexão de rede e sem servidor (Q1, Q2).

---

### Edge Cases

- Foco da janela: sem foco, `ButtonInput<KeyCode>` não registra teclas e o avatar permanece parado — nenhum pânico e nenhum estado corrompido.
- Movimento diagonal: a direção é normalizada antes de aplicar a velocidade, evitando deslocamento $\sqrt{2}$ maior na diagonal; o avatar é limitado (clamp) à área coberta pela câmera.
- Interação fora do raio: `E`/`Espaço` sem ilha próxima é um no-op determinístico — nenhuma escrita no HUD, nenhuma troca de cor e nenhum erro.
- Ilhas sobrepostas: as posições são distintas o bastante para os raios de proximidade não se intersectarem; em empate, a ilha mais próxima do avatar é escolhida de forma determinística.
- Cor repetida: nenhuma das quatro ilhas pode usar a mesma cor das demais; o avatar inicia em cinza-claro, distinto delas, e o fundo é cinza-neutro (Q3).
- `cargo nextest` indisponível → fallback documentado para `cargo test --locked --workspace`; o fallback não autoriza `--fast` a substituir o gate completo.
- Alegação de `cargo check`/`clippy`/`nextest`/`fmt` ou de janela aberta executada nesta etapa → não observado; permanece **[NV]** (SC-001, SC-002).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O crate `crates/cliente` DEVE ser materializado com `Cargo.toml` próprio, pacote `insurgencia-cliente`, `edition = "2024"` e alvo binário padrão em `src/main.rs`, declarando `bevy = "0.20.1"` com **default features** (inclui render e janela), sem `default-features = false`, sem `*` ou intervalo aberto e sem substituição automática de pacote similar. Justificativa registrada: o cliente precisa de janela e renderização (Q4). Nenhuma dependência de rede (`lightyear`, UDP, HTTP) é adicionada neste ciclo (Q2).
- **FR-002**: O aplicativo DEVE construir um `App` Bevy com `DefaultPlugins` e registrar um sistema de `Startup` que monta a **cena principal** — câmera, quatro ilhas, avatar e texto de HUD — antes do primeiro quadro; a cena DEVE ser inteiramente programática, sem `.tscn`, `project.godot` ou cena Godot (US1).
- **FR-003**: A cena principal DEVE conter exatamente uma câmera 3D com projeção perspectiva (componente `Camera3d`), posicionada acima e inclinada sobre a origem, enquadrando simultaneamente as quatro ilhas e o avatar; o fundo DEVE ser cor neutra (cinza) para manter o caráter greybox (US1, SC-004).
- **FR-004**: As quatro ilhas DEVEM ser entidades com geometria primitiva (caixa ou cilindro), posições distintas no plano XZ e cores sólidas e distintas: `Topo` = verde (0,10 / 0,80 / 0,20), `Meio` = amarelo (0,95 / 0,80 / 0,10), `Inferior` = azul (0,10 / 0,45 / 0,95) e `Central` = vermelho (0,90 / 0,12 / 0,12). A Central fica na origem e as três de rota ao redor. Nenhuma textura, sprite, modelo 3D externo ou arte de produção é usado (US2, SC-003, Q3).
- **FR-005**: O avatar DEVE ser uma entidade geométrica primitiva distinta das ilhas (cor cinza-claro, 0,90 / 0,90 / 0,90), iniciando em posição visível dentro do enquadramento da câmera e separada do centro da Ilha Central, e DEVE ser controlável pelo sistema de input (US3).
- **FR-006**: Um sistema de `Update` DEVE ler `ButtonInput<KeyCode>` para WASD e setas e mover o avatar no plano XZ a velocidade constante alvo de 4,0 unidades/s, usando `Time` (delta) e normalizando a direção diagonal; o deslocamento DEVE ser limitado à área visível, sem relógio de parede e sem aleatoriedade (US3).
- **FR-007**: Um sistema de interação DEVE detectar proximidade do avatar a uma ilha (raio alvo de 1,5 unidade) e, ao receber `E` ou `Espaço`, atualizar **localmente** o texto do HUD com o nome/cor da ilha e a cor do avatar para a cor da ilha; fora do raio a operação é no-op. Nenhum evento de rede, estado de servidor, persistência ou autoridade externa é envolvido — a interação existe somente na sessão local (US3, Q1, Q2).
- **FR-008**: Todo código Rust do crate DEVE declarar `unsafe_code = "forbid"`, `clippy::pedantic = "deny"` e `unwrap_used`/`expect_used` em `deny`, tratando falhas com `Result<T, E>`, `?` e `thiserror` onde aplicável, sem alterar `Cargo.toml`/`clippy.toml` para desativar lint (única exceção: `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado). `verify.sh` DEVE permanecer a única fonte de verdade do gate local, com quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — terminando com `verify.sh: ALL STAGES OK`. Os cinco gates determinísticos alvo, `Makefile`, baselines e `evidence.csv` DEVEM permanecer inalterados; nenhum comando é executado nesta etapa; R4 (`validate`) precede R7 (`release`) (SC-001, SC-005).

### Key Entities *(include if feature involves data)*

- **Crate `insurgencia-cliente`**: unidade em `crates/cliente`, membro do workspace `crates/*`, dependente de `bevy = "0.20.1"` em default features; sem rede e sem dependência nova além do engine.
- **App / Cena principal**: `App` Bevy com `DefaultPlugins` e um sistema de `Startup` que monta câmera, ilhas, avatar e HUD; ponto de entrada `src/main.rs`.
- **Câmera**: componente `Camera3d` com projeção perspectiva, único ativo, responsável pelo enquadramento das quatro ilhas e do avatar.
- **Ilha**: entidade geométrica primitiva com nome (`Topo`, `Meio`, `Inferior`, `Central`), posição no plano XZ e cor sólida distinta; alvo da interação local.
- **Avatar**: entidade geométrica controlável, deslocada por WASD/setas e recolorida na interação; nunca carrega autoridade de jogo.
- **Interação local**: par (raio de proximidade, tecla `E`/`Espaço`) que atualiza HUD e cor do avatar somente na sessão do cliente; sem servidor, sem snapshot e sem persistência.

## Success Criteria *(mandatory)*

Critérios a aferir na implementação e em R4. Nenhum comando de `cargo`, gate, teste, build ou formatador foi executado nesta etapa de especificação; as metas abaixo são alvos, não resultados observados aqui.

### Measurable Outcomes

- **SC-001**: Na implementação/R4, `verify.sh` termina com `verify.sh: ALL STAGES OK` e 4 de 4 estágios OK, na ordem `cargo check --locked --workspace --all-targets` → `cargo clippy --workspace --all-targets --all-features -- -D warnings` → `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) → `cargo fmt --all -- --check`, com 0 estágios vermelhos e 0 lints desativados. (baseline: `crates/cliente` inexistente; meta: 4 de 4; nesta etapa: **[NV]**)
- **SC-002**: O binário `insurgencia-cliente` inicia e responde a input: a janela abre com a cena principal, WASD/setas movem o avatar e `E`/`Espaço` próximo a uma ilha altera HUD e cor do avatar — observável em 100% dos testes manuais de input, com 0 pânicos e 0 conexões de rede. (baseline: nenhum binário jogável; meta: binário inicia e responde; nesta etapa: **[NV]**)
- **SC-003**: 4 de 4 ilhas (`Topo`, `Meio`, `Inferior`, `Central`) ficam visíveis simultaneamente no primeiro quadro, com 4 de 4 cores distintas e 0 ilhas ausentes ou repetidas. (baseline: 0 ilhas no cliente; meta: 4 de 4 visíveis e 4 de 4 cores distintas)
- **SC-004**: Exatamente 1 câmera 3D perspectiva ativa enquadra as quatro ilhas e o avatar no arranque, com 0 câmeras duplicadas e 0 elementos fora do enquadramento. (baseline: 0 câmeras no cliente; meta: 1 de 1)
- **SC-005**: 5 de 5 gates determinísticos alvo permanecem PASS e inalterados — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e `Makefile`, baselines e `evidence.csv` não são modificados. (baseline: 5 de 5 registrados em `.specify/memory/STATE.md`; meta: 5 de 5; nesta etapa: **[NV]**)

## Assumptions

- A ADR-004 (Accepted, 20/11/2026) é a autoridade para linguagem/engine: Bevy 0.20.1 e toolchain 1.99.1; nenhuma troca adicional de engine é permitida sem novo ADR. Os crates `insurgencia-contratos` e `insurgencia-sim`, já mesclados, permanecem intactos — este ciclo apenas acrescenta o crate binário de cliente ao mesmo workspace.
- Greybox significa geometria primitiva e cores sólidas, sem arte, texturas ou modelos externos; o único elemento de interface é um texto mínimo de HUD usado como feedback de interação (Q3).
- Single-player local: um jogador, teclado, sem servidor de jogo, sem lobby, sem multiplayer e sem rede neste ciclo (Q1, Q2).
- `bevy` é declarado em *default features* porque o cliente precisa de janela e renderização; a versão é fixada literalmente em `0.20.1`, sem `*` ou intervalo aberto (Q4).
- Convenção de escrita: português brasileiro, termos estrangeiros explicados na 1ª ocorrência, sem TBD/TODO/placeholder; saídas não observadas permanecem **[NV]**; cada agente escreve somente seus próprios arquivos.
- Nenhum `cargo`, gate, teste, build, formatador, `make`, Godot, stage, commit ou push é executado nesta etapa; registro com data/hora pertence a R4, e versão, changelog com FR/T e rollback a R7; R4 precede R7.

## Clarifications

### Session 2026-11-21

- **Q1: O cliente desta feature é single-player local ou multijogador?** → A: **single-player local**. Um único jogador controla o avatar por teclado na própria máquina; não há servidor de jogo, lobby, matchmaking, 5v5 nem múltiplos peers neste ciclo. O objetivo é provar a tese do MVP jogável com a menor dependência possível. (fontes: `.specify/memory/STATE.md` — prioridade da SE, item 1; `.specify/memory/constitution.md` — escopo solo; GDD §15/§16)
- **Q2: Há rede neste ciclo?** → A: **Não**. Nenhum `lightyear`, UDP, snapshot, replicação ou envio de comando é introduzido; a interação com as ilhas é somente estado local da sessão. Os crates `insurgencia-contratos` e `insurgencia-sim` permanecem mesclados e inalterados, mas não são dependências obrigatórias deste binário greybox. (fontes: ADR-004; STATE.md — DEFER de servidores e rede; specs 017/021/022)
- **Q3: Quanto de arte a greybox inclui?** → A: **Nenhuma arte de produção**. Geometria primitiva (caixas/cilindros), quatro cores sólidas distintas para as ilhas, avatar cinza-claro, fundo cinza-neutro e um texto de HUD apenas para feedback. Sem sprites, texturas, modelos externos, animações ou UI decorativa. (fontes: STATE.md — "ilhas em formas geométricas"; GDD §16; convenção 80/20)
- **Q4: Qual engine e versão fixar, e com quais features?** → A: **Bevy 0.20.1**, exatamente, com **default features** — o cliente precisa de janela e renderização, então `default-features = false` é rejeitado. Toolchain `stable 1.99.1` registrada na ADR-004 atende o MSRV publicado do crate. Toda dependência é fixada, sem `*` ou substituição automática de pacote similar. (fontes: ADR-004; `specs/021-migracao-bevy/spec.md:Q1`; `rust-toolchain`; skill `mdmv-rust-thin`)

0 perguntas ao usuário; as quatro ambiguidades foram resolvidas por evidência local (STATE.md, ADR-004, specs 021/022 e GDD). Cada esclarecimento foi formulado e resolvido isoladamente; nenhum marcador `[NEEDS CLARIFICATION]` permaneceu em aberto.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.
>
> Esta feature não produz manifest de orçamento de contexto nem saídas com handles de contexto. A verificação é a dos quatro estágios de `verify.sh` e dos cinco gates determinísticos alvo, conforme SC-001/SC-005, aferida na implementação e em R4 — nunca nesta etapa de especificação. Nenhum artefato R4/R7 é escrito aqui.
