# Shape — Vitória local observável no espelho do cliente

- **Slug**: `mirror-vitoria-local`
- **Created**: 2026-11-24
- **Score**: **270/3125** (`5×3×3×2×3`)
- **Recommended option**: Opção A — helper público na sim (`semear_espelho_local`) + chamada no cliente (BF-003a)

## DoD do ciclo (referência única; idêntico nos cinco artefatos)

- **DoD-1**: espelho `Mundo` do cliente semeado com **4 ilhas (ids 1..=4)**, **1 súdito (id 1 em `(0.0, 236.0)`)** e **3 nós**.
- **DoD-2**: vitória local alcançável convertendo os **3 nós via `propaganda_no`** — mesma rota de `crates/sim/tests/simulacao.rs:2558-2593`.
- **DoD-3**: `hash_estado` **muda** ao semear o espelho (baseline: sem ilhas/súditos/nós).
- **DoD-4**: **BF-003a fechado**; **BF-003b** (mapa completo de 12 ilhas) **permanece aberto**.

## Options

### Option A — Helper público na sim + chamada no cliente (recomendada)

- **Sketch**: expor em `crates/sim/src/lib.rs` um helper público determinístico (`semear_espelho_local(&mut Mundo)`) que insira **4 ilhas (ids 1..=4)** via `adicionar_ilha`, **1 súdito (id 1 em `(0.0, 236.0)`)** via a API existente de súditos e **3 nós** via `adicionar_no` (`lib.rs:719-746`), com posições alinhadas a `posicoes_nos()` (`simulacao.rs:2404-2407`). Em `crates/cliente/src/main.rs:528-605`, `configurar` chama o helper depois de registrar jogadores e PNJs no `Mundo`. A rota de vitória já existe: `propaganda_no` nos 3 nós por `TICKS_PROPAGANDA_NO` ticks → `convertido`, `resistencia_ideologica == 0`, `resultado == Vitoria` (`simulacao.rs:2558-2593`), refletida em `hash_estado` (`lib.rs:2158-2206`) e na HUD. Consumidor: jogador do cliente Bevy + testes de integração da sim.
- **Appetite**: `small` — um helper na sim com três tipos de inserção, uma chamada no cliente e verificação de hash/HUD; sem contrato, protocolo, Cargo, CI, stack ou rede novos.
- **Trade-offs**: impacto **5** (a vitória passa de inalcançável a jogável no cliente), confiança **3** (os números vêm do GDD:275 como `[HIPÓTESE]`; o helper ainda não foi executado), fechamento **3** (consumidor e teste de referência nomeados, mas ids/posições são escolha de protótipo), esforço⁻¹ **2** (sim + cliente + verificação de hash), risco invertido **3** (fronteira intacta; efeito na HUD **[NV]** até verificação). Resultado: **270/3125**.
- **Rabbit holes**: transformar a semente em mapa de 12 ilhas (BF-003b, fora por DoD-4); promover os números do GDD:275 a cânone; duplicar a lógica de inserção no cliente; reescrever `propaganda_no`, `hash_estado` ou a condição de vitória; mexer em contrato, protocolo, Cargo, CI, stack ou rede.

### Option B — Semeadura duplicada dentro do cliente

- **Sketch**: em `configurar`, chamar diretamente `adicionar_ilha`, `adicionar_sudito` e `adicionar_no` com literais locais, sem helper na sim.
- **Appetite**: `small`, mas com duas verdades de estado a manter.
- **Trade-offs**: impacto 5, confiança 3, fechamento **2**, esforço⁻¹ **2**, risco⁻¹ **2** → **120/3125**. A sim já expõe as APIs (`lib.rs:719-746`) e os testes cobrem a rota na sim; duplicar os literais no cliente cria divergência silenciosa entre o que o teste valida e o que o espelho contém.
- **Rabbit holes**: sincronizar manualmente duas listas de ids/posições; ajustar o cliente sempre que a sim mudar; perder a garantia de que `hash_estado` e o teste `tres_nos_geram_vitoria` descrevem o mesmo estado.

### Option C — Apenas HUD/`hash_estado` (sem semear)

- **Sketch**: exibir o hash ou os contadores na HUD sem criar ilhas/súditos/nós no espelho.
- **Appetite**: `xs`, diagnóstico puro.
- **Trade-offs**: impacto **1**, confiança 3, fechamento **1**, esforço⁻¹ 5, risco⁻¹ 5 → **75/3125**. Não torna a vitória alcançável: `Nós: 0/3` permanece e DoD-2 é violado. (O ciclo 057 já reescalonou "HUD republica `hash_estado`" para 250/3125 por não mudar o jogo; aqui a variante é ainda mais estreita.)
- **Rabbit holes**: confundir diagnóstico com entrega; substituir o teste de vitória por comparação de strings de hash.

## Recommendation

