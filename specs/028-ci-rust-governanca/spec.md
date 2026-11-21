# Feature Specification: CI Rust e governança alinhada

**Branch**: `feature/028-ci-rust-governanca` · **Created**: 2026-11-21 · **Status**: Draft · **R1**: go

## User Scenarios

### US1 — CI bloqueia código Rust quebrado (P1)
Um PR com `cargo check`/`clippy`/testes/fmt falhando NÃO recebe check verde no CI.

**Acceptance Scenarios**: 1) workflow `ci-gates.yml` contém job `rust` que executa `./verify.sh`; 2) toolchain 1.99.1 instalada explicitamente (a action exige input); 3) job `gates` (docs) permanece.

### US2 — Autoridade documental coerente (P1)
Um agente lendo `AGENTS.md` sabe que há 5 crates Rust, `verify.sh`, 34 testes e a regra da janela.

**Acceptance Scenarios**: 1) AGENTS.md não afirma "sem código/build/verify.sh"; 2) regra `--minimizado` documentada; 3) `verify.sh:2` não menciona `casv-rust`.

### US3 — GDD sem `[NV]` obsoleto (P2)
GDD:1627 reflete CI materializado, preservando histórico.

## Requirements
- **FR-001**: adicionar job `rust` em `.github/workflows/ci-gates.yml` com `./verify.sh`.
- **FR-002**: toolchain `1.99.1` declarada explicitamente no YAML (a action `dtolnay/rust-toolchain@master` exige o input; valor espelha `rust-toolchain.toml`).
- **FR-003**: `cargo-nextest` instalado no job; fallback do `verify.sh` preservado. **Corrigido**: nenhuma action de nextest é instalada; o script faz fallback para `cargo test --locked`.
- **FR-004**: reescrever `AGENTS.md` com seções obrigatórias e realidade Rust.
- **FR-005**: corrigir cabeçalho de `verify.sh`.
- **FR-006**: atualizar GDD:1627 com nota datada.
- **FR-007**: 5/5 gates SDD + `verify.sh` ALL STAGES OK após mudança.
- **FR-008**: nenhum arquivo de trabalho alheio tocado (ast-*, cor-*, doc-* preservados).

## Clarifications (R3: 0 — registradas)
Q1: CI chama o script, não comandos duplicados. Q2: toolchain explícita no YAML (action exige input), espelhando `rust-toolchain.toml`. Q3: documentação muda só onde há contradição verificada.

## Success Criteria
SC-001: workflow contém job `rust` e job `gates`. SC-002: `./verify.sh` local ALL STAGES OK. SC-003: 5/5 gates PASS. SC-004: AGENTS.md sem contradições; verify.sh sem "casv-rust"; GDD:1627 atualizado. SC-005: CI remoto com job `rust` SUCCESS observado ao vivo.
