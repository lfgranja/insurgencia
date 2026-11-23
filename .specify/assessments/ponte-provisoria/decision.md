# Decision — Ponte provisória determinística

- **Slug**: `ponte-provisoria`
- **Decided**: 2026-11-23T06:25Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | A ponte tem custo, PV e duração canônicos no GDD (§9.3:475,483; §8.4:407) mas nenhuma materialização no Núcleo: não há ação, débito atômico, objeto com 60 PV/1350 ticks nem contagem no HUD; o uso em emboscadas segue apenas `[SUPOSIÇÃO]` (§10.3:574). |
| Força da evidência | strong | GDD §9.3:475,483 fixa 15 Materiais e recusa sem consumo parcial; §8.4:407 fixa 60 PV/45 s e o atalho de 8 s; conversão 1350 ticks a 1/30 s e API Rust vêm do contrato da sessão; scorecard e escolha em STATE.md:317-328; R1/R4/R6 em constitution.md:48-50,79-80,84. |
| Valor vs. inação | strong | 108/625 contra 48/625 (Súditos/Propaganda), 48/625 (Combate/Fôlego) e 10/625 (rede bloqueada por ADR-004); inação deixa o Delta-3 sem recurso construível e a coleta de Materiais sem fechamento mecânico. |
| Viabilidade / appetite | strong | `small`: uma ação em `ACOES_PERMITIDAS`, uma struct em `BTreeMap<u64, Ponte>`, quatro constantes, validação/débito atômico, expiração por tick, hash e HUD; sem `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, Lightyear, async ou sockets. |
| Alinhamento estratégico | strong | Preserva 15 Materiais / 60 PV / 45 s sem renegociação, mantém o atalho de 8 s fora de escopo como limitação e os 45 ticks de construção como protótipo, e respeita a constituição v1.2.2 (R1 `go`, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY após `tasks`, R6 N/A). |
| Postura de risco | strong | Riscos nomeados e mitigados por decisão: débito atômico sem consumo parcial, validação `destino == posicao`, um tick de construção/expiração por ponte e ordem estável do hash cobertos por contrato; atalho de 8 s e 45 ticks registrados; R6 N/A sem issue simulada. |

## 80/20

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Ponte provisória (15 Materiais, 60 PV, 45 s; escolhida) | 3×3×3×4 = 108/625 | **108/625** |
| Súditos/Propaganda | 4×2×2×3 = 48/625 | 48/625 |
| Combate/Fôlego | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**.

A ponte vence porque é o próximo recurso com **todos os números canônicos fechados e base reaproveitável simultaneamente**: GDD §9.3:475,483 fechou 15 Materiais e a recusa sem saldo sem consumo parcial, §8.4:407 fechou 60 PV e 45 s de duração, e o padrão de ações/HUD/hash de ciclos anteriores já existe. Súditos/Propaganda e Combate/Fôlego apoiam-se em `[HIPÓTESE]` (§6.2:275; §10.3:574-579); a rede, embora de impacto máximo, está bloqueada por ADR-004 até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`; nenhuma troca de engine sem novo ADR). Confiança 3 reflete o uso em emboscadas marcado `[SUPOSIÇÃO]` e o atalho de 8 s fora de escopo.

## Veredito e justificativa

