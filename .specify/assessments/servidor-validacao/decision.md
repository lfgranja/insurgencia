# Decision — Validação do servidor headless (CLI autoritativo)

- **Slug**: `servidor-validacao`
- **Decided**: 2026-11-24T10:29Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A/bloqueado (constituição v1.2.2, BF-007)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Whitelist local de 6 ações e tabela de 3 com destino no CLI contra 13 ações e validador órfão no Núcleo (`sim:188-202,2087`; `servidor:16-24,99,158-166`); dois consumidores fora do ciclo ficam sem paridade de aceitação. |
| Força da evidência | strong | Decisão 80/20, auditoria **[EV]** e DoD em `.specify/memory/STATE.md:493-536`; `crates/sim/src/lib.rs:185,188-202,212-220,764-766,809-820,1916-1991,2087-2186`, `crates/servidor/src/main.rs:16-24,66-116,142-189` e `crates/servidor/tests/cli.rs:16-21` lidos; constituição v1.2.2 lida. Nada reexecutado; saídas de gates, sondas e CI permanecem **[NV]**. |
| Valor vs. inação | strong | 1280/3125 contra 1125/3125 (HUD `hash_estado`), 240/3125 (cliente `mover`), 100/3125 (contratos) e 50/3125 (rede, inelegível por ADR-004). Inação mantém BF-011 aberto e um contrato de aceitação duplo, com risco de `ultima_recusa` mutar o hash. |
| Viabilidade / appetite | strong | `small`: duas API públicas no sim, remoção de duas tabelas no servidor, uma chamada de validação antes do agendamento, visão local de `seq`/`request_id`, ajuste de uma fixture e cinco testes; sem contrato, protocolo, `Cargo`, stack, rede, CI ou relógio novos. |
| Alinhamento estratégico | strong | Nenhuma regra de validação, `CodigoRecusa` ou `ultima_recusa` alterada; formato `<tick>\t<sha256>` e hash inicial preservados; reaproveita o padrão dos ciclos 042 e 047-055; R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A**; promoção `dev→main` e deploy ficam com a SE. |
| Postura de risco | strong | Risco concentrado em três pontos — mutação de `ultima_recusa`/hash, replay de `seq`/`request_id` no lote e compatibilidade da fixture/testes — mitigados por validação somente-leitura, visão local de monotônicos e cinco testes nomeados; fronteira de stack intacta; riscos residuais **[NV]** registrados. |

## 80/20 (scorecard único do ciclo)

| Candidato | I | C | Fch | E⁻¹ | R⁻¹ | Impacto × Confiança × Fechamento × Esforço⁻¹ × Risco⁻¹ ÷ 3125 | Resultado |
|:--|--:|--:|--:|--:|--:|:--|:--|
| `servidor-validacao` (BF-011) | 4 | 5 | 4 | 4 | 4 | 4×5×4×4×4 = 1280/3125 | **1280/3125** |
| HUD `hash_estado` (BF-014) | 3 | 5 | 3 | 5 | 5 | 3×5×3×5×5 = 1125/3125 | 1125/3125 |
| Cliente `mover` (BF-012) | 5 | 3 | 4 | 2 | 2 | 5×3×4×2×2 = 240/3125 | 240/3125 |
| Contratos consumidos (BF-013) | 5 | 2 | 5 | 1 | 2 | 5×2×5×1×2 = 100/3125 | 100/3125 |
| Rede real (BF-001) | 5 | 2 | 5 | 1 | 1 | 5×2×5×1×1 = 50/3125 — inelegível (ADR-004) | 50/3125 |

Critério normativo da decisão `2026-11-24T10:29Z`: `impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`, escala 1–5, denominador **5⁵ = 3125**; filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo. Impacto **4** (autoridade da fronteira CLI), confiança **5** (código lido no checkout), fechamento **4** (usuário/automação do CLI e testes nomeados, sem UI), esforço⁻¹ **4** (sim + servidor + 5 testes, sem stack nova), risco⁻¹ **4** (risco em `ultima_recusa`/hash e replay do lote, sem contrato/protocolo/Cargo/CI/rede). Este é o **único** scorecard do assessment; `intake.md`, `research.md` e `shape.md` repetem exatamente estes valores, sem pontuação paralela. A rede é inelegível por autorização ADR-004 (BF-001); R6 é N/A por constituição v1.2.2 (BF-007) e não entra no cálculo.

