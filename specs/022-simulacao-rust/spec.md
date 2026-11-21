# Feature Specification: Simulação Rust do Núcleo (`crates/sim`)

**Feature Branch**: `feature/022-simulacao-rust` · **Created**: 2026-11-20T00:08Z · **Status**: Draft · **Data:** 2026-11-20

**Input**: `.specify/feature.json` aponta `specs/022-simulacao-rust`. A ADR-004 (Accepted, 20/11/2026) autoriza a reescrita do Núcleo em Rust e o crate `insurgencia-contratos` (`crates/contratos`) já está mesclado (spec 021). Esta feature reexpressa em Rust puro, no crate `crates/sim`, o `Mundo` de `nucleo/mundo.gd` e o `ValidacaoComandos` de `nucleo/sim/validacao.gd`: relógio lógico, três fases de tick, validação autoritativa e `hash_estado` determinístico. Gate Rust de referência: `verify.sh` (`verify.sh: ALL STAGES OK`). Regras: skill `mdmv-rust-thin`. Escopo desta etapa: exclusivamente os quatro documentos de especificação; nenhum `cargo`, gate, teste, build, formatador, código Rust, stage, commit ou push.

## User Scenarios & Testing *(mandatory)*

As três histórias abaixo são priorizadas por valor e cada uma é testável de forma independente por leitura dos artefatos; nenhuma pressupõe toolchain instalada, dependência baixada, build, teste executado ou apagamento do histórico GDScript.

### User Story 1 — `Mundo` com relógio lógico e três fases de tick (Priority: P1)

Como desenvolvedor do Núcleo, preciso de um tipo `Mundo` em `crates/sim` com `tick`, `epoch`, `match_id`, `jogadores`, `comandos_pendentes` e `comandos_do_tick`, com `PASSO_SEGUNDOS = 1.0 / 30.0`, cujo `avancar_tick` executa rigorosamente as fases `prepare → skills → finish`, para que a simulação seja determinística e reproduzível sem engine.

**Why this priority**: é o núcleo da simulação; sem o relógio lógico, os jogadores, os comandos e a ordem das fases, nem a validação (US2) nem o hash (US3) têm estado sobre o qual operar.

**Independent Test**: ler a especificação e confirmar os campos do `Mundo`, o literal `PASSO_SEGUNDOS = 1.0 / 30.0` e a sequência `prepare → skills → finish` em `avancar_tick`, com `comandos_pendentes` movidos para `comandos_do_tick` na fase `prepare` e limpos ao final de `finish`, sem qualquer dependência de Bevy/Lightyear neste crate.

**Acceptance Scenarios**:

1. **Given** um `Mundo` recém-criado, **When** o tick é avançado, **Then** as fases ocorrem na ordem `prepare → skills → finish` e `tick` é incrementado exatamente ao final, com `PASSO_SEGUNDOS = 1.0 / 30.0` preservado do GDScript.
2. **Given** comandos agendados, **When** `prepare` executa, **Then** `comandos_do_tick` recebe uma cópia de `comandos_pendentes`, que é esvaziada, e ao final de `finish` `comandos_do_tick` é limpo — nenhum comando reaparece no tick seguinte.
3. **Given** `epoch` e `match_id` inicializados, **When** jogadores são adicionados e removidos, **Then** o estado permanece consistente e nenhum tipo de engine é introduzido em `crates/sim`.

---

### User Story 2 — Validação autoritativa com cinco códigos (Priority: P2)

Como servidor autoritativo, preciso de `ValidacaoComandos` em `crates/sim` com `LIMITE_MOVIMENTO_M = 1.0` e exatamente cinco códigos — `PEER_NAO_AUTORIZADO`, `SEQ_OU_REQUEST_REPLAY`, `ACAO_DESCONHECIDA`, `MOVIMENTO_FORA_DO_LIMITE` e `OK` —, para que intenções forjadas por cliente (peer desconhecido, replay de sequência, ação inventada, salto de movimento) sejam recusadas antes de alterar o estado.

**Why this priority**: é a fronteira de confiança do servidor; depende de US1 para existir, mas é testável isoladamente pela leitura dos cinco códigos, das regras e do limite de 1.0 metro.

**Independent Test**: comparar os cinco códigos literais e `LIMITE_MOVIMENTO_M = 1.0` com `specs/009-servidor-validacao/spec.md:FR-005` e `nucleo/sim/validacao.gd`, confirmando que `OK` só é devolvido quando nenhuma recusa se aplica e que nenhum código é renomeado, fundido ou omitido.

**Acceptance Scenarios**:

