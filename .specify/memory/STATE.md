# STATE.md — Insurgência

- **Data**: 2026-11-19T21:46:45Z
- **Âncora**: `specs/001-excecao-linguagem-licenca-nucleo` (`.specify/feature.json`)
- **Ciclo**: 001 — exceção de linguagem e licença MIT-only do Núcleo (R1 `go`, R4 aprovado, R7 1.0.0)
- **Branch**: `feature/001-excecao-linguagem-licenca-nucleo` (sem merge)
- **HEAD base**: `6e25758` (`main`); factory HEAD como fonte: `b1deab15f482dd85871cd6c6a857ad710b89e2a6`
- **Gate autoritativo do alvo**: `scripts/ci/` local; `check_production_gate.py --check` é N/A por decisão de governança do Bloco 0.a (factory é fonte, não autoridade)

## Gates locais (execução final)

| Comando | Saída |
|---|---|
| `python3 scripts/ci/check_sdd_completeness.py` | PASS (spec_dirs=1, gaps=0, baselined=0) |
| `python3 scripts/ci/check_work_order_dag.py` | PASS (files=1, tasks=35, cycles=0) |
| `python3 scripts/ci/check_task_id_uniqueness.py` | PASS (duplicates=0) |
| `python3 scripts/ci/check_converge_append_only.py` | PASS (files=0) |
| `python3 scripts/ci/hub_docs_gate.py` | PASS (0 erros, 0 avisos) |
| `python3 -m pytest -q` | N/A — `no tests ran` (exit 5); sem suíte no alvo |
| `python3 scripts/ci/check_production_gate.py --check` | N/A — arquivo ausente por decisão 0.a (exit 2 ≠ RED) |

## Dívidas / pendências

1. `docs/governance/sdd-completeness-baseline.json` não materializado (evitar chaves de specs da factory).
2. Aprovação humana dos ADR-001/ADR-002 pendente; estado dos ADRs é `Pendente de aprovação`.
3. `docs/specs/2026-11-19-ciclo-80-20-omoba.md` e `docs/plans/...` permanecem como ciclo de produto; o ciclo 001 é de governança e não os altera.
4. `docs/GDD - Insurgência.md` ainda registra N3 (MIT vs AGPL) e "CI desde o dia 1"; ADR-001/002 decidem localmente, mas a correção documental do GDD fica para ciclo posterior.
5. `checklists/adr-quality.md` tem 40 itens `[ ]` sob revisão humana de requisitos (não bloqueia a implementação conforme portaria reconciliada).
6. Nenhuma engine, código, CI, telemetria ou banco materializado neste ciclo — por escopo, não por omissão.

## [NV] — não verificado ao vivo

- Status de PR, CI, merge, proteções de branch e aprovações no GitHub (`gh`).
- Qualquer execução remota de gate; todas as saídas acima são locais.
- Estado da árvore de trabalho da factory (registrada apenas como fonte, nunca modificada).

## Próximo 80/20 (fora deste ciclo)

1. Corrigir documentalmente o GDD (N3 e contradição CI) com base nos ADRs aprovados.
2. Instalar Godot 4.7.2 e validar a suíte headless — exige download e autorização de dependência.
3. Gate CI remoto (GitHub Actions) — exige autorização e verificação ao vivo.
