# Roteiro de Testes — Ato de Vandalismo do Agitador (ciclo 061)

**Status**: escrito antes do R4 · **Superfície**: CLI/terminal (sem UI, sem janela Bevy, sem rede) · **Execução**: orquestrador/SWE · **Saídas**: coladas em `evidence.md`

**Aviso de valor**: custo 20 de Fôlego, alcance 2 m (`ALCANCE_CONTATO_M`), dano 120, silêncio 45 ticks (1,5 s × 30 Hz), recarga 210 ticks (7 s) e tecla protótipo `H` são **`[HIPÓTESE]`/protótipo** da decisão 80/20 de 2026-11-25. Nenhum número é regra canônica; o GDD é lido, nunca reescrito.

## Escopo e rastreabilidade

| Item | Cobertura (passos) |
|:--|:--|
| FR-001 — `vandalismo` como 15º token de `ACOES_PERMITIDAS`, com destino, linha CLI de 6 tokens | 1 |
| FR-002 — validação de alvo no destino, alcance 2 m, Fôlego ≥ 20, recarga zerada | 2 |
| FR-003 — 120 de dano pela rota única, 45 ticks de silêncio, cancelamento da canalização, custo 20, recarga 210, monotônicos só no aceite, hash | 3, 4, 5, 6 |
| FR-004 — tecla `H`, IDs monotônicos e HUD derivada da sim | 1, 2, 5 (inspeção) |
| SC-001..SC-004 | 1–8 |
| BF-002c (fecha) / BF-015b (permanece aberto) | 1–6 / fora do corte |
| Governança (fora dos FR) | 7, 8 |

T-IDs relacionados: T001/T003/T009 (contrato e aridade), T002/T004/T006/T009 (validação), T004/T005/T007/T009 (efeito, decrescentes e hash), T002/T008/T010 (cliente, HUD e evidência). A implementação nos quatro módulos é do orquestrador/SWE; este roteiro apenas a exercita.

## Pré-condições

1. Worktree `.specify/queue/worktrees/cycle-061-vandalismo/`, branch `feature/061-vandalismo`, base `origin/dev = 52138c3`; identidade em `.specify/feature.json → specs/061-vandalismo-agitador` (nunca inferida do nome do branch); R1 `go`, score 288/3125 (`4×4×2×3×3`), sessão `insurgencia - Ato de Vandalismo do Agitador` (2026-11-25).
2. Implementação pronta nos quatro módulos do plano: `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs`, `crates/cliente/src/main.rs` e `crates/servidor/tests/cli.rs`. Convenção preservada: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, `Result`/`thiserror`, sem `unwrap`/`expect`/`panic!` em produção, `BTreeMap`/`BTreeSet`/`VecDeque`, `f64::total_cmp`, nomes em pt-BR, exceções de lint com razão. **Zero literais de hash de 64 hex na sim.**
3. `TMP=$(mktemp -d)` fora do worktree; `mkdir -p "$TMP/saidas"`. Capturas em `.txt` e manifesto `capturas.json`. Nenhum `git add -A`/`git add .`, stage explícito apenas, sem commit, push, PR, merge, deploy ou issue por este agente.
4. Ambiente headless: nenhuma janela, GPU, rede, relógio externo, aleatoriedade ou socket. Venue para compilação pesada é GitHub Actions; nenhum timeout local é rebaixado para RED.
5. Rollback conhecido e **não executado**: `git revert <sha-do-commit-da-feature>` devolve HEAD a `52138c3`.

## Superfície CLI/terminal

```bash
cargo test --offline --locked -p insurgencia-sim --test simulacao
cargo test --offline --locked -p insurgencia-servidor --test cli
cargo test --offline --locked -p insurgencia-cliente --lib            # [NV] até execução
cargo check --locked -p insurgencia-cliente --all-targets            # [NV] até execução
python3 scripts/ci/check_work_order_dag.py
python3 scripts/ci/check_task_id_uniqueness.py
python3 scripts/ci/check_converge_append_only.py
python3 scripts/ci/hub_docs_gate.py
python3 scripts/ci/check_sdd_completeness.py
./verify.sh
```

Sonda CLI (6 tokens; formato inalterado independentemente do desfecho da validação):

```bash
printf '1 vandalismo 1 1 2.0 0.0\n' \
  | cargo run --offline --locked -p insurgencia-servidor -- --ticks 2
```

Inspeção estática: `grep`/`sed`/`cat` em `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs`, `crates/cliente/src/main.rs` e `crates/servidor/tests/cli.rs`. Nenhuma captura de tela, browser ou interação gráfica.

## Passos numerados (ação → resultado esperado)

