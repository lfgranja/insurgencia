# Decision — Mover validado no cliente

- **Slug**: `mover-validado`
- **Decided**: 2026-11-24
- **R1**: `go` · **R3**: 0 clarificações · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | `mover_jogador` escreve posição em pixels e `sincronizar_sim` a injeta como metros no espelho (`main.rs:618-657,1117-1129`); limite e validador da sim (`lib.rs:212,2414-2445`) nunca são consultados pelo cliente. |
| Força da evidência | strong | Código lido no checkout: bypass, injeção, ordem da `.chain()`, `ComandoLocal`, validador público e conversão 260 px/s ÷ 100 ≈ 0,087 m/tick; sem suposição de design novo. |
| Valor vs. inação | strong | **144/3125** contra BF-014 250/3125 (eliminado: exibir hash não muda o jogo), BF-013 100/3125 (eliminado: diagnóstico) e BF-003b não elegível; inação mantém unit mismatch e paridade quebrada com a validação da CLI. |
| Viabilidade / appetite | strong | `small`/`medium`: cliente + helpers de conversão + reordenação de sistemas; nenhuma mudança em GDD, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede. |
| Alinhamento estratégico | strong | Preserva o limite de 1 m e o validador como autoridade única, o rótulo `[HIPÓTESE]` de `PIXELS_POR_METRO = 100.0` e a constituição v1.2.2 (R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`). |
| Postura de risco | strong | Dupla aplicação de movimento, queima de contador na recusa e ordem de sincronização nomeados e fechados por decisão (DoD-1/DoD-3); dívida R6 registrada como N/A, sem issue simulada nem `gh` improvisado. |

## 80/20

| Candidato | Impacto × Confiança × Fechamento × Esforço⁻¹ × Risco⁻¹ ÷ 3125 | Resultado |
|:--|:--|:--|
| `mover-validado` (BF-012, escolhido) | 4×3×3×2×2 = **144/3125** | **144/3125** |
| Mapa/rotas de 12 ilhas (BF-003b) | 270/3125 histórico inclui BF-003a (fechado); depende de topologia canônica — não elegível | 270/3125 (parcial) |
| HUD republica `hash_estado` (BF-014) | 250/3125 — eliminado: exibir hash não muda o jogo | 250/3125 |
| `contratos` consumido por binários (BF-013) | 5×2×5×1×2 = 100/3125 — eliminado: diagnóstico | 100/3125 |

Critério do contrato — impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, escala 1–5, denominador **5⁵ = 3125**; o quarto e o quinto fatores são inversos de esforço e de risco (5 = menor esforço/risco, 1 = maior). Filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo. Impacto **4** (correção de plumbing com efeito observável, não habilidade visível), confiança **3** (limite/validador lidos; conversão 100 é `[HIPÓTESE]`), fechamento **3** (consumidor nomeado, spec 059 e testes ainda por escrever), esforço⁻¹ **2** (cliente + conversão + reordenação + verificação), risco⁻¹ **2** (unit mismatch, dupla aplicação e ordem de sistemas). O BF-012 fora pontuado 240/3125 em `servidor-validacao`; o ciclo 059 reescalona para **144/3125** (`STATE.md:557,586`). Este é o **único** scorecard do assessment.

## Veredito e justificativa

**Veredito: go** — Opção A (`mover` validado no cliente), scorecard 6/6 strong, **144/3125** (`4×3×3×2×2`). Consumidor nomeado: **jogador do cliente Bevy + validação da sim/CLI**. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros **FR-001..FR-004**, com **DoD**: (1) **mover validado** — `Comando { acao: "mover", destino: metros }` com `ValidacaoComandos::validar` antes de `agendar_comando`; (2) **sem queima de contador na recusa** — `seq`/`request_id` avançam só no aceite; (3) **posição derivada do espelho** — `sincronizar_sim` lê `Mundo` e escreve `JogadorLocal` em pixels, sem injeção local. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. `mover_jogador` (`main.rs:618-657`) constrói `Comando { acao: "mover", destino: metros }` e chama `ValidacaoComandos::validar` **antes** de `agendar_comando`; agenda somente no aceite.
2. `seq`/`request_id` de `ComandoLocal` avançam **apenas no aceite**; `MovimentoForaDoLimite`, replay e imobilização preservam posição e monotônicos — *no counter burn on rejection*.
3. `sincronizar_sim` (`main.rs:1117-1129`) lê `Mundo.jogadores[PEER_ID].posicao` e escreve `JogadorLocal` em pixels; **nenhuma** coordenada local é injetada no espelho.
4. Reordenação da `.chain()`: `avancar_simulacao` antes de `sincronizar_sim`, e `sincronizar_avatar` depois de `sincronizar_sim`.
5. Todos os helpers de destino/alvo operam em metros via `pixels_para_metros`; `PIXELS_POR_METRO = 100.0` permanece `[HIPÓTESE]`/protótipo, `260 px/s → 2,6 m/s ≈ 0,087 m/tick < LIMITE_MOVIMENTO_M = 1.0`; nenhuma regra canônica alterada.
6. Fronteira preservada: nada em GDD, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, CI, stack ou rede. **BF-012 passa a `fechado`**; BF-003b e BF-013 permanecem abertos.
7. Constituição v1.2.2: R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`; **R6/taskstoissues N/A** — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado. **PR #76 (`dev→main`) pertence a outro owner ativo e não é tocado**; `dev→main` e deploy são exclusivos da SE.

## Riscos registrados

- **Unit mismatch pixels↔metros**: mitigado por `pixels_para_metros` em todos os destinos/alvos; o espelho nunca recebe pixels. Valor 100 rotulado `[HIPÓTESE]`, não promovido a cânone.
- **Dupla aplicação de movimento / ordem dos sistemas**: mitigados pela fonte única no espelho e pela ordem `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`.
- **Queima de contador na recusa**: mitigada por validar com os contadores candidatos antes de incrementar; aceite é a única condição de avanço.
- **Arredondamento no limite de 1 m**: passo nominal ≈ 0,087 m/tick é folgado; comportamento exato na fronteira fica **[NV]** até verificação.
- **Dívida R6 / PR #76**: R6 é N/A sob v1.2.2 (sem adaptador de issues); nenhuma issue simulada nem `gh` improvisado. PR #76 é de outro owner ativo, intocável; promoção `dev→main` e deploy são SWE-exclusivos.
- **Verificação**: `./verify.sh` (jobs `gates`/`rust`), `cargo test`/`cargo check` e captura visual Bevy permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: WASD do cliente escreve posição em pixels e a injeta no espelho em metros, burlando o limite de 1 m e o validador da sim; a posição local não deriva do estado autoritativo.
- **Chosen approach**: Opção A — `mover` convertido para metros, validado antes de agendar, com contadores só no aceite; `sincronizar_sim` derivado do espelho e sistemas reordenados.
- **In scope**: `PIXELS_POR_METRO`/`pixels_para_metros`, `mover_jogador`, `ComandoLocal`, `agendar_comando`, `sincronizar_sim`, ordem `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`, helpers de destino/alvo, testes e rastros. **Out of scope**: BF-003b, BF-013, BF-014, rede/ADR-004, contratos/protocolo/Cargo/CI/stack, GDD canônico, promoção `dev→main`/deploy, **R6 (N/A)** e qualquer execução neste assessment.
- **Consumidor**: jogador do cliente Bevy + validação da sim/CLI.
- **Success metrics**: SC-001..SC-005 em `define.md` (todas com baseline **[NV]** até verificação).
- **Comandos de prova** (`STATE.md:590`, não executados aqui): `cargo test --locked -p insurgencia-cliente --lib`; `cargo test --locked -p insurgencia-sim --test simulacao`; `cargo test --locked -p insurgencia-servidor --test cli`; `cargo check --locked -p insurgencia-cliente --all-targets`; `./verify.sh` remoto (`gates`/`rust`).
- **Carried-forward open questions**: defasagem de um tick entre `mover_jogador` e `avancar_simulacao`, arredondamento na fronteira de 1 m e efeito de `atualizar_texto` após a reordenação — validar na especificação.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | `PIXELS_POR_METRO = 100.0`, `pixels_para_metros`/`metros_para_pixels`, destinos/alvos em metros | contrato da sessão; `crates/cliente/src/lib.rs:18-21`; `intake.md` · `shape.md` |
| FR-002 | `Comando { acao: "mover", destino: metros }`, `ValidacaoComandos::validar` antes de `agendar_comando`, contadores só no aceite | `crates/cliente/src/main.rs:618-657`; `crates/sim/src/lib.rs:2414-2445`; `define.md` |
| FR-003 | `sincronizar_sim` lê `Mundo` e escreve `JogadorLocal` em pixels; ordem `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar` | `crates/cliente/src/main.rs:108-128,1117-1129`; `shape.md` |
| FR-004 | evidência/validação/release; comandos de prova e **[NV]** | `STATE.md:590`; `research.md` · `decision.md` |
| SC-001 | avatar/HUD só avançam quando o núcleo aceita `mover` | `define.md` · Success Metrics 1 |
| SC-002 | recusa preserva posição e `seq`/`request_id` — *no counter burn* | `define.md` · Success Metrics 2 |
| SC-003 | passo ≈ 0,087 m/tick aceito; destino > 1 m recusado sem agendamento | `crates/sim/src/lib.rs:212,2440-2445`; `define.md` · Success Metrics 3 |
| SC-004 | `sincronizar_sim` sem escrita de pixels no espelho; ordem dos sistemas verificável | `define.md` · Success Metrics 4 |
| SC-005 | fronteira intacta, estilo Rust e cobertura determinística preservada | `define.md` · Success Metrics 5 · **[NV]** |
| T001 | constante `PIXELS_POR_METRO` e helpers de conversão documentados | `shape.md` · Opção A |
| T002 | `mover_jogador` com `Comando` e `destino` em metros | `shape.md` · Opção A |
| T003 | `ValidacaoComandos::validar` antes de `agendar_comando` | `shape.md` · Opção A |
| T004 | `seq`/`request_id` avançando somente no aceite | `shape.md` · Opção A |
| T005 | `sincronizar_sim` lendo o espelho e escrevendo `JogadorLocal` em pixels | `shape.md` · Opção A |
| T006 | reordenação `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar` | `shape.md` · Opção A |
| T007 | helpers de destino/alvo convertidos via `pixels_para_metros` | `shape.md` · Opção A |
| T008 | bloqueio por imobilização mantido no cliente e na sim | `shape.md` · Assumptions |
| T009 | testes de aceite, recusa sem consumo e derivação do espelho | `shape.md` · Assumptions |
| T010 | rastros e governança: R1 `go`, R3 = 0, R4 antes de R7, **R6 N/A**, PR #76 intocável, não-execução | contrato; `intake.md` |

Os nomes definitivos de T001..T010 são os da spec 059; a coluna "Eco" apenas rastreia a cobertura deste assessment. Nada aqui é especificação, plano, tarefa, implementação ou evidência de execução.

Veredito: go
