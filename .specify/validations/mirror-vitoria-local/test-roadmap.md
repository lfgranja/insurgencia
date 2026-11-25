# Roteiro de Testes — Espelho local de vitória (ciclo 058, BF-003a)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador/SWE · **Evidências aqui**: **[NV]** (nenhum teste, gate, formatador, build, commit, push ou PR executado por este agente)

## Escopo e rastreabilidade

| Requisito | Definição do ciclo | Passos |
|---|---|---|
| FR-001 | Helper/constantes no sim: `semear_espelho_local` e as constantes da semente em `crates/sim` — 4 ilhas (ids `1..=4`), 1 súdito (id `1` em `(0.0, 236.0)`) e 3 nós, valores `[HIPÓTESE]` (GDD:275), sem promover derivação a regra canônica | 1, 2 |
| FR-002 | Chamada no cliente: `insurgencia_sim::semear_espelho_local(&mut sim)` na inicialização do cliente Bevy e HUD coerente (linhas `Ilhas`, `Sinais`, `Nós`, `Resultado`; "Autogestão estabelecida" após a conversão dos 3 nós) | 4, 13 |
| FR-003 | Teste de regressão da sim: `espelho_local_seed_viabiliza_vitoria_e_reflete_hash` e `tres_nos_geram_vitoria` cobrindo semente, vitória por `propaganda_no` e `hash_estado` determinístico | 2, 3 |
| FR-004 | Evidência/validação/release: gates, `./verify.sh`, checks remotos, capturas, **BF-003a fechada**, **BF-003b** (mapa completo de 12 ilhas) aberta, R6 **N/A** (constituição v1.2.2) e rollback = revert do único commit de feature | 1, 5–14 |

T-IDs no escopo: **T001–T010** (cadeia SDD do ciclo 058 — constituição/branch, intake/research, define/shape, decide/clarify, specify, plan, checklist, revisão, tasks, analyze/implementação/converge/validate). R6/taskstoissues não pertence à cadeia: é **N/A** por constituição v1.2.2 — PARE se invocada, **nenhuma issue simulada**. `dev → main` e deploy são exclusivos do SWE; o PR #76 pertence a outro owner e não é tocado neste roteiro.

## Pré-condições

1. Worktree `.specify/queue/worktrees/impl-058-mirror-vitoria/`, branch `feature/058-mirror-vitoria-local`, base `origin/dev = 1855e5c` (`1855e5ccad79d82d341481c4374bf4eca203f5f1`). HEAD deve conferir com a base antes da aplicação do único commit de feature; `git status --short` sem alterações alheias ao ciclo.
2. Toolchain Rust fixada em `rust-toolchain.toml`, `python3` e `gh` disponíveis. `TMP=$(mktemp -d)` fora do worktree; `mkdir -p "$TMP/saidas" "$TMP/saidas/gates"`.
3. Consumer = jogador/cliente Bevy + testes de integração da sim; nenhuma mudança em contratos, `PROTOCOL_VERSION`, GDD, baseline, Cargo, rede ou CI como fonte alterada.
4. Nenhum teste, gate, build, formatador, commit, push, PR, merge ou deploy executado por este agente; os passos abaixo são instruções ao orquestrador/SWE e toda saída é **[NV]** até execução ao vivo.

## Superfície

