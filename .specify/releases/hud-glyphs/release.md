# Release (R7) — 029-hud-glyphs

**Data:** 2026-11-21T05:02Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog
### [1.0.0] — 2026-11-21
### Fixed
- `crates/cliente/src/main.rs:458,555` — HUD com ASCII seguro (`INSURGENCIA - low-poly`, `/` no lugar de `·`), eliminando "tofu" (▯) da fonte padrão do Bevy.
- Nenhuma fonte/asset externo adicionado; lógica, testes e regra da janela preservados.

## Rastreabilidade
FR-001..FR-005 · T001..T008 · SC-001..SC-005 · imagem da SE 04:33Z · GDD §13.2.

## Rollback
`git revert <merge commit do PR>`; restaura as duas strings. Sem baseline/gate/trabalho alheio afetado.

## Não incluído
Assets/fontes externos, janelas normais por iniciativa de agente, inspeção visual **[NV]** até SE.
