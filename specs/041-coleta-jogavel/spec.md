# Feature Specification: Coleta jogável no cliente

**Branch**: `041-coleta-jogavel` (base `bf05cfa`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T035400Z-coleta-jogavel`, `insurgencia - coleta jogavel no cliente`.

**Âncora**: regra canônica em `docs/GDD - Insurgência.md` §7.1 (pulso de 30 s, Fase 1 "Coletar"): **6 s = 180 ticks** a `PASSO_SEGUNDOS = 1/30` em ponto neutro → exatamente **+3 Suprimentos e +2 Materiais**; sair antes zera o acúmulo parcial. O core já contém `TICKS_COLETA = 180`, os campos `materiais`, `suprimentos`, `coleta_ticks` e `coleta_ponto` em `Jogador` e o `hash_estado` refletivo (ciclo 039). Este ciclo acrescenta o **auto-avanço determinístico** no core e a **tecla `C`/HUD** no cliente. Constituição **v1.2.2 do alvo**: **R6 é N/A** (fora da sequência executável até existir `resolve-tasks.sh` e adaptador; nunca simular issue, nunca `gh` improvisado; registrar dívida de automação). R1 `go`; R4 antes de R7 `1.0.0-local`.

## User Scenarios

### US1 — Auto-avanço determinístico da coleta (P1)
Após um comando válido `coletar_materiais`, a coleta avança automaticamente 1 tick por jogador a cada tick do servidor e de todos os clientes determinísticos, sem que o jogador precise reenviar o comando, enquanto permanecer no mesmo ponto; ao completar 180 ticks, o crédito é idêntico em todas as réplicas.

**Why this priority**: sem auto-avanço, a regra canônica de 6 s do GDD §7.1 exige 180 comandos manuais e a coleta não é jogável; sem avanço determinístico, os clientes divergem (Princípios III/VI).

**Independent Test**: enviar um único comando válido, avançar 180 ticks e comparar `hash_estado` tick a tick entre duas instâncias com o mesmo input.

**Acceptance Scenarios**:
1. **Given** um comando `coletar_materiais` válido no tick T com `destino == posicao`, **When** os ticks T+1…T+180 decorrem sem mudança de posição, **Then** `coleta_ticks` avança exatamente 1 por tick e, no tick final, credita +3 Suprimentos e +2 Materiais atomicamente.
2. **Given** dois comandos `coletar_materiais` válidos do mesmo jogador no mesmo tick, **When** `fase_skills` processa, **Then** `coleta_ticks` avança apenas 1 (guarda `BTreeSet<u64>`), não 2.

### US2 — Regra canônica e estado preservados (P1)
O ciclo reforça, sem diluição, a regra do GDD §7.1 e reutiliza o estado já implementado no core: `TICKS_COLETA = 180`, os quatro campos em `Jogador` e o `hash_estado` que os inclui; nenhuma constante canônica ou recompensa é alterada.

**Why this priority**: o core 039 já é a fonte autoritativa; reabrir valores ou campos quebraria a rastreabilidade e o determinismo já auditados.

**Independent Test**: ler `crates/sim/src/lib.rs` e confrontar constantes, campos e serialização com o registrado no ciclo 039.

**Acceptance Scenarios**:
1. **Given** o estado atual do core, **When** revisado, **Then** `TICKS_COLETA = 180` e o crédito +3/+2 permanecem intactos, com auto-avanço apenas acrescentado.
2. **Given** dois estados com `coleta_ticks`/estoques distintos, **When** `hash_estado` é calculado, **Then** os resumos divergem porque os quatro campos entram na serialização.

### US3 — Validação, deduplicação e limite por jogador (P1)
Apenas comandos com `destino` presente e igual à posição atual iniciam ou sustentam a coleta; um comando repetido no mesmo tick não conta duas vezes; jogadores diferentes avançam no mesmo tick porque o limite é por jogador, não global.

**Why this priority**: é a fronteira entre avanço automático legítimo e divergência/débito indevido; o padrão `BTreeSet<u64>` do ciclo 040 já provou essa guarda.

**Independent Test**: enviar comando sem destino, com destino distante e dois comandos no mesmo tick; conferir `DestinoInvalido`, nenhum avanço e avanço único, respectivamente.

**Acceptance Scenarios**:
1. **Given** `coletar_materiais` com `destino = None` ou `destino != posicao`, **When** validado, **Then** retorna `DestinoInvalido` e nenhum `coleta_ticks` é incrementado.
2. **Given** dois jogadores com comandos válidos no mesmo tick, **When** `fase_skills` processa, **Then** ambos avançam 1 tick de coleta.

### US4 — Cancelamento por movimento, sem crédito parcial (P1)
Mover o jogador para uma posição diferente durante a coleta zera `coleta_ticks` e `coleta_ponto`; a saída antes de 180 ticks não credita Suprimentos nem Materiais; um `mover` para a mesma posição não interrompe a coleta.

**Why this priority**: é a condição de saída do GDD §7.1 e a principal fonte de crédito indevido se mal implementada.

**Independent Test**: iniciar coleta, mover para posição diferente antes de 180 ticks e conferir contador zerado e estoques inalterados; repetir com `mover` para a mesma posição e conferir continuidade.

**Acceptance Scenarios**:
1. **Given** coleta em andamento com `coleta_ticks = 90`, **When** `mover` altera a posição, **Then** `coleta_ticks = 0`, `coleta_ponto = None` e nenhum recurso é creditado.
2. **Given** coleta em andamento, **When** `mover` aponta para a mesma posição (`ponto_igual`), **Then** a coleta continua avançando normalmente.

### US5 — Doc comment de `Comando.destino` e fronteira preservada (P2)
O doc comment de `Comando.destino` em `crates/sim/src/lib.rs` passa a citar `mover`, `coletar_materiais` e `construir_barreira`, refletindo os três usos reais do campo; nenhum campo novo é adicionado e nada muda em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`.

**Why this priority**: documentação desatualizada induz implementadores a tratar `destino` como exclusivo de `mover`, gerando retrabalho e risco de mudança de protocolo.

**Independent Test**: ler a definição de `Comando` e conferir que os três comandos são citados e que nenhum arquivo de protocolo/Cargo foi alterado.

**Acceptance Scenarios**:
1. **Given** o doc comment atual ("usado por `mover`"), **When** revisado, **Then** cita `mover`, `coletar_materiais` e `construir_barreira`.
2. **Given** a fronteira do ciclo, **When** comparada com o baseline `bf05cfa`, **Then** `crates/contratos`, `PROTOCOL_VERSION` e `Cargo.toml` estão intactos.

### US6 — Input e HUD do cliente (P2)
A tecla `C`, apenas no instante em que é pressionada (*just pressed*), agenda um comando `coletar_materiais` com `destino` = posição atual e `seq`/`request_id` monotônicos por recurso local; segurar `C` agenda um único comando. O HUD passa a exibir **Suprimentos** e o progresso de **Coleta** (ticks/180), além de Materiais, Barreiras e Construindo.

**Why this priority**: sem atalho e feedback visual, o jogador não percebe a canalização de 6 s e o servidor não consegue deduplicar comandos (Princípio VI).

**Independent Test**: pressionar e segurar `C` em uma sessão headless; conferir um único comando agendado e o HUD com Suprimentos/Coleta/Suprimentos atualizados.

**Acceptance Scenarios**:
1. **Given** `C` ainda não pressionada, **When** a tecla é pressionada e mantida, **Then** exatamente um comando `coletar_materiais` é agendado com `destino` = posição atual e `seq`/`request_id` incrementais.
2. **Given** coleta em andamento, **When** o HUD é renderizado, **Then** exibe Materiais, Barreiras, Construindo, Suprimentos e progresso de Coleta.

### Edge Cases
- Comando válido no tick T e reenviado em T+1 pelo mesmo jogador: avança 1 em T+1, não acumula dois ticks no mesmo tick.
- `mover` para posição diferente zera a coleta; `mover` para a mesma posição (`ponto_igual`) não zera.
- Dois jogadores coletando no mesmo tick: ambos avançam (o limite de 1 tick/tick é **por jogador**).
- `coleta_ticks` em 179 + 1 tick: crédito exato de +3/+2 e contador zerado; em 179 + saída: nenhum crédito.
- `destino` ausente ou distinto da posição: `DestinoInvalido`, nenhum avanço e nenhum crédito.
- `C` mantida pressionada por vários frames: *just pressed* agenda um único comando; `seq`/`request_id` não retrocedem.

## Requirements

### Functional Requirements
- **FR-001**: acrescentar **auto-avanço determinístico** em `crates/sim/src/lib.rs`: após um comando válido `coletar_materiais` no tick, a coleta avança **1 tick por jogador por tick** enquanto o jogador permanecer no mesmo ponto (`coleta_ponto` igual à posição via `ponto_igual`), reaproveitando o padrão de guarda local `BTreeSet<u64>` já usado por `fase_skills` (`coleta_avancada`/`construcao_avancada`) para que um comando no mesmo tick não conte duas vezes; ao atingir `TICKS_COLETA = 180`, creditar atomicamente **+3 Suprimentos e +2 Materiais** e zerar o contador.
- **FR-002**: preservar a regra canônica do GDD §7.1 e o estado autoritativo já implementado no ciclo 039 — `TICKS_COLETA = 180`, `PASSO_SEGUNDOS = 1/30`, campos `materiais`, `suprimentos`, `coleta_ticks`, `coleta_ponto` em `Jogador` e `hash_estado` refletivo; **não alterar** constantes, recompensa ou serialização já auditadas, apenas acrescentar o auto-avanço.
- **FR-003**: manter a validação e a deduplicação: `coletar_materiais` com `destino` ausente ou diferente da posição é recusado com `DestinoInvalido` e não inicia nem avança a coleta; o limite de um avanço por tick é **por jogador**, permitindo que jogadores distintos avancem no mesmo tick; `mover` com mudança de posição zera `coleta_ticks`/`coleta_ponto` sem crédito parcial.
- **FR-004**: corrigir em `crates/sim/src/lib.rs` o doc comment de `Comando.destino` para citar **`mover`, `coletar_materiais` e `construir_barreira`**; nenhum campo novo no comando e **nenhuma alteração** em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`.
- **FR-005**: no cliente, a tecla `C` em *just pressed* agenda um comando `coletar_materiais` com `destino` = posição atual e `seq`/`request_id` monotônicos por recurso local; o HUD exibe **Suprimentos** e o progresso de **Coleta** (ticks/180), além de **Materiais**, **Barreiras** e **Construindo**.
- **FR-006**: estilo e governança obrigatórios: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, commits em PT-BR com staging explícito; R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A** por v1.2.2 (PARE se invocada, nunca simular issue, nunca `gh` improvisado); este agente não executa gates, build, testes, formatadores, commit, push ou PR (**[NV]**). Esclarecimento: `#[expect(clippy::needless_pass_by_value, reason = "...")]` é atributo de lint autorizado, não uma chamada `.expect()`; a vedação a `expect` atinge apenas chamadas de método.

### Key Entities
- **Auto-avanço da coleta**: transição em `fase_skills` que incrementa `coleta_ticks` em 1 por jogador por tick enquanto `coleta_ponto == posicao`, guardada por `BTreeSet<u64>` local; crédito atômico ao completar 180.
- **Regra canônica (GDD §7.1)**: 6 s/180 ticks em ponto neutro, recompensa +3 Suprimentos/+2 Materiais, perda do acúmulo parcial na saída antecipada.
- **Estado do jogador**: `Jogador` com `materiais`, `suprimentos`, `coleta_ticks`, `coleta_ponto` (já presentes no baseline, reutilizados sem alteração).
- **Hash canônico**: `hash_estado` já serializa os quatro campos; continua refletindo estoques e acúmulo para detectar divergência.
- **Comando de coleta**: `Comando` com `acao = "coletar_materiais"`, `destino` = posição atual, `seq`/`request_id` monotônicos; `destino` é reutilizado por `mover`, `coletar_materiais` e `construir_barreira`.
- **Recurso local do cliente**: contadores `seq`/`request_id` incrementados uma vez por *just pressed* de `C`.

## Clarificações
- "Auto-avanço" não substitui a validação: o comando inicial ainda deve ser válido (`destino` presente e igual à posição); o avanço automático só persiste enquanto a igualdade se mantém.
- "1 tick por jogador por tick" é o mesmo padrão `BTreeSet<u64>` do ciclo 040 para construção: jogadores diferentes progridem no mesmo tick; um único jogador não progride duas vezes no mesmo tick.
- "Ponto neutro" continua identificado por `Comando.destino` (já no contrato); nenhuma entidade nova é criada e `PROTOCOL_VERSION` não é alterado.
- "C" é escolha de input do protótipo para o comando de coleta, distinta da tecla `Q`/`B` citada em outras seções; não reabre valores canônicos do GDD §7.1 nem a regra da barricada (H4).
- O doc comment de `Comando.destino` fica em `crates/sim/src/lib.rs` (não em `crates/contratos`): é correção documental, não mudança de contrato.
- **R6 é N/A** por constituição v1.2.2: não é dívida executada, nem regra ativa, nem simulação de issue; se invocada, PARE e escale ao SWE. A dívida de automação (`resolve-tasks.sh`/adaptador) fica registrada, sem `gh` improvisado.
- A seção "Contexto e Orçamento" do template é documental e não lida por gates; nenhum manifesto é criado neste ciclo (fora de fronteira).

## Success Criteria
- **SC-001**: um único comando `coletar_materiais` válido leva à conclusão em exatamente 180 ticks (6 s) com crédito de +3 Suprimentos e +2 Materiais; duas execuções com o mesmo input produzem o mesmo `hash_estado`.
- **SC-002**: nenhum jogador avança mais de 1 tick de coleta por tick (guarda `BTreeSet<u64>`); comandos repetidos no mesmo tick não duplicam o avanço, e jogadores diferentes avançam no mesmo tick.
- **SC-003**: `mover` com mudança de posição zera `coleta_ticks`/`coleta_ponto` sem crédito parcial; `destino` ausente/diferente não inicia nem avança a coleta; estados com acúmulo/estoques distintos geram hashes divergentes.
- **SC-004**: o doc comment de `Comando.destino` cita `mover`, `coletar_materiais` e `construir_barreira`; nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, constantes canônicas ou recompensa.
- **SC-005**: tecla `C` em *just pressed* gera um único comando com `seq`/`request_id` monotônicos e `destino` = posição atual; HUD exibe Suprimentos e progresso de Coleta além de Materiais/Barreiras/Construindo.
- **SC-006**: 6 FR e 6 SC; T001..T012 únicos em DAG; plano com passos 00→26, checklist com `[x]` = revisão de requisitos, roteiro de 10 passos, R6 N/A e fronteira explícita; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; gates/build/testes **[NV]** até execução pelo orquestrador.

## Assumptions
- O core já contém os campos, a constante e o hash do ciclo 039; este ciclo acrescenta apenas o auto-avanço determinístico e o input/HUD do cliente.
- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam o código Rust na etapa 12; o orquestrador executa gates, build e verificações finais.
- Ambiente headless: nenhuma janela, rede, credencial ou comando `gh` é usado na redação destes artefatos.
- Os valores canônicos do GDD §7.1 são estáveis para o alvo; eventuais hipóteses não são convertidas em requisito.
- `Comando.destino` já existe no core e pode ser reutilizado sem alterar o protocolo.

## Out of scope
Reabrir os valores canônicos do GDD §7.1 ou a regra da barricada (H4); alterações em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, manifesto ou CI; adoção de R6, `taskstoissues`, `resolve-tasks.sh` ou simulação de issue; janelas, deploy, commits/push/PRs e alterações preexistentes fora da fronteira. Tudo o que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: coleta jogável no cliente 5×5×5×4 = **500/625**, escolhida como menor incremento jogável sobre o core já implementado nos ciclos 039/040, contra 250/625 (só corrigir o doc comment de `destino`), 108/625 (ponte provisória) e 10/625 (servidor + Lightyear/netcode); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. Nenhum manifesto é criado.
