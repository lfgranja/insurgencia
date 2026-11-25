# Evidência — Roteiro de Testes 061 (Ato de Vandalismo do Agitador)

**Data/hora**: 2026-11-25 (UTC; carimbo de segundo a segundo **[NV]** — nenhum `date -u` executado por este agente). **Branch**: `feature/061-vandalismo` · **Âncora**: `origin/dev = 52138c3` · **Fonte**: somente os resultados locais **[EV]** registrados no contexto. **Rotulagem**: toda linha pendente é **[NV]**, nunca PASS.

| Passo | Comando/ação | Saída observada | Resultado | Referência |
|:--|:--|:--|:--|:--|
| 1 — whitelist 15/13 | `cargo test --offline --locked -p insurgencia-servidor --test cli` + inspeção de `ACOES_PERMITIDAS`/`acao_exige_destino` | `11 passed`; whitelist de 15 tokens e aridade de 6 tokens cobertas na suíte da CLI (T003/T009) | PASS | [EV] contexto; `specs/061-vandalismo-agitador/tasks.md` (T003, T009) |
| 2 — alvo/alcance/fôlego/recarga | `cargo test --offline --locked -p insurgencia-sim --test simulacao` | `112 passed`; casos de aceite/recusa tipada em `ValidacaoComandos::validar` (T004, T006, T009) | PASS | [EV] contexto; `specs/061-vandalismo-agitador/tasks.md` (T006, T009) |
| 3 — dano 120 + silêncio 45 | `cargo test --offline --locked -p insurgencia-sim --test simulacao` | `112 passed`; dano 120 pela rota única, silêncio 45, custo 20 e recarga 210 com monotônicos só no aceite (T007, T009) | PASS | [EV] contexto; `specs/061-vandalismo-agitador/tasks.md` (T007, T009) |
| 4 — cancelamento de canalização | `cargo test --offline --locked -p insurgencia-sim --test simulacao` | `112 passed`; ramo aceito de `fase_skills` zera a canalização do alvo na mesma passagem do silêncio (T007, T009) | PASS | [EV] contexto; `specs/061-vandalismo-agitador/tasks.md` (T007) |
| 5 — bloqueio de ações por silêncio | `cargo test --offline --locked -p insurgencia-sim --test simulacao` + inspeção de `crates/cliente/src/main.rs` | `112 passed`; decrescentes de silêncio/recarga uma vez por tick e recusa de ações durante o silêncio; despacho de `KeyCode::KeyH` e linha da HUD rastreados em T008 | PASS | [EV] contexto; `specs/061-vandalismo-agitador/tasks.md` (T005, T008) |
| 6 — determinismo/hash | `cargo test --offline --locked -p insurgencia-sim --test simulacao` | `112 passed`; **zero golden hashes fixos na sim** — hash divergente da baseline e igual entre replays equivalentes (T004, T009) | PASS | [EV] contexto; `specs/061-vandalismo-agitador/plan.md` (risco "golden literals") |
| 7 — gates Python + formatação | `python3 scripts/ci/check_work_order_dag.py`; `check_task_id_uniqueness.py`; `check_converge_append_only.py`; `hub_docs_gate.py`; `check_sdd_completeness.py`; `rustfmt --check` nos 4 `.rs` | `DAG PASS`, `task-id PASS`, `converge PASS`, `hub_docs PASS`; `FMT_OK`. `check_sdd_completeness` → **FAIL** com causa única: `validate`/`release` ainda ausentes no momento da execução; reexecução **[NV]** após a escrita destes artefatos | PASS (4 gates + FMT_OK) · FAIL pontual (completeness, pré-escrita) · reexecução **[NV]** | [EV] contexto; `scripts/ci/` |
| 8 — `verify.sh` remoto (jobs `gates`/`rust`) | `./verify.sh` + GitHub Actions, workflow `deterministic-gates` | Nenhuma saída observada no contexto; `cargo check -p insurgencia-cliente --all-targets` ainda em execução **[NV]** | **[NV]** | Referência **[NV]**: `https://github.com/lfgranja/insurgencia/actions/workflows/ci-gates.yml`; runs em `https://github.com/lfgranja/insurgencia/actions/runs/<id>` **[NV]** — nenhum run observado |

## Consolidação

- **PASS observado**: passos 1–6 (suítes Rust locais: **112 passed** na sim e **11 passed** na CLI) e parte do passo 7 (quatro gates Python e `FMT_OK`).
- **FAIL observado e explicado**: `check_sdd_completeness.py`, apenas por `validate`/`release` ausentes no instante da execução — causa resolvida por esta escrita, reexecução **[NV]**.
- **[NV] (nenhum PASS afirmado)**: `cargo check -p insurgencia-cliente --all-targets`, `cargo test --offline --locked -p insurgencia-cliente --lib`, `./verify.sh` completo (`ALL STAGES OK`), jobs remotos `gates`/`rust`, URLs de runs, capturas de terminal/JSON e captura visual da HUD.
- A atribuição de cada contagem agregada ao passo correspondente segue o mapeamento FR↔SC↔T em `specs/061-vandalismo-agitador/`; nenhuma saída individual de teste além das contagens acima foi colada. Nenhum valor numérico do protótipo é apresentado como cânone — todos são **`[HIPÓTESE]`**.
- Este arquivo é append-only: novas execuções devem ser acrescentadas com carimbo e rótulo `[EV]`, sem reescrever as linhas acima.

## Adendo [EV] 2026-11-25T06:04:01Z — reexecução após analyze e correções

- [EV] `cargo test --offline --locked -p insurgencia-sim --test simulacao` → **114 passed; 0 failed** (106 baseline + 8 novos: 6 do SimCore + 2 do SimTestsFix).
- [EV] `cargo test --offline --locked -p insurgencia-servidor --test cli` → **11 passed; 0 failed** (whitelist 15, destino 13).
- [EV] `rustfmt --edition 2021 --check` nos 4 `.rs` alterados → `FMT_OK`.
- [EV] `cargo check --offline --locked -p insurgencia-cliente --all-targets` → `Finished dev profile` (build frio anterior 14m05s; recheck 13,81s).
- [EV] 5 gates Python → `check_sdd_completeness PASS`, `check_work_order_dag PASS`, `check_task_id_uniqueness PASS`, `check_converge_append_only PASS`, `hub_docs_gate PASS`.
- Correções aplicadas aos achados do `/speckit.analyze`: A1/A2 (cliente: validação antes do agendamento + linha de silêncio/recarga na HUD), A4 (spec), A5/A6/A7 (sim). Nenhuma regra canônica alterada; todos os números permanecem `[HIPÓTESE]`.
- [NV] CI remoto `gates`/`rust`, `./verify.sh` completo e captura visual da HUD — a observar no PR; nenhum verde remoto é afirmado aqui.

