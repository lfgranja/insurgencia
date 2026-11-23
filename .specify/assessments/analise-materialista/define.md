# Define — Análise Materialista (E do Intelectual)

- **Slug**: `analise-materialista`
- **Created**: 2026-11-23T16:55Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §10.2:554-559; `STATE.md:432-442`; `crates/sim/src/lib.rs`; ciclos 052/053; Constituição v1.2.2)

## Problem Statement

O GDD §10.2:554-559 descreve a habilidade E do Intelectual — 20 Fôlego, 12 m, −25% de resistência por 5 s, não-acumulação — e o ciclo 052 abriu o slot E3 (`nivel_minimo_para_acao` já mapeia `propaganda`/`propaganda_no` para nível 3), enquanto o ciclo 053 criou PNJs defensores com PV e ativo. No entanto, o Núcleo não tem **nenhum** dos três elementos que os unem: `Pnj` não carrega marca, `Jogador` não carrega recarga de análise, `ACOES_PERMITIDAS` não tem `analise_materialista` e `aplicar_dano_a_pnj` não aplica bônus. A regra existe apenas como linha de tabela `[HIPÓTESE]` no GDD e como dívida explícita do ciclo 052 ("novas habilidades W/E/R ficam para ciclo posterior"). O recorte resolve isso com a **marca determinística em PNJs ativos**: um campo por PNJ, um contador de recarga por jogador, uma ação validada, um bônus aritmético no dano e a reflexão de ambos os campos em `hash_estado`. Não é magia, não é visão, não é canalização e não é nova regra canônica: é a primeira habilidade de papéis com verdade observável no Núcleo, com todos os números rotulados `[HIPÓTESE]`/`escolha de protótipo`.

## Affected Users & Stakeholders

- **Users**: jogadores no papel Intelectual — precisam que a tecla `E` tenha efeito auditável num PNJ ativo dentro de 12 m, com custo 20 Fôlego, janela de 5 s e recarga de 15 s legíveis; quem inspeciona a HUD — precisa da linha `Análise: N` (PNJs marcados ativos) sem perder nenhuma linha ou tecla existente, em especial `W` de movimento.
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de dois campos, cinco constantes, uma ação, validação reaproveitada, bônus local em `aplicar_dano_a_pnj`, decrementos por tick e hash refletivo, sem reescrever `atacar_pnj` nem o `BTreeMap` de PNJs; mantenedores de testes — precisam de 5 novos testes em `crates/sim/tests/simulacao.rs` com os 131 existentes preservados; designer — precisa que 20/12 m/15 s/−25%/5 s continuem `[HIPÓTESE]` e que o corte não seja lido como implementação completa do §10.2 (visão, execução 0,3 s, jogadores e estruturas ficam fora); SWE/orquestrador — precisa de uma fatia verificável por inspeção de código e execução do binário com `--minimizado`, sem janela normal.

## Goals

