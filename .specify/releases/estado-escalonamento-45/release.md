# Release (R7) — 037-estado-escalonamento-45

**Data**: 2026-11-22T23:01Z · **Versão**: `1.0.0` (SemVer) · **R4**: aprovado em `.specify/validations/estado-escalonamento-45/validate.md` (antes desta release)

## Changelog
### [1.0.0] — 2026-11-22
### Added
- FR-001 / T002, T003, T018 — estado verificado registrado: HEAD `efb4d4b`, `origin/dev=efb4d4b`, `origin/main=c126b6e`, 0 PRs para dev; PR #45 `dev → main` OPEN/green/ACR sem blockers (comentário `7086327685`) na fila SWE, com decisão explícita de **não mesclar `main`** (escalonar ≠ promover).
- FR-002 / T003, T015 — PR #44 mesclado em dev às 21:44:38Z (`efb4d4b`) registrado como encerramento do ciclo 036 (Noto Sans OFL 1.1, blocker P1 `BEVY_ASSET_ROOT` corrigido em `01ff4d0`, captura final sem env, evidência 11/11).
- FR-003 / T003, T014 — gates observados (`verify.sh` 41/41, 5 Python PASS) e contagens (36/238/36/36/0/13) citados com `find` somente leitura e ressalva de snapshot; divergência registrada conforme Princípio VI.
- FR-004 / T003, T011 — decisão de governança: constituição v1.0.0 do alvo prevalece (Bloco 0.a) sobre prompt v4.6/v1.2.0; divergência é **decisão, não dívida**; R6 não adotado; `taskstoissues`/`resolve-tasks.sh` não usados.
- FR-005 / T002, T005 — 80/20 documentado: documentação/escalonamento 3×5×5×5 = 375/625 escolhido sobre paridade `construir_barreira` 60/625.
- FR-006 / T009, T012, T013 — ordem normativa do alvo: `analyze.md` READ-ONLY antes de `tasks.md`, R4 antes de R7 `1.0.0`, R1 `go`, checklist revisado.
- FR-007 / T015 — 14 artefatos na fronteira exata, `converge.md` novo e append-only; nenhum `converge.md` existente editado; nada fora da fronteira tocado.
- FR-008 / T014 — `test-roadmap.md` escrito antes do PR com 10 passos, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback, passo de notas vs HEAD/contagens e passo de não-adoção de R6.
- FR-009 / T016, T017 — não-execução: nenhum build/gate/formatador/teste/commit/push/PR; PR #45 não mesclado; factory `b1deab1` somente fonte; pendências **[NV]**.

## Rastreabilidade
FR-001..FR-009 · SC-001..SC-006 · T001..T018 · R1 `go` (`.specify/assessments/estado-escalonamento-45/decision.md`, scorecard 6/6 strong, 80/20 = 375/625) · R4 (`.specify/validations/estado-escalonamento-45/validate.md`) · roteiro (`.specify/validations/estado-escalonamento-45/test-roadmap.md`). Âncoras: HEAD `efb4d4b` / `origin/main c126b6e`, PR #44 (21:44:38Z) e #45 (fila SWE), `.specify/memory/constitution.md` v1.0.0, specs/036 e PR #40 (`c126b6e`).

## Rollback
Para reverter o ciclo 037: excluir os 5 artefatos de `.specify/assessments/estado-escalonamento-45/`, os 6 artefatos de `specs/037-estado-escalonamento-45/`, `validate.md`, `test-roadmap.md` e `release.md`, e restaurar o stub original de `specs/037-estado-escalonamento-45/spec.md`. Se o baseline append-only tiver sido atualizado, remover a entrada 037. **Nunca executar neste ciclo — apenas documentar.** Não reverter PR #44/#40, o histórico do ciclo 036, `docs/GDD*`, baselines, `STATE.md`, `PROGRESSO.md`, `feature.json`, ledger, CI, paths untracked alheios, e não mesclar/fechar PR #45 — decisão do SWE.

## Não incluído
Promoção `dev → main`/merge do PR #45, paridade `construir_barreira`, adoção de R6 ou reordenação do fluxo, `taskstoissues`/`resolve-tasks.sh`, edição de `converge.md` existente, mudanças em código/`Cargo.toml`/`Cargo.lock`, `docs/GDD*`, baselines, ledger, CI, factory `b1deab1` (somente fonte), paths untracked alheios, janelas, credenciais, deploy, assinatura GPG, commits/push/PRs deste agente e reexecução de `./verify.sh`, gates ou capturas — todos **[NV]** até o orquestrador/SWE.
