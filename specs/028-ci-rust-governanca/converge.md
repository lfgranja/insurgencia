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

## 2026-11-21T04:35Z — Revisão pós-CI (append-only)

- CI do PR #34 (run `44866375752`): job `gates` SUCCESS; job `rust` FAILURE em `Setup Rust` — `'toolchain' is a required input` (dtolnay/rust-toolchain@master não lê `rust-toolchain.toml`). Corrigido com `toolchain: '1.99.1'` explícito.
- Revisão de artefatos: spec FR-002/FR-003, plan, analyze (C6), validate (evidência remota) e release (Fixed) atualizados. Nenhuma baseline reescrita; nenhum converge anterior editado.
- Regra da janela reforçada pela SE às 04:33Z: agente sempre `--minimizado`; janela normal encerrada imediatamente. Registrada em AGENTS.md e STATE.md.
- Inspeção visual da SE confirmada por imagem anexada às 04:33Z: low-poly visível (terreno irregular, 4 ilhas, árvores/rochas, avatar, HUD) — resolve `[NV]` SC-005 do ciclo 027.
- Reexecução local de `./verify.sh` e 5 gates pendente após esta revisão; CI remoto com job `rust` verde ainda **[NV]** até observação ao vivo.

## 2026-11-21T04:42Z — Revisão pós-CI #2 (append-only)

- CI run `44866958727` (após correção do toolchain): job `gates` SUCCESS; job `rust` FAILURE em `wayland-sys` — `Package 'wayland-client' not found` no runner Ubuntu. Corrigido com step `Install system libraries` (pkg-config, libwayland-dev, libxkbcommon-dev, libx11-dev, libudev-dev).
- Verificação local: `pkg-config --exists wayland-client` OK (Fedora), YAML parse com 4 steps no job rust, `./verify.sh` ALL STAGES OK, 5/5 gates PASS.
- Artefatos revisados: analyze (C7), release, validate, spec/plan. Nenhum converge anterior editado; nenhuma baseline reescrita.
- CI remoto com job `rust` verde ainda **[NV]** até observação ao vivo.

