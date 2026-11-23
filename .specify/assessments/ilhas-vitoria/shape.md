# Shape — Controle de ilha por Súdito (subset de vitória)

- **Slug**: `ilhas-vitoria`
- **Created**: 2026-11-23T11:12Z
- **Recommended option**: Opção A — Controle de ilha por Súdito (10 s contínuos, presença global documentada)

## Options

### Option A — Controle de ilha por Súdito (recomendada)

- **Sketch**: em `crates/sim/src/lib.rs`, declarar `Ilha { id, controlada, controle_ticks }`, `Mundo.ilhas: BTreeMap<u64, Ilha>` com `adicionar_ilha(id)` e `Mundo.sinais: VecDeque<String>`. Inserir `fase_controle_ilhas()` em `avancar_tick`, entre `fase_defesa()` e `fase_finish()`: se `suditos.values().any(|s| s.convertido)`, cada ilha incrementa `controle_ticks` e, ao atingir **300** (10 s a 30 Hz), marca `controlada = true`; se não há Súdito convertido, `controle_ticks = 0` e `controlada = false` (Latente), enfileirando `ilha_perdida` uma vez quando a ilha estava controlada. `hash_estado` passa a serializar ilhas e sinais em ordem estável; a HUD mostra contagem de ilhas controladas e os sinais. **Sem mapa nem rede: a presença de qualquer Súdito convertido é global** — simplificação explicitamente documentada; o Súdito é proxy da condição "Desafio de Ilha concluído" de §9.4, que não é modelada separadamente.
- **Appetite**: `small`/`medium` (poucos dias) — um struct, dois campos em `Mundo`, uma fase no tick, um acumulador com precedente em `fase_defesa`, uma fila de sinais, duas projeções (hash e HUD); sem contrato, protocolo, Cargo, stack, rede, topologia de ilhas ou Nós.
- **Trade-offs**: ganha a primeira condição de vitória verificável da sim, com regra **canônica** (não hipotética) e padrão determinístico já usado por barricadas/coleta; paga o preço de uma abstração de protótipo (presença global + proxy do Desafio) que precisa ficar rotulada para não virar dogma. Impacto **4** (primeira fatia de vitória observável), confiança **3** (regra e acumulador conhecidos, mas semântica de presença global, emissão de sinal e hash precisam de validação), facilidade **3** (poucos pontos, porém cruzando tick, hash e HUD) e risco invertido **3** (fronteira de sim intacta e determinismo preservado, mas simplificação e sinais exigem disciplina) → **108/625**.
- **Rabbit holes**: inventar raio/posição de Súdito por ilha, mapa, rotas ou servidor central com dedupe de 2 s (hipótese §9.1:439); implementar Nós de Controle, Resistência Ideológica, papéis W/E/R ou tela de resultado; enfileirar `ilha_perdida` a cada tick sem presença; acumular `controle_ticks` intermitentemente em vez de zerar; esquecer ilhas/sinais no `hash_estado` e quebrar o replay; confundir o proxy do protótipo com o texto canônico de §9.4; tocar em contratos, protocolo, Cargo, stack ou rede.

### Option B — Nós de Controle + vitória completa

- **Sketch**: modelar os 3 Nós, a Resistência Ideológica e a condição final de vitória da Ilha Central, incluindo desbloqueios de rotas (§9.4:491-494).
- **Appetite**: `large` (semanas) — exige estado de Nó, topologia de rotas e o próprio estado de ilha que o Option A ainda vai introduzir.
- **Trade-offs**: impacto 4, mas confiança 2 e facilidade 2 (Nós e Ilha Central não têm estado na sim e dependem de mapa/rotas), risco invertido 3 → **48/625**. É a frente natural seguinte, porém pressupõe o trabalho desta opção e abre escopo muito maior que o subset de controle.
- **Rabbit holes**: Resistência Ideológica, conversão dos 3 Nós, regras de desbloqueio por rota, tela de resultado, "combate puro" fora do MVP.

### Option C — Papéis W/E/R + níveis

- **Sketch**: introduzir papéis (Intelectual, Artesão, Guerreiro, Camponês, Agitador) com níveis e habilidades W/E/R como novo contrato de ações.
- **Appetite**: `large` (semanas) — novo contrato de comandos, escalonamento e integração com Fôlego/Materiais.
- **Trade-offs**: impacto 4, confiança 2, facilidade 2, risco invertido 3 → **48/625**. Apesar de usar sistemas já implementados, não endereça a regra §9.4 e multiplicaria frentes de regra sem fechar a vitória.
- **Rabbit holes**: novas ações em `ACOES_PERMITIDAS`, curva de níveis, balanceamento de W/E/R, sincronia com papéis no cliente.

