# Roteiro de Testes — Coleta jogável no cliente (ciclo 041)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador/SWE · **Evidências aqui**: **[NV]** (nada executado neste ciclo pelo agente SDD)

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passos 2–4 — regra canônica do GDD §7.1:303,310: **6 s = 180 ticks** a 1/30 s em ponto neutro rendem exatamente **+3 Suprimentos e +2 Materiais**; sair antes **zera o acúmulo parcial**; nenhum valor aproximado ou recompensa alternativa |
| FR-002 | passos 2, 8, 9 — contrato técnico auditado: `TICKS_COLETA = 180`, `coleta_ticks`/`coleta_ponto` em `Jogador`, `Comando.destino` como ponto de coleta, doc comment corrigido para citar `mover`, `coletar_materiais` e `construir_barreira`; nenhuma mensagem nova e nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml` |
| FR-003 | passos 2, 4–6 — auto-avanço determinístico: após um comando válido `coletar_materiais`, 1 tick por jogador por tick enquanto o ponto for o mesmo; `mover` para posição diferente cancela; `BTreeSet<u64>` local ao tick impede dupla contagem no mesmo tick |
| FR-004 | passos 2–4 — crédito atômico de +3/+2 somente no tick 180, com zeração de `coleta_ticks`/`coleta_ponto`; nenhum crédito parcial, nenhum crédito no tick 179 e nenhum reembolso |
| FR-005 | passo 7 — `hash_estado` inclui `coleta_ticks`/`coleta_ponto` de cada jogador; resumos divergem para progressos distintos e replay idêntico tick a tick para sequências iguais |
| FR-006 | passos 8–10 — tecla `C` em *just pressed* agenda `coletar_materiais` com `destino` = posição atual e `seq`/`request_id` monotônicos; HUD com `Suprimentos` e progresso de `Coleta`, além de Materiais/Barreiras/Construindo; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, commits PT-BR; constituição v1.2.2, R1 `go`, R4 antes de R7, **R6 N/A** |
| Governança (fora dos FR) | passos 1, 10 — 5 artefatos na fronteira exata, `tasks` antes de `analyze` (READ-ONLY), `converge` append-only, rollback documentado e rótulo **[NV]** em toda captura |
| SC-001..SC-006 | passos 1–10 |

T-IDs relacionados: T001–T012 (SDD, com `tasks` antes de `analyze` READ-ONLY e os artefatos de validação/release no mesmo intervalo). A implementação em `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs` e `crates/cliente/src/main.rs` é handoff ao SWE/orquestrador, fora da fronteira SDD deste ciclo e **[NV]** até execução.

## Pré-condições
1. Worktree `.specify/queue/worktrees/coleta-cliente-041/`, branch `041-coleta-jogavel`, base `bf05cfa`; sessão `20261123T035400Z-coleta-jogavel`. Artefatos SDD presentes na fronteira exata, com `tasks` antes de `analyze.md` (READ-ONLY).
2. Implementação de referência em `crates/sim/src/lib.rs`, testes em `crates/sim/tests/simulacao.rs` e cliente em `crates/cliente/src/main.rs` prontos pelo SWE: auto-avanço com `BTreeSet<u64>` local ao tick, doc comment de `Comando.destino` corrigido, tecla `C` e HUD; nenhum `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml` alterado; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`.
3. `TMP=$(mktemp -d)` fora do worktree para capturas; `mkdir -p "$TMP/saidas"`. Nenhum gate, build, commit, push, PR, merge ou deploy executado por este agente — os passos abaixo são instruções ao orquestrador/SWE.

