# Shape — Armadilha Improvisada (E do Artesão)

- **Slug**: `armadilha-improvisada`
- **Created**: 2026-11-24T12:15Z
- **Recommended option**: Opção A — Armadilha Improvisada: controle de área determinístico com imobilização por `equipe` (única habilidade nomeada do §10.3 no corte)

## Options

### Option A — Armadilha Improvisada em estado determinístico (recomendada)

- **Sketch**: em `crates/sim/src/lib.rs`, declarar constantes `[HIPÓTESE]`/`escolha de protótipo` `CUSTO_ARMADILHA_FOLEGO = 25`, `CUSTO_ARMADILHA_MATERIAIS = 5`, `ALCANCE_ARMADILHA_M = 3.0`, `RAIO_ARMADILHA_M = 1.0`, `TICKS_ARMAR_ARMADILHA = 30`, `TICKS_IMOBILIZACAO_ARMADILHA = 60`, `TICKS_DURACAO_ARMADILHA = 1800`, `RECARGA_ARMADILHA_TICKS = 420`, `MAX_ARMADILHAS_POR_JOGADOR = 3`; definir `Armadilha { id, dono, posicao, ticks_armar, ticks_restantes }`, `Mundo.armadilhas: BTreeMap<u64, Armadilha>` e `proxima_armadilha_id`, e acrescentar `Jogador.equipe: u8`, `armadilha_recarga_ticks` e `imobilizado_ticks`. Registrar `"armadilha"` em `ACOES_PERMITIDAS` e nível 3 em `nivel_minimo_para_acao`; validar na ordem destino/alcance → Fôlego → Materiais → recarga, com `CodigoRecusa::ArmadilhaEmRecarga`, cota de uma por conjurador/tick e monotônicos só em comando aceito. Inserir `avancar_armadilhas()` após `avancar_escudos()` e antes de `fase_skills()`: decrementos por tick, disparo no primeiro inimigo a `<= 1 m` com `equipe != dono`, remoção da armadilha e imobilização por 60 ticks; sem disparo, expiração em 1800 ticks; limite de três por conjurador. `mover` é ignorado com `imobilizado_ticks > 0` sem mover posição nem monotônicos. O hash cobre os campos novos e as armadilhas em ordem de `id`. No cliente, `E` despacha `armadilha` na própria posição quando `funcao == Funcao::Artesao` e preserva `analise_materialista` para as demais funções; `mover_jogador`/`sincronizar_sim` respeitam a imobilização; HUD `Armadilhas: N` com `texto_hud` <100 linhas. Cinco testes nomeados elevam 146 → 151.
- **Appetite**: `small` — nove constantes, uma struct, cinco campos, uma ação e um nível na tabela existente, validação reaproveitada de `validar_analise_materialista`, uma fase no ciclo já existente, hash, tecla, bloqueio local, linha de HUD e cinco testes; sem contrato, protocolo, Cargo, CI, stack, rede ou core reescrito.
- **Trade-offs**: ganha a primeira habilidade de controle de área do Artesão com verdade observável e auditável por `hash_estado`, conectando o elenco BF-002b..v ao estilo de ciclos 054-056, ao custo de um corte deliberadamente parcial do §10.3: ficam de fora canalização interrompível, dano mágico, Desafios de Ilha e geometria de mapa. Impacto **5** (habilidade real com efeito sobre o próprio movimento e âncora escrita), confiança **4** (números e rotas lidos, todos rotulados `[HIPÓTESE]`/`escolha de protótipo`), fechamento **4** (jogador do Artesão e testes da sim nomeados), esforço⁻¹ **3** (nova struct, três campos em `Jogador`, dois em `Mundo`, nova fase de tick e cliente), risco invertido **3** (imobilização cruza sim e cliente; cota, ordem de disparo e hash exigem verificação). O rótulo de protótipo é o preço aceito: promover qualquer número a cânone ou estender o corte a ilhas/dano viola a fronteira. **720/3125**.
- **Rabbit holes**: magia, RNG, `Instant`/relógio externo, geometria de mapa, linha de visão, Desafios de Ilha, dano mágico, desarme, armadilhas em cadeia, Súditos com regra própria, promover números a cânone, acumular armadilhas além de três, duplicar débito ou avançar `seq`/`request_id` em comando recusado, mexer em `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`/CI/stack/rede, implementar as demais habilidades W/R, reescrever `avancar_tick` além da inserção da fase, prometer captura visual com janela minimizada ou estourar 100 linhas em `texto_hud`.

