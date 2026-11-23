# Decision — Derrota por timeout e reconquista

- **Slug**: `derrota`
- **Decided**: 2026-11-23T14:25Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | GDD §6.2:277 e §9.4:489,501 descrevem a derrota, mas `ResultadoPartida` só tem `Vitoria`, `Mundo` não acumula reconquistas, `avancar_tick` não tem fase de derrota e a HUD só diz "Autogestão estabelecida"/"em curso": o desfecho perdedor não existe como estado verificável. |
| Força da evidência | strong | GDD §6.2:277 e §9.4:489-501 lidos (com rótulos `[HIPÓTESE]` e critérios de aceitação); `crates/sim/src/lib.rs:14-15,272-276,300-335,517-527,676-694,1182-1197,1337-1362` e `crates/cliente/src/main.rs:26,933` verificados; decisão em `.specify/memory/STATE.md` (`2026-11-23T14:25Z`); assessment/release 050; constituição v1.2.2. |
| Valor vs. inação | strong | 192/625 contra 48/625 (papéis W/E/R), 48/625 (PVE/PNJs) e 10/625 (rede); inação mantém a partida sem fim no lado perdedor e a mensagem "Comuna dissolvida" do §6.2 como letra morta, obrigando ciclos futuros a reconstruir contador, fase, hash e precedência. |
| Viabilidade / appetite | strong | `small`: um `enum`, um `BTreeSet`, duas constantes, uma fase, um `insert` no sinal já emitido, um braço de hash e um braço de HUD; sem mapa, geometria, IA, contrato, protocolo, Cargo, stack, rede ou nova tecla. |
| Alinhamento estratégico | strong | Não reabre a vitória do ciclo 050 nem promove números a cânone: 20 min = 36000 ticks a 30 Hz e 3 reconquistas permanecem `[HIPÓTESE]`/protótipo; derrota por reconquista não é bloqueada por Resistência parcial (§6.2); respeita R1 `go`, R4 antes de R7 `1.0.0-local` e `analyze` READ-ONLY. |
| Postura de risco | strong | Risco invertido 4: sim permanece determinística com `tick` como relógio, `BTreeSet` dedup por id, vitória prevalecendo pela ordem das fases e `resultado.is_some()`, e hash atualizado; contratos/protocolo/Cargo/stack/rede intocados; R6 registrada como N/A, sem issue simulada nem `gh` improvisado. |

## 80/20 (scorecard único do ciclo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Derrota por timeout/reconquista (escolhida) | 4×3×4×4 = 192/625 | **192/625** |
| Papéis W/E/R + níveis | 4×2×2×3 = 48/625 | 48/625 |
| PVE/PNJs | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério do contrato — impacto × confiança × facilidade ÷ risco, escala 1–5, denominador **625** (5⁴); o quarto fator é o risco invertido (segurança): 5 = risco mínimo, 1 = risco máximo. Este é o único scorecard do assessment; os mesmos valores aparecem em `intake.md`, `research.md` e `shape.md`, sem pontuação paralela. Fonte autoritativa: `.specify/memory/STATE.md`, entrada `2026-11-23T14:25Z`.

A derrota vence porque é a única opção que **fecha o ciclo de partida** aberto pela vitória do ciclo 050 com fronteira mínima e risco controlado: o sinal `ilha_perdida:{id}` já é emitido por `fase_controle_ilhas` e o `tick` já é o relógio determinístico a 30 Hz. Impacto **4** registra a completude do desfecho sem inflar o valor de uma regra que reusa infraestrutura existente; confiança **3** registra que a regra e o sinal existem, mas que **todos** os números (20 min = 36000 ticks, 3 reconquistas, os 20 s de §9.4) são `[HIPÓTESE]` e o efeito na HUD permanece **[NV]**; facilidade **4** reflete um enum, um conjunto, duas constantes, uma fase, um `insert`, um braço de hash e um braço de HUD; risco invertido **4** reflete o determinismo preservado, o dedup por id e a precedência obtida pela ordem das fases — não 5 porque mexer na ordem de `avancar_tick` e no `hash_estado` toca o replay, e porque os rótulos de protótipo precisam ser mantidos. Papéis (48/625) e PVE (48/625) são frentes de design maiores que não endereçam o desfecho; rede (10/625) inverte a dependência verificabilidade → sincronização e está bloqueada por ADR-004.

