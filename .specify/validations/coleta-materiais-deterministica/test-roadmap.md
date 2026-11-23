# Roteiro de Testes — Coleta determinística de materiais (ciclo 039)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador/SWE · **Evidências aqui**: **[NV]** (nada executado neste ciclo pelo agente SDD)

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passos 2, 3 — 180 ticks → +3/+2; saída antes zera parcial |
| FR-002 | passos 2–7 — constantes/campos, validação, crédito atômico, hash refletivo, `destino` como ID do ponto |
| FR-003 | passo 8 — fronteira `lib.rs`, protocolo e Cargo intactos |
| FR-004 | passo 9 — ordem v1.2.2, R1/R4/R7, `converge` append-only, R6 fora do fluxo |
| FR-005 | passo 9 — 80/20 = 180/625 e fila documental esgotada |
| FR-006 | passos 1, 8, 9 — 14 artefatos, `STATE.md`/`feature.json`/`converge` alheio preservados |
| FR-007 | passos 1–10 e plano de capturas abaixo |
| FR-008 | passos 9, 10 — não-execução pelo agente SDD, pendências **[NV]**, rollback |
| SC-001..SC-006 | passos 1–10 |

T-IDs relacionados: T001–T015 (SDD), T016 (converge), T017 (R4), T018 (R7) e T019 (implementação em `lib.rs` + gates + PR, **[NV]**).

## Pré-condições

1. Worktree `.specify/queue/worktrees/coleta-materiais-039/`, branch `039-coleta-materiais-deterministica`, base `990ab06`; os 14 artefatos SDD presentes na fronteira exata, com `tasks.md` (T012) antes de `analyze.md` (T013, READ-ONLY).
2. Implementação de referência em `crates/sim/src/lib.rs` pronta pelo SWE: `TICKS_COLETA=180`, campos em `Jogador`, validação, `fase_skills` e `hash_estado`; nenhum `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml` alterado.
3. `TMP=$(mktemp -d)` fora do worktree para capturas; `mkdir -p "$TMP/saidas"`. Nenhum gate, build, commit, push, PR, merge ou deploy executado por este agente — os passos abaixo são instruções ao orquestrador/SWE.

## Superfície documental/CLI
- Inspeção: `git rev-parse`/`git status --short`, `find`, `grep`, `diff`, `cat`, `date -u`, leitura de fontes (GDD §7.1, `lib.rs`, constituição v1.2.2).
- Testes: `cargo test -p sim` (ou o comando canônico do hub) executado **pelo orquestrador**, com saída redirecionada para `$TMP/saidas`; nenhuma janela gráfica, rede ou credencial.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD` e `date -u > "$TMP/saidas/carimbo.txt"`; listar os 14 artefatos e conferir o stub substituído > `"$TMP/saidas/inventario.txt"` | base `990ab06`, branch `039-coleta-materiais-deterministica`, 14 artefatos na fronteira; `.specify/memory/STATE.md` e `.specify/feature.json` intactos | sim |
| 2 | **Caso 180 ticks**: adicionar jogador, agendar `coletar_materiais` com `destino == posicao`, avançar 180 ticks com o jogador parado; `grep -n 'TICKS_COLETA\|180\|3\|2' ... > "$TMP/saidas/caso-180.txt"` | exatamente +3 suprimentos e +2 materiais ao completar o 180.º tick; nenhum crédito no tick 179 | sim |
| 3 | **Caso saída antes**: acumular 179 ticks, mover o jogador para outro ponto (ou omitir o comando) e avançar mais ticks; captura em `"$TMP/saidas/saida-antes.txt"` | `coleta_ticks` e `coleta_ponto` zerados, estoques inalterados — nenhum acúmulo parcial creditado | sim |
| 4 | **Caso recusa sem destino**: `ValidacaoComandos::validar(..., "coletar_materiais", ..., None)` e também com `destino != posicao`; captura em `"$TMP/saidas/recusa-destino.txt"` | `Err(CodigoRecusa::DestinoInvalido)` nos dois casos; comando não altera estado | sim |
| 5 | **Caso hash divergente**: comparar `hash_estado()` de dois mundos idênticos exceto por estoques/`coleta_ticks` diferentes; captura em `"$TMP/saidas/hash-divergente.txt"` | resumos SHA-256 diferentes; os quatro campos novos entram na serialização | sim |
| 6 | **Caso ausência de consumo parcial**: percorrer os 180 ticks registrando estoques a cada tick; captura em `"$TMP/saidas/sem-consumo.txt"` | nenhum débito de suprimentos/materiais durante a coleta; único evento é o crédito atômico final | sim |
| 7 | **Caso determinismo**: executar a mesma sequência de comandos em dois `Mundo` independentes (mesmo `epoch`/`match_id`) e comparar `hash_estado()` a cada tick; captura em `"$TMP/saidas/determinismo.txt"` | hashes idênticos tick a tick em ambas as execuções ( Princípio da simulação determinística ) | sim |
| 8 | **Fronteira/protocolo**: `git status --short`, `grep` por `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, GDScript, GUI, rede, dano/barricada > `"$TMP/saidas/fronteira.txt"` | nenhum desses caminhos alterado; ponto neutro identificado apenas por `Comando.destino`; mudança restrita a `lib.rs` | sim |
| 9 | **Governança/80/20/não-execução**: `grep -nE '10 tasks|11 analyze|R1|R4|R6|1.0.0'` na constituição e nos artefatos > `"$TMP/saidas/governanca.txt"`; `grep -n '180/625'` no intake/concept/decision | `tasks` antes de `analyze` (READ-ONLY); R4 antes de R7 `1.0.0`; `converge` append-only; R6 suspensa (PARE se invocada, nenhuma issue simulada); 180/625 documentado; nenhum build/gate/commit executado pelo agente SDD | parcial (inspeção automatizada + revisão humana) |
| 10 | **Rollback e [NV]**: conferir que toda captura está rotulada **[NV]** até execução pelo orquestrador; `diff` das capturas com a execução real e registrar divergências em relatório | nenhuma captura produzida por este agente; divergência registrada com data/fonte, nunca reescrita sem rastro; rollback documentado em `release.md` e não executado | parcial (revisão humana) |

## Plano de capturas (terminal)
- `$TMP/saidas/carimbo.txt`, `inventario.txt`, `caso-180.txt`, `saida-antes.txt`, `recusa-destino.txt`, `hash-divergente.txt`, `sem-consumo.txt`, `determinismo.txt`, `fronteira.txt`, `governanca.txt` — saídas dos passos 1–9.
- Toda captura é rotulada **[NV]** até o orquestrador executar o passo correspondente; nenhuma captura existe neste ciclo.

## Rollback do roteiro
`rm -rf "$TMP"` remove capturas e arquivos de apoio; encerrar qualquer inspeção sem persistir nada fora do worktree. Se um passo falhar, registrar a saída obtida e corrigir a **fonte** (código/artefato), nunca a expectativa do teste. Não executar `git clean`, `git reset`, force-push, remoção de artefatos do ciclo, merge/PR, deploy nem reexecução de gates por este agente. Para reverter o ciclo, seguir `release.md` (rollback documentado, nunca executado neste ciclo).
