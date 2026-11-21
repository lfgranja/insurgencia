# Feature Specification: Soberania e Exportação — conta/personagem, `COPYRIGHT.txt` e DCO (`crates/exportacao`)

**Feature Branch**: `feature/025-soberania-exportacao` · **Created**: 2026-11-21T01:19Z · **Status**: Draft · **Data:** 2026-11-21

**Input**: `.specify/feature.json` aponta `specs/025-soberania-exportacao`. `.specify/memory/STATE.md` (estado vivo 2026-11-21T00:22Z) fixa o item 4 da prioridade da SE: **exportação de conta/personagem, `COPYRIGHT.txt` e commits assinados (GDD §15.9)**. A GDD §15.5 decide que "exportação de dados é requisito do **Núcleo**, não cortesia do anfitrião" e que nenhuma instância guarda a chave de exportação (GDD:1395); a §15.7 exige "arquivo legível, verificável por ferramenta independente"; a §15.9 exige `LICENSE` e `COPYRIGHT.txt` presentes, DCO/CLA com "todo *commit* assinado; contribuição sem assinatura é recusada" (GDD:1452), arquivo de atribuição com fonte/licença/data e versão do protocolo com *hash* do módulo recusando divergência no *handshake*. A licença do Núcleo e do servidor headless é MIT (ADR-002). Ambiente verificado: **nenhuma chave GPG**, `commit.gpgsign` e `user.signingkey` ausentes — a assinatura de origem adotada é o DCO via trailer `Signed-off-by`, verificável sem GPG (Q2). As specs 021–024 entregaram contratos, Núcleo, cliente greybox e instrumentação; o workspace raiz declara `members = ["crates/*"]` e `verify.sh` é a única fonte de verdade do gate Rust. Escopo desta etapa: exclusivamente os quatro documentos de especificação — nenhum `cargo`, gate, teste, build, formatador, código Rust, stage, commit ou push. Nenhuma saída é afirmada como observada aqui; todo resultado permanece **[NV]** até registro datado em R4.

## User Scenarios & Testing *(mandatory)*

As quatro histórias abaixo são ordenadas por dependência estrutural. Cada uma é testável de forma independente por leitura dos artefatos; nenhuma pressupõe toolchain instalada, gate executado, commit criado ou alegação de execução observada nesta etapa.

### User Story 1 — Exportação local de conta e personagem em JSON versionado (Priority: P1)

Como jogador soberano, preciso exportar minha conta e meu personagem para um arquivo JSON versionado e legível, sem depender de servidor, instância ou registro externo, para que eu possa sair de qualquer instância levando meus dados (GDD §15.5, §15.7, GDD:1395).

**Why this priority**: é a própria definição de soberania do jogador no MVP — "Conta, personagem e histórico são exportáveis antes do lançamento do MVP" (GDD §15.9) — e o pré-requisito da recusa de formato divergente em US2; sem o arquivo exportado, não há roundtrip que verificar.

**Independent Test**: ler `crates/exportacao` e um arquivo de exportação de exemplo, confirmando um JSON com os cinco campos obrigatórios — `conta_id`, `personagem_id`, `criado_em`, `versao`, `rules_hash` (RULES hash) —, `versao` inteiro ≥ 1, `criado_em` em ISO-8601 UTC, `rules_hash` com 64 hex SHA-256 e nenhuma chamada de rede, banco, IAM remoto ou chave de instância; nenhum comando é executado nesta etapa.

**Acceptance Scenarios**:

1. **Given** uma conta e um personagem locais, **When** a exportação é acionada, **Then** é produzido um arquivo JSON com `conta_id`, `personagem_id`, `criado_em`, `versao` e `rules_hash` preenchidos, legível por ferramenta independente, sem servidor ou credencial de instância (GDD:1395, Q1, Q3).
2. **Given** a mesma conta e o mesmo personagem, **When** a exportação é repetida com o mesmo `criado_em` e o mesmo conjunto de regras, **Then** os bytes são idênticos (`diff -q` vazio) — saída determinística, sem relógio de parede implícito nem campo aleatório (Q3).
3. **Given** o `rules_hash`, **When** o arquivo é lido, **Then** ele contém o SHA-256 de 64 hex do conjunto de regras ativo, espelhando o *hash* de conteúdo do *handshake* da GDD §14.2/spec 011, sem guardar a chave de exportação em nenhuma instância (GDD §15.5, FR-002).

