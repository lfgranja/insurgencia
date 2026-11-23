# Implementation Plan — 049-ilhas-vitoria

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 (validate) antes de R7 (release `1.0.0-local`); `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação fica registrada, sem execução.

**Fronteira explícita**: este agente escreve **somente** `specs/049-ilhas-vitoria/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/ilhas-vitoria-049/` (branch `049-ilhas-vitoria`, base `42b5f7b`). Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, `crates/sim`, `crates/cliente`, documentos derivados, artefatos de validação nem código-fonte (salvo agentes de implementação na etapa 12, que recebem `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs` e `crates/sim/tests`). Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]**.

**Rotulagem obrigatória**: a regra **10 s contínuos de Súdito da Comuna** e o sinal **`ilha_perdida`** são cânone (GDD §9.4:489,501; sinais em §9.1). A conversão para **300 ticks** é derivação de `PASSO_SEGUNDOS = 1 / 30`, e a **presença global de qualquer Súdito convertido** é simplificação de protótipo sem mapa — ambas rotuladas como não-canônicas nos 4 artefatos. Nós de Controle, vitória completa, papéis W/E/R, níveis e reconversão da Corporação (20 s, [HIPÓTESE]) permanecem fora de escopo. O mapa FR-001..FR-006 segue o dos artefatos de governança do ciclo: FR-001 regra/simplificação, FR-002 `Ilha`/`Mundo.ilhas`/`adicionar_ilha`, FR-003 `Mundo.sinais`/`ilha_perdida`, FR-004 `fase_controle_ilhas`, FR-005 hash/HUD/fronteira, FR-006 governança/estilo.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`.
2. **01 Intake**: sessão `20261123T111200Z-controle-ilha`, título `insurgencia - controle de ilha por sudito`; 80/20 = **108/625** (`ilhas-vitoria`: 4×3×3×3) contra Nós de Controle + vitória completa **48/625** (4×2×2×3), Papéis W/E/R + níveis **48/625** (4×2×2×3) e rede **10/625** (5×2×1×1, bloqueada por ADR-004 até decisão da SE); decisão em `.specify/memory/STATE.md` (2026-11-23T11:15Z).
3. **02 Research**: GDD §9.4:489,501 (10 s contínuos de Súdito da Comuna; sem Súdito, ilha volta a Latente e emite `ilha_perdida`) e tabela de sinais de §9.1 (`ilha_bloqueada`, `terreno_preparado`, `desafio_iniciado`, `contestacao`, `ilha_convertida`, `ilha_perdida`); `.specify/memory/STATE.md` (decisão 11:15Z); `crates/sim/src/lib.rs` (`PASSO_SEGUNDOS = 1/30`, `avancar_tick` com `prepare`→`skills`→`defesa`→`finish`→reversões, `Mundo` com `BTreeMap`/`VecDeque`, `Sudito.convertido` e `TICKS_PROPAGANDA = 120`, `hash_estado` serializando jogadores/barreiras/pontes/súditos); `crates/cliente/src/main.rs` (`atualizar_texto`, espelho `Simulacao(Mundo)` e linhas atuais do HUD); ciclos 044–048 (fronteiras já fixadas, nenhuma reaberta).
4. **03 Define (problema)**: a sim implementa defesa, pontes, coleta, súditos e combate, mas não possui campo de ilha, progresso de controle nem emissão de `ilha_perdida`; logo a condição canônica de vitória por ilha não é observável. Faltam `Ilha`, `Mundo.ilhas`, `adicionar_ilha`, fila de sinais, fase de controle entre `fase_defesa` e `fase_finish`, inclusão no hash e duas linhas no HUD. Critérios observáveis por FR/SC, com 300 ticks e presença global rotulados como protótipo.
5. **04 Shape (concept)**: 4 opções avaliadas no scorecard (controle de ilha, Nós/vitória, papéis W/E/R, rede); recomendação = `ilhas-vitoria`, subset controle de ilha, sem mudança de stack.
6. **05 Decide**: scorecard **4×3×3×3 = 108/625**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A; critério impacto × confiança × facilidade ÷ risco, denominador 625.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US7, FR-001..FR-006 e SC-001..SC-006; regra canônica distinguida de 300 ticks/presença global (rotulados como protótipo), scorecard e fronteira explícita.
8. **07 Clarify**: presença de qualquer Súdito convertido é global (sem mapa/posição por ilha); contagem é contínua (ausência zera); `ilha_perdida` só na transição controlada → Latente, uma vez por ilha e em ordem de `id`; `fase_controle_ilhas` fica entre `fase_defesa` e `fase_finish`; `adicionar_ilha(id)` reinicia a entrada; `hash_estado` cobre ilhas e sinais; HUD ganha `Ilhas: N` e `Sinais: N` sem remover linhas/teclas; reconversão da Corporação, Nós, papéis e vitória completa ficam de fora; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG linear; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 em `crates/sim/src/lib.rs` (`TICKS_CONTROLE_ILHA = 300` rotulado como derivação, `Ilha`, `Mundo.ilhas`, `adicionar_ilha`, `Mundo.sinais`, `fase_controle_ilhas` entre `fase_defesa` e `fase_finish`, `hash_estado` com ilhas e sinais), em `crates/cliente/src/main.rs` (`Ilhas: N` e `Sinais: N` aditivos) e em `crates/sim/tests` (cobertura determinística de 299/300 ticks, interrupção, perda e replay); sem mudança em contratos, protocolo, Cargo, stack, rede, manifesto ou CI; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`, `///` em itens públicos.
15. **13 Converge (append-only)**: primeira entrada datada em `specs/049-ilhas-vitoria/converge.md`; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade, rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/ilhas-vitoria/test-roadmap.md` (fora da fronteira deste agente). Superfície: inspeção de `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs`, `crates/sim/tests` e da fronteira de arquivos; execução da suíte determinística e do binário com `--minimizado` pelo orquestrador; capturas de terminal e, se houver surface, do texto do HUD **quando aplicável**. Não há surface automatizável para captura visual real do HUD neste alvo headless: toda captura visual/tela é **[NV]** até verificação humana. Rollback documental (limpar capturas em `$TMP`, corrigir a fonte, nunca a expectativa).

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD`, `date -u` e listar os artefatos do ciclo; conferir presença dos 4 artefatos desta fronteira e a ordem `tasks` antes de `analyze` | base `42b5f7b`, branch `049-ilhas-vitoria`, sessão `20261123T111200Z-controle-ilha`, scorecard **108/625**; `STATE.md`, `AGENTS.md`, `concurrency.jsonl` e paths alheiros intactos; R6 fora do fluxo | sim (inspeção/terminal) |
| 2 | Conferir 6 FR e 6 SC na spec, o mapeamento um-para-um no checklist e a rotulação canônico/protótipo nos 4 artefatos (FR-001 regra, FR-005 hash/HUD, FR-006 governança) | rastreabilidade FR-001..FR-006 idêntica à dos artefatos de governança; 10 s/`ilha_perdida` marcados como cânone (§9.4/§9.1) e 300 ticks/presença global como protótipo; nenhum número promovido a regra nova | sim (inspeção) |
| 3 | Inspecionar `Ilha`, `Mundo.ilhas` e `adicionar_ilha(id)`; conferir `Mundo::new` e a ausência de outros produtores de ilha | `Ilha { id, controlada, controle_ticks }` documentada; ilha nova com `controlada = false` e `controle_ticks = 0`; `BTreeMap` ordenado por `id`; `adicionar_ilha` repete sem duplicata ou pânico; sem crate/dependência nova | sim (inspeção) |
| 4 | Conferir `Mundo.sinais: VecDeque<String>` e o ponto exato de `push_back` de `ilha_perdida`; verificar que a presença lida é `Sudito.convertido` global | sinal enfileirado uma vez por transição, em ordem de `id`; ilha nunca controlada não emite; sem relógio, I/O, rede, `HashMap` ou aleatoriedade; §9.1 referenciado sem produtor novo | sim (inspeção) |
| 5 | Inspecionar `avancar_tick` e `fase_controle_ilhas`: ordem das fases e atualização de `controle_ticks`/`controlada` | sequência `prepare`→`skills`→`defesa`→`controle_ilhas`→`finish`→reversões; fase nova entre `fase_defesa` e `fase_finish`, sem alterar as demais; contador contínuo e saturado em 300 | sim (inspeção) |
| 6 | Simular 299/300 ticks com Súdito convertido e uma interrupção no meio (150 + 1 sem presença + 150) | 299 ainda Latente; 300º marca `controlada = true`; interrupção zera o progresso e a ilha não atinge 300 contínuos; nenhum sinal de perda antes do controle | sim (inspeção/cálculo); execução ao vivo **[NV]** |
| 7 | Simular perda de ilha controlada e perda repetida em ticks seguintes; conferir duas ilhas perdidas no mesmo tick | `controle_ticks = 0`, `controlada = false`, Latente e um `ilha_perdida` por transição; ilha já Latente não repete; duas perdas no mesmo tick saem em ordem crescente de `id` | sim (inspeção/cálculo) |
| 8 | Comparar `hash_estado` entre replays idênticos e após inserir ilha/sinal; executar a suíte determinística pelo orquestrador | hashes idênticos no replay; inclusão de ilhas (`id`, `controlada`, `controle_ticks`) e sinais FIFO muda o hash de forma previsível; cobertura nova e existente passam | parcial (inspeção); gates/testes **[NV]** |
| 9 | Inspecionar `atualizar_texto` e conferir `Ilhas: N` (controladas) e `Sinais: N` (fila), além das linhas/teclas anteriores | valores lidos do espelho `Mundo`; linha singular `Ilha: {ilha}` preservada; `Posicao`, `Limites`, `Suprimentos`, `Materiais`, `Fôlego`, `Vida`, `Alvo`, `Autogestão`, `Barreiras`, `Pontes`, `Súditos`, `Construindo`, `Coleta` e `WASD`/`B`/`P`/`C`/`G`/`Q` intactos; nenhum campo de protocolo novo | sim (inspeção de código); HUD visual **[NV]** |
| 10 | Governança, R6 N/A, rollback e rótulos: conferir `tasks` antes de `analyze`, R4 antes de R7 `1.0.0-local`, `converge` append-only, **[NV]** em toda captura e aplicar o rollback documentado | ordem v1.2.2 respeitada; R6 N/A (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado); nenhuma mudança em contratos/protocolo/Cargo/rede; nenhum build/gate/commit/push/PR executado por este agente; fonte corrigida se um passo falhar | parcial (revisão humana) |

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 regra canônica 10 s/`ilha_perdida` e simplificação rotulada (300 ticks, presença global) | 12 implement | T011 |
| FR-002 `Ilha { id, controlada, controle_ticks }`, `Mundo.ilhas` e `adicionar_ilha(id)` | 12 implement | T011 |
| FR-003 `Mundo.sinais: VecDeque<String>` e `ilha_perdida` na transição de perda | 12 implement | T011 |
| FR-004 `fase_controle_ilhas` entre `fase_defesa` e `fase_finish` (300 ticks, Latente, sinal) | 12 implement | T011 |
| FR-005 `hash_estado` com ilhas/sinais, HUD `Ilhas: N`/`Sinais: N` e fronteira determinística | 12/27 | T011, T012 |
| FR-006 governança R1/R4/R6/[NV] e estilo Rust | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Protótipo promovido a cânone | Rotulação explícita nos 4 artefatos (FR-001/SC-001, passo 2 do roteiro); 300 ticks são derivação de 10 s × 30 Hz e a presença global é simplificação de mapa |
| Regra canônica alterada por acidente | GDD §9.4:489,501 e §9.1 são apenas lidos; nenhum número, sinal ou condição canônica é redefinido; Nós/vitória/papéis ficam fora |
| Progresso acumulado em vez de contínuo | FR-004 exige zelo por tick de ausência; passos 6–7 do roteiro cobrem 299/300 e interrupção 150+1+150 |
| `ilha_perdida` duplicado ou emitido sem perda | FR-003 limita o sinal à transição `true`→`false`, uma vez por ilha, em ordem de `id`; passo 7 do roteiro |
| Ordem de fases regressiva | `fase_controle_ilhas` inserida explicitamente entre `fase_defesa` e `fase_finish` em `avancar_tick`; passo 5 do roteiro |
| Hash não determinístico | `BTreeMap`/`VecDeque` e ordem estável; ilhas e sinais entram no `hash_estado` com segmentos previsíveis; passo 8 cobre replay |
| HUD sobrescreve informações existentes | Linhas `Ilhas: N` e `Sinais: N` são aditivas; linha singular `Ilha: {ilha}` e todas as teclas permanecem; passo 9 do roteiro |
| Regressão da sim ou determinismo | Nenhuma edição em contratos/testes além da cobertura nova; `BTreeSet`, regeneração de Fôlego e demais fases intactos |
| Stack/protocolo/rede alterados | Fronteira explícita: nenhuma mudança em contratos, `PROTOCOL_VERSION`, Cargo, manifesto, CI ou rede; ADR-004 mantém rede bloqueada |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida de automação registrada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) antes de R7 (T012) |
| Captura visual fabricada | Sem surface automatizável: HUD visual e execução ao vivo **[NV]**; apenas inspeção e terminal são marcados como automatizáveis |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, sim, cliente e contratos intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]`.
- Rodar a suíte determinística e, se aplicável, o binário com `--minimizado`, conduzindo o roteiro de 10 passos e registrando capturas de terminal/HUD quando houver surface; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004, a regra canônica intacta e **300 ticks/presença global rotulados como protótipo**; R4 antes de R7 `1.0.0-local`.
