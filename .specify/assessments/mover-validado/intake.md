# Intake — Mover validado no cliente

**Data**: 2026-11-24 · **Slug**: `mover-validado` · **Sessão**: `insurgencia - mover validado no cliente` · **Ciclo**: 059 · **Pendência**: BF-012

## Ideia

Capturar, sem reabrir regra canônica, a fatia 80/20 do BF-012: fazer o WASD/setas do cliente Bevy gerar **comandos `mover` validados pelo Núcleo** em vez de escrever diretamente a posição do avatar e injetar coordenadas de tela no espelho autoritativo. Hoje `mover_jogador` chama `deslocar` sobre `JogadorLocal` em unidades de tela, e `sincronizar_sim` copia essa posição para `Mundo.jogadores[PEER_ID].posicao` em metros — sem `ValidacaoComandos::validar`, sem `agendar_comando` e com unit mismatch. A correção introduz a conversão explícita `PIXELS_POR_METRO = 100.0` (`[HIPÓTESE]`/protótipo), agenda `mover` com `destino` em metros **somente após aceite** da validação, avança `seq`/`request_id` **apenas no aceite** e inverte a reconciliação: `sincronizar_sim` passa a ler a posição do espelho e escrever `JogadorLocal` em pixels; `avancar_simulacao` roda antes dele e `sincronizar_avatar` depois dele.

**Motivação**: fechar BF-012 sem tocar em GDD/cânone, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede. O limite de 1 m e o validador de `mover` já existem no sim (`lib.rs:212,2414-2445`) e o cliente já tem o recurso `ComandoLocal` e o padrão `agendar_comando` em todas as outras ações (`main.rs:662-682`); o que falta é ligar o WASD a esse caminho e corrigir a direção da sincronização. É correção de plumbing com consumidor fora dos artefatos do ciclo — o jogador do cliente Bevy e a validação da sim/CLI —, não uma nova habilidade.

## Evidências de entrada (coladas; não reexecutadas)

