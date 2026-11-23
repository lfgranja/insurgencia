# Intake — Controle de ilha por Súdito (subset de vitória)

**Data**: 2026-11-23T11:12Z · **Slug**: `ilhas-vitoria` · **Sessão**: `session_id=20261123T111200Z-controle-ilha`, `session_title=insurgencia - controle de ilha por sudito`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 049: tornar **verificável na simulação determinística** a regra canônica de controle de ilha fixada em GDD §9.4:489,501 — **10 s contínuos de presença de um Súdito da Comuna controlam a ilha; a ausência retorna a ilha a Latente e emite `ilha_perdida`**. O protótipo roda a 30 Hz, então 10 s = **300 ticks** contínuos. A sim já possui Súditos com estado `convertido` (GDD §9.2; ciclo 046), Fôlego/ataque (ciclo 047), coleta e barricadas determinísticas (ciclos 039/040) e um alvo de treino no cliente (ciclo 048) — mas **não possui nenhum estado de ilha, nenhuma contagem de controle e nenhum sinal**: `avancar_tick` executa `prepare → skills → defesa → finish`, e `hash_estado` cobre jogadores, defesas e Súditos, sem ilhas. O recorte acrescenta `Ilha { id, controlada, controle_ticks }` em `Mundo.ilhas: BTreeMap<u64, Ilha>`, uma fase `fase_controle_ilhas` entre `fase_defesa` e `fase_finish`, `Mundo.sinais: VecDeque<String>` para o sinal canônico `ilha_perdida`, e reflexo de ilhas/sinais no hash e no HUD. É a primeira fatia de **condição de vitória** sem mapa, sem Nós de Controle e sem rede.

**Motivação**: fechar a ponte entre a regra canônica já escrita no GDD e um estado observável na sim, com fronteira mínima: nenhuma mudança em contratos, protocolo, Cargo, stack ou rede. GDD §9.1 fixa a semântica de sinais (`ilha_convertida` no estado Convertido, `ilha_perdida` no estado Reconvertido) e §9.4 fixa o critério de controle; ambas são **canônicas**, mas o protótipo não tem geometria de ilha nem servidor central. Por isso, a presença de **qualquer Súdito convertido é tratada como global** — todas as ilhas enxergam a mesma presença — e essa é uma **simplificação explícita a registrar**, não uma regra nova. Nós de Controle, Resistência Ideológica, papéis W/E/R e vitória completa ficam de fora deste ciclo.

## Contexto do ciclo (observado no contrato da sessão e em arquivos lidos; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/ilhas-vitoria-049/`, branch `049-ilhas-vitoria`, base `42b5f7b`.
- Alvo de especificação `specs/049-ilhas-vitoria`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012 (o diretório `specs/049-ilhas-vitoria` ainda não existe no momento da leitura; o handoff é para `/speckit.specify`).
- Constituição v1.2.2: R1 `go`, R4 antes de R7 `1.0.0-local`; **R6 N/A** — fora da sequência executável enquanto não existir o adaptador de issues: dívida de automação, nunca issue simulada, nunca `gh` improvisado.
- Regra canônica lida em `docs/GDD - Insurgência.md`: §9.4:489 — "Controlar uma ilha exige duas condições: superar o Desafio de Ilha e manter ao menos 1 Súdito da Comuna na ilha por 10 s contínuos. Sem o Súdito, a ilha volta a Latente e o sinal `ilha_perdida` é emitido"; §9.4:501 — "Uma ilha cujo desafio foi concluído, mas sem Súdito da Comuna por 10 s, retorna a Latente e emite `ilha_perdida`". Sinais em §9.1:439-455 (`ilha_convertida`, `ilha_perdida`) e presença de Súditos em §9.2:459-466.
- Estado verificado no checkout: `crates/sim/src/lib.rs::avancar_tick` chama `fase_prepare`, `fase_skills`, `fase_defesa`, `fase_finish`, `avancar_reversoes` e incrementa `tick`; `fase_defesa` (506+) usa acumulador `defesa_ticks` com `300` ticks para +1 ponto de autogestão — o mesmo compasso de 10 s a 30 Hz serve de referência para `controle_ticks`; `fase_finish` (868+) limpa comandos, regenera Fôlego e expira barricadas/pontes. `Sudito` (185+) já tem campo `convertido: bool` e a sim já tem helpers de conversão/reversão (858-865, 344-345). `Mundo` (216+) contém `jogadores`, `barreiras`, `pontes`, `suditos` — **sem `ilhas` e sem `sinais`**. `hash_estado` (909+) serializa jogadores e demais mapas, inclusive `sudito.convertido`, mas não há ilhas nem sinais no hash.
- Decisão 80/20 do ciclo 049 registrada em `.specify/memory/STATE.md:363-372` (11:15Z): escolha `ilhas-vitoria`, subset **controle de ilha**; Nós de Controle e Resistência Ideológica ficam para o próximo ciclo; R6 permanece N/A.

