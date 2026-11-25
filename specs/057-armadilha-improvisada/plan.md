# Implementation Plan — 057-armadilha-improvisada

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; `converge` é **append-only**; R1 `go`; R4 (validate) **condicionado ao roteiro de verificação** e antes de R7 (release `1.0.0-local`). **R6 é N/A/bloqueado** (BF-007) por constituição v1.2.2: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação fica registrada, não executada. Promoção `dev→main` e deploy são **SWE-exclusivos**.

**Fronteira explícita**: este agente escreve **somente** `specs/057-armadilha-improvisada/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/armadilha-057/` (branch `feature/057-armadilha-improvisada`, base `origin/dev = 661890f`). O `analyze.md` já presente no diretório é **insumo READ-ONLY**, não reescrito por esta fronteira: o total de artefatos SDD do ciclo é cinco, dos quais quatro pertencem a este agente. Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `.specify/assessments/armadilha-improvisada/`, `.specify/validations/armadilha-improvisada/`, `.specify/releases/armadilha-improvisada/`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, GDD, manifesto, CI ou documentos derivados. O código de implementação (`crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs`, `crates/cliente/src/main.rs`) pertence a agentes de implementação na etapa 12. Não executa gates, build, testes ou formatadores; não abre janelas; não faz commit, push ou PR. Tudo o que depende de execução externa fica **[NV]**.

