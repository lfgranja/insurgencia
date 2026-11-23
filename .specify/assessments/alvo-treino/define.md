# Define — Alvo de treino jogável no cliente

- **Slug**: `alvo-treino`
- **Created**: 2026-11-23T08:18Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (ACR 047; `crates/cliente/src/main.rs`; `crates/sim/src/lib.rs`; GDD §10.1/§10.6/§16.3 via assessment 047; Constituição v1.2.2)

## Problem Statement

A ACR do ciclo 047 entregou `atacar_forte` com efeito determinístico, Fôlego e HUD, mas registrou uma limitação que permanece: o cliente standalone roda com um único jogador local (`PEER_ID = 1`), então a tecla `Q` gera um comando válido que o core aceita, porém `aplicar_ataque` só encontra alvo quando existe outro `peer_id` exatamente no `destino`. Sem segundo jogador/spawn, não há queda de `Vida` visível e a dica "Q para atacar" é enganosa — a verificação do cliente 047 ficou estrutural, não visual. O recorte resolve isso com **scaffolding de protótipo explicitamente não-canônico**: um alvo de treino fixo no espelho `Mundo` do cliente, com marcador visível simples e HUD que mostra sua vida. Não é nova mecânica, novo inimigo nem nova regra: é o recipiente mínimo para que uma fatia já determinística deixe de ser inerte na tela. Nenhum número de combate é reaberto — custo 30, dano 10, alcance 2 m, Fôlego 100 e regenerações 10/s e 5/s continuam `[HIPÓTESE]`/`escolha de protótipo` do ciclo 047.

## Affected Users & Stakeholders

- **Users**: jogadores no cliente standalone — precisam que `Q` tenha efeito observável e uma dica honesta ("Aproxime-se do alvo e use Q para atacar"), em vez de um comando aceito sem consequência visível; quem inspeciona a HUD — precisa de um número de `Alvo: {vida}` que mude quando o ataque acerta.
- **Stakeholders**: mantenedores do cliente — precisam de duas constantes nomeadas, um registro de peer no espelho local, um marcador simples e duas linhas de HUD, sem tocar no core; mantenedores do Núcleo Rust — precisam que `crates/sim`, contratos, protocolo e Cargo permaneçam intocados e que a cobertura determinística existente continue válida; SWE/orquestrador — precisa de uma fatia verificável por inspeção de código e execução do binário com `--minimizado`; designer — precisa que o alvo seja lido como scaffolding não-canônico e que nenhum valor de combate seja promovido a dogma.

## Goals

