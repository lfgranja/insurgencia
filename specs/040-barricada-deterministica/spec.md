# Feature Specification: Barricada determinística de materiais

**Branch**: `040-barricada-deterministica` (base `6b913f0`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: go · **Sessão**: `20261123T024300Z-barricada-deterministica`

**Âncora**: valores canônicos em `docs/GDD - Insurgência.md` §9.3:475,483 e §10.3:574,577-579 — custo **20 Materiais**, canalização **1,5 s = 45 ticks**, **200 PV**, duração **12 s = 360 ticks**, máx. **2 barricadas por jogador** (a 3ª remove a mais antiga), recusa sem saldo **sem consumo parcial**. Fôlego (30), alcance (4 m), recarga (10 s) e dimensões (3 m × 2 m) são `[HIPÓTESE]` e ficam **fora** deste ciclo. Constituição **v1.2.2** do alvo: **R6 é N/A** (fora da sequência executável até existir `resolve-tasks.sh` e adaptador; nunca simular issue, nunca `gh` improvisado; registrar dívida de automação). R1 `go`; R4 antes de R7.

## User Scenarios

### US1 — Construção determinística (P1)
O servidor e todos os clientes determinísticos, ao receberem o mesmo comando `construir_barreira` no mesmo tick, produzem exatamente o mesmo resultado: 45 ticks de canalização, débito de 20 Materiais e uma `Barreira` idêntica.

**Why this priority**: sem canalização e débito determinísticos, a simulação diverge entre clientes e o núcleo deixa de ser replicável (Princípio III/VI).

**Independent Test**: executar o mesmo comando com o mesmo saldo em duas instâncias e comparar `hash_estado` tick a tick.

**Acceptance Scenarios**:
1. **Given** um jogador com `materiais >= 20` e `destino == posicao`, **When** o comando é processado, **Then** `construcao_ticks` inicia em 45 e nenhum Material é debitado antes da conclusão.
2. **Given** 45 ticks decorridos sem mudança de posição e saldo mantido, **When** a canalização completa, **Then** 20 Materiais são debitados atomicamente e uma `Barreira` com 200 PV e 360 ticks é inserida.

### US2 — Recusa sem consumo parcial (P1)
Um jogador sem saldo suficiente, ou que envia `destino` diferente da sua posição, vê a construção recusada e não perde Materiais.

**Why this priority**: consumo parcial corromperia a economia e quebraria a regra canônica "sem 20 Materiais, sem barricada" (§9.3:483).

**Independent Test**: tentar construir com 19 Materiais e com `destino` distinto; conferir `CodigoRecusa::SaldoInsuficiente` e saldo inalterado.

**Acceptance Scenarios**:
1. **Given** `materiais < 20`, **When** `construir_barreira` é chamado, **Then** retorna `CodigoRecusa::SaldoInsuficiente` e `materiais` permanece igual.
2. **Given** `destino != posicao` (comparado por `ponto_igual`), **When** o comando é validado, **Then** é recusado sem iniciar canalização e sem debitar.

### US3 — Limite de duas barricadas (P1)
Um Artesão mantém no máximo duas barricadas ativas; ao concluir a terceira, a mais antiga desaparece.

**Why this priority**: é critério de aceitação canônico (§10.3:579) e impede acúmulo ilimitado de estado.

**Independent Test**: concluir três construções seguidas e conferir que a `Barreira` de menor `id` foi removida.

**Acceptance Scenarios**:
1. **Given** o jogador já possui 2 barricadas, **When** uma terceira completa, **Then** a de menor `id` (mais antiga) é removida e a nova inserida.
2. **Given** o jogador possui 0 ou 1 barricada, **When** uma construção completa, **Then** nenhuma remoção ocorre.

### US4 — Cancelamento por movimento e revalidação (P1)
Mover o jogador durante a canalização zera o progresso; o saldo é revalidado no tick de conclusão, evitando que uma construção iniciada com 20 Materiais termine após gasto alheio.

**Why this priority**: handoff movimento/canalização é a principal fonte de divergência e de débito indevido.

**Independent Test**: iniciar canalização, mover, e tentar completar; conferir `construcao_ticks == 0` e nenhuma barreira criada.

**Acceptance Scenarios**:
1. **Given** canalização em andamento, **When** `mover` altera a posição do jogador, **Then** `construcao_ticks` e `construcao_ponto` voltam ao estado neutro.
2. **Given** saldo caiu abaixo de 20 antes do tick final, **When** `fase_skills` revalida, **Then** a construção é abortada sem débito.

### US5 — Estado replicável e auditável (P2)
O hash do estado reflete canalização em curso e todas as barreiras vivas, em ordem estável, permitindo detectar divergência determinística.

**Why this priority**: é o mecanismo de verificação honesta do núcleo (Princípio VI).

**Independent Test**: construir em duas simulações idênticas e comparar `hash_estado` após cada tick.

**Acceptance Scenarios**:
1. **Given** dois jogadores com canalizações em ticks diferentes, **When** o hash é calculado, **Then** os valores de `construcao_ticks`/`construcao_ponto` entram no hash.
2. **Given** N barreiras com ids distintos, **When** o hash é calculado, **Then** todas entram na ordem de `BTreeMap`, sem dependência de ordem de inserção.

### US6 — Input e HUD do cliente (P2)
A tecla `B` (apenas no instante em que é pressionada) agenda um comando de construção no ponto atual, com `seq`/`request_id` monotônicos; o HUD informa Materiais, Barreiras e Construindo.

**Why this priority**: sem feedback e sem numeração monotônica, o jogador não percebe a canalização e o servidor não consegue deduplicar comandos.

**Independent Test**: pressionar e segurar `B`; conferir um único comando agendado e o HUD atualizado.

**Acceptance Scenarios**:
1. **Given** `B` ainda não pressionada, **When** o botão é pressionado e mantido, **Then** exatamente um comando é agendado com `destino` = posição atual e `seq`/`request_id` incrementais.
2. **Given** canalização em andamento, **When** o HUD é renderizado, **Then** exibe Materiais, contagem de Barreiras e indicação de Construindo.

### Edge Cases
- Saldo exatamente 20: construção permitida; 19: recusada sem débito.
- `destino` igual à posição por `ponto_igual`, mas com diferença de precisão: recusado.
- Dois jogadores canalizando no mesmo tick: ambos avançam (o limite de 1 canalização é **por jogador**).
- Terceira barricada de um mesmo dono com ids não contíguos: remove a de menor `id`.
- `mover` sem alterar posição: não zera canalização (a condição é mudança de posição).
- Barreira atinge 0 PV: removida antes de expirar por tempo; `ticks_restantes` a 0: removida por duração.

## Requirements

### Functional Requirements
- **FR-001**: expor a API Rust no núcleo determinístico: constantes públicas `CUSTO_BARREIRA_MATERIAIS = 20`, `TICKS_CONSTRUCAO_BARREIRA = 45`, `PV_BARREIRA = 200`, `TICKS_DURACAO_BARREIRA = 360`, `MAX_BARREIRAS_POR_JOGADOR = 2`; `Jogador` ganha `construcao_ticks: i64` e `construcao_ponto: Option<(f64,f64)>`; `Mundo` ganha `barreiras: BTreeMap<u64, Barreira>` e `proxima_barreira_id: u64`; struct pública `Barreira { id, dono, posicao, pv, ticks_restantes }`; `CodigoRecusa::SaldoInsuficiente`; `Comando.destino` é o ponto de construção. **Nenhuma mudança** em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`.
- **FR-002**: `construir_barreira` exige `destino == posicao` (via `ponto_igual`) e `materiais >= 20`; fora isso recusa com `CodigoRecusa::SaldoInsuficiente` **sem consumo parcial** e sem iniciar canalização.
- **FR-003**: `fase_skills` revalida o saldo ao completar, avança **no máximo 1 canalização por jogador por tick** (via `BTreeSet<u64>` local), zera `construcao_ticks`/`construcao_ponto` se `mover` mudou a posição, debita 20 Materiais atomicamente na conclusão e insere a `Barreira` com id de `proxima_barreira_id`.
- **FR-004**: respeitar `MAX_BARREIRAS_POR_JOGADOR = 2`: ao exceder, remover a barricada mais antiga do mesmo dono (menor `id`); cada `Barreira` nasce com `pv = 200` e `ticks_restantes = 360` e é removida ao fim da duração ou ao zerar PV.
- **FR-005**: `hash_estado` inclui `construcao_ticks` e `construcao_ponto` de cada jogador e **todas** as barreiras, iteradas na ordem de `BTreeMap`, de modo que estados deterministicamente idênticos tenham hash idêntico.
- **FR-006**: no cliente, a tecla `B` em *just pressed* agenda `construir_barreira` com `destino` = posição atual e `seq`/`request_id` monotônicos por recurso local; o HUD mostra **Materiais**, **Barreiras** e **Construindo**. Estilo obrigatório: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; commits em PT-BR com staging explícito.

### Key Entities
- **Barreira**: parede destrutível publicada no mundo; campos `id` (chave em `BTreeMap`), `dono` (jogador), `posicao`, `pv` (200 → 0 destrói) e `ticks_restantes` (360 → 0 expira).
- **Canalização do jogador**: par `construcao_ticks` (contagem regressiva de 45) e `construcao_ponto` (destino confirmado), reiniciado por mudança de posição.
- **Débito atômico**: única transição que retira 20 Materiais e insere a barreira; nunca ocorre na validação nem no movemento.
- **Recusa por saldo**: `CodigoRecusa::SaldoInsuficiente`, sem efeito colateral no inventário.
- **Comando de construção**: `Comando.destino` é o ponto de construção; `seq`/`request_id` monotônicos no cliente para deduplicação.

## Clarificações
- Valores canônicos vêm de GDD §9.3:475,483 e §10.3:574,577-579 / H4. Fôlego, alcance, recarga e dimensões são `[HIPÓTESE]` e **fora de escopo**.
- "Sem consumo parcial" significa: com menos de 20 Materiais, nada é debitado e nenhuma canalização é iniciada; o débito só acontece na conclusão, após revalidação.
- O limite de uma canalização por jogador por tick é local a `fase_skills` (`BTreeSet<u64>`), não global; jogadores diferentes progridem no mesmo tick.
- A tecla do GDD é `Q`, mas o cliente deste ciclo usa `B`: trata-se de escolha de input do protótipo, não de reabertura da regra canônica de custo/tempo/PV/duração.
- `Comando.destino` é reutilizado como ponto de construção; nenhum campo novo é adicionado ao protocolo, `crates/contratos` ou `PROTOCOL_VERSION`.
- **R6 é N/A** por constituição v1.2.2: não é dívida executada, nem regra ativa, nem simulação de issue; se invocada, PARE e escale ao SWE. A dívida de automação (`resolve-tasks.sh`/adaptador) fica registrada, sem `gh` improvisado.
- Conflito **§16.3/H4 registrado como risco, não reaberto**: §16.3 traz reduções de escala do MVP para canalização (4 s de Propaganda) e condição de vitória; H4 fixa a barricada em 1,5 s/200 PV/12 s. Este ciclo adota H4 como canônico e não altera as hipóteses de escala do MVP.

## Success Criteria
- **SC-001**: com `materiais >= 20` e `destino == posicao`, a barricada é concluída em exatamente 45 ticks e debita exatamente 20 Materiais; duas execuções com o mesmo input produzem o mesmo estado.
- **SC-002**: saldo inferior a 20 ou `destino != posicao` resulta em recusa sem qualquer débito e sem canalização; `CodigoRecusa::SaldoInsuficiente` é sinalizado no caso de saldo.
- **SC-003**: nenhum jogador avança mais de 1 canalização por tick; `mover` com mudança de posição zera a canalização; o saldo é revalidado ao completar.
- **SC-004**: toda `Barreira` nasce com 200 PV e 360 ticks; o máximo por jogador é 2 e a 3ª remove a mais antiga; remoção por PV zero e por duração observada.
- **SC-005**: `hash_estado` muda quando muda canalização ou conjunto de barreiras e é idêntico entre execuções determinísticas, com iteração em ordem de `BTreeMap`.
- **SC-006**: tecla `B` em *just pressed* gera um único comando com `seq`/`request_id` monotônicos e `destino` = posição atual; HUD exibe Materiais/Barreiras/Construindo; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`. Gates, build e confirmação visual ficam **[NV]** até execução pelo orquestrador.

## Assumptions
- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam o código Rust na fase 12; o orquestrador executa gates, build e verificações finais.
- Ambiente headless: nenhuma janela, rede, credencial ou comando `gh` é usado na redação destes artefatos.
- Os valores canônicos do GDD são estáveis para o alvo; eventuais hipóteses não são convertidas em requisito.
- `Comando.destino` já existe no contrato e pode ser reutilizado sem alterar o protocolo.

## Out of scope
Fôlego, alcance, recarga, dimensões e colisão de projéteis `[HIPÓTESE]`; ponte provisória; desafios "Defender Barricada"/"A Greve"; alterações em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, manifesto ou CI; adoção de R6, `taskstoissues`, `resolve-tasks.sh` ou simulação de issue; abertura do conflito §16.3/H4; janelas, deploy, commits/push/PRs e alterações preexistentes fora da fronteira. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: `construir_barreira` 5×5×4×4 = **400/625**, escolhido sobre ponte provisória 108/625, combate/Fôlego 48/625 e servidor+Lightyear 10/625 (ver `.specify/memory/STATE.md`). Nenhum manifesto é criado.
