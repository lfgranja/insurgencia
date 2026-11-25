# Roteiro de Testes — Sincronia cooperativa (ciclo 060, BF-015a)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador/SWE · **Evidências aqui**: **[NV]** (nenhum teste, gate, formatador, build, compilação, commit, push, PR, merge, deploy ou issue executado por este agente; os 14 passos abaixo são instruções, nunca resultados observados).

**Aviso de contrato**: decisão 80/20 **360/3125** (`5×4×3×2×3`, impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵) registrada em `.specify/memory/STATE.md`, com consumidor = **jogador do cliente Bevy (tecla `Space`) + automação headless via CLI**. Contrato fixo, não rediscutido neste roteiro: `sinalizar_sincronia` já está em `ACOES_PERMITIDAS`, é sem destino e nível 1, mas não possui ramo em `fase_skills`. A sim acrescenta `pub const TICKS_JANELA_SINCRONIA: u64 = 150` (5 s @ 30 Hz) e os campos públicos `Mundo.sincronias_iniciadas: u64`, `Mundo.sinalizadores_sincronia: BTreeSet<u64>` e `Mundo.sincronia_ticks_restantes: u64`, zerados/vazios em `Mundo::new`. `avancar_tick` decrementa a janela uma vez por tick; o primeiro sinal com janela zerada a abre, e três peers distintos na mesma janela incrementam `sincronias_iniciadas`, enviam `sincronia_iniciada:<tick>` para `sinais` e reiniciam janela e conjunto. `hash_estado` cobre contador, ticks restantes e IDs dos sinalizadores em ordem. No cliente, `Space` agenda `sinalizar_sincronia` com IDs monotônicos, `configurar` semeia um terceiro peer local e o HUD passa a mostrar `Sincronias: N`. Todos os números são **[HIPÓTESE]** (GDD §8/§12). Nenhuma topologia de ilha, dano de gás, regra canônica, contrato, `PROTOCOL_VERSION`, Cargo, CI, stack, rede ou GDD é alterado. **R6/taskstoissues é N/A** (constituição v1.2.2) e o PR #76 pertence a outro owner — nenhum dos dois é tocado neste roteiro.

## Escopo e rastreabilidade

| Requisito | Definição do ciclo | Passos |
|---|---|---|
| FR-001 | Estado do `Mundo` + janela/hash no sim: `TICKS_JANELA_SINCRONIA = 150` (5 s @ 30 Hz) rotulado `[HIPÓTESE]`, três campos públicos em `Mundo` zerados/vazios em `Mundo::new`; ramo de `sinalizar_sincronia` em `fase_skills` e decremento em `avancar_tick`, primeiro sinal com janela zerada a abre, três peers distintos emitem `sincronia_iniciada:<tick>` e resetam, sem dupla contagem; `hash_estado` cobre contador, janela e IDs ordenados; sem topologia ou dano | 2, 3, 4, 5 |
| FR-002 | Cliente (Espaço/terceiro peer/HUD/CLI): tecla `Space` agenda `sinalizar_sincronia` com `seq`/`request_id` monotônicos, `configurar` semeia terceiro peer local e `texto_hud` mostra `Sincronias: N` derivado da sim; automação headless via CLI preserva aridade, parser e saída `<tick>\t<sha256>`, observando o sinal sem alterar protocolo | 4, 6, 7, 8, 13 |
| FR-003 | Testes de regressão da sim: suíte `--test simulacao` cobre inicialização zerada, abertura da janela, três peers distintos, signal, reset, deduplicação, sinal fora da janela e `hash_estado` determinístico com IDs ordenados, sem reescrever testes anteriores nem afrouxar expectativas | 5, 6 |
| FR-004 | Evidência, validação e release: cinco gates Python, `./verify.sh`, clippy/fmt, checks remotos `gates`/`rust`, capturas, **BF-015a fechada**, R6 **N/A**, PR #76 intocável e rollback = revert do único commit de feature | 1, 9, 10, 11, 12, 14 |

