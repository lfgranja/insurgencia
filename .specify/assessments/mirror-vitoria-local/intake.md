# Intake — Vitória local observável no espelho do cliente

**Data**: 2026-11-24 · **Slug**: `mirror-vitoria-local` · **Sessão**: `insurgencia - mirror vitoria local` · **Ciclo**: 058 · **Pendência**: BF-003a

## Ideia

Capturar, sem tocar em regra canônica, a fatia 80/20 do BF-003: tornar a vitória **localmente alcançável e observável no cliente standalone**. O espelho `Mundo` montado em `configurar` registra apenas jogadores e PNJs — as ilhas são desenhadas na cena Bevy, mas não entram no estado da sim, e não há súdito nem Nó no espelho. Consequência direta: a HUD mostra `Nós: 0/3` e o comando `propaganda_no` nunca encontra um Nó para converter. A condição de vitória está escrita, implementada e coberta por teste determinístico na sim; o que falta é um **recipiente de estado no cliente**. A correção é semear o espelho com uma semente estática prototípica — **4 ilhas (ids 1..=4), 1 súdito (id 1 em `(0.0, 236.0)`) e 3 nós** — por um helper público na sim chamado pelo cliente, preservando a fronteira e os valores `[HIPÓTESE]` do GDD:275.

**Motivação**: fechar BF-003a sem abrir o mapa completo de 12 ilhas (BF-003b, que permanece aberto). O cliente já tem câmera, terreno, ilhas renderizadas, avatar, alvo de treino e HUD; já tem `adicionar_ilha`/`adicionar_no` públicos na sim e já tem um teste de integração que prova que três Nós convertidos por `propaganda_no` zeram a Resistência Ideológica e produzem `ResultadoPartida::Vitoria`. Falta apenas ligar essas pontas no espelho local. É scaffolding de verificação, não conteúdo canônico nem mapa novo.

## Evidências de entrada (coladas; não reexecutadas)

- [`crates/cliente/src/main.rs:528-605`, lido] `configurar` cria câmera, luzes e terreno; desenha cada ilha de `ILHAS` com `criar_ilha` **na cena Bevy**; cria avatar e alvo de treino; monta o texto da HUD; depois constrói `let mut sim = Mundo::new()` e registra **somente** `sim.adicionar_jogador(PEER_ID)`, `sim.adicionar_jogador(PEER_ALVO_TREINO)` e `sim.adicionar_pnj(id, posicao)` para `PNJS_INICIAIS`. Não há `adicionar_ilha`, `adicionar_sudito` nem `adicionar_no` no espelho → o estado da sim não contém ilhas, súditos ou nós e a HUD reporta `Nós: 0/3`. (confidence: high)
- [`crates/sim/src/lib.rs:719-746`, lido] `Mundo::adicionar_ilha(&mut self, id: u64)` insere `Ilha { id, controlada: false, controle_ticks: 0 }` e `Mundo::adicionar_no(&mut self, id: u64, posicao: (f64, f64))` insere `NoControle { id, posicao, convertido: false, propaganda_ticks: 0, inativo_ticks: 0 }`. São APIs públicas já existentes: a semente não precisa de nova regra nem de nova estrutura de estado. (confidence: high)
- [`crates/sim/tests/simulacao.rs:2404-2407`, lido] `posicoes_nos()` fixa três Nós — `(1, (0.5, 0.0))`, `(2, (1.5, 0.0))`, `(3, (2.5, 0.0))` — usados pelo teste de vitória. (confidence: high)
- [`crates/sim/tests/simulacao.rs:2558-2593`, lido] `tres_nos_geram_vitoria()` registra três jogadores e três Nós, agenda `propaganda_no` para todos, avança `TICKS_PROPAGANDA_NO` ticks e confirma `convertido == true` nos três, `resistencia_ideologica == 0` e `resultado == Some(ResultadoPartida::Vitoria)`, estável por mais três ticks. É a rota exata que a semente do cliente deve tornar alcançável. (confidence: high)
- [`crates/sim/src/lib.rs:80-92`, lido] `RESISTENCIA_INICIAL = 100`, `CONTRIBUICAO_NO = 30`, `PISO_DANO_COM_NOS = 10`, `TOTAL_NOS = 3`, `TICKS_PROPAGANDA_NO = 600` (20 s × 30 Hz) — todos rotulados `[HIPÓTESE]`/protótipo, não cânone. (confidence: high)
- [`crates/sim/src/lib.rs:2158-2206`, lido] `hash_estado` serializa jogadores, estruturas, a lista de Nós (`id:posicao:convertido:propaganda:inatividade`), resistência, resultado e última recusa em SHA-256 — semear ilhas/súditos/nós altera a entrada do hash, tornando DoD-3 observável. (confidence: high)
- [`docs/GDD - Insurgência.md:275`, lido] "A Ilha Central possui 3 Nós de Controle — produção, comunicação e segurança — e 100 pontos de Resistência Ideológica. Cada Nó convertido reduz 30 pontos; o pulso de autogestão seguinte reduz os 10 restantes…" — precedido de `[HIPÓTESE]`. Nenhum número é promovido a cânone neste ciclo. (confidence: high)
- [contrato da sessão, imutável] Worktree `.specify/queue/worktrees/impl-058-mirror-vitoria/`, branch `feature/058-mirror-vitoria-local`, base `origin/dev = 1855e5c`; valores da semente (4 ilhas ids 1..=4, súdito id 1 em `(0.0, 236.0)`, 3 nós) e consumidor = jogador do cliente Bevy + testes de integração da sim. (confidence: high)

