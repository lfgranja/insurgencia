# Release (R7) — 061-vandalismo-agitador

**Data**: 2026-11-25 · **Versão**: 1.0.0-local (pré-release local, não promovida a `1.0.0`) · **R4**: aprovado em `.specify/validations/vandalismo-agitador/validate.md`, **antes desta release** (R4 precede R7, constituição v1.2.2) · **Branch**: `feature/061-vandalismo` · **Âncora**: `origin/dev = 52138c3` · **Score**: 288/3125 (`4×4×2×3×3`) · **R1**: `go`.

**Aviso de valor**: custo 20 de Fôlego, alcance 2 m (`ALCANCE_CONTATO_M`), dano 120, silêncio 45 ticks (1,5 s × 30 Hz), recarga 210 ticks (7 s) e tecla protótipo `H` são **`[HIPÓTESE]`/protótipo** da decisão 80/20 de 2026-11-25. Nenhum número é regra canônica; o GDD é lido, nunca reescrito. **BF-002c (Ato de Vandalismo) fecha neste corte**; **BF-015b** (`sinalizar_alavanca`) permanece aberto. **R6 N/A**: fora da sequência executável da constituição v1.2.2 — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado.

## Changelog

### [1.0.0-local] — 2026-11-25

### Added

- **FR-001 / T001, T003, T009 — contrato local**: `vandalismo` entra como 15º token de `ACOES_PERMITIDAS` em `crates/sim/src/lib.rs`, com `acao_exige_destino("vandalismo") == true`; apenas `sinalizar_sincronia` e `sinalizar_alavanca` permanecem sem destino (15 tokens, 13 com destino). A CLI `insurgencia-servidor` aceita a linha de 6 tokens e preserva `<tick>\t<sha256>` antes de cada `avancar_tick`, sem parser local nem whitelist paralela; a cobertura de aridade está em `crates/servidor/tests/cli.rs`.
- **FR-002 / T002, T004, T006, T009 — validação determinística**: `ValidacaoComandos::validar` exige alvo no destino, agressor a no máximo 2 m, Fôlego ≥ 20 e recarga zerada (210 ticks após o uso), com recusa tipada em `CodigoRecusa`. Toda recusa preserva `seq`, `request_id`, Fôlego, recarga e o estado do alvo; `ultima_recusa` participa do hash. Constantes `[HIPÓTESE]` documentadas e marcas em `Jogador`/`Mundo::new`.
- **FR-003 / T004, T005, T007, T009 — efeito e determinismo**: o ramo de `vandalismo` em `fase_skills` aplica **120** de dano pela rota única `aplicar_dano_a_jogador` (absorção de escudo e piso zero), impõe **45** ticks de silêncio e cancela a canalização do alvo na mesma passagem; consome **20** de Fôlego e marca **210** ticks de recarga no agressor, com monotônicos avançando somente no aceite. Decrescentes de silêncio/recarga ocorrem exatamente uma vez por tick em ordem estável de `peer_id`, e `hash_estado` cobre o novo estado. A regressão em `crates/sim/tests/simulacao.rs` afirma divergência da baseline e igualdade entre replays equivalentes, **sem golden hashes fixos**.
- **FR-004 / T002, T008, T010 — consumidor, HUD e evidência**: `crates/cliente/src/main.rs` despacha `KeyCode::KeyH` para `vandalismo` com destino e IDs monotônicos, validando antes de `agendar_comando`; `KeyCode::KeyQ`/`atacar_forte` permanece intacto e a linha acrescentada em `texto_hud` lê somente o estado autoritativo da sim, sem segunda verdade. Evidência em `.specify/validations/vandalismo-agitador/`: roteiro de 8 passos (`test-roadmap.md`), saídas coladas (`evidence.md`) e R4 aprovado (`validate.md`) antes deste R7.
- **Provas locais observadas [EV]**: `cargo test --offline --locked -p insurgencia-sim --test simulacao` → **112 passed**; `cargo test --offline --locked -p insurgencia-servidor --test cli` → **11 passed**; `rustfmt --check` nos 4 `.rs` → **FMT_OK**; gates `DAG PASS`, `task-id PASS`, `converge PASS`, `hub_docs PASS`. `check_sdd_completeness` falhou apenas enquanto `validate`/`release` não existiam; reexecução **[NV]**. `cargo check -p insurgencia-cliente --all-targets`, `cargo test -p insurgencia-cliente --lib`, `./verify.sh` e jobs remotos `gates`/`rust`: **[NV]** — nenhum verde afirmado para eles.

