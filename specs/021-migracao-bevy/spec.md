# Feature Specification: Migração do Núcleo para Bevy 0.20.1 + Lightyear 0.31

**Feature Branch**: `feature/021-migracao-bevy` · **Created**: 2026-11-20T23:57Z · **Status**: Draft · **Data:** 2026-11-20

**Input**: ADR-004 (`docs/adr/004-migracao-bevy-lightyear.md`, Accepted, 20/11/2026) autoriza a reescrita do Núcleo de Godot/GDScript para Rust com Bevy 0.20.1 + Lightyear 0.31.0 e supersede a ADR-001 na parte de linguagem/engine; o restante da ADR-001 permanece como registro histórico. Base técnica: `cargo info bevy@0.20.1` (20/11/2026) declara `rust-version: 1.97.1` no crate publicado, atendido pela toolchain local `stable 1.99.1` registrada na ADR-004. Contratos de origem: `nucleo/contratos/protocolo.gd` e `nucleo/contratos/carregador_modulo.gd`. Gate Rust de referência: `verify.sh`. `.specify/feature.json` aponta `specs/021-migracao-bevy`. Escopo desta etapa: somente os quatro documentos de especificação; nenhum `cargo`, gate, teste, build, formatador, `make`, Godot, stage, commit ou push é executado aqui. Toda saída não observada permanece **[NV]** até evidência datada em R4.

## User Scenarios & Testing *(mandatory)*

As três histórias abaixo são priorizadas por valor e cada uma é testável de forma independente por leitura dos artefatos; nenhuma pressupõe toolchain instalada, dependência baixada, build, teste executado ou apagamento do histórico GDScript.

### User Story 1 — Materializar workspace Rust com `Cargo.toml` e `crates/` (Priority: P1)

Como mantenedor do Insurgência, preciso de um `Cargo.toml` de workspace na raiz e de um diretório `crates/` com os módulos `contratos`, `dominio/sim`, `servidor` e `aplicacao`, para que o Núcleo e o servidor passem a ser compilados em Rust com uma única toolchain, versões fixadas e layout Rust-only conforme `projetos/AGENTS.md`.

**Why this priority**: sem o workspace e o layout canônico, nenhum outro artefato Rust (contratos, gate, testes) tem onde ser materializado; é o pré-requisito estrutural de US2 e US3.

**Independent Test**: ler `Cargo.toml` na raiz e confirmar `[workspace]`, `members` apontando para `crates/*`, `resolver = "2"` e `edition = "2024"`; listar `crates/` e confirmar os quatro crates com seu próprio `Cargo.toml`, sem diretórios `libs/domain` ou `libs/infra` e sem versão flutuante (`*`) ou intervalo aberto.

**Acceptance Scenarios**:

1. **Given** a árvore atual sem `Cargo.toml`, **When** o workspace é materializado, **Then** `Cargo.toml` declara `[workspace]` com `members = ["crates/*"]`, `resolver = "2"`, `edition = "2024"` e `crates/` contém `contratos`, `dominio/sim`, `servidor` e `aplicacao`.
2. **Given** a necessidade de uma dependência de terceiros (Bevy 0.20.1, Lightyear 0.31.0, `thiserror`, `anyhow`), **When** ela é declarada, **Then** a versão é fixada explicitamente, sem `*` ou intervalo aberto, com justificativa registrada, e nenhum pacote similar é substituído automaticamente (anti-*slopsquatting*).

---

### User Story 2 — Portar o Protocolo e os contratos versionados para `crates/contratos` (Priority: P2)

Como desenvolvedor, preciso que `crates/contratos` contenha o equivalente Rust do `Protocolo` e do `CarregadorModulo` do GDScript — `PROTOCOL_VERSION = 1`, `MODULO_ID`, `TipoPacote` (JOIN=1 a EVENTO_DESAFIO=6), `SnapshotMeta` com `to_dict`, `normalizar_pacote`, os quatro códigos de recusa e a validação de `content_hash` com 64 hex —, para que a migração preserve o contrato de rede e de módulo sem divergência de nomes, valores ou formatos.

