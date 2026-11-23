# Shape — Alvo de treino jogável no cliente

- **Slug**: `alvo-treino`
- **Created**: 2026-11-23T08:18Z
- **Recommended option**: Opção A — Alvo de treino no cliente standalone (scaffolding não-canônico)

## Options

### Option A — Alvo de treino no cliente standalone (recomendada)

- **Sketch**: em `crates/cliente/src/main.rs`, declarar `PEER_ALVO_TREINO = 2` e `ALVO_TREINO_POSICAO = (2.0, 0.0)` documentados como scaffolding de protótipo **não-canônico**. Em `configurar`, registrar o peer 2 no espelho `Mundo` (`sim.adicionar_jogador(PEER_ALVO_TREINO)`, posição `(2.0, 0.0)`, vida positiva) e spawnar um marcador visível simples (cubo/cone) com componente `AlvoTreino`. `solicitar_ataque` passa a enviar `destino = Some(ALVO_TREINO_POSICAO)` com `seq`/`request_id` monotônicos; o core já existente encontra o alvo por `ponto_igual`/`peer_id != atacante`, aplica dano 10 e recusa fora de 2 m via `ALCANCE_CONTATO_M`, sem mudança em `crates/sim`. `atualizar_texto` lê o peer 2 e mostra `Alvo: {vida}` e a dica "Aproxime-se do alvo e use Q para atacar".
- **Appetite**: `xs`/`small` (horas a poucos dias) — duas constantes, um `adicionar_jogador` no espelho local, um spawn de malha simples com marcador, a troca de um `destino` e duas linhas de HUD; sem contrato, protocolo, Cargo, stack, rede ou core novos.
- **Trade-offs**: ganha a verificação local do ataque entregue em 047 — o mesmo core determinístico e os mesmos testes, agora com um efeito observável no cliente — ao custo de um objeto de protótipo que não é conteúdo de produto. Impacto **3** (verificabilidade de fatia já implementada, não nova mecânica), confiança **4** (comportamento do core conhecido e coberto, mas efeito visual **[NV]** sem surface automatizável), facilidade **4** (mudança pequena e localizada) e risco invertido **5** (fronteira intocada, cobertura existente preservada). O rótulo não-canônico é o preço aceito: se alguém tratar o alvo como inimigo real, a decisão é violada.
- **Rabbit holes**: transformar o peer 2 em Súdito, inimigo, estrutura de desafio ou entidade de contrato/protocolo; promover valores de §10.1/§10.6 a cânone; implementar mira, projétil, colisão, IA ou spawn remoto; mexer em `aplicar_ataque`/`hash_estado` para "acomodar" o alvo; duplicar débito ou ataque por tick; adicionar tolerância de distância; prometer screenshot da HUD quando a janela roda minimizada.

### Option B — Ilhas/condição de vitória

- **Sketch**: implementar ilhas jogáveis e critério de vitória para dar objetivo de mapa ao ataque.
- **Appetite**: `large` (semanas) — exige geometria de mapa, estado de vitória e regras de desafio ainda em discussão.
- **Trade-offs**: impacto 4, mas confiança 2 e facilidade 2 (valores e topologia ainda `[HIPÓTESE]`, GDD §16.3:1505-1509 prefere desafios de ilha mas não fecha números), risco invertido 3 → **48/625**. É uma frente de design legítima, porém não resolve a limitação concreta da ACR 047 e abre escopo muito maior que o scaffolding.
- **Rabbit holes**: topologia de ilhas, spawning, condição de vitória, Nós, Resistência Ideológica, "combate puro" fora do MVP.

### Option C — Rede / Lightyear primeira

