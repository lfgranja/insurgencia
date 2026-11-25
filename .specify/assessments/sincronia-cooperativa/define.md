# Define — Sincronia cooperativa observável

- **Slug**: `sincronia-cooperativa`
- **Created**: 2026-11-25
- **Score**: **360/3125** (`5×4×3×2×3`: impacto 5 × confiança 4 × fechamento 3 × esforço⁻¹ 2 × risco⁻¹ 3)
- **Pendência**: BF-015a · **Ciclo**: 060
- **Inputs used**: `intake.md` | `research.md` | `crates/sim/src/lib.rs:217-231,242-248,258,262,875-966,2414-2475` | `crates/cliente/src/main.rs:1362` | `docs/GDD - Insurgência.md:349-416,921,937,943-950` | `STATE.md:594-604` | Constituição v1.2.2

## Problem Statement

`sinalizar_sincronia` é uma ação órfã. Ela está na whitelist da sim (`lib.rs:217-231`), dispensa `destino` (`:258,262`) e passa no gate de nível 1 (`:242-248`), de modo que `ValidacaoComandos::validar` (`:2414-2475`) a aceita; porém `fase_skills` (`:875-966`) não tem nenhum ramo para ela e o comando apenas avança `seq`/`request_id` (`:960-965`). No cliente, `DICA_TECLAS` (`main.rs:1362`) não menciona `Espaço` e não há contador de sincronia no painel. Resultado: a regra de Ação Coletiva do GDD — três aliados em cinco segundos (§8.1:358, §8.3:386, §12.2:921,937) — não tem recipiente observável no protótipo. O recorte acrescenta estado determinístico, janela de 150 ticks, sinal FIFO e um caminho de comando no cliente, sem tocar em topologia de ilhas, penalidade de gás, regra canônica, contrato, protocolo, CI, Cargo, stack ou rede.

## DoD do ciclo (referência única; repetido nos cinco artefatos)

- **DoD-1 (janela e estado)**: primeiro `sinalizar_sincronia` com `sincronia_ticks_restantes == 0` abre a janela de `TICKS_JANELA_SINCRONIA = 150` ticks (5 s × 30 Hz); o terceiro `peer_id` distinto na janela incrementa `sincronias_iniciadas`, empurra `sincronia_iniciada:<tick>` em `sinais` e reinicia janela/conjunto; sinal fora da janela não conta e não corrompe estado; `hash_estado` cobre contador, ticks restantes e IDs ordenados.
- **DoD-2 (consumidor)**: jogador do cliente Bevy (tecla `Espaço`, `seq`/`request_id` monotônicos e terceiro par local semeado em `configurar`) e automação headless via CLI `insurgencia-servidor`.
- **DoD-3 (prova)**: `cargo test --locked -p insurgencia-sim --test simulacao`; `cargo test --locked -p insurgencia-servidor --test cli`; `cargo test --locked -p insurgencia-cliente --lib`; `cargo check --locked -p insurgencia-cliente --all-targets`; `./verify.sh` remoto (`gates`/`rust`).
- **DoD-4 (fronteira/backlog)**: sem topologia de ilhas, dano de gás, promoção de número a cânone ou mudança em GDD/`crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`/CI/stack/rede; **BF-015a passa a `fechado`**; BF-015b e BF-002c permanecem abertos.

## Affected Users & Stakeholders

