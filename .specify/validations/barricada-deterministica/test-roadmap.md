# Roteiro de Testes — Construção determinística de barricada (ciclo 040)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador/SWE · **Evidências aqui**: **[NV]** (nada executado neste ciclo pelo agente SDD)

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passos 2–6, 9 — API Rust: constantes públicas `CUSTO_BARREIRA_MATERIAIS = 20`, `TICKS_CONSTRUCAO_BARREIRA = 45`, `PV_BARREIRA = 200`, `TICKS_DURACAO_BARREIRA = 360`, `MAX_BARREIRAS_POR_JOGADOR = 2`; campos `construcao_ticks`/`construcao_ponto` em `Jogador`, `barreiras`/`proxima_barreira_id` em `Mundo`, struct `Barreira`, `CodigoRecusa::SaldoInsuficiente`, `Comando.destino`; nenhuma mudança em `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml` |
| FR-002 | passos 2, 3 — `construir_barreira` exige `destino == posicao` (via `ponto_igual`) e `materiais >= 20`; recusa com `CodigoRecusa::SaldoInsuficiente` sem consumo parcial e sem iniciar canalização |
| FR-003 | passos 2, 3, 8 — `fase_skills` revalida o saldo ao completar, avança no máx. 1 canalização por jogador por tick (`BTreeSet<u64>` local), zera `construcao_ticks`/`construcao_ponto` se `mover` mudou a posição, debita 20 Materiais atomicamente na conclusão e insere a `Barreira` |
| FR-004 | passos 4–6 — limite de 2 barricadas por jogador; a 3.ª remove a mais antiga (menor `id`); `Barreira` nasce com `pv = 200` e `ticks_restantes = 360` e é removida por PV zero ou duração |
| FR-005 | passo 7 — `hash_estado` inclui `construcao_ticks`/`construcao_ponto` de cada jogador e todas as barreiras em ordem de `BTreeMap` |
| FR-006 | passo 9 — tecla `B` em *just pressed* agenda `construir_barreira` com `destino` = posição atual e `seq`/`request_id` monotônicos; HUD Materiais/Barreiras/Construindo; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, commits PT-BR |
| Governança (fora dos FR) | passos 1, 10 — 14 artefatos na fronteira exata, constituição v1.2.2, `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`, `converge` append-only, **R6 N/A** com dívida de automação, rollback e rótulo **[NV]** |
| SC-001..SC-006 | passos 1–10 |

T-IDs relacionados: T001–T013 (SDD, com `tasks` em T012 e `analyze` READ-ONLY em T013) e T014 (`test-roadmap`, `converge`, R4 e R7). A implementação em `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs` e `crates/cliente/src/main.rs` é handoff ao SWE/orquestrador, fora da fronteira SDD deste ciclo e **[NV]** até execução.

