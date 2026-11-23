# Implementation Plan — 042-servidor-headless

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 antes de R7 `1.0.0-local`; `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação fica registrada, não executada.

**Fronteira explícita**: este agente escreve **somente** `specs/042-servidor-headless/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/servidor-headless-042/` (branch `042-servidor-headless`, base `b099093`, sessão `20261123T043000Z-servidor-headless`). Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `.specify/feature.json`, `crates/`, `Cargo.toml`, `Cargo.lock`, documentos derivados nem alterações preexistentes. Não executa gates, build, testes, formatadores, commit, push ou PR. Tudo o que depende de execução externa fica **[NV]**.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada, dívida registrada); R1 `go`.
2. **01 Intake**: sessão `20261123T043000Z-servidor-headless`, título `insurgencia - servidor headless deterministico`; 80/20 = servidor headless **4×4×4×4 = 256/625** contra dívida documental 250/625, combate/Fôlego 48/625 e rede com Lightyear 10/625.
3. **02 Research**: lidos `crates/sim/src/lib.rs` (`Mundo`, `Comando`, `ACOES_PERMITIDAS`, `agendar_comando`, `avancar_tick`, `hash_estado`, `ValidacaoComandos`) e `Cargo.toml` da workspace; confirmado que `hash_estado()` já produz SHA-256 canônico, sem nova dependência de hash.
4. **03 Define (problem)**: faltava autoridade observável e reproduzível fora do core: um executável headless que transforme comandos determinísticos em hash por tick; critérios observáveis por FR/SC.
5. **04 Shape (concept)**: quatro opções avaliadas no scorecard (servidor headless, dívida documental, combate/Fôlego, rede com Lightyear); recomendação = servidor headless determinístico.
6. **05 Decide**: `Veredito: go` (R1), scorecard **256/625**, 0 clarificações em aberto (R3), R6 N/A; R4 precederá R7 `1.0.0-local`.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006; forma de comando, aridade de coordenadas, erro exit 2 e snapshot de hash por tick explicitados.
8. **07 Clarify**: agendamento antes do tick 0 ≠ processamento imediato; validação de forma no parser ≠ validação semântica do core; hash no início do tick ≠ hash pós-`avancar_tick`; `Cargo.lock` gerado ≠ editado; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = revisão de requisitos, **não** implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: `analyze.md` fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos — crate `crates/servidor`, binário `insurgencia-servidor`, parser, agendamento, emissão de hash, testes unitários e de integração — com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`. Fora da fronteira deste agente.
15. **13 Converge (append-only)**: primeira entrada datada em `specs/042-servidor-headless/converge.md`; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Desenho da implementação futura (fora da fronteira SDD — **[NV]**)

Escopo de código previsto: **`crates/servidor/Cargo.toml`**, **`crates/servidor/src/main.rs`**, **`crates/servidor/src/parser.rs`** (ou módulo equivalente), **`crates/servidor/tests/cli.rs`** e a inclusão do membro em **`Cargo.toml`** da workspace; `Cargo.lock` gerado pelo Cargo.