| # | Superfície | Pré-condição | Ação | Resultado esperado | automatizável? |
|:--|:--|:--|:--|:--|:--|
| 1 | Terminal + CLI headless | Código implementado em `crates/sim/src/lib.rs` e `crates/servidor/tests/cli.rs` | Contar `ACOES_PERMITIDAS` e conferir `acao_exige_destino` para `vandalismo`, `sinalizar_sincronia` e `sinalizar_alavanca`; enviar linha CLI de 6 tokens | **15** tokens com `vandalismo` como último; **13** exigem destino e apenas as **2** ações de sinalização devolvem `false`; CLI preserva `<tick>\t<sha256>` antes de cada `avancar_tick`, sem parser local nem whitelist paralela | sim |
| 2 | Terminal (`cargo test`) | T004 e T006 aplicados; constantes `[HIPÓTESE]` documentadas | Rodar a suíte da sim e isolar os casos de aceite/recusa: alvo no destino, 2,00 m vs. 2,01 m, Fôlego 20 vs. 19, recarga 0 vs. 210 | Aceite com `Ok(())`; recusa **tipada** em `CodigoRecusa` para alvo ausente, distância 2,01 m, Fôlego 19 e recarga 210; nenhuma recusa altera `seq`, `request_id`, Fôlego, recarga ou o alvo; `ultima_recusa` preservada | sim |
| 3 | Terminal (`cargo test`) | T007 aplicado; rota única `aplicar_dano_a_jogador` disponível | Aplicar `vandalismo` válido a alvo com escudo neutro e conferir vida, silêncio, Fôlego e recarga; repetir com comando recusado | Dano exato **120** pela rota única (absorção de escudo e piso zero, nunca negativo), **45 ticks** de silêncio no alvo, **−20** de Fôlego e **210 ticks** de recarga no agressor; monotônicos avançam **somente no aceite**; comando recusado não muda nenhuma magnitude | sim |
| 4 | Terminal (`cargo test`) | T007 aplicado; alvo semeado com canalização ativa | Semear canalização no alvo, aplicar `vandalismo` aceito e inspecionar os campos de canalização antes/depois e no hash | Canalização do alvo **zerada na mesma passagem** do silêncio, sem estado parcial observável; nenhum campo remanescente e nenhuma segunda aplicação no tick seguinte | sim |
| 5 | Terminal (`cargo test` + inspeção) | T005, T007 e T008 aplicados | Avançar ticks com `silencio_ticks > 0` e tentar novas ações do alvo; conferir o decrescente e inspecionar o despacho de `KeyCode::KeyH` e a linha de `texto_hud` | Enquanto o silêncio dura, ações do alvo são recusadas sem efeito; silêncio e recarga decrescem **exatamente uma vez por tick** em ordem estável de `peer_id`; ao zerar, o alvo volta a agir; `KeyH` agenda `vandalismo` com destino e IDs crescentes, `KeyQ`/`atacar_forte` intactos, HUD lê somente a sim | sim (inspeção da HUD é parcial) |
| 6 | Terminal (`cargo test`) | T004 e T009 aplicados; sem golden literals | Comparar hashes de dois mundos com a mesma sequência, tick a tick, e de um mundo sem `vandalismo`; conferir ausência de literais de 64 hex | Hashes **iguais entre replays equivalentes** e **diferentes da baseline**; campos novos entram em `hash_estado` em ordem estável; `ultima_recusa` participa do hash; **zero** golden hashes fixos na sim | sim |
| 7 | Terminal (gates Python + `rustfmt`) | Cinco scripts em `scripts/ci/` e os quatro `.rs` alterados presentes; `validate.md` e `release.md` deste ciclo criados | Rodar os cinco gates e `rustfmt --check` nos quatro arquivos; registrar cada exit code em `$TMP/saidas/gates-*.txt` | `DAG PASS`, `task-id PASS`, `converge PASS`, `hub_docs PASS`, `sdd-completeness PASS` (com `validate`/`release` presentes) e `FMT_OK`; qualquer divergência é corrigida na fonte, nunca na expectativa | sim |
| 8 | Terminal + GitHub Actions | Feature pronta no head; jobs `gates` e `rust` habilitados no workflow `deterministic-gates` | Rodar `./verify.sh` e observar os jobs remotos `gates`/`rust` em `https://github.com/lfgranja/insurgencia/actions/workflows/ci-gates.yml` | `verify.sh: ALL STAGES OK` (check → clippy → nextest/test → fmt) e ambos os jobs verdes; até observação, tudo permanece **[NV]** — nenhum PASS é afirmado para CI remoto | parcial (requer observação do CI) |

**Nota de ordem**: o débito de 20 Fôlego, o dano 120, o silêncio 45, o cancelamento da canalização e a recarga 210 ocorrem no ramo aceito de `fase_skills`; os decrescentes de silêncio/recarga ocorrem uma vez por tick em `avancar_tick`, em ordem estável. Recusa preserva `ultima_recusa` e não avança monotônicos.

## Plano de capturas (terminal/JSON)

- `$TMP/saidas/passo-01.txt` … `passo-08.txt` — stdout/stderr e exit code de cada passo, com carimbo `date -u`.
- `$TMP/saidas/gates-*.txt` — saída individual dos cinco gates Python; `$TMP/saidas/fmt.txt` — resultado do `rustfmt --check` (`FMT_OK`).
- `$TMP/saidas/capturas.json` — manifesto com `passo`, `comando`, `exit_code`, `stdout`, `stderr`, `carimbo_utc`, `rotulo` (`[EV]` quando executado, `[NV]` quando pendente). Nenhuma captura de tela, áudio ou browser.
- Toda captura pendente é rotulada **[NV]**; nenhuma é promovida a `[EV]` sem saída colada em `evidence.md`.

## Rollback

Se qualquer passo divergir, corrigir a **fonte** (código/artefato do ciclo), nunca a expectativa do teste — em especial whitelist/aridade (passo 1), estado após recusa (passo 2), magnitudes e monotônicos (passo 3), canalização residual (passo 4), duplicação de decrescentes ou conflito de teclas (passo 5), hash omisso ou literal fixo (passo 6) e gates (passo 7).

Para reverter o corte: `git revert <sha-do-commit-da-feature>`, devolvendo HEAD a `origin/dev = 52138c3` e restaurando os testes e gates do baseline. Isso preserva o histórico: **não** usar `git clean`, `git reset --hard`, force-push, remoção forçada de branches ou reescrita de commits. `rm -rf "$TMP"` remove somente capturas fora do worktree. Não há promoção `dev → main` nem deploy; PR #76 permanece intocável. Rollback documentado e **não executado** por este agente.
