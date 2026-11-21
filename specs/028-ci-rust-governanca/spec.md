# Feature Specification: CI Rust e governança alinhada

**Branch**: `feature/028-ci-rust-governanca` · **Created**: 2026-11-21 · **Status**: Draft · **R1**: go

## User Scenarios

### US1 — CI bloqueia código Rust quebrado (P1)
Um PR com `cargo check`/`clippy`/testes/fmt falhando NÃO recebe check verde no CI.

**Acceptance Scenarios**: 1) workflow `ci-gates.yml` contém job `rust` que executa `./verify.sh`; 2) toolchain vem de `rust-toolchain.toml`; 3) job `gates` (docs) permanece.

### US2 — Autoridade documental coerente (P1)
Um agente lendo `AGENTS.md` sabe que há 5 crates Rust, `verify.sh`, 34 testes e a regra da janela.

**Acceptance Scenarios**: 1) AGENTS.md não afirma "sem código/build/verify.sh"; 2) regra `--minimizado` documentada; 3) `verify.sh:2` não menciona `casv-rust`.

### US3 — GDD sem `[NV]` obsoleto (P2)
GDD:1627 reflete CI materializado, preservando histórico.

## Requirements
- **FR-001**: adicionar job `rust` em `.github/workflows/ci-gates.yml` com `./verify.sh`.
- **FR-002**: toolchain 1.99.1 lida de `rust-toolchain.toml` (sem duplicar versão no YAML).
- **FR-003**: instalar `cargo-nextest` no job; fallback do `verify.sh` preservado.
- **FR-004**: reescrever `AGENTS.md` com seções obrigatórias e realidade Rust.
- **FR-005**: corrigir cabeçalho de `verify.sh`.
- **FR-006**: atualizar GDD:1627 com nota datada.
- **FR-007**: 5/5 gates SDD + `verify.sh` ALL STAGES OK após mudança.
- **FR-008**: nenhum arquivo de trabalho alheio tocado (ast-*, GDD fora da linha 1627, etc.).

## Clarifications (R3: 0 — registradas)
Q1: CI chama o script, não comandos duplicados. Q2: toolchain única em `rust-toolchain.toml`. Q3: documentação muda só onde_há contradição verificada.

## Success Criteria
SC-001: workflow contém job `rust` e job `gates`. SC-002: `./verify.sh` local ALL STAGES OK. SC-003: 5/5 gates PASS. SC-004: AGENTS.md sem contradições; verify.sh sem "casv-rust"; GDD:1627 atualizado. SC-005: CI remoto verde após merge **[NV]** até observação ao vivo.
