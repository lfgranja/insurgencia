# Shape — PNJs defensores com PV e dano determinístico

- **Slug**: `pnj-defensores`
- **Created**: 2026-11-23T15:53Z
- **Recommended option**: Opção A — Defensor determinístico com PV, dano e relógio no Núcleo

## Options

### Option A — Defensor determinístico com PV, dano e relógio no Núcleo (recomendada)

- **Sketch**: o Núcleo passa a registrar `Pnj { id, posicao, pv, ativo }` em `Mundo.pnjs: BTreeMap<u64, Pnj>`. `adicionar_pnj` cria o defensor ativo com **24 PV**; `aplicar_dano_a_pnj` debita o dano do jogador (**10**), clamp em zero e marca `ativo = false` ao esgotar os PV, mantendo a entrada para auditoria. A ação `atacar_pnj` entra em `ACOES_PERMITIDAS` no **nível 1**, usa `destino` na posição do PNJ, `ponto_igual`, `ALCANCE_CONTATO_M = 2.0` e a dedup de um ataque por atacante por tick. No mesmo `avancar_tick`, cada PNJ ativo devolve **8** a cada **90 ticks** (3 s a 30 Hz) ao jogador a ≤ 2 m, em ordem estável de `id`. `hash_estado` cobre `pnjs` com `pv` e `ativo`; o cliente mostra `PNJs: N` ativos e a tecla `T` agenda `atacar_pnj` com o `destino` na posição do defensor. Todos os números são **`[HIPÓTESE]`/protótipo**.
- **Appetite**: `small` (dias) — uma struct, um campo no `Mundo`, dois métodos de API, uma ação nova na lista/mapa de níveis, um braço no processamento de comandos, uma fase de contra-ataque no tick, cobertura no hash, uma tecla e uma linha de HUD; sem `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, mapa, rede ou dependência nova.
- **Trade-offs**: ganha a oposição PVE do GDD §8.1/§8.4/§8.5 convertida em estado observável, com dano auditável e relógio determinístico, reaproveitando `DANO_ATAQUE`, `ALCANCE_CONTATO_M`, `TICKS_COMBATE`, o padrão `aplicar_dano_a_*` e a infraestrutura de nível do ciclo 052. Sacrifica mapa, ilhas, 4→8 defensores por aliado, escalonamento ×5 e as habilidades completas W/E/R — a parte `[HIPÓTESE]` mais custosa. Riscos: confiança 2 (todos os números são chutes de design), facilidade 2 (a entidade e a ação ainda não existem) e risco invertido 3 (tick, hash, validação e HUD exigem verificação); mitigados por constantes explícitas, `BTreeMap` ordenado, guardas de dano, clamp de PV, dedup por atacante e contador por PNJ.
- **Rabbit holes**: IA de patrulha, aggro, pathfinding, spawn por ilha, 4→8 ou ×5, gás lacrimogêneo contínuo, tela de combate, picking de alvo, tabelas de resistência, rebalancear Fôlego/alcance/dano das ações consolidadas, confundir o PNJ com o `AlvoTreino` não-canônico do ciclo 048, ou transformar `atacar_pnj` em habilidade W/E/R.

### Option B — Novas habilidades W/E/R primeiro

- **Sketch**: investir a fatia em Oficina de Campo, Análise Materialista, Plano Mestre, Fuga pelas Sombras, Espalhar Rumores ou Sabotagem Estrutural (GDD §10.2–§10.5), deixando o PVE para depois.
- **Appetite**: `medium` (semanas) — puxa Fôlego, alcance, recarga, canalização, áreas e efeitos por papel para dentro do recorte.
- **Trade-offs**: empata com A no scorecard (4×2×2×3 = 48/625), mas é a frente que o `STATE.md` (15:53Z) empurra explicitamente para o próximo ciclo; seus números são `[HIPÓTESE]` e não há definição fechada de custo, duração ou efeito. O contrato da sessão escolheu `pnj-defensores`.
- **Rabbit holes**: balanceamento por função, sinergias Q/W/E/R, efeitos de área, canalização interrompível — nenhum com valor canônico para este ciclo.

### Option C — Rede / Lightyear primeira

- **Sketch**: postergar a mecânica até transporte, predição e reconciliação cliente-servidor estarem prontos.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de fatia de Núcleo para infraestrutura de rede.
- **Trade-offs**: impacto 5, mas facilidade 1 e risco invertido 1 → **10/625**; ADR-004 bloqueia a troca de stack até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`). Sincronizaria um defensor cujo determinismo local ainda não foi fechado — dependência invertida.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), predição de dano, resolução de concorrência de comandos, troca de engine sem novo ADR.

## Recommendation