T-IDs no escopo: **T001–T010** (cadeia SDD do ciclo 060 — constituição/branch, intake/research, define/shape, decide/clarify, specify, plan, checklist, revisão, tasks, analyze/implementação/converge/validate). R6/taskstoissues não pertence à cadeia: é **N/A** por constituição v1.2.2 — PARE se invocada, **nenhuma issue simulada** e nenhum `gh` improvisado. Promoção `dev → main` e deploy são exclusivos do SWE; o PR #76 pertence a outro owner e **não é tocado** neste roteiro.

## Pré-condições

1. Worktree `.specify/queue/worktrees/cycle-060/`, branch `feature/060-sincronia-cooperativa`, base `origin/dev = e0e8f38`. HEAD deve conferir com a base antes da aplicação do único commit de feature; `git status --short` sem alterações alheias ao ciclo.
2. Toolchain Rust fixada em `rust-toolchain.toml`, `python3` e `gh` disponíveis. `TMP=$(mktemp -d)` fora do worktree; `mkdir -p "$TMP/saidas" "$TMP/saidas/gates"`.
3. Consumer = jogador do cliente Bevy (`Space`) e automação headless via CLI. A implementação de referência é handoff ao SWE/orquestrador, restrita à sim e ao cliente; contratos, `PROTOCOL_VERSION`, GDD, baseline, Cargo, CI, stack e rede não são fontes alteradas.
4. Nenhum teste, gate, build, formatador, commit, push, PR, merge, deploy ou issue executado por este agente; toda saída dos passos permanece **[NV]** até execução ao vivo pelo orquestrador/SWE.

## Superfície

