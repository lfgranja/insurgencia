# Research — Núcleo, contrato versionado e Módulo de exemplo

**Data:** 2026-11-20T02:00Z · **Método:** leitura de artefatos versionados do alvo + execução de comandos locais.

## Evidência local (executada)
| Fato | Comando / fonte | Resultado |
|:--|:--|:--|
| Engine executável | `/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64 --headless --version` | `4.7.2.stable.official.ed1daf0bf`, exit 0 |
| Benchmark existente roda | `... --headless --path tests/benchmark --script headless_tick.gd` | `BENCHMARK ticks=600 entities=64 ... per_tick_us=26.73`, exit 0 |
| Caminhos canônicos ausentes | inventário (216-) | 0/39; apenas `LICENSE`, `.gitignore`, `project.godot`, `tests/` existem |
| Contrato prescrito | `docs/plans/...:27-303` | Task 1 com 9 steps, 8 arquivos, saída esperada de 16 `PASS` |
| doc CLI 4.7 | `docs/research/2026-11-20-godot-headless-oficial.md` | `--script` exige `extends SceneTree` — o teste do plano já cumpre |

## Decisão técnica derivada
Usar o código literal do plano (Steps 1–8), com o binário absoluto no lugar de `godot` nos comandos. Nenhuma dependência externa, nenhum serviço remoto, nenhuma nova ADR.