## Scorecard único do ciclo

| Opção | Impacto × Confiança × Fechamento × Esforço⁻¹ × Risco⁻¹ ÷ 3125 | Resultado | Consumidor |
|:--|:--|:--|:--|
| Helper público na sim + chamada no cliente (BF-003a, escolhida) | 5×3×3×2×3 = **270/3125** | **270/3125** | jogador do cliente Bevy + testes de integração da sim |
| Semeadura duplicada dentro do cliente | 5×3×2×2×2 = 120/3125 | 120/3125 | jogador do cliente Bevy |
| Apenas HUD/`hash_estado` (sem semear) | 1×3×1×5×5 = 75/3125 | 75/3125 | diagnóstico da HUD |

Critério normativo do contrato: **impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹**, escala 1–5, denominador **5⁵ = 3125**; o quarto e o quinto fatores são inversos de esforço e de risco (5 = menor esforço/risco, 1 = maior). Impacto **5** porque a vitória passa de inalcançável a jogável no cliente; confiança **3** porque os números vêm do GDD:275 como `[HIPÓTESE]` e o helper ainda não foi executado; fechamento **3** porque consumidor e teste de referência estão nomeados, mas os ids/posições da semente são escolha de protótipo; esforço⁻¹ **2** porque exige helper na sim, chamada no cliente e verificação de hash; risco⁻¹ **3** porque a fronteira fica intacta (sem regra canônica, CI, protocolo, Cargo, stack ou rede), embora o efeito final na HUD permaneça **[NV]** até verificação. Este é o **único** scorecard; `research.md`, `define.md`, `shape.md` e `decision.md` repetem **270/3125** e os mesmos fatores, sem pontuação paralela. O mesmo valor 270/3125 já constava para "Mapa/rotas e spawn por ilha (BF-003)" no scorecard do ciclo 057 — agora é retomado como recorte BF-003a.

## DoD do ciclo (referência única; repetido nos cinco artefatos)

- **DoD-1**: espelho `Mundo` do cliente semeado com **4 ilhas (ids 1..=4)**, **1 súdito (id 1 em `(0.0, 236.0)`)** e **3 nós**.
- **DoD-2**: vitória local alcançável convertendo os **3 nós via `propaganda_no`** — mesma rota de `crates/sim/tests/simulacao.rs:2558-2593`.
- **DoD-3**: `hash_estado` **muda** ao semear o espelho (baseline: sem ilhas/súditos/nós).
- **DoD-4**: **BF-003a fechado**; **BF-003b** (mapa completo de 12 ilhas) **permanece aberto**.

## Incógnitas de primeira vista

- **[NV]** A posição do súdito `(0.0, 236.0)` e os ids/posições dos 3 nós são escolha de protótipo fixada no contrato; não há ainda spec canônica que os ratifique como regra.
- **[NV]** `adicionar_sudito` precisa ser confirmada na API pública da sim no `/speckit.specify`; `adicionar_ilha`/`adicionar_no` já estão confirmadas em `lib.rs:719-746`.
- **[NV]** O efeito renderizado na HUD (`Nós: 3/3`, `Autogestão estabelecida`) depende de execução; este assessment é somente leitura.
- **[NV]** R4 (antes de R7 `1.0.0-local`) depende de gates não executados aqui.

## Rastreabilidade canônica (spec 058)

FR-001 → helper `semear_espelho_local` / `POSICOES_NOS_LOCAIS` na sim; FR-002 → chamada do helper no `configurar` do cliente; FR-003 → teste `espelho_local_seed_viabiliza_vitoria_e_reflete_hash`; FR-004 → evidência/validação/release. SC-001..SC-006 e T001..T012 são definidos na spec; os mesmos **DoD-1..DoD-4** e o score **270/3125** (5×3×3×2×3) aparecem em `research.md`, `define.md`, `shape.md` e `decision.md`.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/mirror-vitoria-local/`. Não executa gates, build, testes, formatadores ou linters; não faz `git add`/commit/push/PR/merge; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. **PR #76 pertence a outro owner ativo e não é tocado.** Gate de destino: `./verify.sh`; CI = jobs `gates`/`rust` do GitHub Actions; CLI de PR = `gh`; `dev → main` e deploy são exclusivos da SE. Tudo que depende de execução permanece **[NV]**. Constituição v1.2.2: R1 `go`, R4 antes de R7 `1.0.0-local`, **R6/taskstoissues N/A** — não executável sob a constituição; nunca simular issue, nunca `gh` improvisado.