- Terminal local: `git`, `printf`/`cargo run` (sonda CLI), `cargo test`, `cargo check`, `cargo clippy`, `cargo fmt`, `./verify.sh`, cinco scripts `python3 scripts/ci/*.py`.
- Remoto **[NV]**: GitHub Actions (`gates`, `rust`) no head do PR, via `gh`/fonte autorizada.
- Gráfica **[NV]**: janela do cliente Bevy apenas no passo 13 — não automatizável localmente (justificativa abaixo).

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Superfície | Automatizável? | Captura |
|:--|:--|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD origin/dev`, `git branch --show-current` e `date -u`; listar os artefatos do ciclo e conferir a decisão **360/3125** e a rastreabilidade FR-001..FR-004 ↔ T001..T010 em `.specify/memory/STATE.md` | HEAD/base = `e0e8f38`, branch = `feature/060-sincronia-cooperativa`, único commit de feature pendente; BF-015a ainda aberta antes da implementação, PR #76 intocável; nenhum passo executado por este agente | terminal / git | sim | `$TMP/saidas/ancora.txt` **[NV]** |
| 2 | Inspecionar estado e hash: `grep -nE 'TICKS_JANELA_SINCRONIA\|sincronias_iniciadas\|sinalizadores_sincronia\|sincronia_ticks_restantes\|fn hash_estado' crates/sim/src/lib.rs` | `TICKS_JANELA_SINCRONIA: u64 = 150` (5 s @ 30 Hz) documentado e rotulado `[HIPÓTESE]`; os três campos são públicos em `Mundo` e zerados/vazios em `Mundo::new`; `hash_estado` cobre contador, ticks restantes e IDs ordenados dos sinalizadores; nenhum campo de ilha, gás ou dano acrescentado | inspeção estática / sim | sim | `$TMP/saidas/sim-estado.txt` **[NV]** |
| 3 | Inspecionar o ciclo da janela: `grep -nE 'sinalizar_sincronia\|fase_skills\|avancar_tick\|sincronia_iniciada\|sinais' crates/sim/src/lib.rs` | `sinalizar_sincronia` permanece sem destino e nível 1 em `ACOES_PERMITIDAS`/`ValidacaoComandos`; ramo novo em `fase_skills`; `avancar_tick` decrementa a janela uma vez por tick; primeiro sinal com janela zerada a abre em 150 ticks; três peers distintos na mesma janela incrementam `sincronias_iniciadas` **uma** vez, emitem `sincronia_iniciada:<tick>` em `sinais` e reiniciam janela e conjunto; peer repetido não conta duas vezes | inspeção estática / sim | sim (dinâmica nos passos 4–5) | `$TMP/saidas/sim-janela.txt` **[NV]** |
| 4 | **Sonda CLI**: `printf '1 sinalizar_sincronia 1 1\n2 sinalizar_sincronia 1 2\n3 sinalizar_sincronia 1 3\n' \| cargo run --locked -p insurgencia-servidor -- --ticks 2` | stdout com apenas linhas `<tick>\t<sha256>` (64 hex minúsculos), uma por tick, impressas antes do avanço do tick; stderr vazia e exit `0`; formato `<tick>\t<hash>`, hash inicial e determinismo do CLI preservados. A string `sincronia_iniciada:<tick>` **não** é afirmada pela stdout do CLI: ela é assertada pelo teste de regressão da sim (passo 5, `--test simulacao`) | CLI / cargo run | sim | `$TMP/saidas/cli-probe.txt` **[NV]** |
| 5 | `cargo test --locked -p insurgencia-sim --test simulacao` | Suíte verde, `0 failed`; testes existentes (`fases_prepare_skills_finish`, `movimento_aplica_destino`, `replay_de_seq_e_request_id`, `acao_desconhecida`, `limite_de_movimento`, `peer_nao_autorizado`) preservados; novos casos cobrindo janela de 150 ticks, três peers distintos, reset e `hash_estado` determinístico; nenhum teste relaxado | cargo test (sim) | sim | `$TMP/saidas/simulacao.txt` **[NV]** |
| 6 | `cargo test --locked -p insurgencia-servidor --test cli` | Suíte verde, `0 failed`; consumidor headless sem regressão; whitelist, validação e protocolo inalterados | cargo test (servidor) | sim | `$TMP/saidas/cli.txt` **[NV]** |
| 7 | `cargo test --locked -p insurgencia-cliente --lib` | Suíte verde, `0 failed`; helpers e estado do cliente sem regressão; nenhum teste enfraquecido | cargo test (lib do cliente) | sim | `$TMP/saidas/cliente-lib.txt` **[NV]** |
| 8 | `cargo check --locked -p insurgencia-cliente --all-targets` | EXIT `0`; binário, lib e demais targets compilam; `Space`, terceiro peer semeado e `Sincronias: N` presentes no código; sem `#[allow]`, `unwrap()` ou `.expect()` novos | cargo check | sim | `$TMP/saidas/check-cliente.txt` **[NV]** |
| 9 | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | EXIT `0`, nenhum aviso no workspace; convenções `#![deny(clippy::pedantic)]` preservadas, sem `allow` novo | cargo clippy | sim | `$TMP/saidas/clippy.txt` **[NV]** |
| 10 | `cargo fmt --all -- --check` | EXIT `0`, nenhum diff de formatação | cargo fmt | sim | `$TMP/saidas/fmt.txt` **[NV]** |
| 11 | Gates locais: `./verify.sh`; `python3 scripts/ci/check_sdd_completeness.py`; `python3 scripts/ci/check_work_order_dag.py`; `python3 scripts/ci/check_task_id_uniqueness.py`; `python3 scripts/ci/check_converge_append_only.py`; `python3 scripts/ci/hub_docs_gate.py` | `ALL STAGES OK`; os cinco scripts Python encerram com exit `0`: SDD sem lacunas em FR-001..FR-004/T001..T010 e BF-015a fechada, DAG sem ciclo/auto-aresta/órfã, T001–T010 sem duplicatas, `converge.md` append-only e hub-docs sem deriva | terminal / verify.sh + python3 / gates | sim | `$TMP/saidas/verify.txt` e `$TMP/saidas/gates/{sdd,dag,task-id,converge,hub-docs}.txt` **[NV]** |
| 12 | **Checks remotos [NV]**: no head do PR, conferir via `gh pr checks`/`gh run view` (fonte autorizada) os jobs `gates` e `rust` no SHA final | Ambos `gates` e `rust` green no SHA do head; `gates` executando os cinco scripts Python, `rust` com clippy `-D warnings` e as suítes de teste; ACR sem blockers; registrar run ID e carimbo | GitHub Actions / gh (remoto) | parcial (consulta automatizada; execução remota depende do CI) — **[NV]** | `$TMP/saidas/remote-checks.txt` **[NV]** |
| 13 | **Captura visual do HUD Bevy [NV]**: iniciar o cliente em máquina com X11/GPU (pelo SWE; este agente não abre janela) e observar `Sincronias: N` após três pressões de `Space` com o terceiro peer local semeado | Linha `Sincronias: N` avança de `0` para `1` somente após três sinais de peers distintos na mesma janela; IDs monotônicos sem dupla contagem; screenshot + carimbo anexados a `.specify/validations/sincronia-cooperativa/evidence.md` — permanece **[NV]** | GUI / cliente Bevy | **não (local)** — **[NV]** | `$TMP/saidas/hud-visual.md` + screenshot **[NV]** |
| 14 | **Rollback [NV]**: reverter o único commit de feature do worktree, devolvendo HEAD a `e0e8f38` | HEAD = base `e0e8f38`, testes e gates no baseline; nenhum artefato do ciclo removido à força; rollback documentado, **não executado** por este agente | terminal / git | sim (instrução; não executada por este agente) | `$TMP/saidas/rollback.txt` **[NV]** |