### Option D — Rede / Lightyear primeira

- **Sketch**: postergar o estado local até transporte, predição e reconciliação estarem prontos, com ilhas sincronizadas entre peers.
- **Appetite**: `large` (meses) — transforma modelagem de regra em infraestrutura de rede.
- **Trade-offs**: impacto 5, mas facilidade 1 e risco invertido 1 → **10/625**; ADR-004 bloqueia a troca até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`). Sincronizaria um estado de ilha que ainda não existe localmente.
- **Rabbit holes**: lobby/matchmaking, resolução de concorrência de sinais, troca de engine sem novo ADR.

## Recommendation

Escolher **Opção A**. É a única que satisfaz as metas de `define.md` com o maior valor do scorecard (**108/625** contra 48/625 de B e C e 10/625 de D) e com fronteira mínima: impacto **4** (a regra canônica de vitória passa a existir como estado verificável), confiança **3** (GDD §9.4 e o acumulador de 300 ticks de `fase_defesa` são conhecidos, mas presença global, sinal e hash precisam de validação), facilidade **3** (poucos pontos de mudança, ainda que em tick, hash e HUD) e risco invertido **3** (sim permanece determinística; contratos, protocolo, Cargo, stack e rede intactos). B e C dependem de estado de mapa/Nós/papéis que ainda não existe; D inverte a dependência regra local → sincronização e está bloqueada por ADR-004.

A recomendação **não** propõe criar topologia de ilhas, promover a simplificação a cânone nem reabrir hipóteses do GDD (dedupe de 2 s em §9.1:439, reconversão em 20 s em §9.4:494). A verificação é estrutural e determinística: leitura de código, comparação de `hash_estado` e inspeção de HUD; saídas não produzidas neste assessment permanecem **[NV]**.

## Out of Scope (for the recommended option)

- Nós de Controle, Resistência Ideológica, Ilha Central, desbloqueio de rotas e vitória completa; papéis W/E/R, níveis e novas habilidades.
- Geometria de ilha, posição/raio de Súdito por ilha, mapa, topologia, servidor central ou dedupe de sinais em 2 s (`[HIPÓTESE]` §9.1:439); promoção da presença global a regra canônica.
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack, rede; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100` — estilo existente, preservado.
- Mudanças em `AGENTS.md`, `STATE.md`, `.specify/queue/concurrency.jsonl` ou qualquer código-fonte por este assessment; **R6** (N/A por constituição v1.2.2), issues simuladas e `gh` improvisado; gates, build, testes e formatadores neste assessment.

## Assumptions to Validate

- `adicionar_ilha(id)` e o campo `Mundo.ilhas: BTreeMap<u64, Ilha>` são acrescentados sem alterar a assinatura pública de `avancar_tick` nem o formato do contrato; a spec confirma quantas ilhas o protótipo registra.
- A checagem `suditos.values().any(|s| s.convertido)` reflete o estado **do mesmo tick**, pois conversão/reversão ocorreram em fases anteriores de `avancar_tick` (858-865 e 344-345) — validar na leitura da spec.
- `300` ticks = 10 s a 30 Hz é o valor canônico de §9.4 e o mesmo compasso de `defesa_ticks` em `fase_defesa`; a spec não deve substituí-lo por valor `[HIPÓTESE]`.
- `ilha_perdida` é enfileirado uma única vez por transição `controlada → Latente`; a emissão de `ilha_convertida` na transição contrária é pergunta em aberto em `define.md` e, se admitida, deve ser rotulada escolha de protótipo.
- `Mundo.sinais` é `VecDeque<String>` (ou equivalente determinístico) e sua inclusão em `hash_estado` usa ordem de inserção estável; drenagem da fila, se houver, precisa ser determinística.
- A HUD lê `Mundo.ilhas` e `Mundo.sinais` sem inventar geometria; rótulo explícito de "protótipo/presença global" evita leitura canônica indevida.
- A cobertura determinística existente (coleta, barricada, Fôlego, Súditos) permanece válida; verificação é estrutural, com saídas de execução **[NV]** neste assessment.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; B, C e D são as alternativas de `research.md` e não recebem rastro de implementação neste ciclo. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica prevista em `specs/049-ilhas-vitoria/` (ainda a criar). A presença global do Súdito convertido é **simplificação documentada**, não regra canônica. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa. Scorecard único: 108/625 (A), 48/625 (B), 48/625 (C), 10/625 (D); critério impacto × confiança × facilidade ÷ risco, denominador 625.
