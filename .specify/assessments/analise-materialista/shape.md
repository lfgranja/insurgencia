# Shape — Análise Materialista (E do Intelectual)

- **Slug**: `analise-materialista`
- **Created**: 2026-11-23T16:55Z
- **Recommended option**: Opção A — Análise Materialista: marca determinística em PNJs ativos (única fatia do §10.2 no corte)

## Options

### Option A — Análise Materialista em PNJs ativos (recomendada)

- **Sketch**: em `crates/sim/src/lib.rs`, declarar as constantes `[HIPÓTESE]` `CUSTO_ANALISE_MATERIALISTA = 20`, `ALCANCE_ANALISE_MATERIALISTA_M = 12.0`, `TICKS_ANALISE_MATERIALISTA = 150` (5 s a 30 Hz), `RECARGA_ANALISE_MATERIALISTA_TICKS = 450` (15 s) e `BONUS_DANO_ANALISE_PERCENT = 25`; acrescentar `Pnj.marca_ticks: i64` (0 = sem marca) e `Jogador.analise_recarga_ticks: i64`; registrar `"analise_materialista"` em `ACOES_PERMITIDAS` e nível 3 em `nivel_minimo_para_acao`. A validação exige PNJ ativo em `destino`, distância `<= 12 m`, Fôlego `>= 20` e recarga 0; em comando aceito, debita 20, marca 150, inicia recarga 450 e avança `seq`/`request_id`, com cota de um por atacante por tick. Reaplicar **reinicia** para 150 ticks, nunca acumula. `aplicar_dano_a_pnj` aplica `dano + dano * 25 / 100` quando `marca_ticks > 0` (10 → 12), preservando piso zero e recusa de dano não positivo. Decrementos por tick em ordem determinística, sem `Instant`; ambos os campos entram em `hash_estado`. No cliente, tecla `E` (livre; `W` é movimento) agenda no PNJ ativo mais próximo dentro de 12 m, sem comando sem alvo; HUD acrescenta `Análise: N` preservando linhas/teclas existentes; cliente só com `--minimizado`. 5 testes novos em `crates/sim/tests/simulacao.rs`, 131 preservados.
- **Appetite**: `small` — cinco constantes, dois campos, uma ação e um nível na tabela existente, validação reaproveitada de `atacar_pnj`, um bônus local em `aplicar_dano_a_pnj`, dois decrementos por tick, hash refletivo, uma tecla, uma linha de HUD e cinco testes; sem contrato, protocolo, Cargo, stack, rede ou core reescrito.
- **Trade-offs**: ganha a primeira habilidade de papéis com verdade observável e auditável por `hash_estado`, conectando o slot E3 do ciclo 052 aos PNJs do ciclo 053, ao custo de um corte deliberadamente parcial do §10.2: ficam de fora jogadores, estruturas, visão por 10 s e execução de 0,3 s. Impacto **4** (habilidade real, não scaffolding), confiança **2** (todos os números `[HIPÓTESE]`/`escolha de protótipo` e parte do efeito do GDD fora do corte), facilidade **2** (novos campos, ação, validação, tick, bônus e cliente), risco invertido **3** (disciplina de tick, não-acumulação, hash e testes por verificar). O rótulo de protótipo é o preço aceito: promover qualquer número a cânone ou estender o corte a visão/estruturas viola a decisão.
- **Rabbit holes**: magia, RNG, `Instant`/relógio externo, canalização de 0,3 s, linha de visão, "visível através de terreno por 10 s", marcar jogadores/estruturas/Nós, promover números a cânone, reescrever `aplicar_dano_a_pnj` além do bônus ou `atacar_pnj`/`BTreeMap`, acumular marcas, duplicar débito ou avançar `seq`/`request_id` em comando recusado, mexer em `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`/stack/rede, implementar W/R, prometer captura visual com janela minimizada.

### Option B — Demais W/E/R (Escudo, Carga, Oficina, Fuga etc.)

- **Sketch**: entregar outras habilidades completas dos papéis materiais no mesmo ciclo, ampliando a frente de habilidades além do slot E3.
- **Appetite**: `large` (semanas) — cada habilidade exige API própria, validação, janela de tick, custo, alcance e recarga, além de números ainda não fechados no contrato.
- **Trade-offs**: impacto 4 e risco invertido 3 iguais aos da Opção A, mas confiança 2 e facilidade **1** → **24/625**. Não há corte fechado, conversão acordada ou recipiente determinístico pronto; o reaproveitamento imediato de `atacar_pnj`/`aplicar_dano_a_pnj` que a marca possui não existe para essas habilidades. É frente legítima, mas sem lastro no §10.2:554-559 e sem a costura com os ciclos 052/053.
- **Rabbit holes**: Escudo, Carga, Oficina, Fuga, novos números de Fôlego/alcance/recarga, novos campos por habilidade, canalização, IA e geometria.

### Option C — Rede / Lightyear primeira

