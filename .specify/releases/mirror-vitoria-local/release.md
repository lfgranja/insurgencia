# Release (R7) — 058-mirror-vitoria-local

**Data:** 2026-11-24 · **Versão:** `1.0.0-local` (SemVer, pré-release **local**, não promovida a `1.0.0`) · **R4:** **APROVADO CONDICIONADO** em `.specify/validations/mirror-vitoria-local/validate.md`, **antes desta release**, aos jobs remotos `gates`/`rust` green e à captura visual do HUD (passo 13) · **Âncora:** `origin/dev = 1855e5c`, branch `feature/058-mirror-vitoria-local`, constituição v1.2.2.

**Aviso de valor:** GDD:275 permanece `[HIPÓTESE]` — 3 nós, 100 de Resistência Ideológica, −30 por nó, pulso final de −10 e `TICKS_PROPAGANDA_NO = 600` (20 s × 30 Hz) são protótipo, **nunca cânone**. O GDD não é reescrito e nenhuma regra de vitória é criada. **BF-003a (espelho local de vitória) é fechado neste ciclo; BF-003b (mapa completo de 12 ilhas, topologia, spawning e rotas) permanece aberto.** Score **270/3125** (`5×3×3×2×3`). Nenhuma implementação, build, gate, teste, formatador, commit, push, PR, merge, deploy, issue ou promoção foi executado por este agente de documentação.

## Changelog

### [1.0.0-local] — 2026-11-24

> Convenção de IDs: **T001..T010** é a cadeia SDD canônica de `specs/058-mirror-vitoria-local/tasks.md` (T010 agrupa implement → converge → validate → release). Os T001..T012 do `assessment` são rótulos de rastro preliminares e não renomeiam esta cadeia.

### Added
- **FR-001 / T001, T002, T004, T005, T010** — âncora, decisão e helper determinístico: `pub const POSICOES_NOS_LOCAIS: [(u64, (f64, f64)); 3]` (`crates/sim/src/lib.rs:567-568`) com `(1, (0.5, 0.0))`, `(2, (1.5, 0.0))`, `(3, (2.5, 0.0))` e `pub fn semear_espelho_local(&mut Mundo)` (`lib.rs:577-585`), ambos documentados como `[HIPÓTESE]`/protótipo. O helper semeia 4 ilhas (`1..=4`) via `adicionar_ilha`, 1 súdito (id `1`, `(0.0, 236.0)`) via `adicionar_sudito` e 3 nós nas posições constantes via `adicionar_no`, usando apenas `BTreeMap` determinístico e nenhuma estrutura de estado, RNG, relógio, rede, contrato ou protocolo novo. Decisão 80/20 **270/3125** (5×3×3×2×3), R1 `go`, DoD-1/DoD-3 em `spec.md`.
- **FR-002 / T005, T010** — ligação com o consumidor: `insurgencia_sim::semear_espelho_local(&mut sim)` em `configurar` (`crates/cliente/src/main.rs:597`), após jogadores/PNJs e antes de `commands.insert_resource(Simulacao(sim))`. O espelho passa a existir no tick 0 sem janela, campo de protocolo, comando de teclado, HUD novo ou semeadura duplicada. Renderização, câmera, ilhas na cena, avatar e HUD permanecem intactos; a sim continua sendo a única fonte de verdade. A observação visual da HUD (`Ilhas`, `Sinais`, `Nós`, `Resultado` e "Autogestão estabelecida") é **[NV]** até o passo 13 (ver `evidence.md`).
- **FR-003 / T005, T010** — teste de regressão: `espelho_local_seed_viabiliza_vitoria_e_reflete_hash` (`crates/sim/tests/simulacao.rs:4233`) verifica o `Mundo` vazio, as contagens 4/1/3, a mudança de `hash_estado()` após a semente, agenda `propaganda_no` nos três nós e avança `TICKS_PROPAGANDA_NO + 1` ticks até `ResultadoPartida::Vitoria` com `resistencia_ideologica == 0`. `tres_nos_geram_vitoria` (`simulacao.rs:2558`) e os demais testes são preservados. Saída observada da suíte: **`103 passed (1 suite, 0.28s)`**.
- **FR-004 / T001, T005, T010** — evidência e governança: roteiro de **14 passos** escrito antes do PR (`.specify/validations/mirror-vitoria-local/test-roadmap.md`), tabela de saídas em `.specify/validations/mirror-vitoria-local/evidence.md`, validação R4 condicionada (`.specify/validations/mirror-vitoria-local/validate.md`), este release e `specs/058-mirror-vitoria-local/converge.md` como **1ª entrada append-only** da cadeia 058. Gates observados: `work-order-dag PASS (57 files, 622 tasks, 302 edges, 0 cycles)`, `task-id PASS (57, 0 duplicates)`, `converge PASS (39 baselined/39)`, `hub-docs PASS (0 errors, 0 warnings)`; `check_sdd_completeness` falhava **apenas** pela ausência deste `release.md`. Outras saídas: `cargo test --locked -p insurgencia-servidor --test cli` → **`10 passed (1 suite, 0.11s)`**; `cargo check --locked -p insurgencia-sim --tests` → **`Finished dev profile ... in 14.09s`**. BF-003a **fechado**, BF-003b **aberto**, **R6 N/A** sob v1.2.2 (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado).

### Changed
- Nenhuma mudança em regras canônicas, GDD, baseline, contratos, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, CI, manifesto, `AGENTS.md`, stack ou rede. Nenhum número prototípico promovido a cânone e nenhum ciclo anterior reaberto.

## Rastreabilidade

FR-001..FR-004 · SC-001..SC-004 (1:1) · T001..T010 (cadeia canônica: `tasks` T008 → `analyze` READ-ONLY T009 → implement/converge/validate/release T010) · R1 `go` · R4 aprovado **condicionado** e **antes** de R7 · score **270/3125** (`5×3×3×2×3`) · roteiro 14 passos · constituição **v1.2.2** · âncoras: `docs/GDD - Insurgência.md:275`, `crates/sim/src/lib.rs:567-585`, `crates/cliente/src/main.rs:597`, `crates/sim/tests/simulacao.rs:2558,4233`.

## Rollback

Para reverter o ciclo 058, **reverter o único commit de feature** (`git revert <sha-do-commit-de-feature>`), devolvendo o worktree a `origin/dev = 1855e5c`; os testes e gates do baseline voltam a valer. **Não** executar `git clean`, `git reset --hard`, force-push, remoção forçada de artefatos do ciclo, merge de PR ou deploy. O rollback está documentado, **não executado**; a execução é SWE-exclusiva.

**Nenhum merge `dev → main` e nenhum deploy foram realizados.** A versão `1.0.0-local` não foi promovida a `1.0.0`, não saiu deste worktree e não autoriza publicação. PR #76 pertence a outro owner e permanece intocado.

## Não incluído

Mapa completo de 12 ilhas/topologia/spawning/rotas (BF-003b), promoção de `[HIPÓTESE]` a cânone, duplicação da semente no cliente, mudanças em contrato, protocolo, Cargo, CI, GDD, baseline, `AGENTS.md`, `concurrency.jsonl`, `STATE.md` fora da nota append-only do ciclo, PR #76 ou código de outros ciclos; checks remotos, `./verify.sh`, captura visual da HUD, reexecução de gates/formatadores, acionamento de R6, abertura de issue, merge, promoção `dev→main`, deploy ou execução do rollback por este agente.