**Rotulagem obrigatória**: a âncora é GDD §10.3:588-593 (E do Artesão: 25 Fôlego + 5 Materiais, 1 s, 3 m, 14 s de recarga, 1 m de raio, 2 s de imobilização, 60 s de duração, três ativas e aliados imunes). **Todos os números do corte são `[HIPÓTESE]`/protótipo**: 3, 25, 5, 3.0 m, 1.0 m, 30, 60, 1800, 3 e 420 ticks, com `PASSO_SEGUNDOS = 1/30`. Nenhum valor pode ser apresentado como cânone em qualquer um dos 4 artefatos, e o GDD é lido, nunca reescrito. O Desafio de Ilha e dano mágico ficam fora da DoD.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A/bloqueado** (BF-007, não executada, não simulada); R1 `go`; `tasks` antes de `analyze`; `converge` append-only; R4 condicionado ao roteiro e antes de R7 `1.0.0-local`; promoção `dev→main` e deploy SWE-exclusivos.
2. **01 Intake**: sessão `20261124T121311Z-armadilha-improvisada`, título `insurgencia - armadilha improvisada`; 80/20 = **720/3125** (`armadilha-improvisada`, BF-002b, `5×4×4×3×3`) contra mapa/rotas **270/3125**, HUD `hash_estado` **250/3125** (reescalonado de 1125), cliente `mover` **144/3125**, contratos **100/3125** e rede **50/3125** (inelegível por ADR-004); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵ = 3125; decisão e DoD em `.specify/memory/STATE.md` (2026-11-24T12:15Z).
3. **02 Research**: GDD §10.3:588-593 (regra E do Artesão e critérios de aceitação), §10.1 (Q1/W2/E3/R6) e §8.1 (convenção `[HIPÓTESE]`); `.specify/memory/STATE.md` (decisão 12:15Z e DoD 0.9); constituição v1.2.2; `analyze.md` A1..A10 (insumo lido, não reescrito); `crates/sim/src/lib.rs` (`PASSO_SEGUNDOS`, `Jogador`, `Mundo`, `ACOES_PERMITIDAS`, `nivel_minimo_para_acao`, `ValidacaoComandos::validar`, `CodigoRecusa`, `avancar_tick`, `avancar_escudos`, `fase_skills`, `serializar_jogadores`, `hash_estado`, `BTreeMap`/`BTreeSet`, `ponto_igual`); `crates/cliente/src/main.rs` (`KeyCode::KeyE`, `analise_materialista`, `mover_jogador`, `sincronizar_sim`, `texto_hud`, `Funcao`); inventário de **146 testes** ancorado em `661890f`.
4. **03 Define (problema)**: o baseline não tem `Armadilha`, `Mundo.armadilhas`, `proxima_armadilha_id`, `equipe`, `armadilha_recarga_ticks`, `imobilizado_ticks`, ação `armadilha`, fase `avancar_armadilhas`, reflexo no hash, despacho de `E` por `funcao`, HUD `Armadilhas: N` nem bloqueio de movimento/sincronia; logo os critérios de GDD §10.3:591-593 não são observáveis no estado determinístico. Critérios observáveis por FR/SC, todos os números rotulados como protótipo.
5. **04 Shape (concept)**: 6 opções avaliadas no scorecard; recomendação = `armadilha-improvisada`, reaproveitando `BTreeMap`/`BTreeSet`, `ACOES_PERMITIDAS`, `ValidacaoComandos`, `CodigoRecusa`, `avancar_tick`, `hash_estado` e `texto_hud`, sem stack, mapa, contrato, protocolo ou janela nova. Aliados são poupados por `equipe`, não por `funcao`; Súditos não ganham campo novo.
6. **05 Decide**: scorecard **720/3125**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A/bloqueado; DoD 0.9 nomeia consumidor (jogador do Artesão no cliente Bevy e testes de integração da sim) e comandos de prova (`cargo test -p insurgencia-sim --test simulacao`, `cargo test -p insurgencia-cliente --lib`, `./verify.sh` com gates remotos, roteiro 10/10).
7. **06 Specify**: substituir o diretório sem estes artefatos por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006 no mapa de governança (FR-001 regra/números `[HIPÓTESE]`, FR-002 estado/constantes, FR-003 ação/validação/cota, FR-004 fase/arme/disparo/imobilização/expiração/movimento, FR-005 hash, FR-006 cliente/testes/estilo/fronteira/governança), com scorecard, edge cases e fronteira explícita.
8. **07 Clarify**: nível 3 é o único limiar novo; `equipe` começa em `0` e inimigo exige atribuição explícita de equipe diferente; "inimigo com `equipe != dono`" compara com a equipe do dono obtida por `armadilha.dono`; o quarto artefato é recusado com `SaldoInsuficiente` e a única variante nova é `ArmadilhaEmRecarga`; a armadilha e a imobilização escritas no tick `T` só decrescem em `T+1` porque `avancar_armadilhas()` corre antes de `fase_skills()`; `mover` imobilizado não avança monotônicos; `E` despacha por `funcao`, com `texto_hud` < 100 linhas; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de **10 passos**, conjugação dos achados A1..A10 de `analyze.md` aos 6 FR no mapa canônico, R4 condicionado antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação, execução ou verificação ao vivo.
11. **Revisão do checklist**: 6 US/6 FR/6 SC verificáveis por inspeção; rastreabilidade idêntica à de `analyze.md` A1..A10 e à DoD em `STATE.md`; números hipotéticos distinguidos de cânone; fronteira confirmada; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG linear; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: A1..A10, convergência e veredito — já registrado em `analyze.md` neste worktree e **lido como insumo**, não reescrito por esta fronteira; a ordem normativa continua com `tasks` (T009) antes de `analyze` (T010).
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 em `crates/sim/src/lib.rs` (tipo `Armadilha`, `Mundo.armadilhas`/`proxima_armadilha_id`, três campos de `Jogador`, nove constantes documentadas e rotuladas, `"armadilha"` nível 3, validação na ordem destino/alcance → Fôlego → Materiais → recarga → limite três, `ArmadilhaEmRecarga`, cota por conjurador/tick, `avancar_armadilhas()` entre `avancar_escudos()` e `fase_skills()`, disparo por `equipe` em ordem determinística, imobilização por atribuição, expiração em `T+1800`, `mover` ignorado, hash cobrindo campos e armadilhas por `id`) e em `crates/cliente/src/main.rs` (tecla `E` por `funcao`, guards de `mover_jogador`/`sincronizar_sim`, HUD `Armadilhas: N` com `texto_hud` < 100 linhas), além dos cinco testes nomeados em `crates/sim/tests/simulacao.rs` (146 → 151), com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`; cliente somente `--minimizado`. Fora da fronteira deste agente.
15. **13 Converge (append-only)**: entrada datada em `specs/057-armadilha-improvisada/converge.md`, nunca reescrita; **[NV]** para execução e capturas. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando **documentalmente e condicionado ao roteiro de 10 passos**; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7. Registro em `.specify/validations/armadilha-improvisada/`.
17. **26 Release (R7)**: `release.md` em `.specify/releases/armadilha-improvisada/` versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback documentado (nunca executado), somente após R4.

> Os números reservados pela constituição para especificações de implementação não utilizadas neste ciclo permanecem vagos; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/armadilha-improvisada/test-roadmap.md` (fora da fronteira deste agente). Superfície: inspeção de `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs`, `crates/sim/tests/simulacao.rs` e da fronteira de arquivos; execução da suíte (`./verify.sh`, esperado **151 testes**) e do binário **somente com `--minimizado`** pelo orquestrador. Não há surface automatizável para captura visual real do HUD neste alvo headless: toda captura visual/tela é **[NV]** até verificação humana. Rollback documental: corrigir a fonte (spec/plano/código), nunca a expectativa; limpar capturas em `$TMP`.

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD`, `date -u`, listar os artefatos e conferir a ordem `tasks` antes de `analyze` | base `661890f` (`origin/dev`), branch `feature/057-armadilha-improvisada`, sessão `20261124T121311Z-armadilha-improvisada`, scorecard **720/3125**; `STATE.md`, `AGENTS.md`, `concurrency.jsonl`, `analyze.md` e paths alheiros intactos; R6 fora do fluxo, promoção/deploy SWE-exclusivos | sim (inspeção/terminal) |
| 2 | Conferir 6 US, 6 FR e 6 SC, o mapeamento um-para-um no checklist e a rotulação de 3/25/5/3 m/1 m/30/60/1800/420 como `[HIPÓTESE]` nos 4 artefatos | rastreabilidade FR-001..FR-006 idêntica à de A1..A10 e à DoD; 30/60/420/1800 derivam de 30 Hz; GDD intacto, Desafio de Ilha e dano mágico fora da DoD; nenhum número promovido a cânone | sim (inspeção) |
| 3 | Inspecionar `Armadilha`, `Mundo.armadilhas`/`proxima_armadilha_id`, os três campos de `Jogador` e as nove constantes | cinco campos documentados, `BTreeMap` por `id` começando em 1, `equipe = 0`/ticks zerados na criação, constantes rotuladas; nenhum `HashMap`, uuid, RNG, `Instant`, espelho cliente ou estrutura paralela | sim (inspeção) |
| 4 | Validar `armadilha` no nível 3 com nível 2, destino ausente/fora da própria posição/a 3,01 m, fôlego 24, materiais 4, recarga 420, quatro ativas e dois comandos do mesmo conjurador no tick | aceita somente com 25/5/recarga 0 e até três ativas; recusas `AcaoBloqueada`/`DestinoInvalido`/`FolegoInsuficiente`/`SaldoInsuficiente`/`ArmadilhaEmRecarga` na ordem documentada; cota de um por conjurador/tick; monotônicos e saldo só em aceite | sim (inspeção/cálculo) |
| 5 | Aplicar uma armadilha aceita e percorrer os ticks `T`, `T+29`, `T+30`, `T+419`, `T+420`, `T+1799` e `T+1800` | arme 30→1→0, recarga 420→1→0, duração 1800→1→0, sem decremento em `T`; `id` estritamente crescente; quarta recusada sem substituir a mais antiga; nenhum valor negativo | sim (inspeção/cálculo) |
| 6 | Colocar aliado (`equipe` igual) e inimigo (`equipe` diferente) no raio de 1 m; verificar disparo, dano zero, remoção, imobilização 60 e `mover` durante a imobilização | somente o inimigo é imobilizado por 60 ticks por atribuição; aliado segue livre; armadilha removida no disparo; `mover` não muda posição, progresso nem monotônicos; nenhum `Instant`/RNG | sim (inspeção/cálculo) |
| 7 | Comparar `hash_estado` entre replays idênticos e após mutar cada campo novo/campo de armadilha; conferir ordem de `id` e de `peer_id` | hashes idênticos no replay; segmentos refletem `equipe`, recarga, imobilização e `id`/`dono`/`posicao`/`ticks_armar`/`ticks_restantes`; nenhum segmento existente removido | sim (inspeção) |
| 8 | Inspecionar o handler de `KeyCode::KeyE`, `funcao == Funcao::Artesao`, os guards de `mover_jogador`/`sincronizar_sim` e `texto_hud` | Artesão agenda `armadilha` com destino na própria posição; demais funções mantêm `analise_materialista`; imobilizado não move/sincroniza; `Armadilhas: N` preserva linhas/teclas e `texto_hud` tem < 100 linhas; HUD visual **[NV]** | sim (inspeção de código) |
| 9 | Conferir os cinco testes nomeados, 146 preservados (151), ausência de janela normal, relógio externo, RNG, I/O, rede e surface de protocolo; verificar `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]` | estilo Rust v1.2.2 respeitado (`#[expect(...)]` como atributo de lint permitido); `cargo test -p insurgencia-sim --test simulacao` e `cargo test -p insurgencia-cliente --lib` executados pelo orquestrador; nenhuma alteração em contratos, Cargo, CI, Lightyear ou sockets | parcial (inspeção); testes **[NV]** |
| 10 | Governança, R6 N/A, rollback e rótulos: conferir `tasks` antes de `analyze`, R4 condicionado e antes de R7 `1.0.0-local`, `converge` append-only e **[NV]** em toda captura, além de `./verify.sh` remoto `gates`+`rust` e roteiro 10/10 | ordem v1.2.2 respeitada; R6 N/A (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado); promoção/deploy não executados; nenhum build/gate/commit/push/PR executado por este agente; fonte corrigida se um passo falhar | parcial (revisão humana) |

