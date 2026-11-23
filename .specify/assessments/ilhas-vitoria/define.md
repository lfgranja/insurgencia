# Define — Controle de ilha por Súdito (subset de vitória)

- **Slug**: `ilhas-vitoria`
- **Created**: 2026-11-23T11:12Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §9.1/§9.2/§9.4; `crates/sim/src/lib.rs`; `.specify/memory/STATE.md:363-372`; constituição v1.2.2)

## Problem Statement

A regra canônica de controle de ilha está escrita no GDD desde sempre (§9.4:489,501): **10 s contínuos de presença de um Súdito da Comuna controlam a ilha; sem o Súdito, a ilha volta a Latente e emite `ilha_perdida`**. A simulação determinística, porém, não tem estado de ilha: `Mundo` contém jogadores, barricadas, pontes e Súditos, mas nem `ilhas` nem `sinais`; `avancar_tick` roda `prepare → skills → defesa → finish` sem uma fase de controle; `hash_estado` não cobre o que não existe. Consequentemente, a primeira condição de vitória do jogo não pode ser observada, comparada entre execuções ou exibida ao jogador. O recorte resolve isso com um modelo mínimo em `crates/sim/src/lib.rs`: `Ilha { id, controlada, controle_ticks }` em `Mundo.ilhas: BTreeMap<u64, Ilha>`, `adicionar_ilha(id)`, `Mundo.sinais: VecDeque<String>`, a fase `fase_controle_ilhas` entre `fase_defesa` e `fase_finish`, e projeção de ilhas/sinais em hash e HUD. Como o protótipo não tem mapa nem rede, a **presença de qualquer Súdito convertido é tratada como global** — simplificação a documentar, não regra nova. Não se introduzem Nós de Controle, Resistência Ideológica, papéis W/E/R nem vitória completa.

## Affected Users & Stakeholders

- **Users**: jogadores/observadores da sim — precisam ver uma contagem de ilhas controladas e a lista de sinais na HUD, de modo que a regra §9.4 deixe de ser apenas texto do GDD; quem roda a demo determinística — precisa que o controle seja reproduzível tick a tick, inclusive após perda de presença.
- **Stakeholders**: mantenedores da sim — precisam de um acumulador `controle_ticks` com o mesmo padrão de 300 ticks já usado por `defesa_ticks`, e de uma fila de sinais determinística; mantenedores do Núcleo Rust — precisam que `forbid(unsafe_code)`, `deny(clippy::pedantic)`, ausência de `unwrap`/`expect`/`#[allow]` e `max_width=100` sejam respeitados e que o hash cubra o novo estado; designer — precisa que a simplificação de presença global e o uso do Súdito como proxy da condição de Desafio concluído fiquem registrados e **não** sejam promovidos a cânone; SWE/orquestrador — precisa de uma fatia verificável por leitura de código e execução determinística, com handoff em `specs/049-ilhas-vitoria/`.

## Goals

- Declarar `Ilha { id, controlada, controle_ticks }` e `Mundo.ilhas: BTreeMap<u64, Ilha>`, com `adicionar_ilha(id)` para registrar ilhas de forma determinística.
- Implementar a regra canônica: com pelo menos um Súdito convertido, `controle_ticks` avança **1 por tick** até **300 ticks** (10 s a 30 Hz), quando `controlada` passa a `true`; sem Súdito convertido, `controle_ticks` zera, `controlada` passa a `false` (Latente) e, se a ilha estava controlada, `ilha_perdida` é enfileirado em `Mundo.sinais: VecDeque<String>`.
- Inserir `fase_controle_ilhas()` em `avancar_tick`, entre `fase_defesa()` e `fase_finish()`, preservando a ordem das fases existentes e o incremento final de `tick`.
- Incluir ilhas (id, `controlada`, `controle_ticks`) e sinais em `hash_estado`, em ordem estável, para que estados diferentes não colidam e o replay continue determinístico.
- Exibir na HUD a contagem de ilhas controladas e os sinais presentes na fila, sem inventar geometria de mapa.
- **Documentar a simplificação**: ausência de mapa/rede ⇒ presença global de qualquer Súdito convertido; o protótipo usa o Súdito como proxy da Condição "Desafio de Ilha concluído" de §9.4, sem modelar o desafio em si.
- Manter a fronteira: nenhuma mudança em contratos/protocolo/Cargo/stack/rede; rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012 e constituição v1.2.2 (R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**).

## Non-Goals

- Nós de Controle + vitória completa, Resistência Ideológica, Ilha Central, desbloqueio de rotas e tela de resultado — 48/625, próximo ciclo.
- Papéis W/E/R, níveis e novas habilidades — 48/625; nenhum contrato de ação novo.
- Netcode, lobby, matchmaking, predição/reconciliação, `async`, sockets ou Lightyear (ADR-004); rede ficou em 10/625 no scorecard.
- Geometria de ilha, posição de Súdito por ilha, raio de presença, mapa, topologia de rotas ou servidor central com dedupe de sinais em 2 s (`[HIPÓTESE]` §9.1:439, não reaberta).
- Promover a simplificação a cânone: o GDD continua exigindo duas condições e presença por ilha; o proxy do protótipo é rotulado como tal.
- Executar gates, build, testes, formatadores, commit, push ou PR neste assessment; adotar R6 ou simular emissão de issues; tocar em `AGENTS.md`, `STATE.md`, `concurrency.jsonl` ou qualquer código-fonte.

