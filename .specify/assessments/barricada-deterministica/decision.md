# Decision — Barricada determinística de materiais

- **Slug**: `barricada-deterministica`
- **Decided**: 2026-11-23T02:43Z
- **R1**: `go` · **R3**: 0 clarificações
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Artesão sem habilidade verificável; economia de Materiais não fecha em ação, e não há verdade observável sobre canalização/débito/expiração. |
| Força da evidência | strong | GDD §9.3:475,483 e §10.3:574,577-579 lidos com rótulo explícito de "canônicos"; conversão para ticks e API Rust fixadas no contrato; sem suposição de design. |
| Valor vs. inação | strong | 400/625 contra 108/625 (ponte provisória), 48/625 (combate/Fôlego) e 10/625 (servidor + Lightyear); inação mantém o pilar Materialismo Histórico apenas declarado. |
| Viabilidade / appetite | strong | `small`: Núcleo + entrada/HUD, sem `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`; comportamento já especificado no contrato. |
| Alinhamento estratégico | strong | Preserva os valores canônicos, a tese do MVP de desafios em vez de combate puro (§16.3:1509) e a constituição v1.2.2 (R4 antes de R7, `analyze` READ-ONLY após `tasks`). |
| Postura de risco | strong | Riscos nomeados e mitigados por decisão: conflito de rótulo §16.3/H4 registrado sem reabertura; dívida R6 registrada como automação ausente, sem `gh` improvisado nem issue simulada. |

## 80/20

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Construir barricada (escolhida) | 5×5×4×4 = 400/625 | **400/625** |
| Ponte provisória | 3×3×3×4 = 108/625 | 108/625 |
| Combate/Fôlego | 4×2×2×3 = 48/625 | 48/625 |
| Servidor + Lightyear | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 625.

A barricada vence porque é a única opção com **impacto 5 e confiança 5 simultâneos**: os números já são canônicos no GDD e o efeito é verificável por `hash_estado`, sem depender de rede, de balanceamento ou de uma decisão de design pendente. Combater primeiro inverte a premissa do MVP; adiar para netcode sincronizaria um estado que ainda não é determinístico; a ponte provisória é `[SUPOSIÇÃO]` e não tem critérios próprios.

## Veredito e justificativa

**Veredito: go** — Opção A (construção determinística de barricada), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T014. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. Valores canônicos congelados: 20 Materiais, 45 ticks, 200 PV, 360 ticks, máx. 2 por jogador; a 3ª remove a mais antiga; recusa sem saldo sem consumo parcial.
2. Validação e fase conforme contrato: `destino == posicao` via `ponto_igual`, `materiais >= 20`, revalidação ao completar, no máximo 1 canalização por jogador por tick (`BTreeSet<u64>` local), zera ao `mover` mudar posição, débito atômico e inserção ordenada em `BTreeMap`.
3. `hash_estado` cobre `construcao_ticks`, `construcao_ponto` e todas as barreiras na ordem de `BTreeMap`.
4. Cliente: tecla `B` just pressed com `seq`/`request_id` monotônicos via recurso local; HUD com Materiais, Barreiras e Construindo.
5. Fronteira preservada: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`.
6. Constituição v1.2.2 aplicada no ciclo: R4 antes de R7; `analyze` READ-ONLY depois de `tasks`; R6 fora da sequência executável, PARE se invocada.

## Riscos registrados

- **Conflito canônico (§16.3:1505 / H4:1678 vs §9.3:475,483 / §10.3:574,577-579)**: os mesmos números aparecem como canônicos em umas seções e como `[HIPÓTESE]` em outras. Não reabrir: prevalecem §9.3/§10.3; o rótulo divergente fica registrado como dívida documental.
- **Dívida R6 (automação)**: sem `resolve-tasks.sh`/adaptador, nenhuma issue é criada, nenhum `gh` é improvisado; PARE e escale se o fluxo tentar invocar R6. Não é dívida de produto.
- **Verificação**: gates, testes, R4 e R7 permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: sem barricada determinística, o Artesão não tem habilidade verificável e a economia de Materiais não se converte em controle temporário de corredor.
- **Chosen approach**: Opção A — canalização de 45 ticks, 200 PV por 360 ticks, máx. 2 por jogador, com validação atômica e hash determinístico.
- **In scope**: constantes, estado do jogador, tabela de barreiras, fase de skills, hash, entrada `B` e HUD. **Out of scope**: Fôlego/alcance/recarga/dimensões, ponte, netcode, combate, contratos/protocolo, R6.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: relógio de 30 ticks/s, `Comando.destino` sem extensão de protocolo, `ponto_igual` e ordem do `BTreeMap` no hash — validar na especificação.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | constantes 20 / 45 / 200 / 360 / 2 | GDD §9.3:475,483; §10.3:574; contrato |
| FR-002 | `construcao_ticks`, `construcao_ponto`, `SaldoInsuficiente` | contrato; `research.md` |
| FR-003 | `Comando.destino`, `ponto_igual`, `materiais >= 20` | contrato; `define.md` |
| FR-004 | fase: 1/tick, revalida, zera ao mover, débito atômico, remove a mais antiga | contrato; `shape.md` |
| FR-005 | `hash_estado` com campos e barreiras em ordem `BTreeMap` | contrato; `define.md` |
| FR-006 | tecla `B`, `seq`/`request_id`, HUD Materiais/Barreiras/Construindo | contrato; `intake.md` |
| SC-001 | recusa sem consumo parcial | `define.md` · Success Metrics 1 |
| SC-002 | 45 ticks + revalidação ao completar | `define.md` · Success Metrics 2 |
| SC-003 | no máximo 1 canalização por jogador por tick | `define.md` · Success Metrics 3 |
| SC-004 | mover zera `construcao_ticks`/`construcao_ponto` | `define.md` · Success Metrics 3 |
| SC-005 | máx. 2; 3ª remove a mais antiga | `define.md` · Success Metrics 4 |
| SC-006 | 200 PV / 360 ticks e cobertura integral pelo hash | `define.md` · Success Metrics 5 |
| T001 | constantes públicas | `shape.md` · Opção A |
| T002 | campos do Jogador e código de recusa | `shape.md` · Opção A |
| T003 | `Mundo.barreiras`, `proxima_barreira_id`, struct `Barreira` | `shape.md` · Opção A |
| T004 | comando e validação de construção | `shape.md` · Opção A |
| T005 | fase: limite de uma canalização por tick | `shape.md` · Opção A |
| T006 | fase: revalidação e cancelamento por movimento | `shape.md` · Opção A |
| T007 | débito atômico, inserção e remoção da mais antiga | `shape.md` · Opção A |
| T008 | PV / ticks restantes e expiração | `shape.md` · Opção A |
| T009 | `hash_estado` determinístico | `shape.md` · Opção A |
| T010 | cliente: tecla `B` e ids monotônicos | `shape.md` · Opção A |
| T011 | HUD Materiais / Barreiras / Construindo | `shape.md` · Opção A |
| T012 | testes de recusa, canalização e hash | `shape.md` · Assumptions |
| T013 | estilo: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100` | contrato |
| T014 | artefatos do ciclo e evidência de R4 | `define.md` · Open Questions |

Os nomes definitivos de T001..T014 são os de `specs/040-barricada-deterministica/tasks.md`; a coluna "Eco" apenas rastreia a cobertura deste assessment. Nada aqui é especificação, plano ou implementação.

Veredito: go
