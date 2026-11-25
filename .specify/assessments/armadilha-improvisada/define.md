# Define — Armadilha Improvisada (E do Artesão)

- **Slug**: `armadilha-improvisada`
- **Created**: 2026-11-24T12:15Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (decisão 80/20 e DoD em `.specify/memory/STATE.md`; GDD §10.3:588-593; `crates/sim/src/lib.rs`; `crates/cliente/src/main.rs`; constituição v1.2.2)
- **Scorecard único do ciclo**: `armadilha-improvisada` **720/3125** (5×4×4×3×3); mapa/rotas BF-003 270/3125; HUD hash BF-014 250/3125; cliente `mover` BF-012 144/3125; contratos BF-013 100/3125; rede BF-001 50/3125 (inelegível, ADR-004). Nenhuma pontuação paralela.

## Problem Statement

O GDD prevê a Armadilha Improvisada (E do Artesão) como `[HIPÓTESE]`, mas o protótipo determinístico não a torna observável: `crates/sim/src/lib.rs` não tem ação `armadilha`, não tem `Armadilha`, `Mundo.armadilhas`, `Jogador.equipe`, `armadilha_recarga_ticks` ou `imobilizado_ticks`, `avancar_tick` não tem fase de armadilhas, `CodigoRecusa` não tem `ArmadilhaEmRecarga` e `hash_estado` não vê estado de armadilha nem imobilização. No cliente, `E` despacha apenas `analise_materialista`, `mover_jogador`/`sincronizar_sim` movem sem consultar imobilização e a HUD não exibe `Armadilhas:`. Sem esse recorte, o Artesão não tem controle de área verificável e o Núcleo não consegue auditar quem foi imobilizado, por quanto tempo ou se um aliado foi poupado. O problema é de **observabilidade e autoridade do estado de controle de área**, não de nova stack, regra canônica ou rede.

## Affected Users & Stakeholders

- **Users**: jogadores no cliente standalone, especialmente quem escolhe Artesão — precisam armar com `E` na própria posição, ver `Armadilhas: N` e perceber que inimigos param por 60 ticks enquanto aliados atravessam; quem inspeciona a sim — precisa de estado de armadilha/imobilização determinístico, serializado no hash, com arme, expiração, cota e imunidade auditáveis.
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de uma fase `avancar_armadilhas()` no ponto exato do ciclo, com decrementos sem `Instant`, disparo determinístico por `id` e recusa sem mutação de estado; mantenedores do cliente — precisam de despacho por `funcao` sem perder a análise do Intelectual, bloqueio de movimento local coerente com a sim e HUD abaixo de 100 linhas; designer — precisa que 25 Fôlego, 5 Materiais, 3 m, 1 s, 1 m, 2 s, 60 s, 3 ativas e 14 s de recarga sejam lidos como `[HIPÓTESE]`/`escolha de protótipo`, e que os Desafios de Ilha e o "sem dano mágico" fiquem registrados como contexto, não promessa de corte; SWE/orquestrador — precisa de um recorte verificável por `cargo test -p insurgencia-sim --test simulacao`, `cargo test -p insurgencia-cliente --lib`, `./verify.sh` (151 testes), roteiro 10/10 e CI `gates`+`rust`, sem promoção `dev→main` nem deploy por este agente.

## Goals

