# Implementation Plan — 062-coleta-eficiente

**Feature ID**: 062 · **Status**: Especificado · **Âncora**: `origin/dev = d3be560` · **R1**: `go` · **Score**: 1024/3125 (`4×4×4×4×4`) · **Sessão**: `insurgencia - Coleta Eficiente do Camponês`.

**Ordem normativa (constituição v1.2.2, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → 10 tasks → 11 analyze (READ-ONLY) → 12 implement → 13 converge (append-only) → 27 validate (R4) → 26 release (R7 `1.0.0-local`). **R6/taskstoissues é N/A/bloqueado**: fora da sequência executável enquanto o adaptador de issues não existe — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado. Promoção `dev→main`, deploy e PR #76 (verde, mas `CONFLICTING`, de outro dono) são SWE-exclusivos e intocáveis.

**Fronteira deste agente**: escrever somente `spec.md`, `plan.md`, `tasks.md` e `checklists/requirements.md` em `specs/062-coleta-eficiente/`, no worktree `.specify/queue/worktrees/cycle-062-coleta-eficiente` (branch `feature/062-coleta-eficiente`). Nenhum toque em código, GDD, CI, contrato de rede, `Cargo.toml`/`Cargo.lock`, `STATE.md`, `AGENTS.md` ou `concurrency.jsonl`. Nenhum gate, teste, formatador, build, stage, commit, push ou PR executado nesta etapa: tudo permanece **[NV]** até o orquestrador/SWE.

**Rotulagem obrigatória**: custo 15 de Fôlego, buff de 150 ticks (5 s × 30 Hz), conclusão da coleta em 90 ticks contra 180 (`TICKS_COLETA`) e recarga de 300 ticks (10 s) são **`[HIPÓTESE]`/protótipo** da decisão de 2026-11-25, lidos em `GDD:646`. Nenhum número é promovido a cânone; o GDD é lido, nunca reescrito. **BF-002k fecha**; os demais backlog permanecem abertos. A passiva de Ouro/XP fica fora de escopo por não haver primitiva no núcleo.

## Stack e venue

- Rust **1.99.1** com `rustfmt`/`clippy` (`rust-toolchain.toml`); Cargo é o único gerenciador, `Cargo.lock` o único lockfile, `--locked` no gate da sim/CLI. Convenção obrigatória: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, `Result`/`thiserror`, sem `unwrap`/`expect`/`panic!` em produção, `BTreeMap`/`BTreeSet`/`VecDeque`, `f64::total_cmp` (sem epsilon), nomes em pt-BR e exceções de lint escopadas com razão explícita.
- Crates existentes reaproveitados: `insurgencia-sim` (autoridade determinística), `insurgencia-servidor` (CLI headless), `insurgencia-cliente` (binário Bevy + lib pura). Nenhum novo crate, contrato de rede, `PROTOCOL_VERSION`, dependência ou relógio externo.
- **Venue para compilação pesada: GitHub Actions** (jobs `gates` e `rust` de `.github/workflows/ci-gates.yml`). Gate de destino `./verify.sh` (check → clippy → nextest/test → fmt). A captura visual da HUD depende de X11/GPU e permanece **[NV]**; não se rebaixa timeout local nem se afirma verde sem saída colada.
- Comandos de prova (execução **[NV]** pelo orquestrador): `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo check --locked -p insurgencia-cliente --all-targets`, `./verify.sh`.

## Módulos a tocar

| Arquivo | Mudança | FR |
|:--|:--|:--|
| `crates/sim/src/lib.rs` | novo token `coleta_eficiente` (whitelist com 16 tokens) em `ACOES_PERMITIDAS`; `acao_exige_destino` true; constantes `[HIPÓTESE]` (15 Fôlego, 150 buff, 300 recarga, meta 90 vs `TICKS_COLETA` 180); campos de buff/recarga em `Jogador`/`Mundo::new()`, validação em `ValidacaoComandos::validar`, ramo em `fase_skills`, decrescentes exatamente uma vez por tick em `avancar_tick` e cobertura em `serializar_jogadores`/`hash_estado` | FR-001, FR-002, FR-003 |
| `crates/sim/tests/simulacao.rs` | Regressão determinística: aceite/recusa, Fôlego −15, buff 150, recarga 300, 90 vs 180 ticks, sem *stacking*, monotônicos só no aceite e hash divergente/estável — sem golden literals, sem reescrever testes anteriores e com `const` antes do primeiro statement de cada função de teste | FR-003 |
| `crates/cliente/src/main.rs` | Tecla protótipo `KeyCode::KeyK` com destino igual à posição local e IDs monotônicos via validação antes de `agendar_comando`; linha de estado em `texto_hud` lida do espelho da sim | FR-004 |
| `crates/servidor/tests/cli.rs` | Aridade da linha `coleta_eficiente` (6 tokens), whitelist de 16 tokens, `acao_exige_destino` true e formato `<tick>\t<sha256>` preservado | FR-001 |

## Fases

1. **00 Constituição / 01 Intake**: v1.2.2 aplicada, R1 `go`, R6 N/A; score **1024/3125** (BF-002k contra BF-002n `216/3125` e BF-015b `144/3125`), consumidor jogador do Camponês + CLI; base [EV] `origin/dev = d3be560`.
2. **02 Research / 03 Define**: ler `ACOES_PERMITIDAS`, `acao_exige_destino`, `nivel_minimo_para_acao`, `ValidacaoComandos::validar`, `fase_skills`, `TICKS_COLETA`/`coleta_ticks`/`coleta_ponto`, `serializar_jogadores`, `hash_estado`, despacho de teclas e `texto_hud`. Problema: hoje `coleta_eficiente` não existe como token e o ativo não tem efeito; DoD-1..DoD-4 nomeiam comportamento, consumidor, comandos de prova e backlog.
3. **04 Shape / 05 Decide**: opção escolhida = contrato local + validação + efeito no sim + tecla/HUD + regressão. Rejeitadas: regra canônica no GDD, passiva de Ouro/XP (sem primitiva), topologia de 12 ilhas, HUD que exiba hash como diagnóstico, `sinalizar_alavanca`.
4. **06 Specify / 07 Clarify / 08 Plan**: `spec.md` com US1–US4, Edge Cases, FR-001..FR-004 ↔ SC-001..SC-004, **R3 = 1** (uma pergunta respondida na Session 2026-11-25), nenhuma pendência de clarificação em aberto, escopo fora explícito; este plano e a venue GitHub Actions.
5. **09 Checklist / 10 Tasks**: `checklists/requirements.md` com CHK001..CHK008 rastreáveis a FR/SC; `tasks.md` com T001..T010 únicos e fases Setup → Foundational → User Stories → Polish, com `Bloqueado por:` somente para dependências reais.
6. **11 Analyze (READ-ONLY)**: convergência entre decisão 80/20, código lido e governança; `analyze.md` é insumo, não é reescrito nesta etapa; `tasks` precede `analyze`.
7. **12 Implement (pelo orquestrador/SWE)**: nos quatro módulos acima, na ordem T003→T009, preservando a convenção Rust, a regra de monotônicos só no aceite e a declaração de `const` no topo das funções de teste.
8. **13 Converge (append-only) / 27 Validate (R4) / 26 Release (R7)**: entrada datada em `converge.md`, validação condicionada ao roteiro antes do release `1.0.0-local`, evidência com saídas e carimbos; BF-002k fechado, demais itens do backlog abertos. Fora da fronteira deste agente.

## Rastreabilidade (FR → SC → fase → T)

| FR | SC | Fases | T |
|:--|:--|:--|:--|
| FR-001 contrato (`coleta_eficiente` presente em whitelist de 16 tokens, com destino) | SC-001 token e aridade observáveis | 02, 06, 12 | T001, T003, T009, T010 |
| FR-002 validação (destino = posição, 15 Fôlego, recarga 300, sem *stacking*) | SC-002 aceite e recusa determinísticos | 03, 06, 12 | T004, T006, T009 |
| FR-003 efeito (buff 150, 90 vs 180 ticks, monotônicos só no aceite, hash e decrescentes) | SC-003 efeito exato e hash coerente | 03, 06, 12 | T005, T007, T009, T010 |
| FR-004 cliente (`K` + HUD) | SC-004 consumidor e governança | 04, 06, 12 | T002, T008, T009, T010 |

## Riscos e mitigação

| Risco | Mitigação |
|:--|:--|
| `hash_estado` muda ao cobrir buff/recarga novos e quebra comparações antigas | Atualizar baselines e testes de hash **junto** com o estado novo; correlacionar a mudança a FR-003/SC-003; nunca afrouxar expectativa para obter verde. |
| `const` declarado depois de um statement em função de teste | `clippy::items_after_statements` é `deny` via `clippy::pedantic` (ciclo 061 falhou nisso): declarar todo `const` no início da função, antes do primeiro statement, em `crates/sim/tests/simulacao.rs` e em qualquer teste novo. |
| Golden literals de hash não existem na sim | `crates/sim/tests/simulacao.rs` **não contém literais de hash de 64 hex**: afirmar determinismo entre replays equivalentes e divergência da baseline, sem introduzir literal fixo na sim. Se a CLI usar baselines, atualizá-las na mesma mudança. |
| Promoção indevida de `[HIPÓTESE]` a cânone | Todos os números rotulados na spec, neste plano e no checklist; `GDD:646` lido, nunca reescrito; nenhuma regra nova de Ouro/XP, topologia ou recompensa. |
| Comando recusado avançar monotônicos ou cobrar recurso | `seq`/`request_id`, Fôlego, buff e recarga atualizados somente no ramo aceito em `fase_skills`; recusa tipada preserva `ultima_recusa`, que participa do hash. |
| Reaplicação durante buff acumular duração ou recarga | `ValidacaoComandos::validar` e o ramo aceito exigem buff 0 e recarga 0; um segundo `coleta_eficiente` é recusado sem alterar os campos já ativos. |
| Destino diferente da posição própria ser aceito | Comparar `destino` com `jogador.posicao` por igualdade determinística (`f64::total_cmp`/`ponto_igual`), sem tolerância epsilon ad hoc; recusa tipada e sem custo. |
| Contagem da coleta acelerada errada | A meta de conclusão é 90 sob buff e `TICKS_COLETA` (180) sem buff, derivada do estado autoritativo; a recompensa permanece a mesma e o progresso é reiniciado ao trocar de ponto, pela regra existente. |
| Decrescentes de buff/recarga duplicados ou perdidos | Decremento exatamente uma vez por tick em ordem estável de `peer_id`, reaproveitando o padrão das recargas existentes; campos cobertos por `serializar_jogadores`/`hash_estado` no mesmo commit. |
| Tecla `K` conflitar com despachos existentes | Despacho explícito de `KeyCode::KeyK` (livre); demais teclas preservadas; `K` é `[HIPÓTESE]` registrado na única clarificação respondida. |
| Segunda verdade na HUD | `texto_hud` lê exclusivamente o estado da sim; nenhuma regra de validação paralela no cliente. |
| Escopo vazar para CI, rede, 5v5, topologia ou Ouro/XP | Fronteira explícita nos quatro artefatos; nenhum protocolo, `Cargo.toml`, workflow ou GDD tocado; passiva Ouro/XP e BF-015b permanecem fora. |
| R6 adotada por engano | N/A/bloqueado sob v1.2.2: PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado. |
| Verde afirmado sem execução | Gates, testes, build e capturas **[NV]** até o orquestrador; venue GitHub Actions; nenhum timeout local rebaixado para RED. |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Rodar os comandos de prova e `./verify.sh` no head da feature; conferir `ALL STAGES OK`, jobs `gates`/`rust` e a convenção Rust nos quatro módulos, inclusive `const` no início das funções de teste.
- Conferir T001..T010 únicos, nenhum marcador de pendência, rastreabilidade FR↔SC↔T e que `tasks` antecede `analyze` (READ-ONLY), `converge` é append-only e R4 antecede R7 `1.0.0-local`.
- Corrigir divergências na fonte, nunca na expectativa; rollback documentado é o revert do commit único de feature, não executado nesta etapa. BF-002k passa a `fechado`, demais backlog abertos; PR #76, promoção e deploy continuam SWE-exclusivos.
