# Release (R7) — 020-engines-rust

**Data:** 2026-11-20T23:53Z · **Versão:** `1.0.0` · **R4:** aprovado

## Changelog

### [1.0.0] — 2026-11-20
### Added
- `docs/AVALIACAO-ENGINES-RUST.md` — avaliação comparativa de 10 opções, 10 critérios com Σ=100%, matriz com Score e Fase 0/Fase 1, análise por critério, 32 linhas de evidência, recomendação com spikes A/B/C e limitações.
- `docs/avaliacao-engines/pesquisa/rust-{bevy,fyrox,outras,protocolo,matriz-consolidada}.md` — relatórios de pesquisa com fontes primárias e host policy.
- Artefatos SDD 020: assessments, spec, plan, checklist, tasks, analyze, converge, R4 e R7.

### Fixed
- Versões defasadas da avaliação anterior (Bevy 0.19.1 → 0.20.1; Lightyear 0.28–0.30 → 0.31).
- Divergência Fyrox 62,6 vs 65,2 resolvida em favor do relatório dedicado, com registro.
- Categoria de `godot-rust/gdext` corrigida: binding, não engine escrita em Rust.

## Rastreabilidade
FR-001..FR-008 · T001..T010 · SC-001..SC-005 · R4 aprovado · protocolo factory 019/040 · ADR-001.

## Rollback
`git revert <merge commit do PR>`; remove o documento final e os relatórios de pesquisa, restaura `feature.json` para `specs/019-tela-resultado`. Nenhuma baseline, gate, ADR ou trabalho alheio é afetado; ADR-001 permanece vinculante.

## Não incluído
Troca de motor, Rust no Núcleo, protótipo, benchmark pareado, migração de toolchain, deploy/remote.
