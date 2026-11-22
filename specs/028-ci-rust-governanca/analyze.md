# Analyze — 028-ci-rust-governanca (READ-ONLY)

**Data**: 2026-11-21T04:28Z (revisões 04:35Z e 04:42Z) · **Escopo**: somente leitura; nenhuma edição nesta etapa.

## Conflitos artefato ↔ realidade (confirmados por comando)
| ID | Artefato | Conflito | Ação na fonte |
|---|---|---|---|
| C1 | `.github/workflows/ci-gates.yml` | job `gates` roda só 5 gates Python; nenhum cargo | T005: adicionar job `rust` |
| C2/C5 | `AGENTS.md:5,28,47,66,81` | nega código/build/verify.sh/testes/Rust; regra da janela ausente | T006: reescrever |
| C3 | `verify.sh:2` | diz "casv-rust" | T007: corrigir |
| C4 | `docs/GDD - Insurgência.md:1627` | CI remoto DEFER `[NV]` | T008: nota datada |
| C6 | job `rust` (CI) | `dtolnay/rust-toolchain@master` exige input `toolchain`; CI RED 04:29Z | T005b: `toolchain: '1.99.1'` explícito |
| C8 | job `rust` (CI) | `alsa-sys` exige `alsa.pc`; runner Ubuntu sem lib → CI RED 04:45Z | T005d: ampliar step para libasound2-dev + libs X11/GL |
| C7 | job `rust` (CI) | `wayland-sys` exige `libwayland.pc`; runner Ubuntu sem lib → CI RED 04:37Z | T005c: step `Install system libraries` (pkg-config, libwayland-dev, libxkbcommon-dev, libx11-dev, libudev-dev) |

## Conflitos entre artefatos SDD
Nenhum após revisão: spec/plan/checklist/tasks/decision coerentes (FR-001..FR-008 ↔ T001..T011).

## Riscos mitigados
- `verify.sh:79-88` — fallback para `cargo test --locked --workspace` quando nextest ausente; nenhuma action de terceiros exigida.
- Toolchain: valor explícito `1.99.1` no YAML espelha `rust-toolchain.toml` — fonte local, sem duplicação de política (apenas de valor, por exigência da action).
- Bibliotecas de sistema instaladas no runner Ubuntu; `pkg-config --exists wayland-client` confirmado localmente (Fedora).

## Resultado
Analyze PASS — sem conflitos não resolvíveis. Correções C1–C7 aplicadas. CI remoto com job `rust` verde ainda **[NV]** até observação ao vivo.