1. **Given** um `peer_id` ausente de `jogadores`, **When** o comando é validado, **Then** o código é `PEER_NAO_AUTORIZADO`.
2. **Given** `seq` ou `request_id` menor ou igual ao registrado no jogador, **When** o comando é validado, **Then** o código é `SEQ_OU_REQUEST_REPLAY`.
3. **Given** uma ação fora de `ACOES_PERMITIDAS`, **When** o comando é validado, **Then** o código é `ACAO_DESCONHECIDA`.
4. **Given** uma ação `mover` cujo deslocamento excede `LIMITE_MOVIMENTO_M = 1.0`, **When** o comando é validado, **Then** o código é `MOVIMENTO_FORA_DO_LIMITE`; dentro do limite, o código é `OK`.

---

### User Story 3 — `hash_estado` determinístico (Priority: P3)

Como mantenedor, preciso que `hash_estado` em `crates/sim` produza um SHA-256 determinístico do estado (`jogadores`, `tick` e demais componentes da simulação), para que dois mundos alimentados pelas mesmas intenções gerem o mesmo hash e sirvam de golden de paridade entre GDScript e Rust.

**Why this priority**: é a prova de equivalência entre as implementações; depende de US1 e US2 para ter estado, mas é auditável de forma isolada pela propriedade "estados iguais ⇒ hash igual".

**Independent Test**: ler a especificação e confirmar que `hash_estado` serializa os mesmos componentes de `nucleo/mundo.gd:hash_estado` numa ordem fixa e devolve 64 caracteres hex, sem aleatoriedade, sem relógio de parede e sem tempo de ponto flutuante não controlado.

**Acceptance Scenarios**:

1. **Given** dois `Mundo` com `jogadores`, `tick` e componentes idênticos, **When** `hash_estado` é chamado em ambos, **Then** os dois hashes SHA-256 de 64 hex são iguais.
2. **Given** uma alteração qualquer no estado, **When** `hash_estado` é recalculado, **Then** o hash muda — nenhuma chave de estado é silenciosamente ignorada.

---

### Edge Cases

- `tick`, `epoch`, `match_id`, `seq` e `request_id` como tempo inteiro do servidor: nenhum `f64` usado como relógio; `PASSO_SEGUNDOS = 1.0 / 30.0` é preservado como literais de referência (`1`/`30`), não como relógio de ponto flutuante (Q2).
- `mover` com deslocamento exatamente igual a `LIMITE_MOVIMENTO_M = 1.0` é aceito (`OK`); acima de `1.0`, recusado com `MOVIMENTO_FORA_DO_LIMITE`.
- `seq` ou `request_id` repetidos ou regressivos são recusados mesmo que a ação seja válida; a ordem de verificação é peer → replay → ação → movimento, preservando o GDScript.
- Campo ausente ou tipo divergente em `destino` → recusa determinística, sem pânico e sem `.unwrap()`/`.expect()` (Q3).
- `crates/sim` depende somente de `insurgencia-contratos` e da biblioteca padrão; nenhum `bevy`, `lightyear` ou runtime de engine é acrescentado (Q1).
- Alegação de `cargo check`/`clippy`/`nextest`/`fmt` executados nesta etapa → não observado; permanece **[NV]** (SC-001).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O crate `crates/sim` DEVE ser materializado com `Cargo.toml` próprio, declarando dependência explícita e fixada de `insurgencia-contratos` (nome de pacote `insurgencia-contratos`) e SEM dependência de engine (`bevy`, `lightyear` ou equivalente). A dependência é apenas a camada de contratos já mesclada; nenhuma versão flutuante (`*`) ou intervalo aberto é admitido e toda dependência tem justificativa registrada (Q1).
- **FR-002**: `Mundo` DEVE expor `tick`, `epoch`, `match_id`, `jogadores`, `comandos_pendentes` e `comandos_do_tick`, com `PASSO_SEGUNDOS = 1.0 / 30.0` preservado do GDScript, além das operações de adicionar/remover jogador e agendar comando; `epoch` e `match_id` iniciam em `1` e `tick` em `0`.
- **FR-003**: `avancar_tick` DEVE executar exatamente três fases na ordem `prepare → skills → finish`: `prepare` copia `comandos_pendentes` para `comandos_do_tick` e esvazia os pendentes; `skills` aplica as ações permitidas; `finish` aplica os efeitos de fim de tick e limpa `comandos_do_tick`; `tick` é incrementado ao final.
- **FR-004**: `ValidacaoComandos` DEVE expor `ACOES_PERMITIDAS`, `LIMITE_MOVIMENTO_M = 1.0` e exatamente cinco códigos — `PEER_NAO_AUTORIZADO`, `SEQ_OU_REQUEST_REPLAY`, `ACAO_DESCONHECIDA`, `MOVIMENTO_FORA_DO_LIMITE` e `OK` — com os mesmos nomes literais de `specs/009-servidor-validacao/spec.md:FR-005`, sem renomear, fundir ou omitir nenhum.
- **FR-005**: A validação DEVE recusar, nesta ordem: `peer_id` ausente de `jogadores` (`PEER_NAO_AUTORIZADO`); `seq` ou `request_id` menor ou igual ao registrado (`SEQ_OU_REQUEST_REPLAY`); ação fora de `ACOES_PERMITIDAS` (`ACAO_DESCONHECIDA`); deslocamento de `mover` acima de `LIMITE_MOVIMENTO_M = 1.0` (`MOVIMENTO_FORA_DO_LIMITE`). Somente quando nenhuma recusa se aplica o código DEVE ser `OK`.
- **FR-006**: `hash_estado` DEVE calcular SHA-256 determinístico, serializando `jogadores`, `tick` e os componentes da simulação em ordem fixa e devolvendo 64 caracteres hex; estados iguais DEVEM produzir hash igual e qualquer alteração de estado DEVE alterar o hash (Q2).
- **FR-007**: Todo código de produção Rust DEVE usar `Tipos Irish` — tipos ricos do domínio (novos tipos/newtypes e enums para `Tick`, `Epoch`, `MatchId`, `PeerId`, `Fase`, `CodigoValidacao`, `Acao`) em vez de tipos primitivos soltos — e DEVE tratar erros com `Result<T, E>`, `?` e `thiserror`, com `unsafe_code = "forbid"`, `clippy::pedantic = "deny"` e `unwrap_used`/`expect_used` em `deny`. É proibido alterar `Cargo.toml`/`clippy.toml` para desativar lints; única exceção admitida: `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado (Q3, Q4).
- **FR-008**: O gate `verify.sh` DEVE permanecer a única fonte de verdade local, com quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — terminando com `verify.sh: ALL STAGES OK`. Os cinco gates determinísticos alvo (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`), `Makefile`, baselines e `evidence.csv` DEVEM permanecer inalterados. Nenhum comando é executado nesta etapa; R4 (`validate`) precede R7 (`release`) e nenhum artefato dessas portarias é escrito aqui.