- Declarar em `crates/cliente/src/main.rs` as constantes `PEER_ALVO_TREINO = 2` e `ALVO_TREINO_POSICAO = (2.0, 0.0)`, documentadas como **scaffolding de protótipo, não-canônico**.
- Registrar o peer 2 no espelho `Mundo` em `configurar`, na posição do alvo e com vida positiva, de modo que `aplicar_ataque` o encontre como `alvo` (`peer_id != comando.peer_id && ponto_igual(posicao, destino)`) sem nenhuma mudança em `crates/sim`.
- Spawnar um marcador visível simples (cubo ou cone) com componente `AlvoTreino` em `ALVO_TREINO_POSICAO`, distinguindo-o do avatar e de entidades canônicas.
- Fazer `solicitar_ataque` usar `destino = ALVO_TREINO_POSICAO`, mantendo ids `seq`/`request_id` monotônicos e uma pressão = um comando; o core recusa se o jogador estiver fora de 2 m (`ALCANCE_CONTATO_M`), sem dano nem débito parcial.
- Exibir `Alvo: {vida}` na HUD a partir do peer 2 e substituir a dica por "Aproxime-se do alvo e use Q para atacar".
- Manter a fronteira: nada em `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack ou rede; cobertura determinística da sim permanece válida.
- Registrar rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012 e constituição v1.2.2 (R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**).

## Non-Goals

- Promover o alvo a inimigo, Súdito, estrutura de desafio, ponto neutro ou entidade de contrato/protocolo; promover qualquer número de combate a cânone.
- Mudar `crates/sim/src/lib.rs`: nova ação, novo `CodigoRecusa`, geometria de mira, colisão, projéteis, IA, spawn remoto, tolerância de distância ou reescrita de `aplicar_ataque` / `hash_estado`.
- Ilhas, condição de vitória, Nós, Resistência Ideológica, mapa e as demais frentes `[HIPÓTESE]` do GDD.
- Netcode, lobby, matchmaking, predição/reconciliação, `async`, sockets ou Lightyear (ADR-004); rede ficou em 10/625 no scorecard.
- Captura visual real prometida onde não houver surface automatizável: com janela minimizada (`--minimizado`), efeito renderizado é **[NV]**; a prova é estrutural.
- Executar gates, build, testes, formatadores, commit, push ou PR neste assessment; adotar R6 ou simular emissão de issues.

## Success Metrics

- Com o jogador a até **2 m** de `ALVO_TREINO_POSICAO`, `Q` gera `atacar_forte` com `destino = (2.0, 0.0)`, o core encontra o peer 2 e a vida do alvo cai em **10** (escolha de protótipo 047), refletida em `Alvo: {vida}` (baseline: comando inerte e nenhuma linha `Alvo:` — **[NV]** até verificação).
- Com o jogador **fora de 2 m**, o core recusa com `AlvoInvalido`, sem dano no alvo e sem débito de Fôlego; a dica orienta a aproximação (baseline: **[NV]**).
- O peer 2 registrado no espelho é encontrado por `aplicar_ataque` via `ponto_igual` e `peer_id != atacante`; no máximo **1** ataque por jogador por tick continua valendo pelo `BTreeSet<u64>` existente (baseline: **[NV]**).
- O marcador `AlvoTreino` existe como entidade visível simples em `(2.0, 0.0)` e é verificável por inspeção de código (baseline: ausente — **[NV]** quanto a renderização).
- A HUD exibe `Alvo: {vida}` lida do peer 2 e a dica "Aproxime-se do alvo e use Q para atacar" (baseline: `Vida: N` do próprio jogador e dica "Q para atacar" — **[NV]** até verificação).
- Nenhuma mudança em `crates/sim`, contratos, protocolo, Cargo, stack ou rede; testes determinísticos da sim não são alterados e permanecem válidos; estilo Rust `forbid(unsafe_code)` / `deny(clippy::pedantic)` / sem `unwrap`/`expect`/`#[allow]` / `max_width=100` (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, a frente de produto fechada no ciclo 047 permanece verificável apenas no core: o comando `Q` é aceito, mas o cliente standalone não tem alvo, a `Vida` exibida é sempre a do próprio jogador e a dica sugere uma ação sem efeito. Qualquer ciclo futuro que queira demonstrar combate, gravar uma capsula ou integrar com rede teria de reconstruir um recipiente local — e a própria ACR 047 já sinalizou a limitação como pendência de verificação. O custo é a perda da única maneira barata e não-canônica de fechar o laço observável do ataque sem tocar em sim, protocolo, stack ou regra canônica.

## Open Questions

- Nenhuma clarificação de design pendente no contrato: o alvo é explicitamente scaffolding não-canônico e os números de combate foram fixados no ciclo 047 (R3 = 0 clarificações).
- **[NV]** Confirmar na especificação se o marcador deve ser cubo ou cone — o contrato permite "cubo/cone"; a escolha é de implementação, não de regra.
- **[NV]** Confirmar que `adicionar_jogador(PEER_ALVO_TREINO)` com vida positiva basta para `hash_estado`/replay não divergirem entre clientes (o peer 2 é determinístico, sem aleatoriedade).
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** Os nomes definitivos de T001..T012 são os de `specs/048-alvo-treino/tasks.md`, ainda em construção pelo `/speckit.specify`.
- **[NV]** Roteiro de 10 passos: inspeção de código, execução do binário com `--minimizado` e capturas de terminal/HUD quando aplicáveis; não prometer captura visual real sem surface automatizável.

## Rastreabilidade (escopo do problema)

- FR-001 → rótulo não-canônico e constantes `PEER_ALVO_TREINO = 2` / `ALVO_TREINO_POSICAO = (2.0, 0.0)`; FR-002 → registro do peer 2 no espelho `Mundo` em `configurar`; FR-003 → marcador visível simples com componente `AlvoTreino`; FR-004 → `solicitar_ataque` com `destino = ALVO_TREINO_POSICAO`, validação de alcance de 2 m e ids monotônicos; FR-005 → HUD `Alvo: {vida}` e dica "Aproxime-se do alvo e use Q para atacar"; FR-006 → fronteira (sim/contratos/protocolo/Cargo/stack/rede intactos), rastros e governança.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis especificados em `specs/048-alvo-treino/`, fora desta fronteira.
- R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A** por constituição v1.2.2. Todo número de combate citado é `[HIPÓTESE]`/`escolha de protótipo` do ciclo 047; nada aqui é cânone.
