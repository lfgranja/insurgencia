# Release (R7) — 030-demo-deterministica-headless

**Data:** 2026-11-21T06:00Z · **Versão:** `1.0.0` (SemVer) · **R4:** aprovado

## Changelog
### [1.0.0] — 2026-11-21
### Added
- `crates/sim/examples/demo_deterministica.rs` — demonstração determinística e headless derivada apenas de `insurgencia_sim` (`Mundo`, `Comando`, `PASSO_SEGUNDOS`, `avancar_tick`, `hash_estado`): 2 jogadores, 30 ticks, `hash_estado()` por tick. Sem Bevy/janela/rede/aleatoriedade.
- Saída reproduzível: duas execuções com `diff` vazio (`DETERMINISTIC_OK`), `hash_final=6fcc2f9e652da1834e213f6afa2eb58d306bfbc5846060ec0513b67f68294f12`, `segundos=1`.
- Cobertura automática por `./verify.sh` (que executa `--all-targets` internamente), sem novo job de CI; regra da janela (`--minimizado`) e princípio I/VI preservados.
- Origem: proposta [sb-16 — Demo versionada](../../assessments/sb-16-demo-itch-io/decision.md) (ADOPT) — item executável de maior retorno do ciclo 80/20 (síntese das 28 decisões).

## Rastreabilidade
FR-001..FR-008 · T001..T011 · SC-001..SC-006 · R1 go · R4 `.specify/validations/demo-deterministica-headless/validate.md`.

## Rollback
`git revert <merge commit do PR>` remove o exemplo e restaura o estado anterior; nenhuma baseline, gate, contrato ou trabalho alheio afetado. Sem alteração de simulação, CI ou constituição.

## Não incluído
Steam/App ID/SDK, janela normal por iniciativa de agente, publicação em itch.io como fonte, e afirmação de `verify.sh` completo ou CI remoto — ambos **[NV]** por timeout (180 s e 600 s) causado por processos `cargo` concorrentes de outros repositórios na máquina.
