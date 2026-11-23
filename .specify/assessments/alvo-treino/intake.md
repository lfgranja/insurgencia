# Intake — Alvo de treino jogável no cliente

**Data**: 2026-11-23T08:18Z · **Slug**: `alvo-treino` · **Sessão**: `session_id=20261123T081800Z-alvo-treino`, `session_title=insurgencia - alvo de treino jogavel`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 048: tornar **observável no cliente standalone** o ataque `atacar_forte` entregue no ciclo 047. A ACR do ciclo 047 registrou a limitação com precisão — o cliente roda com um único jogador local (`PEER_ID = 1`), a tecla `Q` envia um comando válido e o core o valida (`ACOES_PERMITIDAS`, alvo, alcance e Fôlego), mas **não há segundo jogador/spawn**: `aplicar_ataque` só encontra alvo quando existe outro `peer_id` no `destino`, então nenhuma queda de `Vida` aparece no HUD e a dica "Q para atacar" é enganosa. O recorte acrescenta um **alvo de treino determinístico, explicitamente não-canônico**: registrar o peer 2 no espelho `Mundo` do cliente e spawnar um marcador visível simples, com `solicitar_ataque` passando a mirar a posição fixa do alvo. **Nenhuma regra canônica é reaberta** — os números de combate (custo 30, dano 10, alcance 2 m, Fôlego 100, regeneração 10/s e 5/s) permanecem `[HIPÓTESE]`/`escolha de protótipo` do ciclo 047.

**Motivação**: fechar o laço de verificação do ciclo 047 sem inventar geometria de mira, IA, spawn remoto ou rede. O ataque já é determinístico e coberto por testes na sim; falta apenas um recipiente no cliente para que a ação e o HUD deixem de ser meramente estruturais e passem a ter um efeito local inspecionável. É **scaffolding de protótipo**, não conteúdo de produto: o alvo não é Súdito, inimigo, estrutura de desafio nem entidade de contrato — é um espelho local para tornar o ataque verificável.

## Contexto do ciclo (observado no contrato da sessão; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/alvo-treino-048/`, branch `048-alvo-treino`, base `fdc6c08`.
- Alvo de especificação `specs/048-alvo-treino`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012 (o diretório continha apenas o template `spec.md` no momento da leitura).
- Constituição v1.2.2: R1 `go`, R4 antes de R7 `1.0.0-local`; **R6 N/A** — não pertence à sequência executável enquanto não existir o adaptador de issues: dívida de automação, nunca issue simulada, nunca `gh` improvisado.
- Estado verificado no checkout: `crates/cliente/src/main.rs` define `const PEER_ID: u64 = 1`, registra só esse jogador no espelho (`sim.adicionar_jogador(PEER_ID)` em `configurar`), e `solicitar_ataque` usa `destino = Some((x, y))` (a própria posição). `crates/sim/src/lib.rs::aplicar_ataque` procura o alvo entre `jogadores` com `peer_id != comando.peer_id && ponto_igual(jogador.posicao, destino)`, aplica `ALCANCE_CONTATO_M = 2.0` e debita `CUSTO_ATAQUE_FOLEGO = 30`. Não há constante `PEER_ALVO_TREINO`, `ALVO_TREINO_POSICAO`, componente `AlvoTreino`, linha `Alvo:` no HUD nem dica de aproximação.

## Scorecard único do ciclo

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Alvo de treino no cliente standalone (escolhida) | 3×4×4×5 = 240/625 | **240/625** |
| Ilhas/condição de vitória | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = **625** (o quarto fator é o risco invertido/segurança: 5 = risco mínimo, 1 = risco máximo). Este é o **único** scorecard do assessment; `research.md`, `shape.md` e `decision.md` repetem os mesmos valores, sem pontuação paralela.

