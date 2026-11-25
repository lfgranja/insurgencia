# Research — Sincronia cooperativa observável

**Data**: 2026-11-25 · Evidências coladas do contrato da sessão, do código verificado no checkout, do GDD e do `STATE.md`; sem reexecução de gates, build ou testes.

## Evidência primária

- [source: `crates/sim/src/lib.rs:217-231`, lido] `ACOES_PERMITIDAS` contém `"sinalizar_sincronia"`; o comando passa no primeiro filtro de `ValidacaoComandos::validar` (`:2431-2433`). Não é ação desconhecida — falta apenas processamento. (confidence: high)
- [source: `crates/sim/src/lib.rs:258,262`, lido] `sinalizar_sincronia` e `sinalizar_alavanca` são as únicas ações que dispensam `destino`; a janela pode ser registrada sem geometria de ilha. (confidence: high)
- [source: `crates/sim/src/lib.rs:242-248`, lido] `nivel_minimo_para_acao` não inclui `sinalizar_sincronia`; pelo mapa Q1/W2/E3/R6, ações sem slot ficam em nível 1 — nenhum gate de habilidade bloqueia o sinal. (confidence: high)
- [source: `crates/sim/src/lib.rs:2414-2475`, lido] O validador recusa replay (`SeqOuRequestReplay`), whitelist e nível antes de qualquer efeito; os ramos seguintes cobrem apenas `mover`, `atacar_forte`, coletas, construções, propaganda e nós. **Não há validação nem efeito de sincronia**: o aceite é estrutural, sem estado associado. (confidence: high)
- [source: `crates/sim/src/lib.rs:875-966`, lido] `fase_skills` processa doze ações por `else if`, mas não `sinalizar_sincronia`. O comando chega, passa no gate de nível (`:902-905`), não casa com nenhum ramo (`:915-959`) e só atualiza `seq`/`request_id` (`:960-965`). **É a lacuna exata do BF-015a.** (confidence: high)
- [source: `crates/cliente/src/main.rs:1362`, lido] `DICA_TECLAS` não menciona `Espaço` nem sincronia; sem a tecla e sem o contador no painel, o jogador não tem como descobrir ou verificar a ação. (confidence: high)
- [source: `docs/GDD - Insurgência.md:349-416`, lido] §8.1: Rota Central, "Ilha 1 preparação; Ilhas 2 e 3: 3 jogadores em janela de 5 s" (`:358`). §8.3: "três jogadores aliados devem entrar simultaneamente em janela de 5 s; entrando escalonados, são repelidos por gás lacrimogêneo e expulsos" (`:386`); critério de aceitação 4,8 s vs. 5,3 s (`:395`). A regra três-em-cinco é intenção canônica de design; **gás, expulsão e geometria de ilha ficam fora do ciclo**. (confidence: high)
- [source: `docs/GDD - Insurgência.md:921,937,943-950`, lido] §12.2 reserva a barra de espaço para sinalizar presença, com o primeiro sinal abrindo a janela de 5 s (`:921`); três sinais em 5,0 s iniciam o desafio, em 5,1 s há repulsão (`:937`); a barra "0/3 → 3/3" descreve a UI ideal (`:943-950`). Valores de janela são `[HIPÓTESE]` para o protótipo. (confidence: high)
- [source: contrato da sessão, imutável] `pub const TICKS_JANELA_SINCRONIA: u64 = 150` (5 s × 30 Hz); campos públicos `sincronias_iniciadas: u64`, `sinalizadores_sincronia: BTreeSet<u64>`, `sincronia_ticks_restantes: u64` zerados/vazios em `Mundo::new`; `avancar_tick` decrementa a janela uma vez por tick; primeiro sinal com janela zerada a abre; três peers distintos na mesma janela incrementam o contador, empurram `sincronia_iniciada:<tick>` em `sinais` e reiniciam janela/conjunto; `hash_estado` cobre contador, ticks e IDs ordenados. Cliente: `Espaço` agenda o comando com ids monotônicos, semeia terceiro par local em `configurar` e mostra `Sincronias: N`. Nada de topologia de ilhas, dano de gás, cânone, contrato, protocolo, CI ou Cargo. (confidence: high)
- [source: `.specify/memory/STATE.md:594-604`, lido] Decisão 80/20 do ciclo 060: **360/3125** (`5×4×3×2×3`) para BF-015a, 288/3125 (BF-002c), 144/3125 (BF-015b); DoD(0.9) com comportamento, consumidor (jogador Bevy + automação CLI), comandos de prova e backlog (BF-015a `fechado`; BF-015b/BF-002c abertos). Âncora `origin/dev = e0e8f38`; PR #76 `OPEN`, `mergeable=false/dirty`, checks verdes, SWE-exclusivo. (confidence: high)
- [source: `.specify/memory/constitution.md` via contrato, lido] v1.2.2: R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY, `converge` append-only; **R6/taskstoissues N/A**, PARE se invocada, sem issue simulada nem `gh` improvisado. (confidence: high)

## Interpretação

A lacuna é de **estado e processamento**, não de autorização ou de regra. Todos os filtros já aceitam `sinalizar_sincronia` (whitelist, sem `destino`, nível 1), e nenhum efeito acontece depois disso: `fase_skills` não tem ramo e o cliente não tem tecla. A contribuição 80/20 é, portanto, um **contador determinístico com janela** no `Mundo` mais um caminho de comando no cliente: 150 ticks por janela, três peers distintos, sinal FIFO e hash cobrindo o novo estado. Isso torna a Ação Coletiva observável no menor recipiente possível — sem ilha, sem gás, sem topologia e sem promover os números do GDD a cânone. As quatro opções do scorecard são: `sinalizar_sincronia` (escolhida, **360/3125**), Ato de Vandalismo (288/3125), `sinalizar_alavanca` (144/3125) e mapa de 12 ilhas (não elegível). Nenhuma renegociação das demais pontuações.