- **Users**: jogador do cliente Bevy — precisa de uma tecla (`Espaço`) que registre sua presença e de um contador `Sincronias: N` que mude quando três aliados sinalizam na janela, em vez de um comando aceito sem efeito; operador da automação headless — precisa reproduzir os três sinais via CLI `insurgencia-servidor` e observar o signal e o contador sem janela gráfica.
- **Stakeholders**: mantenedores da sim — precisam de um único recipiente determinístico (`Mundo` + `avancar_tick` + `hash_estado`), sem nova regra canônica e sem quebrar replay; mantenedores do cliente — precisam reutilizar o padrão existente de `ComandoLocal`/ids monotônicos, semear um terceiro par local apenas como scaffolding e estender a HUD sem duplicar verdade de estado; SWE/orquestrador — precisa de uma fatia verificável por inspeção de código + comandos de prova em `STATE.md:602`, sem executar gates neste assessment; designer — precisa que 150 ticks / 3 peers continuem `[HIPÓTESE]` do GDD e que nenhuma topologia de 12 ilhas seja decidida. **PR #76 não é tocado** (outro owner ativo, verde mas `dirty`); `dev→main`/deploy são SWE-exclusivos.

## Goals

- Declarar `pub const TICKS_JANELA_SINCRONIA: u64 = 150` e os campos públicos `sincronias_iniciadas: u64`, `sinalizadores_sincronia: BTreeSet<u64>` e `sincronia_ticks_restantes: u64` em `Mundo`, zerados/vazios em `Mundo::new` — **DoD-1**.
- Fazer `avancar_tick` decrementar a janela uma vez por tick; primeiro sinal com janela zerada a abre; três `peer_id` distintos na mesma janela incrementam o contador, empurram `sincronia_iniciada:<tick>` em `sinais` e reiniciam janela e conjunto — **DoD-1**.
- Fazer `hash_estado` cobrir `sincronias_iniciadas`, `sincronia_ticks_restantes` e os IDs dos sinalizadores em ordem ordenada, preservando determinismo/replay — **DoD-1**.
- Ligar `Espaço` no cliente a `sinalizar_sincronia` com `seq`/`request_id` monotônicos, semear o terceiro par local em `configurar` e exibir `Sincronias: N` na HUD (com a dica de teclado atualizada) — **DoD-2**.
- Cobrir com testes de regressão a abertura da janela, o terceiro peer distinto, o sinal fora da janela, o signal emitido, o `hash_estado` e o caminho CLI — **FR-003**.
- Fechar **BF-015a** mantendo BF-015b e BF-002c abertos — **DoD-4**.
- Preservar a fronteira: nenhuma mudança em GDD/cânone, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede; nenhuma issue simulada (R6 N/A); PR #76 intocável — **DoD-4**.
- Registrar rastros **FR-001..FR-004** (sim estado/janela/hash, cliente Espaço/HUD, testes de regressão, evidência/release) e constituição v1.2.2 (R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, **R6/taskstoissues N/A**).

## Non-Goals

- Topologia de ilhas, rotas, spawning, captura de ilha, barra circular "0/3 → 3/3", gás lacrimogêneo, expulsão ou qualquer penalidade do GDD §8/§12 — são intenção de design, não este ciclo.
- Promover 150 ticks, três peers ou qualquer número do GDD a regra canônica; alterar whitelist, validador ou mapa de níveis.
- Rede real, Lightyear, lobby, predição, reconciliação cliente-servidor ou `async` — ADR-004 e fronteira do contrato 060.
- Consumo de `crates/contratos`, nova mensagem de protocolo, `PROTOCOL_VERSION`, Cargo, CI ou stack.
- Novas habilidades, Ato de Vandalismo (BF-002c), `sinalizar_alavanca` (BF-015b), HUD com `hash_estado` (BF-014) ou diagnóstico sem comportamento.
- Captura visual prometida, emissão de issues (R6), operações em `dev→main`/deploy, qualquer interação com PR #76 e execução de gates/build/testes/formatadores neste assessment.

## Success Metrics

