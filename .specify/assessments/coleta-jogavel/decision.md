# Decision — Coleta jogável no cliente

- **Slug**: `coleta-jogavel`
- **Decided**: 2026-11-23T03:54Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | A regra de GDD §7.1 está no Núcleo, mas o avanço só ocorre com comando a cada tick: sem auto-avanço, entrada `C` ou HUD de `Suprimentos`/`Coleta`, a coleta não é jogável. |
| Força da evidência | strong | GDD §7.1:303,310 lido; `crates/sim/src/lib.rs` verificado (`TICKS_COLETA`, campos, `aplicar_coleta`, `BTreeSet`, `avancar_construcoes_automaticas`, `hash_estado`); contrato da sessão imutável; constituição v1.2.2 lida. |
| Valor vs. inação | strong | 500/625 contra 250/625 (só doc comment), 108/625 (ponte provisória) e 10/625 (netcode); inação mantém a renda canônica implementada, porém inalcançável ao jogador. |
| Viabilidade / appetite | strong | `small`: uma rotina espelhando `avancar_construcoes_automaticas`, entrada `C`, dois elementos de HUD e um doc comment; sem `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`. |
| Alinhamento estratégico | strong | Preserva 6 s/180 ticks/+3/+2 sem renegociação, fecha a Fase 1 do pulso de autogestão e respeita R1 `go`, R4 antes de R7 `1.0.0-local` e `analyze` READ-ONLY. |
| Postura de risco | strong | Dupla contagem mitigada por `BTreeSet<u64>`, cancelamento por `mover` via `ponto_igual`, hash já refletivo; R6 registrada como N/A — dívida de automação, sem issue simulada nem `gh` improvisado. |

## 80/20 (scorecard único do ciclo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Coleta no cliente (escolhida) | 5×5×5×4 = 500/625 | **500/625** |
| Só corrigir o doc comment de `destino` | 2×5×5×5 = 250/625 | 250/625 |
| Ponte provisória | 3×3×3×4 = 108/625 | 108/625 |
| Servidor + Lightyear (netcode) | 5×2×1×1 = 10/625 | 10/625 |

Critério do contrato — impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 625 (5⁴); o quarto fator é o risco invertido (segurança): 5 = risco mínimo, 1 = risco máximo. Este é o único scorecard do assessment; os mesmos valores aparecem em `intake.md`, `research.md` e `shape.md`, sem pontuação paralela.

A coleta no cliente vence porque é a única opção com **impacto 5, confiança 5 e facilidade 5 simultâneos**: o número já é canônico, o estado já está no sim e auditado, e o auto-avanço reusa um padrão já implementado no ciclo 040. Só o doc comment tem impacto 2; a ponte é `[SUPOSIÇÃO]` (GDD §10.3:574) e não tem SC próprios; netcode sincronizaria uma fatia que ainda não é jogável. Contraste: ciclo 039 = 180/625 (coleta no Núcleo), ciclo 040 = 400/625 (barricada), ciclo 041 = 500/625 (coleta jogável no cliente).

## Veredito e justificativa

