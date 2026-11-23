# Decision — Nós de Controle e vitória por Resistência Ideológica

- **Slug**: `nos-vitoria`
- **Decided**: 2026-11-23T12:50Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | GDD §6.2/§8.5 fixa a vitória por conversão dos Nós e Resistência 100 → 0, mas `Mundo` não tem `nos`, `resistencia_ideologica` nem `resultado` e `ACOES_PERMITIDAS` só conhece `"propaganda"` de Súditos: o fim de partida não existe como estado verificável. |
| Força da evidência | strong | GDD §6.2:271-285 e §8.5:416-437 lidos (inclusive os rótulos `[HIPÓTESE]` e os critérios de aceitação); `crates/sim/src/lib.rs:14-15,99-109,228-265,429-437,575-596,1009-1078` e `crates/cliente/src/main.rs:709-746,813-880` verificados; decisão em `.specify/memory/STATE.md` (`2026-11-23T12:50Z`); constituição v1.2.2. |
| Valor vs. inação | strong | 60/625 contra 48/625 (papéis W/E/R), 48/625 (PVE/PNJs) e 10/625 (rede); inação mantém a vitória apenas no papel e obriga qualquer frente futura a reconstruir Nós, Resistência e pulso final. |
| Viabilidade / appetite | strong | `small`/`medium`: um struct, três campos em `Mundo`, duas ações com validação/dedup, um pulso no tick, projeções em hash/HUD e duas teclas; padrões de acumulador, fila e hash reaproveitados dos ciclos 046-049; sem contrato, protocolo, Cargo, stack, mapa ou rede. |
| Alinhamento estratégico | strong | Não reabre números de ciclos anteriores e não promove os números `[HIPÓTESE]` a cânone; dano não converte Nó e não pode zerar a Resistência; derrota por timeout/reconquista fica fora de escopo; respeita R1 `go`, R4 antes de R7 `1.0.0-local` e `analyze` READ-ONLY. |
| Postura de risco | strong | Risco invertido 3: sim permanece determinística e contratos/protocolo/Cargo/stack/rede intocados; dedup por `(id do Nó, tick)`, conversão única e teto/piso de dano evitam dupla contagem e vitória por destruição; Nós/Resistência/resultado entram no `hash_estado`; R6 registrada como N/A, sem issue simulada nem `gh` improvisado. |

## 80/20 (scorecard único do ciclo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Nós de Controle + Resistência + vitória (escolhida) | 5×2×2×3 = 60/625 | **60/625** |
| Papéis W/E/R + níveis | 4×2×2×3 = 48/625 | 48/625 |
| PVE/PNJs | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério do contrato — impacto × confiança × facilidade ÷ risco (o quarto fator é o risco invertido/segurança: 5 = risco mínimo, 1 = risco máximo), escala 1–5, denominador **625** (5⁴). Este é o único scorecard do assessment; os mesmos valores aparecem em `intake.md`, `research.md` e `shape.md`, sem pontuação paralela. Fonte autoritativa: `.specify/memory/STATE.md`, entrada `2026-11-23T12:50Z`.

Nós/vitória vence porque é a única opção que fecha a partida com regra de **conversão, não demolição**, e com fronteira determinística preservada. Impacto **5** registra a condição final de vitória observável; confiança **2** registra que, embora a regra esteja escrita, **todos** os números (3 Nós, 100 Resistência, −30/Nó, pulso −10, teto −20 com piso 10, 20 s/600 ticks de Propaganda e 20 s/600 ticks de inatividade) são `[HIPÓTESE]` e nenhum estado correspondente existe hoje; facilidade **2** reflete ações, validação, tick, hash, HUD e teclas, ainda que sem contrato novo; risco invertido **3** reflete o determinismo intacto e o custo de disciplina imposto pelos rótulos de protótipo e pela separação entre dano e conversão. Papéis (48/625) e PVE/PNJs (48/625) são frentes paralelas que não entregam o fim da partida; rede (10/625) inverte a dependência regra local → sincronização e está bloqueada por ADR-004.

## Veredito e justificativa

