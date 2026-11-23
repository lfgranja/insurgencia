# Decision — Defesa de barricada (autogestão)

- **Slug**: `defesa-barricada`
- **Decided**: 2026-11-23T05:47Z
- **R1**: `go` · **R3**: 0 clarificações · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Barricada 040 não rende o efeito canônico do GDD §7.1:306; a vitória por autogestão (§6.2:273) segue apenas declarada, sem verdade observável sobre tempo sob defesa, posse do defensor ou perda de acúmulo. |
| Força da evidência | strong | GDD §7.1:306 lido com a regra "+1 a cada 10 s de barricada intacta"; §10.3:574,577-579 fixa 200 PV/360 ticks como canônicos; scorecard e escolha registrados em STATE.md:304-315; mesmo contrato da sessão fixa os campos e a fase. |
| Valor vs. inação | strong | 256/625 contra 108/625 (ponte provisória), 48/625 (Súditos/Propaganda) e 10/625 (rede bloqueada por ADR-004); inação deixa o ciclo 040 sem fechamento entre obra coletiva e condição de vitória. |
| Viabilidade / appetite | strong | `small`: dois campos, uma fase determinística e uma linha no HUD, sobre a barricada já entregue; sem `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, Lightyear, async ou sockets. |
| Alinhamento estratégico | strong | Preserva a regra canônica de 10 s, a tese de vitória por conversão (§6.2:273) e a constituição v1.2.2 (R1 `go`, R4 antes de R7, `analyze` READ-ONLY após `tasks`, R6 N/A). |
| Postura de risco | strong | Riscos nomeados e mitigados por decisão: simplificação dono-defensor registrada (STATE.md:315); "tempo alvo 8 s" tratado como pacing, não crédito; disciplina de tick e ordem do hash cobertas por contrato; R6 N/A sem issue simulada. |

## 80/20

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Defesa de barricada → +1 autogestão/10 s (escolhida) | 4×4×4×4 = 256/625 | **256/625** |
| Ponte provisória | 3×3×3×4 = 108/625 | 108/625 |
| Súditos/Propaganda | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**.

A defesa vence porque é a única opção com **recompensa canônica e base já implementada simultaneamente**: GDD §7.1:306 fechou o "+1 a cada 10 s" e o ciclo 040 entregou a barricada (200 PV/360 ticks) com hash determinístico. Súditos/Propaganda apoia-se em `[HIPÓTESE]` (§6.2:275); a ponte é `[SUPOSIÇÃO]` (§10.3:574) e não tem recompensa própria; a rede, embora de impacto máximo, está bloqueada por ADR-004 até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`; nenhuma troca de engine sem novo ADR).

## Veredito e justificativa