### Key Entities *(include if feature involves data)*

- **`Mundo`**: estado da simulação em `crates/sim` — `tick`, `epoch`, `match_id`, `jogadores`, `comandos_pendentes`, `comandos_do_tick` e `PASSO_SEGUNDOS = 1.0 / 30.0`; avança em três fases.
- **`Comando`**: intenção de um peer (`peer_id`, `acao`, `seq`, `request_id`, `destino`) agendada em `comandos_pendentes` e movida para `comandos_do_tick` na fase `prepare`.
- **`Jogador`**: entrada em `jogadores` indexada por `peer_id`, com posição, vida, `seq` e `request_id` usados pela validação.
- **`ValidacaoComandos` / `CodigoValidacao`**: regra pura que devolve um de cinco códigos — `PEER_NAO_AUTORIZADO`, `SEQ_OU_REQUEST_REPLAY`, `ACAO_DESCONHECIDA`, `MOVIMENTO_FORA_DO_LIMITE`, `OK` — com `LIMITE_MOVIMENTO_M = 1.0`.
- **`hash_estado`**: SHA-256 de 64 hex sobre a serialização determinística do estado; golden de paridade entre GDScript e Rust.
- **`Tipos Irish`**: convenção de modelagem com tipos ricos (newtypes e enums) para os conceitos do domínio, sem engine e sem obsessão por primitivos.
- **Crate `insurgencia-sim`**: unidade em `crates/sim`, dependente de `insurgencia-contratos`, sem engine; reunida sob o mesmo workspace dos spec 021.

## Success Criteria *(mandatory)*

Critérios a aferir na implementação e em R4. Nenhum comando de `cargo`, gate, teste, build ou formatador foi executado nesta etapa de especificação; as metas abaixo são alvos, não resultados observados aqui.

### Measurable Outcomes