**Why this priority**: é a camada de compatibilidade entre o histórico GDScript e o novo Núcleo Rust; sem equivalência exata, servidor e cliente não concordam sobre versão, tipos de pacote, recusas e hash.

**Independent Test**: comparar as constantes, enumerações e formatos de `crates/contratos` com `nucleo/contratos/protocolo.gd:4-26,28-57` e `nucleo/contratos/carregador_modulo.gd:4-43`, conferindo os quatro códigos `VERSAO_PROTOCOLO_DIVERGENTE`, `HASH_MODULO_DIVERGENTE`, `MODULO_AUSENTE`, `MANIFEST_INVALID` e a rejeição de hash com 63/65 hex; nenhum comando é executado nesta etapa.

**Acceptance Scenarios**:

1. **Given** `nucleo/contratos/protocolo.gd`, **When** o equivalente Rust é escrito em `crates/contratos`, **Then** `PROTOCOL_VERSION = 1`, `MODULO_ID = "insurgencia.exemplo"`, os seis valores de `TipoPacote` (1–6), os campos de `PADROES`, `SnapshotMeta.to_dict` e `normalizar_pacote` preservam nomes, valores e comportamento de ignorar chaves desconhecidas.
2. **Given** `nucleo/contratos/carregador_modulo.gd`, **When** um módulo é validado em Rust, **Then** os quatro códigos de recusa são devolvidos exatamente como no GDScript e `content_hash` só é aceito com 64 caracteres hex (SHA-256), com `content_bytes` estritamente entre 1 e 25.000.000.

---

### User Story 3 — Adotar `verify.sh` como gate Rust preservando os cinco gates alvo (Priority: P3)

Como mantenedor, preciso que `verify.sh` continue sendo a única fonte de verdade do gate local, com os quatro estágios `cargo check`, `cargo clippy`, `cargo nextest run` (fallback `cargo test`) e `cargo fmt --check`, e que os cinco gates determinísticos do alvo permaneçam intactos, para que a migração não reduza a proteção de governança nem troque a ordem de validação.

**Why this priority**: é a garantia de que a mudança de linguagem não enfraquece o mecanismo de verificação existente; depende de US1 para existir e de US2 para ter o que verificar, mas é independentemente auditável pela leitura dos scripts.

**Independent Test**: ler `verify.sh` e confirmar a sequência dos quatro estágios, os modos `--fast`, `--install-hook` e `--uninstall-hook` e a ausência de alteração de lints; ler `scripts/ci/` e confirmar a presença inalterada dos cinco gates Python. Nenhum desses comandos é executado nesta etapa; a saída permanece **[NV]**.

**Acceptance Scenarios**:

1. **Given** o `verify.sh` atual, **When** a migração é especificada, **Then** o gate Rust mantém quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — e falha no primeiro estágio vermelho.
2. **Given** os cinco gates determinísticos do alvo, **When** o workspace Rust é introduzido, **Then** `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only` e `hub_docs_gate` permanecem presentes e inalterados, e nenhum baseline, `evidence.csv` ou `Makefile` é modificado.

---

### Edge Cases