## Alternativas rejeitadas (mesmo scorecard único, 360/3125 para a escolhida)

1. **Ato de Vandalismo (BF-002c)** — rejeitada (288/3125): impacto 4, confiança 4, fechamento 2, esforço⁻¹ 3, risco⁻¹ 3. É uma frente de conteúdo útil, mas depende de superfície de mundo e não endereça a lacuna de processamento já identificada em `fase_skills`. Permanece aberta por DoD-4.
2. **`sinalizar_alavanca` (BF-015b)** — rejeitada (144/3125): impacto 4, confiança 3, fechamento 3, esforço⁻¹ 2, risco⁻¹ 2. Mesma família de comando sem efeito, porém ligada a quebra-cabeça de alavanca — exigiria estado de desafio; a sincronia é a intenção de Ação Coletiva explicitamente reservada no GDD. Permanece aberta por DoD-4.
3. **Mapa/rotas de 12 ilhas (BF-003b)** — rejeitada sem pontuação paralela: depende de decisão canônica de topologia no GDD e de penalidade (gás/expulsão), explicitamente fora deste contrato.
4. **Autorização e execução externa (BF-001/006/007/009/010 e BF-004/005)** — rejeitadas sem pontuação: exigem rede, permissões ou ações fora da fronteira do agente; ADR-004 continua bloqueando a troca de stack.
5. **Diagnóstico (BF-013/014)** — rejeitadas sem pontuação: não produzem comportamento observável para o jogador nem para a automação headless.

## Riscos registrados (não reabertos)

- **Janela e contagem**: mitigadas por decisão — janela single-flight (`sincronia_ticks_restantes == 0` abre), `BTreeSet<u64>` deduplica peers dentro da janela e o reset ao atingir três impede contagem dupla. O comportamento exato no tick 150 fica **[NV]** até verificação.
- **Sinal fora da janela**: deve ser ignorado sem corromper contador, conjunto ou `sinais`; nenhuma tolerância extra é acrescentada. A janela é reiniciada somente no primeiro sinal seguinte ao fechamento.
- **Determinismo/hash**: novos campos públicos entram em `hash_estado` com IDs ordenados; sem isso, replay e comparação entre cliente/CLI divergiriam. Cobertura por testes de regressão (FR-003), **[NV]** até execução.
- **Canonização indevida**: 150 ticks / 3 peers vêm de GDD §8/§12 como `[HIPÓTESE]`; nenhum número é promovido a cânone e nenhuma topologia de ilha é decidida. Gás e expulsão permanecem intenção de design, não implementação deste ciclo.
- **Fronteira**: nada em GDD, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede; o terceiro par local do cliente é scaffolding determinístico, não entidade canônica.
- **Verificação**: `./verify.sh`, jobs `gates`/`rust`, build, testes, captura visual da HUD e R4/R7 ficam **[NV]** — nenhum foi executado por este agente.
- **Dívida R6 / PR #76**: R6 é N/A por v1.2.2; nenhuma issue simulada, nenhum `gh` improvisado. PR #76 (`dev→main`) está verde, mas `dirty` e pertence a outro owner — intocável; promoção e deploy são exclusivos da SE.

## DoD do ciclo (referência única; repetido nos cinco artefatos)

- **DoD-1 (janela e estado)**: primeiro `sinalizar_sincronia` com `sincronia_ticks_restantes == 0` abre a janela de `TICKS_JANELA_SINCRONIA = 150` ticks (5 s × 30 Hz); o terceiro `peer_id` distinto na janela incrementa `sincronias_iniciadas`, empurra `sincronia_iniciada:<tick>` em `sinais` e reinicia janela/conjunto; sinal fora da janela não conta e não corrompe estado; `hash_estado` cobre contador, ticks restantes e IDs ordenados.
- **DoD-2 (consumidor)**: jogador do cliente Bevy (tecla `Espaço`, `seq`/`request_id` monotônicos e terceiro par local semeado em `configurar`) e automação headless via CLI `insurgencia-servidor`.
- **DoD-3 (prova)**: `cargo test --locked -p insurgencia-sim --test simulacao`; `cargo test --locked -p insurgencia-servidor --test cli`; `cargo test --locked -p insurgencia-cliente --lib`; `cargo check --locked -p insurgencia-cliente --all-targets`; `./verify.sh` remoto (`gates`/`rust`).
- **DoD-4 (fronteira/backlog)**: sem topologia de ilhas, dano de gás, promoção de número a cânone ou mudança em GDD/`crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`/CI/stack/rede; **BF-015a passa a `fechado`**; BF-015b e BF-002c permanecem abertos.

## Rastreabilidade

Fatos colados como observados, sem reexecução. Nenhum comando de gate, build, git remoto, GitHub ou emissão de issue foi executado por este agente. **Score: 360/3125** (`5×4×3×2×3`) é o único do ciclo e reaparece em `intake.md`, `define.md`, `shape.md` e `decision.md`. **DoD-1..DoD-4** são idênticos nos cinco artefatos: janela de 150 ticks com terceiro peer distinto e hash; consumidor jogador Bevy (Espaço) + automação CLI; comandos de prova; BF-015a fechado com BF-015b/BF-002c abertos. Rastros da spec 060: **FR-001** = estado/janela/hash na sim; **FR-002** = cliente Espaço + HUD; **FR-003** = testes de regressão; **FR-004** = evidência/validação/release. SC/T são definidos na spec; os comandos de prova em `STATE.md:602` não foram executados aqui. R6/taskstoissues = N/A por constituição v1.2.2.