- Terminal local: `git`, `cargo test`, `./verify.sh`, `python3 scripts/ci/*.py`, `cargo fmt --all -- --check`.
- Remoto **[NV]**: GitHub Actions (`gates`, `rust`) no head do PR, via `gh`/fonte autorizada.
- Gráfica **[NV]**: janela do cliente Bevy apenas no passo 13 — não automatizável localmente (justificativa abaixo).

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Superfície | Automatizável? | Captura |
|:--|:--|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD origin/dev` e `git branch --show-current`, com `date -u` no worktree | HEAD/base = `1855e5c`, branch = `feature/058-mirror-vitoria-local`, único commit de feature pendente; FR-004: BF-003a fechada e BF-003b aberta nos artefatos do ciclo | terminal / git | sim | `$TMP/saidas/ancora.txt` **[NV]** |
| 2 | `cargo test --locked -p insurgencia-sim --test simulacao espelho_local_seed_viabiliza_vitoria_e_reflete_hash -- --exact` | `test ... ok`, `1 passed; 0 failed`; 4 ilhas (`1..=4`), 1 súdito (id `1` em `(0.0, 236.0)`), 3 nós e `hash_estado` alterado de forma determinística | cargo test (sim) | sim | `$TMP/saidas/sim-seed.txt` **[NV]** |
| 3 | `cargo test --locked -p insurgencia-sim --test simulacao tres_nos_geram_vitoria -- --exact` | `test ... ok`, `1 passed; 0 failed`; 3 nós convertidos por `propaganda_no`, resistência `0`, `ResultadoPartida::Vitoria`, estável após ticks extras | cargo test (sim) | sim | `$TMP/saidas/sim-vitoria.txt` **[NV]** |
| 4 | `cargo test --locked -p insurgencia-servidor --test cli` | suíte verde, nenhuma regressão no consumidor servidor; protocolo inalterado | cargo test (servidor) | sim | `$TMP/saidas/cli.txt` **[NV]** |
| 5 | `./verify.sh` | `ALL STAGES OK`; estágios `check`, `clippy`, `test`, `fmt` sem falha | terminal / verify.sh | sim | `$TMP/saidas/verify.txt` **[NV]** |
| 6 | `python3 scripts/ci/check_sdd_completeness.py` | exit `0`, nenhuma lacuna SDD nos artefatos do ciclo; contagens emitidas registradas | python3 / gates | sim | `$TMP/saidas/gates/sdd.txt` **[NV]** |
| 7 | `python3 scripts/ci/check_work_order_dag.py` | exit `0`, DAG sem ciclo, auto-aresta ou órfã; R6 ausente da cadeia | python3 / gates | sim | `$TMP/saidas/gates/dag.txt` **[NV]** |
| 8 | `python3 scripts/ci/check_task_id_uniqueness.py` | exit `0`, T001–T010 com IDs únicos | python3 / gates | sim | `$TMP/saidas/gates/task-id.txt` **[NV]** |
| 9 | `python3 scripts/ci/check_converge_append_only.py` | exit `0`, `converge.md` do ciclo 058 acrescentado sem reescrita de artefatos alheios | python3 / gates | sim | `$TMP/saidas/gates/converge.txt` **[NV]** |
| 10 | `python3 scripts/ci/hub_docs_gate.py` | exit `0`, documentos-hub sem deriva introduzida pelo ciclo | python3 / gates | sim | `$TMP/saidas/gates/hub-docs.txt` **[NV]** |
| 11 | `cargo fmt --all -- --check` | exit `0`, nenhum diff de formatação | terminal / cargo fmt | sim | `$TMP/saidas/fmt.txt` **[NV]** |
| 12 | **Checks remotos [NV]**: no head do PR, conferir via `gh pr checks`/`gh run view` (fonte autorizada) os jobs `gates` e `rust` no SHA final | ambos `gates` e `rust` green no SHA do head; `gates` com os cinco scripts Python, `rust` com clippy `-D warnings` e suíte de testes; ACR sem blockers; registrar run ID e carimbo | GitHub Actions / gh (remoto) | parcial (consulta automatizada; execução remota depende do CI) — **[NV]** | `$TMP/saidas/remote-checks.txt` **[NV]** |
| 13 | **Captura visual do HUD Bevy [NV]**: iniciar o cliente e observar a semente local (ilhas, nós, súdito; linhas `Ilhas`, `Sinais`, `Nós`, `Resultado`) e a mensagem "Autogestão estabelecida" após a conversão dos 3 nós | HUD coerente com a semente e com a vitória local; screenshot + carimbo anexados a `.specify/validations/mirror-vitoria-local/evidence.md` | GUI / cliente Bevy | **não (local)** — **[NV]** | `$TMP/saidas/hud-visual.md` + screenshot **[NV]** |
| 14 | **Rollback [NV]**: reverter o único commit de feature do worktree, devolvendo HEAD a `1855e5c` | HEAD = base `1855e5c`, testes e gates no baseline; nenhum artefato do ciclo removido à força | terminal / git | sim (instrução; não executada por este agente) | `$TMP/saidas/rollback.txt` **[NV]** |

## Plano de capturas

- `$TMP/saidas/ancora.txt` (passo 1); `sim-seed.txt` e `sim-vitoria.txt` (passos 2–3); `cli.txt` (passo 4); `verify.txt` (passo 5).
- `$TMP/saidas/gates/{sdd,dag,task-id,converge,hub-docs}.txt` (passos 6–10); `fmt.txt` (passo 11).
- `$TMP/saidas/remote-checks.txt` (passo 12) e `$TMP/saidas/hud-visual.md` + screenshot (passo 13) — **[NV]**.
- `$TMP/saidas/rollback.txt` (passo 14). Toda captura permanece rotulada **[NV]** até o orquestrador/SWE executar o passo correspondente; nenhuma captura existe neste ciclo.

## Itens [NV] e escalonamento

- **Passo 12 — checks remotos [NV]**: `gates`/`rust` rodam no GitHub Actions sobre o head do PR; somente o SWE confirma o SHA final e o estado do run. Não reexecutar localmente nem afirmar verde sem a leitura remota.
- **Passo 13 — captura visual do HUD Bevy [NV]**: **não automatizável localmente**. Justificativa: a compilação do cliente Bevy excedeu o timeout de **900 s (15 min)** em tentativa local anterior, e este ambiente não dispõe de janela gráfica automatizável para a inspeção. **Escalonamento**: o SWE/orquestrador deve executar o cliente em máquina com X11/GPU, anexar screenshot e carimbo a `.specify/validations/mirror-vitoria-local/evidence.md`; até lá o item permanece **[NV]**: a evidência visual de FR-002 (chamada no cliente/HUD) fica pendente e o comportamento é coberto indiretamente pelos testes de regressão da sim (FR-003, passos 2–3).

## Rollback do roteiro

`rm -rf "$TMP"` remove capturas e arquivos de apoio fora do worktree. Se algum passo falhar, registrar a saída obtida e corrigir a **fonte** (código/artefato do ciclo), nunca a expectativa do teste. Para reverter a mudança, **reverter o único commit de feature** (`git revert <sha-do-commit-de-feature>`), devolvendo o worktree a `1855e5c`. Não executar `git clean`, `git reset --hard`, force-push, remoção de artefatos do ciclo, merge de PR, deploy, nem reexecução de gates por este agente; `dev → main` e deploy permanecem SWE-exclusivos.