---

### User Story 2 — Importação com recusa de formato divergente (Priority: P1)

Como mantenedor do Núcleo, preciso que a importação aceite somente arquivos conformes ao formato versionado e recuse qualquer divergência com erro legível e código distinto, para que um arquivo alterado, de versão diferente ou corrompido nunca seja aceito silenciosamente (GDD §14.2, §15.9).

**Why this priority**: é o mecanismo de portabilidade segura — a GDD §15.9 exige que a troca inicial recuse versão divergente com erro legível, e a §15.7 exige verificabilidade por ferramenta independente; sem recusa explícita, a exportação viraria aceitação de estado pronto.

**Independent Test**: ler o importador em `crates/exportacao` e seus testes de recusa, confirmando códigos distintos para JSON inválido, versão divergente, campo ausente/malformado, `rules_hash` com comprimento diferente de 64 e `rules_hash` divergente, com `exit` não zero e nenhuma conversão automática; nenhum comando é executado nesta etapa.

**Acceptance Scenarios**:

1. **Given** um arquivo exportado conforme, **When** ele é importado, **Then** todos os cinco campos são preservados e o resultado é `OK`, com 0 perdas e 0 divergências (SC-002, FR-004).
2. **Given** um arquivo com `versao` diferente da versão suportada, **When** ele é importado, **Then** a importação é recusada com erro legível e código de versão divergente, sem tentativa de conversão ou atualização automática de versão (Q3, FR-003).
3. **Given** um arquivo com campo ausente, `criado_em` malformado ou `rules_hash` com comprimento diferente de 64 hex, **When** ele é importado, **Then** é recusado com código distinto e mensagem legível antes de qualquer leitura de regras — nenhum estado pronto é aceito e nenhum acesso a servidor ocorre (Q1, Q3, FR-003).

---

### User Story 3 — `COPYRIGHT.txt` com fonte, licença e data (Priority: P2)

Como mantenedor do hub, preciso de um `COPYRIGHT.txt` na raiz que declare fonte, licença MIT e data para cada arquivo e asset do projeto, para que nenhum arquivo fique sem licença declarada e a atribuição seja publicada desde a primeira versão (GDD §15.8, §15.9, GDD:1451).

**Why this priority**: é a condição de soberania documental do MVP — "`LICENSE` e `COPYRIGHT.txt` presentes; nenhum arquivo sem licença declarada" (GDD §15.9) —, independente do código de exportação, e validada antes de empacotar cada Módulo (§14.7).

**Independent Test**: ler `COPYRIGHT.txt` na raiz e confirmar uma entrada por arquivo/asset com fonte, licença e data, licença MIT para o Núcleo e o servidor headless (ADR-002), restrições quando houver, e 0 arquivos sem licença declarada; nenhum comando é executado nesta etapa.

**Acceptance Scenarios**:

1. **Given** o repositório com `COPYRIGHT.txt`, **When** o arquivo é lido, **Then** cada entrada declara fonte, licença e data, com 0 arquivos sem licença declarada (SC-003, FR-005).
2. **Given** um Módulo prestes a ser empacotado, **When** a atribuição é validada, **Then** o `COPYRIGHT.txt` cobre o Módulo antes do empacotamento, conforme a seção 14.7 da GDD, sem citar como própria a AGPL de terceiros mencionada na §15.5 (Q4, FR-005).

---

### User Story 4 — DCO em todo commit e gate que recusa commit sem trailer (Priority: P2)

Como mantenedor do hub, preciso que todo *commit* contenha o trailer `Signed-off-by: Nome <email>` (DCO) e que um gate recuse qualquer contribuição sem ele, para satisfazer "todo *commit* assinado; contribuição sem assinatura é recusada" (GDD:1452) mesmo sem chave GPG no ambiente.