## Scorecard único do ciclo

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Controle de ilha por Súdito — 10 s contínuos (escolhida) | 4×3×3×3 = 108/625 | **108/625** |
| Nós de Controle + vitória completa | 4×2×2×3 = 48/625 | 48/625 |
| Papéis W/E/R + níveis | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = **625** (o quarto fator é o risco invertido/segurança: 5 = risco mínimo, 1 = risco máximo). Este é o **único** scorecard do assessment; `research.md`, `shape.md` e `decision.md` repetem os mesmos valores, sem pontuação paralela. Fonte autoritativa: `.specify/memory/STATE.md:363-372`.

Escolhe-se o controle de ilha: é o maior valor do scorecard (108/625) e a única opção que materializa uma regra **canônica** do GDD com fronteira mínima — um novo mapa em `Mundo`, uma fase no tick, uma fila de sinais e suas projeções em hash/HUD. Impacto **4** porque introduz a primeira condição de vitória observável da sim; confiança **3** porque a regra é canônica e o acumulador de 300 ticks já tem precedente em `fase_defesa`, mas a semântica de presença global e o acoplamento com sinais/hash precisam de validação na spec; facilidade **3** porque são poucos pontos de mudança, porém cruzam tick, hash e HUD; risco invertido **3** porque a fronteira de sim permanece determinística e nada muda em contratos/protocolo/Cargo/stack/rede, mas a simplificação de presença global e a emissão de sinais exigem disciplina para não virar regra canônica nova. As alternativas de 48/625 dependem de Nós e papéis ainda não modelados; a rede (10/625) permanece bloqueada por ADR-004.

## Evidências de entrada (coladas; não reexecutadas)

- `docs/GDD - Insurgência.md:489` (lido): regra canônica de controle — duas condições (Desafio concluído + 1 Súdito da Comuna por 10 s contínuos), perda retorna a Latente e emite `ilha_perdida`. `:501` repete o critério de aceitação. `:439` registra `[HIPÓTESE]` a deduplicação de sinais duplicados em 2 s pelo servidor central — **não reaberta** neste ciclo.
- `docs/GDD - Insurgência.md:447-448` (lido): tabela de sinais de §9.1 — `ilha_convertida` no estado Convertido e `ilha_perdida` no estado Reconvertido; `:466` fixa que uma ilha convertida libera 2 Súditos por pulso (3 na Rota Central) — `[HIPÓTESE]` de §9.2, fora de escopo.
- `crates/sim/src/lib.rs:393-399` (lido): `avancar_tick` com ordem `prepare → skills → defesa → finish → reversoes → tick += 1`. A nova fase insere-se entre `fase_defesa()` e `fase_finish()`.
- `crates/sim/src/lib.rs:506-525` (lido): `fase_defesa` usa `defesa_ticks` com marco de `300` ticks (10 s a 30 Hz) e zera o acumulador quando a condição de presença/login falha — precedente direto para `controle_ticks`.
- `crates/sim/src/lib.rs:185-191,216-238,858-865,344-345` (lido): `Sudito` com `convertido: bool`; `Mundo` sem `ilhas`/`sinais`; helpers de conversão e reversão já existem.
- `crates/sim/src/lib.rs:909-980` (lido): `hash_estado` serializa jogadores, barreiras, pontes e Súditos (inclusive `convertido`), determinístico por `BTreeMap` — é aqui que ilhas e sinais devem entrar.
- Contrato da sessão (imutável): `Ilha { id, controlada, controle_ticks }`; `Mundo.ilhas: BTreeMap<u64, Ilha>` e `adicionar_ilha(id)`; `Mundo.sinais: VecDeque<String>` (ou equivalente determinístico); fase `fase_controle_ilhas` entre `fase_defesa` e `fase_finish`: 300 ticks com súdito convertido → `controlada = true`; sem súdito convertido zera progresso, marca `controlada = false` e enfileira `ilha_perdida` se estava controlada; hash inclui ilhas e sinais; HUD mostra contagem de ilhas controladas e sinais. **Nenhuma mudança em contratos/protocolo/Cargo/stack/rede**.
- Contrato da sessão (imutável): regras Rust — `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`. R1 `go`; R4 antes de R7 `1.0.0-local`; R6 N/A.
- `.specify/memory/STATE.md:363-372` (lido): scorecard 108/48/48/10 com os quatro fatores; escolha `ilhas-vitoria` — subset controle de ilha; presença de Súditos convertidos tratada como global por ausência de mapa/rede, com simplificação registrada; Nós e papéis para o próximo ciclo.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/ilhas-vitoria/`. Não executa gates, build, testes ou formatadores; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `AGENTS.md`, `STATE.md`, `crates/`, `Cargo.toml`, `PROTOCOL_VERSION` nem qualquer código-fonte. Tudo o que depende de execução permanece **[NV]**. R6 não é invocada — N/A por constituição v1.2.2. Roteiro de verificação previsto: leitura de código, execução determinística da sim e inspeção de hash/HUD **quando aplicável**; saídas não produzidas neste assessment permanecem **[NV]**.
