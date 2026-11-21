# Analyze — 028-ci-rust-governanca (READ-ONLY)

**Data**: 2026-11-21T04:28Z · **Escopo**: somente leitura; nenhuma edição nesta etapa.

## Conflitos artefato ↔ realidade (todos confirmados por comando)
| ID | Artefato | Conflito | Ação na fonte |
|---|---|---|---|
| C1 | `.github/workflows/ci-gates.yml` | job `gates` roda só 5 gates Python; nenhum cargo | T005: adicionar job `rust` |
| C2 | `AGENTS.md:5,28,47,66,81` | nega código/build/verify.sh/testes/Rust | T006: reescrever |
| C3 | `verify.sh:2` | diz "casv-rust" | T007: corrigir |
| C4 | `docs/GDD - Insurgência.md:1627` | CI remoto DEFER `[NV]` | T008: nota datada |
| C5 | `AGENTS.md` | regra da janela ausente embora STATE:22 diga que está lá | T006: documentar |

## Conflitos entre artefatos SDD
Nenhum: spec/plan/checklist/tasks/decision coerentes (FR-001..FR-008 ↔ T001..T011).

## Risco mitigado no código
`verify.sh:79-88` — se `cargo nextest --version` falhar, o script faz fallback para `cargo test --locked --workspace`. Portanto o job CI **não exige** action de terceiros para nextest; o script é a fonte de verdade e o fallback já é auditável. Decisão registrada: não instalar nextest via action; toolchain lida de `rust-toolchain.toml` (1.99.1 + rustfmt/clippy).

## Resultado
Analyze PASS — sem conflitos não resolvíveis. Correções C1–C5 aplicadas na implementação; reexecute gates ao final.