- **Crate**: `insurgencia-servidor`, dependendo somente de `insurgencia-sim`; `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, `max_width = 100`, sem `unwrap`/`expect`/`#[allow]`, erros via `Result<_, ErroCli>` e `std::process::ExitCode` (2), `///` em itens públicos.
- **CLI**: ler `--ticks N` com `N > 0`; ausência, zero, negativo ou não inteiro → erro determinístico e exit 2, antes de qualquer stdout.
- **Parser**: uma linha por comando, tokens por whitespace; `peer_id: u64`, `acao: String`, `seq: i64`, `request_id: i64` e `x y: f64` finitos; ações de `ACOES_PERMITIDAS` com coordenadas obrigatórias em `mover`/`coletar_materiais`/`construir_barreira` e proibidas nas demais; primeira falha encerra com erro estável e número de linha 1-based.
- **Agendamento**: para cada linha válida, `adicionar_jogador` na primeira aparição do `peer_id` e `agendar_comando` na ordem de leitura; nenhuma reordenação ou deduplicação no binário; nenhuma regra semântica do core reaberta.
- **Loop**: para `tick` em `0..N`, imprimir `format!("{tick}\t{}", mundo.hash_estado())` e chamar `mundo.avancar_tick()`; exatamente `N` linhas, sem cabeçalho. O hash é o snapshot no início do tick; os comandos enfileirados antes do tick 0 são consumidos pelo primeiro `avancar_tick()`.
- **Testes**: unidades do parser (tipos, aridade, ações, números não finitos, erro com número de linha) e integração da CLI (duas execuções com a mesma stdin e `--ticks`, `diff` vazio; contagem de linhas e formato; entrada inválida com exit 2 e stdout vazia), usando apenas `std` e a biblioteca já disponível.
- **Não tocar**: `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION`, manifesto, CI, `concurrency.jsonl`, `STATE.md`, `AGENTS.md`; nenhuma dependência externa, async, socket ou Lightyear.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/servidor-headless/test-roadmap.md` (fora da fronteira deste agente). Superfície: CLI headless + `insurgencia-sim`; capturas de terminal; coluna `automatizável?`; rollback doc/estado.

1. Confirmar que R6 está fora do fluxo por constituição v1.2.2 (PARE se invocada; nenhuma issue simulada; dívida de automação registrada).
2. Conferir 6 FR e 6 SC na spec e o mapeamento um-para-um no checklist; T001..T012 únicos em DAG.
3. Verificar que `crates/servidor` depende só de `insurgencia-sim`, que a workspace inclui o membro e que `Cargo.lock` foi gerado pelo Cargo (sem async, socket, Lightyear ou nova dependência).
4. Validar `--ticks N` positivo: stdout com exatamente `N` linhas `0..N-1` no formato `<tick>\t<64 hex>`, sem cabeçalho nem ruído.
5. Validar a parser com as seis ações: coordenadas obrigatórias em `mover`/`coletar_materiais`/`construir_barreira` e proibidas nas outras três.
6. Validar erro determinístico: linha malformada, ação desconhecida, campos não numéricos/não finitos, número de linha 1-based, exit 2, stdout vazia e nenhum pânico.
7. Validar argumentos inválidos (`--ticks` ausente, `0`, negativo, não inteiro) com exit 2 e mensagem estável.
8. Comparar stdout de duas execuções com a mesma entrada (`diff` vazio) e conferir que o snapshot da linha 0 é o estado inicial após o agendamento.
9. Conferir agendamento em ordem: `Comando` preservado, peers registrados na primeira aparição, sem reordenação, e hash refletindo o processamento no tick seguinte.
10. Rodar gates/build/testes/formatador e comparar notas com o estado ao vivo; registrar resultados como observados ou **[NV]** — nunca afirmar verde sem saída colada.

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 crate `crates/servidor`/binário/só `insurgencia-sim`/workspace/`Cargo.lock` | 12 implement | T011 |
| FR-002 CLI `--ticks N` positivo/erro de argumento exit 2 | 12 implement | T011 |
| FR-003 parser da stdin/coordenadas/ações/erro determinístico | 12 implement | T011 |
| FR-004 agendamento em ordem antes do tick 0/`peer_id`/`Comando` | 12 implement | T011 |
| FR-005 saída `<tick>\t<hash>` por tick/determinismo | 12 implement | T011 |
| FR-006 estilo/testes/governança/R6 N/A/R4 antes de R7 | 00/12/27/26 | T001, T011, T012 |

Os 4 artefatos deste agente são produzidos por T005 (`spec.md`), T006 (`plan.md`), T007 (`checklists/requirements.md`) e T009 (`tasks.md`); T008 registra a revisão do checklist. T010–T012 e o código de T011 estão fora da fronteira de escrita deste agente e seguem a ordem v1.2.2.

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Hash divergente entre execuções | FR-005: apenas `Mundo::hash_estado()`, sem relógio, rede, aleatoriedade ou iteração não determinística; passos 8 e 10 do roteiro |
| Entrada malformada virando estado silencioso | FR-003: erro determinístico na primeira falha, com número de linha, exit 2 e stdout vazia |
| Coordenadas tratadas de forma inconsistente | FR-003: obrigatórias em três ações, proibidas nas outras três; seis ações de `ACOES_PERMITIDAS` |
| Comandos reordenados ou derrubados antes do tick 0 | FR-004: leitura completa da stdin, `adicionar_jogador` na primeira aparição e `agendar_comando` em ordem |
| Novo hashing ou dependência externa | FR-001/FR-005: reutilizar `hash_estado()` do core; crate depende somente de `insurgencia-sim` |
| `Cargo.lock` editado à mão | FR-001/Clarificações: artefato gerado pelo Cargo, nunca editado por agente |
| Pânico ou saída parcial em erro | FR-002/FR-003/FR-006: `Result`/`ExitCode`, sem `unwrap`/`expect`; erro antes de qualquer tick |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida registrada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) antes de R7 (T012) |
| Afirmar verde sem execução | Gates/build/testes **[NV]** até o orquestrador; nada afirmado como verde sem saída colada |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, código e manifestos intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar `cargo build`/`cargo test`/`cargo clippy` e o gate determinístico do alvo; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)`, ausência de `unwrap`/`expect`/`#[allow]` e `max_width = 100`.
- Rodar o roteiro de 10 passos, colar saídas observadas e registrar capturas; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo e a dívida de automação registrada, sem `gh` improvisado; R4 antes de R7 `1.0.0-local`.