**Why this priority**: fecha a cadeia de autoria do MVP; usa o DCO como assinatura de origem verificável, única forma disponível neste ambiente, sem introduzir dependência de GPG, servidor de chaves ou segredo (Q2).

**Independent Test**: ler `scripts/dco-check.sh` e sua integração ao pre-commit, confirmando que ele aceita um trailer `Signed-off-by` bem formado (exit 0) e recusa commit sem trailer ou com trailer malformado (exit não zero, erro legível), sem alterar os quatro estágios de `verify.sh`; nenhum commit é criado nesta etapa.

**Acceptance Scenarios**:

1. **Given** um commit com `Signed-off-by: Nome <email>`, **When** `scripts/dco-check.sh` é executado, **Then** termina com exit 0 e nenhuma mensagem de recusa (SC-004, FR-006).
2. **Given** um commit sem trailer `Signed-off-by` ou com trailer malformado, **When** o gate é executado, **Then** ele falha com exit não zero e mensagem legível, e a contribuição é recusada conforme GDD:1452 (SC-004, FR-007).
3. **Given** o pre-commit instalado por `verify.sh --install-hook`, **When** o gate DCO é acionado, **Then** `verify.sh` continua com seus quatro estágios inalterados terminando em `verify.sh: ALL STAGES OK` e os cinco gates determinísticos permanecem PASS (Q2, FR-008, SC-001).

---

### Edge Cases

