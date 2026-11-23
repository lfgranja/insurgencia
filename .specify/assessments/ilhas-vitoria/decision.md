# Decision — Controle de ilha por Súdito (subset de vitória)

- **Slug**: `ilhas-vitoria`
- **Decided**: 2026-11-23T11:12Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | GDD §9.4:489,501 fixa 10 s contínuos de Súdito da Comuna e o sinal `ilha_perdida`, mas `Mundo` não tem `ilhas` nem `sinais`: a condição de vitória canônica não existe como estado verificável na sim. |
| Força da evidência | strong | GDD §9.1:439-455, §9.2:459-466 e §9.4:487-502 lidos; `crates/sim/src/lib.rs` verificado (`avancar_tick` 393-399, `fase_defesa` 506-525, `Sudito.convertido` 185-191, `Mundo` 216-238, conversão 344-345/858-865, `hash_estado` 909-980); decisão 80/20 em `.specify/memory/STATE.md:363-372`; constituição v1.2.2. |
| Valor vs. inação | strong | 108/625 contra 48/625 (Nós/vitória), 48/625 (papéis W/E/R) e 10/625 (rede); inação mantém a regra de vitória apenas no papel e obriga o próximo ciclo a reconstruir estado de ilha e histórico de sinais. |
| Viabilidade / appetite | strong | `small`/`medium`: um struct, dois campos em `Mundo`, uma fase no tick com acumulador de 300 ticks já precedido por `fase_defesa`, uma fila de sinais e suas projeções em hash/HUD; sem contrato, protocolo, Cargo, stack, mapa ou rede. |
| Alinhamento estratégico | strong | Não reabre nenhum número canônico nem hipóteses (§9.1:439 dedupe em 2 s; §9.4:494 reconversão em 20 s); respeita R1 `go`, R4 antes de R7 `1.0.0-local` e `analyze` READ-ONLY; a simplificação de presença global é **documentada**, não promovida a cânone. |
| Postura de risco | strong | Risco invertido 3: sim permanece determinística, contratos/protocolo/Cargo/stack/rede intocados; emissão de `ilha_perdida` restrita à transição evita fila espúria e divergência de hash; ilhas e sinais entram no `hash_estado`; R6 registrada como N/A, sem issue simulada nem `gh` improvisado. |

## 80/20 (scorecard único do ciclo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Controle de ilha por Súdito — 10 s contínuos (escolhida) | 4×3×3×3 = 108/625 | **108/625** |
| Nós de Controle + vitória completa | 4×2×2×3 = 48/625 | 48/625 |
| Papéis W/E/R + níveis | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério do contrato — impacto × confiança × facilidade ÷ risco, escala 1–5, denominador **625** (5⁴); o quarto fator é o risco invertido (segurança): 5 = risco mínimo, 1 = risco máximo. Este é o único scorecard do assessment; os mesmos valores aparecem em `intake.md`, `research.md` e `shape.md`, sem pontuação paralela. Fonte autoritativa: `.specify/memory/STATE.md:363-372`.

O controle de ilha vence porque é a única opção que materializa uma regra **canônica** de vitória com fronteira mínima e determinismo preservado. Impacto **4** registra a introdução da primeira condição de vitória observável da sim; confiança **3** registra que o GDD e o acumulador de 300 ticks são conhecidos, mas que a semântica de presença global, a emissão de sinais e o hash exigem validação na spec; facilidade **3** reflete poucos pontos de mudança, porém distribuídos entre tick, hash e HUD; risco invertido **3** reflete a fronteira determinística intacta e o custo de disciplina imposto pela simplificação. Nós/vitória (48/625) pressupõem o estado de ilha desta fatia e topologia de rotas; papéis (48/625) abrem novo contrato de ações sem endereçar §9.4; rede (10/625) inverte a dependência regra local → sincronização e está bloqueada por ADR-004.

## Veredito e justificativa

