# Convergência — 028-ci-rust-governanca

**Data**: 2026-11-21T04:28Z · **Modo**: READ-ONLY, append-only.

## Evidências executadas
- `python3 -c 'import yaml; ...'` → `jobs: ['gates', 'rust']`, exit 0.
- `./verify.sh` → ALL STAGES OK, 34/34 testes, 4 estágios (check → clippy → nextest/test → fmt).
- 5 gates: sdd-completeness PASS (após validate/release), work-order-dag PASS (28 specs, 243 tasks), task-id-uniqueness PASS, converge-append-only PASS (10 arquivos), hub-docs-gate PASS (0 erros/0 warnings).
- `pytest -q` → exit 5 (N/A, sem suíte Python).

## Correções aplicadas (C1–C5 do analyze)
- C1: job `rust` em `.github/workflows/ci-gates.yml` executando `./verify.sh`, toolchain lida de `rust-toolchain.toml`, sem action de nextest (fallback do script auditado em `verify.sh:79-88`).
- C2/C5: `AGENTS.md` reescrito — 5 crates Rust, 34 testes, `verify.sh`, regra da janela, ADR-004, Bloco 0.a, SDD, append-only, 80/20, untrusted content.
- C3: `verify.sh:2` — "Insurgência" em vez de "casv-rust".
- C4: `docs/GDD - Insurgência.md:1627` — CI materializado com check `gates SUCCESS` observado em 2026-11-21T02:57Z; job `rust` **[NV]** até execução ao vivo.

## Atendimento
FR-001..FR-008 atendidos. SC-001..SC-004 atendidos; SC-005 **[NV]** (CI remoto com o novo job ainda não observado ao vivo). Nenhum arquivo alheio tocado (ast-* assessments preservados). Baseline de converge inalterado (converge.md é novo, 028 não é baselined).