- **Versão divergente na importação**: `versao` diferente da suportada é recusada com código próprio; nunca há conversão silenciosa nem upgrade automático do arquivo (Q3).
- **`rules_hash` malformado**: comprimento diferente de 64 hex é recusado antes de abrir qualquer arquivo de regras, espelhando `REJEICAO_MANIFESTO` da spec 011.
- **`rules_hash` divergente**: 64 hex válidos mas diferentes do conjunto de regras ativo geram código distinto de `rules_hash` malformado — nenhum dos dois é tratado como `OK`.
- **`criado_em` ausente ou fora de ISO-8601 UTC**: recusado como campo malformado; nenhuma data é inferida do relógio de parede.
- **JSON inválido ou vazio**: recusado antes de qualquer validação de campo, com erro legível.
- **Exportação repetida**: com as mesmas entradas e `criado_em` explícito, produz bytes idênticos — nenhum campo aleatório, nenhum carimbo de tempo implícito (Q3).
- **Commit sem trailer e sem GPG**: ausência de chave GPG não é usada como desculpa — o DCO via `Signed-off-by` é exigido de todo commit; nenhuma chave, segredo ou serviço externo é introduzido (Q2).
- **AGPL de terceiros**: a licença AGPL citada na GDD §15.5 (Lichess) é de terceiros e não se aplica ao Insurgência; o `COPYRIGHT.txt` declara MIT (Q4).
- **Servidores comunitários e forks**: nenhuma história depende de servidor comunitário, federação ou fork publicado — itens 5 e 6 do STATE.md ficam DEFER (Q1).
- **Alegação de execução**: `cargo`, gates, commits, `verify.sh`, `scripts/dco-check.sh` ou roundtrip ditos executados nesta etapa → não observado; permanece **[NV]** (SC-001–SC-005).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Um novo crate `crates/exportacao` (pacote `insurgencia-exportacao`) DEVE ser materializado no workspace `members = ["crates/*"]` com `edition = "2024"` e alvo de biblioteca. Todo código Rust DEVE declarar `unsafe_code = "forbid"`, `clippy::pedantic = "deny"` e `unwrap_used`/`expect_used` em `deny`, tratando falhas com `Result<T, E>`, `?` e `thiserror` onde aplicável, sem desativar lint em `Cargo.toml`/`clippy.toml` (única exceção: `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado). Nenhuma dependência de rede, banco, HTTP, IAM remoto, servidor ou chave de instância é introduzida (US1, Q1, skill `mdmv-rust-thin`).
- **FR-002**: A exportação DEVE produzir um arquivo JSON versionado contendo exatamente os campos obrigatórios `conta_id`, `personagem_id`, `criado_em`, `versao` e `rules_hash`. `versao` DEVE ser inteiro ≥ 1; `criado_em` DEVE ser data/hora ISO-8601 em UTC; `rules_hash` (RULES hash) DEVE ser o SHA-256 de 64 hex do conjunto de regras ativo, na forma do *hash* de conteúdo do *handshake* da GDD §14.2/spec 011. O arquivo DEVE ser legível por ferramenta independente (GDD §15.7), determinístico para a mesma entrada e `criado_em` explícito, e NÃO DEVE conter chave de exportação guardada por instância (GDD:1395, US1, Q3).
- **FR-003**: A importação DEVE aceitar somente arquivos conformes ao formato versionado e DEVE recusar, com erro legível e códigos distintos: JSON inválido, `versao` divergente da suportada, campo obrigatório ausente ou malformado, `rules_hash` com comprimento diferente de 64 hex e `rules_hash` divergente do conjunto de regras ativo. NENHUMA conversão silenciosa, atualização automática de versão, aceitação de estado pronto ou chamada de servidor é permitida (US2, Q1, Q3).
- **FR-004**: O ciclo exportar → importar DEVE preservar 100% dos cinco campos, com 0 perdas e 0 divergências; a exportação de conta, personagem e histórico DEVE estar disponível localmente antes do lançamento do MVP, conforme GDD §15.9. A saída repetida com a mesma entrada e o mesmo `criado_em` DEVE ser byte-idêntica (US1, US2, SC-002).
- **FR-005**: `COPYRIGHT.txt` DEVE existir na raiz do repositório com uma entrada por arquivo e asset, declarando fonte, licença e data (e restrições quando houver), com licença MIT para o Núcleo e o servidor headless (ADR-002, GDD:1451) e 0 arquivos sem licença declarada. A atribuição DEVE ser validada antes de empacotar cada Módulo (GDD §14.7) e publicada desde a primeira versão (GDD §15.8). A AGPL de terceiros citada na GDD §15.5 NÃO DEVE ser atribuída ao Insurgência (US3, Q4).
- **FR-006**: Todo commit DEVE conter o trailer `Signed-off-by: Nome <email>` como DCO (Developer Certificate of Origin), atendendo "todo *commit* assinado; contribuição sem assinatura é recusada" (GDD:1452). A assinatura de origem é o DCO, verificável sem GPG; nenhuma chave GPG, `commit.gpgsign`, `user.signingkey`, servidor de chaves ou segredo é exigido ou introduzido neste ciclo (US4, Q2).
- **FR-007**: `scripts/dco-check.sh` DEVE verificar o trailer `Signed-off-by` no(s) commit(s) e falhar com exit não zero e mensagem legível quando ele estiver ausente ou malformado, aceitando (exit 0) somente trailer bem formado. O gate DEVE ser integrado ao pre-commit por `verify.sh --install-hook` SEM alterar os quatro estágios de `verify.sh`, sem modificar `Makefile`, baselines ou `evidence.csv`, e sem escrever fora da worktree além dos modos explícitos já existentes (US4, SC-004).
- **FR-008**: `verify.sh` DEVE permanecer a única fonte de verdade do gate local, com quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — terminando com `verify.sh: ALL STAGES OK`. Os cinco gates determinísticos alvo (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) DEVEM permanecer PASS e inalterados (SC-001, SC-005).

### Key Entities *(include if feature involves data)*

- **Conta exportada**: identidade local do jogador, identificada por `conta_id`; nenhuma instância guarda a chave de exportação.
- **Personagem exportado**: progresso do jogador, identificado por `personagem_id`, associado à conta e ao histórico exportável.
- **Arquivo de exportação**: JSON versionado com `conta_id`, `personagem_id`, `criado_em`, `versao` e `rules_hash`; legível por ferramenta independente e determinístico.
- **`rules_hash` (RULES hash)**: SHA-256 de 64 hex do conjunto de regras ativo; espelha o *hash* de conteúdo do *handshake* (GDD §14.2/spec 011) e distingue formato malformado de divergência.
- **`COPYRIGHT.txt`**: arquivo de atribuição na raiz com fonte, licença MIT e data por arquivo/asset, validado antes de empacotar Módulos.
- **Trailer `Signed-off-by`**: linha `Signed-off-by: Nome <email>` que materializa o DCO em cada commit; assinatura de origem verificável sem GPG.
- **`scripts/dco-check.sh`**: gate que recusa commit sem trailer `Signed-off-by`, integrado ao pre-commit sem tocar os quatro estágios de `verify.sh`.
- **Crate `insurgencia-exportacao`**: unidade em `crates/exportacao`, membro do workspace, sem rede, responsável por exportar e importar o JSON versionado.

## Success Criteria *(mandatory)*

Critérios a aferir na implementação e em R4. Nenhum comando de `cargo`, gate, teste, build, formatador, `verify.sh`, `scripts/dco-check.sh` ou criação de commit foi executado nesta etapa de especificação; as metas abaixo são alvos, não resultados observados aqui.

### Measurable Outcomes

- **SC-001**: Na implementação/R4, `verify.sh` termina com `verify.sh: ALL STAGES OK` e 4 de 4 estágios OK na ordem `cargo check --locked --workspace --all-targets` → `cargo clippy --workspace --all-targets --all-features -- -D warnings` → `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) → `cargo fmt --all -- --check`, com 0 estágios vermelhos e 0 lints desativados. (baseline: estado vivo 5/5 PASS e `verify.sh` ALL STAGES OK; meta: 4 de 4; nesta etapa: **[NV]**)
- **SC-002**: 1 de 1 arquivos exportados são reimportados com os 5 de 5 campos (`conta_id`, `personagem_id`, `criado_em`, `versao`, `rules_hash`) preservados, 0 perdas e 0 divergências, e a exportação repetida com a mesma entrada e `criado_em` é byte-idêntica. (baseline: nenhuma exportação; meta: roundtrip 1 de 1; nesta etapa: **[NV]**)
- **SC-003**: Exatamente 1 `COPYRIGHT.txt` na raiz, presente em 1 de 1 verificações, com fonte, licença e data declaradas para 100% dos arquivos/assets e 0 arquivos sem licença declarada; licença MIT para Núcleo/servidor headless. (baseline: nenhum arquivo de atribuição materializado nesta etapa; meta: 1 de 1; nesta etapa: **[NV]**)
- **SC-004**: Gate DCO — 100% dos commits com trailer `Signed-off-by` bem formado, `scripts/dco-check.sh` com exit 0 quando presente e exit não zero com mensagem legível quando ausente/malformado, 0 contribuições sem assinatura aceitas, 0 alteração nos quatro estágios de `verify.sh`. (baseline: nenhuma chave GPG, `commit.gpgsign`/`user.signingkey` ausentes; meta: 100% de cobertura DCO; nesta etapa: **[NV]**)
- **SC-005**: Os 5 gates determinísticos alvo (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) permanecem 5 de 5 PASS e inalterados, com `Makefile`, baselines e `evidence.csv` intactos. (baseline: estado vivo 5/5 PASS; meta: 5 de 5; nesta etapa: **[NV]**)

