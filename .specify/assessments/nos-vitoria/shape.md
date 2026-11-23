# Shape — Nós de Controle e vitória por Resistência Ideológica

- **Slug**: `nos-vitoria`
- **Created**: 2026-11-23T12:50Z
- **Recommended option**: Opção A — Nós de Controle + Resistência + vitória (todos os números `[HIPÓTESE]`/protótipo)

## Options

### Option A — Nós de Controle + Resistência + vitória (recomendada)

- **Sketch**: em `crates/sim/src/lib.rs`, declarar `NoControle { id, convertido, propaganda_ticks, inativo_ticks }`, `Mundo.nos: BTreeMap<u64, NoControle>` com registro determinístico dos 3 Nós em posições fixas, `resistencia_ideologica: i64` (100) e `resultado: Option<ResultadoPartida>` — todos `[HIPÓTESE]`. Acrescentar `propaganda_no` e `atacar_no` a `ACOES_PERMITIDAS`, validando `destino` contra a posição fixa do Nó; `propaganda_no` avança `propaganda_ticks` em 1 por comando (dedup por `(id do Nó, tick)`) e, aos **600 ticks** (20 s a 30 Hz `[HIPÓTESE]`), converte uma vez e subtrai **30**; `atacar_no` aplica no máximo **−20** com piso **10** enquanto restar Nó não convertido e marca **600 ticks** de inatividade, sem converter. Com os 3 Nós convertidos, o pulso seguinte aplica **−10** e grava `resultado = Vitoria` ("Autogestão estabelecida"). `hash_estado` passa a serializar Nós, Resistência e resultado em ordem estável; a HUD mostra `Resistência` e `Nós`, com `R`/`F` reservados e mapeamento determinístico simples documentado como protótipo.
- **Appetite**: `small`/`medium` (poucos dias) — um struct, três campos em `Mundo`, duas ações, validação/dedup, um pulso no tick, duas projeções (hash e HUD) e duas teclas; sem contrato, protocolo, Cargo, stack, mapa, geometria, papéis, PNJs ou rede.
- **Trade-offs**: ganha a condição final de vitória verificável da sim, com regra de conversão em vez de demolição e padrões determinísticos já usados por Súditos/ilhas (ciclos 046-049); paga o preço de números todos hipotéticos e de uma abstração de protótipo (posições fixas, pulso local, mapeamento de teclas) que precisa ficar rotulada para não virar dogma. Impacto **5** (fecha a partida), confiança **2** (todos os números são `[HIPÓTESE]` e não há estado de Nó/Resistência/resultado hoje), facilidade **2** (poucos pontos de código, porém cruzando ações, validação, tick, hash, HUD e teclas) e risco invertido **3** (sim permanece determinística e fronteira intacta, mas exige disciplina para separar dano de conversão e não promover hipótese a cânone) → **60/625**.
- **Rabbit holes**: inventar mapa, geometria de Nó, raio de presença ou pré-requisito de rotas/Plano Mestre modelado; implementar papéis W/E/R, PVE/PNJs, tela de resultado ou variante duas Comunas; deixar `atacar_no` converter Nó ou zerar a Resistência; duplicar a subtração de 30 por conversão repetida; esquecer Nós/Resistência/resultado no `hash_estado`; confundir `"propaganda"` (Súditos) com `propaganda_no`; tocar em contratos, protocolo, Cargo, stack ou rede.

### Option B — Papéis W/E/R + níveis

- **Sketch**: introduzir Intelectual, Artesão, Guerreiro, Camponês e Agitador com níveis e habilidades W/E/R como novo contrato de ações.
- **Appetite**: `large` (semanas) — novo contrato de comandos, escalonamento e integração com Fôlego/Materiais.
- **Trade-offs**: impacto 4, confiança 2, facilidade 2, risco invertido 3 → **48/625**. Usa sistemas já implementados, mas não endereça a condição final de vitória e multiplicaria frentes de regra sem fechar a partida.
- **Rabbit holes**: curva de níveis, balanceamento de W/E/R, sincronia de papéis no cliente e novas ações em `ACOES_PERMITIDAS`.

### Option C — PVE/PNJs

- **Sketch**: modelar patrulhas, negociação com trabalhadores e desafios de Ilha (§8.5:420-433) como microssistemas com estado próprio.
- **Appetite**: `large` (semanas) — exige geometria de patrulha, calendário de 20 s, diálogo com lógica e recompensas; depende de mapa que o protótipo não tem.
- **Trade-offs**: impacto 4, confiança 2, facilidade 2, risco invertido 3 → **48/625**. Aumenta o escopo determinístico sem entregar o fim de partida; é frente paralela, não pré-requisito deste recorte.
- **Rabbit holes**: patrulhas e raio de 120 m, 2 acertos para virar propagandista, cooldown de 60 s por PNJ, recompensas de Materiais/Suprimentos.