- **Sketch**: postergar a verificação local até transporte, predição e reconciliação cliente-servidor estarem prontos, com um peer remoto real como alvo.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de scaffolding de cliente para infraestrutura de rede.
- **Trade-offs**: impacto 5, mas facilidade 1 e risco invertido 1 → **10/625**; ADR-004 bloqueia a troca até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`). Sincronizaria um ataque cuja verificação local ainda não foi fechada.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), predição de dano, resolução de concorrência de comandos, troca de engine sem novo ADR.

## Recommendation

Escolher **Opção A**. É a única que satisfaz as metas de `define.md` com fronteira mínima: impacto **3** (o ataque 047 passa de inerte a observável no cliente), confiança **4** (o core é conhecido e testado, mas o efeito visual permanece **[NV]**), facilidade **4** (duas constantes, um peer no espelho, um marcador e duas linhas de HUD) e risco invertido **5** (nenhuma mudança em `crates/sim`, contratos, protocolo, Cargo, stack ou rede; cobertura determinística preservada). B (48/625) é frente de design maior e não endereça a ACR; C (10/625) inverte a dependência verificabilidade → sincronização e está bloqueada por ADR-004. Scorecard único do ciclo: **240/625** contra 48/625 (B) e 10/625 (C).

A recomendação **não** propõe promover nenhum número a cânone, criar inimigo ou mexer no core: o alvo é scaffolding de protótipo, rotulado como tal, e apenas devolve ao cliente um efeito que o core já produz. A verificação é estrutural por desenho — com janela minimizada, captura visual real fica **[NV]** e não é prometida.

## Out of Scope (for the recommended option)

- Inimigo, Súdito, estrutura de desafio, ponto neutro, entidade de contrato/protocolo e promoção de qualquer número de combate a cânone (§10.1:519-523 e §10.6:676-680 continuam `[HIPÓTESE]`).
- Mudanças em `crates/sim/src/lib.rs` (ação, `CodigoRecusa`, `aplicar_ataque`, `hash_estado`, alcance), mira, projéteis, colisão, física, IA, spawn remoto ou geometria nova.
- Ilhas, condição de vitória, mapa, Nós e Resistência Ideológica; netcode, lobby, matchmaking, `async`, sockets e Lightyear (ADR-004).
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`; **R6** (N/A por constituição v1.2.2), issues simuladas e `gh` improvisado; gates, build, testes e formatadores neste assessment.
- Captura visual real onde não houver surface automatizável — marcada **[NV]**, não prometida.

## Assumptions to Validate

- `adicionar_jogador(PEER_ALVO_TREINO)` no espelho local aceita uma posição inicial e vida positiva sem depender de spawn remoto; confirmar a API em `crates/sim/src/lib.rs` no `/speckit.specify`.
- `ALVO_TREINO_POSICAO = (2.0, 0.0)` está dentro dos limites do mapa e a uma distância alcançável (`≤ ALCANCE_CONTATO_M = 2.0`) a partir de uma posição inicial razoável do avatar; se não estiver, a dica de aproximação ainda resolve, mas a verificabilidade cai — validar no roteiro.
- `solicitar_ataque` com `destino = ALVO_TREINO_POSICAO` mantém "uma pressão = um comando" e ids monotônicos, e o core recusa sem efeito colateral quando fora de alcance (`CodigoRecusa::AlvoInvalido`), reaproveitando o `BTreeSet<u64>` do ciclo 047.
- O componente `AlvoTreino` é apenas um marcador local; a malha (cubo/cone) é escolha de implementação e não entra em contrato, protocolo ou regra canônica.
- A HUD lê a vida do peer 2 (`Alvo: {vida}`) sem confundir com `Vida: N` do próprio jogador; a dica substitui a anterior "Q para atacar".
- A cobertura determinística da sim (100/100 no ciclo 047) permanece válida sem reescrita; a verificação do cliente é estrutural (inspeção de código + binário com `--minimizado` + HUD por leitura de código), com captura visual **[NV]** salvo surface automatizável.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; B e C são as alternativas de `research.md` e não recebem rastro de implementação neste ciclo. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica prevista em `specs/048-alvo-treino/` (template no momento da leitura). Todo número de combate citado é `[HIPÓTESE]`/`escolha de protótipo` do ciclo 047; nada aqui é cânone. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa.
