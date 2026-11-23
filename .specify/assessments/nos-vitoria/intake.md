# Intake — Nós de Controle e vitória por Resistência Ideológica

**Data**: 2026-11-23T12:50Z · **Slug**: `nos-vitoria` · **Sessão**: `session_id=20261123T125000Z-nos-vitoria`, `session_title=insurgencia - nos de controle e vitoria`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 050: tornar **verificável na simulação determinística** a condição final de vitória do GDD §6.2/§8.5 — converter os **3 Nós de Controle da Ilha Central** por Propaganda e zerar os **100 pontos de Resistência Ideológica**, encerrando a partida com "Autogestão estabelecida" sem destruir a Sede. A sim já possui Súditos convertidos (ciclo 046), Fôlego/ataque (047), alvo de treino (048) e controle de ilha por presença contínua (049, `Mundo.ilhas`/`Mundo.sinais`/`fase_controle_ilhas`), mas **não possui estado de Nó, nem Resistência Ideológica, nem resultado de partida**: `ACOES_PERMITIDAS` contém apenas `"propaganda"` (conversão de Súdito, ciclo 046), sem `propaganda_no` nem `atacar_no`.

**Motivação**: fechar a ponte entre a regra de vitória escrita no GDD e um estado observável na sim, com fronteira mínima — nenhuma mudança em contratos, protocolo, Cargo, stack ou rede. O GDD é explícito ao rotular essa regra como **`[HIPÓTESE]`** (§6.2:273-275): todos os números abaixo são **escolhas de protótipo**, não cânone a promover:

- 3 Nós (produção, comunicação, segurança) e 100 pontos de Resistência Ideológica `[HIPÓTESE]`;
- −30 pontos por Nó convertido, uma única vez `[HIPÓTESE]`;
- pulso de autogestão seguinte aplica os −10 restantes e encerra a partida `[HIPÓTESE]`;
- dano direto não converte Nó, reduz no máximo 20 pontos e nunca abaixo de 10 enquanto restar Nó não convertido `[HIPÓTESE]`;
- 20 s de Propaganda canalizada por Nó = **600 ticks** a 30 Hz `[HIPÓTESE]`/derivação de protótipo (60 s acumulados para os 3 Nós, conforme §8.5);
- 20 s de inatividade após dano = **600 ticks** `[HIPÓTESE]`/derivação de protótipo.

Derrota por timeout (20 min) ou reconquista de 3 ilhas (§6.2:277, §8.5) fica **fora de escopo** — o recorte entrega apenas o caminho de vitória. Nenhum número de ciclos anteriores é reaberto; os padrões de acumulador por tick, dedup por `(peer_id, tick)`, fila FIFO e `hash_estado` determinístico são reaproveitados.

