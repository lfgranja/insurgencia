# Decision — Combate e Fôlego

- **Slug**: `combate-folego`
- **Decided**: 2026-11-23T07:35Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | `atacar_forte` está em `ACOES_PERMITIDAS` mas não tem efeito, e o `Jogador` não carrega Fôlego: a reserva física do GDD §10.1:517 não tem recipiente observável, nem débito, dano, regeneração ou relógio de combate verificáveis. |
| Força da evidência | strong | GDD §10.1:517-523 lido com os rótulos `[HIPÓTESE]` de 100, 10/s, 5/s e 2 m; §10.6:680 dá o nome canônico do protótipo, sem promover números; custo 30, dano 10 e 90 ticks são escolhas de protótipo fixadas no contrato e em `STATE.md:342-351`. |
| Valor vs. inação | strong | 48/625 empatado com o 48/625 do ciclo 046 (Súditos/Propaganda) e à frente da rede (10/625); inação deixa a última frente de produto sem mudança de stack (`STATE.md:351`) sem estado determinístico para cansaço ou acerto. |
| Viabilidade / appetite | strong | `small`: uma ação existente ganha efeito, três campos no `Jogador`, um acumulador em `fase_finish`, duas recusas, `BTreeSet<u64>`, hash e HUD; sem `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, Lightyear, `async` ou sockets. |
| Alinhamento estratégico | strong | Preserva os rótulos: 100/10/s/5/s/2 m continuam `[HIPÓTESE]`, 30/10/90 ticks continuam escolha de protótipo; respeita §16.3:1509 ("em vez de combate puro") pelo escopo mínimo e a constituição v1.2.2 (R1 `go`, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY após `tasks`, `converge` append-only, R6 N/A). |
| Postura de risco | strong | Riscos nomeados e mitigados por decisão: débito atômico, um ataque por jogador por tick, sem duplo crédito de regeneração, `BTreeMap`/ordem estável no hash, recusa sem consumo parcial e fronteira sem contrato/protocolo; empate 48/625 resolvido pelo contrato 047; R6 N/A sem issue simulada. |

## 80/20

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Combate/Fôlego — `atacar_forte` com efeito (escolhida) | 4×2×2×3 = 48/625 | **48/625** |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**.

Combate/Fôlego vence porque é a única frente de produto restante com **fronteira pequena e efeito auditável por `hash_estado` simultaneamente**: a ação já existe em `ACOES_PERMITIDAS`, e o contrato da sessão fixou a API e os valores de protótipo sem tocar em contrato, protocolo ou stack. Confiança **2** e facilidade **2** registram honestamente que todo número de combate é `[HIPÓTESE]`/`escolha de protótipo` (§10.1:519-523; contrato) e que a objeção histórica de GDD §16.3:1509 — MVP de desafios de ilha, "em vez de combate puro" — permanece válida: o recorte é um ataque e um recurso, não um sistema de combate. Risco invertido **3** reflete a disciplina de tick e de hash ainda por verificar. O empate numérico com Súditos/Propaganda (48/625 no ciclo 046) é resolvido pelo contrato 047 e não é pontuação a renegociar; a rede (10/625) está bloqueada por ADR-004 e sincronizaria um estado que ainda não é determinístico.

## Veredito e justificativa

**Veredito: go** — Opção A (`atacar_forte` com efeito e Fôlego observável), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 (`1.0.0-local`), R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. Valores rotulados: Fôlego máximo **100**, regeneração **10/s fora de combate** e **5/s em combate** e alcance contato **2 m** permanecem `[HIPÓTESE]` (GDD §10.1:519-523); custo de ataque **30**, dano **10** e duração de combate **90 ticks** são **escolhas de protótipo** do contrato da sessão. Nenhum valor pode ser apresentado como cânone; os números de §10.6 também são `[HIPÓTESE]` e não são reaproveitados.
2. Estado e hash: `Jogador` ganha `folego` (**teto 100 — `[HIPÓTESE]`**), `folego_ticks` (acumulador) e `combate_ticks` (**90 ticks — escolha de protótipo**), todos incluídos no `hash_estado` em ordem estável.
3. Ação e validação: `atacar_forte` ganha efeito em `crates/sim/src/lib.rs`; valida Fôlego (**≥ 30 — escolha de protótipo**) e alvo no ponto/alcance (**2 m — `[HIPÓTESE]`**), com `CodigoRecusa::FolegoInsuficiente` e `CodigoRecusa::AlvoInvalido`, sem débito parcial.
4. Dano e disciplina: dano **10 (escolha de protótipo)** aplicado com `saturating_sub`; no máximo **um** ataque por jogador por tick via `BTreeSet<u64>`, sem duplicatas.
5. Regeneração: em `fase_finish`, **+1 Fôlego a cada 3 ticks** fora de combate (de **10/s — `[HIPÓTESE]`**) e **+1 a cada 6 ticks** com `combate_ticks > 0` (de **5/s — `[HIPÓTESE]`**), sem crédito no tick do ataque e sem reinício indevido do acumulador.
6. Cliente e compatibilidade: tecla `Q` just-pressed com `seq`/`request_id` monotônicos via recurso local agenda `atacar_forte`; HUD exibe `Fôlego`/`Vida`; nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`.
7. Estilo (ADR-004 / constituição): `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem `async`/rede/Lightyear.
8. Fronteira preservada: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`. Constituição v1.2.2 aplicada: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Nenhum número é cânone (GDD §10.1:519-523, §10.6:680)**: 100, 10/s, 5/s, 2 m são `[HIPÓTESE]`; 30, 10 e 90 ticks são escolhas de protótipo. Se a spec promover qualquer valor a dogma, é violação deste assessment — não reabrir o contrato.
- **Objeção §16.3:1509**: o MVP prova jogabilidade por desafios de ilha, "em vez de combate puro". Mitigada pelo escopo mínimo (um ataque, um recurso, um HUD); IA, projéteis, colisão e balanceamento ficam fora. Não reabrir a decisão do contrato 047.
- **Empate 48/625 com Súditos/Propaganda (ciclo 046)**: resolvido pelo contrato da sessão; não é pontuação a renegociar, nem reavaliação do rastro FR/SC/T de 046.
- **Disciplina de tick e hash**: risco invertido 3 (não 5) — débito atômico, um ataque por jogador por tick, acumulador sem duplo crédito e ordem estável no `hash_estado` exigem verificação; permanecem **[NV]** até execução.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação**: gates, testes, R4 e R7 (`1.0.0-local`) permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: `atacar_forte` existe sem efeito e o GDD §10.1:517-523 não tem recipiente no Núcleo: não há reserva física, débito, dano, regeneração dependente de combate nem relógio observável; cansaço e acerto não são verificáveis por `hash_estado`.
- **Chosen approach**: Opção A — campos `folego`/`folego_ticks`/`combate_ticks` no `Jogador`, efeito de `atacar_forte` com validação de Fôlego (**30 — escolha de protótipo**) e alvo (**2 m — `[HIPÓTESE]`**), dano **10 (escolha de protótipo)** com `saturating_sub`, `BTreeSet<u64>` para um ataque por jogador por tick, regeneração em `fase_finish` (**+1/3 ticks** fora de combate, **+1/6 ticks** em combate, derivado de **10/s** e **5/s `[HIPÓTESE]`**), janela de **90 ticks (escolha de protótipo)**, hash refletivo, tecla `Q` e HUD `Fôlego`/`Vida`.
- **In scope**: constantes documentadas e rotuladas, três campos e hash, efeito/validação/recusas, dano e débito atômicos, disciplina de tick, regeneração acumulada, cliente e HUD. **Out of scope**: IA, Súditos, projéteis, colisão, balanceamento e os números de §10.6 (`[HIPÓTESE]`), Súditos/Propaganda e seu rastro (ciclo 046), mapa/ilhas, "combate puro", netcode, contratos/protocolo, **R6 (N/A)**.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: obtenção do ponto do alvo para o alcance de **2 m (`[HIPÓTESE]`)** sem geometria; variantes de `CodigoRecusa` disponíveis; `atacar_forte`/`BTreeSet<u64>` sem extensão de protocolo; ausência de duplo crédito de regeneração no tick do ataque — validar na especificação.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | constantes rotuladas: 100, 10/s, 5/s, 2 m `[HIPÓTESE]`; 30, 10, 90 ticks escolha de protótipo | GDD §10.1:519-523; contrato da sessão; `intake.md` |
| FR-002 | `folego`, `folego_ticks`, `combate_ticks` no `Jogador`, cobertos por `hash_estado` | contrato; `define.md` |
| FR-003 | efeito de `atacar_forte`: validação de Fôlego (30) e alvo no ponto/alcance (2 m) | contrato; `shape.md` |
| FR-004 | dano 10 com `saturating_sub`, débito atômico, um ataque por jogador por tick via `BTreeSet<u64>` | contrato; `shape.md` |
| FR-005 | regeneração em `fase_finish` (+1/3 ticks e +1/6 ticks) e janela de 90 ticks | contrato; `define.md` |
| FR-006 | `CodigoRecusa::FolegoInsuficiente`/`AlvoInvalido`, tecla `Q`, HUD `Fôlego`/`Vida`, fronteira inalterada | contrato; `intake.md` |
| SC-001 | ataque aceito debita 30 e aplica 10 de dano sem valor negativo | `define.md` · Success Metrics 1 |
| SC-002 | Fôlego < 30 recusa sem dano e sem débito parcial | `define.md` · Success Metrics 2 |
| SC-003 | alvo fora do ponto/alcance (2 m) recusa com `AlvoInvalido`, sem consumo | `define.md` · Success Metrics 3 |
| SC-004 | no máximo 1 ataque por jogador por tick, sem duplicatas | `define.md` · Success Metrics 4 |
| SC-005 | regeneração +1/3 ticks fora de combate e +1/6 ticks em combate; janela de 90 ticks | `define.md` · Success Metrics 5 |
| SC-006 | campos no `hash_estado`, tecla `Q`, HUD `Fôlego`/`Vida`, fronteira sem contrato/protocolo/rede | `define.md` · Success Metrics 6 |
| T001 | constantes documentadas e rotuladas | `shape.md` · Opção A |
| T002 | campos `folego`/`folego_ticks`/`combate_ticks` no `Jogador` | `shape.md` · Opção A |
| T003 | cobertura integral em `hash_estado` em ordem estável | `shape.md` · Opção A |
| T004 | efeito de `atacar_forte` em `ACOES_PERMITIDAS` | `shape.md` · Opção A |
| T005 | validação de alvo no ponto/alcance e `AlvoInvalido` | `shape.md` · Opção A |
| T006 | validação de Fôlego e `FolegoInsuficiente` | `shape.md` · Opção A |
| T007 | débito atômico e dano `saturating_sub` | `shape.md` · Opção A |
| T008 | `BTreeSet<u64>`: um ataque por jogador por tick | `shape.md` · Opção A |
| T009 | `combate_ticks = 90` e transição de combate | `shape.md` · Opção A |
| T010 | regeneração acumulada em `fase_finish` (3/6 ticks) | `shape.md` · Opção A |
| T011 | cliente: tecla `Q` e ids monotônicos | `shape.md` · Opção A |
| T012 | HUD `Fôlego`/`Vida` e verificações de dano, recusa, tick e hash | `shape.md` · Assumptions |

Os nomes definitivos de T001..T012 serão os de `specs/047-combate-folego/tasks.md` (o diretório continha apenas o template `spec.md` no momento da leitura); a coluna "Eco" apenas rastreia a cobertura deste assessment. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa. Todo valor de combate citado é `[HIPÓTESE]`/`escolha de protótipo`; nada aqui é especificação, plano ou implementação.

Veredito: go
