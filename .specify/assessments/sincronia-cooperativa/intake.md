# Intake — Sincronia cooperativa observável

**Data**: 2026-11-25 · **Slug**: `sincronia-cooperativa` · **Sessão**: `insurgencia - sincronia cooperativa` · **Ciclo**: 060 · **Pendência**: BF-015a

## Ideia

Capturar, sem reabrir regra canônica, a fatia 80/20 do BF-015a: dar **efeito observável** à ação `sinalizar_sincronia`. O comando já está na whitelist do Núcleo (`crates/sim/src/lib.rs:217-231`), dispensa `destino` (`:258,262`) e passa no gate de nível 1 (`:242-248`), mas `fase_skills` (`:875-966`) não tem ramo algum para ele: o comando apenas avança `seq`/`request_id`. A contribuição é uma **janela determinística de 150 ticks (5 s × 30 Hz)**: o primeiro sinal com janela zerada a abre; três `peer_id` distintos sinalizando dentro dela incrementam `sincronias_iniciadas`, emitem `sincronia_iniciada:<tick>` em `sinais` e reiniciam janela e conjunto. O cliente ganha a tecla `Espaço`, semeia um terceiro par local no espelho e exibe `Sincronias: N`; `hash_estado` passa a cobrir o estado novo.

**Motivação**: fechar BF-015a sem tocar em GDD/cânone, topologia de ilhas, dano de gás, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede. O GDD fixou a regra "três aliados em cinco segundos" como intenção de Ação Coletiva (§8.1:358, §8.3:386, §12.2:921,937), mas o protótipo não tem nenhum recipiente no Núcleo para registrar a presença — a ação existe no contrato e no validador e desaparece no processamento do tick. É **plumbing determinístico com consumidor observável**, não nova habilidade e não implementação da ilha. Os números 150 ticks / 3 peers são `[HIPÓTESE]` derivada do GDD, nunca promovidos a cânone.

## Evidências de entrada (coladas; não reexecutadas)

