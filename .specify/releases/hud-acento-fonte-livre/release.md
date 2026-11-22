# Release (R7) — 036-hud-acento-fonte-livre

**Data**: 2026-11-22T21:24Z · **Versão**: `1.0.0` (SemVer) · **R4**: aprovado em `.specify/validations/hud-acento-fonte-livre/validate.md` (antes desta release)

## Changelog
### [1.0.0] — 2026-11-22
### Added
- FR-001 / T001–T007, T018 — HUD com grafia correta: `Text::new("INSURGÊNCIA — low-poly")` (U+00CA, U+2014), superando o paliativo ASCII do ciclo 029 por decisão explícita do SWE.
- FR-002 / T007, T018 — `format!` em `atualizar_texto` com `Limites: X +/-{LIMITE_X:.0} · Y +/-{LIMITE_Y:.0}` (U+00B7), preservando posição, ilha e controles.
- FR-003 / T003, T018 — fonte livre embarcada `assets/fonts/NotoSans-Regular.ttf` (621572 bytes, SHA-256 `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823`), carregada via `AssetServer`/`Handle<Font>` e aplicada no `TextFont`; nenhuma dependência `Cargo` nova.
- FR-004 / T003, T018 — licença SIL OFL 1.1 integral em `assets/fonts/OFL.txt` e proveniência em `docs/governance/font-provenance.md` (origem, tamanho, hash, URL primária e cobertura U+00CA/U+2014/U+00B7).
- FR-005 / T005, T014 — ausência de tofu `▯` com cobertura comprovada por `fc-match` e passo explícito de verificação dos glifos na captura **[NV]**.
- FR-006 / T002, T014 — regra da janela preservada: cliente `--minimizado`, capturas em Xvfb isolado, sem janela no desktop da SE.
- FR-007 / T008–T013, T015–T017 — 14 artefatos SDD na ordem normativa, `analyze.md` READ-ONLY antes de `tasks.md`, `converge.md` append-only (1ª entrada), R4 aprovado antes de R7 `1.0.0`, rollback documentado.
- FR-008 / T014 — `test-roadmap.md` escrito antes do PR com escopo/FR/T, pré-condições, 10 passos numerados, superfície CLI/visual isolada, plano de capturas Xvfb, coluna `automatizável?`, rollback, passo dos três glifos/sem `▯` e passo de hash/licença da fonte.
- FR-009 / T016, T017 — fronteira e não-execução: nenhum build/gate/formatador/commit/push/PR; PR #40 mesclado às 20:52:35Z (`c126b6e`) registrado como fato observado; histórico 029 intacto.

## Rastreabilidade
FR-001..FR-009 · SC-001..SC-006 · T001..T018 · R1 `go` (`.specify/assessments/hud-acento-fonte-livre/decision.md`, scorecard 6/6 strong, 80/20 = 240/625) · R4 (`.specify/validations/hud-acento-fonte-livre/validate.md`) · roteiro (`.specify/validations/hud-acento-fonte-livre/test-roadmap.md`). Âncoras: `crates/cliente/src/main.rs`, `assets/fonts/`, `docs/governance/font-provenance.md`, captura `docs/evidencias/hud-confirmacao-visual-2026-11-22.png`, specs/029 e PR #40 (`c126b6e`).

## Rollback
Para reverter o ciclo 036: restaurar as duas strings ASCII em `crates/cliente/src/main.rs` (`INSURGENCIA - low-poly`, `/` nos limites), remover `fontes: Res<AssetServer>`, `Handle<Font>` e `font: fonte.into()` do `TextFont`, excluir `assets/fonts/`, `docs/governance/font-provenance.md` e `docs/evidencias/` (se criado pelo ciclo), os 5 artefatos de `.specify/assessments/hud-acento-fonte-livre/`, os 6 artefatos de `specs/036-hud-acento-fonte-livre/`, `validate.md`, `release.md` e `test-roadmap.md`, e restaurar o stub original de `specs/036-hud-acento-fonte-livre/spec.md`. Se o baseline append-only tiver sido atualizado, remover a entrada 036. **Nunca executar neste ciclo — apenas documentar.** Não reverter PR #40, o histórico do ciclo 029, `docs/GDD*`, baselines pré-existentes, `STATE.md`, `PROGRESSO.md`, `feature.json`, ledger, CI, factory `b1deab1` (somente fonte) nem paths untracked alheios. Arquivos temporários do roteiro são removidos com `rm -rf "$TMP"`.

## Não incluído
Segunda implementação da correção, mudanças em contratos de simulação/manifesto, `Cargo.toml`/`Cargo.lock`, `docs/GDD*`, baselines, `STATE.md`, `PROGRESSO.md`, `feature.json`, ledger, CI, factory `b1deab1` (somente fonte), 24 paths untracked alheios preservados, janelas normais, credenciais, deploy, assinatura GPG, commits/push/PRs deste agente e reexecução de `./verify.sh`, gates ou capturas — todos **[NV]** até o orquestrador.