## Pré-condições
1. Worktree `.specify/queue/worktrees/barricada-040/`, branch `040-barricada-deterministica`, base `6b913f0`; sessão `20261123T024300Z-barricada-deterministica`. Artefatos SDD presentes na fronteira exata, com `tasks.md` (T012) antes de `analyze.md` (T013, READ-ONLY).
2. Implementação de referência em `crates/sim/src/lib.rs`, testes em `crates/sim/tests/simulacao.rs` e cliente em `crates/cliente/src/main.rs` prontos pelo SWE: constantes, campos, validação, `fase_skills`, `hash_estado`, tecla `B` e HUD; nenhum `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml` alterado; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`.
3. `TMP=$(mktemp -d)` fora do worktree para capturas; `mkdir -p "$TMP/saidas"`. Nenhum gate, build, commit, push, PR, merge ou deploy executado por este agente — os passos abaixo são instruções ao orquestrador/SWE.

## Superfície documental/CLI
- Inspeção: `git rev-parse`/`git status --short`, `find`, `grep`, `diff`, `cat`, `date -u`, leitura de fontes (GDD §9.3/§10.3, `lib.rs`, `simulacao.rs`, `main.rs`, constituição v1.2.2).
- Testes: `cargo test -p sim` e, se aplicável, `cargo test -p cliente` (ou o comando canônico do hub), executados **pelo orquestrador**, com saída redirecionada para `$TMP/saidas`; nenhuma janela gráfica, rede ou credencial. Tecla `B` simulada apenas por teste automatizado de entrada, nunca por janela real.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD` e `date -u > "$TMP/saidas/carimbo.txt"`; listar os 14 artefatos e conferir `tasks` antes de `analyze` > `"$TMP/saidas/inventario.txt"` | base `6b913f0`, branch `040-barricada-deterministica`, 14 artefatos na fronteira; `.specify/memory/STATE.md`, `concurrency.jsonl` e paths alheios intactos | sim |
| 2 | **Caso 45 ticks e +20→0**: adicionar jogador com 20 Materiais, agendar `construir_barreira` com `destino == posicao`, avançar 45 ticks com o jogador parado e registrar saldo/`construcao_ticks`/`barreiras` a cada tick em `"$TMP/saidas/caso-45-ticks.txt"` | nenhuma barreira ou débito no tick 44; no tick 45 débito atômico de exatamente 20 (20→0), `construcao_ticks`/`construcao_ponto` zerados e `Barreira` com `pv = 200` e `ticks_restantes = 360` inserida | sim |
| 3 | **Caso saldo 19 recusado e destino inválido**: chamar `construir_barreira` com 19 Materiais e com `destino != posicao`; avançar ticks e capturar em `"$TMP/saidas/saldo-insuficiente.txt"` | `Err(CodigoRecusa::SaldoInsuficiente)` para 19, sem débito, sem barreira e sem canalização; `destino != posicao` recusado por `ponto_igual` com o estado inalterado — nenhum consumo parcial em nenhum caso | sim |
| 4 | **Caso 200 PV**: com uma barreira ativa, aplicar dano registrado até zerar `pv` e capturar em `"$TMP/saidas/pv-200.txt"` | `PV_BARREIRA = 200` no tick de construção; a barreira é removida de `barreiras` somente quando `pv` chega a 0, sem afetar as demais nem o saldo | sim |
| 5 | **Caso 360 ticks**: construir, avançar 359 ticks e depois o 360.º, registrando `ticks_restantes` em `"$TMP/saidas/duracao-360.txt"` | `TICKS_DURACAO_BARREIRA = 360`; `ticks_restantes` decresce um por tick, a barreira permanece no tick 359 e é removida no tick 360, sem reembolso de Materiais | sim |
| 6 | **Caso 3.ª barricada**: com 2 barricadas ativas de `dono` distinto, construir uma 3.ª pelo mesmo jogador e inspecionar `barreiras`/`proxima_barreira_id` em `"$TMP/saidas/tres-barricadas.txt"` | `MAX_BARREIRAS_POR_JOGADOR = 2`; a 3.ª é inserida e a mais antiga do jogador removida, `proxima_barreira_id` permanece monotônico e a iteração segue a ordem de `BTreeMap` | sim |
| 7 | **Caso hash divergente e determinismo**: comparar `hash_estado()` de dois mundos idênticos exceto por `construcao_ticks`/`construcao_ponto`/`barreiras`, e executar a mesma sequência em dois `Mundo` independentes (mesmo `epoch`/`match_id`) comparando a cada tick; capturar em `"$TMP/saidas/hash-determinismo.txt"` | resumos SHA-256 diferentes no primeiro par e idênticos tick a tick no segundo; os novos campos do jogador e todas as barreiras em ordem de `BTreeMap` entram na serialização | sim |
| 8 | **Caso dupla construção no mesmo tick**: agendar dois `construir_barreira` para o mesmo jogador no mesmo tick; em seguida emitir `mover` que altera a posição e avançar ticks; capturar em `"$TMP/saidas/dupla-construcao.txt"` | `BTreeSet<u64>` local ao tick permite no máx. 1 avanço de canalização por jogador por tick — o 45.º tick não é antecipado; `mover` com posição diferente zera `construcao_ticks`/`construcao_ponto` sem débito | sim |
| 9 | **Fronteira de contrato/Cargo e cliente**: `git status --short` e `grep` por `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, GDScript, GUI, rede, `#[allow]`, `unwrap`/`expect` > `"$TMP/saidas/fronteira.txt"`; verificar no teste do cliente o `seq`/`request_id` monotônico e o HUD | nenhum desses caminhos ou padrões alterado; mudança restrita a `lib.rs`, `simulacao.rs` e `main.rs`; tecla `B` em *just pressed* agenda `construir_barreira` com `destino` = posição atual e HUD mostra Materiais/Barreiras/Construindo; `[HIPÓTESE]` (fôlego/alcance/recarga/dimensões) fora e conflito §16.3/H4 registrado, não reaberto | parcial (inspeção automatizada + revisão humana) |
| 10 | **Governança, R6 N/A, rollback e [NV]**: `grep -nE '10 tasks|11 analyze|R1|R4|R6|1.0.0-local'` na constituição e nos artefatos > `"$TMP/saidas/governanca.txt"`; conferir rótulo **[NV]** em toda captura e aplicar o rollback documentado | `tasks` antes de `analyze` (READ-ONLY); R4 **condicionado à execução deste roteiro** e antes de R7 `1.0.0-local`; `converge` append-only; **R6 N/A** — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado, dívida de automação registrada; nenhum build/gate/commit executado pelo agente SDD e rollback documentado em `release.md`, nunca executado | parcial (revisão humana) |

## Plano de capturas (terminal)
- `$TMP/saidas/carimbo.txt`, `inventario.txt`, `caso-45-ticks.txt`, `saldo-insuficiente.txt`, `pv-200.txt`, `duracao-360.txt`, `tres-barricadas.txt`, `hash-determinismo.txt`, `dupla-construcao.txt`, `fronteira.txt`, `governanca.txt` — saídas dos passos 1–10.
- Toda captura é rotulada **[NV]** até o orquestrador executar o passo correspondente; nenhuma captura existe neste ciclo pelo agente SDD.

## Rollback do roteiro
`rm -rf "$TMP"` remove capturas e arquivos de apoio; encerrar qualquer inspeção sem persistir nada fora do worktree. Se um passo falhar, registrar a saída obtida e corrigir a **fonte** (código/artefato), nunca a expectativa do teste — em especial o caso de dupla construção (passo 8), corrigindo com `BTreeSet<u64>` local ao tick e revalidando o histórico append-only em `converge.md`. Não executar `git clean`, `git reset`, force-push, remoção de artefatos do ciclo, merge/PR, deploy nem reexecução de gates por este agente. Para reverter o ciclo, seguir `release.md` (rollback documentado, nunca executado neste ciclo).