### Option B — Mapa/rotas e spawn por ilha (BF-003)

- **Sketch**: entregar um módulo determinístico de mapa com rotas e spawn por ilha, substituindo os quatro PNJs em posições fixas.
- **Appetite**: `large` (semanas) — exige geometria, tabela de rotas, regras de spawn e integração com as ilhas já presentes, além de números ainda `[HIPÓTESE]`.
- **Trade-offs**: impacto **5**, confiança **3**, fechamento **3**, esforço⁻¹ **2**, risco invertido **3** → **270/3125**. É frente legítima e de alto impacto, mas não tem corte fechado no contrato 057 e não entrega habilidade jogável neste ciclo.
- **Rabbit holes**: geometria, pathfinding, spawn determinístico por ilha, reequilíbrio de PNJs, rotas de Camponês inimigo, mudança em `specs/053-pnj-defensores`.

### Option C — HUD republica `hash_estado` (BF-014)

- **Sketch**: acrescentar ao painel do cliente o hash canônico corrente, para diagnóstico determinístico pelo jogador.
- **Appetite**: `small` (horas) — uma linha de HUD e nenhuma mudança na sim.
- **Trade-offs**: impacto **2**, confiança **5**, fechamento **1**, esforço⁻¹ **5**, risco invertido **5** → **250/3125** (reescalonado de 1125/3125 na decisão 12:15Z: exibir hash não muda comportamento de jogo). É barato e seguro, mas não cria regra observável nem consumidor novo além do diagnóstico.
- **Rabbit holes**: poluir `texto_hud` (já com 99 linhas), prometer captura visual, confundir diagnóstico com jogabilidade.

### Option D — Cliente agenda `mover` (BF-012)

- **Sketch**: fazer o cliente agendar o comando `mover` no Núcleo em vez de injetar posição direta em `sincronizar_sim`.
- **Appetite**: `medium` — toca no espelho local, no agendamento e na reconciliação com a sim.
- **Trade-offs**: impacto **4**, confiança **3**, fechamento **3**, esforço⁻¹ **2**, risco invertido **2** → **144/3125**. Corrige plumbing real, mas é invisível como habilidade e tem risco de unidade/dupla aplicação de movimento.
- **Rabbit holes**: duplo movimento, divergência cliente/servidor, unidades de tempo, mudança no espelho autoritativo sem teste.

### Option E — `contratos` consumido por binários (BF-013)

- **Sketch**: ligar o módulo `crates/contratos`, hoje desconectado, ao cliente e ao servidor headless.
- **Appetite**: `large` — mexe na fronteira de contrato/protocolo, explicitamente fora do ciclo.
- **Trade-offs**: impacto **5**, confiança **2**, fechamento **5**, esforço⁻¹ **1**, risco invertido **2** → **100/3125**. Consumidor protocolo/servidor, não o jogador; violaria a fronteira "sem contrato/protocolo" do contrato da sessão.
- **Rabbit holes**: `PROTOCOL_VERSION`, serialização compartilhada, compatibilidade com o formato `<tick>\t<sha256>`, ADR-004.

### Option F — Rede autoritativa real (BF-001)

- **Sketch**: adotar Lightyear ou emendar ADR-004 para transporte, predição e reconciliação cliente-servidor.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de habilidade determinística para infraestrutura de rede.
- **Trade-offs**: impacto **5**, confiança **2**, fechamento **5**, esforço⁻¹ **1**, risco invertido **1** → **50/3125**, **inelegível** por autorização (ADR-004; sem `lightyear` em `Cargo.toml`/`Cargo.lock`). Inverte a dependência determinismo → sincronização.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), predição de imobilização, resolução de concorrência de comandos, troca de engine sem novo ADR.

## Recommendation

Escolher **Opção A**. É a única que satisfaz as metas de `define.md` com fronteira mínima e lastro escrito: impacto **5** (o primeiro incremento nomeado do elenco de habilidades ganha efeito sobre o próprio movimento do jogador), confiança **4** (GDD §10.3:588-593 e contrato 057 fixam números e rotas, todos rotulados `[HIPÓTESE]`/`escolha de protótipo`), fechamento **4** (consumidor jogador do Artesão e testes nomeados), esforço⁻¹ **3** (struct, campos, ação, fase, hash, cliente e testes, sem stack nova) e risco invertido **3** (imobilização sim/cliente, cota, ordem de disparo e hash por verificar). B (270/3125) não tem corte fechado; C (250/3125) é diagnóstico; D (144/3125) e E (100/3125) são plumbing/fronteira proibida; F (50/3125) é inelegível por ADR-004. Scorecard único do ciclo: **720/3125**, repetido em `intake.md`, `research.md` e `decision.md`, sem pontuação paralela.

