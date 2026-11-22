# Release (R7) — 011-compatibilidade-manifesto

**Data:** 2026-11-20T03:18Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `manifesto.json` — 6 campos obrigatórios, hash `b2c11e6d…d746` (FR-001).
- `nucleo/compatibilidade/manifesto.gd` — `carregar`, `validar_formato` e `conferir` com ordem formato→versão→módulo→64 hex→regras→hash (FR-002/003).
- `ferramentas/verificar_compatibilidade.gd` — gate local `COMPAT_OK` / `COMPAT_FALHA` e exibição `CLIENTE_*` (FR-004).
- `Makefile` — alvo `check-compat` (FR-005).
- `tests/test_manifesto.gd` — 7 verificações (FR-006).

### Fixed
- Hash de `regras.cfg` via `HashingContext` (API `sha256_string()` do plano não existe no Godot 4.7).
- SC-004 corrigido para o comportamento real: script retorna `1`, `make` retorna `2`.

## Rastreabilidade
FR-001..FR-007 · T001..T010 · SC-001..SC-004 · R4 aprovado.

## Rollback
`git revert <merge commit do PR>`; remove os 5 arquivos novos, restaura `feature.json` para `specs/010-ilha-sincronia`. Nenhuma baseline, gate, ADR ou trabalho alheio é afetado.

## Não incluído
`evidence.csv`, `make check`, sondagem de lobby/worker, publicação de release, deploy/remote.
