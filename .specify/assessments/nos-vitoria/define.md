# Define — Nós de Controle e vitória por Resistência Ideológica

- **Slug**: `nos-vitoria`
- **Created**: 2026-11-23T12:50Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §6.2/§8.5; `crates/sim/src/lib.rs`; `crates/cliente/src/main.rs`; `.specify/memory/STATE.md`; constituição v1.2.2)

## Problem Statement

A condição de vitória canônica do GDD — converter a Ilha Central por Propaganda, não por demolição (§6.2:271-285; §8.5:416-437) — não existe como estado verificável na simulação determinística. `Mundo` possui jogadores, barreiras, pontes, Súditos, ilhas e sinais (ciclos 046-049), mas **não possui Nós de Controle, Resistência Ideológica nem resultado de partida**; `ACOES_PERMITIDAS` só conhece `"propaganda"` para Súditos; a HUD mostra `Ilhas`/`Sinais`, mas não `Resistência`/`Nós`. Consequentemente, não há como reproduzir tick a tick a sequência "3 Nós convertidos → −90 → pulso −10 → Vitoria com a ilha intacta", nem como impedir que dano vença a partida. O recorte resolve isso com um modelo mínimo em `crates/sim/src/lib.rs` e duas projeções no cliente: `NoControle { id, convertido, propaganda_ticks, inativo_ticks }` em `Mundo.nos: BTreeMap<u64, NoControle>`, `resistencia_ideologica: i64` e `resultado: Option<ResultadoPartida>`, com as ações `propaganda_no`/`atacar_no`. **Todos os números envolvidos são `[HIPÓTESE]`/escolhas de protótipo do GDD**, não cânone: 3 Nós, 100 de Resistência, −30 por Nó, pulso final −10, teto de dano −20 com piso 10, 20 s (600 ticks) de Propaganda por Nó, 20 s (600 ticks) de inatividade após dano.

## Affected Users & Stakeholders

- **Users**: jogadores/observadores da sim — precisam ver `Resistência` e `Nós` convertidos na HUD e acionar `R`/`F` com destino determinístico, para que a vitória deixe de ser apenas texto do GDD; quem roda a demo determinística — precisa que a sequência de conversão, dano e pulso final seja reproduzível e comparável por hash.
- **Stakeholders**: mantenedores da sim — precisam de um modelo de Nó com contadores de Propaganda/inatividade, de validação de comando com `destino` na posição fixa do Nó, de dedup por `(id do Nó, tick)` e de um pulso que não reorderene `avancar_tick`; mantenedores do Núcleo Rust — precisam respeitar `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, ausência de `unwrap`/`expect`/`#[allow]` e `max_width = 100`; designer — precisa que todos os `[HIPÓTESE]` permaneçam rotulados e que dano nunca converta Nó; SWE/orquestrador — precisa de uma fatia verificável por leitura de código e execução determinística, com handoff em `specs/050-nos-vitoria/`.

## Goals

- Declarar `NoControle { id, convertido, propaganda_ticks, inativo_ticks }`, `Mundo.nos: BTreeMap<u64, NoControle>`, `resistencia_ideologica: i64` (início 100 `[HIPÓTESE]`) e `resultado: Option<ResultadoPartida>`, com registro determinístico dos 3 Nós em posições fixas `[HIPÓTESE]`.
- Acrescentar `propaganda_no` e `atacar_no` a `ACOES_PERMITIDAS`, com `destino` = posição fixa do Nó e validação de existência/posição — **sem alterar o esquema do comando** nem criar contrato, protocolo ou `Cargo.toml` novo. A ação `"propaganda"` existente (Súditos) continua distinta.
- Implementar Propaganda canalizada: **600 ticks** (20 s a 30 Hz `[HIPÓTESE]`) por Nó, com auto-avanço por **um** comando e dedup por `(id do Nó, tick)`; na conclusão, `convertido = true` e `resistencia_ideologica` perde **30 uma única vez** `[HIPÓTESE]`.
- Implementar dano que **não converte**: `atacar_no` reduz no máximo **20** pontos `[HIPÓTESE]`, com piso **10** enquanto restar Nó não convertido `[HIPÓTESE]`, marca `inativo_ticks = 600` (20 s `[HIPÓTESE]`) e não altera a contribuição de 30 do Nó atingido.
- Resolver o pulso final: quando os 3 Nós estiverem convertidos, o pulso seguinte aplica **−10** `[HIPÓTESE]`, zera a Resistência e grava `resultado = Some(ResultadoPartida::Vitoria)` com a mensagem "Autogestão estabelecida", mantendo a Ilha Central e a Sede intactas.
- Incluir `nos`, `resistencia_ideologica` e `resultado` em `hash_estado`, em ordem estável, preservando o replay determinístico existente.
- Exibir `Resistência` e `Nós` na HUD; reservar `R`/`F` para `propaganda_no`/`atacar_no` no Nó de mapeamento determinístico simples (ou documentar o mapeamento como protótipo se a interação de teclado for complexa — sem picking/raio).
- Manter a fronteira: nenhuma mudança em contratos/protocolo/Cargo/stack/rede; rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012; R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**.