## Correspondência dos achados A1..A10 (`analyze.md`) aos 6 FR

| Achado | FR |
|:--|:--|
| A1 decisão **720/3125**, regra de GDD §10.3:588-593, rotulação e Desafio de Ilha/dano mágico fora da DoD | FR-001 |
| A2 `Armadilha` com cinco campos, `armadilhas: BTreeMap<u64, Armadilha>` e `proxima_armadilha_id` | FR-002 |
| A3 três campos de `Jogador` (`equipe`, `armadilha_recarga_ticks`, `imobilizado_ticks`) | FR-002 |
| A4 nove constantes documentadas e rotuladas, derivadas de 30 Hz | FR-002 |
| A5 `"armadilha"` em `ACOES_PERMITIDAS` no nível 3, sem contrato/protocolo/Cargo/CI | FR-003 |
| A6 validação destino/alcance → Fôlego → Materiais → recarga, `ArmadilhaEmRecarga`, cota e monotônicos | FR-003 |
| A7 `avancar_armadilhas()` após `avancar_escudos()` e antes de `fase_skills()`, arme/disparo por `equipe`/imobilização sem dano | FR-004 |
| A8 expiração em 1800, teto de três com `SaldoInsuficiente`, ids crescentes e `mover` ignorado | FR-004 |
| A9 hash com os três campos e as armadilhas por `id`; cliente `E` por `funcao`, guards de movimento/sincronia, `Armadilhas: N` e `texto_hud` < 100 linhas | FR-005 (hash), FR-006 (cliente) |
| A10 cinco testes nomeados (146 → 151), estilo Rust, `tasks` antes de `analyze`, R6 N/A, R4 antes de R7 e **[NV]** | FR-006 |

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 regra de GDD §10.3:588-593 e números `[HIPÓTESE]` | 01/02/07 | T002, T004, T005 |
| FR-002 `Armadilha`, `Mundo`, campos de `Jogador` e nove constantes | 12 implement | T011 |
| FR-003 ação nível 3, validação na ordem, cota e recusas | 12 implement | T011 |
| FR-004 fase, arme, disparo por `equipe`, imobilização, expiração e `mover` | 12 implement | T011 |
| FR-005 hash cobrindo campos e armadilhas por `id` | 12/27 | T011, T012 |
| FR-006 cliente `E`/HUD/guards, cinco testes, 146 preservados, estilo e governança | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Hipótese promovida a cânone ou GDD reescrito | Rotulação explícita nos 4 artefatos (FR-001, passo 2): GDD §10.3:588-593 é lido; 30/60/420/1800 derivam de `PASSO_SEGUNDOS = 1/30`; Desafio de Ilha fora da DoD |
| Aliado ativado por engano | Imunidade por `equipe` do dono obtido via `armadilha.dono`; `funcao` nunca é sinônimo de equipe; `equipe = 0` por padrão e inimigo exige atribuição explícita; passo 6 |
| Súdito tratado como inimigo ou campo inventado | Súditos não têm `equipe` neste corte; critério GDD registrado como não coberto; nenhum dano a Súditos pela armadilha |
| Armadilha disparando antes do arme ou em decremento duplo | `ticks_armar = 30`, arme zerado em `T+30`; `avancar_armadilhas()` uma vez por tick, antes de `fase_skills()`, sem decremento no tick de aplicação; passos 5 e 6 |
| Expiração só ao ativar ou id reutilizado | `ticks_restantes = 1800` removido em `T+1800` mesmo sem ativação; `proxima_armadilha_id` estritamente crescente e remoção em ordem de `id`; passo 5 |
| Quarta armadilha substituindo a mais antiga | Recusa na validação com `SaldoInsuficiente` quando há três ativas do mesmo `dono`; passo 4 |
| Recarga burlada ou duplo comando no tick | `ArmadilhaEmRecarga` distinta de `AcaoBloqueada`; recarga por conjurador, cota `BTreeSet<u64>` consumida somente em aceite, sem débito parcial; passo 4 |
| Jogador imobilizado continuando a se mover | `mover` ignorado em `fase_skills` com `imobilizado_ticks > 0`, sem posição/progresso/monotônicos; guards locais em `mover_jogador`/`sincronizar_sim`; passos 6 e 8 |
| `E` com dois despachos ou HUD acima de 100 linhas | Despacho exclusivo por `funcao == Funcao::Artesao`, senão `analise_materialista`; `texto_hud` com < 100 linhas; passo 8 |
| Hash cego ao novo estado ou ordem instável | Três campos no segmento de jogadores por `peer_id` e armadilhas por `id`; passo 7 |
| Ação virar contrato, habilidade extra ou magia | `"armadilha"` apenas no sim; whitelist derivada de `ACOES_PERMITIDAS` (ciclo 056); sem dano, RNG, visibilidade, mapa, item ou campo de protocolo; passos 4 e 9 |
| Stack/protocolo/rede alterados ou janela normal aberta | Fronteira explícita: nenhuma mudança em contratos, `PROTOCOL_VERSION`, Cargo, manifesto, CI ou rede; ADR-004 mantém rede bloqueada; cliente somente `--minimizado`; passos 9 e 10 |
| R6 adotada, issue simulada ou promoção indevida | R6 N/A/bloqueado por v1.2.2 (BF-007); PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; `dev→main` e deploy SWE-exclusivos |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) condicionado e antes de R7 (T012); passo 10 |
| Verde afirmado sem execução | Gates/build/testes/capturas **[NV]** até o orquestrador; nada afirmado como verde sem saída colada |
| Fronteira estourada | Só 4 artefatos SDD escritos por este agente; `analyze.md` lido, não reescrito; `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, assessments, validations, releases, sim, cliente e contratos intactos |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar `./verify.sh` e os gates remotos `gates`+`rust`; rodar `cargo test -p insurgencia-sim --test simulacao` (esperado **151**) e `cargo test -p insurgencia-cliente --lib`; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]`.
- Rodar o roteiro de 10 passos e o binário **somente com `--minimizado`**, registrando capturas de terminal/HUD quando houver surface; nenhuma janela normal. Comparar notas com o estado ao vivo, sem afirmar verde sem execução.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004 e **todos os números rotulados como `[HIPÓTESE]`/protótipo, nunca cânone**; R4 condicionado e antes de R7 `1.0.0-local`.
- Corrigir qualquer divergência na fonte (spec, plano, checklist, tasks ou código), nunca afrouxando expectativas de teste; rollback documentado, nunca executado neste ciclo. Promover `dev→main` e publicar somente por decisão do SWE.