- [source: `crates/sim/src/lib.rs:217-231`, lido] `ACOES_PERMITIDAS` inclui `"sinalizar_sincronia"` — o comando é conhecido e aceito pelo validador de whitelist. (confidence: high)
- [source: `crates/sim/src/lib.rs:258,262`, lido] Comentário e predicate confirmam que `sinalizar_sincronia` (com `sinalizar_alavanca`) dispensa `destino`: não há exigência de ponto na validação. (confidence: high)
- [source: `crates/sim/src/lib.rs:242-248`, lido] `nivel_minimo_para_acao` não lista `sinalizar_sincronia`; ações sem slot ficam no nível 1, logo qualquer jogador autorizado pode sinalizar. (confidence: high)
- [source: `crates/sim/src/lib.rs:2414-2475`, lido] `ValidacaoComandos::validar` checa peer, replay de `seq`/`request_id`, whitelist, nível e os ramos com `destino` (`mover`, `coletar`, barreira, ponte); **nenhum ramo exige ou processa sinal de sincronia** — o aceite é apenas estrutural. (confidence: high)
- [source: `crates/sim/src/lib.rs:875-966`, lido] `fase_skills` tem ramos para `mover`, `atacar_forte`, `coletar_materiais`, construções, propaganda, nós, `atacar_pnj`, `analise_materialista`, `escudo` e `armadilha`, mas **nenhum para `sinalizar_sincronia`**: ao final do loop (`:960-965`) apenas `seq`/`request_id` são atualizados. É a raiz do BF-015a. (confidence: high)
- [source: `crates/cliente/src/main.rs:1362`, lido] `DICA_TECLAS` anuncia WASD/setas, B, P, C, G, R, F, T, E, Y e V — **não há menção a `Espaço` nem a sincronia**; a ação não é acessível nem descoberta pelo jogador. (confidence: high)
- [source: `docs/GDD - Insurgência.md:349-416`, lido] §8.1 tabela de rotas fixa "3 jogadores em janela de 5 s" na Rota Central (`:358`); §8.3 exige três aliados entrando simultaneamente e descreve gás/expulsão como penalidade (`:386,391-395`). A regra de três-em-cinco é intenção de design; **topologia e penalidade não entram neste ciclo**. (confidence: high)
- [source: `docs/GDD - Insurgência.md:921,937,943-950`, lido] §12.2 reserva a **barra de espaço** para "Sinalizar sincronia", com o primeiro sinal abrindo a janela de 5 s (`:921`); critério: três jogadores em 5,0 s iniciam o desafio, em 5,1 s são repelidos (`:937`); a barra "0/3 → 3/3" é ilustração de UI (`:943-950`). (confidence: high)
- [source: contrato da sessão, imutável] Worktree `.specify/queue/worktrees/cycle-060/`, branch `feature/060-sincronia-cooperativa`, base `origin/dev = e0e8f38`; `pub const TICKS_JANELA_SINCRONIA: u64 = 150`; campos públicos de `Mundo` `sincronias_iniciadas: u64`, `sinalizadores_sincronia: BTreeSet<u64>`, `sincronia_ticks_restantes: u64` (zero/vazio em `Mundo::new`); `avancar_tick` decrementa a janela uma vez por tick; primeiro sinal com janela zerada a abre; três peers distintos na janela incrementam o contador, emitem `sincronia_iniciada:<tick>` em `sinais` e reiniciam janela/conjunto; `hash_estado` cobre contador, ticks e IDs ordenados. Cliente: `Espaço` agenda `sinalizar_sincronia` com ids monotônicos, terceiro par local semeado em `configurar`, HUD `Sincronias: N`. (confidence: high)
- [source: `.specify/memory/STATE.md:594-604`, lido] Decisão 80/20 do ciclo 060: BF-015a escolhido por **360/3125** (`5×4×3×2×3`); BF-002c 288/3125, BF-015b 144/3125; DoD(0.9) com comportamento, consumidor, comandos de prova e backlog fixados. Âncora: cinco gates PASS (`sdd 58/0`, `dag 58/632/311/0`, `task-id 0 dup`, `converge 41/41`, `hub 0 erros`). PR #76 (`dev→main`) está `OPEN`, `mergeable=false/dirty`, checks `gates`/`rust` verdes no run `45525589489` — SWE-exclusivo e de outro dono. (confidence: high)
- [source: `.specify/memory/constitution.md` via contrato, lido] Constituição v1.2.2: R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY, `converge` append-only; **R6/taskstoissues N/A** — PARE se invocada, nunca issue simulada, nunca `gh` improvisado. (confidence: high)

## Scorecard único do ciclo

| Candidato | Impacto × Confiança × Fechamento × Esforço⁻¹ × Risco⁻¹ ÷ 3125 | Resultado | Consumidor |
|:--|:--|:--|:--|
| `sinalizar_sincronia` (BF-015a, escolhido) | 5×4×3×2×3 = **360/3125** | **360/3125** | jogador do cliente Bevy (Espaço) + automação headless via CLI |
| Ato de Vandalismo (BF-002c) | 4×4×2×3×3 = 288/3125 | 288/3125 | jogador/servidor |
| `sinalizar_alavanca` (BF-015b) | 4×3×3×2×2 = 144/3125 | 144/3125 | jogador/servidor |

Critério normativo do contrato: **impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹**, escala 1–5, denominador **5⁵ = 3125**; o quarto e o quinto fatores são inversos de esforço e de risco (5 = menor esforço/risco, 1 = maior). Filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo. Impacto **5** porque a Ação Coletiva é pilar do GDD e a fatia destrava a primeira mecânica cooperativa observável; confiança **4** porque whitelist, validador, nível e ausência de ramo foram lidos no código, embora a janela de 150 ticks seja `[HIPÓTESE]`; fechamento **3** porque consumidor e comandos de prova estão nomeados, mas a spec 060 e os testes ainda serão escritos; esforço⁻¹ **2** porque a mudança cruza sim (estado, tick, hash) e cliente (tecla, par local, HUD); risco⁻¹ **3** porque nenhuma regra canônica é alterada, mas novos campos públicos e o `hash_estado` precisam permanecer determinísticos. Rejeitados sem pontuação paralela: BF-003b (depende de topologia canônica), BF-001/006/007/009/010 (autorização), BF-004/005 (execução externa), BF-013/014 (diagnóstico). Este é o **único** scorecard do assessment; os mesmos valores reaparecem em `research.md`, `shape.md` e `decision.md`.

