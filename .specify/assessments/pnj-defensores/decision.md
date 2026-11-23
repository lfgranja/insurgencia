# Decision — PNJs defensores com PV e dano determinístico

- **Slug**: `pnj-defensores`
- **Decided**: 2026-11-23T15:53Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | O GDD §8.1:361 fixa 4 forças de segurança e 8 dmg/s por 3 s = 24 PV como `[HIPÓTESE]`, e §8.4/§8.5 descreve patrulhas/PNJs; `Mundo` não tem `pnjs`, não há `Pnj`, `adicionar_pnj`, `aplicar_dano_a_pnj` ou `atacar_pnj` em `ACOES_PERMITIDAS`, e a HUD não conta defensores — a oposição PVE existe só como parágrafo. |
| Força da evidência | strong | GDD §8.1:357,361,363 e §8.4/§8.5 lidos com os valores rotulados `[HIPÓTESE]`; `STATE.md` (15:53Z) fixa o 80/20 em 48/625, o empate com novas habilidades W/E/R e os 10/625 da rede; código confirmado por leitura (`PASSO_SEGUNDOS`, `DANO_ATAQUE`, `ALCANCE_CONTATO_M`, `TICKS_COMBATE`, `ACOES_PERMITIDAS`, `nivel_minimo_para_acao`, `Jogador.funcao/nivel`, `Mundo`, `aplicar_dano_a_*`, `avancar_tick`, `hash_estado`, HUD e `solicitar_ataque`). |
| Valor vs. inação | strong | **48/625**, empatado com novas habilidades W/E/R (48/625) e à frente de rede (10/625); sem o recorte, o único adversário descrito no GDD segue sem PV observável, sem relógio de dano e sem recipiente no cliente — e a fila de frentes adiadas desde o ciclo 049 nunca fecha. |
| Viabilidade / appetite | strong | `small`: struct de quatro campos, um `BTreeMap`, dois métodos, uma ação na lista/mapa de níveis, um braço no tick, uma fase de contra-ataque, cobertura no hash, uma tecla e uma linha de HUD; reaproveita constantes e padrões consolidados e não toca em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, Lightyear, `async` ou sockets. |
| Alinhamento estratégico | strong | Preserva 24 PV, 8 dmg, 90 ticks, 2 m e dano 10 como `[HIPÓTESE]`/protótipo; usa a infraestrutura de papéis/níveis do base `4975fd4` sem reabri-la; respeita a constituição v1.2.2 (R1 `go`, R4 antes de R7 `1.0.0-local`, R6 N/A) e ADR-004; novas habilidades W/E/R e escalonamento 4→8/×5 fora de escopo. |
| Postura de risco | strong | Riscos nomeados e mitigados por decisão: constantes explícitas, `BTreeMap` ordenado, guardas de dano e clamp de PV, `ativo = false` sem remoção do id, dedup por atacante/tick, contador por PNJ, ordem crescente de `id` no tick, hash posicional, tecla `T` sem novo campo de protocolo; empate 48/625 desempatado pelo contrato e pelo lastro do §8.1; R6 N/A sem issue simulada. |

## 80/20

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| PNJs defensores (PV + dano determinístico, escolhida) | 4×2×2×3 = 48/625 | **48/625** |
| Novas habilidades W/E/R | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**. Fonte autoritativa: `.specify/memory/STATE.md`, entrada `2026-11-23T15:53Z`.

PNJs defensores vence porque é a única opção com **valores escritos no GDD e fronteira mínima simultaneamente**: §8.1:361 já traz 4 forças, 8 dmg/s por 3 s e os 24 pontos, e o contrato da sessão fixou a tradução (24 PV, ataque de 8 a cada 90 ticks, 2 m, dano 10), a API e o cliente, sem mapa, ilhas ou rede. Confiança 2 e facilidade 2 registram honestamente que os números são `[HIPÓTESE]` e que a entidade não existe ainda; por isso o recorte reaproveita `DANO_ATAQUE`, `ALCANCE_CONTATO_M`, `TICKS_COMBATE`, `aplicar_dano_a_*`, `avancar_tick` e `hash_estado`. O empate com novas habilidades W/E/R (48/625) é resolvido pelo contrato e pelo `STATE.md` ("novas habilidades W/E/R ficam para o próximo ciclo"): o adversário tem lastro documental, enquanto Oficina de Campo, Análise Materialista, Plano Mestre etc. dependem de Fôlego, áreas, canalização e NPCs ainda não fechados. A rede inverte a dependência entre determinismo e sincronização e permanece bloqueada por ADR-004.

## Veredito e justificativa