- **Sketch**: postergar a verificação determinística local até transporte, predição e reconciliação cliente-servidor estarem prontos, com marca sincronizada entre peers.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de habilidade determinística para infraestrutura de rede.
- **Trade-offs**: impacto 5, mas facilidade 1 e risco invertido 1 → **10/625**; ADR-004 bloqueia a troca até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`). Sincronizaria uma marca cuja verificação local ainda não foi fechada — inverte a dependência determinismo → sincronização.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), predição de dano e de marca, resolução de concorrência de comandos, troca de engine sem novo ADR.

## Recommendation

Escolher **Opção A**. É a única que satisfaz as metas de `define.md` com fronteira mínima e lastro escrito: impacto **4** (slot E3 deixado em dívida pelo ciclo 052 ganha uma habilidade real sobre os PNJs do ciclo 053), confiança **2** (números `[HIPÓTESE]`/`escolha de protótipo`, parte do efeito do GDD fora do corte), facilidade **2** (dois campos, uma ação, validação reaproveitada, bônus local, tick, hash, cliente e testes) e risco invertido **3** (disciplina de tick, não-acumulação, hash e 5 testes novos ainda por verificar). B (24/625) não tem corte fechado nem reaproveitamento imediato; C (10/625) inverte a dependência determinismo → sincronização e está bloqueada por ADR-004. Scorecard único do ciclo: **48/625** contra 24/625 (B) e 10/625 (C).

A recomendação **não** promove número a cânone, não marca jogadores/estruturas, não implementa visão ou canalização e não reescreve o core além do contrato: o corte é protótipo rotulado, verificável por `hash_estado` e por cinco testes determinísticos. A verificação do cliente é estrutural por desenho — com `--minimizado`, a linha `Análise: N` renderizada fica **[NV]** e não é prometida como captura real.

## Out of Scope (for the recommended option)

- Promoção de 20/12 m/15 s/−25%/5 s ou de 150/450 ticks e do bônus 25% a cânone; reabertura do contrato 054 ou dos rótulos do GDD §10.2.
- Jogadores, estruturas, Nós ou qualquer entidade que não seja PNJ ativo; visão através de terreno por 10 s, execução de 0,3 s, linha de visão, mapa, geometria e canalização.
- Magia, RNG, `Instant`/relógio externo, `async`, sockets, Lightyear, rede, protocolo, `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml` (ADR-004).
- Demais habilidades W/E/R (Escudo, Carga, Oficina, Fuga etc.) e seus números — ciclos posteriores (24/625).
- Reescrita de `aplicar_dano_a_pnj`, `atacar_pnj` ou do `BTreeMap` de PNJs além do bônus local e do campo de marca; alteração do piso zero, da recusa de dano não positivo ou da transição para inativo.
- Mudanças em `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`; **R6** (N/A por constituição v1.2.2), issues simuladas e `gh` improvisado; gates, build, testes e formatadores neste assessment.
- Captura visual real onde não houver surface automatizável — marcada **[NV]**, não prometida; cliente somente com `--minimizado`, sem janela normal.

## Assumptions to Validate

- `Pnj.marca_ticks` e `Jogador.analise_recarga_ticks` são serializados por `hash_estado` em ordem estável, sem divergência entre execuções determinísticas; a ordem exata dos decrementos por tick é escolha de implementação, não de regra — validar na especificação.
- O predicado de PNJ ativo e o cálculo de distância jogador↔PNJ reaproveitam os de `atacar_pnj`, trocando apenas o alcance de 2 m por `ALCANCE_ANALISE_MATERIALISTA_M = 12.0`; a conversão 5 s → 150 ticks e 15 s → 450 ticks assume 30 Hz (`TICKS_POR_SEGUNDO`), conforme o contrato.
- `aplicar_dano_a_pnj` aceita o bônus `dano + dano * 25 / 100` sem alterar `saturating_sub(...).max(0)`: 10 → 12, piso zero preservado, dano não positivo recusado; o bônus incide somente com `marca_ticks > 0`.
- A cota de um `analise_materialista` por atacante por tick e o avanço de `seq`/`request_id` apenas em comando aceito reaproveitam a disciplina de comandos existente, sem duplicar débito nem marcar em comando recusado.
- Reaplicar a marca reinicia para 150 ticks — não soma nem estende — cobrindo "marcas de dois Intelectuais no mesmo alvo não se acumulam" (§10.2:559) sem estado adicional por atacante.
- Cliente: `E` está livre e `W` continua movimento; a escolha do PNJ ativo mais próximo dentro de 12 m é determinística inclusive em empate, sem RNG e sem ordem de iteração instável; nenhum comando é gerado sem alvo. A linha `Análise: N` conta PNJs marcados ativos e não remove linhas/teclas existentes.
- Os 5 testes novos em `crates/sim/tests/simulacao.rs` cobrem marca e bônus 10→12, não-acumulação, recarga 450, alcance 12 m e hash, com os 131 existentes preservados; estilo `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`. Tudo permanece **[NV]** até execução.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006 (detalhados em `define.md` e no handoff de `decision.md`); B e C são as alternativas de `research.md` e não recebem rastro de implementação neste ciclo. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica prevista em `specs/054-analise-materialista/` (ainda não criado), validação em `.specify/validations/analise-materialista/` e release em `.specify/releases/analise-materialista/`. Todo número de habilidade citado é `[HIPÓTESE]`/`escolha de protótipo`; nada aqui é cânone. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa.