## DoD do ciclo (referência única; repetido nos cinco artefatos)

- **DoD-1 (janela e estado)**: primeiro `sinalizar_sincronia` com `sincronia_ticks_restantes == 0` abre a janela de `TICKS_JANELA_SINCRONIA = 150` ticks (5 s × 30 Hz); o terceiro `peer_id` distinto na janela incrementa `sincronias_iniciadas`, empurra `sincronia_iniciada:<tick>` em `sinais` e reinicia janela/conjunto; sinal fora da janela não conta e não corrompe estado; `hash_estado` cobre contador, ticks restantes e IDs ordenados.
- **DoD-2 (consumidor)**: jogador do cliente Bevy (tecla `Espaço`, `seq`/`request_id` monotônicos e terceiro par local semeado em `configurar`) e automação headless via CLI `insurgencia-servidor`.
- **DoD-3 (prova)**: `cargo test --locked -p insurgencia-sim --test simulacao`; `cargo test --locked -p insurgencia-servidor --test cli`; `cargo test --locked -p insurgencia-cliente --lib`; `cargo check --locked -p insurgencia-cliente --all-targets`; `./verify.sh` remoto (`gates`/`rust`).
- **DoD-4 (fronteira/backlog)**: sem topologia de ilhas, dano de gás, promoção de número a cânone ou mudança em GDD/`crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`/CI/stack/rede; **BF-015a passa a `fechado`**; BF-015b e BF-002c permanecem abertos.

## Incógnitas de primeira vista

- **[NV]** Efeito renderizado do HUD e do indicador de sincronia depende de janela/GPU; este assessment é somente leitura.
- **[NV]** Nomes definitivos de SC/T na spec 060 e a conformidade de R4/R7 dependem de gates não executados aqui.
- **[NV]** O comportamento exato do decaimento da janela no tick 150 (fronteira 5,0 s × 30 Hz) precisa de verificação em tempo de execução pelo orquestrador.
- **[NV]** A automação CLI deve gerar três `peer_id` distintos sem servidor de rede; o formato do comando via `insurgencia-servidor` fica na especificação.
- **[NV]** PR #76 está verde, mas `dirty` e de outro owner: nenhuma interação, rebase ou merge.

## Rastreabilidade canônica (spec 060)

**FR-001** = estado e janela na sim (`TICKS_JANELA_SINCRONIA = 150`, `sincronias_iniciadas`, `sinalizadores_sincronia`, `sincronia_ticks_restantes`, decremento em `avancar_tick`, emissão de `sincronia_iniciada:<tick>` e cobertura do `hash_estado`); **FR-002** = cliente com tecla `Espaço`, ids monotônicos, terceiro par local em `configurar` e HUD `Sincronias: N`; **FR-003** = testes de regressão (janela, terceiro peer distinto, sinal fora da janela, hash, CLI); **FR-004** = evidência/validação/release (`[NV]` aqui). **DoD-1..DoD-4** e o score **360/3125** (`5×4×3×2×3`) reaparecem em `research.md`, `define.md`, `shape.md` e `decision.md`.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/sincronia-cooperativa/`. Não executa gates, build, testes, formatadores ou linters; não faz `git add`/commit/push/PR/merge; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/*`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. **PR #76 (`dev→main`) pertence a outro owner ativo, está verde mas `dirty`, e não é tocado**; `dev→main` e deploy são SWE-exclusivos. R6/taskstoissues **N/A** por constituição v1.2.2 — nenhuma issue simulada, nenhum `gh` improvisado. Gate de destino: `./verify.sh` (jobs `gates`/`rust`); CLI de PR = `gh`. Tudo que depende de execução permanece **[NV]**.