## Success Metrics

- Com **pelo menos um Súdito convertido** presente, `controle_ticks` de cada ilha registrada cresce **exatamente 1 por tick** e, ao completar **300 ticks contínuos**, `controlada == true` (baseline: ilhas inexistentes — **[NV]** até verificação).
- Com **qualquer tick sem Súdito convertido**, o progresso é interrompido: `controle_ticks == 0` imediatamente, `controlada == false`; um único tick sem presença impede a conclusão dos 10 s contínuos (baseline: **[NV]**).
- Quando uma ilha **estava controlada** e perde o Súdito, `ilha_perdida` entra em `Mundo.sinais` **uma vez por transição**; sem transição, nenhum sinal espúrio é enfileirado (baseline: **[NV]**).
- A ordem das fases é preservada: `fase_controle_ilhas` executa após `fase_defesa` e antes de `fase_finish` no mesmo tick, e o controle observa o estado `convertido` já atualizado pelas fases de conversão/reversão (baseline: **[NV]**).
- `hash_estado` reflete `ilhas` e `sinais`: estados com contagens de controle ou filas diferentes produzem hashes diferentes; estados idênticos produzem o mesmo hash, mantendo replay determinístico (baseline: **[NV]**).
- A HUD mostra a **contagem de ilhas controladas** e os **sinais** da fila, com rótulo que identifica a leitura como estado da sim prototípica (baseline: sem linhas de ilha/sinal — **[NV]**).
- Nenhuma mudança em contratos/protocolo/Cargo/stack/rede; cobertura determinística existente preservada; estilo Rust `#![forbid(unsafe_code)]` / `#![deny(clippy::pedantic)]` / sem `unwrap`/`expect`/`#[allow]` / `max_width=100` (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, a regra de vitória mais importante do GDD permanece apenas documental: a sim converte Súditos, aplica Fôlego, coleta e constrói barricadas, mas nunca registra quem controla uma ilha nem emite o sinal canônico de perda. Qualquer ciclo futuro de Nós de Controle ou vitória completa teria de reconstruir o estado de ilha e o histórico de sinais do zero — e a própria decisão 80/20 de 049 já apontou esse subset como a frente de maior valor (108/625). O custo é a impossibilidade de verificar, comparar por hash ou exibir ao jogador a condição de vitória sem abrir uma frente de mapa/rede muito maior.

## Open Questions

- Nenhuma clarificação de design pendente no contrato — a regra é canônica e a simplificação foi decidida na decisão 80/20 (R3 = 0 clarificações no registro de `STATE.md`).
- **[NV]** A spec deve confirmar se `ilha_convertida` (§9.1:447) também será enfileirado na transição para `controlada`, ou se o ciclo se limita a `ilha_perdida` conforme o contrato. Em caso de inclusão, registrar a emissão como escolha de protótipo, não como regra canônica nova.
- **[NV]** A spec deve definir como as ilhas são registradas (`adicionar_ilha`) e quantas existem no protótipo — o contrato exige a API, não uma topologia.
- **[NV]** A spec deve fixar o formato dos `String` em `Mundo.sinais` (o nome canônico `ilha_perdida` é obrigatório) e se a fila é drenada em algum ponto do tick.
- **[NV]** A forma de exibir sinais na HUD (lista completa ou contagem) é escolha de implementação; o contrato exige apenas que os sinais apareçam.
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** Os nomes definitivos de T001..T012 serão os de `specs/049-ilhas-vitoria/tasks.md`, ainda a criar pelo `/speckit.specify`.

## Rastreabilidade (escopo do problema)

- FR-001 → modelo `Ilha { id, controlada, controle_ticks }`, `Mundo.ilhas: BTreeMap<u64, Ilha>` e `adicionar_ilha(id)`; FR-002 → regra canônica §9.4: 300 ticks contínuos com Súdito → `controlada`, ausência → Latente e `controle_ticks = 0`; FR-003 → `Mundo.sinais: VecDeque<String>` e `ilha_perdida` na transição de perda; FR-004 → `fase_controle_ilhas` entre `fase_defesa` e `fase_finish`, observando o estado `convertido` do mesmo tick; FR-005 → presença global do Súdito convertido como **simplificação documentada**, sem mapa/geometria e sem modelar o Desafio de Ilha; FR-006 → `hash_estado` com ilhas e sinais, HUD com contagem/sinais, fronteira intacta e rastros/governança.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis especificados em `specs/049-ilhas-vitoria/`, fora desta fronteira.
- R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A** por constituição v1.2.2. Nenhum número canônico do GDD é reaberto; a dedupe de sinais em 2 s e a reconversão em 20 s continuam `[HIPÓTESE]` e fora de escopo.