- **SC-001**: Na implementação/R4, `verify.sh` termina com `verify.sh: ALL STAGES OK` e 4 de 4 estágios OK, na ordem `cargo check --locked --workspace --all-targets` → `cargo clippy --workspace --all-targets --all-features -- -D warnings` → `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) → `cargo fmt --all -- --check`, com 0 estágios vermelhos. (baseline: `crates/sim` inexistente; meta: 4 de 4; nesta etapa: **[NV]**)
- **SC-002**: `ValidacaoComandos` expõe 5 de 5 códigos com os literais exatos `PEER_NAO_AUTORIZADO`, `SEQ_OU_REQUEST_REPLAY`, `ACAO_DESCONHECIDA`, `MOVIMENTO_FORA_DO_LIMITE` e `OK`, e 0 códigos renomeados ou ausentes frente a `specs/009-servidor-validacao/spec.md:FR-005`, com `LIMITE_MOVIMENTO_M = 1.0`. (baseline: 0 no Rust; meta: 5 de 5)
- **SC-003**: `avancar_tick` executa 3 de 3 fases na ordem exata `prepare → skills → finish`, com `PASSO_SEGUNDOS = 1.0 / 30.0` preservado, 0 fases fora de ordem e 0 comandos persistidos entre ticks. (baseline: 0 no Rust; meta: 3 de 3)
- **SC-004**: Para estados iguais, `hash_estado` produz hashes iguais em 100% dos casos (mesmo SHA-256 de 64 hex); para estados diferentes, 0 colisões por omissão de campo. (baseline: 0 hashes no Rust; meta: 100% de determinismo)
- **SC-005**: 5 de 5 gates determinísticos alvo permanecem PASS e inalterados — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e `Makefile`, baselines e `evidence.csv` não são modificados. (baseline: 5 de 5 registrados em `.specify/memory/STATE.md`; meta: 5 de 5)

## Assumptions

- A ADR-004 (Accepted, 20/11/2026) é a autoridade para a linguagem/engine; `crates/contratos` (`insurgencia-contratos`) já está mesclado pela spec 021 e é a única dependência interna deste crate.
- O `Mundo` de referência é `nucleo/mundo.gd` e a validação de referência é `nucleo/sim/validacao.gd`; nomes, literais e ordem de fases são preservados, e divergências são registradas, não niveladas por inferência.
- Convenção de escrita: português brasileiro, termos estrangeiros explicados na 1ª ocorrência, sem TBD/TODO/placeholder; saídas não observadas permanecem **[NV]**; cada agente escreve somente seus próprios arquivos.
- Nenhum `cargo`, gate, teste, build, formatador, `make`, Godot, stage, commit ou push é executado nesta etapa; registro com data/hora pertence a R4, e versão, changelog com FR/T e rollback a R7; R4 precede R7.

## Clarifications

### Session 2026-11-20

- **Q1: `crates/sim` pode depender de engine (Bevy/Lightyear)?** → A: Não. Este crate é lógica de domínio pura e depende somente de `insurgencia-contratos` e da biblioteca padrão; `bevy`/`lightyear` ficam nos crates de servidor/aplicação da spec 021. Nenhuma versão flutuante ou substituição automática de pacote similar. (fontes: ADR-004; `specs/021-migracao-bevy/spec.md`; skill `mdmv-rust-thin`)
- **Q2: Como representar o tempo do servidor — ponto flutuante ou inteiro?** → A: tempo inteiro do servidor. `tick`, `epoch`, `match_id`, `seq` e `request_id` são inteiros; `PASSO_SEGUNDOS = 1.0 / 30.0` é preservado como literais de referência (`1`/`30`), não como relógio `f64`. Isso elimina não determinismo por arredondamento e permite `hash_estado` estável. (fontes: `nucleo/mundo.gd` — `PASSO_SEGUNDOS`, `avancar_tick`, `hash_estado`; `specs/009-servidor-validacao/spec.md:FR-001,FR-007`)
- **Q3: Como tratar erros sem `.unwrap()`/`.expect()`?** → A: proibição absoluta — `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, `unwrap_used` e `expect_used` em `deny`; erro é `Result<T, E>` com `?` e `thiserror`. Não se altera `Cargo.toml`/`clippy.toml` para desativar lints; única exceção admitida é `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado. (fontes: skill `mdmv-rust-thin`; `specs/021-migracao-bevy/spec.md:Q3`)
- **Q4: Como modelar os tipos do domínio neste crate (`Tipos Irish`)?** → A: adotar `Tipos Irish` — tipos ricos (novos tipos/newtypes e enums) para `Tick`, `Epoch`, `MatchId`, `PeerId`, `Fase` (`Prepare`/`Skills`/`Finish`), `CodigoValidacao` (5 variantes) e `Acao`, em vez de primitivos soltos. Isso torna estados inválidos inexprimíveis, preserva os literais do GDScript e mantém o crate sem dependência de engine (Q1). (fontes: `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `specs/009-servidor-validacao/spec.md:FR-005`)

0 perguntas ao usuário; as quatro ambiguidades foram resolvidas por evidência local (ADR-004, spec 021, GDScript de referência e skill `mdmv-rust-thin`). Cada esclarecimento foi formulado e resolvido isoladamente; nenhum marcador `[NEEDS CLARIFICATION]` permaneceu em aberto.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.
>
> Esta feature não produz manifest de orçamento de contexto nem saídas com handles de contexto. A verificação é a dos quatro estágios de `verify.sh` e dos cinco gates determinísticos alvo, conforme SC-001/SC-005, aferida na implementação e em R4 — nunca nesta etapa de especificação. Nenhum artefato R4/R7 é escrito aqui.