- Registrar a regra e todos os números como `[HIPÓTESE]`/`escolha de protótipo`: 25 Fôlego, 5 Materiais, 1 s de arme, 3 m de alcance, 1 m de raio, 2 s de imobilização, 60 s de duração, máximo 3 por jogador, 14 s de recarga e aliados imunes por `equipe`; conversão `PASSO_SEGUNDOS = 1/30` → 30 ticks de arme, 60 ticks de imobilização, 1800 ticks de duração, 420 ticks de recarga. Nenhum número é promovido a cânone e o GDD não é reaberto.
- Declarar constantes rotuladas com `///` (Fôlego/Materiais/ticks como inteiros, distâncias como `f64` com comparação por `hypot`/`total_cmp`, sem tolerância), mantendo `max_width = 100`, `#![forbid(unsafe_code)]` e `#![deny(clippy::pedantic)]`.
- Definir `Armadilha { id, dono, posicao, ticks_armar, ticks_restantes }`, acrescentar `Mundo.armadilhas: BTreeMap<u64, Armadilha>` e `Mundo.proxima_armadilha_id`, e acrescentar `Jogador.equipe: u8`, `Jogador.armadilha_recarga_ticks` e `Jogador.imobilizado_ticks`, zerados/inicializados de forma determinística na criação, sem `HashMap`, RNG, `Instant` ou estrutura paralela.
- Aceitar `armadilha` em `ACOES_PERMITIDAS` com nível **3** explícito em `nivel_minimo_para_acao` (slot E), com validação local ao sim na ordem **destino/alcance → Fôlego → Materiais → recarga**: destino na própria posição, distância `<= 3 m`, Fôlego `>= 25`, Materiais `>= 5`, recarga 0; `CodigoRecusa::ArmadilhaEmRecarga` para recarga ativa; cota de **uma armadilha por conjurador por tick**; `seq`/`request_id` avançam apenas em comando aceito e nenhuma recusa altera o estado.
- Inserir `avancar_armadilhas()` em `avancar_tick` **após** `avancar_escudos()` e **antes** de `fase_skills()`, decrementando uma vez por tick arme, recarga e imobilização em ordem determinística; ao completar 30 ticks de arme, a armadilha dispara no primeiro inimigo a `<= 1 m` com `equipe != dono`, é removida e imobiliza a vítima por 60 ticks; sem inimigo elegível, permanece ativa até 1800 ticks; limite de três ativas por conjurador.
- Fazer `mover` ser ignorado enquanto `jogador.imobilizado_ticks > 0`, sem alterar posição nem avançar `seq`/`request_id`, com a mesma disciplina no cliente (`mover_jogador` e `sincronizar_sim` não agenda/injeta movimento local enquanto imobilizado).
- Incluir `equipe`, `armadilha_recarga_ticks`, `imobilizado_ticks` e todas as armadilhas em `serializar_jogadores`/`hash_estado`, com segmentos em ordem crescente de `id`, preservando o hash SHA-256 canônico, o formato `<tick>\t<sha256>` do CLI e `ultima_recusa`.
- Cliente: tecla `E` despacha `armadilha` com `destino` na própria posição quando `funcao == Funcao::Artesao`; para as demais funções, mantém `analise_materialista` inalterado; HUD acrescenta `Armadilhas: N` preservando todas as linhas e teclas existentes e mantendo `texto_hud` com **menos de 100 linhas**.
- Escrever 5 testes nomeados em `crates/sim/tests/simulacao.rs` e preservar os **146** existentes (→ **151**), registrar rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012 e respeitar a constituição v1.2.2 (R1 `go`, R3 0 clarificações, R4 antes de R7 `1.0.0-local`, **R6 N/A**).

## Non-Goals