- Declarar, `[HIPÓTESE]`/`escolha de protótipo`, `CUSTO_ANALISE_MATERIALISTA = 20`, `ALCANCE_ANALISE_MATERIALISTA_M = 12.0`, `TICKS_ANALISE_MATERIALISTA = 150` (5 s a 30 Hz), `RECARGA_ANALISE_MATERIALISTA_TICKS = 450` (15 s) e `BONUS_DANO_ANALISE_PERCENT = 25`, derivados de §10.2:554-559 sem promover nenhum valor a cânone.
- Acrescentar `Pnj.marca_ticks: i64` (0 = sem marca) e `Jogador.analise_recarga_ticks: i64`, ambos serializados por `hash_estado` em ordem estável.
- Registrar `"analise_materialista"` em `ACOES_PERMITIDAS` e nível 3 em `nivel_minimo_para_acao` (slot E3), reaproveitando o mapa Q1/W2/E3/R6 do ciclo 052.
- Validar a ação com `destino` na posição de um PNJ **ativo**, distância jogador↔PNJ `<= 12 m`, Fôlego `>= 20` e recarga 0; recusar sem efeito colateral nos demais casos.
- Debitar 20 Fôlego, marcar o PNJ por 150 ticks e iniciar recarga de 450 **somente em comando aceito**, avançando `seq`/`request_id` apenas nesse caso; cota de um por atacante por tick.
- Garantir não-acumulação: reaplicar a marca **reinicia** para 150 ticks, nunca soma nem estende; duas marcas de Intelectuais diferentes no mesmo PNJ não se acumulam (§10.2:559).
- Aplicar `+25%` em `aplicar_dano_a_pnj` quando `marca_ticks > 0`: `dano + dano * BONUS_DANO_ANALISE_PERCENT / 100` (10 → 12), preservando o piso zero e a rejeição de dano não positivo.
- Decrementar `marca_ticks` e `analise_recarga_ticks` por tick em ordem determinística, sem `Instant`/relógio externo, refletindo o resultado em `hash_estado`.
- Cliente: tecla `E` (livre; `W` é movimento) agenda `analise_materialista` no PNJ ativo mais próximo dentro de 12 m, **sem comando sem alvo**; HUD acrescenta `Análise: N` e preserva todas as linhas/teclas existentes; nenhuma janela normal — cliente só com `--minimizado`.
- Cobrir com 5 novos testes em `crates/sim/tests/simulacao.rs` (marca e bônus 10→12, não-acumulação, recarga 450, alcance 12 m, hash), preservando os 131 existentes; estilo `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.
- Registrar rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012 e constituição v1.2.2 (R1 `go`, R3 0 clarificações, R4 antes de R7 `1.0.0-local`, **R6 N/A**).

## Non-Goals

- Promover 20 Fôlego, 12 m, 15 s, −25%, 5 s, 150/450 ticks ou o bônus 25% a cânone; reabrir o contrato da sessão ou os rótulos do GDD §10.2.
- Marcar jogadores, estruturas, Nós ou qualquer entidade que não seja PNJ ativo; implementar "estruturas marcadas visíveis através de terreno por 10 s", execução de 0,3 s, linha de visão, geometria de mapa ou canalização.
- Implementar magia, RNG, `Instant`/relógio externo, `async`, sockets, Lightyear, rede, protocolo, `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml` (ADR-004).
- Implementar as demais habilidades W/E/R (Escudo, Carga, Oficina, Fuga etc.) ou novos números de Fôlego/alcance/recarga para elas — ficam para ciclos posteriores (24/625 no scorecard).
- Reescrever `aplicar_dano_a_pnj`, `atacar_pnj` ou o `BTreeMap` de PNJs além do bônus local e do novo campo; alterar o piso zero, a recusa de dano não positivo ou a transição para inativo.
- Captura visual real prometida onde não houver surface automatizável: cliente somente com `--minimizado`, linha `Análise: N` renderizada é **[NV]**; a prova é estrutural por leitura de código.
- Executar gates, build, testes, formatadores, commit, push ou PR neste assessment; adotar R6 ou simular emissão de issues.

## Success Metrics

- Com Fôlego `>= 20`, recarga 0 e PNJ ativo a `<= 12 m`, `analise_materialista` é aceito: debita **20** Fôlego, marca o PNJ com **150 ticks** e inicia recarga de **450 ticks**; `seq`/`request_id` avançam apenas nesse caso (baseline: ação inexistente — **[NV]** até verificação).
- Enquanto `marca_ticks > 0`, `aplicar_dano_a_pnj` aplica **+25%**: dano **10 → 12**, com piso zero preservado; dano não positivo continua sem efeito (baseline: 10 — **[NV]**).
- Reaplicar a marca no mesmo PNJ **reinicia** para 150 ticks, sem acumulação nem extensão; duas marcas de Intelectuais diferentes não se somam (baseline: sem marca — **[NV]**).
- Com recarga `> 0` (até 450 ticks após o uso), Fôlego `< 20`, sem PNJ ativo no `destino` ou distância `> 12 m`, o comando é recusado sem dano, sem débito e sem avanço de `seq`/`request_id` (baseline: **[NV]**).
- Apenas **um** `analise_materialista` por atacante por tick é aceito; decrementos de `marca_ticks` e `analise_recarga_ticks` ocorrem em ordem determinística, sem `Instant`, e ambos os campos aparecem em `hash_estado` (baseline: campos ausentes — **[NV]**).
- Cliente: `E` agenda no PNJ ativo mais próximo dentro de 12 m, nenhum comando sem alvo; HUD mostra `Análise: N`, preservando linhas/teclas existentes e `W` de movimento (baseline: sem linha `Análise:` — **[NV]**).
- 5 novos testes em `crates/sim/tests/simulacao.rs` passam e os **131** existentes permanecem válidos; estilo `forbid(unsafe_code)` / `deny(clippy::pedantic)` / sem `unwrap`/`expect`/`#[allow]` / `max_width = 100`; nenhuma mudança em sim além do contrato, contratos, protocolo, Cargo, stack ou rede (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, o slot E3 aberto no ciclo 052 e os PNJs entregues no ciclo 053 permanecem desconectados: a habilidade do Intelectual continua descrita apenas no GDD, sem verdade observável no Núcleo, sem custo debitado, sem janela de 5 s, sem recarga e sem bônus de dano auditável. Qualquer ciclo futuro que queira habilidades W/E/R, desafios de ilha com sabotagem ou sincronização de rede teria de reconstruir o mesmo par de campos, a mesma validação e a mesma disciplina de tick — e o faria sem o lastro do §10.2:554-559 já fixado no contrato 054. O custo é perder a única fatia de habilidade com corte fechado, efeito determinístico por `hash_estado` e fronteira que não toca contrato, protocolo, stack ou rede.

