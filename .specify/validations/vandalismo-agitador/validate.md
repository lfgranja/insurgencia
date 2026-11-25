# Validação (R4) — 061-vandalismo-agitador

**Data**: 2026-11-25 · **Branch**: `feature/061-vandalismo` (worktree `.specify/queue/worktrees/cycle-061-vandalismo`, âncora `origin/dev = 52138c3`) · **Veredito**: aprovado · R4 precede R7 (`1.0.0-local`, constituição v1.2.2).

**Aviso de valor**: custo 20 de Fôlego, alcance 2 m, dano 120, silêncio 45 ticks, recarga 210 ticks e tecla `H` são **`[HIPÓTESE]`/protótipo**; nenhum número é promovido a cânone e o GDD não é reescrito. **BF-002c fecha** neste corte; **BF-015b** (`sinalizar_alavanca`) permanece aberto. **R6 N/A** (PARE se invocada; nenhuma issue simulada, nenhum `gh` improvisado).

## Comandos e saídas coladas do contexto [EV]

As saídas abaixo foram **executadas pelo orquestrador** e são aqui transcritas; este agente não executou gate, teste, formatador, build, git, commit, push, PR nem deploy.

```text
$ cargo test --offline --locked -p insurgencia-sim --test simulacao
  → 112 passed

$ cargo test --offline --locked -p insurgencia-servidor --test cli
  → 11 passed

$ rustfmt --check nos 4 arquivos .rs alterados
  → FMT_OK

$ python3 scripts/ci/check_work_order_dag.py        → DAG PASS
$ python3 scripts/ci/check_task_id_uniqueness.py    → task-id PASS
$ python3 scripts/ci/check_converge_append_only.py  → converge PASS
$ python3 scripts/ci/hub_docs_gate.py               → hub_docs PASS
$ python3 scripts/ci/check_sdd_completeness.py      → FAIL: ausência de validate/release
                                                      (causa resolvida por esta escrita;
                                                       reexecução [NV])
```

Observação adicional **[EV]**: **zero literais de hash fixos (golden hashes) na sim**; os testes de determinismo comparam replays equivalentes e divergência da baseline, sem endurecer expectativa para obter verde.

## Critérios de sucesso

| SC | Evidência observada | Veredito |
|:--|:--|:--|
| **SC-001** — token e aridade mensuráveis | `cargo test --offline --locked -p insurgencia-servidor --test cli` → **11 passed** (aridade de 6 tokens e whitelist de 15 rastreadas em T003/T009); inspeção do contrato local em `crates/sim/src/lib.rs`; formato `<tick>\t<sha256>` preservado | **PASS** |
| **SC-002** — validação determinística | `cargo test --offline --locked -p insurgencia-sim --test simulacao` → **112 passed** (aceite/recusa tipada de alvo, 2 m, 20 Fôlego e 210 ticks de recarga, T006/T009); recusa sem avanço de `seq`/`request_id` | **PASS** |
| **SC-003** — efeito exato e hash coerente | `cargo test --offline --locked -p insurgencia-sim --test simulacao` → **112 passed** (dano 120, silêncio 45, cancelamento da canalização, custo 20, recarga 210 e hash divergente/estável, T004/T005/T007/T009); zero golden hashes fixos na sim | **PASS** |
| **SC-004** — consumidor e governança | Quatro gates Python **PASS** (`DAG`, `task-id`, `converge`, `hub_docs`) e `FMT_OK` nos quatro `.rs`; despacho de `KeyCode::KeyH` e HUD derivada da sim rastreados em T008/T010. **`[NV]`**: `cargo test --offline --locked -p insurgencia-cliente --lib`, `cargo check --locked -p insurgencia-cliente --all-targets`, `./verify.sh` e os jobs remotos `gates`/`rust` — nenhum destes conta como PASS | **PASS** no escopo local observado, com ressalvas **[NV]** acima |

## Ressalvas [NV]

- `cargo check -p insurgencia-cliente --all-targets`: em execução no contexto, **sem saída observada** — **[NV]**.
- `cargo test --offline --locked -p insurgencia-cliente --lib`: não executado neste contexto — **[NV]**.
- CI remoto: jobs `gates` e `rust` em `https://github.com/lfgranja/insurgencia/actions/workflows/ci-gates.yml` ainda **não observados** — **[NV]**; nenhum verde é afirmado para eles.
- `./verify.sh` completo (check → clippy → nextest/test → fmt) e a mensagem `verify.sh: ALL STAGES OK`: **[NV]**.
- `check_sdd_completeness.py`: o FAIL observado tinha como única causa a ausência de `validate.md` e `release.md`; com a escrita destes artefatos a causa está resolvida, mas a **reexecução permanece [NV]**.
- Captura visual da HUD (X11/GPU), builds locais adicionais, stage, commit, push, PR, merge e deploy: **[NV]** e fora da autoridade deste agente.

## R4

Aprovado. O veredito apoia-se nas saídas **[EV]** coladas acima (112 passed na sim, 11 passed na CLI, `FMT_OK` e quatro gates Python PASS) e na rastreabilidade FR-001..FR-004 ↔ SC-001..SC-004 ↔ T001..T010 de `specs/061-vandalismo-agitador/`. R4 antecede R7 `1.0.0-local` conforme a constituição v1.2.2. A aprovação **não** cobre CI remoto, `cargo check --all-targets` do cliente, `verify.sh` completo nem a reexecução de `check_sdd_completeness.py`, todos **[NV]**; esses itens são de responsabilidade do orquestrador/SWE, que deve anexar as saídas em `evidence.md` antes de qualquer promoção. Nenhuma issue é criada (R6 N/A), nenhum commit é feito e nenhuma promoção `dev → main` ou deploy é autorizada por esta validação.

## Revalidação R4 — 2026-11-25T06:04:01Z

Após as correções do `/speckit.analyze`, a validação local foi reexecutada: **114 passed** na sim, **11 passed** na CLI, `FMT_OK`, `cargo check -p insurgencia-cliente --all-targets` concluído e **5/5 gates Python PASS** (inclusive `check_sdd_completeness`, antes FAIL apenas por `validate`/`release` ausentes). Os critérios SC-001..SC-004 permanecem atendidos; a ressalva de `cargo check` e de `check_sdd_completeness` nas linhas acima fica superada por esta reexecução, preservada como histórico. Veredito R4 mantido: **aprovado**. CI remoto, `verify.sh` completo e captura visual continuam **[NV]** até observação ao vivo no PR.