Escolher **Opção A**. É a única que satisfaz os quatro DoDs com fronteira mínima: impacto **5** (vitória de inalcançável a jogável no cliente), confiança **3** (valores `[HIPÓTESE]` em GDD:275 e helper por executar), fechamento **3** (consumidor = jogador do cliente Bevy + testes de integração da sim; teste de referência `simulacao.rs:2558-2593`), esforço⁻¹ **2** (helper na sim, chamada no cliente, verificação de hash) e risco invertido **3** (nenhuma mudança em regra canônica, CI, protocolo, Cargo, stack ou rede). **Score: 270/3125** — o mesmo valor já atribuído a BF-003 no scorecard do ciclo 057, agora com corte fechado. B (120/3125) duplica verdade de estado; C (75/3125) não entrega vitória.

A recomendação **não** propõe promover número a cânone, abrir BF-003b (mapa de 12 ilhas, DoD-4), mudar regra de vitória, CI, protocolo ou stack. A verificação é estrutural por desenho — `./verify.sh` (jobs `gates`/`rust`) é o gate de destino, mas nenhum gate é executado neste assessment; efeito final na HUD permanece **[NV]**.

## Por que não há mudança de regra, CI ou protocolo

- **Regra**: os números (3 nós, 100 de Resistência, −30 por nó, pulso de −10, 600 ticks) já existem no GDD:275 como `[HIPÓTESE]` e na sim (`lib.rs:80-92`); a semente apenas materializa esse estado no espelho. Nenhuma constante canônica é criada ou promovida.
- **CI**: o gate de destino é `./verify.sh` e os jobs `gates`/`rust` do GitHub Actions; a mudança é código Rust comum, coberto pela suíte existente. Não há novo job, workflow, threshold ou ferramenta — e este agente não executa gates.
- **Protocolo/contrato/stack**: a semente é estado local determinístico do `Mundo`; não há mensagem nova, `PROTOCOL_VERSION`, alteração em `crates/contratos`, `Cargo.toml`, Lightyear (ADR-004) ou rede. `dev → main` e deploy são exclusivos da SE; PR #76 pertence a outro owner e não é tocado; R6/taskstoissues é N/A (constituição v1.2.2), sem issue simulada nem `gh` improvisado.

## Out of Scope (for the recommended option)

- Mapa de 12 ilhas, rotas, spawning por ilha e reconquista — **BF-003b aberto** (DoD-4).
- Promoção da `[HIPÓTESE]` do GDD:275 a cânone; reescrita de `propaganda_no`, `hash_estado` ou da condição de vitória.
- Duplicação da semeadura no cliente (opção B) e HUD diagnóstica sem semear (opção C).
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.github/workflows`, `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`; gates, build, testes e formatadores neste assessment; commit/push/PR (`gh`); `dev → main` ou deploy; PR #76; **R6** e issues simuladas.
- Captura visual prometida onde não houver surface automatizável — marcada **[NV]**.

## Assumptions to Validate

- A API pública de súditos em `Mundo` aceita id 1 e posição `(0.0, 236.0)` sem depender de spawn remoto — `adicionar_ilha`/`adicionar_no` já confirmadas em `lib.rs:719-746`.
- As posições dos 3 nós na semente podem seguir `posicoes_nos()` (`simulacao.rs:2404-2407`); são escolha de protótipo, não cânone.
- `(0.0, 236.0)` está dentro do terreno gerado em `configurar` e o súdito entra na serialização usada por `hash_estado`.
- O helper é estático/determinístico: semear duas vezes com o mesmo estado produz o mesmo `hash_estado`; a mudança esperada (DoD-3) é em relação ao espelho **sem** semente.
- A HUD deixa de mostrar `Nós: 0/3` e passa a ler os 3 nós do espelho; efeito renderizado **[NV]** até execução.
- Cobertura determinística existente (`tres_nos_geram_vitoria`) permanece válida e é a referência de DoD-2; estilo Rust `forbid(unsafe_code)`/`deny(clippy::pedantic)`/sem `unwrap`/`expect`/`#[allow]`/`max_width = 100` preservado (**[NV]**).

## Rastreabilidade

Opção A ↔ FR-001..FR-004 no mapeamento canônico da spec (**FR-001** = helper `semear_espelho_local` / `POSICOES_NOS_LOCAIS` na sim; **FR-002** = chamada no `configurar` do cliente; **FR-003** = teste `espelho_local_seed_viabiliza_vitoria_e_reflete_hash`; **FR-004** = evidência/validação/release) e SC-001..SC-006 (detalhados em `define.md` e no handoff de `decision.md`); B e C são as alternativas de `research.md` e não recebem rastro de implementação neste ciclo. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica prevista em `specs/058-mirror-vitoria-local/` (em construção). Todo número de vitória citado é `[HIPÓTESE]` (GDD:275); nada aqui é cânone. **R6/taskstoissues é N/A** por constituição v1.2.2 e não recebe tarefa. **Score: 270/3125** (5×3×3×2×3), repetido em `intake.md`, `research.md`, `define.md` e `decision.md`.