- `rust-version` divergente: `main` do Bevy registra `1.100.0`, mas o crate publicado `bevy@0.20.1` declara `1.97.1` — usar o valor publicado (Q1), nunca inferir valor intermediário.
- Versão 0.x do Bevy: churn de API aproximadamente a cada 3 meses; toda dependência é fixada, sem `*` ou intervalo aberto, e a justificativa cabe no commit (FR-002).
- Lightyear diverge entre a UI (`0.30.0`) e `main`/README (`0.31.0`); a ADR-004 fixa `0.31.0` — registrar a divergência, não usar a versão da UI (Q2).
- Hash com 63 ou 65 caracteres, ou fora do alfabeto hexadecimal → recusar; nenhum hash é aceito por correspondência parcial (FR-005, SC-003).
- `content_bytes` igual a 0 ou acima de 25.000.000 → recusar com o mesmo critério do carregador GDScript.
- Campo obrigatório ausente, campo desconhecido ou tipo divergente → recusa determinística, sem pânico, sem `.unwrap()`/`.expect()` (FR-006, SC-004).
- `cargo nextest` indisponível → fallback documentado para `cargo test --locked --workspace`; o fallback não substitui o gate completo por `--fast`.
- Tentativa de desativar `unsafe_code = "forbid"` ou `clippy::pedantic = "deny"` em `Cargo.toml`/`clippy.toml` para fazer o gate passar → fora de escopo; corrigir o código, nunca o lint.
- Tentativa de apagar os `.gd` do histórico do Git → fora de escopo; os 31 arquivos permanecem como registro histórico versionado (Q4).
- Alegação de `cargo check`/`clippy`/`nextest`/`fmt` executado nesta etapa → não observado; permanece **[NV]** (SC-001).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O workspace Rust DEVE ser materializado com `Cargo.toml` na raiz contendo `[workspace]`, `members = ["crates/*"]`, `resolver = "2"` e `edition = "2024"`, e DEVE adotar a toolchain `stable 1.99.1` registrada na ADR-004 como suficiente para o MSRV publicado `1.97.1` do `bevy@0.20.1`.
- **FR-002**: O diretório `crates/` DEVE conter os módulos `contratos`, `dominio/sim`, `servidor` e `aplicacao`, cada um com seu `Cargo.toml`, seguindo o layout Rust-only de `projetos/AGENTS.md` (sem `libs/domain`/`libs/infra`). Toda dependência (Bevy 0.20.1, Lightyear 0.31.0, `thiserror` em libs, `anyhow` em apps) DEVE ter versão fixada explicitamente e justificativa no commit, sem `*` ou intervalo aberto e sem substituição automática de pacote similar.
- **FR-003**: `crates/contratos` DEVE conter o equivalente Rust de `nucleo/contratos/protocolo.gd`: `PROTOCOL_VERSION = 1`, `MODULO_ID = "insurgencia.exemplo"`, `TipoPacote` com JOIN=1, JOIN_ACEITO=2, JOIN_RECUSADO=3, COMANDO=4, SNAPSHOT=5, EVENTO_DESAFIO=6, os quatro conjuntos de campos de `PADROES`, `SnapshotMeta` com os mesmos cinco campos e `to_dict`, e `normalizar_pacote` preservando somente chaves conhecidas e injetando `tipo`.
- **FR-004**: O contrato DEVE preservar exatamente os quatro códigos de recusa do GDScript — `VERSAO_PROTOCOLO_DIVERGENTE`, `HASH_MODULO_DIVERGENTE`, `MODULO_AUSENTE` e `MANIFEST_INVALID` — como constantes ou variantes de erro equivalentes, com os mesmos nomes e valores literais.
- **FR-005**: A validação de módulo DEVE aceitar `content_hash` somente como SHA-256 de 64 caracteres hex e `content_bytes` estritamente entre 1 e 25.000.000, preservando os critérios de `carregador_modulo.gd:36-43` e devendo devolver `Result<T, E>` (com `thiserror` em libs) em vez de dicionários de erro mutáveis.
- **FR-006**: Todo código de produção Rust DEVE declarar `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, avisos fatais e proibição de `.unwrap()`/`.expect()`; erros DEVEM ser tratados com `Result<T, E>`, `?`, `thiserror` em libs e `anyhow` em apps. É proibido alterar `Cargo.toml` ou `clippy.toml` para desativar lints; exceção de proc-macro, se inevitável, usa estritamente `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]`.
- **FR-007**: `verify.sh` DEVE permanecer a única fonte de verdade do gate local Rust, com quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — preservando os modos `--fast`, `--install-hook` e `--uninstall-hook` e falhando no primeiro estágio vermelho.
- **FR-008**: Os cinco gates determinísticos do alvo (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) DEVEM permanecer presentes e inalterados, assim como `Makefile`, baselines e `evidence.csv`. Os 31 arquivos `.gd` DEVEM permanecer no histórico do Git como registro versionado. Nenhum `cargo`, gate, teste, build, formatador, stage, commit ou push é executado nesta etapa de especificação; R4 (`validate`) precede R7 (`release`) e nenhum artefato dessas portarias é escrito aqui.

### Key Entities *(include if feature involves data)*

- **Workspace**: `Cargo.toml` raiz com `[workspace]`, `members = ["crates/*"]`, `resolver = "2"` e `edition = "2024"`; une os quatro crates sob uma única toolchain.
- **Crate Rust-only**: unidade em `crates/` (`contratos`, `dominio/sim`, `servidor`, `aplicacao`) com `Cargo.toml` próprio e versões fixadas; substitui o layout `libs/domain`/`libs/infra` conforme `projetos/AGENTS.md`.
- **Protocolo**: contrato versionado em `crates/contratos` com `PROTOCOL_VERSION = 1`, `MODULO_ID`, `TipoPacote` (1–6), `PADROES`, `SnapshotMeta` e `normalizar_pacote`; identidade definida pelos nomes/valores do GDScript de origem.
- **Código de recusa**: uma das quatro constantes `VERSAO_PROTOCOLO_DIVERGENTE`, `HASH_MODULO_DIVERGENTE`, `MODULO_AUSENTE`, `MANIFEST_INVALID`; devolvido como erro tipado, nunca como pânico.
- **Hash de conteúdo**: SHA-256 de 64 caracteres hex associado a `content_bytes` (1–25.000.000); valida a identidade do módulo.
- **Gate Rust**: `verify.sh` com quatro estágios (check → clippy → nextest → fmt), único determinismo local para o código Rust.
- **Gates alvo**: os cinco scripts Python de governança, preservados como condição de não regressão.

## Success Criteria *(mandatory)*

Critérios a aferir na implementação e em R4. Nenhum comando de `cargo`, gate, teste, build ou formatador foi executado nesta etapa de especificação; as metas abaixo são alvos, não resultados observados aqui.

### Measurable Outcomes

- **SC-001**: Na implementação/R4, os quatro estágios de `verify.sh` terminam com exit `0` e saída auditável — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — com 0 estágios vermelhos. (baseline: workspace Rust inexistente; meta: 4 de 4 estágios OK; nesta etapa: **[NV]**)
- **SC-002**: `crates/contratos` expõe 4 de 4 códigos de recusa com os literais exatos `VERSAO_PROTOCOLO_DIVERGENTE`, `HASH_MODULO_DIVERGENTE`, `MODULO_AUSENTE` e `MANIFEST_INVALID`, e 0 códigos renomeados ou ausentes frente a `protocolo.gd:16-19`. (baseline: 0 no Rust; meta: 4 de 4)
- **SC-003**: 100% das validações de `content_hash` aceitam somente 64 caracteres hex; 0 casos com 63, 65 ou caracteres inválidos aceitos, e `content_bytes` fora de 1–25.000.000 é sempre recusado. (baseline: 0 validações Rust; meta: 100% conforme `carregador_modulo.gd:36-43`)
- **SC-004**: 0 ocorrências de `unsafe`, `.unwrap()` ou `.expect()` no código de produção Rust, com `unsafe_code = "forbid"`, `clippy::pedantic = "deny"` e `unwrap_used`/`expect_used` em nível `deny`; 0 alterações de lint para burlar o gate. (baseline: 0 arquivos Rust; meta: 0 ocorrências)
- **SC-005**: 5 de 5 gates determinísticos alvo permanecem PASS e inalterados — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e `make check`/`verify.sh` não têm alvo, baseline ou `evidence.csv` modificado. (baseline: 5 de 5 registrados em `.specify/memory/STATE.md:87,116-120`; meta: 5 de 5)

## Assumptions

- A ADR-004 (Accepted, 20/11/2026) é a autoridade para a mudança de linguagem/engine; a ADR-001 permanece como registro histórico nos demais pontos, e nenhuma troca adicional de engine é permitida sem novo ADR.
- O MSRV a considerar é o do crate publicado (`bevy@0.20.1` → `rust-version: 1.97.1`), não o valor `1.100.0` lido no `main` do repositório; a toolchain local `stable 1.99.1` registrada na ADR-004 o atende. Nenhum comando foi executado nesta etapa para reconfirmar isso.
- O layout é Rust-only conforme `projetos/AGENTS.md`: `crates/`, não `libs/domain` + `libs/infra`; os 31 arquivos GDScript viram registro histórico versionado, sem remoção do Git.
- A equivalência de contratos é definida pelos arquivos `nucleo/contratos/protocolo.gd` e `nucleo/contratos/carregador_modulo.gd`; nomes, valores literais e formatos são preservados, e divergências são registradas, não niveladas por inferência.
- Convenção de escrita: português brasileiro, termos estrangeiros explicados na 1ª ocorrência, tabelas para dados estruturados, sem TBD/TODO/placeholder; cada agente escreve somente seus próprios arquivos.
- Nenhum `cargo`, gate, teste, build, formatador, `make`, Godot, stage, commit ou push é executado nesta etapa; saídas não observadas permanecem **[NV]** até R4, e R4 precede R7.

## Clarifications

### Session 2026-11-20

- **Q1: Qual MSRV fixar, dado que o `main` do Bevy declara 1.100.0?** → A: fixar o MSRV **1.97.1 publicado**, porque `cargo info bevy@0.20.1` (20/11/2026) declara `rust-version: 1.97.1` no crate efetivamente publicado, e a toolchain local `stable 1.99.1` o atende. O valor `1.100.0` do `main` e a divergência são registrados na ADR-004, nunca usados como requisito, e nenhum comando foi executado nesta etapa para reconfirmá-los. (fontes: `docs/adr/004-migracao-bevy-lightyear.md:13`; `specs/020-engines-rust/spec.md:115`)
- **Q2: Qual versão de Lightyear usar, dado o `0.x` e a divergência UI/`main`?** → A: fixar **Lightyear 0.31.0** exatamente, conforme ADR-004, e não a versão `0.30.0` mostrada na UI/relatório anterior. Toda dependência é fixada sem `*` ou intervalo aberto, e a justificativa é registrada no commit; nenhum pacote similar é substituído automaticamente. (fontes: `docs/adr/004-migracao-bevy-lightyear.md:13,17,24`; `specs/020-engines-rust/spec.md:59`)
- **Q3: Como tratar erros sem `.unwrap()`/`.expect()`?** → A: proibição absoluta — `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, `unwrap_used` e `expect_used` em `deny`; erro é `Result<T, E>` com `?`, `thiserror` em libs e `anyhow` em apps. Não se altera `Cargo.toml`/`clippy.toml` para desativar lints; a única exceção admitida é `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado. (fontes: skill `mdmv-rust-thin`; `docs/adr/004-migracao-bevy-lightyear.md:19-24`)
- **Q4: O que acontece com o GDScript histórico?** → A: os 31 arquivos `.gd` permanecem versionados no Git como registro histórico; a implementação passa a ser Rust-only em `crates/` e os contratos são reexpressos em Rust com nomes/valores equivalentes. Nenhum arquivo GDScript é apagado, nenhuma baseline/gate é alterado e nenhuma alegação de execução é feita nesta etapa. (fontes: `docs/adr/004-migracao-bevy-lightyear.md:29,31,42`; `projetos/AGENTS.md` — layout Rust-only)

0 perguntas ao usuário; as quatro ambiguidades foram resolvidas por evidência local (ADR-004, contratos GDScript, skill `mdmv-rust-thin` e `projetos/AGENTS.md`). Cada esclarecimento foi formulado e resolvido isoladamente; nenhum marcador `[NEEDS CLARIFICATION]` permaneceu em aberto.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.
>
> Esta feature não produz manifest de orçamento de contexto nem saídas com handles de contexto. A verificação é a dos quatro estágios de `verify.sh` e dos cinco gates determinísticos alvo, conforme SC-001/SC-005, aferida na implementação e em R4 — nunca nesta etapa de especificação. Nenhum artefato R4/R7 é escrito aqui.