## Veredito e justificativa

**Veredito: go** — Opção A (whitelist derivada do Núcleo + validação na fronteira do CLI), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, **R6 N/A/bloqueado**. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, build, teste, sonda, roteiro, commit, push ou PR foi produzido ou executado neste assessment; todo resultado de verificação permanece **[NV]**.

## Condições (R1)

1. **API pública do sim**: `pub const ACOES_PERMITIDAS: &[&str]` com as mesmas 13 entradas e ordem de `sim:188-202`; `pub fn acao_exige_destino(acao: &str) -> bool` devolvendo `true` para 11 ações e `false` exatamente para `sinalizar_sincronia` e `sinalizar_alavanca`; documentação `///` nos itens públicos.
2. **Regra intacta**: nenhuma alteração em `ValidacaoComandos::validar` (`sim:2087-2186`), `CodigoRecusa` (`sim:1953-1991`), `nivel_minimo_para_acao` (`sim:212-220`), limites de movimento ou `ultima_recusa` (`sim:491,1916-1947`). A chamada na fronteira é somente-leitura.
3. **Parser do servidor**: remover a whitelist local de 6 ações (`servidor:16-24`) e o `exige_destino` hardcoded de 3 ações (`:99`); aridade 4/6 e exigência de coordenadas `f64` finitas passam a derivar de `acao_exige_destino`, rejeitando coordenadas extras nas 2 ações sem destino.
4. **Validação antes do agendamento**: cada comando é validado com `ValidacaoComandos::validar(mundo, peer_id, acao, seq, request_id, destino)` antes de `agendar_comando` (`sim:764-766`); comandos recusados não são agendados e não mutam o `Mundo`.
5. **Replay no lote**: manter visão local de `seq`/`request_id` no servidor para o lote lido, porque o `Mundo` só atualiza esses monotônicos na execução do tick seguinte (`sim:809-820`); a visão local não substitui `SeqOuRequestReplay` nem persiste entre execuções.
6. **Recusa determinística**: `exit 2`, `stdout` vazia e `stderr` `linha N: <CodigoRecusa>` para comandos recusados pelo validador, preservando o `ExitCode::from(2)` e a escrita na `stderr` de `servidor:181-189`; nenhuma linha de hash é emitida quando a execução falha.
7. **Formato e hash**: emissão `<tick>\t<sha256>` antes de `avancar_tick` (`servidor:168-175`) e hash inicial com `stdin` vazia idênticos ao estado inicial do `Mundo`, inclusive `ultima_recusa` None.
8. **Fixture e testes preservados**: ajustar somente `crates/servidor/tests/cli.rs:19` de `coletar_materiais 1 1 0.5 0.5` para `coletar_materiais 1 1 0 0`; não tocar testes de parser que usam `mover (3.5,-4)`. Cinco testes nomeados — `todas_as_treze_acoes_do_sim_sao_aceitas_pela_whitelist_do_cli`, `acoes_com_destino_exigem_coordenadas_finitas_e_rejeitam_coordenadas_extras`, `comando_recusado_pela_validacao_retorna_exit_2_sem_stdout_e_com_codigo_na_stderr`, `comandos_validados_preservam_formato_tick_hash_e_determinismo`, `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial` — elevam 141 → **146** sem afrouxar os existentes.
9. **Verificação (por executar)**: `cargo test -p insurgencia-servidor --test cli` (esperado **146 testes**), sondas `printf ... | cargo run -p insurgencia-servidor -- --ticks N`, roteiro 10/10 em `.specify/validations/servidor-validacao/`, `./verify.sh` e CI `gates`+`rust`; tudo **[NV]** neste assessment.
10. **Constituição v1.2.2**: `tasks` (T009) antes de `analyze` READ-ONLY (T010), `converge` append-only, R4 condicionado ao roteiro e antes de R7 `1.0.0-local` (T012); **R6 N/A/bloqueado** — PARE se invocada, nunca issue simulada, nunca `gh` improvisado, dívida BF-007 entregue ao SWE; promoção `dev→main` e deploy são SWE-exclusivos; BF-011 passa a `fechado` com evidência, sem reabrir sem regressão provada.

