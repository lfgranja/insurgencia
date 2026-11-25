# Shape — Sincronia cooperativa observável

- **Slug**: `sincronia-cooperativa`
- **Created**: 2026-11-25
- **Score**: **360/3125** (`5×4×3×2×3`)
- **Recommended option**: Opção A — `sinalizar_sincronia` com janela determinística (BF-015a)

## Options

### Option A — `sinalizar_sincronia` com janela determinística (recomendada)

- **Sketch**: na sim, declarar `pub const TICKS_JANELA_SINCRONIA: u64 = 150` e os campos públicos `sincronias_iniciadas`, `sinalizadores_sincronia: BTreeSet<u64>` e `sincronia_ticks_restantes`, zerados/vazios em `Mundo::new`. `avancar_tick` decrementa a janela uma vez por tick; o primeiro sinal com janela zerada a abre; três `peer_id` distintos na mesma janela incrementam o contador, empurram `sincronia_iniciada:<tick>` em `sinais` e reiniciam janela e conjunto. `hash_estado` passa a cobrir contador, ticks restantes e IDs ordenados. No cliente, `Espaço` agenda `sinalizar_sincronia` com `seq`/`request_id` monotônicos, `configurar` semeia o terceiro par local e a HUD mostra `Sincronias: N`.
- **Appetite**: `small`/`medium` (dias) — estado + tick + hash na sim, um helper de registro, tecla/par/HUD no cliente e testes de regressão; sem contrato, protocolo, Cargo, CI, stack, rede ou GDD novos.
- **Trade-offs**: impacto **5** (Ação Coletiva observável pela primeira vez), confiança **4** (whitelist, validador, nível e ausência de ramo lidos no código; janela de 150 ticks é `[HIPÓTESE]`), fechamento **3** (consumidor e comandos de prova nomeados, spec e testes por escrever), esforço⁻¹ **2** (sim + cliente + CLI + hash) e risco⁻¹ **3** (fronteira preservada, mas novos campos públicos e hash exigem disciplina determinística) → **360/3125**. Ganha uma mecânica cooperativa verificável sem ilha nem rede; sacrifica a ilusão de que a janela já é o desafio completo — ela é apenas o contador que o GDD descreve.
- **Rabbit holes**: implementar topologia de 12 ilhas, barra "0/3 → 3/3", gás lacrimogêneo ou expulsão; promover 150 ticks/3 peers a cânone; duplicar o contador no cliente; deixar `hash_estado` de fora dos campos novos; aceitar sinal duplicado do mesmo peer; mexer em whitelist/validador para "acomodar" a ação.

### Option B — Ato de Vandalismo (BF-002c)

- **Sketch**: implementar o ato de vandalismo como conteúdo de mundo com efeito observável.
- **Appetite**: `medium` (dias/semanas) — exige superfície de objeto, interação e regras de mundo ainda não fechadas.
- **Trade-offs**: impacto **4**, confiança **4**, fechamento **2**, esforço⁻¹ **3**, risco⁻¹ **3** → **288/3125**. Não endereça a lacuna de processamento em `fase_skills` e depende de estado de mundo que este contrato não autoriza; permanece aberta por DoD-4.
- **Rabbit holes**: regras de dano a propriedade, inventário, economia de facção — fora do ciclo 060.

### Option C — `sinalizar_alavanca` (BF-015b)

- **Sketch**: dar efeito à outra ação sem `destino` da mesma família, ligada ao quebra-cabeça de alavancas.
- **Appetite**: `small`/`medium` — semelhante à Opção A, mas exige estado de desafio de alavanca.
- **Trade-offs**: impacto **4**, confiança **3**, fechamento **3**, esforço⁻¹ **2**, risco⁻¹ **2** → **144/3125**. A sincronia é a intenção de Ação Coletiva explicitamente reservada no GDD §8/§12; a alavanca pertence ao desafio de quebra-cabeça e exigiria mais estado. Permanece aberta por DoD-4.
- **Rabbit holes**: ordem de alavancas, reset em erro, recompensas da Ilha da Montanha — fora do contrato.

### Option D — Mapa/rotas de 12 ilhas (BF-003b)

- **Sketch**: fechar topologia e spawning do mapa completo antes de qualquer mecânica cooperativa.
- **Appetite**: `large` (semanas) — depende de decisão canônica no GDD.
- **Trade-offs**: rejeitada sem pontuação paralela: o restante do BF-003b depende de topologia canônica e de penalidade de gás/expulsão (§8.3), explicitamente fora do contrato 060. Não resolve a ação órfã.
- **Rabbit holes**: reabrir o GDD, desenhar Ilhas 2/3, captura e reconquista — fora do contrato 060.

## Recommendation

Escolher **Opção A**. É a única que satisfaz simultaneamente as metas de `define.md`: comportamento observável para o jogador do cliente Bevy e para a automação headless (impacto 5), autoridade já lida no código — whitelist, sem `destino`, nível 1 e nenhum ramo em `fase_skills` (confiança 4), consumidor e comandos de prova nomeados (fechamento 3), com fronteira pequena sem contrato/protocolo/Cargo/CI/stack/rede/GDD (esforço⁻¹ 2 e risco⁻¹ 3). B e C são frentes legítimas, mas pontuam menos e não endereçam o achado concreto; D depende de decisão canônica. 80/20: **360/3125** contra 288/3125 (B), 144/3125 (C) e BF-003b não elegível. Critério do contrato: impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador **5⁵ = 3125**; o quarto e o quinto fatores são inversos de esforço e de risco. Nenhum número do GDD é promovido a cânone e nenhuma topologia de ilha é decidida.