- Promover 25/5/3/1/1/2/60/3/14 ou 30/60/1800/420 ticks a cânone; reabrir o contrato da sessão, o GDD §10.3 ou os rótulos `[HIPÓTESE]`.
- Implementar dano mágico, canalização interrompível, Desafios de Ilha ("A Ocupação da Fábrica", "Delta do Comércio"), pontos de coleta neutros, rotas de Camponês inimigo, geometria de mapa ou linha de visão.
- Implementar Súditos inimigos com regra própria, NPCs caçadores de armadilha, desarme, armadilhas aliadas ofensivas ou imobilização em cadeia. Aliados e Súditos aliados são poupados pelo predicado de `equipe`; Súditos inimigos entram como inimigos pelo mesmo predicado.
- Introduzir magia, RNG, `Instant`/relógio externo, `async`, sockets, Lightyear, rede, protocolo, `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml` (ADR-004).
- Implementar as demais habilidades Q/W/R (Oficina de Campo, Sabotagem Estrutural etc.) ou novos números para elas — ficam para ciclos posteriores (BF-002b..v).
- Mudar o formato `<tick>\t<sha256>` do servidor headless, a whitelist derivada de `ACOES_PERMITIDAS`, a ordem das fases já existentes além da inserção de `avancar_armadilhas()`, o piso zero de dano ou a transição de PNJ para inativo.
- Captura visual real prometida onde não houver surface automatizável: cliente somente com `--minimizado`, linha `Armadilhas: N` renderizada é **[NV]**; a prova é estrutural por leitura de código.
- Executar gates, build, testes, formatadores, commit, push ou PR neste assessment; adotar R6 ou simular emissão de issues.

## Success Metrics

- Com Fôlego `>= 25`, Materiais `>= 5`, recarga 0 e destino na própria posição a `<= 3 m`, `armadilha` é aceita: debita **25 Fôlego** e **5 Materiais**, cria uma armadilha com 30 ticks de arme, inicia recarga de 420 ticks e avança `seq`/`request_id` apenas nesse caso (teste `armadilha_custa_25_folego_e_5_materiais`; baseline: ação inexistente — **[NV]** até verificação).
- Destino ausente, distância `> 3 m`, Fôlego `< 25`, Materiais `< 5`, recarga `> 0`, nível `< 3` ou cota já atingida no tick: comando recusado sem criar armadilha, sem débito e sem avanço de `seq`/`request_id`; `ArmadilhaEmRecarga` é o código da recarga (teste `armadilha_recusa_destino_alem_de_3_m`; baseline: **[NV]**).
- Após 30 ticks de arme, o primeiro inimigo a `<= 1 m` com `equipe != dono` fica imobilizado por exatamente **60 ticks**, a armadilha é removida e nenhum dano é aplicado; aliados e Súditos aliados com a mesma `equipe` atravessam sem disparar (teste `armadilha_imobiliza_inimigo_por_60_ticks_e_poupa_aliados`; baseline: **[NV]**).
- Sem inimigo elegível, a armadilha expira ao completar **1800 ticks**; um mesmo conjurador nunca mantém mais de **três** ativas, com a quarta tentativa recusada sem débito e sem avanço de monotônicos (teste `armadilha_expira_apos_1800_ticks_e_limite_de_tres`; baseline: **[NV]**).
- `hash_estado` difere quando `equipe`, recarga, imobilização, presença/posição de armadilha ou `id` mudam, e é idêntico entre execuções determinísticas do mesmo estado; armadilhas entram em ordem crescente de `id` (teste `hash_reflete_armadilha_e_imobilizacao`; baseline: campos ausentes — **[NV]**).
- Cliente: `E` despacha `armadilha` com destino na própria posição para `Funcao::Artesao` e preserva `analise_materialista` para as outras funções; `mover_jogador`/`sincronizar_sim` não movem nem agendam comando local enquanto `imobilizado_ticks > 0`; HUD mostra `Armadilhas: N` com `texto_hud` abaixo de 100 linhas (baseline: sem linha `Armadilhas:` e sem bloqueio — **[NV]**).
- 5 novos testes em `crates/sim/tests/simulacao.rs` passam e os **146** existentes permanecem válidos (→ **151**); estilo `forbid(unsafe_code)` / `deny(clippy::pedantic)` / sem `unwrap`/`expect`/`#[allow]` / `max_width = 100`; nenhuma mudança em contratos, protocolo, Cargo, stack, rede ou GDD (baseline: **[NV]**).

## Cost of Inaction