## Contexto do ciclo (observado no contrato da sessão e em arquivos lidos; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/nos-vitoria-050/`, branch `050-nos-vitoria`, base `9b4f5b4`.
- Alvo de especificação `specs/050-nos-vitoria` (continha apenas o template `spec.md` no momento da leitura); handoff ao `/speckit.specify`, com rastros FR-001..FR-006, SC-001..SC-006, T001..T012.
- Constituição v1.2.2: R1 `go`, R4 antes de R7 `1.0.0-local`; **R6 N/A** — fora da sequência executável enquanto não existir o adaptador de issues: dívida de automação, nunca issue simulada, nunca `gh` improvisado.
- GDD §6.2:271-285 (lido): vitória por converter a Ilha Central, não por demolição; os 3 Nós e a Resistência de 100 são `[HIPÓTESE]`; critérios de aceitação exigem que dano não altere a contribuição de 30 de cada Nó, que o fim ocorra no primeiro pulso em que a Resistência chega a 0 com a ilha intacta e que nenhuma estrutura da Sede seja destruída.
- GDD §8.5:416-437 (lido): tabela da Ilha Central — "converter 3 Nós de Controle; Resistência Ideológica 100 → 0", pré-requisito de 2 ilhas por rota + Plano Mestre, penalidade de dano não converter Nó; critérios: "60 s acumulados de Propaganda" e "reduzir exatamente 90 pontos", "pulso seguinte aplica os 10 finais" e "Nó atingido apenas por dano permanece contabilizado na Resistência após seus 20 s de inatividade".
- `crates/sim/src/lib.rs:14-15,99-109,228-265,429-437,575-596,1009-1078` (lido): passo fixo de 30 Hz; `ACOES_PERMITIDAS` com 8 ações, sem `propaganda_no`/`atacar_no`; `Mundo` já tem `jogadores`, `barreiras`, `pontes`, `suditos`, `ilhas`, `sinais`; `avancar_tick` roda `prepare → skills → defesa → controle_ilhas → finish → reversoes → tick += 1`; `fase_controle_ilhas` usa presença global de Súdito convertido; `hash_estado` já serializa ilhas e sinais em ordem estável.
- `crates/cliente/src/main.rs:709-746,813-880` (lido): `KeyG` agenda `"propaganda"` (Súdito), `KeyQ` agenda `"atacar_forte"` (alvo de treino); HUD já exibe `Ilhas: {ilhas_controladas}` e `Sinais: {sinais}` — faltam `Resistência` e `Nós` convertidos, além das teclas `R`/`F` reservadas para `propaganda_no`/`atacar_no`.
- Contrato da sessão (imutável): `NoControle { id, convertido, propaganda_ticks, inativo_ticks }`, `Mundo.nos: BTreeMap<u64, NoControle>`, `resistencia_ideologica: i64`, `resultado: Option<ResultadoPartida>`; ações `propaganda_no` e `atacar_no` em `ACOES_PERMITIDAS` com `destino` = posição fixa do Nó, validação, auto-avanço por um comando e dedup por id de Nó/tick; conversão reduz 30 uma vez; com os 3 Nós convertidos, o pulso seguinte aplica −10 e `resultado = Vitoria`. Derrota fora de escopo.
- Contrato da sessão (imutável): regras Rust — `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; R1 `go`; R4 antes de R7 `1.0.0-local`; R6 N/A.
- `.specify/memory/STATE.md` (lido, entrada `2026-11-23T12:50Z`): decisão 80/20 do ciclo 050 com os quatro fatores e a escolha `nos-vitoria`; todos os números do GDD rotulados `[HIPÓTESE]`.

## Scorecard único do ciclo

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Nós de Controle + Resistência + vitória (escolhida) | 5×2×2×3 = 60/625 | **60/625** |
| Papéis W/E/R + níveis | 4×2×2×3 = 48/625 | 48/625 |
| PVE/PNJs | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (risco invertido/segurança: 5 = risco mínimo, 1 = risco máximo), escala 1–5, denominador 5⁴ = **625**. Este é o **único** scorecard do assessment; `research.md`, `shape.md` e `decision.md` repetem os mesmos valores, sem pontuação paralela. Fonte autoritativa: `.specify/memory/STATE.md`, entrada `2026-11-23T12:50Z`.

Escolhe-se Nós/vitória: é o maior valor do scorecard (60/625) e a única opção que materializa a condição final de vitória, agora factível após controle de ilha e Súditos. Impacto **5** porque fecha o objetivo da partida; confiança **2** porque **todos** os números são `[HIPÓTESE]` e não há estado de Nó/Resistência/resultado na sim; facilidade **2** porque a fatia cruza novas ações, validação, tick, hash, HUD e teclas, ainda que sem contrato novo; risco invertido **3** porque a fronteira permanece determinística e isolada, mas exige disciplina para não promover hipótese a cânone, não confundir dano com conversão e não divergir o hash.

## Evidências de entrada (coladas; não reexecutadas)

- `docs/GDD - Insurgência.md:271-285` (lido): §6.2 — vitória por autogestão na Ilha Central; 3 Nós e 100 de Resistência marcados `[HIPÓTESE]`; −30/Nó, pulso final −10, teto de dano 20 com piso 10, 20 s de canalização por Nó; dano não altera a contribuição do Nó; partida termina no pulso em que a Resistência chega a 0 com a ilha intacta.
- `docs/GDD - Insurgência.md:416-437` (lido): §8.5 — linha da Ilha Central e critérios de aceitação: 60 s acumulados de Propaganda, exatamente −90, pulso final −10, inatividade de 20 s após dano sem perder a contribuição de 30.
- `crates/sim/src/lib.rs:14-15,99-109` (lido): `PASSO_SEGUNDOS = 1/30`; `ACOES_PERMITIDAS` não inclui `propaganda_no`/`atacar_no`; a ação existente `"propaganda"` converte Súditos e **não** deve ser reutilizada para Nós sem rótulo distinto.
- `crates/sim/src/lib.rs:228-265,429-437` (lido): `Mundo` sem `nos`/`resistencia_ideologica`/`resultado`; ordem das fases de `avancar_tick` e ponto de inserção da lógica de Nós junto ao pulso, preservando o incremento final de `tick`.
- `crates/sim/src/lib.rs:575-596,1009-1078` (lido): padrões de 049 reaproveitáveis — acumulador por tick, emissão de sinal em evento (não a cada tick) e `hash_estado` com `ilhas`/`sinais` em ordem estável.
- `crates/cliente/src/main.rs:709-746,813-880` (lido): padrão de handler `just_pressed` com `seq`/`request_id` monotônicos e HUD com `Ilhas`/`Sinais`; pontos de extensão para `R`/`F` e para `Resistência`/`Nós`.
- `.specify/memory/STATE.md` (lido, `2026-11-23T12:50Z`): 60/625 escolhida; papéis 48/625, PVE/PNJs 48/625, rede 10/625; R6 N/A.
- `.specify/memory/constitution.md` — constituição v1.2.2 (lida em assessments anteriores): R1 `go`, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY e `converge` append-only; **R6 N/A**.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/nos-vitoria/`. Não executa gates, build, testes ou formatadores; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `AGENTS.md`, `STATE.md`, `crates/`, `Cargo.toml`, `PROTOCOL_VERSION` nem qualquer código-fonte. Tudo o que depende de execução permanece **[NV]**; R6 não é invocada — N/A por constituição v1.2.2. Roteiro de verificação previsto (somente quando executado por agente posterior): leitura de código, simulação determinística tick a tick, comparação de `hash_estado` e inspeção de HUD/teclas — saídas não produzidas neste assessment permanecem **[NV]**.