- [`crates/cliente/src/main.rs:618-657`, lido] `mover_jogador` lê WASD/setas, normaliza a direção, calcula `passo = direcao.normalize() * VELOCIDADE * tempo.delta_secs()` e faz `jogador.0 = deslocar(jogador.0, (passo.x, passo.y))` em **unidades de tela**. Não há `Comando`, não há validação e não há `agendar_comando`: o movimento local nunca passa pelo Núcleo. Bloqueia apenas quando `imobilizado_ticks > 0` no espelho (`:624-631`). (confidence: high)
- [`crates/cliente/src/main.rs:1117-1129`, lido] `sincronizar_sim` obtém `simulacao.0.jogadores.get_mut(&PEER_ID)` e executa `remoto.posicao = (f64::from(x), f64::from(y))` com `(x, y) = jogador.0.posicao` — **injeção direta de pixels do cliente no espelho em metros**. É o bypass que o BF-012 precisa eliminar, invertendo a leitura: espelho → local. (confidence: high)
- [`crates/sim/src/lib.rs:212`, lido] `const LIMITE_MOVIMENTO_M: f64 = 1.0` — teto de distância por comando `mover`, em metros; rótulo de constante interna da sim, sem cânone novo. (confidence: high)
- [`crates/sim/src/lib.rs:2414-2445`, lido] `ValidacaoComandos::validar(mundo, peer_id, acao, seq, request_id, destino)` checa peer, replay de `seq`/`request_id`, whitelist, nível e, para `acao == "mover"`, exige `destino` e recusa `MovimentoForaDoLimite` se `hypot(x - origem_x, y - origem_y) > LIMITE_MOVIMENTO_M`. O validador já é público (`lib.rs:2277`) e é a autoridade de aceite que o cliente ainda não consulta. (confidence: high)
- [`crates/cliente/src/main.rs:108-128`, lido] Na ordem `.chain()` do `Update`, `mover_jogador` vem antes de `sincronizar_avatar` (`:114`) e `sincronizar_sim` (`:126`) vem antes de `avancar_simulacao` (`:127`). A ordem exigida pelo contrato é `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`: hoje avatar e espelho são atualizados com dados locais antes do tick autoritativo. (confidence: high)
- [`crates/cliente/src/main.rs:662-682`, lido] `solicitar_barreira` e os demais helpers já incrementam `local.seq`/`local.request_id` e chamam `simulacao.0.agendar_comando(Comando { peer_id: PEER_ID, acao, seq, request_id, destino: Some(...) })`. É o padrão a reutilizar — com a diferença de que, em `mover`, os contadores só podem avançar **depois** do aceite. (confidence: high)
- [`crates/cliente/src/lib.rs:18-21,156-187`, lido] `POSICAO_INICIAL = (0.0, -236.0)`, `VELOCIDADE = 260.0` ("unidades de tela"), `EstadoJogador.posicao: (f32, f32)` e `deslocar` com `limitar`. Confirma que o estado local do avatar é pixels e o espelho da sim é metros. (confidence: high)
- [contrato da sessão, imutável] Worktree `.specify/queue/worktrees/cycle-059/`, branch `feature/059-mover-validado`, base `origin/dev = 737b876`; score **144/3125** (`4×3×3×2×2`); conversão `PIXELS_POR_METRO = 100.0` (`[HIPÓTESE]`, sem mudança de GDD/cânone); `260 px/s ÷ 100 = 2,6 m/s ≈ 0,087 m/tick < LIMITE_MOVIMENTO_M = 1.0`; consumidor = jogador do cliente Bevy + validação da sim/CLI. (confidence: high)
- [`.specify/memory/STATE.md:582-592`, lido] Decisão 80/20 do ciclo 059: BF-012 escolhido por 144/3125; BF-003b não elegível sem decisão canônica de topologia, BF-013 eliminado por diagnóstico, BF-014 eliminado porque "exibir hash não muda o jogo". DoD(0.9) e governança (R6 N/A, PR #76 intocável, `dev→main`/deploy SWE-exclusivos) fixados. (confidence: high)
- [`.specify/memory/constitution.md` via contrato, lido] Constituição v1.2.2: R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY, `converge` append-only; **R6/taskstoissues N/A** — PARE se invocada, nunca issue simulada, nunca `gh` improvisado. (confidence: high)

## Scorecard único do ciclo

| Candidato | Impacto × Confiança × Fechamento × Esforço⁻¹ × Risco⁻¹ ÷ 3125 | Resultado | Consumidor |
|:--|:--|:--|:--|
| `mover-validado` (BF-012, escolhido) | 4×3×3×2×2 = **144/3125** | **144/3125** | jogador do cliente Bevy + validação da sim/CLI |
| Mapa/rotas de 12 ilhas (BF-003b) | o **270/3125** registrado inclui o BF-003a já fechado; o restante depende de topologia canônica no GDD — não elegível | 270/3125 (parcial) | jogador/servidor |
| HUD republica `hash_estado` (BF-014) | 250/3125 — eliminado: exibir hash não muda comportamento de jogo | 250/3125 | diagnóstico da HUD |
| `contratos` consumido por binários (BF-013) | 5×2×5×1×2 = 100/3125 — eliminado: diagnóstico, sem comportamento observável | 100/3125 | protocolo/servidor |

Critério normativo do contrato: **impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹**, escala 1–5, denominador **5⁵ = 3125**; o quarto e o quinto fatores são inversos de esforço e de risco (5 = menor esforço/risco, 1 = maior). Filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo. Impacto **4** porque a correção destrava paridade cliente/Núcleo mas não é habilidade visível; confiança **3** porque limite e validador foram lidos no código e a conversão 100 é `[HIPÓTESE]`; fechamento **3** porque consumidor e comandos de prova estão nomeados, mas a spec 059 e os testes ainda serão escritos; esforço⁻¹ **2** porque exige cliente, helpers de conversão, reordenação de sistemas e verificação; risco⁻¹ **2** por unit mismatch, dupla aplicação de movimento e ordem de sincronização. O BF-012 já havia sido pontuado em **240/3125** (`5×3×4×2×2`) em `.specify/assessments/servidor-validacao/decision.md`; o ciclo 059 o reescalona para **144/3125** (`4×3×3×2×2`), conforme `STATE.md:557,586`. Este é o **único** scorecard do assessment.

## DoD do ciclo (referência única; repetido nos cinco artefatos)

- **DoD-1 (mover validado)**: `mover_jogador` constrói `Comando { acao: "mover", destino: metros }`, chama `ValidacaoComandos::validar` **antes** de `agendar_comando` e só agenda no aceite.
- **DoD-2 (sem queima de contador na recusa)**: recusa (`MovimentoForaDoLimite`, replay, imobilização) preserva a posição e **não** avança `seq`/`request_id` — *no counter burn on rejection*.
- **DoD-3 (posição derivada do espelho)**: `sincronizar_sim` lê `Mundo.jogadores[PEER_ID].posicao` e escreve `JogadorLocal` em pixels; **nunca** injeta pixels no espelho. Ordem: `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`.
- **DoD-4 (unidades)**: todos os helpers de destino/alvo operam em metros via `pixels_para_metros`; `PIXELS_POR_METRO = 100.0` é `[HIPÓTESE]`, `260 px/s → 2,6 m/s ≈ 0,087 m/tick < 1,0 m`; nenhuma regra canônica alterada.
- **DoD-5 (consumidor/backlog)**: consumidor = jogador do cliente Bevy + validação da sim/CLI; **BF-012 fechado**; BF-003b e BF-013 permanecem abertos.

## Incógnitas de primeira vista

- **[NV]** Efeito renderizado do avatar/HUD com `--minimizado` depende de execução; este assessment é somente leitura.
- **[NV]** Nomes definitivos de SC/T na spec 059 e a conformidade de R4/R7 dependem de gates não executados aqui.
- **[NV]** O comportamento exato de arredondamento pixels↔metros no limite de 1 m precisa de verificação em tempo de execução pelo orquestrador.
- **[NV]** Captura visual Bevy permanece sem janela automatizável; compilação pesada vai para GitHub Actions.

## Rastreabilidade canônica (spec 059)

**FR-001** = conversão `PIXELS_POR_METRO = 100.0` e helpers `pixels_para_metros`/`metros_para_pixels` no cliente; **FR-002** = `mover_jogador` agenda `mover` validado, com `seq`/`request_id` só no aceite; **FR-003** = `sincronizar_sim` deriva a posição do espelho e reordenação `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`; **FR-004** = evidência/validação/release (`[NV]` aqui). DoD-1..DoD-5 e o score **144/3125** (`4×3×3×2×2`) reaparecem em `research.md`, `define.md`, `shape.md` e `decision.md`.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/mover-validado/`. Não executa gates, build, testes, formatadores ou linters; não faz `git add`/commit/push/PR/merge; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/*`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. **PR #76 (`dev→main`) pertence a outro owner ativo e não é tocado**; `dev→main` e deploy são SWE-exclusivos. R6/taskstoissues **N/A** por constituição v1.2.2 — nenhuma issue simulada, nenhum `gh` improvisado. Gate de destino: `./verify.sh` (jobs `gates`/`rust`); CLI de PR = `gh`. Tudo que depende de execução permanece **[NV]**.