Escolher **Opção A**. Ela satisfaz as metas de `define.md` com a menor fronteira: impacto 4 (a penalidade `[HIPÓTESE]` do GDD vira PV observável e dano com relógio), confiança 2 (24, 8, 90, 2 m e 10 são chutes rotulados), facilidade 2 (não existe `Pnj`, `Mundo.pnjs` ou `atacar_pnj`; as constantes e os padrões já existem) e risco invertido 3 (tick, hash, validação e HUD exigem verificação). O empate com B em 48/625 é desempatado pelo contrato da sessão e pelo lastro documental: o GDD já escreveu valores para o adversário (§8.1:361), o `STATE.md` diz que W/E/R ficam para o próximo ciclo e a fila de frentes adiou PVE/PNJs desde o ciclo 049. C inverte a dependência determinismo → sincronização e está bloqueada por ADR-004. 80/20: **48/625** contra 48/625 (B) e 10/625 (C).

A recomendação **não** propõe mapa, ilhas, escalonamento 4→8/×5 nem habilidades W/E/R: o recorte é um defensor determinístico, não o sistema de oposição completo.

## Out of Scope (for the recommended option)

- `[HIPÓTESE]` GDD §10.2–§10.5: habilidades completas Q/W/E/R (Oficina de Campo, Análise Materialista, Plano Mestre, Fuga pelas Sombras, Espalhar Rumores, Sabotagem Estrutural etc.) e seus valores de Fôlego, alcance, recarga, custo e duração.
- Mapa, rotas, ilhas, spawn, patrulha com navegação, aggro, 4→8 defensores por aliado, variante de estresse ×5 e penalidade de gás lacrimogêneo (§8.1/§8.2/§8.4/§8.5).
- Reabrir papéis/níveis do ciclo 052 (`Funcao`, `nivel`, 900 ticks, teto 12, Q1/W2/E3/R6) ou rebalancear dano/custo/PV/Fôlego/alcance das ações consolidadas.
- O `AlvoTreino` não-canônico do ciclo 048 (`PEER_ALVO_TREINO = 2`, `ALVO_TREINO_POSICAO`, HUD `Alvo:`) — permanece como está, sem virar PNJ.
- Novos campos de comando/protocolo, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`.
- R6 (N/A por constituição v1.2.2): nenhuma issue simulada, nenhum `gh` improvisado.

## Assumptions to Validate

- `Pnj.posicao` é `(f64, f64)`, o mesmo tipo de `Jogador.posicao` e de `Comando.destino`; `adicionar_pnj(id, posicao)` inicializa `pv = 24` e `ativo = true` — **`[HIPÓTESE]`** de protótipo, confirmar no `/speckit.specify`.
- `PV_PNJ_INICIAL = 24`, `DANO_PNJ = 8` e `TICKS_ATAQUE_PNJ = 90` são novas constantes **`[HIPÓTESE]`**; `DANO_ATAQUE = 10`, `ALCANCE_CONTATO_M = 2.0` e `PASSO_SEGUNDOS = 1/30` são reaproveitadas, sem valores paralelos divergentes.
- A janela de contra-ataque é contada **por PNJ** (campo de ticks na entidade), não por `tick % 90` global, para que defensores registrados em ticks diferentes tenham fases independentes; confirmar na spec.
- `atacar_pnj` é nível 1: entra em `ACOES_PERMITIDAS` e resolve via default de `nivel_minimo_para_acao` ou entrada explícita — uma única fonte de verdade, sem duplicar a tabela do ciclo 052.
- Dano ao PNJ: `dano <= 0`, `id` ausente ou `ativo == false` não alteram estado; PV é clampado em zero e `ativo` passa a `false` em PV ≤ 0, sem remover a entrada do `BTreeMap`.
- Ataque ao jogador: apenas `ativo == true`, distância ≤ 2 m (euclidiana, sem raiz para comparação) e um disparo por janela de 90 ticks; a iteração segue ordem crescente de `id`.
- `hash_estado` inclui `pnjs` em posição fixa, serializando `id`, `posicao`, `pv` e `ativo` de forma estável e independente de formatação de texto.
- A tecla `T` agenda `atacar_pnj` com `seq`/`request_id` incrementados como em `solicitar_ataque` (Q); o critério de escolha do defensor (mais próximo, menor `id` ou posição fixa) é decisão da spec, não deste assessment.
- HUD `PNJs: N` conta apenas `ativo == true`; sem nova tela, picking ou captura visual prometida.
- Estilo obrigatório: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, sem `async`/rede/Lightyear.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; B e C são as alternativas de `research.md` e não recebem rastro de implementação. T001..T012 pertencem ao handoff em `decision.md`. Todos os números (24, 8, 90, 2 m, 10) são registrados como `[HIPÓTESE]`/protótipo, não como pendências de design; **R6 é N/A** por constituição v1.2.2.