### Changed

- Nenhuma mudança em regras canônicas, GDD, `AGENTS.md`, `.specify/memory/STATE.md`, `.specify/queue/concurrency.jsonl`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, CI ou artefatos de outros ciclos. Staging explícito por caminho; nenhum `git add -A`/`git add .`, commit, push, PR, merge ou deploy executado nesta release.

## Rastreabilidade

| FR | SC | T | Evidência |
|:--|:--|:--|:--|
| FR-001 — token 15º, com destino, CLI 6 tokens | SC-001 | T001, T003, T009 | CLI `11 passed`; `test-roadmap.md` passo 1 |
| FR-002 — alvo, 2 m, 20 Fôlego, 210 ticks | SC-002 | T002, T004, T006, T009 | Sim `112 passed`; passo 2 |
| FR-003 — 120 dano, 45 silêncio, cancelamento, custo/recarga, hash | SC-003 | T004, T005, T007, T009 | Sim `112 passed` + zero golden hashes; passos 3–6 |
| FR-004 — tecla `H`, HUD derivada da sim, provas | SC-004 | T002, T008, T010 | Quatro gates PASS + `FMT_OK`; passos 7–8 (`verify.sh` remoto **[NV]**) |

Cadeia SDD: `00 constituição` → intake/research/define/shape/decide → `06 specify` → `07 clarify` (R3 = 1, respondida) → `08 plan` → `09 checklist` → `10 tasks` → `11 analyze` (READ-ONLY) → `12 implement` → `13 converge` (append-only) → `27 validate` (R4, aprovado) → `26 release` (R7, `1.0.0-local`). R6 não executada. Identidade por `.specify/feature.json → specs/061-vandalismo-agitador`.

## Rollback

Para reverter o ciclo 061, **reverter o único commit de feature** (`git revert <sha-do-commit-da-feature>`), devolvendo HEAD a `origin/dev = 52138c3` e restaurando os testes e gates do baseline. Isso remove o token `vandalismo`, as constantes e marcas, os decrescentes, o ramo de `fase_skills`, a cobertura de `hash_estado`, a tecla `H`, a linha da HUD e as regressões — **sem apagar histórico**: não usar `git clean`, `git reset --hard`, force-push, remoção forçada de branches ou reescrita de commits. Artefatos do ciclo não são removidos à força. Não há merge `dev → main` nem deploy neste ciclo; PR #76 permanece intocável (outro owner). Rollback documentado e **não executado** por este agente.

## Não incluído

- **GDD e regras canônicas**: nenhum número `[HIPÓTESE]` é promovido; nenhuma regra nova de dano, expulsão ou canalização é inventada.
- **CI**: nenhuma edição em `.github/workflows/ci-gates.yml`, gates, `verify.sh` ou configuração; jobs remotos `gates`/`rust` **[NV]**.
- **Rede**: nenhum contrato de rede, `PROTOCOL_VERSION`, Lightyear, socket, async, matchmaking ou multiplayer real.
- **5v5 e 12 ilhas**: nenhuma topologia, rota, spawning ou mapa completo.
- **R6/`taskstoissues`**: N/A/bloqueado sob v1.2.2; nenhuma issue real ou simulada, nenhum `gh` improvisado.
- **Promoção `dev → main` e deploy**: versão `1.0.0-local` restrita a este worktree; nenhum push, PR (inclusive PR #76), merge, publicação, tag ou deploy autorizado.
- Também fora: `sinalizar_alavanca` (BF-015b, permanece aberto), captura visual da HUD, builds locais adicionais e reexecução de gates/testes por este agente.