**Veredito: go** — Opção A (Nós de Controle + Resistência + vitória, todos os números `[HIPÓTESE]`/protótipo), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. **Modelo e rótulo**: `NoControle { id, convertido, propaganda_ticks, inativo_ticks }`, `Mundo.nos: BTreeMap<u64, NoControle>`, `resistencia_ideologica: i64` e `resultado: Option<ResultadoPartida>` declarados em `crates/sim/src/lib.rs`. Os 3 Nós, as posições fixas, a Resistência inicial de 100, −30/Nó, o pulso final −10, o teto de dano −20 com piso 10 e os 20 s (600 ticks a 30 Hz) de Propaganda e inatividade são **`[HIPÓTESE]`/escolhas de protótipo**, documentados como tal em spec e comentários — nunca promovidos a cânone.
2. **Ações e validação**: `propaganda_no` e `atacar_no` entram em `ACOES_PERMITIDAS` ao lado de `"propaganda"` (Súditos, ciclo 046), com `destino` = posição fixa do Nó; validação de Nó existente e posição, recusa sem efeito colateral, ids `seq`/`request_id` monotônicos e nenhuma mudança no esquema de `Comando`, contrato ou protocolo.
3. **Propaganda e dedup**: `propaganda_ticks` avança **1 por comando** até **600 ticks contínuos** (20 s `[HIPÓTESE]`), com auto-avanço por um comando e dedup por `(id do Nó, tick)`; a conversão ocorre **uma vez**, marca `convertido = true` e subtrai **exatamente 30** da Resistência. Comandos repetidos em Nó convertido não subtraem novamente.
4. **Dano não converte**: `atacar_no` não altera `convertido` nem a contribuição de 30 do Nó; aplica no máximo **−20** com piso **10** enquanto restar Nó não convertido e marca `inativo_ticks = 600` (20 s `[HIPÓTESE]`). A Resistência não pode chegar a 0 por dano — somente pelo pulso final.
5. **Pulso final**: quando os 3 Nós estiverem convertidos no mesmo tick, o pulso seguinte aplica **−10** uma única vez, grava `resultado = Some(Vitoria)` com a mensagem "Autogestão estabelecida" e mantém a Ilha Central e a Sede intactas. Derrota por timeout/reconquista fica fora de escopo.
6. **Determinismo**: `nos` (id, `convertido`, `propaganda_ticks`, `inativo_ticks`), `resistencia_ideologica` e `resultado` entram em `hash_estado` em ordem estável; estados diferentes produzem hashes diferentes e o replay determinístico existente permanece válido. Ordem das fases de `avancar_tick` e incremento final de `tick` preservados.
7. **HUD e teclas**: a HUD exibe `Resistência` e `Nós` convertidos com rótulo de protótipo; `R`/`F` agenda `propaganda_no`/`atacar_no` com mapeamento determinístico simples ao Nó (ou as teclas reservadas com o mapeamento documentado, se a interação for complexa) — sem picking, raio ou tela de resultado prometidos.
8. **Fronteira e estilo**: nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack ou rede; padrões de acumulador/dedup/hash dos ciclos 046-049 reaproveitados; `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.
9. **Constituição v1.2.2**: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Hipótese promovida a cânone**: mitigado por decisão — rótulo explícito `[HIPÓTESE]`/protótipo para cada número em spec, HUD e comentários. Se a spec apresentar 3 Nós/100/−30/−10/−20/piso 10/600 ticks como regra canônica, é violação deste assessment.
- **Vitória por destruição**: `atacar_no` converter Nó, ignorar o teto de 20 ou o piso 10, ou zerar a Resistência contraria §6.2:282 e §8.5:437. A Resistência só chega a 0 pelo pulso final após as 3 conversões.
- **Dupla contagem e continuidade**: dedup por `(id do Nó, tick)`, auto-avanço por um comando e conversão idempotente por Nó evitam −30 repetido; os 600 ticks de Propaganda devem ser contínuos por comando válido, sem acúmulo espúrio.
- **Hash e determinismo**: Nós, Resistência e resultado precisam entrar em `hash_estado` em ordem estável; omissão geraria colisão entre estados diferentes. Permanece **[NV]** até execução.
- **Fronteira de stack**: nenhuma mudança em contratos/protocolo/Cargo/stack/rede; ADR-004 continua bloqueando rede até decisão da SE; a ação `"propaganda"` de Súditos permanece distinta de `propaganda_no`.
- **HUD/teclas**: mapeamento simples de `R`/`F` é documentado como protótipo se a seleção por proximidade exigir geometria; não prometer picking, barra animada ou tela de resultado.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação geral**: gates, build, testes, formatadores, R4 e R7 (`1.0.0-local`) permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: a condição de vitória do GDD (§6.2/§8.5 — converter 3 Nós por Propaganda, Resistência 100 → 0, pulso final, dano que não converte) não existe como estado verificável: `Mundo` não tem Nós/Resistência/resultado, o contrato de ações não tem `propaganda_no`/`atacar_no` e a HUD não mostra `Resistência`/`Nós`.
- **Chosen approach**: Opção A — `NoControle`/`Mundo.nos`, `resistencia_ideologica`, `resultado`; novas ações com `destino` fixo e validação; 600 ticks de Propaganda com dedup e −30 único; dano com teto −20/piso 10 e 600 ticks de inatividade sem conversão; pulso −10 + `Vitoria`; hash e HUD atualizados. **Todos os números `[HIPÓTESE]`/protótipo.**
- **In scope**: modelo/API, ações e validação, regras de Propaganda/dano/pulso, hash, HUD, teclas `R`/`F`, rastros e governança. **Out of scope**: derrota por timeout/reconquista, mapa/geometria/rotas/Plano Mestre modelado, papéis W/E/R, PVE/PNJs, tela de resultado, variante duas Comunas, contratos/protocolo/Cargo/stack/rede e **R6 (N/A)**.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: ids/posições fixas dos 3 Nós e forma de `ResultadoPartida`; fase do pulso final em `avancar_tick`; dedup de `atacar_no` por comando ou tick e aplicação do piso 10; mapeamento determinístico de `R`/`F`; rótulo de protótipo na HUD — validar na spec e no roteiro de verificação determinística.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | `NoControle { id, convertido, propaganda_ticks, inativo_ticks }` e `Mundo.nos: BTreeMap<u64, NoControle>` com 3 Nós em posições fixas `[HIPÓTESE]` | contrato da sessão; `intake.md` |
| FR-002 | `resistencia_ideologica: i64` (100) e `resultado: Option<ResultadoPartida>` `[HIPÓTESE]` | GDD §6.2:273; `define.md` |
| FR-003 | `propaganda_no`/`atacar_no` em `ACOES_PERMITIDAS`, `destino` na posição fixa do Nó, validação sem mudar esquema de comando | `crates/sim/src/lib.rs:99-109`; `shape.md` · Opção A |
| FR-004 | 600 ticks de Propaganda, auto-avanço por um comando, dedup por `(id do Nó, tick)` e −30 uma vez na conversão | GDD §8.5:435-437; `define.md` |
| FR-005 | `atacar_no` com teto −20/piso 10 e 600 ticks de inatividade, sem converter Nó nem alterar sua contribuição de 30 | GDD §6.2:282, §8.5:437; `shape.md` · Opção A |
| FR-006 | pulso final −10 + `resultado = Vitoria`, `hash_estado`, HUD `Resistência`/`Nós`, teclas `R`/`F`, fronteira e constituição | contrato; `crates/sim/src/lib.rs:1009-1078`; `.specify/memory/constitution.md` |
| SC-001 | Propaganda contínua converte aos 600 ticks; 3 Nós = 60 s acumulados `[HIPÓTESE]` | `define.md` · Success Metrics 1 |
| SC-002 | cada conversão subtrai exatamente 30 uma única vez; dedup sem dupla contagem | `define.md` · Success Metrics 2 |
| SC-003 | dano não converte: teto −20, piso 10, 600 ticks de inatividade, contribuição preservada | `define.md` · Success Metrics 3 |
| SC-004 | pulso seguinte aos 3 Nós aplica −10, Resistência 0 e `Vitoria` com ilha intacta | `define.md` · Success Metrics 4 |
| SC-005 | `hash_estado` reflete Nós/Resistência/resultado; replay determinístico preservado | `define.md` · Success Metrics 5 |
| SC-006 | HUD com `Resistência`/`Nós`, teclas `R`/`F` com mapeamento simples, fronteira e estilo Rust intactos | `define.md` · Success Metrics 6 |
| T001 | struct `NoControle` e constantes `[HIPÓTESE]` | `shape.md` · Opção A |
| T002 | campos `Mundo.nos` / `resistencia_ideologica` / `resultado` | `shape.md` · Opção A |
| T003 | registro determinístico dos 3 Nós em posições fixas | `shape.md` · Assumptions |
| T004 | `propaganda_no` e `atacar_no` em `ACOES_PERMITIDAS` | `shape.md` · Opção A |
| T005 | validação de `destino`/Nó existente com recusa sem efeito | `shape.md` · Assumptions |
| T006 | `propaganda_ticks` com marco de 600, auto-avanço e dedup por `(id, tick)` | `shape.md` · Opção A |
| T007 | conversão única com −30 exato | `shape.md` · Opção A |
| T008 | dano com teto −20/piso 10 e 600 ticks de inatividade, sem conversão | `shape.md` · Opção A |
| T009 | pulso final −10 e `resultado = Vitoria` | `shape.md` · Opção A |
| T010 | Nós, Resistência e resultado em `hash_estado` em ordem estável | `shape.md` · Assumptions |
| T011 | HUD `Resistência`/`Nós` e teclas `R`/`F` com mapeamento documentado | `shape.md` · Opção A |
| T012 | rastros e governança: R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**; não-execução | `.specify/memory/constitution.md`; `intake.md` |

Os nomes definitivos de T001..T012 serão os de `specs/050-nos-vitoria/tasks.md` (diretório ainda a criar); a coluna "Eco" apenas rastreia a cobertura deste assessment. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa. Todos os números de jogo são escolhas de protótipo `[HIPÓTESE]`; nada aqui é especificação, plano ou implementação.

Veredito: go
