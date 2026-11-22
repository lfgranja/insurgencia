# Release (R7) — 035-advisories-acr-rastreabilidade

**Data:** 2026-11-22 · **Versão:** `1.0.0` (SemVer) · **R4:** aprovado em `.specify/validations/advisories-acr-rastreabilidade/validate.md` (antes desta release)

## Changelog
### [1.0.0] — 2026-11-22
### Added
- FR-001 / T002–T006 — fechamento rastreável dos 10 advisories de ACR dos PRs #39/#40/#41 (comentários `7082836571`, `7082907820`, `7082916777`), sem blockers, sem redefinir `conferir`/011.
- FR-002 / T007 — especificação da correção de `STATE.md` para 34 specs / 237 assessments / 34 validations / 34 releases / 0 bugs / 13 research e âncora coerente com `feature.json`.
- FR-003 / T007 — especificação da correção de `evidence.md` para cinco execuções/capturas e converge baselinado `17/17`.
- FR-004 / T007 — especificação da correção da afirmação de `.gitignore` nos artefatos 034: `.gitignore:5-6` contém `/.specify/queue/`; regra legítima preservada.
- FR-005 / T007, T010, T012 — especificação da saída canônica do CLI: imprimir o hash minúsculo calculado (`hash_regras`) em `COMPAT_OK`, aceitando `content_hash` maiúsculo via `eq_ignore_ascii_case`; ordem e exit codes 0/1/2 preservados, `crates/contratos` intacto.
- FR-006 / T001–T017 — 14 artefatos SDD na ordem normativa, `analyze.md` READ-ONLY **antes** de `tasks.md`, `converge.md` append-only, R4 aprovado antes de R7, changelog FR/T e rollback documentado.
- FR-007 / T014 — `test-roadmap.md` escrito antes do PR com 10 passos, capturas de terminal/JSON, coluna `automatizável?`, rollback e passo explícito de `content_hash` maiúsculo → hash minúsculo canônico.
- FR-008 / T018 — fronteira registrada: nenhum build/gate/commit/push/PR, nenhum merge em `main` (PR #40 na fila SWE), factory `b1deab1` somente fonte, 194 paths untracked alheios preservados.
- Evidências coladas: `./verify.sh` `ALL STAGES OK` / 41 de 41 / `VERIFY_EXIT=0` e 5 gates em `2026-11-22T18:32:17Z`; PR #40 `OPEN` com `gates`/`rust` pass. Roteiro, correções na fonte e gates pós-correção permanecem **[NV]** até execução pelo orquestrador.

## Rastreabilidade
FR-001..FR-008 · SC-001..SC-005 · T001..T018 · R1 go (`.specify/assessments/advisories-acr-rastreabilidade/decision.md`, 6/6 strong, 80/20 = 400/625 = 0,64) · R4 (`.specify/validations/advisories-acr-rastreabilidade/validate.md`). Âncoras: PRs #39/#40/#41, artefatos 034, `specs/011-compatibilidade-manifesto/spec.md`, `docs/GDD - Insurgência.md` §1/§14.2. Roteiro: `.specify/validations/advisories-acr-rastreabilidade/test-roadmap.md`. Convergência: `specs/035-advisories-acr-rastreabilidade/converge.md` (append-only).

## Rollback

Para reverter o ciclo 035, remover os 5 artefatos de `.specify/assessments/advisories-acr-rastreabilidade/`, os 6 artefatos de `specs/035-advisories-acr-rastreabilidade/`, `validate.md`, `release.md` e `test-roadmap.md`, e restaurar o stub original de `specs/035-advisories-acr-rastreabilidade/spec.md`. Se o orquestrador já tiver aplicado as quatro correções na fonte (T018), reverter `STATE.md`, `evidence.md`, a afirmação de `.gitignore` em 034 e a linha de saída em `verificar_manifesto.rs` ao estado anterior, e remover a entrada correspondente do baseline de converge. **Nunca executar neste ciclo — apenas documentar.** Não remover a regra `/.specify/queue/` do `.gitignore`, não mesclar PR #40 em `main`, não executar `git clean`, `git reset`, force-push nem tocar em factory `b1deab1`, `docs/GDD*`, ledger, CI, baselines pré-existentes ou 194 paths untracked alheios.

## Não incluído
Redefinir `conferir`/011; remover regra de `.gitignore`; instalar Godot ou paridade Rust↔Godot (autorização externa; 80/625); mesclar PR #40 em `main`; factory `b1deab1` (somente fonte), `docs/GDD*`, ledger, CI, baselines pré-existentes, 194 paths untracked alheios, janelas, credenciais, deploy/remote, builds/gates/commits/push/PRs e reexecução de `./verify.sh` ou do roteiro. O que não foi executado permanece **[NV]**.
