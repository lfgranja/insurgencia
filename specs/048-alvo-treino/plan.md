# Implementation Plan — 048-alvo-treino

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 (validate) antes de R7 (release `1.0.0-local`); `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação fica registrada, sem execução.

**Fronteira explícita**: este agente escreve **somente** `specs/048-alvo-treino/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/alvo-treino-048/` (branch `048-alvo-treino`, base `fdc6c08`). Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, `crates/sim`, documentos derivados, artefatos de validação nem código-fonte (salvo agentes de implementação na etapa 12, que recebem apenas `crates/cliente/src/main.rs`). Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]**.

**Rotulagem obrigatória**: o alvo de treino é **scaffolding de protótipo explicitamente não-canônico**. `PEER_ALVO_TREINO = 2`, `ALVO_TREINO_POSICAO = (2.0, 0.0)`, o marcador `AlvoTreino` e as linhas do HUD são escolhas locais do cliente, não regra de produto, não personagem do GDD e não número canônico. Os valores de combate reutilizados da sim (alcance `2.0 m`, custo `30`, dano `10`, `90 ticks`) permanecem `[HIPÓTESE]`/protótipo conforme a spec 047. **Nenhum valor pode ser apresentado como cânone** em qualquer um dos 4 artefatos. O mapa FR-001..FR-006 segue o dos artefatos de governança do ciclo: FR-001 constantes/rotulação, FR-002 peer 2 + marcador, FR-003 destino do `Q`/alcance do core, FR-004 HUD, FR-005 código/fronteira/determinismo, FR-006 governança.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`.
2. **01 Intake**: sessão `20261123T081800Z-alvo-treino`, título `insurgencia - alvo de treino jogavel`; 80/20 = **240/625** (`alvo-treino`: 3×4×4×5) contra Ilhas/condição de vitória **48/625** (4×2×2×3) e rede **10/625** (5×2×1×1, bloqueada por ADR-004 até decisão da SE); decisão em `.specify/memory/STATE.md` (2026-11-23T08:18Z).
3. **02 Research**: ACR do ciclo 047 (`Q` gera comando válido no core, mas o cliente standalone só tem `PEER_ID = 1`, sem alvo no `destino`, ataque inerte e dica enganosa), `.specify/memory/STATE.md` (decisão 08:18Z), `crates/cliente/src/main.rs` (`configurar`, `PEER_ID = 1`, `POSICAO_INICIAL = (0.0, -236.0)` em `lib.rs:18`, `sincronizar_sim` espelhando a posição local no peer 1, `solicitar_ataque` com `destino` na própria posição, `atualizar_texto` com `Fôlego`/`Vida` e dica "Q para atacar", `ModoJanela`/`--minimizado`), `crates/sim/src/lib.rs` (`adicionar_jogador` na origem, `ALCANCE_CONTATO_M = 2.0`, `aplicar_ataque` com `peer_id != comando.peer_id` e igualdade exata via `ponto_igual`, `CodigoRecusa::AlvoInvalido`/`FolegoInsuficiente`, `hash_estado`, cobertura determinística), constituição v1.2.2 e R6 N/A citados com fonte; ADR-004 registrada como bloqueio de rede, não reaberta.
4. **03 Define (problem)**: a sim valida e aplica `atacar_forte`, mas o cliente standalone não oferece alvo: o `destino` é a própria posição do jogador, o espelho `Mundo` só tem o peer 1 e a sim procura outro peer naquele ponto (`peer_id != comando.peer_id`). Resultado: comando aceito na geração, recusado no efeito (`AlvoInvalido`), nenhuma vida de alvo para observar e dica "Q para atacar" enganosa. Critérios observáveis por FR/SC, com o protótipo rotulado como não-canônico.
5. **04 Shape (concept)**: 3 opções avaliadas no scorecard (alvo de treino, ilhas/vitória, rede); recomendação = `alvo-treino`, sem mudança de stack.
6. **05 Decide**: scorecard **3×4×4×5 = 240/625**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A; critério impacto × confiança × facilidade ÷ risco, denominador 625.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US7, FR-001..FR-006 e SC-001..SC-006; alvo de treino e todos os números reutilizados rotulados como protótipo/`[HIPÓTESE]`, nunca cânone.
8. **07 Clarify**: o alvo é um peer sintético no espelho do cliente, não rede nem segundo jogador; `adicionar_jogador` cria peers na origem e `aplicar_ataque` exige igualdade exata com o `destino`, logo `configurar` deve atribuir explicitamente a posição `(2.0, 0.0)` ao peer 2; o avatar nasce em `(0.0, -236.0)` e `sincronizar_sim` propaga seus movimentos ao peer 1, então o jogador precisa aproximar-se a no máximo `2.0 m` antes que `Q` cause dano — a dica existe para isso; o marcador usa a transformação `(x, ALTURA_ILHA + 3.0, -y)`; nenhuma regra da sim é reescrita; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG linear; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 **somente em `crates/cliente/src/main.rs`** — constantes documentadas `PEER_ALVO_TREINO = 2` e `ALVO_TREINO_POSICAO = (2.0, 0.0)` rotuladas como protótipo; `configurar` registra o peer 2 no espelho `Mundo`, atribui explicitamente `jogadores.get_mut(&PEER_ALVO_TREINO).posicao = ALVO_TREINO_POSICAO` e spawna cubo/cone simples com componente `AlvoTreino` na transformação `(x, ALTURA_ILHA + 3.0, -y)`; `solicitar_ataque` passa `destino = Some(ALVO_TREINO_POSICAO)` mantendo `Q`, `seq`/`request_id` e um comando por pressão; `atualizar_texto` exibe `Alvo: {vida}` e a dica "Aproxime-se do alvo e use Q para atacar"; sem mudança em `crates/sim`, contratos, protocolo, Cargo, stack ou rede — com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100` (o atributo de lint `#[expect(...)]` permitido continua válido). **[NV]** até execução.
15. **13 Converge (append-only)**: primeira entrada datada em `specs/048-alvo-treino/converge.md`; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade, rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/alvo-treino/test-roadmap.md` (fora da fronteira deste agente). Superfície: inspeção de `crates/cliente/src/main.rs` e da fronteira de arquivos, execução do binário com `--minimizado` pelo orquestrador, capturas de terminal e do texto do HUD **quando aplicável**. Não há surface automatizável para captura visual real do HUD neste alvo headless: toda captura visual/tela é **[NV]** até verificação humana. Rollback documental (limpar capturas em `$TMP`, corrigir a fonte, nunca a expectativa).

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD`, `date -u` e listar os artefatos do ciclo; conferir presença dos 4 artefatos desta fronteira e a ordem `tasks` antes de `analyze` | base `fdc6c08`, branch `048-alvo-treino`, sessão `20261123T081800Z-alvo-treino`, scorecard **240/625**; `STATE.md`, `AGENTS.md`, `concurrency.jsonl` e paths alheiros intactos; R6 fora do fluxo | sim (inspeção/terminal) |
| 2 | Conferir 6 FR e 6 SC na spec, o mapeamento um-para-um no checklist e a rotulação não-canônica em todos os 4 artefatos (FR-001 constantes, FR-004 HUD, FR-006 governança) | rastreabilidade FR-001..FR-006 idêntica à dos artefatos de governança; `PEER_ALVO_TREINO = 2`, `(2.0, 0.0)` e as linhas do HUD marcados como protótipo; nenhum número promovido a cânone | sim (inspeção) |
| 3 | Inspecionar `configurar`: constantes documentadas, `sim.adicionar_jogador(PEER_ALVO_TREINO)`, a atribuição explícita da posição do peer 2 e o spawn do marcador `AlvoTreino` | espelho `Mundo` com peers `1` (posição espelhada por `sincronizar_sim`) e `2` fixo em `(2.0, 0.0)`; marcador com transformação `(x, ALTURA_ILHA + 3.0, -y)` e malha/material já existentes; sem asset, dependência, física, rede ou segundo jogador humano | sim (inspeção) |
| 4 | Conferir que `aplicar_ataque` busca o alvo por igualdade exata (`ponto_igual`) e que a posição do peer 2 foi fixada no `destino`; comparar com o spawn `(0.0, -236.0)` e o limite `ALCANCE_CONTATO_M = 2.0` da sim | sem mudança em `crates/sim`: o alvo é encontrado em `(2.0, 0.0)`; no spawn o jogador está a ~236 m e o ataque é recusado, e exatamente `2.0 m` é aceito (a sim recusa só com `>`); nenhuma linha de `crates/sim` ou `crates/contratos` alterada | sim (inspeção/cálculo) |
| 5 | Inspecionar `solicitar_ataque` e simular uma pressão única de `Q` com o jogador aproximado a `(1.0, 0.0)` | um `atacar_forte` com `destino = Some((2.0, 0.0))`, `peer_id = 1`, `seq`/`request_id` incrementados; dica e linhas do HUD preservadas; sem comando duplicado por frame | sim (inspeção); execução ao vivo **[NV]** |
| 6 | Verificar o efeito pela regra existente da sim: alvo dentro de `2 m` e fora de `2 m`, Fôlego suficiente e insuficiente, dois comandos no mesmo tick | dentro do alcance: vida do alvo cai `10` (satura em `0`) e Fôlego cai `30`; fora: `AlvoInvalido` sem dano/débito; Fôlego baixo: `FolegoInsuficiente`; `BTreeSet` deduplica; nenhuma regra da sim editada | sim (inspeção/leitura da sim) |
| 7 | Inspecionar `atualizar_texto` e conferir `Alvo: {vida}`, a dica exata "Aproxime-se do alvo e use Q para atacar" e as linhas/teclas anteriores | valor lido de `jogadores.get(&PEER_ALVO_TREINO)`; `Fôlego`, `Vida`, `Posicao`, materiais, barreiras, pontes, súditos e `WASD`/`B`/`P`/`C`/`G` preservados; nenhum campo de protocolo novo | sim (inspeção de código) |
| 8 | Executar o binário com `--minimizado` (pelo orquestrador), redirecionando saída para `$TMP/saidas`; capturar terminal e, se houver surface, o texto do HUD | janela minimizada sem travar; nenhum pânico ou `unwrap`; saída registrada; não havendo saída de terminal para o HUD, registrar **[NV]** — sem prometer captura visual real | parcial (terminal); HUD visual **[NV]** |
| 9 | Conferir estilo Rust e fronteira: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, ausência de `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`, e `git status --short` restrito a `crates/cliente/src/main.rs` + 4 artefatos | nenhuma mudança em `crates/sim`, contratos, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI ou rede; atributo de lint `#[expect(...)]` permitido; cobertura determinística existente não editada | sim (inspeção); gates/build/testes **[NV]** |
| 10 | Governança, R6 N/A, rollback e rótulos: conferir `tasks` antes de `analyze`, R4 antes de R7 `1.0.0-local`, `converge` append-only, **[NV]** em toda captura e aplicar o rollback documentado | ordem v1.2.2 respeitada; R6 N/A (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado); nenhum build/gate/commit/push/PR executado por este agente; fonte corrigida se um passo falhar | parcial (revisão humana) |

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 constantes `PEER_ALVO_TREINO`/`ALVO_TREINO_POSICAO` e rotulação não-canônica | 12 implement | T011 |
| FR-002 peer 2 no espelho `Mundo` (posição fixada em `ALVO_TREINO_POSICAO`) e marcador `AlvoTreino` no cliente | 12 implement | T011 |
| FR-003 destino do `Q` = `ALVO_TREINO_POSICAO` e validação de alcance do core (sem mudança na sim) | 12 implement | T011 |
| FR-004 HUD `Alvo: {vida}` e dica "Aproxime-se do alvo e use Q para atacar" | 12 implement | T011 |
| FR-005 código/fronteira/determinismo (sim, contratos, protocolo, Cargo e testes intactos) | 12/27 | T011, T012 |
| FR-006 governança R1/R4/R6/[NV] e estilo Rust | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Protótipo promovido a cânone | Rotulação explícita nos 4 artefatos (FR-001/SC-001, passo 2 do roteiro); peer 2 e `(2.0, 0.0)` são escolhas locais do cliente |
| Ataque continua inerte | FR-002 fixa a posição do peer 2 no `destino` (igualdade exata) e FR-003 troca o `destino` para `ALVO_TREINO_POSICAO`; passos 3–6 do roteiro confirmam peer, alcance e efeito pela regra existente |
| Dica enganosa persistir | FR-004 exige a dica exata "Aproxime-se do alvo e use Q para atacar"; passo 7 do roteiro |
| Jogador no spawn fora de alcance sem orientação | O avatar nasce a ~236 m do alvo; a dica orienta a aproximação e a sim recusa com `AlvoInvalido` acima de `2.0 m` sem dano ou débito; comportamento congelado |
| Alvo com vida zero gerando expectativa de morte | Sim existente satura em `0` e não remove o alvo; marcador permanece — registrado como protótipo, sem animação ou vitória |
| Regressão da sim ou do determinismo | FR-005: nenhuma edição em `crates/sim`/testes; cobertura determinística existente permanece válida; `hash_estado` e ordem estável intactos |
| Stack/protocolo/rede alterados | Fronteira explícita: nenhuma mudança em contratos, `PROTOCOL_VERSION`, Cargo, manifesto ou CI; ADR-004 mantém rede bloqueada |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida de automação registrada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) antes de R7 (T012) |
| Captura visual fabricada | Sem surface automatizável: HUD visual e execução ao vivo **[NV]**; apenas inspeção e terminal são marcados como automatizáveis |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, sim e contratos intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]`.
- Rodar o binário com `--minimizado`, conduzir o roteiro de 10 passos e registrar capturas de terminal/HUD quando houver surface; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004 e **todo o conteúdo do alvo de treino rotulado como protótipo não-canônico**; R4 antes de R7 `1.0.0-local`.