**Veredito: go** — Opção A (defesa determinística de barricada), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. Regra canônica congelada: **+1 ponto de autogestão a cada 300 ticks (10 s a 1/30 s)** de barricada intacta com o dono no mesmo ponto; crédito e zeração de `defesa_ticks` no tick em que 300 é atingido; **nenhum crédito parcial**.
2. Condição de defesa: `pv > 0` e `ticks_restantes > 0` (barricada intacta) e `ponto_igual(dono.posicao, barreira.posicao)`. Se o dono sai, a barricada é destruída ou expira, `defesa_ticks` zera e não credita.
3. Estado e hash: `Jogador.pontos_autogestao: i64` e `Barreira.defesa_ticks: i64` existem, começam em 0 e são integralmente cobertos por `hash_estado` em ordem estável (`BTreeMap`).
4. Fase determinística: no máximo um incremento de `defesa_ticks` por barricada por tick, sem duplo crédito; HUD exibe `Autogestão`.
5. Simplificação registrada: dono no mesmo ponto como defensor; papéis Guerreiro/Artesão e reforço não entram neste ciclo.
6. Fronteira preservada: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`; nenhum Lightyear/async/sockets.
7. Constituição v1.2.2 aplicada: R4 antes de R7; `analyze` READ-ONLY depois de `tasks`; **R6 N/A** — removida da sequência executável; PARE e escale se invocada, sem `gh` improvisado nem issue simulada.

## Riscos registrados

- **Simplificação dono-defensor (STATE.md:315)**: o GDD §7.1:306 prevê Guerreiro na barricada e Artesão reforçando. Não reabrir nem estender papéis neste ciclo; se a SE decidir papéis distintos, é cycles futuro com contrato próprio.
- **Tempo alvo vs. regra de recompensa (§7.1:306)**: a coluna "tempo alvo 8 s" é pacing; a regra canônica de 10 s prevalece para o crédito. Não ajustar a regra para caber no pacing.
- **Disciplina de tick e hash**: risco invertido 4 (não 5) — um incremento por barreira por tick e ordem estável do `hash_estado` exigem verificação; são **[NV]** até execução.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação**: gates, testes, R4 e R7 permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: a barricada 040 não converte tempo sob defesa em autogestão; a fase Defender e a condição de vitória por conversão seguem sem suporte determinístico observável.
- **Chosen approach**: Opção A — `defesa_ticks` avança 1/tick com barricada intacta e dono no mesmo ponto; a cada 300 ticks, +1 em `pontos_autogestao` e zeração; perda da condição zera sem crédito; HUD mostra `Autogestão`.
- **In scope**: constantes (300 ticks; reuso de 200 PV/360 ticks), campos em `Jogador`/`Barreira`, condição de posse/ruptura, fase determinística, `hash_estado`, HUD. **Out of scope**: papéis Guerreiro/Artesão, Súditos/Nós/Propaganda, ponte, netcode, combate, contratos/protocolo, R6 (N/A).
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: relógio de 30 ticks/s (10 s = 300 ticks), exposição de `pv`/`ticks_restantes`/`dono`/`posicao` sem extensão de protocolo, `ponto_igual` e ordem do `BTreeMap` no hash — validar na especificação.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | constantes: 300 ticks por ponto; reuso de 200 PV / 360 ticks da barricada 040 | GDD §7.1:306; §10.3:574,577-579; contrato |
| FR-002 | `Jogador.pontos_autogestao: i64` e `Barreira.defesa_ticks: i64` | contrato; `define.md` |
| FR-003 | condição: `pv > 0`, `ticks_restantes > 0`, dono no mesmo ponto via `ponto_igual` | contrato; `shape.md` |
| FR-004 | fase determinística: +1 tick por barreira, crédito e zeração ao completar 300 | contrato; `define.md` |
| FR-005 | zeração sem crédito ao sair do ponto, destruir ou expirar; sem crédito parcial | contrato; `intake.md` |
| FR-006 | HUD `Autogestão`; nenhuma entrada nova; fronteira sem contrato/protocolo/rede | contrato; `intake.md` |
| SC-001 | +1 exatamente a cada 300 ticks | `define.md` · Success Metrics 1 |
| SC-002 | 299 ticks não creditam; contador volta a 0 ao creditar | `define.md` · Success Metrics 2 |
| SC-003 | sair do ponto zera `defesa_ticks` sem crédito | `define.md` · Success Metrics 3 |
| SC-004 | destruição/expiração zeram `defesa_ticks` sem crédito | `define.md` · Success Metrics 4 |
| SC-005 | 1 incremento por barreira por tick; campos no `hash_estado` em ordem estável | `define.md` · Success Metrics 5 |
| SC-006 | HUD exibe `Autogestão`; no máximo 1 crédito por barricada (360 ticks) | `define.md` · Success Metrics 6 |
| T001 | constante de 300 ticks e reuso de 200 PV / 360 ticks | `shape.md` · Opção A |
| T002 | campo `pontos_autogestao` em `Jogador` | `shape.md` · Opção A |
| T003 | campo `defesa_ticks` em `Barreira` | `shape.md` · Opção A |
| T004 | cobertura de ambos os campos em `hash_estado` | `shape.md` · Opção A |
| T005 | predicado de barricada intacta (`pv > 0`, `ticks_restantes > 0`) | `shape.md` · Opção A |
| T006 | predicado de dono no mesmo ponto (`ponto_igual`) | `shape.md` · Opção A |
| T007 | fase determinística de incremento (1/tick por barreira) | `shape.md` · Opção A |
| T008 | crédito +1 e zeração ao completar 300 ticks | `shape.md` · Opção A |
| T009 | zeração sem crédito nos três gatilhos de perda | `shape.md` · Opção A |
| T010 | estilo: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100` | contrato; ADR-004 |
| T011 | HUD com `Autogestão` | `shape.md` · Opção A |
| T012 | testes de acúmulo, crédito, zeração e hash | `shape.md` · Assumptions |

Os nomes definitivos de T001..T012 são os de `specs/044-defesa-barricada/tasks.md`; a coluna "Eco" apenas rastreia a cobertura deste assessment. R6 é **N/A** por constituição v1.2.2 e não recebe tarefa. Nada aqui é especificação, plano ou implementação.

Veredito: go
