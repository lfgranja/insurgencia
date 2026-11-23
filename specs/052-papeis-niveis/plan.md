# Implementation Plan — 052-papeis-niveis

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 (validate) antes de R7 (release `1.0.0-local`); `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação é registrada, não executada.

**Fronteira explícita**: este agente escreve **somente** `specs/052-papeis-niveis/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/papeis-052/` (branch `052-papeis-niveis`, base `4f3966e`). Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, `crates/sim`, `crates/cliente`, documentos derivados, artefatos de validação nem código-fonte (salvo agentes de implementação na etapa 12, que recebem `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs` e `crates/cliente/src/main.rs`). Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]**.

**Rotulagem obrigatória**: os papéis materiais (Artesão, Guerreiro, Intelectual, Camponês, Agitador), o teto de nível **12** e o desbloqueio **Q no 1, W no 2, E no 3, R no 6** são `[HIPÓTESE]` em GDD §10.1:515-528 (resumo em §10.8:744-759). **900 ticks por nível** é derivação de `PASSO_SEGUNDOS = 1 / 30` (30 s × 30 Hz), também protótipo. A atribuição inicial `Funcao::Artesao`, o uso do `tick` global como cronômetro de progressão e o mapeamento de W/E/R para comandos **existentes** (`construir_barreira`/`construir_ponte`, `propaganda`/`propaganda_no`, `atacar_no`) são simplificações determinísticas, rotuladas como não-canônicas nos 4 artefatos. Habilidades reais de GDD §10.2–10.6, seleção de papel, PVE/PNJs e rede permanecem fora de escopo.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`.
2. **01 Intake**: sessão `20261123T150800Z-papeis-niveis`, título `insurgencia - papeis e niveis`; 80/20 = **4×2×2×3 = 48/625** (papeis-niveis) contra PVE/PNJs **48/625** e rede **10/625** (bloqueada por ADR-004 até decisão da SE); decisão em `.specify/memory/STATE.md` (2026-11-23T15:08Z). O desempate com PVE/PNJs cabe à reutilização de comandos existentes e à âncora direta em GDD §10.1.
3. **02 Research**: GDD §10:513 (papéis materiais, não classes mágicas), §10.1:515-528 (Fôlego, teto 12, desbloqueio Q1/W2/E3/R6 — todos `[HIPÓTESE]`), §10.2–10.6:536-709 (Intelectual, Artesão, Agitador, Camponês, Guerreiro), §10.8:744-759 (tabela-resumo); `.specify/memory/STATE.md` (decisão 15:08Z); `crates/sim/src/lib.rs` (`PASSO_SEGUNDOS = 1/30`, `Jogador`, `Mundo::adicionar_jogador`, `fase_prepare`, `avancar_tick`, `ACOES_PERMITIDAS`, `ValidacaoComandos::validar`, `CodigoRecusa`, `serializar_jogadores`, `hash_estado`); `crates/cliente/src/main.rs` (`texto_hud`, `atualizar_texto`, `PEER_ID`, teclas atuais); `crates/sim/tests/simulacao.rs` (suíte determinística existente).
4. **03 Define (problema)**: a sim já tem ações Q/W/E/R-like (`atacar_forte`, construções, propaganda, ataques a Nó), Fôlego, ilhas, Nós e desfechos, mas não tem papel, nível nem gate de progressão; logo os limiares de GDD §10.1 não são observáveis no estado determinístico. Faltam `Funcao`, `Jogador.funcao`, `Jogador.nivel`, `TICKS_POR_NIVEL`, `NIVEL_MAXIMO`, `CodigoRecusa::AcaoBloqueada`, o mapa de níveis em `validar`, a inclusão de papel/nível no hash e as duas linhas do HUD. Critérios observáveis por FR/SC, com 900 ticks e todos os números rotulados como protótipo.
5. **04 Shape (concept)**: 3 opções avaliadas no scorecard (papeis-niveis, PVE/PNJs, rede); recomendação = `papeis-niveis`, reaproveitando `ACOES_PERMITIDAS` e `ValidacaoComandos`, sem mudança de stack, sem PNJs e sem protocolo.
6. **05 Decide**: scorecard **4×2×2×3 = 48/625**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A; critério impacto × confiança × facilidade ÷ risco, denominador 625.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006; regras canônicas distinguidas de todos os números `[HIPÓTESE]` e de 900 ticks/progressão por `tick` global (rotulados como protótipo), scorecard e fronteira explícita.
8. **07 Clarify**: os números do GDD são hipóteses rotuladas, não cânone operacional; 900 ticks derivam de 30 s × 30 Hz e o nível sobe em `fase_prepare`, antes dos comandos do tick, saturando em 12; o papel é fixo (`Funcao::Artesao`) e não altera efeitos; as quatro ações sem slot Q/W/E/R não são gateadas; W/E/R são apenas os comandos existentes, nunca habilidades novas; `AcaoBloqueada` vem depois de par/replay/ação desconhecida e antes das recusas específicas; `texto_hud` lê o espelho autoritativo e `serializar_jogadores` cobre papel e nível; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG linear; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 em `crates/sim/src/lib.rs` (`Funcao` com cinco variantes documentadas e `Jogador.funcao = Funcao::Artesao` em `adicionar_jogador`; `TICKS_POR_NIVEL = 900` e `NIVEL_MAXIMO = 12` rotulados `[HIPÓTESE]`; `Jogador.nivel = 1` e +1 em `fase_prepare` nos múltiplos de 900, saturando em 12; mapa Q1/W2/E3/R6 sobre as ações existentes em `ValidacaoComandos::validar`, com `CodigoRecusa::AcaoBloqueada` depois de par/replay/ação desconhecida e antes das recusas específicas; `serializar_jogadores` com papel e nível), em `crates/cliente/src/main.rs` (linhas `Função: <nome>` e `Nível: N/12` em `texto_hud`, a partir do espelho `Mundo`, sem novas teclas) e em `crates/sim/tests/simulacao.rs` (cobertura de 0/899/900/1800/10800 ticks, saturação, todos os limiares de desbloqueio, precedência de `AcaoBloqueada`, ações livres de nível e estabilidade do hash).
15. **13 Converge (append-only)**: primeira entrada datada em `specs/052-papeis-niveis/converge.md`; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade, rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/papeis-niveis/test-roadmap.md` (fora da fronteira deste agente). Superfície: inspeção de `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs`, `crates/sim/tests/simulacao.rs` e da fronteira de arquivos; execução da suíte determinística e do binário com `--minimizado` pelo orquestrador; capturas de terminal e, se houver surface, do texto do HUD **quando aplicável**. Não há surface automatizável para captura visual real do HUD neste alvo headless: toda captura visual/tela é **[NV]** até verificação humana. Rollback documental (corrigir a fonte, nunca a expectativa; limpar capturas em `$TMP`).

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD`, `date -u` e listar os artefatos do ciclo; conferir presença dos 4 artefatos desta fronteira e a ordem `tasks` antes de `analyze` | base `4f3966e`, branch `052-papeis-niveis`, sessão `20261123T150800Z-papeis-niveis`, scorecard **48/625**; `STATE.md`, `AGENTS.md`, `concurrency.jsonl` e paths alheiros intactos; R6 fora do fluxo | sim (inspeção/terminal) |
| 2 | Conferir 6 FR e 6 SC na spec, o mapeamento um-para-um no checklist e a rotulação de todos os números como `[HIPÓTESE]`/protótipo nos 4 artefatos (FR-001 papéis, FR-002 nível/900 ticks, FR-006 governança) | rastreabilidade FR-001..FR-006 idêntica à dos artefatos de governança; nível 1–12, 900 ticks e Q1/W2/E3/R6 marcados como hipótese/derivação; nenhum número promovido a regra nova | sim (inspeção) |
| 3 | Inspecionar `Funcao`, `Jogador.funcao`, `adicionar_jogador`, `TICKS_POR_NIVEL`, `NIVEL_MAXIMO` e `Jogador.nivel` | cinco variantes documentadas, papel inicial `Artesao` e nível inicial 1; constantes 900 e 12 rotuladas `[HIPÓTESE]`; nenhum `Instant`/`SystemTime`, contador por jogador, crate ou dependência novo | sim (inspeção) |
| 4 | Simular `tick = 0/899/900/1800/10800/11000` e conferir `fase_prepare` antes dos comandos do tick | 899 mantém nível 1; 900 eleva a 2; 1800 eleva a 3; 10800 atinge 12; ticks posteriores saturam; o comando agendado no tick 900 já vê nível 2 | sim (inspeção/cálculo); execução ao vivo **[NV]** |
| 5 | Validar as ações existentes nos níveis 1, 2, 3, 5 e 6 contra a tabela Q1/W2/E3/R6 | `atacar_forte` aceito no 1; construções no 2; `propaganda`/`propaganda_no` no 3; `atacar_no` no 6; `mover`, sinais e coleta nunca gateados; `ACOES_PERMITIDAS` e o tratamento de teclado intactos | sim (inspeção/cálculo) |
| 6 | Combinar `AcaoBloqueada` com destino ausente, Fôlego zero, saldo zero e Nó inativo; conferir precedência de par/replay/ação desconhecida | bloqueio devolvido antes das recusas específicas; `PeerNaoAutorizado`, `SeqOuRequestReplay` e `AcaoDesconhecida` continuam na frente; `validar` não muta o mundo nem agenda comandos | sim (inspeção/cálculo) |
| 7 | Comparar `hash_estado` entre replays idênticos e após mudar apenas papel ou nível; executar a suíte determinística pelo orquestrador | hashes idênticos no replay; inclusão de `funcao`/`nivel` em `serializar_jogadores` muda o hash de forma previsível; segmentos existentes preservados; cobertura nova e existente passam | parcial (inspeção); gates/testes **[NV]** |
| 8 | Inspecionar `texto_hud` e `atualizar_texto`; conferir `Função: Artesão`/`Guerreiro`/`Intelectual`/`Camponês`/`Agitador` e `Nível: N/12`, além da conservação das linhas e teclas | valores lidos do espelho `Mundo`; denominador derivado de `NIVEL_MAXIMO`; linhas `Posicao`, `Ilha`, `Limites`, `Suprimentos`, `Materiais`, `Fôlego`, `Vida`, `Alvo`, `Autogestão`, `Barreiras`, `Pontes`, `Súditos`, `Ilhas`, `Sinais`, `Resistência`, `Nós`, `Resultado`, `Construindo`, `Coleta` e teclas `WASD`/`B`/`P`/`C`/`G`/`R`/`F`/`Q` intactas; nenhum campo de protocolo novo | sim (inspeção de código); HUD visual **[NV]** |
| 9 | Conferir que nenhum gate depende de Aleatoriedade, I/O, relógio externo, rede ou segundo jogador; verificar `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]` | progressão e desbloqueio derivados apenas de `tick` e do estado do jogador; estilo Rust v1.2.2 respeitado; `#[expect(...)]` como atributo de lint permanece permitido | sim (inspeção); gates **[NV]** |
| 10 | Governança, R6 N/A, rollback e rótulos: conferir `tasks` antes de `analyze`, R4 antes de R7 `1.0.0-local`, `converge` append-only, **[NV]** em toda captura e aplicar o rollback documentado | ordem v1.2.2 respeitada; R6 N/A (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado); nenhuma mudança em contratos/protocolo/Cargo/rede; nenhum build/gate/commit/push/PR executado por este agente; fonte corrigida se um passo falhar | parcial (revisão humana) |

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 papéis materiais (`Funcao`, `Jogador.funcao`, padrão `Artesao`) e rotulação `[HIPÓTESE]` | 12 implement | T011 |
| FR-002 nível determinístico (`TICKS_POR_NIVEL = 900`, `NIVEL_MAXIMO = 12`, `fase_prepare`) | 12 implement | T011 |
| FR-003 mapa Q1/W2/E3/R6 sobre ações existentes, sem habilidades/teclas novas | 12 implement | T011 |
| FR-004 `CodigoRecusa::AcaoBloqueada` e ordem das guardas em `validar` | 12 implement | T011 |
| FR-005 HUD `Função`/`Nível`, hash com papel/nível e fronteira determinística | 12/27 | T011, T012 |
| FR-006 governança R1/R4/R6/[NV] e estilo Rust | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Hipótese promovida a cânone | Rotulação explícita nos 4 artefatos (FR-001/SC-001, passo 2 do roteiro): nível 1–12, 900 ticks e Q1/W2/E3/R6 são protótipo/derivação; GDD é lido, nunca reescrito |
| Progressão no tick errado | Incremento em `fase_prepare`, antes dos comandos, nos múltiplos positivos de 900; cobertura 0/899/900/1800/10800 no passo 4 e em T011 |
| Nível estourando o teto | Saturação explícita em `NIVEL_MAXIMO = 12`; ticks adicionais não mudam o valor (passo 4) |
| Jogador adicionado depois do início | Começa no nível 1 e sobe no próximo múltiplo de 900; simplificação documentada na spec, sem contador por jogador |
| Papel confundido com habilidade ou bônus | `Funcao` é identificador/rótulo; nenhum efeito, custo, alcance ou gate por papel; passos 3 e 5 |
| W/E/R virarem habilidades novas | Mapa incide somente sobre comandos já presentes em `ACOES_PERMITIDAS`; nenhuma ação, tecla, sinal ou efeito novo; passo 5 |
| Ações básicas travadas | `mover`, sinais e coleta permanecem livres de nível; passo 5 |
| `AcaoBloqueada` mascarando outras recusas | Ordem de guardas explícita: par/replay/ação desconhecida → nível → destino/alvo/Fôlego/saldo/Nó; passo 6 |
| Hash não determinístico | `BTreeMap` por `peer_id` e serialização estável de papel/nível; novos segmentos previsíveis; passo 7 cobre replay |
| HUD sobrescreve informações existentes | Apenas duas linhas acrescentadas em `texto_hud`; todas as demais linhas e teclas permanecem; passo 8 |
| Tecla/UI inventada | Nenhuma tecla nova: papel é fixo e o desbloqueio é passivo; passos 5 e 8 |
| Stack/protocolo/rede alterados | Fronteira explícita: nenhuma mudança em contratos, `PROTOCOL_VERSION`, Cargo, manifesto, CI ou rede; ADR-004 mantém rede bloqueada |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida de automação registrada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) antes de R7 (T012) |
| Captura visual fabricada | Sem surface automatizável: HUD visual e execução ao vivo **[NV]**; apenas inspeção e terminal são marcados como automatizáveis |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, sim, cliente e contratos intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]`.
- Rodar a suíte determinística e, se aplicável, o binário com `--minimizado`, conduzindo o roteiro de 10 passos e registrando capturas de terminal/HUD quando houver surface; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004 e as regras dos ciclos 049–051 intactas; **todos os números rotulados como `[HIPÓTESE]`/protótipo**; R4 antes de R7 `1.0.0-local`.
