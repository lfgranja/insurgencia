# Tasks: Coleta Eficiente do Camponês (062)

**Feature**: `062-coleta-eficiente` — ver `spec.md`, `plan.md` e `checklists/requirements.md`. R1 `go`, score 1024/3125 (`4×4×4×4×4`), BF-002k.

**Convenção**: cada tarefa segue o formato `- [ ] T###` com descrição e ao menos um caminho de arquivo entre crases, IDs únicos e `Bloqueado por: T###` somente para dependência real. Todos os números são `[HIPÓTESE]`/protótipo. Execução de testes, gates, build, captura, commit, push e PR pertence ao orquestrador/SWE e permanece **[NV]** nesta etapa documental. Nenhum marcador de pendência permanece na spec. Em funções de teste, todo `const` fica antes do primeiro statement (`clippy::items_after_statements` é `deny` via pedantic).

## Phase 1: Setup

- [ ] T001 Confirmar em `.specify/feature.json` que a identidade é `specs/062-coleta-eficiente` e em `specs/062-coleta-eficiente/spec.md` a âncora `origin/dev = d3be560`, o branch `feature/062-coleta-eficiente` e o R1 `go`, sem usar nome de branch como identidade (FR-001, SC-001).
- [ ] T002 Fixar em `specs/062-coleta-eficiente/spec.md` o DoD-1..DoD-4, o rótulo `[HIPÓTESE]` de custo 15 Fôlego/buff 150 ticks/coleta 90 vs 180 ticks/recarga 300 ticks e a tecla protótipo `K`, conferindo que a única clarificação da Session 2026-11-25 está respondida (passiva Ouro/XP fora de escopo por ausência de primitiva) e que nenhuma pendência de clarificação resta (FR-002, FR-003, FR-004, SC-002, SC-003, SC-004). Bloqueado por: T001.

## Phase 2: Foundational

- [ ] T003 Acrescentar `coleta_eficiente` como novo token (whitelist com 16 tokens) em `ACOES_PERMITIDAS` e manter `acao_exige_destino("coleta_eficiente") == true` em `crates/sim/src/lib.rs`, preservando `false` apenas para as duas ações sem destino; atualizar a aridade tratada em `crates/servidor/tests/cli.rs` sem parser local nem whitelist paralela (FR-001, SC-001). Bloqueado por: T002.
- [ ] T004 Definir em `crates/sim/src/lib.rs` as constantes `[HIPÓTESE]` (`CUSTO_COLETA_EFICIENTE_FOLEGO = 15`, `BUFF_COLETA_EFICIENTE_TICKS = 150`, `RECARGA_COLETA_EFICIENTE_TICKS = 300`, meta de conclusão 90 sob buff contra `TICKS_COLETA = 180`), os campos de buff/recarga em `Jogador`/`Mundo::new()` e a cobertura correspondente em `serializar_jogadores`/`hash_estado` (FR-002, FR-003, SC-002, SC-003). Bloqueado por: T003.
- [ ] T005 Decrementar em `crates/sim/src/lib.rs` buff e recarga de `coleta_eficiente` exatamente uma vez por tick em `avancar_tick`, em ordem estável de `peer_id`, reaproveitando o padrão das recargas existentes e sem golden literals na sim (FR-003, SC-003). Bloqueado por: T004.

## Phase 3: User Stories

- [ ] T006 Validar em `crates/sim/src/lib.rs` (`ValidacaoComandos::validar`) que o destino é exatamente a posição do próprio jogador, que o Fôlego é ≥ 15, que a recarga está zerada e que nenhum buff de coleta está ativo (sem *stacking*), com recusa tipada em `CodigoRecusa` que não altera `seq`, `request_id`, Fôlego, progresso de coleta nem hash (FR-002, SC-002). Bloqueado por: T004.
- [ ] T007 Implementar em `crates/sim/src/lib.rs` (`fase_skills`) o ramo aceito de `coleta_eficiente`: custo 15 de Fôlego, 150 ticks de buff e 300 de recarga no agressor, conclusão da coleta em 90 ticks sob buff contra 180 sem buff com a mesma recompensa, monotônicos avançando somente no aceito e `ultima_recusa` preservada na recusa (FR-003, SC-003). Bloqueado por: T005, T006.
- [ ] T008 Despachar em `crates/cliente/src/main.rs` a tecla protótipo `KeyCode::KeyK` para `coleta_eficiente` com destino igual à posição local e IDs monotônicos, validando antes de `agendar_comando` e preservando as demais teclas, e acrescentar em `texto_hud` uma linha de estado (buff, recarga, Fôlego) lida da sim sem regra paralela (FR-004, SC-004). Bloqueado por: T006.
- [ ] T009 Escrever em `crates/sim/tests/simulacao.rs` a regressão determinística (aceite, recusa, Fôlego −15, buff 150, recarga 300, 90 vs 180 ticks, sem acumulação, monotônicos só no aceite e hash divergente/estável, sem golden literals, sem reescrever testes anteriores e com `const` antes do primeiro statement de cada função de teste) e em `crates/servidor/tests/cli.rs` a cobertura de aridade/whitelist e do formato `<tick>\t<sha256>` (FR-001, FR-002, FR-003, SC-001, SC-002, SC-003). Bloqueado por: T007.

## Phase 4: Polish

- [ ] T010 Registrar evidência, validação e release do corte em `specs/062-coleta-eficiente/plan.md`: roteiro com `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo check --locked -p insurgencia-cliente --all-targets` e `./verify.sh` (jobs `gates`/`rust` no GitHub Actions), R4 antes de R7 `1.0.0-local`, BF-002k fechado, demais backlog abertos e R6 N/A, com toda execução **[NV]** até o orquestrador/SWE (FR-004, SC-004). Bloqueado por: T008, T009.
