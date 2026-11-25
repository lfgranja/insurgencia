# Shape — Mover validado no cliente

- **Slug**: `mover-validado`
- **Created**: 2026-11-24
- **Recommended option**: Opção A — `mover` validado no cliente (BF-012)

## Options

### Option A — `mover` validado no cliente (recomendada)

- **Sketch**: o WASD/setas deixa de escrever `JogadorLocal` diretamente. O passo em pixels é convertido por `pixels_para_metros` (`PIXELS_POR_METRO = 100.0`, `[HIPÓTESE]`) e origina `Comando { acao: "mover", destino: metros }`; `ValidacaoComandos::validar` decide o aceite **antes** de `agendar_comando`, e `seq`/`request_id` só avançam no aceite. `sincronizar_sim` passa a ler `Mundo.jogadores[PEER_ID].posicao` e escrever `JogadorLocal` em pixels; a `.chain()` do `Update` vira `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`. Os demais helpers de destino/alvo também passam a operar em metros via `pixels_para_metros`.
- **Appetite**: `small`/`medium` (dias) — cliente + helpers de conversão + reordenação de sistemas + testes; sem contrato, protocolo, Cargo, CI, stack ou rede novos.
- **Trade-offs**: impacto **4**, confiança **3**, fechamento **3**, esforço⁻¹ **2**, risco invertido **2** → **144/3125**. Ganha paridade cliente/Núcleo, fim do unit mismatch e recusa sem consumo de contador; sacrifica a ilusão de movimento instantâneo (o avatar passa a depender do aceite da sim) e exige disciplina de conversão em todo helper. Riscos nomeados: dupla aplicação de movimento e ordem de sincronização — ambos fechados por decisão (DoD-1/DoD-3).
- **Rabbit holes**: reescrever `LIMITE_MOVIMENTO_M` ou o validador para "acomodar" o cliente; predição/reconciliação de rede; duplicar a posição em dois estados sem fonte única; mover a conversão para `crates/contratos` (é BF-013, fora).

### Option B — Mapa/rotas de 12 ilhas (BF-003b)

- **Sketch**: usar a fatia para fechar topologia e spawning do mapa completo de 12 ilhas.
- **Appetite**: `large` (semanas) — depende de decisão canônica de topologia no GDD.
- **Trade-offs**: o **270/3125** histórico inclui o BF-003a já fechado no ciclo 058; o restante não é elegível neste ciclo (`STATE.md:586`). Não resolve o bypass de `mover` nem o unit mismatch.
- **Rabbit holes**: reabrir o GDD, desenhar rotas, spawning e reconquista — fora do contrato 059.

### Option C — HUD republica `hash_estado` (BF-014)

- **Sketch**: acrescentar ao painel do cliente o hash canônico corrente, para diagnóstico determinístico.
- **Appetite**: `small` (horas) — uma linha de HUD, nenhuma mudança na sim.
- **Trade-offs**: impacto **2**, confiança **5**, fechamento **1**, esforço⁻¹ **5**, risco invertido **5** → **250/3125**. Eliminada em `STATE.md:586`: exibir hash não muda o jogo e não torna o movimento autoritativo.
- **Rabbit holes**: poluir `texto_hud`, confundir diagnóstico com jogabilidade, prometer captura visual.

### Option D — `contratos` consumido por binários (BF-013)

- **Sketch**: ligar `crates/contratos`, hoje desconectado, ao cliente e ao servidor para compartilhar tipos e constantes.
- **Appetite**: `large` — mexe em `PROTOCOL_VERSION` e na fronteira de módulo.
- **Trade-offs**: impacto **5**, confiança **2**, fechamento **5**, esforço⁻¹ **1**, risco invertido **2** → **100/3125**. Eliminada: diagnóstico, sem comportamento observável para o jogador; permanece aberta por DoD-5.
- **Rabbit holes**: serialização compartilhada, compatibilidade do formato `<tick>\t<sha256>`, ADR-004.

## Recommendation

Escolher **Opção A**. É a única que satisfaz simultaneamente as metas de `define.md`: comportamento observável para o jogador do cliente Bevy e para a validação da sim/CLI (impacto 4), autoridade já lida no código — limite de 1 m e validador público (confiança 3), consumidor e comandos de prova nomeados (fechamento 3), com fronteira pequena sem contrato/protocolo/Cargo/CI/stack/rede (esforço⁻¹ 2 e risco⁻¹ 2). B depende de decisão canônica e não toca no defeito; C e D são diagnósticos eliminados pelo próprio scorecard do ciclo. 80/20: **144/3125** contra 250/3125 (C, eliminada), 100/3125 (D, eliminada) e BF-003b não elegível. O BF-012 já fora pontuado em 240/3125 em ciclo anterior; o ciclo 059 adota **144/3125** (`4×3×3×2×2`) como pontuação única, sem renegociar as demais.

## Out of Scope (for the recommended option)

- Mapa de 12 ilhas/topologia/spawning (BF-003b) e consumo de `crates/contratos`/`PROTOCOL_VERSION` (BF-013) — ambos abertos.
- HUD com hash (BF-014), rede real/Lightyear, predição, reconciliação, lobby e novas habilidades.
- Mudança em `LIMITE_MOVIMENTO_M`, no validador da sim ou em qualquer número canônico do GDD; promoção de `PIXELS_POR_METRO = 100.0` a regra canônica.
- R6/taskstoissues (N/A por v1.2.2), emissão simulada de issues, PR #76 (outro owner), `dev→main`/deploy (SWE-exclusivos) e qualquer execução de gates/build/testes/formatadores neste assessment.

## Assumptions to Validate

- `ValidacaoComandos` é público em `crates/sim/src/lib.rs:2277` e `validar` aceita o `Mundo` do espelho sem nova API — confirmar na spec 059.
- A origem usada pelo validador é a posição corrente do espelho (`jogador.posicao`), coerente com o `destino` derivado do estado local convertido — confirmar que não há defasagem de um tick entre `mover_jogador` e `avancar_simulacao`.
- `PASSO_SEGUNDOS` é 1/30: 2,6 m/s ÷ 30 ≈ 0,087 m/tick, abaixo de 1 m — confirmar no `/speckit.specify`; arredondamento na fronteira fica **[NV]**.
- A reordenação do `.chain()` não quebra `atualizar_texto` nem as ações com tecla `just_pressed` — validar na especificação.
- Imobilização (`imobilizado_ticks > 0`) continua bloqueando movimento tanto no cliente quanto na sim, sem agendar comando.

## Rastreabilidade

Opção A ↔ **FR-001** (conversão `PIXELS_POR_METRO`/`pixels_para_metros`), **FR-002** (`mover` validado antes de agendar, contadores só no aceite), **FR-003** (`sincronizar_sim` derivado do espelho + reordenação `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`), **FR-004** (evidência/validação, **[NV]** aqui). Alternativas B/C/D são as mesmas rejeitadas em `research.md` e não recebem rastro de implementação. DoD-1..DoD-5 e o score **144/3125** são idênticos nos cinco artefatos; SC/T pertencem ao handoff em `decision.md`, com definição canônica na spec 059.