**Sonda literal do passo 4** (o `|` é escapado na tabela apenas para renderização Markdown; o comando executado é este):

```sh
printf '1 sinalizar_sincronia 1 1\n2 sinalizar_sincronia 1 2\n3 sinalizar_sincronia 1 3\n' | cargo run --locked -p insurgencia-servidor -- --ticks 2
```

## Plano de capturas

- `$TMP/saidas/ancora.txt` (passo 1); `sim-estado.txt` e `sim-janela.txt` (passos 2–3); `cli-probe.txt` (passo 4).
- `$TMP/saidas/{simulacao,cli,cliente-lib,check-cliente,clippy,fmt}.txt` (passos 5–10); `$TMP/saidas/verify.txt` e `$TMP/saidas/gates/{sdd,dag,task-id,converge,hub-docs}.txt` (passo 11).
- `$TMP/saidas/remote-checks.txt` (passo 12), `$TMP/saidas/hud-visual.md` + screenshot (passo 13) e `$TMP/saidas/rollback.txt` (passo 14) — **[NV]**.
- Toda captura permanece rotulada **[NV]** até o orquestrador/SWE executar o passo correspondente; nenhuma captura existe neste ciclo.

## Itens [NV] e escalonamento

- **Passo 12 — checks remotos [NV]**: `gates`/`rust` rodam no GitHub Actions sobre o head do PR; somente o SWE confirma o SHA final e o estado do run. Não reexecutar localmente nem afirmar verde sem a leitura remota.
- **Passo 13 — captura visual do HUD Bevy [NV]**: **não automatizável localmente**. Justificativa: este ambiente não dispõe de janela gráfica automatizável para inspeção e há precedente documentado no ciclo 058 de compilação do cliente Bevy excedendo **900 s (15 min)** em tentativa local; a verificação local é estática e indireta (passos 3, 4, 7 e 8). **Escalonamento**: o SWE/orquestrador executa o cliente em máquina com X11/GPU, anexa screenshot e carimbo a `.specify/validations/sincronia-cooperativa/evidence.md`; até lá o item permanece **[NV]** e a evidência visual de FR-003 fica pendente.
- **Promoção e backlog [NV]**: `dev → main`, deploy e o PR #76 (outro owner) são SWE-exclusivos; BF-015a só é fechada nos artefatos após a execução validada deste roteiro; R6 permanece **N/A** por constituição v1.2.2.

## Rollback do roteiro

`rm -rf "$TMP"` remove capturas e arquivos de apoio fora do worktree. Se algum passo falhar, registrar a saída obtida e corrigir a **fonte** (código/artefato do ciclo), nunca a expectativa do teste — em especial janela promovida a regra canônica ou valores não rotulados `[HIPÓTESE]` (passos 2–3), contagem dupla de peer (passos 3–4), formato/hash do CLI alterados (passos 4–6) e HUD com contador derivado de estado local (passos 8 e 13). Para reverter a mudança, **reverter o único commit de feature** (`git revert <sha-do-commit-de-feature>`), devolvendo o worktree a `e0e8f38`. Não executar `git clean`, `git reset --hard`, force-push, remoção de artefatos do ciclo, merge de PR, deploy, abertura de issue ou reexecução de gates por este agente; `dev → main` e deploy permanecem SWE-exclusivos.
