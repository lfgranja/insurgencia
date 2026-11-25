# Research — Vitória local observável no espelho do cliente

**Data**: 2026-11-24 · Evidências coladas do contrato da sessão, do código verificado no checkout, do GDD:275 e do scorecard do ciclo 057; sem reexecução de gates, build ou testes.

## Evidência primária

- [source: `crates/cliente/src/main.rs:528-605`, lido] `configurar` desenha as ilhas com `criar_ilha` na cena Bevy, cria avatar/alvo de treino e monta a HUD, mas o `Mundo` recebe apenas `adicionar_jogador(PEER_ID)`, `adicionar_jogador(PEER_ALVO_TREINO)` e `adicionar_pnj` para `PNJS_INICIAIS`. **Não há nenhuma chamada a `adicionar_ilha`, `adicionar_sudito` ou `adicionar_no`** no espelho. Logo, o estado serializado da sim não contém ilhas, súditos ou nós e a HUD informa `Nós: 0/3`. (confidence: high)
- [source: `crates/sim/src/lib.rs:719-746`, lido] `adicionar_ilha(id)` e `adicionar_no(id, posicao)` são métodos públicos já presentes em `Mundo`, com estado inicial determinístico (`controlada: false`, `controle_ticks: 0`; `convertido: false`, `propaganda_ticks: 0`, `inativo_ticks: 0`). Um helper de semente pode chamá-los sem criar estrutura nova nem regra nova. (confidence: high)
- [source: `crates/sim/tests/simulacao.rs:2404-2407`, lido] `posicoes_nos()` fixa `(1, (0.5, 0.0))`, `(2, (1.5, 0.0))`, `(3, (2.5, 0.0))` — três nós, coerentes com `TOTAL_NOS = 3` e com a tríade `[HIPÓTESE]` do GDD:275. (confidence: high)
- [source: `crates/sim/tests/simulacao.rs:2558-2593`, lido] `tres_nos_geram_vitoria()` prova a rota completa: registrar jogadores, `adicionar_no`, posicionar, agendar `propaganda_no`, avançar `TICKS_PROPAGANDA_NO` ticks → os três `convertido`, `resistencia_ideologica == 0`, `resultado == Some(ResultadoPartida::Vitoria)`, estável por três ticks seguintes. A vitória local é alcançável **desde que o espelho tenha nós**. (confidence: high)
- [source: `crates/sim/src/lib.rs:80-92`, lido] `RESISTENCIA_INICIAL = 100`, `CONTRIBUICAO_NO = 30`, `PISO_DANO_COM_NOS = 10`, `TOTAL_NOS = 3`, `TICKS_PROPAGANDA_NO = 600` — `[HIPÓTESE]`/protótipo, não cânone. Nenhum é reaberto. (confidence: high)
- [source: `crates/sim/src/lib.rs:2158-2206`, lido] `hash_estado` inclui a lista de Nós, a resistência e o resultado na entrada SHA-256; portanto semear o espelho **altera o hash** mesmo antes de qualquer conversão — base observável para DoD-3. (confidence: high)
- [source: `docs/GDD - Insurgência.md:275`, lido] Vitória = autogestão na Ilha Central por 3 Nós, 100 de Resistência, −30 por Nó, pulso final de −10; todo o parágrafo é `[HIPÓTESE]`. O recorte **usa** esses números como protótipo e não os promove a regra canônica. (confidence: high)
- [source: contrato da sessão, imutável] Semente fixa: 4 ilhas ids 1..=4, 1 súdito id 1 em `(0.0, 236.0)`, 3 nós; DoD-1..DoD-4; consumidor = jogador do cliente Bevy + testes de integração da sim; score 270/3125 (5×3×3×2×3); worktree `impl-058-mirror-vitoria`, branch `feature/058-mirror-vitoria-local`, base `origin/dev = 1855e5c`. (confidence: high)
- [source: `.specify/assessments/armadilha-improvisada/{intake,shape,decision}.md`, lidos] O ciclo 057 já pontuou "Mapa/rotas e spawn por ilha (BF-003)" em **270/3125** (5×3×3×2×3) e o deixou sem corte fechado; o ciclo 058 retoma exatamente esse valor como BF-003a, com corte agora fechado. (confidence: high)
- [source: `.specify/memory/constitution.md`, lido via contrato] Constituição v1.2.2: R1 `go`, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY, `converge` append-only; **R6/taskstoissues N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado. (confidence: high)

## Interpretação