**Veredito: go** — Opção A (coleta jogável no cliente), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. Regra canônica congelada: 6 s = 180 ticks a 1/30 s em ponto neutro → +3 Suprimentos e +2 Materiais em crédito atômico; sair antes zera `coleta_ticks` e `coleta_ponto`, sem crédito parcial.
2. Auto-avanço determinístico no Núcleo: após comando válido `coletar_materiais`, avanço de 1 tick por jogador por tick enquanto o jogador permanecer no mesmo ponto; `mover` para posição diferente cancela; comando e auto-avanço no mesmo tick não contam duas vezes (`BTreeSet<u64>`, padrão do ciclo 040, espelhando `avancar_construcoes_automaticas`).
3. `hash_estado` continua cobrindo `materiais`, `suprimentos`, `coleta_ticks` e `coleta_ponto` em ordem estável de `BTreeMap`.
4. Cliente: tecla `C` em just pressed agenda `coletar_materiais` com `destino` = posição atual e `seq`/`request_id` monotônicos; HUD exibe `Suprimentos` e progresso de `Coleta` além de Materiais/Barreiras/Construindo.
5. Correção documental: doc comment de `Comando.destino` passa a citar `mover`, `coletar_materiais` e `construir_barreira`.
6. Fronteira preservada: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`; sem `unsafe`, `unwrap`, `expect` ou `#[allow]` — `forbid(unsafe_code)`, `deny(clippy::pedantic)`, `max_width=100`.
7. Constituição v1.2.2 aplicada no ciclo: R1 `go`, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Dupla contagem (tick com comando + auto-avanço)**: mitigada pelo `BTreeSet<u64>` local, já usado por `aplicar_coleta` e `avancar_construcoes_automaticas`.
- **Cancelamento e ponto**: `ponto_igual` (via `total_cmp`) é a comparação canônica; `mover` para posição diferente zera o parcial, preservando "sair antes zera o acúmulo parcial" e evitando reintrodução no mesmo tick.
- **Determinismo do hash**: campos de coleta e estoque já entram em `hash_estado`; a iteração de `jogadores` permanece em `BTreeMap`.
- **Dívida R6 (automação, N/A)**: fora da sequência executável na v1.2.2; nenhuma issue é criada e nenhum `gh` é improvisado. Não é dívida de produto.
- **Verificação**: gates, build, testes, R4 e R7 permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: a coleta canônica existe no Núcleo, mas avança só com comando por tick e não tem entrada nem feedback no cliente — não é jogável nem observável de ponta a ponta.
- **Chosen approach**: Opção A — auto-avanço determinístico de 1 tick/jogador/tick no mesmo ponto com guarda `BTreeSet<u64>`, entrada `C` com ids monotônicos e HUD de `Suprimentos`/`Coleta`.
- **In scope**: rotina de auto-avanço em `fase_skills`, cancelamento por `mover`, guarda de dupla contagem, crédito atômico em 180 ticks, `hash_estado`, doc comment de `Comando.destino`, tecla `C`, HUD. **Out of scope**: reabertura de §7.1, ponto neutro no contrato, ponte, barricada, netcode, combate, R6, mudanças em contratos/protocolo/Cargo.toml.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: confirmação do relógio de 30 Hz na especificação, ordem de serialização do hash e nomes definitivos de T001..T012 em `specs/041-coleta-jogavel/tasks.md` — validar no `/speckit.specify`.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | regra 6 s/180 ticks → +3 Suprimentos +2 Materiais; sair zera parcial | GDD §7.1:303,310; `intake.md` |
| FR-002 | auto-avanço 1 tick/jogador/tick no mesmo ponto; `BTreeSet<u64>`; cancela ao mover | contrato; `shape.md` · Opção A |
| FR-003 | `hash_estado` cobre `coleta_ticks`, `coleta_ponto`, `materiais`, `suprimentos` | `crates/sim/src/lib.rs`; `define.md` |
| FR-004 | validação de `destino` = posição para `coletar_materiais`; ids monotônicos | `crates/sim/src/lib.rs`; contrato |
| FR-005 | cliente: tecla `C` just pressed, `destino` = posição atual | contrato; `shape.md` · Opção A |
| FR-006 | HUD `Suprimentos`/`Coleta`; doc comment de `Comando.destino` com as três ações | contrato; `intake.md` |
| SC-001 | 180 ticks → crédito atômico +3/+2 | `define.md` · Success Metrics 1 |
| SC-002 | sair antes zera o parcial, sem crédito parcial | `define.md` · Success Metrics 2 |
| SC-003 | no máximo 1 avanço por jogador por tick | `define.md` · Success Metrics 3 |
| SC-004 | auto-avanço continua sem novo comando no mesmo ponto | `define.md` · Success Metrics 4 |
| SC-005 | `hash_estado` determinístico cobre os quatro campos | `define.md` · Success Metrics 5 |
| SC-006 | entrada `C`, HUD e doc comment corrigido | `define.md` · Success Metrics 6 |
| T001 | constante `TICKS_COLETA` e campos de estoque/coleta preservados | `shape.md` · Opção A |
| T002 | guarda `BTreeSet<u64>` de coleta no mesmo tick | `shape.md` · Opção A |
| T003 | rotina `avancar_coletas_automaticas` espelhando a de construção | `shape.md` · Opção A |
| T004 | integração do auto-avanço em `fase_skills` | `shape.md` · Opção A |
| T005 | cancelamento por `mover` para posição diferente | `shape.md` · Opção A |
| T006 | crédito atômico +3/+2 ao atingir 180 ticks | `shape.md` · Opção A |
| T007 | validação `DestinoInvalido` para destino ausente/divergente | `shape.md` · Opção A |
| T008 | `hash_estado` determinístico dos campos de coleta | `shape.md` · Opção A |
| T009 | doc comment de `Comando.destino` com `mover`/`coletar_materiais`/`construir_barreira` | contrato |
| T010 | cliente: tecla `C` e ids monotônicos | `shape.md` · Opção A |
| T011 | HUD `Suprimentos` e progresso de `Coleta` | `shape.md` · Opção A |
| T012 | testes de auto-avanço, cancelamento, dupla contagem e hash; artefatos do ciclo | `shape.md` · Assumptions |
| R6 | N/A — constituição v1.2.2; dívida de automação, nunca issue simulada ou `gh` improvisado | `.specify/memory/constitution.md:50` |

Os nomes definitivos de T001..T012 são os de `specs/041-coleta-jogavel/tasks.md`; a coluna "Eco" apenas rastreia a cobertura deste assessment. Nada aqui é especificação, plano ou implementação.

Veredito: go