## Out of Scope (for the recommended option)

- Ilhas, rotas, topologia, spawning, barra de sincronia "0/3 → 3/3", gás lacrimogêneo, expulsão e qualquer penalidade do GDD §8/§12.
- Promoção de 150 ticks / 3 peers ou de qualquer valor do GDD a regra canônica; mudança em whitelist, validador ou mapa de níveis.
- Ato de Vandalismo (BF-002c), `sinalizar_alavanca` (BF-015b), mapa de 12 ilhas (BF-003b), HUD com hash (BF-014) e rede real/Lightyear (ADR-004).
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.github/workflows`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`; PR #76 (outro owner ativo, verde mas `dirty`), `dev→main`/deploy (SWE-exclusivos), **R6** (N/A por v1.2.2) e issues simuladas.
- Execução de gates, build, testes e formatadores neste assessment; captura visual prometida onde não houver surface automatizável — marcada **[NV]**.

## Assumptions to Validate

- `avancar_tick` é o único lugar onde a janela é decrementada, uma vez por tick, sem depender de `destino` nem de jogador presente — confirmar na spec 060 que nenhum outro caminho a altera.
- O primeiro sinal com `sincronia_ticks_restantes == 0` abre a janela e zera o conjunto; sinais repetidos do mesmo `peer_id` não contam como dois — confirmar com testes de regressão (FR-003).
- Ao atingir três peers distintos, o contador incrementa, `sincronia_iniciada:<tick>` entra em `sinais` e janela/conjunto reiniciam; o tick usado no signal é o corrente — validar na especificação.
- `hash_estado` inclui `sincronias_iniciadas`, `sincronia_ticks_restantes` e os IDs ordenados de `sinalizadores_sincronia`, preservando replay determinístico entre cliente e CLI.
- O cliente já tem o padrão `ComandoLocal` com ids monotônicos; `Espaço` deve gerar um comando por pressão e o terceiro par local em `configurar` é apenas scaffolding determinístico, não entidade canônica.
- A HUD lê `sincronias_iniciadas` do espelho `Mundo` sem duplicar verdade de estado; a dica em `main.rs:1362` passa a anunciar `Espaço`.
- A automação CLI `insurgencia-servidor` consegue produzir três `peer_id` distintos e observar `sincronia_iniciada` sem janela gráfica — validar nos comandos de prova de `STATE.md:602`; todo efeito renderizado permanece **[NV]**.

## DoD do ciclo (referência única; repetido nos cinco artefatos)

- **DoD-1 (janela e estado)**: primeiro `sinalizar_sincronia` com `sincronia_ticks_restantes == 0` abre a janela de `TICKS_JANELA_SINCRONIA = 150` ticks (5 s × 30 Hz); o terceiro `peer_id` distinto na janela incrementa `sincronias_iniciadas`, empurra `sincronia_iniciada:<tick>` em `sinais` e reinicia janela/conjunto; sinal fora da janela não conta e não corrompe estado; `hash_estado` cobre contador, ticks restantes e IDs ordenados.
- **DoD-2 (consumidor)**: jogador do cliente Bevy (tecla `Espaço`, `seq`/`request_id` monotônicos e terceiro par local semeado em `configurar`) e automação headless via CLI `insurgencia-servidor`.
- **DoD-3 (prova)**: `cargo test --locked -p insurgencia-sim --test simulacao`; `cargo test --locked -p insurgencia-servidor --test cli`; `cargo test --locked -p insurgencia-cliente --lib`; `cargo check --locked -p insurgencia-cliente --all-targets`; `./verify.sh` remoto (`gates`/`rust`).
- **DoD-4 (fronteira/backlog)**: sem topologia de ilhas, dano de gás, promoção de número a cânone ou mudança em GDD/`crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`/CI/stack/rede; **BF-015a passa a `fechado`**; BF-015b e BF-002c permanecem abertos.

## Rastreabilidade

Opção A ↔ **FR-001** (`TICKS_JANELA_SINCRONIA = 150`, campos de `Mundo`, decremento em `avancar_tick`, `sincronia_iniciada:<tick>` e `hash_estado`), **FR-002** (cliente: `Espaço`, ids monotônicos, terceiro par local em `configurar`, HUD `Sincronias: N`), **FR-003** (testes de regressão), **FR-004** (evidência/validação, **[NV]** aqui). Alternativas B/C/D são as mesmas rejeitadas em `research.md` e não recebem rastro de implementação. **DoD-1..DoD-4** e o score **360/3125** (`5×4×3×2×3`) são idênticos nos cinco artefatos; SC/T pertencem ao handoff em `decision.md`, com definição canônica na spec 060. **R6/taskstoissues é N/A** por constituição v1.2.2 e não recebe tarefa; PR #76 não é tocado.