## Clarifications

Resoluções das quatro ambiguidades detectadas no texto de entrada, com a fonte local que as autoriza:

- **Q1 — Há servidor neste ciclo?** **Não.** Exportação e importação são operações **locais**, em arquivo JSON, executadas no Núcleo; nenhuma chamada de rede, banco, HTTP, IAM remoto, federação, servidor comunitário ou chave guardada por instância. A GDD §15.5 decide que "exportação de dados é requisito do **Núcleo**, não cortesia do anfitrião" e que nenhuma instância guarda a chave de exportação (GDD:1395); os itens 5 e 6 do STATE.md (dois servidores com Módulos diferentes, queda do servidor de referência, servidores comunitários e forks publicados) ficam DEFER. Fonte: GDD §15.5, GDD:1395, §15.9, STATE.md (estado vivo 2026-11-21T00:22Z).
- **Q2 — DCO ou GPG?** **DCO** (Developer Certificate of Origin) via trailer `Signed-off-by: Nome <email>`, verificado por `scripts/dco-check.sh`. O ambiente **não tem chave GPG** e não tem `commit.gpgsign` nem `user.signingkey` configurados (verificado nesta etapa), portanto GPG não é exigido: a GDD:1452 ("todo *commit* assinado; contribuição sem assinatura é recusada") é atendida no sentido de **assinatura de origem**. Assinatura criptográfica GPG fica **[NV]**/fora de escopo deste ciclo, sem introduzir chave, segredo ou servidor de chaves. Fonte: GDD §15.9 (linha do DCO/CLA), verificação local do ambiente.
- **Q3 — Qual é o formato de exportação?** **JSON versionado**, com `versao` inteiro ≥ 1 e os campos obrigatórios `conta_id`, `personagem_id`, `criado_em` (ISO-8601 UTC), `versao` e `rules_hash` (RULES hash — SHA-256 de 64 hex do conjunto de regras ativo). A importação recusa versão divergente, campo ausente/malformado e `rules_hash` malformado ou divergente com códigos distintos e erro legível, sem conversão silenciosa — o mesmo princípio do *handshake* que recusa versão divergente na GDD §14.2/§15.9 e da spec 011. `criado_em` é explícito para garantir determinismo. Fonte: GDD:1395, GDD §14.2/§15.7/§15.9, spec 011.
- **Q4 — Qual licença aparece na atribuição?** **MIT** para o Núcleo e o servidor headless, conforme ADR-002 e GDD:1451, com `LICENSE` e `COPYRIGHT.txt` presentes e fonte/licença/data por arquivo. A licença **AGPL-3.0** citada na GDD §15.5 é de **terceiros** (Lichess) e não se aplica ao Insurgência; nenhum arquivo fica sem licença declarada. Fonte: ADR-002, GDD §15.5, §15.8, §15.9, GDD:1451.

