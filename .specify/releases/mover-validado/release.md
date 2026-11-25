# Release (R7) — 059-mover-validado

**Data:** 2026-11-24 · **Versão:** `1.0.0-local` (SemVer, pré-release **local**, não promovida a `1.0.0`) · **R4:** **APROVADO CONDICIONADO** em `.specify/validations/mover-validado/validate.md`, **antes desta release** — condicionado ao recheck local do cliente/clippy e aos jobs remotos `gates`/`rust` green · **Âncora:** `origin/dev = 737b876`, branch `feature/059-mover-validado`, constituição v1.2.2.

**Aviso de valor:** `PIXELS_POR_METRO = 100.0`, `VELOCIDADE = 260.0` px/s, 2,6 m/s e ≈ 0,087 m/tick são **`[HIPÓTESE]`/protótipo**, nunca cânone; `LIMITE_MOVIMENTO_M = 1.0` e o validador da sim permanecem a autoridade única, e o GDD não é reescrito. **BF-012 (`mover` validado no cliente) é fechado neste ciclo; BF-003b (mapa de 12 ilhas) e BF-013 (`contratos` consumido por binários) permanecem abertos.** Score **144/3125** (`4×3×3×2×2`). Nenhuma implementação, build, gate, teste, formatador, commit, push, PR, merge, deploy, issue ou promoção foi executado por este agente de documentação.

## Changelog

### [1.0.0-local] — 2026-11-24

> Convenção de IDs: **T001..T010** é a cadeia SDD canônica de `specs/059-mover-validado/tasks.md` (T010 agrupa implement → converge → validate → release); rótulos preliminares de assessment não renomeiam esta cadeia.

### Added