**Veredito: go** — Opção A (defensor determinístico com PV, dano e relógio no Núcleo), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 (`1.0.0-local`), R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, formatação, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. Regra congelada **`[HIPÓTESE]`/protótipo**: `PV_PNJ_INICIAL = 24`, `DANO_PNJ = 8`, `TICKS_ATAQUE_PNJ = 90` (3 s a 30 Hz) e alcance de **2 m**; dano do jogador **10** reaproveitado de `DANO_ATAQUE`. Nenhum número é promovido a cânone; 4→8 e ×5 do GDD §8.1 não são reabertos.
2. Entidade e estado: `struct Pnj { id, posicao, pv, ativo }` e `Mundo.pnjs: BTreeMap<u64, Pnj>` em ordem estável de `id`; `adicionar_pnj(id, posicao)` registra defensor ativo com 24 PV.
3. Dano recebido: `aplicar_dano_a_pnj(id, dano)` ignora `dano <= 0`, `id` inexistente e PNJ inativo; clampa PV em zero e marca `ativo = false` em PV ≤ 0 **sem remover a entrada do mapa**.
4. Ação do jogador: `atacar_pnj` em `ACOES_PERMITIDAS`, nível **1**, com `destino` na posição do PNJ, `ponto_igual`, `ALCANCE_CONTATO_M = 2.0` e no máximo um ataque por atacante por tick; reaproveita a validação de nível do ciclo 052.
5. Ataque do defensor: PNJ ativo devolve **8** a cada **90 ticks** ao jogador a ≤ 2 m, com contador por PNJ, iteração em ordem crescente de `id` e nenhum relógio de parede (`Instant`/`SystemTime`).
6. Determinismo: `Mundo.pnjs` (incluindo `pv` e `ativo`) entra em `hash_estado` em posição fixa, serializado de forma independente de formatação de texto.
7. Cliente: HUD `PNJs: N` com a contagem de ativos e tecla `T` que agenda `atacar_pnj` com `seq`/`request_id` incrementados e `destino` na posição do PNJ; nenhuma habilidade W/E/R nova, nenhum novo campo de comando ou protocolo.
8. Compatibilidade e estilo: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, dependências ou `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem `async`/rede/Lightyear.
9. Constituição v1.2.2 aplicada: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Hipóteses do GDD §8.1**: 24 PV, 8 dmg, 90 ticks, 2 m e o dano 10 são chutes de design/protótipo, não cânone; rotular como tal em spec, código e HUD. Não reabrir o empate 48/625 com novas habilidades W/E/R nem o ×2/×5 da Rota Superior.
- **Confusão conceitual com o alvo de treino**: `PEER_ALVO_TREINO`, `ALVO_TREINO_POSICAO`, `AlvoTreino` e a linha `Alvo:` do ciclo 048 são scaffolding não-canônico de cliente; o PNJ é entidade determinística do Núcleo. Não fundir os dois nem reutilizar a linha `Alvo:` como contagem de defensores.
- **Hash/determinismo e relógio**: `hash_estado` e a ordem de `avancar_tick` são pontos sensíveis de replay; `pnjs` entra em posição fixa e o contra-ataque corre por `id`, com contador por PNJ (89 não dispara, 90 dispara uma vez). Permanece **[NV]** até execução.
- **Dupla contagem e PV negativo**: sem as guardas de `aplicar_dano_a_pnj` e a dedup de `atacar_pnj`, um comando repetido poderia debitar PV duas vezes no mesmo tick; mitigado por dano ≤ 0, id inexistente, inatividade, clamp e `BTreeSet<u64>` existente.
- **Inatividade vs. remoção**: PNJ com PV ≤ 0 fica `ativo = false` mas permanece no `BTreeMap`; remover a entrada apagaria a auditabilidade e faria o hash ignorar o desfecho do combate.
- **Escolha do destino no cliente**: o contrato exige "`T` ataca o PNJ no destino", mas não fixa o critério de seleção (mais próximo, menor `id` ou posição fixa); é decisão da spec, não lacuna deste assessment.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação**: gates, build, testes, formatação, R4 e R7 (`1.0.0-local`) permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: o GDD §8.1/§8.4/§8.5 descreve forças de segurança e dano como `[HIPÓTESE]`, mas o Núcleo não tem `Pnj`/`Mundo.pnjs`, não aceita `atacar_pnj`, não devolve dano do defensor e a HUD não conta ativos; a oposição PVE segue sem verdade observável.
- **Chosen approach**: Opção A — `Pnj { id, posicao, pv, ativo }`, `Mundo.pnjs: BTreeMap<u64, Pnj>`, `adicionar_pnj`, `aplicar_dano_a_pnj`, `atacar_pnj` nível 1 com `destino` na posição do PNJ, contra-ataque de 8 a cada 90 ticks em 2 m, `hash_estado` refletivo, HUD `PNJs: N` e tecla `T`.
- **In scope**: entidade, mapa, API de dano, ação e validação, fase de contra-ataque, hash, HUD e tecla. **Out of scope**: novas habilidades W/E/R (`[HIPÓTESE]` §10.2–§10.5), mapa/ilhas, 4→8/×5, escalonamento e gás, rebalanceamento de ações consolidadas, alvo de treino do ciclo 048, netcode, contratos/protocolo, R6 (N/A).
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: tipo/assinatura de `Pnj.posicao` e `adicionar_pnj`, contador por PNJ vs. `tick % 90`, critério de seleção do destino na tecla `T`, posição de `pnjs` no `hash_estado` e ordem de `atacar_pnj` entre as recusas — validar na especificação.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | `Pnj { id, posicao, pv, ativo }` e `Mundo.pnjs: BTreeMap<u64, Pnj>` | GDD §8.1:357,361; contrato da sessão |
| FR-002 | `adicionar_pnj(id, posicao)` com PV inicial 24 `[HIPÓTESE]` | GDD §8.1:361 (8 × 3 s = 24); contrato |
| FR-003 | `aplicar_dano_a_pnj`: guardas, clamp, `ativo = false` sem remoção; dano do jogador 10 | `crates/sim/src/lib.rs:521,560,579,113`; contrato |
| FR-004 | `atacar_pnj` nível 1 em `ACOES_PERMITIDAS`, `destino` na posição do PNJ, alcance 2 m e dedup | `crates/sim/src/lib.rs:137-148,158-165,116,691`; contrato |
| FR-005 | Contra-ataque de 8 a cada 90 ticks a jogador em 2 m; integração em `avancar_tick` e `hash_estado` | `crates/sim/src/lib.rs:15,119,368-403,601`; contrato |
| FR-006 | HUD `PNJs: N` ativos e tecla `T`; sem novas habilidades W/E/R, contrato, protocolo, Cargo, stack ou rede | `crates/cliente/src/main.rs:744-762,965-968`; contrato; `STATE.md` 15:53Z |
| SC-001 | PNJ com 24 PV começa ativo e aparece em `Mundo.pnjs`/`hash_estado` | `define.md` · Success Metrics 1 |
| SC-002 | `atacar_pnj` nível 1 reduz PV de 24 para 14; fora de 2 m não altera estado | `define.md` · Success Metrics 2 |
| SC-003 | Dano ≤ 0/id inexistente/inativo não mudam estado; PV clampado e `ativo = false` | `define.md` · Success Metrics 3 |
| SC-004 | 89 ticks não disparam; 90.º dispara 8 uma vez; fora de 2 m não há dano ao jogador | `define.md` · Success Metrics 4 |
| SC-005 | Um ataque por atacante/tick e um contra-ataque por PNJ/janela; hash estável | `define.md` · Success Metrics 5 |
| SC-006 | HUD `PNJs: N` e tecla `T` com `destino` na posição do PNJ; sem habilidades novas | `define.md` · Success Metrics 6 |
| T001 | `struct Pnj { id, posicao, pv, ativo }` | `shape.md` · Opção A |
| T002 | `Mundo.pnjs: BTreeMap<u64, Pnj>` em `new()` | `shape.md` · Opção A |
| T003 | Constantes `[HIPÓTESE]` `PV_PNJ_INICIAL = 24`, `DANO_PNJ = 8`, `TICKS_ATAQUE_PNJ = 90` | `shape.md` · Assumptions |
| T004 | `adicionar_pnj(id, posicao)` registrando defensor ativo com PV inicial | `shape.md` · Opção A |
| T005 | `aplicar_dano_a_pnj` com guardas, clamp e transição para inativo | `shape.md` · Opção A |
| T006 | `atacar_pnj` em `ACOES_PERMITIDAS` e nível 1 em `nivel_minimo_para_acao` | `shape.md` · Opção A |
| T007 | Processamento de `atacar_pnj` no tick: `destino`, `ponto_igual`, alcance e dedup | `shape.md` · Opção A |
| T008 | Fase de contra-ataque do PNJ: 8/90 ticks a jogador em 2 m, ordem por `id` | `shape.md` · Opção A |
| T009 | `hash_estado` cobrindo `Mundo.pnjs` em posição fixa | `shape.md` · Opção A |
| T010 | Cliente: tecla `T` agenda `atacar_pnj` com `destino` na posição do PNJ | `shape.md` · Opção A |
| T011 | HUD `PNJs: N` com contagem de ativos | `shape.md` · Opção A |
| T012 | Testes de 24→14, PV ≥ 0, inatividade, 89/90 ticks, alcance, dedup e hash | `shape.md` · Assumptions |

Os nomes definitivos de T001..T012 serão os de `specs/053-pnj-defensores/tasks.md` (o diretório hoje contém apenas o template `spec.md`); a coluna "Eco" apenas rastreia a cobertura deste assessment. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa. Nada aqui é especificação, plano ou implementação.

Veredito: go
