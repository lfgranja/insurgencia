# Feature Specification: Fechamento de rastreabilidade (ciclos 028–030)

**Branch**: `031-fechamento-rastreabilidade` · **Created**: 2026-11-21 · **Status**: Draft · **R1**: go

## User Scenarios
### US1 — Rastreabilidade auditável (P1)
Um auditor que lê os artefatos SDD dos ciclos 028–030 encontra o estado real do repositório, não um registro defasado.

**Acceptance Scenarios**: 1) `specs/029-hud-glyphs/tasks.md` T004–T008 e `specs/028-ci-rust-governanca/tasks.md` T011 marcados `[x]` com nota datada `2026-11-21T13:23Z`; 2) nenhuma ocorrência de "`verify.sh --all-targets`" nos arquivos designados; 3) `AGENTS.md` preciso sobre Lightyear.

### US2 — Evidências datadas coladas (P1)
Toda afirmação de "verde" ou de estado remoto vem acompanhada de comando executado e saída colada, com timestamp.

**Acceptance Scenarios**: 1) `./verify.sh` em `2026-11-21T13:21Z` com `ALL STAGES OK` e `34 tests run: 34 passed`; 2) 5 gates Python PASS em `13:18:27Z`; 3) `gh` ao vivo em `13:19:29Z` com 0 PRs abertos e 5 runs `deterministic-gates` `success`.

### US3 — Histórico preservado, SDD ordenado (P2)
Correções são acréscimos datados; nenhuma entrada antiga de `converge.md` é reescrita e nenhuma etapa do SDD é pulada.

**Acceptance Scenarios**: 1) entradas datadas `2026-11-21T13:23Z` em `specs/030-demo-deterministica-headless/converge.md` e `docs/PROGRESSO.md`; 2) R4 aprovado antes de R7; 3) release `1.0.0` com changelog FR/T e rollback.

## Requirements
- **FR-001**: marcar T004–T008 de `specs/029-hud-glyphs/tasks.md` e T011 de `specs/028-ci-rust-governanca/tasks.md` como `[x]`, acrescentando nota datada `2026-11-21T13:23Z` com PR #35 (`970af4e`, CI `44869231266`, STATE 04:33Z) e PR #34 (`d2b23e4`, CI `44868073016`); não reescrever a seção Analyze existente.
- **FR-002**: substituir "`verify.sh --all-targets`" por "`./verify.sh` (que executa `--all-targets` internamente)" em `specs/030-demo-deterministica-headless/{spec.md,plan.md,tasks.md,checklists/requirements.md}`, `.specify/validations/demo-deterministica-headless/validate.md`, `.specify/releases/demo-deterministica-headless/release.md`, `.specify/memory/STATE.md` e `docs/research/slotbound-2026-11-21-sintese.md`.
- **FR-003**: **append** (nunca editar linhas antigas) em `specs/030-demo-deterministica-headless/converge.md` e `docs/PROGRESSO.md`, com entradas datadas `2026-11-21T13:23Z` registrando a errata de redação e todas as evidências/correções aplicadas.
- **FR-004**: ajustar em `AGENTS.md` apenas a frase sobre Lightyear para "declarado em ADR-004; ausente de `Cargo.toml`/`Cargo.lock` (verificado 21/11)"; nenhuma outra alteração no arquivo.
- **FR-005**: registrar como autoridade de estado as evidências datadas: `./verify.sh` (13:21Z: stage 1 62s, stage 2 160s, 34 testes, stage 4 5s, `ALL STAGES OK`, `VERIFY_EXIT=0`, 4 estágios check→clippy→nextest→fmt); 5 gates Python (13:18:27Z: `spec_dirs=30 gaps=0`, `262 tarefas/0 ciclos`, `0 duplicatas`, converge `13/13`, hub-docs `0 erros`; `check_production_gate.py` ausente → exit 2 = N/A, Bloco 0.a); `gh` (13:19:29Z: repo público, default `main`, 0 PRs abertos, 5 runs `success`, `dev` com required check `gates`/`enforce_admins=false`/sem reviews, `main` 404, `dev` = `3709c56` = HEAD); `grep` Lightyear sem ocorrências nas manifestações.
- **FR-006**: preservar `converge.md` append-only e o SDD ordenado (R1 go → R4 → R7), com release `1.0.0` (SemVer), changelog FR-001..FR-006 + T-IDs e rollback documentado; 13 artefatos escritos nas fronteiras A/B, nenhum build/gate/commit.

## Success Criteria
SC-001: 0 tarefas herdadas pendentes sem nota datada (029 T004–T008; 028 T011). SC-002: 0 ocorrências de "`verify.sh --all-targets`" nos 8 arquivos designados. SC-003: entradas datadas `2026-11-21T13:23Z` presentes em `converge.md` (030) e `docs/PROGRESSO.md`, com histórico antigo intacto. SC-004: `AGENTS.md` declara Lightyear em ADR-004 e ausente das manifestações. SC-005: evidências datadas coladas (verify 34 testes, 5 gates, gh ao vivo). SC-006: R4 aprovado antes de R7; release `1.0.0` com changelog FR/T e rollback; nenhum build/gate/commit; fronteiras A/B respeitadas.
