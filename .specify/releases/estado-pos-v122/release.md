# Release (R7) — 038-estado-pos-v122

**Data**: 2026-11-23T01:31Z · **Versão**: `1.0.0` (SemVer) · **R4**: aprovado em `.specify/validations/estado-pos-v122/validate.md` (antes desta release)

## Changelog
### [1.0.0] — 2026-11-23
### Added
- FR-001 / T002, T003, T019 — estado verificado registrado: HEAD/`dev` `e010363`, `main` `8b0b044`, 0 PRs para dev; PR #48 `dev → main` OPEN/green/ACR sem blockers (comentário `7089052846`) na fila SWE, com decisão explícita de **não mesclar `main`** (escalonar ≠ promover).
- FR-002 / T003 — gates Python PASS e 41/41 testes observados; deploy ausente no workflow (dívida escalada e aberta); inventário 1 worktree, 0 stashes, 24 untracked preservados; reexecução **[NV]**.
- FR-003 / T003, T015 — deriva documental registrada em `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` e `STATE.md` (âncora `efb4d4b`/PR #45) **sem edição**; contagens versionadas 37/65/37/37/0/10 distinguidas do checkout compartilhado 238/13 (untracked históricos).
- FR-004 / T003, T011 — constituição **v1.2.2** do alvo aplicada: merge autônomo só em `dev`, promoção/deploy ao SWE, `tasks` antes de `analyze` (READ-ONLY), R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only.
- FR-005 / T006, T013 — **R6 suspensa por decisão ratificada**: fora do fluxo até `resolve-tasks.sh`/adaptador, PARE se invocada, nenhuma issue simulada, nenhum `taskstoissues`/`gh` improvisado; inclusão exige emenda ratificada.
- FR-006 / T002, T005 — 80/20 documentado: documentação pós-v1.2.2 3×5×5×5 = 375/625 escolhido sobre coletar materiais 5×4×3×3 = 180/625.
- FR-007 / T016 — 14 artefatos na fronteira exata; `converge.md` novo e append-only; cinco documentos com deriva e alterações preexistentes (`feature.json`, `PROGRESSO.md`) intactos.
- FR-008 / T015 — `test-roadmap.md` escrito antes do PR com 10 passos, superfície documental/CLI, capturas de terminal, `automatizável?`, rollback, passos de PR #48 green, notas vs HEAD/contagens e R6 fora do fluxo.
- FR-009 / T017, T018 — não-execução: nenhum build/gate/formatador/teste/commit/push/PR; `main` não mesclado; factory `b1deab1` (`origin/dev 05b977f`) somente fonte; pendências **[NV]**.

## Rastreabilidade
FR-001..FR-009 · SC-001..SC-006 · T001..T019 · R1 `go` (`.specify/assessments/estado-pos-v122/decision.md`, scorecard 6/6 strong, 80/20 = 375/625) · R4 (`.specify/validations/estado-pos-v122/validate.md`) · roteiro (`.specify/validations/estado-pos-v122/test-roadmap.md`, passos 2/9/10). Âncoras: HEAD/`dev` `e010363` / `main` `8b0b044`, PR #48 (fila SWE), `.specify/memory/constitution.md` v1.2.2, cinco documentos com deriva e contagens versionadas 37/65/37/37/0/10.

## Rollback
Para reverter o ciclo 038: excluir os 5 artefatos de `.specify/assessments/estado-pos-v122/`, os 6 artefatos de `specs/038-estado-pos-v122/` (incluindo o stub substituído, restaurando-o), `validate.md`, `test-roadmap.md` e `release.md`. Se o baseline append-only tiver sido atualizado, remover a entrada 038. **Nunca executar neste ciclo — apenas documentar.** Não reverter PR #48, o histórico dos ciclos anteriores, `docs/GDD*`, baselines, ledger, CI, `STATE.md`, `PROGRESSO.md`, `feature.json`, os cinco documentos com deriva, paths untracked alheios, as alterações preexistentes fora da fronteira, e não mesclar/fechar PR #48 nem executar deploy — decisão do SWE.

## Não incluído
Promoção `dev → main`/merge do PR #48; edição de `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` ou `STATE.md`; reexecução de `./verify.sh`, gates ou capturas; coleta de materiais; adoção de R6 como regra ativa, `taskstoissues`/`resolve-tasks.sh` ou simulação de issues; edição de `converge.md` existente; mudanças em código/`Cargo.toml`/`Cargo.lock`, `docs/GDD*`, baselines, ledger, CI, factory `b1deab1` (somente fonte), paths untracked alheios, janelas, credenciais, deploy, assinatura GPG, commits/push/PRs deste agente e alterações preexistentes fora da fronteira — todos **[NV]** até o orquestrador/SWE.