- Com o primeiro `sinalizar_sincronia` e `sincronia_ticks_restantes == 0`, a janela passa a valer **150** ticks; cada `avancar_tick` a decrementa exatamente **1** (baseline: campo inexistente — **[NV]** até execução).
- Com três `peer_id` distintos sinalizando na mesma janela, `sincronias_iniciadas` incrementa **1**, `sincronia_iniciada:<tick>` entra em `sinais`, e janela/conjunto reiniciam; o quarto sinal não conta dentro da nova janela (baseline: nenhum efeito — **[NV]**).
- Com um sinal fora da janela, contador, conjunto e `sinais` permanecem íntegros; a janela é aberta somente pelo próximo primeiro sinal (baseline: **[NV]**).
- `hash_estado` muda quando contador, ticks restantes ou conjunto de sinalizadores mudam, com IDs ordenados de forma determinística (baseline: campos não cobertos — **[NV]**).
- No cliente, `Espaço` agenda `sinalizar_sincronia` com ids monotônicos, o terceiro par local existe no espelho e a HUD mostra `Sincronias: N` lida de `Mundo.sincronias_iniciadas` (baseline: comando inalcançável e dica sem `Espaço` em `main.rs:1362` — **[NV]**).
- Nenhuma mudança em GDD/cânone, sim além do recipiente acordado, contratos, protocolo, Cargo, CI, stack ou rede; estilo Rust (`forbid(unsafe_code)`, `deny(clippy::pedantic)`) e cobertura determinística preservados — **[NV]** até gates.

## Cost of Inaction

Sem o recorte, a regra de Ação Coletiva mais citada do GDD continua sem tradução no protótipo: o comando `sinalizar_sincronia` é aceito pelo validador e descartado em `fase_skills`, a tecla não existe e o painel não mostra progresso. Qualquer ciclo futuro que queira demonstrar coordenação entre jogadores precisaria reconstruir o mesmo recipiente — e o risco é reimplementarmos a regra no cliente, sem determinismo nem cobertura de hash, quando o Núcleo já é a autoridade natural. O custo é perder a única fatia barata que torna "três em cinco segundos" verificável antes de qualquer trabalho de ilha ou rede. BF-015a continua aberto e BF-015b/BF-002c seguem sem ponto de comparação.

## Open Questions

- **Nenhuma clarificação de produto bloqueante (R3 = 0)**: o contrato fixa janela, contador, sinal, hash, tecla, par local, HUD, consumidor e score; nenhum `[NEEDS CLARIFICATION]` a responder neste assessment.
- **[NV]** Comportamento exato do decaimento no tick 150 (5,0 s × 30 Hz) e a fronteira entre "dentro" e "fora" da janela precisam de verificação em execução.
- **[NV]** Nomes definitivos de SC/T na spec 060, o formato do comando via CLI `insurgencia-servidor` e a conformidade de R4/R7 dependem de gates não executados aqui.
- **[NV]** Captura visual do HUD com janela minimizada depende de GPU/janela; permanece fora deste assessment — a prova é estrutural.
- **[NV]** PR #76 está `dirty` e de outro owner; qualquer rebase/merge depende da SE, não deste ciclo.

## Rastreabilidade (escopo do problema)

- **FR-001** → sim: `TICKS_JANELA_SINCRONIA = 150`, `sincronias_iniciadas`, `sinalizadores_sincronia`, `sincronia_ticks_restantes`, decremento em `avancar_tick`, emissão de `sincronia_iniciada:<tick>` em `sinais` e cobertura do `hash_estado`.
- **FR-002** → cliente: tecla `Espaço` agendando `sinalizar_sincronia` com ids monotônicos, terceiro par local em `configurar` e HUD `Sincronias: N` (dica atualizada).
- **FR-003** → testes de regressão: abertura da janela, terceiro peer distinto, sinal fora da janela, signal, hash e CLI.
- **FR-004** → evidência/validação/release; prova por inspeção de código + comandos de `STATE.md:602`, **[NV]** neste assessment.
- SC-001..SC-006 → *Success Metrics* acima; T001..T010 → entregáveis da spec 060, fora desta fronteira. Os mesmos **DoD-1..DoD-4** e o score **360/3125** aparecem em `intake.md`, `research.md`, `shape.md` e `decision.md`.
