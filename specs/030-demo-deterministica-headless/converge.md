# Convergência — 030-demo-deterministica-headless

**Data**: 2026-11-21T06:00Z · **Modo**: READ-ONLY, append-only.

## Evidências (observadas — comandos executados pelo orquestrador)
- `rustc --edition 2021 -D warnings` contra a rlib pré-construída de `insurgencia_sim` → PASS.
- `clippy-driver --edition 2021 -D warnings -W clippy::pedantic` → exit 0 (`CLIPPY_EXIT=0`).
- Duas execuções do binário → `diff` vazio (`DETERMINISTIC_OK`), 30 ticks, `hash_final=6fcc2f9e652da1834e213f6afa2eb58d306bfbc5846060ec0513b67f68294f12`, `segundos=1`.
- `verify.sh` completo e CI remoto: **[NV]** — duas tentativas estouraram 180 s e 600 s por processos `cargo` concorrentes de outros repositórios na máquina (constituição VI: nenhum verde afirmado sem saída observada).
- Exemplo coberto por `verify.sh --all-targets` quando o `cargo` estiver livre.

## Atendimento
FR-001..FR-008 atendidos; SC-001..SC-004 e SC-006 atendidos; SC-005 permanece **[NV]** conforme registrado. Regra da janela respeitada (`--minimizado` sempre; nenhuma janela normal por iniciativa de agente). Princípio I/VI: saída de LLM informativa, gate determinístico como autoridade. Nenhum trabalho alheio tocado; SDD ordenado (R1 go → R4 → R7).

## 2026-11-21T13:23Z — Errata de redação (append-only)

- A expressão `verify.sh --all-targets` era sintaticamente inválida: `grep` em `verify.sh:5-15,70,75` confirma que o script não expõe essa flag e a usa internamente. Corrigida para `./verify.sh` (que executa `--all-targets` internamente) nos artefatos do ciclo 030, em `.specify/memory/STATE.md` e em `docs/research/slotbound-2026-11-21-sintese.md`. Correção exclusivamente documental: FR-001..FR-008, SC-001..SC-006 e as evidências acima permanecem inalteradas; nenhuma linha anterior desta convergência foi reescrita, nenhum gate/build/commit executado.
