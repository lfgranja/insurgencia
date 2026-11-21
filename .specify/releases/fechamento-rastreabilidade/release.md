# Release (R7) — 031-fechamento-rastreabilidade

**Data:** 2026-11-21T13:23Z · **Versão:** `1.0.0` (SemVer) · **R4:** aprovado

## Changelog
### [1.0.0] — 2026-11-21
### Added
- FR-001 / T005, T006 — rastreabilidade herdada fechada: 029 T004–T008 (PR #35, `970af4e`, CI `44869231266`, STATE 04:33Z) e 028 T011 (PR #34, `d2b23e4`, CI `44868073016`) marcados `[x]` com nota datada `2026-11-21T13:23Z`; Analyze existente preservada.
- FR-002 / T007 — errata de redação nos 8 arquivos designados: "`verify.sh --all-targets`" → "`./verify.sh` (que executa `--all-targets` internamente)", conforme `grep verify.sh:5-15,70,75` (13:19Z).
- FR-003 / T008 — entradas datadas acrescentadas em `specs/030-demo-deterministica-headless/converge.md` e `docs/PROGRESSO.md`, sem editar linhas antigas.
- FR-004 / T009 — `AGENTS.md`: Lightyear "declarado em ADR-004; ausente de `Cargo.toml`/`Cargo.lock` (verificado 21/11)"; restante do arquivo intacto.
- FR-005 / T010 — evidências datadas registradas: `./verify.sh` 13:21Z (`ALL STAGES OK`, 34 testes, `VERIFY_EXIT=0`, 4 estágios), 5 gates 13:18:27Z, `gh` 13:19:29Z (0 PRs, 5 runs success, `dev` protegido, `dev`=HEAD=`3709c56`), factory N/A (Bloco 0.a).
- FR-006 / T011, T012 — SDD ordenado R1 go → R4 → R7; `converge.md` append-only; release `1.0.0` com este changelog e rollback documentado; 13 artefatos escritos nas fronteiras A/B.

## Rastreabilidade
FR-001..FR-006 · T001..T012 · SC-001..SC-006 · R1 go (`.specify/assessments/fechamento-rastreabilidade/decision.md`) · R4 `.specify/validations/fechamento-rastreabilidade/validate.md`.

## Rollback
`git revert <merge commit do PR>` restaura os artefatos ao estado anterior ao ciclo 031; como as alterações são exclusivamente documentais e os appends são entradas autocontidas, basta remover as notas datadas `2026-11-21T13:23Z` e as entradas acrescentadas. Nenhuma baseline, gate, contrato, simulação, CI, constituição ou trabalho alheio é afetado. Não houve build, gate ou commit neste ciclo.

## Não incluído
Factory (fonte HEAD `b1deab1`, nada transplantado), `sb-*`, `docs/OBJETIVO.md`, `docs/ROADMAP.md`, `docs/research/index.md`, janelas do cliente, credenciais, abertura de PRs e qualquer reexecução de gates além das evidências já coladas.