## Assumptions

- O workspace raiz já declara `members = ["crates/*"]` e `resolver = "2"`; `verify.sh` é a única fonte de verdade do gate Rust e os cinco gates determinísticos, `Makefile`, baselines e `evidence.csv` não são modificados.
- A ADR-004 (Accepted, 20/11/2026) fixa Bevy 0.20.1 e toolchain 1.99.1; os crates `contratos`, `sim`, `cliente` e `metricas` permanecem intactos em suas responsabilidades — este ciclo acrescenta `crates/exportacao`, `scripts/dco-check.sh` e `COPYRIGHT.txt`, sem trocar engine, sem reabrir ADR-001.
- A exportação ocorre antes do lançamento do MVP (GDD §15.9) e cobre conta, personagem e histórico; o escopo de campos obrigatórios é `conta_id`, `personagem_id`, `criado_em`, `versao` e `rules_hash` (Q3), sem campos opcionais não declarados nesta especificação.
- Nenhuma chave GPG, segredo, serviço remoto ou dependência de rede está disponível ou é introduzida; o DCO é a forma verificável de assinatura de origem neste ambiente (Q1, Q2).
- A execução de `verify.sh`, do gate DCO, do roundtrip e a criação de commits pertencem à implementação/R4; nenhuma foi realizada nesta etapa. Toda saída permanece **[NV]** até registro com data/hora em R4, que precede R7 (SemVer, changelog com IDs FR/T e rollback).

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates.
> Ela apenas torna o artefato visível para quem cria a spec, sem depender de imposição automática.

Para specs deste hub, copie `.specify/templates/context-budget-manifest-template.json` para
`specs/[###-feature-name]/examples/context-budget-manifest.json`, preencha `cycle_id`, os caps
declarados e `output_handles[]`, e valide com:

```bash
python3 scripts/ci/check_context_budget.py specs/[###-feature-name]/examples/context-budget-manifest.json
```

O manifesto é declarativo e opt-in; honestidade dos rótulos e limites de autoridade estão em
`docs/governance/context-budget-manifest.md`. Não use esta seção como checklist embutido —
checklists ficam em `checklists/requirements-quality.md`.
