# Release (R7) — 062-coleta-eficiente

**Data**: 2026-11-25 · **Versão**: 1.0.0-local (SemVer, pré-release local) · **R4**: aprovado (condicionado) em `.specify/validations/coleta-eficiente/validate.md`, antes desta release · **Branch**: `feature/062-coleta-eficiente` · **Âncora**: `origin/dev = d3be560`.

## Changelog

### [1.0.0-local] — 2026-11-25

### Added
- FR-001 / T001, T003, T009 — `coleta_eficiente` acrescentado a `ACOES_PERMITIDAS` (`crates/sim/src/lib.rs`): whitelist com **16** tokens, **14** com destino, `acao_exige_destino("coleta_eficiente") == true` e `false` apenas para `sinalizar_sincronia`/`sinalizar_alavanca`; a CLI `insurgencia-servidor` aceita a linha de 6 tokens preservando `<tick>\t<sha256>`, sem parser ou whitelist paralela em `crates/servidor/tests/cli.rs`.
- FR-002 / T004, T006, T009 — validação autoritativa em `ValidacaoComandos::validar`: destino exatamente igual à posição do próprio jogador, Fôlego ≥ 15, recarga zerada e nenhum buff ativo; recusa tipada em `CodigoRecusa` preserva Fôlego, `seq`/`request_id`, progresso de coleta, `ultima_recusa` e hash; reaplicação não acumula (*stacking*). Constantes e campos de buff/recarga cobertos em `serializar_jogadores`.
- FR-003 / T005, T007, T009 — efeito em `fase_skills`: custo **15 Fôlego**, buff **150 ticks**, recarga **300 ticks**, conclusão da coleta em **90 ticks** contra **180** (`TICKS_COLETA`) com a **mesma recompensa**; monotônicos avançam só no aceite; decrescentes de buff/recarga ocorrem exatamente uma vez por tick em ordem estável de `peer_id`; novos campos entram em `hash_estado`. Todos os números são **[HIPÓTESE]** de `GDD:646`, lido e não reescrito.
- FR-004 / T002, T008, T009, T010 — consumidor no cliente sem UI: tecla protótipo `KeyCode::KeyK` despacha `coleta_eficiente` com destino na posição local e IDs monotônicos, validando antes de `agendar_comando`; `texto_hud` exibe linha de estado (buff, recarga, Fôlego) lida do espelho autoritativo da sim, sem segunda verdade.
- BF-002k / T010 — `Coleta Eficiente` fecha neste corte; a passiva de Ouro/XP permanece fora de escopo por ausência de primitiva no núcleo; os demais itens do backlog permanecem abertos, **R6 N/A** (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado).
- T001–T010 — rastreabilidade concluída: identidade em `.specify/feature.json`, DoD-1..DoD-4 e rótulos **[HIPÓTESE]** na spec, `test-roadmap.md` com 8 passos CLI/terminal escrito antes do PR, validação R4 condicionada e esta release R7 `1.0.0-local`, na ordem da constituição v1.2.2.
- Execução pendente — nenhum build, teste, gate, formatador, captura, commit, push, PR, merge ou deploy foi executado por este agente; contagens, `./verify.sh`, jobs `gates`/`rust` e capturas permanecem **[NV]** até o orquestrador/SWE colar as saídas em `evidence.md`.

## Rastreabilidade

FR-001..FR-004 · SC-001..SC-004 · T001..T010 (`specs/062-coleta-eficiente/tasks.md`) · BF-002k · R1 `go` (score 1024/3125, `4×4×4×4×4`, decisão 80/20 de 2026-11-25 em `.specify/memory/STATE.md`) · roteiro (`.specify/validations/coleta-eficiente/test-roadmap.md`, passos 1–8) · R4 (`.specify/validations/coleta-eficiente/validate.md`, aprovado **condicionado** à verificação final) · R7 `1.0.0-local` (este arquivo). Âncoras: `origin/dev = d3be560`, branch `feature/062-coleta-eficiente`, worktree `.specify/queue/worktrees/cycle-062-coleta-eficiente/`, `.specify/feature.json → specs/062-coleta-eficiente`, constituição v1.2.2 e `GDD:646` (somente leitura). Verificação final: `cargo test --offline --locked -p insurgencia-sim --test simulacao`, `cargo test --offline --locked -p insurgencia-servidor --test cli`, `cargo check --locked -p insurgencia-cliente --all-targets`, `./verify.sh` e jobs remotos `gates`/`rust` — todos **[NV]** até evidência colada.

## Rollback

Para reverter o corte **sem apagar histórico**: aplicar **`git revert <hash do commit da feature>`** no head da branch — o commit cujo pai é a âncora `d3be560` — preservando todos os commits anteriores e o rastro da tentativa. Remover da fronteira os quatro artefatos deste ciclo somente se o revert não os tiver removido; se o baseline append-only de `converge.md` tiver recebido entrada 062, registrar a reversão como nova entrada, nunca reescrever a anterior. **Nunca executar neste ciclo — apenas documentar.** Não usar `git reset --hard`, `git clean`, force-push, reescrita de história, remoção de artefatos alheios, merge/PR/deploy, nem reverter PR #76 (`dev→main`, verde mas `CONFLICTING`, de outro dono), ciclos anteriores, `docs/GDD*`, baselines ou ledger.

## Não incluído

GDD e qualquer regra canônica (números de `GDD:646` não são promovidos); CI e configuração de gates/workflow; contrato de rede, `PROTOCOL_VERSION`, Lightyear ou qualquer forma de multiplayer real/5v5; mapa de 12 ilhas, topologia, rotas ou spawning; passiva de Ouro/XP (sem primitiva no núcleo); **R6/`taskstoissues`** — N/A sob a constituição v1.2.2, PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado; promoção `dev→main`, PR #76, merge, auto-merge, force-push, assinatura, deploy, commits, push, PRs, janelas, credenciais e execução de testes/gates/formatadores por este agente — todos **[NV]** até o orquestrador/SWE.

## Adendo [EV] 2026-11-25T07:27:56Z — revalidação local

Contagens finais: **121 testes da sim** e **12 da CLI**; `clippy -D warnings` limpo com `#[expect(clippy::too_many_lines, reason = "16 ações em ordem fixa no tick")]` escopado em `Mundo::fase_skills`; 5/5 gates Python PASS. R4 aprovado após reexecução local. CI remoto e `verify.sh` completo permanecem **[NV]** até o PR. R7 `1.0.0-local` mantido; nenhum número promovido a cânone e nenhuma primitiva de Ouro/XP introduzida.