## Non-Goals

- Derrota por timeout de 20 min ou por 3 reconquistas (`[HIPÓTESE]` §6.2:277, §8.5:416) — fora de escopo; nenhum relógio de partida ou condição de "Comuna dissolvida".
- Pré-requisito de "2 ilhas por rota + Plano Mestre" (§8.5:416) e topologia de rotas/Ilha Central — documental; sem mapa, geometria ou raio de Nó.
- Papéis W/E/R, níveis (48/625), PVE/PNJs e negociação com patrulhas (48/625), tela de resultado, variante duas Comunas e qualquer fork de servidor.
- Netcode, lobby, matchmaking, predição/reconciliação, `async`, sockets ou Lightyear (ADR-004); rede ficou em 10/625 no scorecard.
- Promover números `[HIPÓTESE]` a cânone, reabrir números dos ciclos 046-049 ou confundir a ação `"propaganda"` (Súditos) com `propaganda_no`.
- Executar gates, build, testes, formatadores, commit, push ou PR neste assessment; adotar R6 ou simular emissão de issues; tocar em `AGENTS.md`, `STATE.md`, `concurrency.jsonl` ou qualquer código-fonte.

## Success Metrics

- Com Propaganda contínua em um Nó, `propaganda_ticks` cresce **exatamente 1 por tick** e, aos **600 ticks contínuos** (20 s a 30 Hz `[HIPÓTESE]`), o Nó converte — 3 Nós = 1800 ticks/60 s acumulados, coerentes com §8.5 (baseline: Nós inexistentes — **[NV]** até verificação).
- Cada conversão reduz `resistencia_ideologica` em **exatamente 30, uma única vez**; comandos repetidos em Nó já convertido não voltam a subtrair; dedup por `(id, tick)` impede contagem dupla no mesmo tick (baseline: **[NV]**).
- `atacar_no` **nunca** converte Nó: o dano acumulado respeita o teto de **−20** e o piso de **10** enquanto houver Nó não convertido; o Nó atingido fica inativo por **600 ticks** e continua contabilizando seus 30 pontos (baseline: **[NV]**).
- Com os 3 Nós convertidos, o **pulso seguinte** aplica exatamente **−10**, leva a Resistência a 0 e grava `resultado = Vitoria` ("Autogestão estabelecida") sem destruir estrutura da Sede; nenhum outro evento pode zerar a Resistência (baseline: **[NV]**).
- `hash_estado` reflete Nós (`id`, `convertido`, `propaganda_ticks`, `inativo_ticks`), Resistência e resultado: estados diferentes produzem hashes diferentes e estados idênticos, o mesmo hash; replay determinístico preservado (baseline: **[NV]**).
- A HUD mostra `Resistência` e a contagem de `Nós` convertidos; `R`/`F` geram `propaganda_no`/`atacar_no` com `destino` na posição fixa do Nó e ids monotônicos, com mapeamento simples documentado como protótipo (baseline: sem linhas/teclas — **[NV]**).
- Nenhuma mudança em contratos/protocolo/Cargo/stack/rede; estilo Rust `#![forbid(unsafe_code)]` / `#![deny(clippy::pedantic)]` / sem `unwrap`/`expect`/`#[allow]` / `max_width=100` (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, a vitória continua sendo apenas uma regra escrita: a sim já converte Súditos, controla ilhas e emite sinais, mas nunca registra Nós, Resistência ou resultado — logo a sequência "convertida, não demolida" não pode ser executada, comparada por hash ou exibida ao jogador. Qualquer frente futura de papéis, PVE/PNJs ou rede teria de reconstruir o estado de Nó e a regra de pulso do zero. A decisão 80/20 do ciclo 050 já apontou esta como a frente de maior valor (60/625) justamente porque é a última condição que fecha a partida.

## Open Questions

- Nenhuma clarificação de design pendente no contrato — os números são `[HIPÓTESE]` conhecidos e a decisão 80/20 registrou **R3 = 0 clarificações**; a spec deve rotulá-los, não "corrigi-los".
- **[NV]** A spec deve fixar os ids e as posições fixas dos 3 Nós e o formato de `ResultadoPartida::Vitoria` (enum vs. struct) sem inventar geometria de mapa nem tela de resultado.
- **[NV]** A spec deve definir se o pulso final é uma fase própria em `avancar_tick` ou uma checagem dentro da fase de Nós, preservando a ordem `prepare → skills → defesa → controle_ilhas → finish → reversoes` e o incremento final de `tick`.
- **[NV]** A spec deve decidir se `atacar_no` é debitado por comando ou por tick (dedup), e se o piso 10 é aplicado por comando ou como clamp global da Resistência — sempre com teto −20 e sem conversão.
- **[NV]** A spec deve escolher o mapeamento determinístico de `R`/`F` (Nó mais próximo por distância calculada a partir de posições fixas, ou índice fixo) e documentar a escolha como protótipo; não prometer picking.
- **[NV]** A spec deve confirmar que a HUD rotula `Resistência`/`Nós` como estado de protótipo e que nenhum número `[HIPÓTESE]` aparece como regra canônica.
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** Os nomes definitivos de T001..T012 serão os de `specs/050-nos-vitoria/tasks.md`, ainda a criar pelo `/speckit.specify`.

## Rastreabilidade (escopo do problema)

- **FR-001** → `NoControle { id, convertido, propaganda_ticks, inativo_ticks }` e `Mundo.nos: BTreeMap<u64, NoControle>` com registro determinístico dos 3 Nós em posições fixas `[HIPÓTESE]`; **FR-002** → `resistencia_ideologica: i64` (100 inicial) e `resultado: Option<ResultadoPartida>` `[HIPÓTESE]`; **FR-003** → `propaganda_no`/`atacar_no` em `ACOES_PERMITIDAS`, com `destino` na posição fixa do Nó e validação, sem mudar o esquema de comando; **FR-004** → Propaganda de 600 ticks com auto-avanço por um comando, dedup por `(id do Nó, tick)` e −30 uma vez por conversão; **FR-005** → dano com teto −20/piso 10 e 600 ticks de inatividade, sem converter Nó nem alterar sua contribuição de 30; **FR-006** → pulso final −10 com `resultado = Vitoria`, `hash_estado`, HUD com `Resistência`/`Nós`, teclas `R`/`F`, fronteira e governança.
- **SC-001..SC-006** → os sete `Success Metrics` acima cobrem os seis critérios (os dois primeiros granularizam SC-001/SC-002); **T001..T012** → entregáveis especificados em `specs/050-nos-vitoria/`, fora desta fronteira.
- R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A** por constituição v1.2.2. Scorecard **único** do ciclo: 60/625 (escolhida), 48/625 (papéis), 48/625 (PVE/PNJs), 10/625 (rede); critério impacto × confiança × facilidade ÷ risco, denominador **625**. Nenhum número `[HIPÓTESE]` do GDD é reaberto ou promovido a cânone.