**Veredito: go** — Opção A (ponte provisória determinística), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. Regra canônica congelada: custo **15 Materiais** (GDD §9.3:475,483) e recusa sem saldo **sem consumo parcial** — débito atômico de 15, todo ou nada; `destino == posicao` é pré-condição da ação.
2. Estado e duração canônicos: `Ponte { id, dono, posicao, pv, ticks_restantes }` em `BTreeMap<u64, Ponte>` em `Mundo`, com `CUSTO_PONTE_MATERIAIS = 15`, `PV_PONTE = 60`, `TICKS_DURACAO_PONTE = 1350` (45 s a 1/30 s); remoção quando `pv == 0` ou `ticks_restantes == 0`, uma única vez.
3. Construção como protótipo: `TICKS_CONSTRUCAO_PONTE = 45` é escolha de implementação, **não valor canônico**; registrada como tal, sem promoção a regra do GDD.
4. Atalho de 8 s entre ilhas (§8.4:407) depende de mapa/ilhas e fica **fora de escopo**, registrado como limitação; nenhuma travessia, ilha Delta ou emboscada entra neste ciclo.
5. Ação e compatibilidade: `construir_ponte` entra em `ACOES_PERMITIDAS` da sim; nenhuma alteração em `crates/contratos` ou `PROTOCOL_VERSION`; HUD com contagem de Pontes e tecla `P`.
6. Determinismo e estilo: todas as Pontes cobertas por `hash_estado` em ordem estável, no máximo um tick de construção e um de expiração por ponte; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem async/rede/Lightyear.
7. Fronteira preservada: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`.
8. Constituição v1.2.2 aplicada: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Atalho de 8 s (§8.4:407)**: fora de escopo por depender de mapa/ilhas; registrado como limitação, não reaberto. Se a SE quiser travessia, é ciclo futuro com contrato próprio.
- **Tempo de construção de 45 ticks**: escolha de protótipo, não valor canônico; não negociar nem confundir com os 45 s de duração (1350 ticks).
- **Disciplina de débito, validação e hash**: risco invertido 4 (não 5) — débito atômico sem consumo parcial, `destino == posicao`, um tick de construção/expiração por ponte e ordem estável do `BTreeMap` no `hash_estado` exigem verificação; permanecem **[NV]** até execução.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação**: gates, testes, R4 e R7 permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: a ponte provisória tem regras canônicas fechadas no GDD (15 Materiais, 60 PV, 45 s) mas nenhuma materialização determinística no Núcleo; o Delta-3 e o uso em emboscadas seguem apenas documentais, sem custo debitado, objeto com PV/duração ou contagem observável.
- **Chosen approach**: Opção A — `construir_ponte` em `ACOES_PERMITIDAS` valida `destino == posicao` e saldo ≥ 15, debita 15 Materiais de forma atômica (recusa sem consumo parcial) e cria `Ponte { id, dono, posicao, pv, ticks_restantes }` em `BTreeMap<u64, Ponte>` em `Mundo`, com 60 PV e 1350 ticks de duração, construção de 45 ticks (protótipo), expiração determinística, remoção por `pv == 0`, cobertura em `hash_estado` e HUD com contagem de Pontes e tecla `P`.
- **In scope**: constantes (15 / 60 / 1350; 45 de construção como protótipo), ação, struct em `BTreeMap`, validação e débito atômico, expiração, hash, HUD. **Out of scope**: atalho de 8 s (limitação de mapa/ilhas), valor canônico para os 45 ticks de construção, mapa/ilhas/emboscadas, combate/Fôlego, Súditos/Nós/Propaganda, netcode, contratos/protocolo, R6 (N/A).
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: relógio de 30 ticks/s (45 s = 1350 ticks), exposição de `Mundo`/`Comando.destino` para a validação sem estender o protocolo, atomicidade do débito de Materiais, ordem do `BTreeMap` no `hash_estado` e tecla `P` no HUD — validar na especificação.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | constantes: 15 Materiais, 60 PV, 1350 ticks (45 s); 45 ticks de construção como protótipo | GDD §9.3:475,483; §8.4:407; contrato |
| FR-002 | ação `construir_ponte` em `ACOES_PERMITIDAS` da sim | contrato; `define.md` |
| FR-003 | struct pública `Ponte { id, dono, posicao, pv, ticks_restantes }` em `BTreeMap<u64, Ponte>` em `Mundo` | contrato; `shape.md` |
| FR-004 | validação `destino == posicao`, saldo ≥ 15 e débito atômico sem consumo parcial | GDD §9.3:483; contrato |
| FR-005 | expiração determinística em 1350 ticks, remoção por `pv == 0`, cobertura em `hash_estado` | contrato; `shape.md` |
| FR-006 | HUD com contagem de Pontes e tecla `P`; fronteira sem contrato/protocolo/rede | contrato; `intake.md` |
| SC-001 | construção válida debita 15 e cria Ponte com 60 PV / 1350 ticks | `define.md` · Success Metrics 1 |
| SC-002 | saldo < 15 recusa sem consumo parcial e sem Ponte | `define.md` · Success Metrics 2 |
| SC-003 | `destino != posicao` recusa sem débito | `define.md` · Success Metrics 3 |
| SC-004 | remoção por `pv == 0` ou `ticks_restantes == 0`, uma única vez | `define.md` · Success Metrics 4 |
| SC-005 | `hash_estado` estável; 1 tick de construção e 1 de expiração por ponte | `define.md` · Success Metrics 5 |
| SC-006 | HUD exibe contagem de Pontes e tecla `P` constrói; nada em contrato/protocolo/rede | `define.md` · Success Metrics 6 |
| T001 | constantes `CUSTO_PONTE_MATERIAIS`, `PV_PONTE`, `TICKS_DURACAO_PONTE`, `TICKS_CONSTRUCAO_PONTE` | `shape.md` · Opção A |
| T002 | ação `construir_ponte` em `ACOES_PERMITIDAS` | `shape.md` · Opção A |
| T003 | struct `Ponte` e campo `BTreeMap<u64, Ponte>` em `Mundo` | `shape.md` · Opção A |
| T004 | validação `destino == posicao` e saldo ≥ 15 | `shape.md` · Opção A |
| T005 | débito atômico e recusa sem consumo parcial | `shape.md` · Opção A |
| T006 | fase de construção de 45 ticks (protótipo) | `shape.md` · Opção A |
| T007 | expiração determinística por `ticks_restantes` e remoção por `pv == 0` | `shape.md` · Opção A |
| T008 | cobertura de todas as Pontes em `hash_estado` em ordem estável | `shape.md` · Opção A |
| T009 | estilo: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100` | contrato; ADR-004 |
| T010 | HUD com contagem de Pontes e tecla `P` | `shape.md` · Opção A |
| T011 | limitação registrada: atalho de 8 s fora de escopo | `intake.md`; `shape.md` |
| T012 | testes de custo, recusa, construção, expiração, hash e HUD | `shape.md` · Assumptions |

Os nomes definitivos de T001..T012 são os de `specs/045-ponte-provisoria/tasks.md`; a coluna "Eco" apenas rastreia a cobertura deste assessment. R6 é **N/A** por constituição v1.2.2 e não recebe tarefa. Nada aqui é especificação, plano ou implementação.

Veredito: go