## Riscos registrados

- **Mutação colateral do hash**: `ultima_recusa` entra no SHA-256 (`sim:1916-1947`); validar escrevendo no `Mundo` mudaria o hash inicial e a saída tick a tick. Mitigado por validação somente-leitura e pelo teste `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial`.
- **Replay no mesmo lote**: `seq`/`request_id` do `Mundo` só avançam no tick seguinte (`sim:809-820`); dois comandos com o mesmo `seq` na entrada poderiam passar. Mitigado pela visão local no servidor e pelo teste de determinismo.
- **Compatibilidade de fixture e parser**: `coletar_materiais 1 1 0.5 0.5` é inválido para o Núcleo e `mover (3.5,-4)` excede 1 m; a validação na fronteira mudará o que a `stdin` aceita. Mitigado ajustando só a fixture e preservando os testes de parser como testes de sintaxe.
- **Dupla autoridade**: expor `ACOES_PERMITIDAS` e `acao_exige_destino` cria API pública que outros módulos podem reimplementar; a spec deve fixar que o sim é a única origem e que o servidor não mantém tabela paralela.
- **Dívida R6**: N/A por constituição v1.2.2 (BF-007); não é dívida de produto, nenhuma issue simulada e nenhum `gh` improvisado.
- **Verificação geral**: gates, build, testes, sondas, `./verify.sh`, roteiro, CI, R4 e R7 permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: o CLI headless aceita 6 ações e agenda sem validar, enquanto o sim tem 13 ações e um validador não chamado — dois contratos de aceitação divergentes, com risco de mutar `ultima_recusa` e o hash se a correção for ingênua.
- **Chosen approach**: Opção A — `pub const ACOES_PERMITIDAS` e `pub fn acao_exige_destino` no sim; parser do servidor derivado; `ValidacaoComandos::validar` antes de `agendar_comando` com visão local de `seq`/`request_id`; recusa `exit 2`/`stdout` vazia/`stderr linha N: <CodigoRecusa>`; formato `<tick>\t<sha256>` e hash inicial preservados; fixture `0 0`.
- **In scope**: API pública e whitelist única, derivação de aridade/coordenadas, validação na fronteira, replay do lote, recusa determinística, fixture, cinco testes, estilo Rust, rastros e governança. **Out of scope**: regras de validação/`CodigoRecusa`/`ultima_recusa`, formato/hash, testes de parser com `mover (3.5,-4)`, HUD (BF-014), cliente `mover` (BF-012), contratos (BF-013), rede/ADR-004, contratos/protocolo/Cargo/CI/stack, promoção `dev→main`/deploy, **R6 (N/A)** e qualquer execução neste assessment.
- **Success metrics**: SC-001..SC-006 em `define.md` (um teste nomeado por SC-001..SC-005; SC-006 com fronteira, fixture e verificação), todas com baseline **[NV]** até execução.
- **Carried-forward open questions**: renderização textual do `<CodigoRecusa>` na `stderr`, mensagens do parser para aridade/coordenadas, delimitação da visão local de `seq`/`request_id` ao lote, nomes finais de FR/SC/T em `spec.md`/`tasks.md`, conformidade de R4/R7 com gates e roteiro. Nenhuma é clarificação de produto (R3 = 0).

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | whitelist única derivada de `ACOES_PERMITIDAS`, exposta como `pub const` sem mudar entradas/ordem | `crates/sim/src/lib.rs:188-202`; `decision.md` · Condição 1 |
| FR-002 | `pub fn acao_exige_destino`: 11 `true`, `false` só para `sinalizar_sincronia` e `sinalizar_alavanca` | `crates/sim/src/lib.rs:2094-2186` + contrato; `define.md` · Goals |
| FR-003 | parser do servidor sem whitelist local, aridade 4/6 e coordenadas finitas derivadas da API pública | `crates/servidor/src/main.rs:16-24,66-116`; `shape.md` · Opção A |
| FR-004 | `ValidacaoComandos::validar` antes de `agendar_comando`, visão local de `seq`/`request_id`, sem mutar `ultima_recusa` | `crates/sim/src/lib.rs:2087-2186,764-766,1916-1947`; `decision.md` · Condições 4-5 |
| FR-005 | recusa `exit 2`/`stdout` vazia/`stderr linha N: <CodigoRecusa>` e fixture `coletar_materiais 1 1 0 0` | `crates/servidor/src/main.rs:181-189`; `crates/servidor/tests/cli.rs:16-21`; `decision.md` · Condições 6,8 |
| FR-006 | formato `<tick>\t<sha256>`, hash inicial, cinco testes, estilo Rust e governança | `crates/servidor/src/main.rs:168-175`; contrato da sessão; `decision.md` · Condições 7-10 |
| SC-001 | 13 ações aceitas pela whitelist derivada — `todas_as_treze_acoes_do_sim_sao_aceitas_pela_whitelist_do_cli` | `define.md` · SC-001 |
| SC-002 | 11 ações com destino finito, 2 sem destino extra — `acoes_com_destino_exigem_coordenadas_finitas_e_rejeitam_coordenadas_extras` | `define.md` · SC-002 |
| SC-003 | recusa exit/stdout/stderr sem agendar — `comando_recusado_pela_validacao_retorna_exit_2_sem_stdout_e_com_codigo_na_stderr` | `define.md` · SC-003 |
| SC-004 | formato tick/hash e determinismo — `comandos_validados_preservam_formato_tick_hash_e_determinismo` | `define.md` · SC-004 |
| SC-005 | hash inicial com `stdin` vazia — `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial` | `define.md` · SC-005 |
| SC-006 | fixture `0 0`, parser preservado, 146 testes, fronteira e estilo intactos | `define.md` · SC-006 |
| T001 | 00 constituição/branch — worktree `servidor-056`, branch `feature/056-servidor-validacao`, base `0bc2c25`; R1 `go`, R6 N/A | contrato da sessão; constituição v1.2.2 |
| T002 | 01 intake + 02 research — scorecard 1280/3125, fontes lidas, nada executado | `intake.md`; `research.md` |
| T003 | 03 define + 04 shape — problema, métricas e opções A-D + rede inelegível | `define.md`; `shape.md` |
| T004 | 05 decide + 07 clarify — R1 `go`, R3 = 0, handoff ao `/speckit.specify` | este `decision.md` |
| T005 | 06 specify — `spec.md` com US/FR/SC, API pública, recusa e formato | `specs/056-servidor-validacao/spec.md` (em construção) |
| T006 | 08 plan — roteiro de 10 passos, fronteira e R4 antes de R7 | `specs/056-servidor-validacao/plan.md` (a produzir) |
| T007 | 09 checklist de requisitos — um item por FR-001..FR-006 | `checklists/requirements.md` (a produzir) |
| T008 | revisão do checklist — 6 FR/6 SC verificáveis por inspeção | `checklists/requirements.md` (a produzir) |
| T009 | 10 tasks — este arquivo: T001..T012 com DAG linear, `tasks` antes de `analyze` | `specs/056-servidor-validacao/tasks.md` (a produzir) |
| T010 | 11 analyze (READ-ONLY) — depois de `tasks`, sem edição de insumos | `specs/056-servidor-validacao/analyze.md` (a produzir) |
| T011 | 12 implement + 13 converge + 27 validate (R4) — sim, servidor, fixture, 5 testes e roteiro 10/10 | `crates/sim`, `crates/servidor`, `.specify/validations/servidor-validacao/` |
| T012 | 26 release (R7 `1.0.0-local`) — changelog por FR/T, rastreabilidade e rollback documentado, após R4 | `.specify/releases/servidor-validacao/release.md` (a produzir) |

Os nomes definitivos de T001..T012 são os de `specs/056-servidor-validacao/tasks.md`; a coluna "Eco" registra apenas a cobertura deste assessment. **R6 é N/A** por constituição v1.2.2 (BF-007) e não recebe tarefa. Nada aqui é especificação, plano ou implementação, e nada foi executado. Todo resultado de verificação permanece **[NV]**.

Veredito: go
