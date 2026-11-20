# Release (R7) — 008-nucleo-contrato

**Data:** 2026-11-20T02:02Z · **Versão:** `1.0.0` · **R4:** aprovado em 2026-11-20T02:02Z · **Estado:** pronto para PR (sem merge automático)

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `nucleo/contratos/protocolo.gd` — `Protocolo` versionado, `TipoPacote`, `normalizar_pacote`, `SnapshotMeta` (FR-003, T002).
- `nucleo/contratos/carregador_modulo.gd` — validação de 11 campos, 10 obrigatórios, tipos, hash e limites (FR-004, T003).
- `modulos/exemplo/{module.json,personagens.json,regras.cfg}` — Módulo declarativo de exemplo (FR-005, T004/T005).
- `tests/test_contrato.gd` — 16 verificações headless, `OK test_contrato`, exit 0 (FR-006, T006).
- `project.godot` — config do Núcleo 4.7 sem `run/main_scene`; diff do arquivo anterior registrado em `specs/008-nucleo-contrato/project-godot-diff.md` (FR-002, T001).

### Changed
- `.specify/feature.json` — âncora passa a `specs/008-nucleo-contrato` (via `create-new-feature.sh`).

## Rastreabilidade
FR-001..FR-007 · T001..T008 · SC-001..SC-006 · R4 `validate.md` aprovado.

## Rollback
`git revert <commit do PR>` na ordem inversa; restaura `project.godot` ao conteúdo documentado no diff registrado e remove os paths criados. Nenhuma baseline, gate, ADR, remote ou trabalho de outras sessões é afetado.

## Não incluído
Servidor, netcode, Ilha Meio, manifesto, `Makefile`, `evidence.csv`, deploy, remote. 25 dos 39 caminhos canônicos permanecem pendentes das Tasks 2–6.