## Veredito e justificativa

**Veredito: go** — Opção A (derrota por timeout/reconquista, com todos os números `[HIPÓTESE]`/protótipo), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. **Modelo e rótulo**: `ResultadoPartida::Derrota` (ou as variantes `DerrotaPorTempo`/`DerrotaPorReconquista`) declarada ao lado de `Vitoria`, `Mundo.ilhas_reconquistadas: BTreeSet<u64>` inicializado vazio em `new()`, e constantes `TICKS_LIMITE_PARTIDA = 36000` e `MAX_RECONQUISTAS = 3` em `crates/sim/src/lib.rs`. Os números 20 min (36000 ticks a 30 Hz, derivados de `PASSO_SEGUNDOS = 1/30`) e 3 reconquistas são **`[HIPÓTESE]`/escolhas de protótipo**, documentados como tal em spec, HUD e comentários — nunca promovidos a cânone.
2. **Registro da reconquista**: no ramo de `fase_controle_ilhas` que já emite `sinais.push_back(format!("ilha_perdida:{id}"))`, acrescentar `ilhas_reconquistadas.insert(id)` — dedup por id, contagem por ilha e não por emissão de sinal. A fila `sinais` é FIFO e não histórico; nenhuma reconquista é contada sem o sinal correspondente. As regras de perda de ilha de §9.4 (10 s de ausência, `[HIPÓTESE]` de 20 s de reconversão) não são reabertas.
3. **Fase de derrota**: inserir `fase_derrota` imediatamente após `fase_vitoria` e antes de `fase_finish` em `avancar_tick`: se `resultado.is_some()`, nada; se `tick >= TICKS_LIMITE_PARTIDA`, gravar Derrota; se `ilhas_reconquistadas.len() >= MAX_RECONQUISTAS`, gravar Derrota. Nenhum relógio de parede (`Instant`/`SystemTime`), nenhuma reordenação das fases existentes e nenhuma alteração do `tick += 1`.
4. **Precedência**: vitória prevalece se ocorrer no mesmo tick — obtida pela ordem `fase_vitoria → fase_derrota` e pela guarda `resultado.is_some()`, reaproveitando o padrão já usado em `fase_vitoria`. Derrota por 3 reconquistas não é bloqueada por Resistência parcial, conforme §6.2 ("mesmo que a Resistência da Ilha Central já esteja abaixo de 50").
5. **Determinismo e hash**: incluir `ilhas_reconquistadas` e a nova variante em `hash_estado` em ordem estável, atualizando o `match` hoje exaustivo (`Some(ResultadoPartida::Vitoria) => "vitoria", None => "nenhum"`). Estados que diferem apenas no desfecho ou no conjunto produzem hashes diferentes; o replay determinístico existente permanece válido.
6. **HUD**: `atualizar_texto` passa a exibir "Comuna dissolvida" no campo `Resultado` quando houver derrota, mantendo "Autogestão estabelecida" para vitória e "em curso" para `None`. **Sem novas teclas, eventos, telas, animação ou som**; o campo `Resultado` já existe no cliente.
7. **Fronteira e estilo**: nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack, rede ou mapa; nenhuma simulação da Corporação como agente, geometria de reconquista ou reabertura da vitória 050. Padrões de enum/fase/hash dos ciclos 046-050 reaproveitados; `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.
8. **Verificação**: roteiro determinístico por inspeção de código e execução da sim — timeout em 36000 ticks, terceiro id distinto em `ilhas_reconquistadas`, vitória e derrota no mesmo tick, dedup de id repetido, hashes distintos e texto da HUD. Efeito renderizado sem surface automatizável é **[NV]**; não prometer screenshot real.
9. **Constituição v1.2.2**: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Hipótese promovida a cânone**: mitigado por decisão — rótulo explícito `[HIPÓTESE]`/protótipo para 36000 ticks, 3 reconquistas e os 20 s de §9.4 em spec, HUD e comentários. Se a spec apresentar esses números como regra canônica, é violação deste assessment.
- **Dupla contagem**: `fase_controle_ilhas` pode emitir `ilha_perdida:{id}` em ticks diferentes para a mesma ilha; o `BTreeSet<u64>` dedup por id é a mitigação. Usar a fila `sinais` como contador geraria contagem errada — a fila é FIFO e não histórico.
- **Precedência trocada**: inverter a ordem das fases ou introduzir prioridade ad-hoc faria a derrota sobrescrever a vitória no mesmo tick. A guarda `resultado.is_some()` e a ordem `fase_vitoria → fase_derrota` são a prova estrutural; permanece **[NV]** até execução.
- **Hash e determinismo**: `match` exaustivo em `hash_estado` precisa ser estendido na mesma fatia; omitir o conjunto ou a variante geraria colisão entre estados diferentes e quebraria o replay. Permanece **[NV]** até execução.
- **Tempo determinístico**: `tick` (`i64`) é o único relógio; `>= 36000` evita dependência de igualdade exata. Introduzir relógio de parede, aleatoriedade ou `async` quebraria o determinismo e a fronteira de stack.
- **HUD**: o campo `Resultado` já existe; a mudança é um braço de texto. Não prometer tela de fim de partida, animação, som ou nova tecla; efeito renderizado é **[NV]**.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação geral**: gates, build, testes, formatadores, R4 e R7 (`1.0.0-local`) permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: a derrota descrita no GDD (§6.2:277 — relógio de 20 min ou 3 reconquistas; §9.4:489,501 — `ilha_perdida`) não existe como estado verificável: `ResultadoPartida` só tem `Vitoria`, `Mundo` não acumula reconquistas, `avancar_tick` não tem fase de derrota, o hash só codifica vitória e a HUD não sabe dizer "Comuna dissolvida".
- **Chosen approach**: Opção A — variante de `ResultadoPartida`, `Mundo.ilhas_reconquistadas: BTreeSet<u64>`, `TICKS_LIMITE_PARTIDA = 36000`, `MAX_RECONQUISTAS = 3`, registro do id no ponto do `ilha_perdida`, `fase_derrota` após `fase_vitoria` com guarda `resultado.is_some()`, hash atualizado e um braço na HUD. **Todos os números `[HIPÓTESE]`/protótipo.**
- **In scope**: modelo/constantes, registro de reconquista, fase e precedência, hash, HUD, rastros e governança. **Out of scope**: reabertura da vitória 050, mapa/geometria/IA da Corporação, papéis W/E/R, PVE/PNJs, tela de resultado, novas teclas, contratos/protocolo/Cargo/stack/rede e **R6 (N/A)**.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: variante única `Derrota` ou `DerrotaPorTempo`/`DerrotaPorReconquista`; semântica de dedup quando a mesma ilha é perdida, reconquistada e perdida de novo; precedência no mesmo tick; codificação em `hash_estado` e ausência de divergência de replay — validar na spec e no roteiro determinístico.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | `ResultadoPartida::Derrota` (ou `DerrotaPorTempo`/`DerrotaPorReconquista`) e constantes `TICKS_LIMITE_PARTIDA = 36000` / `MAX_RECONQUISTAS = 3`, rotuladas `[HIPÓTESE]` | contrato da sessão; `intake.md`; GDD §6.2:277 |
| FR-002 | `Mundo.ilhas_reconquistadas: BTreeSet<u64>` inicializado vazio e `insert(id)` no ponto do `ilha_perdida:<id>` em `fase_controle_ilhas`, dedup por id | `crates/sim/src/lib.rs:676-694`; `shape.md` · Opção A |
| FR-003 | `fase_derrota` após `fase_vitoria`: guarda `resultado.is_some()`, `tick >= 36000`, `len() >= 3` | `crates/sim/src/lib.rs:517-527,1182-1197`; `define.md` |
| FR-004 | precedência — vitória prevalece no mesmo tick; derrota por reconquista não bloqueada por Resistência parcial; regras de §9.4 não reabertas | GDD §6.2 (critérios), §9.4:489,501; `shape.md` · Recommendation |
| FR-005 | `hash_estado` com conjunto e variante em ordem estável (match exaustivo atualizado) e HUD `Resultado` com "Comuna dissolvida", sem novas teclas | `crates/sim/src/lib.rs:1337-1362`; `crates/cliente/src/main.rs:933`; `shape.md` · Opção A |
| FR-006 | fronteira (contratos/protocolo/Cargo/stack/rede/mapa intactos), rastros e constituição v1.2.2 | contrato; `.specify/memory/constitution.md` |
| SC-001 | `tick >= 36000` com `resultado == None` grava Derrota; antes disso, nada | `define.md` · Success Metrics 1 |
| SC-002 | cada `ilha_perdida:<id>` registra o id uma vez; 3 ids distintos geram Derrota mesmo com Resistência < 50 | `define.md` · Success Metrics 2 |
| SC-003 | vitória e derrota no mesmo tick deixam `resultado = Vitoria` (ordem das fases + guarda) | `define.md` · Success Metrics 3 |
| SC-004 | id repetido não aumenta o contador; nenhuma reconquista sem sinal | `define.md` · Success Metrics 4 |
| SC-005 | `hash_estado` distingue estados que diferem só no desfecho/conjunto; replay válido | `define.md` · Success Metrics 5 |
| SC-006 | HUD "Comuna dissolvida" / "Autogestão estabelecida" / "em curso"; fronteira e estilo Rust preservados | `define.md` · Success Metrics 6 |
| T001 | variante `ResultadoPartida::Derrota` (ou `DerrotaPorTempo`/`DerrotaPorReconquista`) documentada como `[HIPÓTESE]` | `shape.md` · Opção A |
| T002 | `Mundo.ilhas_reconquistadas: BTreeSet<u64>` inicializado vazio em `new()` | `shape.md` · Opção A |
| T003 | constantes `TICKS_LIMITE_PARTIDA = 36000` e `MAX_RECONQUISTAS = 3` rotuladas `[HIPÓTESE]` | `shape.md` · Opção A |
| T004 | `insert(id)` em `fase_controle_ilhas` no ramo do `ilha_perdida` | `shape.md` · Opção A |
| T005 | `fase_derrota` inserida após `fase_vitoria` em `avancar_tick` | `shape.md` · Opção A |
| T006 | guarda `resultado.is_some()` e precedência da vitória no mesmo tick | `shape.md` · Assumptions |
| T007 | comparações `tick >= TICKS_LIMITE_PARTIDA` e `len() >= MAX_RECONQUISTAS` | `shape.md` · Opção A |
| T008 | `hash_estado` com conjunto e variante em ordem estável; `match` exaustivo atualizado | `shape.md` · Assumptions |
| T009 | HUD `Resultado` com "Comuna dissolvida", sem novas teclas | `shape.md` · Opção A |
| T010 | rótulos `[HIPÓTESE]`/protótipo em spec, HUD e comentários; vitória 050 não reaberta | `define.md` · Non-Goals |
| T011 | roteiro determinístico (timeout, 3º id, mesmo tick, dedup, hash, HUD; **[NV]** no visual) | contrato; `define.md` · Open Questions |
| T012 | rastros e governança: R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**; não-execução | `.specify/memory/constitution.md`; `intake.md` |

Os nomes definitivos de T001..T012 serão os de `specs/051-derrota/tasks.md` (o diretório continha apenas o template `spec.md` no momento da leitura); a coluna "Eco" apenas rastreia a cobertura deste assessment. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa. Todo número citado é `[HIPÓTESE]`/`escolha de protótipo`; nada aqui é especificação, plano ou implementação.

Veredito: go