## Open Questions

- Nenhuma clarificação de design pendente no contrato: o corte está fechado em PNJs ativos, números `[HIPÓTESE]` e não-acumulação (R3 = 0 clarificações).
- **[NV]** Confirmar na especificação a ordem exata dos decrementos por tick (`marca_ticks` e `analise_recarga_ticks`) e o ponto do ciclo em que `hash_estado` os serializa, preservando ordem estável.
- **[NV]** Confirmar que o predicado de "PNJ ativo" e o cálculo de distância reaproveitam integralmente o de `atacar_pnj`, trocando apenas o alcance de 2 m por 12 m.
- **[NV]** Confirmar a escolha do PNJ "mais próximo" no cliente em caso de empate determinístico de distância, sem RNG e sem depender de ordem de iteração instável.
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** Os nomes definitivos de T001..T012 serão os de `specs/054-analise-materialista/tasks.md`, ainda não criado no momento da leitura.
- **[NV]** Roteiro de verificação previsto: inspeção de código, execução do binário com `--minimizado` e capturas de terminal/HUD quando aplicáveis; não prometer captura visual real sem surface automatizável.

## Rastreabilidade (escopo do problema)

- FR-001 → constantes `[HIPÓTESE]`/`escolha de protótipo` (20, 12 m, 150 ticks, 450 ticks, 25%) derivadas de GDD §10.2:554-559; FR-002 → `Pnj.marca_ticks` e `Jogador.analise_recarga_ticks`, ambos em `hash_estado`; FR-003 → `"analise_materialista"` em `ACOES_PERMITIDAS`, nível 3 em `nivel_minimo_para_acao` e validação (PNJ ativo no `destino`, `<= 12 m`, Fôlego `>= 20`, recarga 0); FR-004 → aplicação atômica (débito 20, marca 150, recarga 450), `seq`/`request_id` só em aceito, um por atacante/tick e não-acumulação; FR-005 → bônus `dano + dano * 25 / 100` em `aplicar_dano_a_pnj` (10 → 12) com piso zero; FR-006 → cliente (tecla `E`, PNJ ativo mais próximo, sem comando sem alvo, HUD `Análise: N`, preservação de linhas/teclas), 5 testes novos + 131 preservados, fronteira e governança.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis especificados em `specs/054-analise-materialista/`, fora desta fronteira, com validação em `.specify/validations/analise-materialista/` e release em `.specify/releases/analise-materialista/`.
- R1 `go`, R3 0 clarificações, R4 antes de R7 `1.0.0-local`, **R6 N/A** por constituição v1.2.2. Todo número de habilidade citado é `[HIPÓTESE]`/`escolha de protótipo`; nada aqui é cânone, especificação, plano ou implementação.
