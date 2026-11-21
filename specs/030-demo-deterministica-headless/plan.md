# Implementation Plan — 030-demo-deterministica-headless

1. Confirmar R1 `go` no veredito ADOPT de `.specify/assessments/sb-16-demo-itch-io/decision.md` (síntese das 28 decisões em `docs/research/slotbound-2026-11-21-sintese.md`).
2. Materializar `crates/sim/examples/demo_deterministica.rs`: `Mundo::new()` → 2 jogadores → `agendar_comando` `mover` (30 ticks) → `avancar_tick()` com `PASSO_SEGUNDOS` → `hash_estado()` por tick. Apenas `insurgencia_sim`.
3. Compilar com `rustc --edition 2021 -D warnings` contra a rlib pré-construída de `insurgencia_sim` → PASS.
4. Rodar `clippy-driver --edition 2021 -D warnings -W clippy::pedantic` → exit 0.
5. Executar o binário duas vezes e comparar com `diff`: vazio (`DETERMINISTIC_OK`), 30 ticks, `hash_final=6fcc2f9e...`, `segundos=1`.
6. Registrar honestamente `verify.sh` completo e CI remoto como **[NV]** (duas tentativas estouraram 180 s e 600 s por `cargo` concorrente de outros repos).
7. Regra da janela: validação sempre em `--minimizado`; nenhuma janela normal por iniciativa de agente; demo permanece headless.
8. Princípio I/VI: saída de LLM informativa; gate determinístico é autoridade; nenhum verde sem comando executado e saída colada.
9. Verificar cobertura automática: `./verify.sh` (que executa `--all-targets` internamente) inclui o exemplo, sem novo job.
10. converge (append-only, 2026-11-21) → validate (R4 aprovado) → release (R7, `1.0.0`, changelog `sb-16`, rollback) → PR → CI verde → merge.
