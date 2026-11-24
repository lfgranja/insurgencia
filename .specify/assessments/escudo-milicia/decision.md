# Decision — Escudo de Milícia (W do Guerreiro)

- **Slug**: `escudo-milicia`
- **Decided**: 2026-11-23T22:00Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A/bloqueado (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | GDD §10.6:687-692 prevê o W do Guerreiro como `[HIPÓTESE]`, mas a sim não tem ação, constantes, campos nem rota única de dano ao jogador, e a HUD não mostra absorção; dois consumidores reais (jogador do cliente e sim determinístico) ficam sem efeito observável. |
| Força da evidência | strong | Decisão 80/20 e DoD em `.specify/memory/STATE.md:476-491`; GDD §10.6:687-692 lido; `crates/sim/src/lib.rs:15,101,110,137,143,154,166-178,190-196,248-...,568-584,841,961-995,1614-1650,1742-1785` e `crates/cliente/src/main.rs` verificados por leitura; constituição v1.2.2 lida. Nada reexecutado; efeito visual e gates permanecem **[NV]**. |
| Valor vs. inação | strong | 960/3125 contra 216/3125 (mapa/rotas PVE), 50/3125 (rede, inelegível por ADR-004) e 0 (rastreabilidade já fechada). Inação mantém duas rotas de dano direto sem absorção auditável e o W do Guerreiro apenas no papel. |
| Viabilidade / appetite | strong | `small`/`medium`: seis constantes (uma reservada), quatro campos em `Jogador`, uma ação nível 2 validada no sim, `aplicar_dano_a_jogador` substituindo duas atribuições, avanço de 150/420 ticks, quatro campos no hash, tecla `V`, linha `Escudo: N (ticks)` e cinco testes; sem contrato, protocolo, Cargo, stack, rede, mapa ou CI novos. |
| Alinhamento estratégico | strong | Nenhum número promovido a cânone; +15% contínuo registrado como reserva documentada; reaproveita o padrão dos ciclos 047-054; R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A**; promoção `dev→main` e deploy ficam com a SE. |
| Postura de risco | strong | Risco concentrado na rota única e mitigado por cinco testes nas fronteiras 250/PNJ/420/150/hash, piso zero, atribuição (nunca soma) e um alvo; fronteira de stack intacta; riscos residuais (visual **[NV]**, R4/R7 não executados) registrados explicitamente. |

## 80/20 (scorecard único do ciclo)

| Candidato | I | C | Fch | E⁻¹ | R⁻¹ | Impacto × Confiança × Fechamento × Esforço⁻¹ × Risco⁻¹ ÷ 3125 | Resultado |
|:--|--:|--:|--:|--:|--:|:--|:--|
| `escudo-milicia` (BF-002a) | 4 | 4 | 5 | 3 | 4 | 4×4×5×3×4 = 960/3125 | **960/3125** |
| Mapa/rotas de PVE (BF-003) | 4 | 3 | 3 | 2 | 3 | 4×3×3×2×3 = 216/3125 | 216/3125 |
| Rede real (BF-001) | 5 | 2 | 5 | 1 | 1 | 5×2×5×1×1 = 50/3125 — inelegível (ADR-004) | 50/3125 |
| Rastreabilidade 053/054 (BF-008) | 2 | 5 | 0 | 5 | 5 | 2×5×0×5×5 = 0 — já fechada | 0 |

Critério normativo da decisão `2026-11-23T22:00Z`: `impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`, escala 1–5, denominador **5⁵ = 3125**; filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo. Impacto **4** (defesa com regra tabelada no GDD), confiança **4** (valores e rotas lidos; **[NV]** no render), fechamento **5** (cliente + sim consomem o resultado), esforço⁻¹ **3** (escopo real em sim e cliente, com cinco testes), risco⁻¹ **4** (nenhuma troca de stack/protocolo/rede; risco concentrado na rota única). Este é o **único** scorecard do assessment; `intake.md`, `research.md` e `shape.md` repetem exatamente estes valores, sem pontuação paralela. A rede é inelegível por autorização ADR-004 (SWE decide); a rastreabilidade já foi corrigida em `d7f4103` e o fator fechamento 0 a elimina.

## Veredito e justificativa

**Veredito: go** — Opção A (Escudo de Milícia no Núcleo + cliente, com rota única de dano), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, **R6 N/A/bloqueado**. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, roteiro, commit, push ou PR foi produzido ou executado neste assessment; todo resultado de verificação permanece **[NV]**.

## Condições (R1)

1. **Regra rotulada**: GDD §10.6:687-692 inalterado; 25 Fôlego, 3 m, 250, 150 ticks, 420 ticks e nível 2 registrados como `[HIPÓTESE]`/`escolha de protótipo` em spec, validação e release. `RESISTENCIA_CONTINUA_ESCUDO_PERC = 15` fica **reservada/documentada**, sem dano contínuo no Núcleo e sem promessa de +15% contra gás.
2. **Constantes e estado**: `CUSTO_ESCUDO_FOLEGO = 25`, `ABSORCAO_ESCUDO = 250`, `TICKS_DURACAO_ESCUDO = 150`, `RECARGA_ESCUDO_TICKS = 420`, `ALCANCE_ESCUDO_M = 3.0` com `///`; campos `escudo_pv`, `escudo_ticks`, `escudo_recarga_ticks`, `escudo_alvo` em `Jogador`, zerados na criação, sem `HashMap`, RNG ou `Instant`.
3. **Ação e validação**: `"escudo"` em `ACOES_PERMITIDAS` com nível **2** explícito em `nivel_minimo_para_acao`; validação local ao sim (Fôlego ≥ 25, recarga 0, alvo self/aliado a ≤ 3 m, cota de um comando por atacante/tick) e monotônicos avançando somente em comando aceito.
4. **Rota única**: `aplicar_dano_a_jogador(&mut self, peer_id, dano)` substitui as duas atribuições diretas em `aplicar_ataque` (PvP) e `fase_ataque_pnjs`; absorção na **vítima**, `escudo_pv` consumido primeiro, excedente em `vida` com piso zero, `combate_ticks` e ordem das fases preservados.
5. **Tick e não-acumulação**: aplicação válida **atribui** 250/150/420 (nunca `+=`), cobre um alvo (self **ou** aliado, nunca os dois), expira exatamente ao completar 150 ticks zerando `escudo_pv`/`escudo_alvo`, e só reaplica com recarga 0; decremento único por tick em `avancar_tick`.
6. **Hash**: `serializar_jogadores`/`hash_estado` incluem os quatro campos em ordem de `peer_id` do `BTreeMap`; aplicar, absorver, expirar e carregar recarga alteram o hash; dois mundos idênticos mantêm o mesmo hash tick a tick.
7. **Cliente**: `solicitar_escudo` com `just_pressed(KeyCode::KeyV)` escolhendo self/aliado dentro de 3 m, sem comando e sem monotônicos sem alvo; HUD com `Escudo: N (ticks)` (N = absorção restante), preservando `WASD` (movimento), `T` (PNJ), `E` (análise), `Q` (ataque) e as linhas atuais.
8. **Testes e estilo**: cinco testes nomeados (`escudo_absorve_ate_250_dano`, `escudo_absorve_dano_de_pnj`, `escudo_recarga_420_ticks`, `escudo_expira_apos_150_ticks`, `hash_estado_cobre_escudo`) elevando 136 → **141**, sem afrouxar os existentes; `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, `///` em itens públicos.
9. **Verificação (por executar)**: `./verify.sh` esperado **141 testes**, cinco gates locais, roteiro 10/10 em `.specify/validations/escudo-milicia/` e CI `gates`+`rust` no head do PR — tudo **[NV]** neste assessment; captura visual real não prometida sem surface automatizável.
10. **Constituição v1.2.2**: `tasks` (T009) antes de `analyze` READ-ONLY (T010), `converge` append-only com **[NV]**, R4 condicionado ao roteiro e antes de R7 `1.0.0-local` (T012); **R6 N/A/bloqueado** — PARE se invocada, nunca issue simulada, nunca `gh` improvisado, dívida BF-007 entregue ao SWE; promoção `dev→main` e deploy são SWE-exclusivos; BF-002a passa a `fechado` com evidência, sem reabrir sem regressão provada.

## Riscos registrados

- **Rota única com regressão dupla**: `aplicar_dano_a_jogador` afeta PvP e PNJ; uma regressão atinge as duas fontes. Mitigado por substituir as duas atribuições e pelos testes de 250 e de PNJ, com piso zero preservado.
- **Acumulação e alvo duplo**: 250 + 250 virar 500, duração dobrada ou escudo em dois alvos violam §10.6:692. Mitigado por atribuição (nunca `+=`), um `escudo_alvo` e reaplicação só com recarga 0.
- **Relógio e determinismo**: `Instant`, arredondamento, decremento duplo no tick de aplicação ou `HashMap` divergente quebrariam o hash. Mitigado por ticks inteiros derivados de `PASSO_SEGUNDOS = 1/30` e segmento estável em ordem de `peer_id`.
- **Fronteira e visual**: captura da HUD com `--minimizado` não tem surface automatizável — **[NV]**, prova estrutural; nenhuma mudança em contratos, protocolo, Cargo, CI, stack ou rede é permitida neste corte.
- **Dívida R6**: N/A por constituição v1.2.2; não é dívida de produto, nenhuma issue simulada e nenhum `gh` improvisado.
- **Verificação geral**: gates, build, testes, `./verify.sh`, roteiro, CI, R4 e R7 permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: o Escudo de Milícia do GDD não é observável: sim sem ação/constantes/campos/rota única e HUD sem absorção, com dano ao jogador dividido entre PvP e PNJ.
- **Chosen approach**: Opção A — constantes `[HIPÓTESE]`, quatro campos em `Jogador`, `escudo` nível 2 validado no sim, `aplicar_dano_a_jogador` como rota única, 150/420 ticks por tick, hash com os quatro campos, tecla `V` e HUD `Escudo: N (ticks)`.
- **In scope**: regra rotulada, estado e constantes, validação, rota única, aplicação/expiração/recarga, hash, cliente, cinco testes, rastros e governança. **Out of scope**: cânone/GDD, +15% contínuo e canalização, mapa/rotas PVE, rede/ADR-004, contratos/protocolo/Cargo/CI/stack, promoção `dev→main`/deploy, **R6 (N/A)** e qualquer execução neste assessment.
- **Success metrics**: SC-001..SC-006 em `define.md` (SC-001..SC-005 com teste nomeado; SC-006 com fronteira, cliente e verificação), todas com baseline **[NV]** até execução.
- **Carried-forward open questions**: critério determinístico de seleção self/aliado a 3 m no cliente; nomes finais de FR/SC/T em `spec.md`/`tasks.md`; conformidade de R4/R7 com gates e roteiro; efeito renderizado da HUD com `--minimizado`; estabilidade do hash com os quatro campos. Nenhuma é clarificação de produto (R3 = 0).

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | regra e números `[HIPÓTESE]`, GDD inalterado, +15% reservado/documentado | GDD §10.6:687-692; `intake.md`; `decision.md` · Condição 1 |
| FR-002 | seis constantes (uma reservada) e campos `escudo_pv`/`escudo_ticks`/`escudo_recarga_ticks`/`escudo_alvo` em `Jogador` | `crates/sim/src/lib.rs:15,101,248-...`; `shape.md` · Opção A |
| FR-003 | `escudo` em `ACOES_PERMITIDAS`, nível 2, custo 25, alcance 3 m, validação e monotônicos só em aceite | `crates/sim/src/lib.rs:166-178,190-196`; `define.md` · Goals |
| FR-004 | `aplicar_dano_a_jogador` como rota única; absorção na vítima, 150/420 ticks, atribuição, um alvo | `crates/sim/src/lib.rs:841,992,699-712`; `shape.md` · Opção A |
| FR-005 | quatro campos serializados em `serializar_jogadores`/`hash_estado`, ordem de `peer_id` | `crates/sim/src/lib.rs:1614-1650,1742-1785`; `define.md` · Success Metrics |
| FR-006 | tecla `V`, HUD `Escudo: N (ticks)`, cinco testes, estilo Rust e governança | contrato da sessão; constituição v1.2.2; `decision.md` · Condições 7-10 |
| SC-001 | absorção de até 250; excedente com piso zero — `escudo_absorve_ate_250_dano` | `define.md` · SC-001 |
| SC-002 | dano de PNJ absorvido pela mesma rota do PvP — `escudo_absorve_dano_de_pnj` | `define.md` · SC-002 |
| SC-003 | recarga 420 ticks, recusa sem efeito — `escudo_recarga_420_ticks` | `define.md` · SC-003 |
| SC-004 | expiração em 150 ticks, sem acumulação — `escudo_expira_apos_150_ticks` | `define.md` · SC-004 |
| SC-005 | hash muda com os quatro campos — `hash_estado_cobre_escudo` | `define.md` · SC-005 |
| SC-006 | cliente, 141 testes, fronteira e estilo preservados | `define.md` · SC-006 |
| T001 | 00 constituição/branch — worktree `escudo-055`, branch `feature/055-escudo-milicia`, base `3f3b01c`; R1 `go`, R6 N/A | contrato da sessão; constituição v1.2.2 |
| T002 | 01 intake + 02 research — scorecard 960/3125, fontes lidas, nada executado | `intake.md`; `research.md` |
| T003 | 03 define + 04 shape — problema, métricas e 3 opções + opção eliminada | `define.md`; `shape.md` |
| T004 | 05 decide + 07 clarify — R1 `go`, R3 = 0, handoff ao `/speckit.specify` | este `decision.md` |
| T005 | 06 specify — `spec.md` com US/FR/SC e números rotulados | `specs/055-escudo-milicia/spec.md` (em construção) |
| T006 | 08 plan — roteiro de 10 passos, fronteira e R4 antes de R7 | `specs/055-escudo-milicia/plan.md` (a produzir) |
| T007 | 09 checklist de requisitos — um item por FR-001..FR-006 | `checklists/requirements.md` (a produzir) |
| T008 | revisão do checklist — 6 FR/6 SC verificáveis por inspeção | `checklists/requirements.md` (a produzir) |
| T009 | 10 tasks — este arquivo: T001..T012 com DAG linear, `tasks` antes de `analyze` | `specs/055-escudo-milicia/tasks.md` (a produzir) |
| T010 | 11 analyze (READ-ONLY) — depois de `tasks`, sem edição de insumos | `specs/055-escudo-milicia/analyze.md` (já lido, fora desta escrita) |
| T011 | 12 implement + 13 converge + 27 validate (R4) — sim, cliente, 5 testes e roteiro 10/10 | `crates/sim`, `crates/cliente`, `.specify/validations/escudo-milicia/` |
| T012 | 26 release (R7 `1.0.0-local`) — changelog por FR/T, rastreabilidade e rollback documentado, após R4 | `.specify/releases/escudo-milicia/release.md` (a produzir) |

Os nomes definitivos de T001..T012 são os de `specs/055-escudo-milicia/tasks.md`; a coluna "Eco" registra apenas a cobertura deste assessment. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa. Todo número citado é `[HIPÓTESE]`/`escolha de protótipo`; nada aqui é especificação, plano ou implementação, e nada foi executado.

Veredito: go
