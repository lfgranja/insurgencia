# Tasks: Ato de Vandalismo do Agitador (061)

**Feature**: `061-vandalismo-agitador` — ver `spec.md`, `plan.md` e `checklists/requirements.md`. R1 `go`, score 288/3125 (`4×4×2×3×3`), BF-002c.

**Convenção**: cada tarefa segue o formato `- [ ] T###` com descrição e ao menos um caminho de arquivo entre crases, IDs únicos e `Bloqueado por: T###` somente para dependência real. Todos os números são `[HIPÓTESE]`/protótipo. Execução de testes, gates, build, captura, commit, push e PR pertence ao orquestrador/SWE e permanece **[NV]** nesta etapa documental. Nenhum marcador de pendência permanece na spec.

## Phase 1: Setup

- [ ] T001 Confirmar em `.specify/feature.json` que a identidade é `specs/061-vandalismo-agitador` e em `specs/061-vandalismo-agitador/spec.md` a âncora `origin/dev = 52138c3`, o branch `feature/061-vandalismo` e o R1 `go`, sem usar nome de branch como identidade (FR-001, SC-001).
- [ ] T002 Fixar em `specs/061-vandalismo-agitador/spec.md` o DoD-1..DoD-4, o rótulo `[HIPÓTESE]` de custo 20/alcance 2 m/dano 120/silêncio 45/recarga 210 e a tecla protótipo `H`, conferindo que a única clarificação da Session 2026-11-25 está respondida e que nenhuma pendência de clarificação resta (FR-002, FR-003, FR-004, SC-002, SC-003, SC-004).

## Phase 2: Foundational

- [ ] T003 Acrescentar `vandalismo` como 15º token de `ACOES_PERMITIDAS` e manter `acao_exige_destino("vandalismo") == true` em `crates/sim/src/lib.rs`, preservando `false` apenas para as duas ações sem destino; atualizar a aridade tratada em `crates/servidor/tests/cli.rs` sem parser local nem whitelist paralela (FR-001, SC-001). Bloqueado por: T002.
- [ ] T004 Definir em `crates/sim/src/lib.rs` as constantes `[HIPÓTESE]` (`CUSTO_VANDALISMO_FOLEGO = 20`, `ALCANCE_CONTATO_M = 2.0` reaproveitado, `DANO_VANDALISMO = 120`, `TICKS_SILENCIO_VANDALISMO = 45`, `RECARGA_VANDALISMO_TICKS = 210`), as marcas de silêncio/recarga do alvo e do agressor em `Jogador`/`Mundo::new()` e a cobertura correspondente em `hash_estado` (FR-002, FR-003, SC-002, SC-003). Bloqueado por: T003.
- [ ] T005 Decrementar em `crates/sim/src/lib.rs` silêncio e recarga de `vandalismo` exatamente uma vez por tick em `avancar_tick`, em ordem estável de `peer_id`, reaproveitando o padrão das recargas existentes e sem golden literals na sim (FR-003, SC-003). Bloqueado por: T004.

## Phase 3: User Stories

- [ ] T006 Validar em `crates/sim/src/lib.rs` (`ValidacaoComandos::validar`) o alvo no destino, o alcance de 2 m, o Fôlego ≥ 20 e a recarga zerada de `vandalismo`, com recusa tipada em `CodigoRecusa` que não altera `seq`, `request_id`, Fôlego nem o alvo (FR-002, SC-002). Bloqueado por: T004.
- [ ] T007 Implementar em `crates/sim/src/lib.rs` (`fase_skills`) o ramo de `vandalismo`: 120 de dano pela rota única `aplicar_dano_a_jogador`, 45 ticks de silêncio, cancelamento da canalização do alvo, custo 20 e recarga 210 no agressor, com monotônicos avançando somente no aceito e `ultima_recusa` preservada na recusa (FR-003, SC-003). Bloqueado por: T005, T006.
- [ ] T008 Despachar em `crates/cliente/src/main.rs` a tecla protótipo `KeyCode::KeyH` para `vandalismo` com `destino` e IDs monotônicos, validando antes de `agendar_comando`, preservando `KeyCode::KeyQ`/`atacar_forte`, e acrescentar em `texto_hud` uma linha de estado lida da sim sem regra paralela (FR-004, SC-004). Bloqueado por: T006.
- [ ] T009 Escrever em `crates/sim/tests/simulacao.rs` a regressão determinística (aceite, recusa, 120 dano, 45 silêncio, cancelamento, 20 Fôlego, 210 recarga, monotônicos só no aceite e hash divergente/estável, sem golden literals e sem reescrever testes anteriores) e em `crates/servidor/tests/cli.rs` a cobertura de aridade/whitelist e do formato `<tick>\t<sha256>` (FR-001, FR-002, FR-003, SC-001, SC-002, SC-003). Bloqueado por: T007.

## Phase 4: Polish

- [ ] T010 Registrar evidência, validação e release do corte em `specs/061-vandalismo-agitador/plan.md`: roteiro com `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo test --locked -p insurgencia-cliente --lib`, `cargo check --locked -p insurgencia-cliente --all-targets` e `./verify.sh` (jobs `gates`/`rust` no GitHub Actions), R4 antes de R7 `1.0.0-local`, BF-002c fechado, BF-015b aberto e R6 N/A, com toda execução **[NV]** até o orquestrador/SWE (FR-004, SC-004). Bloqueado por: T008, T009.
