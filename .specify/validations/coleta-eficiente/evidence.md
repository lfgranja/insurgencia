# Evidência — Roteiro de Testes 062 (Coleta Eficiente do Camponês)

**Data**: 2026-11-25 (carimbo de segundo a segundo **[NV]** — nenhum `date -u` executado por este agente). **Branch**: `feature/062-coleta-eficiente` · **Âncora**: `origin/dev = d3be560` · **Worktree**: `.specify/queue/worktrees/cycle-062-coleta-eficiente/` · **Superfície**: CLI/terminal, **sem UI**.
**Rotulagem**: toda linha abaixo é **[NV]** até o orquestrador/SWE anexar a saída observada com comando, carimbo e fonte; nenhum PASS é afirmado neste ciclo. Todos os números do protótipo são **[HIPÓTESE]**.

| Passo | Comando/ação | Saída observada | Resultado | Referência |
|:--|:--|:--|:--|:--|
| 1 — whitelist 16/14 | Inspeção de `ACOES_PERMITIDAS`/`acao_exige_destino` + `printf '1 coleta_eficiente 1 1 0.0 0.0\n' \| cargo run --locked -p insurgencia-servidor -- --ticks 1` | Nenhuma saída colada **[NV]**; contagens de 16 tokens/14 com destino pendentes | **[NV]** | `test-roadmap.md` passo 1; T003, T009 |
| 2 — destino/fôlego/recarga | `cargo test --offline --locked -p insurgencia-sim --test simulacao` + `cargo test --offline --locked -p insurgencia-servidor --test cli` | Nenhuma saída colada **[NV]**; contagem de testes e recusas tipadas pendentes | **[NV]** | `test-roadmap.md` passo 2; T004, T006, T009 |
| 3 — 90 vs 180 ticks, recompensa igual | Suíte determinística da sim, comparação sob buff contra baseline | Nenhuma saída colada **[NV]**; ticks de conclusão e recompensa pendentes | **[NV]** | `test-roadmap.md` passo 3; T005, T007, T009 |
| 4 — sem *stacking* | Reaplicação de `coleta_eficiente` com buff 150/recarga 300 ativos | Nenhuma saída colada **[NV]**; recusa sem acumulação pendente | **[NV]** | `test-roadmap.md` passo 4; T006, T007 |
| 5 — decrescentes e reativação | Avanço de ticks sobre buff/recarga e reaplicação após expiração | Nenhuma saída colada **[NV]**; um decremento por tick e reativação pendentes | **[NV]** | `test-roadmap.md` passo 5; T005, T007 |
| 6 — hash determinístico | Replays equivalentes em `Mundo` independentes, comparação de `hash_estado()` a cada tick | Nenhuma saída colada **[NV]**; igualdade entre replays e divergência da baseline pendentes | **[NV]** | `test-roadmap.md` passo 6; T004, T009 |
| 7 — gates Python + clippy/fmt | Cinco `python3 scripts/ci/*.py`, `cargo check --locked -p insurgencia-cliente --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check` | Nenhuma saída colada **[NV]**; resultados dos gates, clippy e fmt pendentes | **[NV]** | `test-roadmap.md` passo 7; `scripts/ci/` |
| 8 — `verify.sh` remoto | `./verify.sh` + jobs `gates`/`rust` do workflow `deterministic-gates` | Nenhuma saída observada **[NV]**; `ALL STAGES OK` e status dos jobs pendentes | **[NV]** | Referência **[NV]**: `https://github.com/lfgranja/insurgencia/actions/workflows/ci-gates.yml`; runs em `https://github.com/lfgranja/insurgencia/actions/runs/<id>` — nenhum run observado |

## Consolidação

- **PASS observado**: nenhum neste ciclo. Toda execução de teste, gate, build, formatador, captura e CI permanece **[NV]** até o orquestrador/SWE anexar as saídas.
- Contagens de testes/gates, carimbos, URLs de runs e capturas de terminal/JSON **serão coladas pelo orquestrador** em adendos datados, sem alterar as linhas acima.
- A inspeção estática de `KeyCode::KeyK` e `texto_hud` substitui qualquer captura visual: não há janela Bevy, browser ou acionamento de tecla real neste roteiro.
- O rollback do roteiro é `rm -rf "$TMP"`; o da feature é `git revert <hash do commit da feature>` (pai `d3be560`), documentado e não executado.

**Append-only**: este arquivo é append-only. Novas execuções devem ser acrescentadas como adendos com data/hora e rótulo **[EV]**, nunca reescrevendo ou reclassificando as linhas **[NV]** anteriores.

## Adendo [EV] 2026-11-25T07:27:56Z — verificação local final

- [EV] `cargo test --offline --locked -p insurgencia-sim --test simulacao` → **121 passed; 0 failed** (114 anteriores + 7 novos determinísticos).
- [EV] `cargo test --offline --locked -p insurgencia-servidor --test cli` → **12 passed; 0 failed** (whitelist 16, destino 14).
- [EV] `rustfmt --edition 2021 --check` nos 4 `.rs` → `FMT_OK`.
- [EV] `cargo clippy --workspace --all-targets --all-features -- -D warnings` → `Finished dev profile`, sem erros. Exceção escopada e documentada: `#[expect(clippy::too_many_lines, reason = "16 ações em ordem fixa no tick")]` em `Mundo::fase_skills` (101/100 linhas após a 16ª ação); nenhum blanket `allow`.
- [EV] 5 gates Python → `check_sdd_completeness PASS`, `check_work_order_dag PASS`, `check_task_id_uniqueness PASS`, `check_converge_append_only PASS`, `hub_docs_gate PASS`.
- Correções aplicadas após `analyze`: F1 (documentação sem afirmação de posição do token), F2 (`ultima_recusa` preservada na recusa), F3 (`assert_eq!` de Fôlego), E0594 em 5 testes (`get_mut` em vez de indexação mutável de `BTreeMap`) e lint `too_many_lines`.
- [NV] CI remoto `gates`/`rust`, `verify.sh` completo e captura visual da HUD — a observar no PR.

