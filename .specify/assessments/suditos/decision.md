# Decision — Súditos e Propaganda

- **Slug**: `suditos`
- **Decided**: 2026-11-23T06:58Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | A regra §9.2:464 é critério de aceitação escrito, mas não há no Núcleo contador de exposição, estado de conversão nem relógio de reversão; o pilar Materialismo Histórico / Ação Coletiva (§9.2:469) segue apenas declarado. |
| Força da evidência | strong | GDD §9.2:457-467 lido com a regra canônica de 4 s / 3 s; o que é hipótese (spawn, 6 m, funções, 60 s, Nós) foi explicitamente excluído; conversão 120/90 ticks, API prevista e 80/20 fixados no contrato da sessão e em `STATE.md` (06:58Z). |
| Valor vs. inação | strong | 48/625, empatado com Combate/Fôlego (48/625) e à frente de rede (10/625); inação deixa a única regra verificável de Súditos sem recipiente determinístico, embasando qualquer ciclo futuro de Nós/ilhas. |
| Viabilidade / appetite | strong | `small`: uma ação em `ACOES_PERMITIDAS`, struct em `BTreeMap<u64, Sudito>`, duas APIs, um comando com `BTreeSet<u64>`, hash e HUD; sem `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, Lightyear, `async` ou sockets. |
| Alinhamento estratégico | strong | Preserva 4 s = 120 ticks e 3 s = 90 ticks sem renegociação, mantém spawn/raio/funções rotulados `[HIPÓTESE]` e fora de escopo, e respeita a constituição v1.2.2 (R1 `go`, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY após `tasks`, `converge` append-only, R6 N/A). |
| Postura de risco | strong | Riscos nomeados e mitigados por decisão: injeção determinística no lugar de spawn hipotético, um avanço por id por tick, `BTreeMap` ordenado no hash e fronteira sem contrato/protocolo; empate 48/625 desempatado por contrato; R6 N/A sem issue simulada. |

## 80/20

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Súditos/Propaganda (escolhida) | 4×2×2×3 = 48/625 | **48/625** |
| Combate/Fôlego | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**.

Súditos/Propaganda vence porque é a única opção com **regra de aceitação canônica e fronteira pequena simultaneamente**: GDD §9.2:464 fechou "4 s de Propaganda → para de atacar estruturas e passa a panfletar; dano de jogador → reversão em 3 s", e o contrato da sessão fixou a conversão (120/90 ticks) e a API sem tocar em contrato, protocolo ou stack. Confiança 2 e facilidade 2 registram honestamente que o ecossistema do Súdito (spawn, raio, funções) é `[HIPÓTESE]` e que não existe sistema prévio no Núcleo — por isso a injeção determinística. O empate com Combate/Fôlego (48/625) é resolvido pelo contrato e pelo lastro canônico: Fôlego/alcance/recarga/dimensões são hipóteses (§10.3:574-579) e o MVP prova jogabilidade por desafios de ilha, "em vez de combate puro" (§16.3:1509). A rede, de impacto 5, está bloqueada por ADR-004 até decisão da SE e sincronizaria um estado que ainda não é determinístico.

## Veredito e justificativa

**Veredito: go** — Opção A (conversão determinística de Súdito), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 (`1.0.0-local`), R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. Regra canônica congelada: **120 ticks (4 s)** de exposição à Propaganda convertem o Súdito inimigo (`convertido = true` — para de atacar estruturas e passa a panfletar); dano de jogador inicia **90 ticks (3 s)** de reversão, ao fim dos quais `convertido = false`. Sem conversão antes de 120 e sem reversão antes de 90.
2. Estado e API: `Sudito { id, posicao, convertido, propaganda_ticks, reversao_ticks }` pública em `BTreeMap<u64, Sudito>` no `Mundo`; `adicionar_sudito(id, posicao)` para injeção determinística e `aplicar_dano_a_sudito(id)` para dano — sem IA, projéteis ou geometria de raio.
3. Ação e auto-avanço: `propaganda` entra em `ACOES_PERMITIDAS` da sim; o comando carrega `BTreeSet<u64>` e avança **no máximo um** tick de `propaganda_ticks` ou `reversao_ticks` por súdito por tick, sem duplicatas.
4. Determinismo e hash: todos os campos de todos os súditos, em ordem de `BTreeMap`, são refletidos em `hash_estado`.
5. Cliente e compatibilidade: tecla `G` just-pressed com `seq`/`request_id` monotônicos via recurso local; HUD com contagem de convertidos; nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` ou dependências.
6. Estilo (ADR-004 / constituição): `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem `async`/rede/Lightyear.
7. Fronteira preservada: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`.
8. Constituição v1.2.2 aplicada: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Hipóteses do ecossistema (GDD §9.2:461)**: spawn a cada 30 s, raio de 6 m, limite de 3 Súditos, funções trabalhador/propagandista/miliciano, permanência de 60 s e renda do trabalhador ficam fora de escopo e rotulados `[HIPÓTESE]`. Não reabrir; se a SE quiser o ecossistema, é ciclo futuro com contrato próprio.
- **Hipóteses de Nós (§6.2:275)**: conversão de Nós, Resistência Ideológica e Propaganda canalizada de 20 s permanecem fora do recorte desde o ciclo 044 — não são pendências deste assessment.
- **Empate 48/625 com Combate/Fôlego**: resolvido pelo contrato da sessão e pelo lastro canônico de §9.2:464; não é uma pontuação a renegociar.
- **Disciplina de tick e hash**: risco invertido 3 (não 5) — um avanço por id por tick, `BTreeSet` sem duplicatas e ordem estável do `BTreeMap` no `hash_estado` exigem verificação; permanecem **[NV]** até execução.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação**: gates, testes, R4 e R7 (`1.0.0-local`) permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: sem conversão determinística, a regra §9.2:464 (4 s de Propaganda interrompem ataque a estruturas; dano de jogador reverte em 3 s) não tem estado observável no Núcleo; o avanço material e ideológico segue apenas declarado.
- **Chosen approach**: Opção A — `Sudito` em `BTreeMap<u64, Sudito>`, injeção por `adicionar_sudito(id, posicao)`, `propaganda` em `ACOES_PERMITIDAS` com `BTreeSet<u64>` de ids expostos, conversão aos 120 ticks, `aplicar_dano_a_sudito(id)` com reversão aos 90 ticks, hash refletivo e HUD com contagem de convertidos e tecla `G`.
- **In scope**: constantes 120/90, struct e tabela, duas APIs, ação e auto-avanço, fase determinística, `hash_estado`, entrada `G` e HUD. **Out of scope**: spawn/raio/funções e 60 s (`[HIPÓTESE]` §9.2:461), Nós/Resistência Ideológica (§6.2:275), mapa/ilhas, combate/Fôlego, IA/projéteis, netcode, contratos/protocolo, R6 (N/A).
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: relógio de 30 ticks/s (4 s = 120, 3 s = 90), `propaganda` e `BTreeSet<u64>` sem extensão de protocolo, `adicionar_sudito`/`aplicar_dano_a_sudito` como vias exclusivas, dedup de ids e ordem do `BTreeMap` no hash — validar na especificação.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | constantes 120 ticks (4 s) e 90 ticks (3 s) | GDD §9.2:464; contrato da sessão |
| FR-002 | `Sudito { id, posicao, convertido, propaganda_ticks, reversao_ticks }` em `BTreeMap<u64, Sudito>` no `Mundo` | contrato; `define.md` |
| FR-003 | `adicionar_sudito(id, posicao)` e `aplicar_dano_a_sudito(id)` | contrato; `intake.md` |
| FR-004 | `propaganda` em `ACOES_PERMITIDAS`; auto-avanço com `BTreeSet<u64>`, 1 avanço por id/tick | contrato; `shape.md` |
| FR-005 | `hash_estado` cobre todos os campos em ordem de `BTreeMap` | contrato; `define.md` |
| FR-006 | tecla `G` just-pressed, ids monotônicos, HUD com contagem de convertidos; sem contrato/protocolo novos | contrato; `intake.md` |
| SC-001 | 120 ticks expostos → `convertido = true`; para de atacar estruturas e panfleta | `define.md` · Success Metrics 1 |
| SC-002 | 119 ticks não convertem; conversão no tick exato de 120 | `define.md` · Success Metrics 2 |
| SC-003 | dano inicia 90 ticks de reversão; ao fim, `convertido = false` | `define.md` · Success Metrics 3 |
| SC-004 | 89 ticks não revertem; reversão no tick exato de 90 | `define.md` · Success Metrics 4 |
| SC-005 | no máximo 1 avanço por súdito por tick; campos no `hash_estado` em ordem estável | `define.md` · Success Metrics 5 |
| SC-006 | HUD com convertidos e tecla `G`; fronteira sem contrato/protocolo/rede | `define.md` · Success Metrics 6 |
| T001 | constantes 120/90 | `shape.md` · Opção A |
| T002 | struct `Sudito` e `BTreeMap<u64, Sudito>` em `Mundo` | `shape.md` · Opção A |
| T003 | `adicionar_sudito(id, posicao)` | `shape.md` · Opção A |
| T004 | `aplicar_dano_a_sudito(id)` | `shape.md` · Opção A |
| T005 | ação `propaganda` em `ACOES_PERMITIDAS` | `shape.md` · Opção A |
| T006 | comando com `BTreeSet<u64>` e validação de ids/duplicatas | `shape.md` · Opção A |
| T007 | avanço de `propaganda_ticks` e conversão aos 120 | `shape.md` · Opção A |
| T008 | avanço de `reversao_ticks` e reversão aos 90 | `shape.md` · Opção A |
| T009 | `hash_estado` determinístico com todos os campos | `shape.md` · Opção A |
| T010 | cliente: tecla `G` e ids monotônicos | `shape.md` · Opção A |
| T011 | HUD com contagem de convertidos | `shape.md` · Opção A |
| T012 | testes de conversão, reversão, disciplina de tick e hash | `shape.md` · Assumptions |

Os nomes definitivos de T001..T012 serão os de `specs/046-suditos/tasks.md` (o diretório hoje contém apenas o template `spec.md`); a coluna "Eco" apenas rastreia a cobertura deste assessment. R6 é **N/A** por constituição v1.2.2 e não recebe tarefa. Nada aqui é especificação, plano ou implementação.

Veredito: go
