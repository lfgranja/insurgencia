# Release (R7) — 018-iam-adapter-local

**Data:** 2026-11-20T11:35Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `nucleo/iam/iam_interface.gd` — porta com os 4 métodos imutáveis do contrato e retorno neutro seguro (FR-001).
- `nucleo/iam/iam_adapter_local.gd` — adaptador determinístico em processo, sem servidor, segredo, rede ou credencial persistida; 9 códigos legíveis e tabela local de 6 permissões (FR-002..FR-006).
- `tests/test_iam_adapter_local.gd` — 36 verificações de sucesso, recusa de permissão, sessão inválida, revogação, determinismo e ausência de segredos (FR-007).

### Fixed
- Herança e anotações por `class_name` substituídas por `preload`/caminho literal; `\t` literal corrigido para TAB.

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-005 · R4 aprovado · ADR-003.

## Rollback
`git revert <merge commit do PR>`; remove `nucleo/iam/` e o teste, restaura `feature.json` para `specs/017-rede-e2e`. ADR-003 e o contrato de arquitetura permanecem — a implementação é reversível sem reabrir a decisão. Nenhuma baseline, gate ou trabalho alheio é afetado.

## Não incluído
Serviço IAM remoto, login obrigatório, chaves do jogador, federação, 5v5, lobby, deploy/remote.
