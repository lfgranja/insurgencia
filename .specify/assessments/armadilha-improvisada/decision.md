# Decision — Armadilha Improvisada (E do Artesão)

- **Slug**: `armadilha-improvisada`
- **Decided**: 2026-11-24T12:15Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2, BF-007)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | O GDD §10.3:588-593 descreve a habilidade E do Artesão como `[HIPÓTESE]`, mas `ACOES_PERMITIDAS` não tem `armadilha`, `Jogador` não tem `equipe`/recarga/imobilização, `Mundo` não tem `armadilhas`, `avancar_tick` não tem a fase e `hash_estado` não vê nenhum desses campos; no cliente, `E` só analisa e `mover` é incondicional. |
| Força da evidência | strong | `STATE.md` lido com auditoria, backlog, decisão 12:15Z e DoD; GDD §10.3:588-593 lido com os rótulos e critérios; `crates/sim/src/lib.rs` verificado (`ACOES_PERMITIDAS:190-202`, `nivel_minimo_para_acao:214-220`, `Jogador:281-...`, `Mundo:462-...`, `avancar_tick:768-790`, `avancar_escudos:1065-1080`, `hash_estado:1903-1949`, `CodigoRecusa:1953-1991`, validações a partir de `:2000`); `crates/cliente/src/main.rs` verificado (`mover_jogador:612`, `solicitar_analise_materialista:923-955`, `sincronizar_sim:1042`, `texto_hud:1123-1221` com 99 linhas); constituição v1.2.2 lida. |
| Valor vs. inação | strong | 720/3125 à frente de mapa/rotas (270), HUD hash (250), cliente `mover` (144) e contratos (100); sem o recorte, o primeiro incremento nomeado do elenco de habilidades segue sem verdade observável e o Artesão sem controle de área. |
| Viabilidade / appetite | strong | `small`: nove constantes, uma struct, cinco campos, uma ação e um nível na tabela, validação reaproveitada, uma fase no ciclo existente, hash, tecla `E`, bloqueio local, HUD e cinco testes; sem `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede. |
| Alinhamento estratégico | strong | Preserva os rótulos: 25/5/3 m/1 s/1 m/2 s/60 s/3/14 s permanecem `[HIPÓTESE]` e 30/60/1800/420 ticks são `escolha de protótipo`; respeita ADR-004, a fronteira sem GDD canônico e a constituição v1.2.2 (R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, R6 N/A); demais habilidades ficam para ciclos posteriores. |
| Postura de risco | strong | Riscos nomeados e mitigados por decisão: ordem de validação, cota por conjurador/tick, limite de três, imunidade por `equipe`, imobilização coerente sim/cliente, decrementos sem `Instant`, hash em ordem de `id`, `texto_hud` <100 linhas e 5 testes novos com 146 preservados; R6 N/A sem issue simulada nem `gh` improvisado. |

## 80/20 (scorecard único do ciclo)

| Opção | Impacto × Confiança × Fechamento × Esforço⁻¹ × Risco⁻¹ ÷ 3125 | Resultado |
|---|---|---|
| Armadilha Improvisada (BF-002b, E do Artesão — escolhida) | 5×4×4×3×3 = 720/3125 | **720/3125** |
| Mapa/rotas e spawn por ilha (BF-003) | 5×3×3×2×3 = 270/3125 | 270/3125 |
| HUD republica `hash_estado` (BF-014) | 2×5×1×5×5 = 250/3125 | 250/3125 |
| Cliente agenda `mover` (BF-012) | 4×3×3×2×2 = 144/3125 | 144/3125 |
| `contratos` consumido por binários (BF-013) | 5×2×5×1×2 = 100/3125 | 100/3125 |
| Rede autoritativa real (BF-001) | 5×2×5×1×1 = 50/3125 — inelegível (ADR-004) | 50/3125 |

Critério do contrato — impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, escala 1–5, denominador **5⁵ = 3125**; o quarto e o quinto fatores são inversos de esforço e de risco (5 = menor esforço/risco, 1 = maior). Filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo. BF-014 foi reescalonado de 1125 para 250 na decisão 12:15Z porque exibir hash não muda o jogo; BF-001 permanece inelegível por autorização. Este é o **único** scorecard do assessment; os mesmos valores aparecem em `intake.md`, `research.md` e `shape.md`, sem pontuação paralela.

A Armadilha Improvisada vence porque é a única opção com **regra escrita no GDD e fronteira fechada no contrato simultaneamente**: o §10.3:588-593 fixou E do Artesão, 25 Fôlego, 5 Materiais, 1 s, 3 m, raio 1 m, 2 s de imobilização, 60 s de duração, três ativas e imunidade de aliados, e o contrato 057 fixou a conversão para ticks, a estrutura de estado, a ordem de validação, o ponto da fase, o predicado de `equipe`, a tecla, o bloqueio local, o hash e os cinco testes — sem tocar contrato, protocolo, Cargo, CI, stack, rede ou GDD canônico. Impacto **5** registra controle de área com efeito sobre o próprio movimento do jogador; confiança **4** registra que os números existem no GDD mas são `[HIPÓTESE]`; fechamento **4** nomeia jogador e testes; esforço⁻¹ **3** e risco invertido **3** registram honestamente nova struct, três campos em `Jogador`, dois em `Mundo`, nova fase de tick, imobilização em duas frentes e hash por verificar.

## Veredito e justificativa

**Veredito: go** — Opção A (Armadilha Improvisada: controle de área determinístico com imobilização por `equipe`), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 (`1.0.0-local`), R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, formatação, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. **Constantes rotuladas**: `CUSTO_ARMADILHA_FOLEGO = 25`, `CUSTO_ARMADILHA_MATERIAIS = 5`, `ALCANCE_ARMADILHA_M = 3.0`, `RAIO_ARMADILHA_M = 1.0`, `TICKS_ARMAR_ARMADILHA = 30`, `TICKS_IMOBILIZACAO_ARMADILHA = 60`, `TICKS_DURACAO_ARMADILHA = 1800`, `RECARGA_ARMADILHA_TICKS = 420` e `MAX_ARMADILHAS_POR_JOGADOR = 3` documentados como `[HIPÓTESE]`/`escolha de protótipo`, derivados de GDD §10.3:588-593 e de `PASSO_SEGUNDOS = 1/30`; nenhum número é promovido a cânone e o GDD não é reaberto.
2. **Estado novo**: `Armadilha { id, dono, posicao, ticks_armar, ticks_restantes }`, `Mundo.armadilhas: BTreeMap<u64, Armadilha>` com `proxima_armadilha_id`, e `Jogador.equipe: u8`, `armadilha_recarga_ticks` e `imobilizado_ticks`, inicializados de forma determinística, sem `HashMap`, RNG ou `Instant`.
3. **Ação e nível**: `"armadilha"` em `ACOES_PERMITIDAS` e nível 3 em `nivel_minimo_para_acao` (slot E), preservando as 13 ações existentes e a whitelist derivada do servidor headless.
4. **Validação ordenada**: destino/alcance (`<= 3 m` a partir da própria posição) → Fôlego `>= 25` → Materiais `>= 5` → recarga 0; `CodigoRecusa::ArmadilhaEmRecarga` na recarga; cota de uma armadilha por conjurador por tick; recusa sem criar armadilha, sem débito e sem avanço de `seq`/`request_id`.
5. **Fase determinística**: `avancar_armadilhas()` após `avancar_escudos()` e antes de `fase_skills()`, decrementando arme, duração, recarga e imobilização uma vez por tick em ordem estável; disparo no primeiro inimigo a `<= 1 m` com `equipe != dono`, remoção da armadilha e imobilização por 60 ticks; sem alvo, expiração em 1800 ticks; máximo de três por conjurador.
6. **Movimento e hash**: `mover` ignorado com `imobilizado_ticks > 0` sem mudar posição nem monotônicos; `equipe`, `armadilha_recarga_ticks`, `imobilizado_ticks` e as armadilhas em ordem crescente de `id` entram em `hash_estado`, preservando SHA-256 canônico e o formato `<tick>\t<sha256>`.
7. **Cliente e HUD**: tecla `E` despacha `armadilha` com `destino` na própria posição quando `funcao == Funcao::Artesao`, preservando `analise_materialista` para as demais funções; `mover_jogador` e `sincronizar_sim` não agenda/injeta movimento local enquanto imobilizado; HUD acrescenta `Armadilhas: N` com `texto_hud` **<100 linhas**, sem remover linhas ou teclas existentes; cliente somente com `--minimizado`.
8. **Testes e estilo**: 5 novos testes em `crates/sim/tests/simulacao.rs` (`armadilha_custa_25_folego_e_5_materiais`, `armadilha_recusa_destino_alem_de_3_m`, `armadilha_imobiliza_inimigo_por_60_ticks_e_poupa_aliados`, `armadilha_expira_apos_1800_ticks_e_limite_de_tres`, `hash_reflete_armadilha_e_imobilizacao`), 146 existentes preservados (→ 151); `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.
9. **Fronteira preservada**: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack, rede, Lightyear, sockets, GDD canônico ou habilidades W/R; nenhuma mudança em `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`.
10. **Constituição v1.2.2**: R1 `go`, R3 0 clarificações, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** (BF-007) — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Número promovido a cânone (GDD §10.3:588-593)**: 25/5/3 m/1 s/1 m/2 s/60 s/3/14 s são `[HIPÓTESE]`; 30/60/1800/420 ticks são escolhas de protótipo. Se a spec promover qualquer valor a dogma, viola este assessment — não reabrir o contrato nem o GDD.
- **Imobilização divergente sim/cliente**: a sim recusa `mover` sem mutar posição ou monotônicos e o cliente deve parar de agendar/injetar movimento no mesmo intervalo; qualquer divergência produz estado local diferente do Núcleo. Risco invertido **3** (não 5) por essa razão; **[NV]** até execução.
- **Disciplina de tick e hash**: decrementos de quatro relógios em ordem determinística sem `Instant`, cota por conjurador/tick, limite de três e serialização estável de jogadores e armadilhas em ordem de `id` exigem verificação; permanecem **[NV]**.
- **Imunidade e seleção de alvo**: `equipe` precisa ser inicializado de forma determinística; o "primeiro inimigo" depende de ordem estável de `peer_id` no `BTreeMap`. Sem isso, aliados podem ser imobilizados ou o disparo pode variar entre execuções.
- **Recursos e recusa**: quarta armadilha e recarga ativa devem ser recusadas sem débito e sem avanço de `seq`/`request_id`; duplo débito seria regressão contra os testes existentes de Fôlego/Materiais.
- **Corte parcial do GDD**: canalização interrompível, dano mágico, Súditos com regra própria e Desafios de Ilha ficam fora; se a spec os incluir, viola a fronteira e o appetite `small`.
- **HUD**: `texto_hud` tem 99 linhas no checkout; a linha `Armadilhas: N` precisa entrar sem ultrapassar 100, sob pena de quebrar o contrato.
- **Demais habilidades e rede**: não são pontuação a renegociar (BF-003 270, BF-014 250, BF-012 144, BF-013 100, BF-001 50 inelegível); nenhum rastro FR/SC/T lhes é atribuído neste ciclo.
- **Verificação visual**: cliente somente com `--minimizado` e ausência de surface automatizável de captura tornam `Armadilhas: N` renderizada **[NV]**; a prova é estrutural. Não prometer screenshot real.
- **Dívida R6**: N/A por constituição v1.2.2 (BF-007), não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação geral**: gates, build, testes, formatadores, R4 e R7 (`1.0.0-local`) permanecem **[NV]** — nenhum foi executado por este agente; nenhum commit, push ou PR foi feito.

