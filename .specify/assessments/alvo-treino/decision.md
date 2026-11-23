# Decision — Alvo de treino jogável no cliente

- **Slug**: `alvo-treino`
- **Decided**: 2026-11-23T08:18Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | A ACR 047 registrou que `Q` gera comando válido, mas o cliente tem só `PEER_ID = 1`: sem alvo no `destino`, `aplicar_ataque` não encontra vítima, a `Vida` nunca muda e a dica "Q para atacar" é enganosa. |
| Força da evidência | strong | ACR 047 lida em `.specify/releases/combate-folego/release.md:38`; `crates/cliente/src/main.rs` e `crates/sim/src/lib.rs` verificados (`aplicar_ataque`, `ALCANCE_CONTATO_M = 2.0`, `ponto_igual`, `BTreeSet<u64>`); contrato da sessão imutável; constituição v1.2.2 lida. |
| Valor vs. inação | strong | 240/625 contra 48/625 (ilhas/vitória) e 10/625 (rede); inação mantém o ataque 047 inerte no cliente e a limitação da ACR como pendência aberta. |
| Viabilidade / appetite | strong | `xs`/`small`: duas constantes, um registro de peer no espelho, um marcador simples, um `destino` e duas linhas de HUD; sem `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack ou rede. |
| Alinhamento estratégico | strong | Não reabre nenhum número de combate (§10.1/§10.6 permanecem `[HIPÓTESE]`/`escolha de protótipo`), não cria entidade canônica e respeita R1 `go`, R4 antes de R7 `1.0.0-local` e `analyze` READ-ONLY. |
| Postura de risco | strong | Risco invertido 5: nada muda no core, contrato, protocolo ou stack; recusa fora de 2 m e um ataque por tick são reaproveitados da sim 047; alvo rotulado como scaffolding não-canônico; R6 registrada como N/A, sem issue simulada nem `gh` improvisado. |

## 80/20 (scorecard único do ciclo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Alvo de treino no cliente standalone (escolhida) | 3×4×4×5 = 240/625 | **240/625** |
| Ilhas/condição de vitória | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério do contrato — impacto × confiança × facilidade ÷ risco, escala 1–5, denominador **625** (5⁴); o quarto fator é o risco invertido (segurança): 5 = risco mínimo, 1 = risco máximo. Este é o único scorecard do assessment; os mesmos valores aparecem em `intake.md`, `research.md` e `shape.md`, sem pontuação paralela.

O alvo de treino vence porque é a única opção que resolve o achado concreto da ACR 047 com **fronteira mínima e risco mínimo simultâneos**: o core determinístico já aplica `atacar_forte`, basta um segundo peer no espelho local para o efeito existir. Impacto **3** registra honestamente que se trata de verificabilidade de uma fatia já implementada, não de nova mecânica; confiança **4** registra que o comportamento do core é conhecido e coberto por testes, mas que o efeito renderizado na HUD permanece **[NV]** sem surface automatizável; facilidade **4** reflete duas constantes, um registro, um marcador e duas linhas de HUD; risco invertido **5** reflete a fronteira intocada e a cobertura determinística preservada. Ilhas/vitória (48/625) dependem de mapa e regras `[HIPÓTESE]` e não endereçam a ACR; rede (10/625) está bloqueada por ADR-004 e inverte a dependência verificabilidade → sincronização. Âncora: STATE.md, decisão 80/20 do ciclo 048, 08:18Z — 240/48/10.

## Veredito e justificativa

**Veredito: go** — Opção A (alvo de treino no cliente standalone, scaffolding não-canônico), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. **Constante e rótulo**: `PEER_ALVO_TREINO = 2` e `ALVO_TREINO_POSICAO = (2.0, 0.0)` declarados em `crates/cliente/src/main.rs` e documentados como **scaffolding de protótipo, explicitamente não-canônico**; nenhum número de combate é promovido a cânone (GDD §10.1/§10.6 permanecem `[HIPÓTESE]`/`escolha de protótipo` do ciclo 047).
2. **Espelho local**: `configurar` registra o peer 2 no espelho `Mundo` na posição do alvo, com vida positiva, para que `aplicar_ataque` o encontre por `peer_id != comando.peer_id && ponto_igual(posicao, destino)` — **sem mudança em `crates/sim`**.
3. **Marcador visível**: spawn de uma malha simples (cubo/cone) em `ALVO_TREINO_POSICAO` com componente `AlvoTreino`, distinto do avatar e de entidades canônicas; escolha de forma é de implementação, não de regra.
4. **Mira e ids**: `solicitar_ataque` usa `destino = Some(ALVO_TREINO_POSICAO)` mantendo `seq`/`request_id` monotônicos e uma pressão = um comando; o core recusa fora de 2 m (`ALCANCE_CONTATO_M`) com `CodigoRecusa::AlvoInvalido`, sem dano nem débito parcial, e mantém no máximo um ataque por jogador por tick via `BTreeSet<u64>` existente.
5. **HUD honesta**: `atualizar_texto` exibe `Alvo: {vida}` lida do peer 2 e substitui a dica por "Aproxime-se do alvo e use Q para atacar"; `Vida: N` continua sendo a do jogador local.
6. **Fronteira preservada**: nada em `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack ou rede; cobertura determinística da sim permanece válida; estilo Rust `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.
7. **Verificação estrutural**: roteiro de 10 passos por inspeção de código, execução do binário com `--minimizado` e capturas de terminal/HUD **quando aplicáveis**; captura visual real não é prometida sem surface automatizável — nesses pontos, **[NV]**.
8. **Constituição v1.2.2**: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Alvo canônico indevido**: mitigado por decisão — marcador simples, componente `AlvoTreino` no cliente, rótulo explícito de scaffolding e nenhuma entrada em `crates/sim` ou no contrato. Se a spec tratar o peer 2 como inimigo/Súdito/estrutura, é violação deste assessment.
- **Fora de alcance**: além de 2 m o core recusa (`hypot > ALCANCE_CONTATO_M`) sem efeito; a dica de aproximação torna isso legível. Nenhuma tolerância, mira ou geometria é acrescentada — o comportamento é o da sim 047, reaproveitado.
- **Débito/dupla contagem e hash**: `BTreeSet<u64>`, recusa sem efeito colateral e `hash_estado` determinístico vêm do ciclo 047 e não são reescritos; o peer 2 é determinístico, sem aleatoriedade. Permanece **[NV]** até execução.
- **Verificação visual**: janela minimizada (`--minimizado`) e ausência de surface automatizável de captura tornam o efeito renderizado **[NV]**; a prova é estrutural. Não prometer screenshot real.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação geral**: gates, build, testes, R4 e R7 (`1.0.0-local`) permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: o ataque determinístico entregue em 047 é inerte no cliente standalone — sem segundo peer no espelho, `Q` gera comando válido, mas nenhuma `Vida` muda e a dica "Q para atacar" é enganosa (ACR 047).
- **Chosen approach**: Opção A — constantes `PEER_ALVO_TREINO = 2` / `ALVO_TREINO_POSICAO = (2.0, 0.0)`, registro do peer 2 no espelho `Mundo`, marcador visível com componente `AlvoTreino`, `solicitar_ataque` com `destino = ALVO_TREINO_POSICAO` e HUD com `Alvo: {vida}` + dica de aproximação. Scaffolding **não-canônico**.
- **In scope**: constantes e rótulo, espelho local, marcador, mira fixa com ids monotônicos, validação de alcance reaproveitada, HUD e rastros. **Out of scope**: inimigo/Súdito/estrutura canônica, nova regra ou número, mudanças em `crates/sim`/contratos/protocolo/Cargo/stack/rede, ilhas/vitória, captura visual prometida e **R6 (N/A)**.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: forma do marcador (cubo/cone); alcance alcançável a partir da posição inicial; API de `adicionar_jogador` com posição/vida; ausência de divergência de `hash_estado`/replay com o peer 2 — validar na especificação e no roteiro de 10 passos.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | constantes `PEER_ALVO_TREINO = 2` / `ALVO_TREINO_POSICAO = (2.0, 0.0)`, rotuladas não-canônicas | contrato da sessão; `intake.md` |
| FR-002 | peer 2 registrado no espelho `Mundo` em `configurar` (posição do alvo, vida positiva) | `crates/cliente/src/main.rs`; `shape.md` · Opção A |
| FR-003 | marcador visível simples (cubo/cone) com componente `AlvoTreino` em `(2.0, 0.0)` | contrato; `shape.md` · Opção A |
| FR-004 | `solicitar_ataque` com `destino = ALVO_TREINO_POSICAO`, ids monotônicos, recusa fora de 2 m | `crates/sim/src/lib.rs:462-495`; `define.md` |
| FR-005 | HUD `Alvo: {vida}` e dica "Aproxime-se do alvo e use Q para atacar" | contrato; `shape.md` · Opção A |
| FR-006 | fronteira (sim/contratos/protocolo/Cargo/stack/rede intactos), rastros e constituição v1.2.2 | contrato; `.specify/memory/constitution.md` |
| SC-001 | a ≤ 2 m, `Q` acerta o peer 2 e `Alvo: {vida}` cai 10 (escolha de protótipo 047) | `define.md` · Success Metrics 1 |
| SC-002 | a > 2 m, core recusa `AlvoInvalido`, sem dano nem débito; dica orienta | `define.md` · Success Metrics 2 |
| SC-003 | `aplicar_ataque` encontra o peer 2 por `ponto_igual`/`peer_id != atacante`; ≤ 1 ataque/tick | `define.md` · Success Metrics 3 |
| SC-004 | marcador `AlvoTreino` verificável por inspeção de código em `(2.0, 0.0)` | `define.md` · Success Metrics 4 |
| SC-005 | HUD com `Alvo: {vida}` e dica de aproximação | `define.md` · Success Metrics 5 |
| SC-006 | nenhuma mudança em sim/contratos/protocolo/Cargo/stack/rede; cobertura válida; estilo Rust | `define.md` · Success Metrics 6 |
| T001 | constantes `PEER_ALVO_TREINO` e `ALVO_TREINO_POSICAO` documentadas | `shape.md` · Opção A |
| T002 | `adicionar_jogador(PEER_ALVO_TREINO)` no espelho `Mundo` em `configurar` | `shape.md` · Opção A |
| T003 | peer 2 com posição `(2.0, 0.0)` e vida positiva no espelho | `shape.md` · Opção A |
| T004 | componente `AlvoTreino` e spawn do marcador (cubo/cone) | `shape.md` · Opção A |
| T005 | `solicitar_ataque` com `destino = Some(ALVO_TREINO_POSICAO)` | `shape.md` · Opção A |
| T006 | ids `seq`/`request_id` monotônicos preservados (uma pressão = um comando) | `shape.md` · Opção A |
| T007 | validação de alcance de 2 m reaproveitada (`AlvoInvalido`), sem mudança no core | `shape.md` · Assumptions |
| T008 | um ataque por tick via `BTreeSet<u64>` existente e recusa sem débito | `shape.md` · Assumptions |
| T009 | HUD `Alvo: {vida}` lida do peer 2 | `shape.md` · Opção A |
| T010 | dica "Aproxime-se do alvo e use Q para atacar" substitui a anterior | `shape.md` · Opção A |
| T011 | roteiro de 10 passos (inspeção, `--minimizado`, capturas quando aplicáveis; **[NV]** no visual) | contrato; `define.md` · Open Questions |
| T012 | rastros e governança: R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**; não-execução | `.specify/memory/constitution.md`; `intake.md` |

Os nomes definitivos de T001..T012 serão os de `specs/048-alvo-treino/tasks.md` (o diretório continha apenas o template `spec.md` no momento da leitura); a coluna "Eco" apenas rastreia a cobertura deste assessment. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa. Todo número de combate citado é `[HIPÓTESE]`/`escolha de protótipo` do ciclo 047; nada aqui é especificação, plano ou implementação.

Veredito: go