Escolhe-se o alvo de treino: é o maior valor do scorecard (240/625) e a única opção que resolve o achado concreto da ACR 047 com fronteira mínima — somente `crates/cliente/src/main.rs`, nenhuma regra canônica, nenhum contrato/protocolo/Cargo/rede. Impacto **3** porque o ganho é de verificabilidade local de uma fatia já implementada, não de nova mecânica; confiança **4** porque o comportamento do core é conhecido e coberto por testes, mas o efeito visual no cliente permanece **[NV]** sem surface automatizável; facilidade **4** porque são duas constantes, um registro de peer, um marcador e duas linhas de HUD; risco invertido **5** porque nada muda no `crates/sim`, protocolo ou stack e a cobertura determinística existente permanece válida. Ilhas/vitória (48/625) dependem de mapa e regras ainda `[HIPÓTESE]`; rede (10/625) está bloqueada por ADR-004 e sincronizaria uma fatia cuja verificação local ainda não foi fechada.

## Evidências de entrada (coladas; não reexecutadas)

- `.specify/releases/combate-folego/release.md`, seção "Limitação da verificação do cliente standalone" (lida): "o cliente standalone roda com um único jogador local (`PEER_ID = 1` em `crates/cliente/src/main.rs`): não há segundo jogador nem spawn remoto na sessão. A tecla `Q` envia `atacar_forte` e o comando é validado pelo core [...], mas o dano depende de um alvo existente no `destino`; sem segundo jogador/spawn não há alvo e, portanto, nenhuma queda de `Vida` visível no HUD. A verificação do cliente é **estrutural** [...], **não visual**: a ausência de dano renderizado não é falha."
- `crates/cliente/src/main.rs` (lido): `const PEER_ID: u64 = 1`; `configurar` faz `criar_avatar(...)`, `sim.adicionar_jogador(PEER_ID)` e insere `JogadorLocal`, `ComandoLocal { seq, request_id }`, `Simulacao(sim)`, `AccumuladorTick`; `solicitar_ataque` usa `teclado.just_pressed(KeyCode::KeyQ)`, incrementa ids e agenda `Comando { acao: "atacar_forte", destino: Some((x, y)) }`; `atualizar_texto` já mostra `Fôlego: N` e `Vida: N` e a dica "Q para atacar"; `ModoJanela`/`--minimizado` já permitem iniciar a janela minimizada.
- `crates/sim/src/lib.rs` (lido): `ALCANCE_CONTATO_M = 2.0`, `CUSTO_ATAQUE_FOLEGO = 30`, `ACOES_PERMITIDAS` com `"atacar_forte"`; `aplicar_ataque` seleciona `alvo` com `peer_id != comando.peer_id && ponto_igual(jogador.posicao, destino)`, recusa se `hypot > ALCANCE_CONTATO_M` ou `folego < custo`, e deduplica por `BTreeSet<u64>` — ou seja, **basta um segundo peer no espelho para o ataque ter efeito**, sem tocar no core.
- Contrato da sessão (imutável): constantes `PEER_ALVO_TREINO = 2` e `ALVO_TREINO_POSICAO = (2.0, 0.0)` em `crates/cliente/src/main.rs`; `configurar` registra o peer 2 no espelho `Mundo` e spawn marcador visível simples (cubo/cone) com componente `AlvoTreino`; `solicitar_ataque` passa a usar `destino = ALVO_TREINO_POSICAO` (o core recusa se o jogador estiver fora de 2 m); HUD mostra `Alvo: {vida}` e dica "Aproxime-se do alvo e use Q para atacar"; nada em `crates/sim`, contratos, protocolo, Cargo, stack ou rede.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/alvo-treino/`. Não executa gates, build, testes ou formatadores; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. Tudo o que depende de execução permanece **[NV]**. R6 não é invocada — N/A por constituição v1.2.2. Roteiro de verificação previsto: 10 passos por inspeção de código, execução do binário com `--minimizado` e capturas de terminal/HUD **quando aplicável**; captura visual real não é prometida se não houver surface automatizável — nesses pontos, **[NV]**.