A recomendação **não** promove número a cânone, não implementa dano mágico, ilhas ou geometria, não toca contrato/protocolo/Cargo/CI/rede e não reescreve o core além da inserção de `avancar_armadilhas()`. A verificação do cliente é estrutural por desenho — com `--minimizado`, a linha `Armadilhas: N` renderizada fica **[NV]** e não é prometida como captura real.

## Out of Scope (for the recommended option)

- Promoção de 25/5/3 m/1 s/1 m/2 s/60 s/3/14 s ou de 30/60/1800/420 ticks a cânone; reabertura do contrato 057 ou dos rótulos do GDD §10.3.
- Canalização interrompível, dano mágico, Desafios de Ilha, pontos de coleta neutros, rotas de Camponês, mapa, geometria e linha de visão.
- Súditos inimigos com regra própria, desarme, armadilhas em cadeia ou imobilização de aliados.
- Magia, RNG, `Instant`/relógio externo, `async`, sockets, Lightyear, rede, protocolo, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` ou CI (ADR-004).
- Demais habilidades Q/W/R (Oficina de Campo, Sabotagem Estrutural etc.) e seus números — ciclos posteriores (BF-002b..v).
- Reescrita de `avancar_tick` além da inserção da fase, do formato `<tick>\t<sha256>` ou da whitelist derivada de `ACOES_PERMITIDAS`.
- Mudanças em `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`; **R6** (N/A por constituição v1.2.2, BF-007), issues simuladas e `gh` improvisado; gates, build, testes e formatadores neste assessment.
- Captura visual real onde não houver surface automatizável — **[NV]**, não prometida; cliente somente com `--minimizado`.

## Assumptions to Validate

- `PASSO_SEGUNDOS = 1/30` converte 1 s/2 s/60 s/14 s em 30/60/1800/420 ticks; a recarga de 420 ticks deriva dos 14 s do GDD e é escolha de protótipo, não número literal do §10.3.
- `Jogador.equipe` é inicializado de forma determinística na criação; aliados e Súditos aliados usam o mesmo valor e não disparam a armadilha, enquanto inimigos com `equipe != dono` disparam.
- A seleção do "primeiro inimigo" a `<= 1 m` segue ordem determinística de `peer_id` no `BTreeMap`, sem RNG; a distância usa `hypot` sem tolerância, como as validações existentes.
- `avancar_armadilhas()` decrementa arme, duração, recarga e imobilização uma vez por tick, em ordem estável, removendo a armadilha e imobilizando ao disparar; a ordem exata dos decrementos é escolha de implementação, não de regra — validar na especificação.
- A cota de três por conjurador e a cota de uma por conjurador por tick são compatíveis com a disciplina de comandos do ciclo 056; a quarta tentativa é recusada sem débito e sem avanço de `seq`/`request_id`.
- `mover` recusado por imobilização não altera posição nem monotônicos, e o cliente deixa de agenda/injetar movimento local no mesmo intervalo — sem divergência entre espelho e Núcleo.
- Cliente: `E` despacha `armadilha` somente para `Funcao::Artesao` e preserva `analise_materialista` para as demais funções; `texto_hud` ganha `Armadilhas: N` sem ultrapassar 100 linhas e sem remover linhas/teclas existentes.
- Os 5 testes novos cobrem custo 25+5, recusa além de 3 m, imobilização 60 ticks com aliados poupados, expiração 1800 com limite 3 e hash; os 146 existentes permanecem válidos (→ 151); estilo `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`. Tudo permanece **[NV]** até execução.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006 (detalhados em `define.md` e no handoff de `decision.md`); B..F são as alternativas de `research.md` e não recebem rastro de implementação neste ciclo. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica prevista em `specs/057-armadilha-improvisada/` (em construção), validação em `.specify/validations/armadilha-improvisada/` e release em `.specify/releases/armadilha-improvisada/`. Todo número de habilidade citado é `[HIPÓTESE]`/`escolha de protótipo`; nada aqui é cânone. **R6 é N/A** (BF-007) por constituição v1.2.2 e não recebe tarefa.