## Handoff — `/speckit.specify`

- **Problem**: o GDD §10.3:588-593 descreve a Armadilha Improvisada do Artesão como `[HIPÓTESE]`, mas o Núcleo não tem ação, estado, fase, recusa nem hash para ela, e o cliente não a despacha nem bloqueia movimento: a regra segue sem verdade observável.
- **Chosen approach**: Opção A — constantes rotuladas (25/5/3 m/1 m/30/60/1800/420/3), `Armadilha` e campos em `Mundo`/`Jogador`, `"armadilha"` nível 3 com validação ordenada e cota, `avancar_armadilhas()` no ponto exato do ciclo, imunidade por `equipe`, `mover` ignorado enquanto imobilizado, hash refletivo, tecla `E` por `Funcao::Artesao`, bloqueio local e HUD `Armadilhas: N`. Todo número `[HIPÓTESE]`/`escolha de protótipo`.
- **In scope**: constantes e rótulos, struct e campos, ação/nível/recusa/validação, fase de tick com arme/disparo/expiração/limite, imobilização e imunidade, hash, cliente/HUD e 5 testes novos com 146 preservados. **Out of scope**: dano mágico, canalização, ilhas, mapa, demais W/R, rede/ADR-004, contratos/protocolo/Cargo/CI/stack, GDD canônico e **R6 (N/A)**.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: ordem exata dos decrementos em `avancar_armadilhas()` e ponto de serialização no `hash_estado`; critério determinístico de "primeiro inimigo" em `<= 1 m`; inicialização de `equipe` e scaffolding do espelho para observar aliado/inimigo; compatibilidade da cota de um por conjurador/tick com a disciplina do ciclo 056; conversão 1 s/2 s/60 s/14 s → 30/60/1800/420 ticks a 30 Hz — validar na especificação e no roteiro de verificação.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | constantes rotuladas: 25 Fôlego, 5 Materiais, 3 m, 1 s, 1 m, 2 s, 60 s, 3 ativas, 14 s → 30/60/1800/420 ticks | GDD §10.3:588-593; contrato da sessão; `intake.md` |
| FR-002 | `Armadilha { id, dono, posicao, ticks_armar, ticks_restantes }`, `Mundo.armadilhas`/`proxima_armadilha_id`, `Jogador.equipe`/`armadilha_recarga_ticks`/`imobilizado_ticks` | contrato; `crates/sim/src/lib.rs:281-360,462-510`; `define.md` |
| FR-003 | `"armadilha"` em `ACOES_PERMITIDAS`, nível 3 em `nivel_minimo_para_acao`, validação destino/alcance → Fôlego → Materiais → recarga, `CodigoRecusa::ArmadilhaEmRecarga`, cota por conjurador/tick | `crates/sim/src/lib.rs:190-220,1953-1991,2000-2085`; `shape.md` · Opção A |
| FR-004 | `avancar_armadilhas()` após `avancar_escudos()` e antes de `fase_skills()`: arme 30, disparo a `<= 1 m` com `equipe != dono`, remoção, imobilização 60, expiração 1800, limite 3 | `crates/sim/src/lib.rs:768-790,1065-1080`; contrato; `shape.md` · Opção A |
| FR-005 | `mover` ignorado sem mover posição nem monotônicos; hash cobre campos e armadilhas em ordem de `id` | `crates/sim/src/lib.rs:1903-1949`; contrato; `shape.md` · Opção A |
| FR-006 | cliente (tecla `E` por `funcao`, destino próprio, bloqueio local, HUD `Armadilhas: N`, `texto_hud` <100 linhas), 5 testes novos + 146 preservados, fronteira e governança | contrato; `.specify/memory/constitution.md`; `intake.md` |
| SC-001 | custo 25 Fôlego + 5 Materiais, arme 30 e recarga 420 em comando aceito — `armadilha_custa_25_folego_e_5_materiais` | `define.md` · Success Metrics 1 |
| SC-002 | recusa de destino além de 3 m sem débito nem monotônicos — `armadilha_recusa_destino_alem_de_3_m` | `define.md` · Success Metrics 2 |
| SC-003 | imobilização de inimigo por 60 ticks e imunidade de aliados por `equipe` — `armadilha_imobiliza_inimigo_por_60_ticks_e_poupa_aliados` | `define.md` · Success Metrics 3 |
| SC-004 | expiração em 1800 ticks e limite de três ativas — `armadilha_expira_apos_1800_ticks_e_limite_de_tres` | `define.md` · Success Metrics 4 |
| SC-005 | hash reflete armadilha e imobilização em ordem de `id` — `hash_reflete_armadilha_e_imobilizacao`; cliente e HUD permanecem coerentes | `define.md` · Success Metrics 5-6 |
| SC-006 | 5 testes novos e 146 preservados (→ 151); estilo Rust e fronteira intactos | `define.md` · Success Metrics 7 |
| T001 | 00 constituição/branch — worktree `armadilha-057`, branch `feature/057-armadilha-improvisada`, base `661890f`; R1 `go`, R6 N/A | contrato da sessão; constituição v1.2.2 |
| T002 | 01 intake + 02 research — scorecard 720/3125, fontes lidas, nada executado | `intake.md`; `research.md` |
| T003 | 03 define + 04 shape — problema, métricas e opções A-F + rede inelegível | `define.md`; `shape.md` |
| T004 | 05 decide + 07 clarify — R1 `go`, R3 = 0, handoff ao `/speckit.specify` | este `decision.md` |
| T005 | 06 specify — `spec.md` com US/FR/SC, constantes, estado, ação, fase, cliente e HUD | `specs/057-armadilha-improvisada/spec.md` (em construção) |
| T006 | 08 plan — roteiro de 10 passos, fronteira e R4 antes de R7 | `specs/057-armadilha-improvisada/plan.md` (a produzir) |
| T007 | 09 checklist de requisitos — um item por FR-001..FR-006 | `specs/057-armadilha-improvisada/checklists/requirements.md` (a produzir) |
| T008 | revisão do checklist — 6 FR/6 SC verificáveis por inspeção | `specs/057-armadilha-improvisada/checklists/requirements.md` (a produzir) |
| T009 | 10 tasks — T001..T012 com DAG linear, `tasks` antes de `analyze` | `specs/057-armadilha-improvisada/tasks.md` (a produzir) |
| T010 | 11 analyze (READ-ONLY) — depois de `tasks`, sem edição de insumos | `specs/057-armadilha-improvisada/analyze.md` (a produzir) |
| T011 | 12 implement + 13 converge + 27 validate (R4) — sim, cliente, 5 testes e roteiro 10/10 | `crates/sim`, `crates/cliente`, `.specify/validations/armadilha-improvisada/` |
| T012 | 26 release (R7 `1.0.0-local`) — changelog por FR/T, rastreabilidade e rollback documentado, após R4 | `.specify/releases/armadilha-improvisada/release.md` (a produzir) |

Os nomes definitivos de T001..T012 são os de `specs/057-armadilha-improvisada/tasks.md`; a coluna "Eco" registra apenas a cobertura deste assessment. **R6 é N/A** por constituição v1.2.2 (BF-007) e não recebe tarefa. Nada aqui é especificação, plano ou implementação, e nada foi executado. Todo resultado de verificação permanece **[NV]**.

Veredito: go