- **FR-001 / T002, T004, T005, T007, T010** — conversão explícita pixels↔metros no cliente: `pub const PIXELS_POR_METRO: f64 = 100.0` com doc `[HIPÓTESE]` (`crates/cliente/src/lib.rs:23-27`) e helpers puros `pixels_para_metros` (`:203-213`) e `metros_para_pixels` (`:215-229`, com `#[expect(clippy::cast_possible_truncation, reason = ...)]` estritamente local). 260 px/s ÷ 100 = 2,6 m/s ≈ 0,087 m/tick a 30 Hz, abaixo de `LIMITE_MOVIMENTO_M = 1.0` (`crates/sim/src/lib.rs:212`); todos os destinos/alvos das ações do cliente são construídos em metros (`crates/cliente/src/main.rs:657,704,730,751`). Nenhum relógio, RNG, rede, contrato, `PROTOCOL_VERSION`, regra canônica ou GDD alterado.
- **FR-002 / T003, T005, T010** — `mover_jogador` (`crates/cliente/src/main.rs:619-680`) deixou de mutar `JogadorLocal` diretamente: bloqueia `imobilizado_ticks > 0` (`:626-633`), monta `Comando { acao: "mover", destino: Some(destino_m) }` e chama `ValidacaoComandos::validar(&simulacao.0, PEER_ID, "mover", local.seq + 1, local.request_id + 1, Some(destino_m))` **antes** de `agendar_comando` (`:658-673`). Somente no ramo de aceite a posição local avança e `ComandoLocal.seq`/`request_id` são incrementados (`:670-672`); na recusa por limite, replay ou imobilização, posição e monotônicos permanecem intactos — *no counter burn on rejection*. A decisão continua sendo do validador da sim, sem reimplementação no cliente.
- **FR-003 / T003, T004, T005, T007, T010** — sincronização autoritativa: `sincronizar_sim` (`crates/cliente/src/main.rs:1144-1149`) lê `simulacao.0.jogadores[PEER_ID].posicao` em metros e grava `jogador.0.posicao = metros_para_pixels(remoto.posicao)`; a antiga injeção de pixels no espelho e o retorno antecipado por imobilização local foram removidos. A cadeia `Update` passa a rodar `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar` (`:126-128`), com `atualizar_texto` por último, para que avatar/HUD desenhem somente o estado aceito e avançado pelo núcleo — sem segunda verdade, sem dupla aplicação e sem plugin, janela ou recurso Bevy novo.
- **FR-004 / T001, T005, T010** — evidência, validação e governança: roteiro de **14 passos** escrito antes do PR (`.specify/validations/mover-validado/test-roadmap.md`), tabela de saídas em `.specify/validations/mover-validado/evidence.md`, validação **R4 aprovada condicionada antes deste R7** (`.specify/validations/mover-validado/validate.md`), este release e `specs/059-mover-validado/converge.md` como **1ª entrada append-only** do ciclo. Provas observadas: `cargo fmt --all -- --check` **OK**, `cargo test --locked -p insurgencia-sim --test simulacao` → **`103 passed (1 suite, 0.42s)`**, `cargo test --locked -p insurgencia-servidor --test cli` → **`10 passed (1 suite, 0.20s)`**; gates `work-order-dag PASS (58 files, 632 tasks, 311 edges, 0 cycles)`, `task-id PASS (58, 0 duplicates)`, `converge PASS (40/40)`, `hub-docs PASS (0 errors)`. `check_sdd_completeness` falhava **apenas** pela ausência deste `release.md`: **este artefato resolve essa causa**, restando o recheck do gate pelo orquestrador. R6/taskstoissues é **N/A** (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado). Pendências **[NV] até recheck do orquestrador**: `cargo test --locked -p insurgencia-cliente --lib`, `cargo check --locked -p insurgencia-cliente --all-targets` e `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `./verify.sh`, jobs remotos `gates`/`rust` e captura visual do HUD também **[NV]**.

### Changed

- Nenhuma mudança em regras canônicas, GDD, baseline, contratos, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, CI, manifesto, `AGENTS.md`, stack ou rede. Nenhum valor prototípico promovido a cânone, nenhum ciclo anterior reaberto e nenhuma issue criada.

## Rastreabilidade

FR-001..FR-004 · SC-001..SC-004 (1:1) · T001..T010 (cadeia canônica: `tasks` T008 → `analyze` READ-ONLY T009 → implement/converge/validate/release T010) · R1 `go` · **R4 antes de R7** · score **144/3125** (`4×3×3×2×2`) · roteiro de 14 passos · constituição **v1.2.2** · âncoras: `crates/cliente/src/lib.rs:23-27,203-229`, `crates/cliente/src/main.rs:126-128,619-680,1144-1149`, `crates/sim/src/lib.rs:212,2414-2445`, `origin/dev = 737b876`.

## Rollback

Para reverter o ciclo 059, **reverter o único commit de feature** (`git revert <sha-do-commit-de-feature>`), devolvendo o worktree a `origin/dev = 737b876` (`737b876142396f51d33b16c39a36ee5e61d6220a`); os testes e gates do baseline voltam a valer. **Não** executar `git clean`, `git reset --hard`, force-push, remoção forçada de artefatos do ciclo, merge de PR ou deploy. Rollback está documentado, **não executado** — a execução é SWE-exclusiva.

**Nenhum merge `dev → main` e nenhum deploy foram realizados nem autorizados por esta release.** A versão `1.0.0-local` não foi promovida a `1.0.0`, não saiu deste worktree e não autoriza publicação. PR #76 pertence a outro owner e permanece intocado.

## Não incluído

Promoção `dev → main`/deploy, abertura ou merge de PR (inclusive PR #76), acionamento de R6 ou simulação de issues, reexecução de gates, testes, formatadores ou capturas por este agente, mapa completo de 12 ilhas (BF-003b), consumo de `crates/contratos` (BF-013), HUD com `hash_estado` (BF-014), promoção de `[HIPÓTESE]` a cânone, mudanças em `crates/sim`, contratos, protocolo, Cargo, CI, GDD, baseline, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md` ou código/artefatos de outros ciclos.
