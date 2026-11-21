# Validação (R4) — 030-demo-deterministica-headless

**Data**: 2026-11-21T06:00Z · **Resultado**: PASS (local) — aprovado para R7

| Verificação | Saída |
|---|---|
| `rustc --edition 2021 -D warnings` vs. rlib pré-construída de `insurgencia_sim` | PASS |
| `clippy-driver --edition 2021 -D warnings -W clippy::pedantic` | exit 0 (`CLIPPY_EXIT=0`) |
| Duas execuções do binário + `diff` | vazio — `DETERMINISTIC_OK`, 30 ticks, `hash_final=6fcc2f9e...`, `segundos=1` |
| Regra da janela | `--minimizado` sempre; nenhuma janela normal aberta por agente |
| Princípio I/VI | saída de LLM informativa; gate determinístico é autoridade |
| `verify.sh` completo + CI remoto | **[NV]** — timeout 180 s e 600 s por `cargo` concorrente de outros repos |

## R4
Aprovado para R7. Evidência local colada acima (comandos executados pelo orquestrador); nenhum verde remoto afirmado. Mudança restrita a novo exemplo em `crates/sim/examples/`, sem contratos/protocolo/simulação/gates alterados. `./verify.sh` (que executa `--all-targets` internamente) cobrirá o exemplo quando o `cargo` estiver livre; CI remoto permanece **[NV]** até confirmação ao vivo.
