# Implementation Plan — 061-vandalismo-agitador

**Feature ID**: 061 · **Status**: Especificado · **Âncora**: `origin/dev = 52138c3` · **R1**: `go` · **Score**: 288/3125 (`4×4×2×3×3`) · **Sessão**: `insurgencia - Ato de Vandalismo do Agitador`.

**Ordem normativa (constituição v1.2.2, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → 10 tasks → 11 analyze (READ-ONLY) → 12 implement → 13 converge (append-only) → 27 validate (R4) → 26 release (R7 `1.0.0-local`). **R6/taskstoissues é N/A/bloqueado**: fora da sequência executável enquanto o adaptador de issues não existe — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado. Promoção `dev→main`, deploy e PR #76 são SWE-exclusivos e intocáveis.

**Fronteira deste agente**: escrever somente `spec.md`, `plan.md`, `tasks.md` e `checklists/requirements.md` em `specs/061-vandalismo-agitador/`, no worktree `.specify/queue/worktrees/cycle-061-vandalismo` (branch `feature/061-vandalismo`). Nenhum toque em código, GDD, CI, contrato de rede, `Cargo.toml`/`Cargo.lock`, `STATE.md`, `AGENTS.md` ou `concurrency.jsonl`. Nenhum gate, teste, formatador, build, stage, commit, push ou PR executado nesta etapa: tudo permanece **[NV]** até o orquestrador/SWE.

**Rotulagem obrigatória**: custo 20 Fôlego, alcance 2 m (`ALCANCE_CONTATO_M`), dano 120, silêncio 45 ticks (1,5 s × 30 Hz) e recarga 210 ticks (7 s) são **`[HIPÓTESE]`/protótipo** da decisão de 2026-11-25. Nenhum número é promovido a cânone; o GDD é lido, nunca reescrito. **BF-002c fecha**; **BF-015b** (`sinalizar_alavanca`) permanece aberto.

## Stack e venue

- Rust **1.99.1** com `rustfmt`/`clippy` (`rust-toolchain.toml`); Cargo é o único gerenciador, `Cargo.lock` o único lockfile, `--locked` no gate da sim/CLI. Convenção obrigatória: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, `Result`/`thiserror`, sem `unwrap`/`expect`/`panic!` em produção, `BTreeMap`/`BTreeSet`/`VecDeque`, `f64::total_cmp`, nomes em pt-BR e exceções de lint com razão explícita.
- Crates existentes reaproveitados: `insurgencia-sim` (autoridade determinística), `insurgencia-servidor` (CLI headless), `insurgencia-cliente` (binário Bevy + lib pura). Nenhum novo crate, contrato de rede, `PROTOCOL_VERSION` ou dependência.
- **Venue para compilação pesada: GitHub Actions** (jobs `gates` e `rust` de `.github/workflows/ci-gates.yml`). Gate de destino `./verify.sh` (check → clippy → nextest/test → fmt). A captura visual da HUD depende de X11/GPU e permanece **[NV]**; não se rebaixa timeout local nem se afirma verde sem saída colada.
- Comandos de prova (execução **[NV]** pelo orquestrador): `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo test --locked -p insurgencia-cliente --lib`, `cargo check --locked -p insurgencia-cliente --all-targets`, `./verify.sh`.

## Módulos a tocar

| Arquivo | Mudança | FR |
|:--|:--|:--|
| `crates/sim/src/lib.rs` | 15º token `vandalismo` em `ACOES_PERMITIDAS`; `acao_exige_destino` true; constantes `[HIPÓTESE]`, marcas em `Jogador`/`Mundo::new`, decrescentes em `avancar_tick`, validação em `ValidacaoComandos::validar`, ramo em `fase_skills` e cobertura em `hash_estado` | FR-001, FR-002, FR-003 |
| `crates/sim/tests/simulacao.rs` | Regressão determinística: aceite/recusa, 120 dano, 45 silêncio, cancelamento, 20 Fôlego, 210 recarga, monotônicos só no aceite e hash divergente/estável — sem golden literals e sem reescrever testes anteriores | FR-003 |
| `crates/cliente/src/main.rs` | Tecla protótipo `H` com destino e IDs monotônicos via validação antes de `agendar_comando`; `Q` preservado em `atacar_forte`; linha de estado em `texto_hud` | FR-004 |
| `crates/servidor/tests/cli.rs` | Aridade da linha `vandalismo` (6 tokens), whitelist de 15 tokens, `acao_exige_destino` true e formato `<tick>\t<sha256>` preservado | FR-001 |

## Fases

1. **00 Constituição / 01 Intake**: v1.2.2 aplicada, R1 `go`, R6 N/A; score **288/3125** (BF-002c contra BF-015b `144/3125`), consumidor jogador da Função Agitadora + CLI; base `origin/dev = 52138c3`.
2. **02 Research / 03 Define**: ler `ACOES_PERMITIDAS`, `acao_exige_destino`, `nivel_minimo_para_acao`, `ValidacaoComandos::validar`, `fase_skills`, rota única `aplicar_dano_a_jogador`, `hash_estado`, despacho de teclas e `texto_hud`. Problema: hoje `vandalismo` não existe como token e não produz efeito; DoD-1..DoD-4 nomeiam comportamento, consumidor, comandos de prova e backlog.
3. **04 Shape / 05 Decide**: opção escolhida = contrato local + validação + efeito no sim + tecla/HUD + regressão. Rejeitadas: regra canônica de dano/expulsão no GDD, topologia de 12 ilhas, HUD que exiba hash como diagnóstico, `sinalizar_alavanca`.
4. **06 Specify / 07 Clarify / 08 Plan**: `spec.md` com US1–US4, FR-001..FR-004 ↔ SC-001..SC-004, **R3 = 1** (uma pergunta respondida na Session 2026-11-25), nenhuma pendência de clarificação em aberto, escopo fora explícito; este plano e a venue GitHub Actions.
5. **09 Checklist / 10 Tasks**: `checklists/requirements.md` com CHK001..CHK008 rastreáveis a FR/SC; `tasks.md` com T001..T010 únicos e fases Setup → Foundational → User Stories → Polish, sem seções de implementação.
6. **11 Analyze (READ-ONLY)**: convergência entre decisão 80/20, código lido e governança; `analyze.md` é insumo, não é reescrito nesta etapa; `tasks` precede `analyze`.
7. **12 Implement (pelo orquestrador/SWE)**: nos quatro módulos acima, na ordem T003→T009, preservando convenção Rust e a regra de monotônicos só no aceite.
8. **13 Converge (append-only) / 27 Validate (R4) / 26 Release (R7)**: entrada datada em `converge.md`, validação condicionada ao roteiro antes do release `1.0.0-local`, evidência com saídas e carimbos; BF-002c fechado e BF-015b aberto. Fora da fronteira deste agente.

## Rastreabilidade (FR → SC → fase → T)

| FR | SC | Fases | T |
|:--|:--|:--|:--|
| FR-001 contrato (`vandalismo` 15º, com destino) | SC-001 token e aridade observáveis | 02, 06, 12 | T001, T003, T009, T010 |
| FR-002 validação (alvo, 2 m, 20 Fôlego, 210 ticks) | SC-002 aceite e recusa determinísticos | 03, 06, 12 | T004, T006, T009 |
| FR-003 efeito (120 dano, 45 silêncio, cancela canalização, monotônicos só no aceite, hash) | SC-003 efeito exato e hash coerente | 03, 06, 12 | T005, T007, T009, T010 |
| FR-004 cliente (`H` + HUD) | SC-004 consumidor e governança | 04, 06, 12 | T002, T008, T009, T010 |

## Riscos e mitigação

| Risco | Mitigação |
|:--|:--|
| `hash_estado` muda ao cobrir novo estado e quebra comparações antigas | Atualizar baselines e testes de hash **junto** com o estado novo; correlacionar a mudança a FR-003/SC-003; nunca afrouxar expectativa para obter verde. |
| Golden literals de hash não existem na sim | `crates/sim/tests/simulacao.rs` **não contém literais de hash de 64 hex**: afirmar determinismo entre replays equivalentes e divergência da baseline, sem introduzir literal fixo na sim. Se a CLI usar baselines, atualizá-las na mesma mudança. |
| Promoção indevida de `[HIPÓTESE]` a cânone | Todos os números rotulados na spec, neste plano e no checklist; GDD lido, nunca reescrito; nenhuma regra nova de dano/expulsão/topologia. |
| Comando recusado avançar monotônicos ou cobrar recurso | `seq`/`request_id`, Fôlego e recarga atualizados somente no ramo aceito em `fase_skills`; recusa tipada preserva `ultima_recusa`, que participa do hash. |
| Canalização do alvo sobreviver ao silêncio | No mesmo ramo aceito, zerar os campos de canalização do alvo em uma única passagem, sem estado parcial observável pelo hash. |
| Decrescentes de silêncio/recarga duplicados ou perdidos | Decremento exatamente uma vez por tick em ordem estável de `peer_id`, reaproveitando o padrão das recargas existentes. |
| Tecla `H` conflitar com `Q`/`atacar_forte` | Despacho explícito de `KeyCode::KeyH`; `Q` permanece intacto; `H` é `[HIPÓTESE]` registrado na única clarificação respondida. |
| Segunda verdade na HUD | `texto_hud` lê exclusivamente o estado da sim; nenhuma regra de validação paralela no cliente. |
| Escopo vazar para CI, rede, 5v5 ou 12 ilhas | Fronteira explícita nos quatro artefatos; nenhum protocolo, `Cargo.toml`, workflow ou GDD tocado; BF-015b permanece aberto. |
| R6 adotada por engano | N/A/bloqueado sob v1.2.2: PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado. |
| Verde afirmado sem execução | Gates, testes, build e capturas **[NV]** até o orquestrador; venue GitHub Actions; nenhum timeout local rebaixado para RED. |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Rodar os quatro comandos de prova e `./verify.sh` no head da feature; conferir `ALL STAGES OK`, jobs `gates`/`rust` e a convenção Rust nos quatro módulos.
- Conferir T001..T010 únicos, nenhum marcador de pendência, rastreabilidade FR↔SC↔T e que `tasks` antecede `analyze` (READ-ONLY), `converge` é append-only e R4 antecede R7 `1.0.0-local`.
- Corrigir divergências na fonte, nunca na expectativa; rollback documentado é o revert do commit único de feature, não executado nesta etapa. BF-002c passa a fechado; BF-015b permanece aberto; promoção e deploy continuam SWE-exclusivos.