**Veredito: go** — Opção A (controle de ilha por Súdito, 10 s contínuos, presença global documentada), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. **Modelo e API**: `Ilha { id, controlada, controle_ticks }`, `Mundo.ilhas: BTreeMap<u64, Ilha>` e `adicionar_ilha(id)` declarados em `crates/sim/src/lib.rs`; a spec fixa quantas ilhas o protótipo registra, sem inventar topologia ou geometria.
2. **Regra canônica**: com pelo menos um Súdito convertido, `controle_ticks` avança 1 por tick e, aos **300 ticks contínuos** (10 s a 30 Hz, GDD §9.4:489), `controlada = true`. Um único tick sem Súdito zera `controle_ticks` e marca `controlada = false` (Latente) — nenhum acúmulo intermitente.
3. **Sinal de perda**: `Mundo.sinais: VecDeque<String>` (ou equivalente determinístico) recebe `ilha_perdida:<id>` **uma vez por evento de perda** — transição `controlada → Latente` **ou** perda de progresso parcial (`controle_ticks > 0` antes de 300) — por falta de Súdito, conforme spec/código. A hipótese `[HIPÓTESE]` de dedupe em 2 s (§9.1:439) **não** é implementada; emissão de `ilha_convertida` fica como pergunta a validar na spec e, se admitida, é escolha de protótipo.
4. **Fase no tick**: `fase_controle_ilhas()` executada em `avancar_tick` entre `fase_defesa()` e `fase_finish()`, observando o estado `convertido` já resolvido no mesmo tick; ordem das demais fases e incremento final de `tick` preservados.
5. **Determinismo**: `hash_estado` passa a incluir ilhas (`id`, `controlada`, `controle_ticks`) e a fila de sinais em ordem estável; estados diferentes produzem hashes diferentes e o replay determinístico existente permanece válido.
6. **HUD e simplificação**: a HUD exibe contagem de ilhas controladas e os sinais, com rótulo que identifica a leitura como protótipo. A **presença de qualquer Súdito convertido é tratada como global** — ausência de mapa/rede — e o Súdito é proxy da condição "Desafio de Ilha concluído" de §9.4; essa simplificação deve ser registrada em spec e comentários, sem promoção a cânone.
7. **Fronteira e estilo**: nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack ou rede; padrões de acumulador/hash dos ciclos 039/040 reaproveitados; `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.
8. **Constituição v1.2.2**: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Presença global indevida**: mitigado por decisão — rótulo explícito de simplificação em spec, HUD e comentários; nenhum raio, posição por ilha, mapa ou rota é inventado. Se a spec tratar a presença global como texto canônico de §9.4, é violação deste assessment.
- **Proxy do Desafio**: o GDD exige duas condições; o protótipo observa apenas o Súdito convertido. A distinção precisa constar na spec para que o proxy não vire regra canônica.
- **Sinal repetido e hash**: `ilha_perdida:<id>` apenas em eventos de perda (transição `controlada → Latente` ou progresso parcial) evita fila espúria e divergência de `hash_estado`; inclusão de ilhas e sinais no hash é obrigatória, não opcional. Permanece **[NV]** até execução.
- **Continuidade dos 10 s**: um único tick sem presença zera o progresso; não pode haver tolerância, acúmulo intermitente ou "perdão" de tick.
- **Fronteira de stack**: nenhuma mudança em contratos/protocolo/Cargo/stack/rede; ADR-004 continua bloqueando rede até decisão da SE.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação geral**: gates, build, testes, formatadores, R4 e R7 (`1.0.0-local`) permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: a regra canônica de vitória (GDD §9.4:489,501 — 10 s contínuos de Súdito da Comuna, retorno a Latente e `ilha_perdida` na ausência) não existe como estado verificável: `Mundo` não tem ilhas nem sinais, `avancar_tick` não tem fase de controle e `hash_estado` não cobre o novo estado.
- **Chosen approach**: Opção A — `Ilha { id, controlada, controle_ticks }`, `Mundo.ilhas: BTreeMap<u64, Ilha>` com `adicionar_ilha(id)`, `Mundo.sinais: VecDeque<String>`, `fase_controle_ilhas` entre `fase_defesa` e `fase_finish` (300 ticks → `controlada`; ausência → Latente + `ilha_perdida` na transição), hash e HUD atualizados. **Presença de qualquer Súdito convertido tratada como global: simplificação documentada.**
- **In scope**: modelo/API, regra de 300 ticks, sinal de perda, fase no tick, hash, HUD, rastros e governança. **Out of scope**: Nós/vitória completa, papéis W/E/R, mapa/geometria/rotas, dedupe de 2 s, contratos/protocolo/Cargo/stack/rede, captura prometida e **R6 (N/A)**.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: emissão ou não de `ilha_convertida`; quantidade de ilhas registradas e API `adicionar_ilha`; formato dos `String` em `Mundo.sinais` e eventual drenagem da fila; forma de exibição dos sinais na HUD; validar na spec e no roteiro de verificação determinística.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | `Ilha { id, controlada, controle_ticks }`, `Mundo.ilhas: BTreeMap<u64, Ilha>`, `adicionar_ilha(id)` | contrato da sessão; `intake.md` |
| FR-002 | regra canônica §9.4: 300 ticks contínuos com Súdito → `controlada`; ausência → Latente e `controle_ticks = 0` | `docs/GDD - Insurgência.md:489,501`; `define.md` |
| FR-003 | `Mundo.sinais: VecDeque<String>` e `ilha_perdida:<id>` uma vez por evento de perda (transição `controlada → Latente` ou progresso parcial) | GDD §9.1:447-448, §9.4:489; `shape.md` · Opção A |
| FR-004 | `fase_controle_ilhas` entre `fase_defesa` e `fase_finish`, lendo `sudito.convertido` do mesmo tick | `crates/sim/src/lib.rs:393-399,858-865`; `define.md` |
| FR-005 | presença global de qualquer Súdito convertido como **simplificação documentada**; sem mapa/geometria e sem modelar o Desafio de Ilha | contrato da sessão; `.specify/memory/STATE.md:363-372`; `research.md` |
| FR-006 | ilhas e sinais em `hash_estado`, HUD com contagem/sinais, fronteira intacta, rastros e constituição v1.2.2 | contrato; `crates/sim/src/lib.rs:909-980`; `.specify/memory/constitution.md` |
| SC-001 | com Súdito convertido, `controle_ticks` cresce 1/tick e `controlada = true` aos 300 ticks | `define.md` · Success Metrics 1 |
| SC-002 | qualquer tick sem Súdito zera o progresso e impede a conclusão dos 10 s | `define.md` · Success Metrics 2 |
| SC-003 | perda de ilha controlada → Latente **ou** de progresso parcial → `ilha_perdida:<id>` uma vez por evento, sem sinais espúrios | `define.md` · Success Metrics 3 |
| SC-004 | ordem das fases preservada; controle observa conversão/reversão do mesmo tick | `define.md` · Success Metrics 4 |
| SC-005 | `hash_estado` reflete ilhas e sinais; replay determinístico preservado | `define.md` · Success Metrics 5 |
| SC-006 | HUD com contagem de ilhas controladas e sinais; fronteira e estilo Rust intactos | `define.md` · Success Metrics 6 |
| T001 | struct `Ilha` e campos `Mundo.ilhas` / `Mundo.sinais` | `shape.md` · Opção A |
| T002 | `adicionar_ilha(id)` com registro determinístico | `shape.md` · Opção A |
| T003 | checagem de presença via `suditos.values().any(|s| s.convertido)` (presença global) | `shape.md` · Opção A |
| T004 | acumulador `controle_ticks` com marco de 300 ticks e continuidade rígida | `shape.md` · Assumptions |
| T005 | transição para `controlada = true` aos 300 ticks | `shape.md` · Opção A |
| T006 | enfileiramento de `ilha_perdida:<id>` apenas em eventos de perda (transição de `controlada` ou progresso parcial) | `shape.md` · Opção A |
| T007 | `fase_controle_ilhas()` inserida entre `fase_defesa()` e `fase_finish()` em `avancar_tick` | `shape.md` · Opção A |
| T008 | ilhas e sinais incluídos em `hash_estado` em ordem estável | `shape.md` · Assumptions |
| T009 | HUD com contagem de ilhas controladas e sinais da fila | `shape.md` · Opção A |
| T010 | rótulo de protótipo/presença global em HUD e comentários, sem promoção a cânone | `shape.md` · Opção A |
| T011 | roteiro de verificação determinística (leitura de código, hash, HUD; **[NV]** o que não for executado) | contrato; `define.md` · Open Questions |
| T012 | rastros e governança: R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**; não-execução | `.specify/memory/constitution.md`; `intake.md` |

Os nomes definitivos de T001..T012 serão os de `specs/049-ilhas-vitoria/tasks.md` (diretório ainda a criar); a coluna "Eco" apenas rastreia a cobertura deste assessment. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa. A presença global do Súdito convertido é **simplificação documentada**, não regra canônica; nenhum número do GDD foi reaberto e nada aqui é especificação, plano ou implementação.

Veredito: go