Sem o corte, o primeiro incremento nomeado do elenco de habilidades (BF-002b) permanece apenas no GDD: o Artesão continua sem controle de área jogável, `mover` segue incondicional no cliente e o hash não distingue um mundo com armadilha de um sem ela. Qualquer ciclo futuro que queira desafios de ilha com interceptação de rotas, imobilização de inimigos ou sincronização de estado de armadilha teria de reconstruir a mesma estrutura, a mesma fase de tick e a mesma disciplina de recusa — e o faria sem o lastro de §10.3:588-593 já fixado no contrato 057. Alternativamente, gastar o ciclo com BF-014 (250/3125) produziria apenas diagnóstico na HUD, sem mudar o jogo.

## Open Questions

- Nenhuma clarificação de design pendente no contrato: o corte está fechado em constantes `[HIPÓTESE]`, imunidade por `equipe`, tecla `E`, bloqueio local e HUD `Armadilhas: N` (R3 = 0 clarificações).
- **[NV]** Confirmar na especificação a ordem exata dos decrementos de `ticks_armar`, `ticks_restantes`, `armadilha_recarga_ticks` e `imobilizado_ticks` dentro de `avancar_armadilhas()` e o ponto de serialização no `hash_estado`, preservando ordem estável.
- **[NV]** Confirmar que a seleção do "primeiro inimigo" a `<= 1 m` usa ordem determinística de `peer_id` no `BTreeMap`, sem RNG e sem depender de ordem de iteração instável.
- **[NV]** Confirmar a inicialização de `Jogador.equipe` na criação e o valor de `PEER_ALVO_TREINO` no espelho do cliente, necessários para observar imunidade e imobilização sem segundo jogador real.
- **[NV]** Confirmar que a quarta armadilha é recusada sem débito e sem avanço de monotônicos, e que a cota de uma por conjurador por tick não conflita com a cota global de comandos do ciclo 056.
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** Os nomes definitivos de T001..T012 serão os de `specs/057-armadilha-improvisada/tasks.md`, ainda não criado no momento da leitura.
- **[NV]** Roteiro de verificação previsto: inspeção de código, testes da sim e do cliente e execução com `--minimizado`; não prometer captura visual real sem surface automatizável.

## Rastreabilidade (escopo do problema)

- FR-001 → constantes e rótulos `[HIPÓTESE]`/`escolha de protótipo` (25, 5, 3 m, 1 s, 1 m, 2 s, 60 s, 3, 14 s → 30/60/1800/420 ticks) derivadas de GDD §10.3:588-593; FR-002 → `Armadilha` e os campos em `Mundo`/`Jogador`, ambos em `hash_estado`; FR-003 → `"armadilha"` em `ACOES_PERMITIDAS`, nível 3 em `nivel_minimo_para_acao`, validação destino/alcance → Fôlego → Materiais → recarga, `CodigoRecusa::ArmadilhaEmRecarga` e cota por conjurador/tick; FR-004 → `avancar_armadilhas()` entre `avancar_escudos()` e `fase_skills()`, arme 30, disparo por `equipe != dono` a `<= 1 m`, remoção, imobilização 60, expiração 1800 e limite 3; FR-005 → `mover` ignorado sem mover posição nem monotônicos e hash cobrindo campos/armadilhas em ordem de `id`; FR-006 → cliente (tecla `E` por `funcao`, destino próprio, bloqueio local, HUD `Armadilhas: N` com `texto_hud` <100 linhas), 5 testes novos e 146 preservados, fronteira e governança.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis especificados em `specs/057-armadilha-improvisada/`, fora desta fronteira, com validação em `.specify/validations/armadilha-improvisada/` e release em `.specify/releases/armadilha-improvisada/`.
- R1 `go`, R3 0 clarificações, R4 antes de R7 `1.0.0-local`, **R6 N/A** (BF-007). Todo número de habilidade citado é `[HIPÓTESE]`/`escolha de protótipo`; nada aqui é cânone, especificação, plano ou implementação.
