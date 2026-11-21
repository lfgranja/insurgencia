# Feature Specification: Demo determinística headless

**Branch**: `feature/030-demo-deterministica-headless` · **Created**: 2026-11-21 · **Status**: Draft · **R1**: go

## User Scenarios
### US1 — Demo reproduzível sem janela (P1)
Qualquer terceiro executa `crates/sim/examples/demo_deterministica.rs` e obtém a mesma saída, sem Steam, SDK, rede ou janela.

**Acceptance Scenarios**: 1) duas execuções produzem `diff` vazio (`DETERMINISTIC_OK`); 2) 30 ticks, `hash_final=6fcc2f9e...`, `segundos=1`; 3) nenhuma janela gráfica é aberta.

## Requirements
- **FR-001**: exemplo em `crates/sim/examples/demo_deterministica.rs` usando apenas `insurgencia_sim` (`Mundo`, `Comando`, `PASSO_SEGUNDOS`, `avancar_tick`, `hash_estado`); sem Bevy/`DefaultPlugins`, janela, relógio, rede ou aleatoriedade.
- **FR-002**: sequência fixa: 2 jogadores, 30 ticks, comandos `mover` com monotônicos iguais a `seq`/`request_id`, `hash_estado()` impresso por tick.
- **FR-003**: saída verificável por `diff` entre duas execuções (`DETERMINISTIC_OK`), `hash_final=6fcc2f9e652da1834e213f6afa2eb58d306bfbc5846060ec0513b67f68294f12`, `segundos=1`.
- **FR-004**: regra da janela: validação sempre em `--minimizado`; nenhuma janela normal por iniciativa de agente; o exemplo permanece headless.
- **FR-005**: princípio I/VI: saída de LLM é informativa, nunca roteia estado nem finaliza validação; o gate determinístico (`verify.sh`/`--all-targets`) é a única autoridade de "verde", sem afirmação sem comando executado e saída colada.
- **FR-006**: cobertura automática por `./verify.sh` (que executa `--all-targets` internamente), sem novo job de CI.
- **FR-007**: release versionada `1.0.0` (SemVer), changelog linkando `sb-16` e rollback documentado; R4 precede R7.
- **FR-008**: convergência append-only com entradas datadas (2026-11-21).

## Success Criteria
SC-001: `rustc --edition 2021 -D warnings` contra a rlib pré-construída de `insurgencia_sim` → PASS. SC-002: `clippy-driver --edition 2021 -D warnings -W clippy::pedantic` → exit 0. SC-003: duas execuções → `diff` vazio, `DETERMINISTIC_OK`, 30 ticks, hash_final `6fcc2f9e...`, `segundos=1`. SC-004: nenhuma janela normal aberta por agente. SC-005: `verify.sh` completo e CI remoto **[NV]** por timeout (180 s e 600 s) causado por processos `cargo` concorrentes de outros repositórios na máquina. SC-006: R4 aprovado antes de R7; release `1.0.0` com changelog `sb-16` e rollback.