### Option D — Rede / Lightyear primeira

- **Sketch**: postergar a regra local até transporte, predição e reconciliação estarem prontos, com resultado de partida sincronizado entre peers.
- **Appetite**: `large` (meses) — transforma modelagem de regra em infraestrutura de rede.
- **Trade-offs**: impacto 5, mas facilidade 1 e risco invertido 1 → **10/625**; ADR-004 bloqueia a troca até decisão da SE. Sincronizaria um resultado que ainda não existe localmente.
- **Rabbit holes**: lobby/matchmaking, concorrência de conversão entre Comunas, troca de engine sem novo ADR.

## Recommendation

Escolher **Opção A**. É a maior do scorecard (**60/625** contra 48/625 de B e C e 10/625 de D) e a única que fecha a condição de vitória com fronteira mínima: impacto **5** (a partida passa a ter fim verificável por conversão, não demolição), confiança **2** (a regra está escrita mas **todos** os números são `[HIPÓTESE]` e nada existe como estado hoje), facilidade **2** (ações, validação, tick, hash, HUD e teclas — sem contrato novo) e risco invertido **3** (determinismo preservado e fronteira intacta, com o custo de disciplina imposto pelos rótulos de protótipo). B e C abrem frentes paralelas que não entregam o fim da partida; D inverte a dependência regra local → sincronização e está bloqueada por ADR-004.

A recomendação **não** cria topologia de Nós, não promove número `[HIPÓTESE]` a cânone, não reatribui a ação `"propaganda"` dos Súditos e não implementa derrota por timeout/reconquista. A verificação é estrutural e determinística: leitura de código, simulação tick a tick, comparação de `hash_estado` e inspeção de HUD/teclas; saídas não produzidas neste assessment permanecem **[NV]**.

## Out of Scope (for the recommended option)

- Derrota por 20 min ou 3 reconquistas (`[HIPÓTESE]` §6.2:277, §8.5:416); relógio de partida e tela "Comuna dissolvida".
- Pré-requisito de 2 ilhas por rota + Plano Mestre, mapa, geometria, raio de Nó e topologia de rotas; papéis W/E/R, níveis, PVE/PNJs e variante duas Comunas.
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack ou rede; `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]` e `max_width = 100` são estilo existente, preservado.
- Mudanças em `AGENTS.md`, `STATE.md`, `.specify/queue/concurrency.jsonl` ou qualquer código-fonte por este assessment; **R6** (N/A por constituição v1.2.2), issues simuladas e `gh` improvisado; gates, build, testes e formatadores neste assessment.

## Assumptions to Validate

- `NoControle { id, convertido, propaganda_ticks, inativo_ticks }`, `Mundo.nos: BTreeMap<u64, NoControle>`, `resistencia_ideologica` e `resultado` são acrescentados sem alterar assinatura pública de `avancar_tick`, esquema de `Comando` ou contrato; a spec fixa ids/posições fixas dos 3 Nós como protótipo `[HIPÓTESE]`.
- As novas ações `propaganda_no`/`atacar_no` entram em `ACOES_PERMITIDAS` ao lado de `"propaganda"` (Súditos) — a validação exige `destino` igual à posição fixa do Nó e Nó existente; recusa sem efeito colateral, reaproveitando o padrão de validação de 047/048.
- 600 ticks = 20 s a 30 Hz para Propaganda e para inatividade, e −30/−10/−20/piso 10/100/3 Nós são `[HIPÓTESE]`: a spec rotula cada número e não os apresenta como cânone; §8.5:435 ("60 s acumulados", "exatamente 90") é o critério de consistência.
- Auto-avanço por **um** comando e dedup por `(id do Nó, tick)` impedem dupla contagem; conversão repetida em Nó já convertido não subtrai 30 outra vez; dano nunca converte nem altera a contribuição de 30.
- O pulso final observa os 3 Nós convertidos **no mesmo tick** e aplica −10 uma única vez, gravando `resultado = Vitoria`; a ordem das fases e o incremento final de `tick` são preservados.
- `hash_estado` inclui Nós, Resistência e resultado em ordem estável; a HUD lê esses campos sem inventar barra animada ou tela de resultado, e `R`/`F` usam mapeamento determinístico simples (documentado se a seleção for complexa).
- A cobertura determinística existente (Súditos, ilhas, Fôlego, coleta, barricadas) permanece válida; verificação é estrutural, com saídas de execução **[NV]** neste assessment.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; B, C e D são as alternativas de `research.md` e não recebem rastro de implementação neste ciclo. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica prevista em `specs/050-nos-vitoria/` (ainda a criar). Todos os números de jogo são **escolhas de protótipo `[HIPÓTESE]`**, nunca cânone. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa. Scorecard **único**: 60/625 (A), 48/625 (B), 48/625 (C), 10/625 (D); critério impacto × confiança × facilidade ÷ risco, denominador **625**.