## Superfície documental/CLI
- Inspeção: `git rev-parse`/`git status --short`, `find`, `grep`, `diff`, `cat`, `date -u`, leitura de fontes (GDD §7.1, `lib.rs`, `simulacao.rs`, `main.rs`, constituição v1.2.2).
- Testes: `cargo test -p sim` e, se aplicável, `cargo test -p cliente` (ou o comando canônico do hub), executados **pelo orquestrador**, com saída redirecionada para `$TMP/saidas`; nenhuma janela gráfica, rede ou credencial. Tecla `C` simulada apenas por teste automatizado de entrada, nunca por janela real.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD` e `date -u > "$TMP/saidas/carimbo.txt"`; listar os 5 artefatos do ciclo e conferir `tasks` antes de `analyze` > `"$TMP/saidas/inventario.txt"` | base `bf05cfa`, branch `041-coleta-jogavel`, 5 artefatos na fronteira; `.specify/memory/STATE.md`, `concurrency.jsonl` e paths alheios intactos | sim |
| 2 | **Caso 180 ticks**: adicionar jogador em ponto neutro, agendar um `coletar_materiais` válido, avançar 180 ticks sem novos comandos e registrar `coleta_ticks` a cada tick em `"$TMP/saidas/caso-180-ticks.txt"` | `TICKS_COLETA = 180`; o contador avança exatamente 1 por tick, de 0 a 180, sem estagnação e sem salto; o auto-avanço independe de comandos intermediários | sim |
| 3 | **Caso +3/+2**: no cenário do passo 2, registrar `suprimentos`/`materiais` nos ticks 179 e 180 em `"$TMP/saidas/caso-credito.txt"` | nenhum crédito no tick 179; no tick 180, crédito atômico de exatamente **+3 Suprimentos e +2 Materiais**, com `coleta_ticks`/`coleta_ponto` zerados e nenhum crédito extra no tick seguinte | sim |
| 4 | **Caso saída antes zera o parcial**: iniciar coleta, avançar 90 ticks, emitir `mover` para posição diferente e avançar mais ticks; capturar em `"$TMP/saidas/saida-antes.txt"` | `mover` com posição diferente zera `coleta_ticks`/`coleta_ponto`; nenhum crédito ocorre, nem no ponto antigo nem no novo, enquanto não houver novo comando válido e 180 ticks ininterruptos | sim |
| 5 | **Caso dupla coleta no mesmo tick**: agendar dois `coletar_materiais` para o mesmo jogador no mesmo tick e também deixar o auto-avanço atuar nesse tick; capturar em `"$TMP/saidas/dupla-coleta.txt"` | `BTreeSet<u64>` local ao tick, compartilhado entre comando e auto-avanço, permite no máx. **1 avanço por jogador por tick** — o 180.º tick não é antecipado e o crédito não é duplicado | sim |
| 6 | **Caso um comando único**: agendar exatamente um `coletar_materiais` válido, avançar 180 ticks e comparar com uma sequência que reenvia o comando a cada tick; capturar em `"$TMP/saidas/comando-unico.txt"` | um único comando sustenta toda a coleta e credita +3/+2 no tick 180; reenviar o comando em ticks intermediários não reinicia, não acelera nem duplica o progresso — mesma sequência de estados | sim |
| 7 | **Caso hash divergente e determinismo**: comparar `hash_estado()` de dois mundos idênticos exceto por `coleta_ticks`/`coleta_ponto`, e executar a mesma sequência de comandos em dois `Mundo` independentes (mesmo `epoch`/`match_id`) comparando a cada tick; capturar em `"$TMP/saidas/hash-determinismo.txt"` | resumos SHA-256 diferentes no primeiro par e idênticos tick a tick no segundo; os campos de coleta entram na serialização e o auto-avanço é replayável | sim |
| 8 | **Caso tecla C/HUD e doc comment**: em teste automatizado de entrada, pressionar `C` em *just pressed* e em `held`, inspecionar o comando agendado e o texto do HUD, e conferir o doc comment de `Comando.destino` > `"$TMP/saidas/cliente-c-hud.txt"` | `C` em *just pressed* agenda `coletar_materiais` com `destino` = posição atual e `seq`/`request_id` monotônicos; `held` não agenda; HUD mostra `Suprimentos` e progresso de `Coleta` além de Materiais/Barreiras/Construindo; o doc comment cita `mover`, `coletar_materiais` e `construir_barreira` | parcial (teste automatizado + inspeção) |
| 9 | **Fronteira de contrato/Cargo**: `git status --short` e `grep` por `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, GDScript, GUI, rede, `#[allow]`, `unwrap`/`expect`, `unsafe` > `"$TMP/saidas/fronteira.txt"`; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e `max_width = 100` | nenhum desses caminhos ou padrões alterado; mudança restrita a `lib.rs`, `simulacao.rs` e `main.rs`; nenhuma mensagem nova no protocolo; convenções de lint e largura preservadas | sim |
| 10 | **Governança, R6 N/A, rollback e [NV]**: `grep -nE 'tasks|analyze|R1|R4|R6|1.0.0-local'` na constituição e nos artefatos > `"$TMP/saidas/governanca.txt"`; conferir rótulo **[NV]** em toda captura e aplicar o rollback documentado | `tasks` antes de `analyze` (READ-ONLY); R4 **condicionado à execução deste roteiro** e antes de R7 `1.0.0-local`; `converge` append-only; **R6 N/A** — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado, dívida de automação registrada; nenhum build/gate/commit executado pelo agente SDD e rollback documentado em `release.md`, nunca executado | parcial (revisão humana) |

## Plano de capturas (terminal)
- `$TMP/saidas/carimbo.txt`, `inventario.txt`, `caso-180-ticks.txt`, `caso-credito.txt`, `saida-antes.txt`, `dupla-coleta.txt`, `comando-unico.txt`, `hash-determinismo.txt`, `cliente-c-hud.txt`, `fronteira.txt`, `governanca.txt` — saídas dos passos 1–10.
- Toda captura é rotulada **[NV]** até o orquestrador executar o passo correspondente; nenhuma captura existe neste ciclo pelo agente SDD.

## Rollback do roteiro
`rm -rf "$TMP"` remove capturas e arquivos de apoio; encerrar qualquer inspeção sem persistir nada fora do worktree. Se um passo falhar, registrar a saída obtida e corrigir a **fonte** (código/artefato), nunca a expectativa do teste — em especial os casos de dupla coleta (passo 5) e comando único (passo 6), corrigindo com `BTreeSet<u64>` local ao tick compartilhado e revalidando o histórico append-only em `converge.md`. Não executar `git clean`, `git reset`, force-push, remoção de artefatos do ciclo, merge/PR, deploy nem reexecução de gates por este agente. Para reverter o ciclo, seguir `release.md` (rollback documentado, nunca executado neste ciclo).