O achado é de **estado ausente**, não de regra ausente. `Mundo` já sabe adicionar ilhas e nós (lib.rs:719-746), já converte nós por `propaganda_no`, já produz `Vitoria` e já reflete isso no hash (tests:2558-2593; lib.rs:2158); o cliente já desenha ilhas e já tem HUD. O que falta é o elo: popular o espelho local com uma semente determinística e chamar esse helper de `configurar`. A contribuição 80/20 é, portanto, de **plumbing de estado entre sim e cliente**, com três opções possíveis — helper público na sim (escolhida, **270/3125**), duplicação da semeadura no cliente (120/3125) e HUD/`hash_estado` sem semear (75/3125). Todas são avaliadas no mesmo scorecard único; nenhuma altera GDD canônico, CI, protocolo, Cargo, stack ou rede.

## Alternativas rejeitadas (mesmo scorecard único, 270/3125 para a escolhida)

1. **Semeadura duplicada dentro do cliente** — rejeitada (120/3125: 5×3×2×2×2). impacto 5 (a vitória também vira jogável), confiança 3, mas fechamento **2** porque a verdade do estado passa a viver em dois lugares (o cliente reimplementa o que a sim já expõe), esforço⁻¹ **2** e risco⁻¹ **2** por divergência silenciosa entre testes da sim e espelho do cliente. Rejeitada porque quebra a vantagem central da opção escolhida: uma única implementação da semente, testável na sim e reaproveitada pelo cliente.
2. **Apenas HUD/`hash_estado` (sem semear)** — rejeitada (75/3125: 1×3×1×5×5). É a variante diagnóstica: exibir hash ou contadores sem criar ilhas/súditos/nós. Baixíssimo esforço e risco, mas **não torna a vitória alcançável** — viola diretamente DoD-2 e deixa `Nós: 0/3`. (O ciclo 057 já reescalonou "HUD republica `hash_estado`" para 250/3125 por não mudar o jogo; aqui a variante é ainda mais estreita.)
3. **Abrir o mapa completo de 12 ilhas (BF-003b)** — rejeitada sem pontuação paralela: é justamente a pendência que este recorte **mantém aberta** por DoD-4. Topologia, spawning e critérios de 12 ilhas não estão fechados e extrapolam o contrato 058.
4. **Promover os números do GDD:275 a cânone / mudar regra de vitória** — rejeitada sem pontuação: `[HIPÓTESE]` é rótulo fixado no GDD e o contrato veta mudança de regra canônica.
5. **Rede / Lightyear primeiro** — rejeitada sem pontuação: ADR-004 bloqueia a troca até decisão da SE; a verificação local sequer foi fechada.

## Riscos registrados (não reabertos)

- **Semente tratada como conteúdo canônico**: mitigado por rótulo explicitamente prototípico — ids e posições são escolha de protótipo do contrato, não regra do GDD; nenhum número é promovido.
- **Divergência sim ↔ cliente**: mitigado pela opção A — o helper público vive na sim, os testes de integração cobrem a rota de vitória e o cliente só o chama; a duplicação (opção B) é precisamente o risco rejeitado.
- **Determinismo/hash**: a semente é estática (sem aleatoriedade), logo `hash_estado` muda de forma previsível — DoD-3. A cobertura determinística existente não é reescrita.
- **Verificação**: execução de `./verify.sh`, jobs `gates`/`rust`, build, testes e captura de HUD ficam **[NV]** — nenhum foi executado por este agente.
- **Dívida R6**: N/A por constituição v1.2.2; nenhuma issue simulada, nenhum `gh` improvisado. PR #76 pertence a outro owner e não é tocado.
- **Fronteira de promoção**: `dev → main` e deploy são exclusivos da SE; este assessment apenas recomenda `go`.

## Rastreabilidade

Fatos colados como observados, sem reexecução. Nenhum comando de gate, build, git remoto, GitHub ou emissão de issue foi executado. **Score: 270/3125** (5×3×3×2×3) é o único do ciclo e reaparece em `intake.md`, `define.md`, `shape.md` e `decision.md`. **DoD-1..DoD-4** são idênticos nos cinco artefatos: 4 ilhas (ids 1..=4), 1 súdito (id 1 em `(0.0, 236.0)`) e 3 nós; vitória por `propaganda_no` nos 3 nós; `hash_estado` muda; BF-003a fechado e BF-003b aberto. Rastros canônicos da spec: **FR-001** = helper `semear_espelho_local` / `POSICOES_NOS_LOCAIS` na sim; **FR-002** = chamada no `configurar` do cliente; **FR-003** = teste `espelho_local_seed_viabiliza_vitoria_e_reflete_hash`; **FR-004** = evidência/validação/release. SC-001..SC-006 e T001..T012 completam o rastro em `specs/058-mirror-vitoria-local/` (ainda em construção). **R6 = N/A** por constituição v1.2.2.
