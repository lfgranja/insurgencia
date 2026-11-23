# Shape — Súditos e Propaganda

- **Slug**: `suditos`
- **Created**: 2026-11-23T06:58Z
- **Recommended option**: Opção A — Conversão determinística de Súdito

## Options

### Option A — Conversão determinística de Súdito (recomendada)

- **Sketch**: o Núcleo mantém súditos injetados por `adicionar_sudito(id, posicao)` numa tabela ordenada. Um comando `propaganda` informa, por `BTreeSet<u64>`, quais súditos inimigos foram expostos; a cada tick, cada id avança **no máximo uma vez** seu contador de exposição. Ao completar **120 ticks (4 s)**, o súdito para de atacar estruturas e passa a panfletar (`convertido = true`). Se o jogador aplica dano (`aplicar_dano_a_sudito(id)`), um relógio de **90 ticks (3 s)** corre e, ao completar, o súdito volta a atacar (`convertido = false`). Todo o estado é coberto por `hash_estado`; o cliente usa a tecla `G` e o HUD mostra quantos estão convertidos.
- **Appetite**: `small` (dias) — uma ação em `ACOES_PERMITIDAS`, uma struct pública em `BTreeMap<u64, Sudito>`, duas APIs de injeção/dano, um comando de auto-avanço, hash e HUD; sem `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, rede ou dependência nova.
- **Trade-offs**: ganha a única regra de Súditos com critério de aceitação escrito (GDD §9.2:464) convertida em comportamento observável e auditável por hash. Sacrifica spawn, raio e funções — justamente a parte `[HIPÓTESE]` do §9.2:461 — e toda a cadeia de Nós/Resistência Ideológica (§6.2:275). Risco principal: confiança 2 e facilidade 2 refletem a ausência do sistema de Súditos no Núcleo e a disciplina de tick necessária; mitigados por injeção determinística de ids, `BTreeSet` sem duplicatas e ordem estável do `BTreeMap`.
- **Rabbit holes**: implementar spawn a cada 30 s ou contagem por ilha; medir raio de 6 m e aplicar geometria; criar papéis trabalhador/propagandista/miliciano com renda e 60 s de permanência; ligar conversão a Nós ou Resistência Ideológica; transformar `propaganda` em pacote de protocolo; duplicar avanço de contador quando o mesmo id aparece duas vezes no comando.

### Option B — Combate/Fôlego primeiro

- **Sketch**: investir a fatia em combate (tiros, dano, Fôlego), deixando a conversão ideológica para depois.
- **Appetite**: `medium` (semanas) — puxa IA, física de projéteis e balanceamento para dentro do recorte.
- **Trade-offs**: empata com A no scorecard (4×2×2×3 = 48/625), mas Fôlego, alcance, recarga e dimensões são `[HIPÓTESE]` (GDD §10.3:574-579) e o contrato da sessão escolheu `suditos`. GDD §16.3:1509 é explícito: o MVP prova jogabilidade por desafios de ilha, "em vez de combate puro".
- **Rabbit holes**: balanceamento de armas, regeneração de Fôlego, triangulação de IA inimiga — nenhum com valor canônico para este ciclo.

### Option C — Rede / Lightyear primeira

- **Sketch**: postergar a mecânica até transporte, predição e reconciliação cliente-servidor estarem prontos.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de fatia de Núcleo para infraestrutura de rede.
- **Trade-offs**: impacto 5, mas facilidade 1 e risco invertido 1 → 10/625; ADR-004 bloqueia a troca até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`). Sincronizaria um estado que ainda não é determinístico.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), predição de conversão, resolução de concorrência de comandos, troca de engine sem novo ADR.

## Recommendation

Escolher **Opção A**. Ela satisfaz as metas de `define.md` com a menor fronteira: impacto 4 (a regra §9.2:464 vira estado verificável por `hash_estado`), confiança 2 (só a regra é canônica; spawn/raio/funções são hipóteses explicitamente excluídas), facilidade 2 (não existe sistema de Súditos no Núcleo; a injeção determinística substitui o spawn) e risco invertido 3 (disciplina de tick e ordem do hash exigem verificação). Empate com B em 48/625 é desempatado pelo contrato da sessão e pelo lastro canônico de A; C inverte a dependência determinismo → sincronização e está bloqueada por ADR-004. 80/20: **48/625** contra 48/625 (B) e 10/625 (C).

A recomendação **não** propõe implementar os sistemas hipotéticos: o recorte é a regra de aceitação, não o ecossistema em volta dela.

## Out of Scope (for the recommended option)

- `[HIPÓTESE]` GDD §9.2:461: spawn por pulso de 30 s, raio de 6 m, limite de 3 Súditos, funções trabalhador/propagandista/miliciano, conservação de função, permanência de 60 s e renda do trabalhador.
- `[HIPÓTESE]` GDD §6.2:275: conversão de Nós e Resistência Ideológica; Propaganda canalizada de 20 s; mapa, ilhas e Rota Central.
- Combate/Fôlego, IA, projéteis, colisão, dano de armas; netcode, lobby, matchmaking, `async`, sockets, Lightyear.
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`.
- R6 (N/A por constituição v1.2.2): nenhuma issue simulada, nenhum `gh` improvisado.

## Assumptions to Validate

- O relógio corrente do Núcleo é 30 ticks/s: 4 s = 120 ticks e 3 s = 90 ticks — confirmar no `/speckit.specify`.
- `propaganda` cabe em `ACOES_PERMITIDAS` e o comando com `BTreeSet<u64>` transporta os ids expostos sem estender `crates/contratos` ou `PROTOCOL_VERSION`.
- `adicionar_sudito(id, posicao)` é a via de injeção determinística e `aplicar_dano_a_sudito(id)` é a via de dano, sem IA ou geometria de raio neste recorte.
- Um único avanço por id por tick (dedup natural do `BTreeSet`) basta para a disciplina de tick; ids repetidos não creditam duas vezes.
- `BTreeMap<u64, Sudito>` com ids monotônicos preserva a ordem de `hash_estado` entre execuções.
- Conversão e reversão são estados binários em `Sudito.convertido`; `posicao` é armazenada e hashada, mas não usada para raio/area nesta fatia.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; B e C são as alternativas de `research.md` e não recebem rastro de implementação. T001..T012 pertencem ao handoff em `decision.md`; os `[HIPÓTESE]` acima são registrados como fora de escopo, não como pendências de design.
