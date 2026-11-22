# Feature Specification: Gate CLI determinístico de manifesto de compatibilidade

**Branch**: `034-gate-manifesto-cli` (base `7329c53`) · **Created**: 2026-11-22 · **Status**: Draft · **R1**: go

**Âncora obrigatória**: `specs/011-compatibilidade-manifesto/spec.md` FR-001..FR-006, `manifesto.json`, ciclo 033 (`crates/contratos/src/manifesto.rs` e `crates/contratos/src/lib.rs`) e `docs/GDD - Insurgência.md` §1/§14.2. Este ciclo **não redefine** o contrato: acrescenta somente um adaptador CLI headless sobre `conferir`.

## User Scenarios

### US1 — Verificar compatibilidade a partir de arquivos reais (P1)
Uma pessoa ou gate de CI aponta para um manifesto e para um arquivo de regras; o CLI calcula o SHA-256 do conteúdo real das regras e aplica o contrato existente, produzindo uma única linha de resultado e um código de saída estável.

**Why this priority**: é o valor central — transformar o contrato já testado em verificação executável e determinística.

**Independent Test**: criar um arquivo de regras temporário, calcular seu hash, gerar um manifesto com esse hash e executar o CLI; observar `COMPAT_OK` com exit 0.

**Acceptance Scenarios**:
1. **Given** manifesto íntegro e regras cujo SHA-256 casa com `content_hash`, **When** o CLI é executado sem argumentos opcionais, **Then** imprime `COMPAT_OK module_id=insurgencia.exemplo content_hash=<hash>` em stdout e sai 0.
2. **Given** o mesmo manifesto, **When** `module_id` e `versao_local` são passados explicitamente com os valores do contrato, **Then** o resultado é idêntico ao caso 1.
3. **Given** um manifesto com `content_hash` em maiúsculas, **When** as regras têm o mesmo hash em minúsculas, **Then** a comparação de `conferir` (`eq_ignore_ascii_case`) aceita e a saída é determinística.

### US2 — Recusas legíveis e códigos de saída estáveis (P1)
Toda condição de incompatibilidade é reportada com o literal oficial do contrato na stderr e exit 1; uso inválido e falhas operacionais são distintos (exit 2).

**Why this priority**: sem códigos estáveis o CLI não serve como gate e pode ser confundido com erro do processo.

**Independent Test**: variar um campo por execução do manifesto de referência e conferir literal e exit code.

**Acceptance Scenarios**:
1. **Given** JSON inválido ou campos obrigatórios ausentes, **When** o CLI roda, **Then** imprime `COMPAT_FALHA MANIFEST_INVALID` na stderr e sai 1.
2. **Given** `protocol_version` divergente de `versao_local`, **Then** `COMPAT_FALHA VERSAO_PROTOCOLO_DIVERGENTE`, exit 1 — antes de qualquer checagem de módulo ou hash.
3. **Given** `module_id` divergente, **Then** `COMPAT_FALHA MODULO_AUSENTE`, exit 1.
4. **Given** arquivo de regras inexistente, **Then** o adaptador passa `None` a `conferir` e o resultado é `COMPAT_FALHA REGRAS_AUSENTES`, exit 1.
5. **Given** hash das regras diferente de `content_hash`, **Then** `COMPAT_FALHA HASH_MODULO_DIVERGENTE`, exit 1.
6. **Given** argumentos em número errado ou `versao_local` não numérico, **When** o CLI roda, **Then** mensagem de uso/erro legível na stderr e exit 2, sem linhas `COMPAT_OK`/`COMPAT_FALHA`.
7. **Given** falha de leitura que não seja arquivo de regras inexistente (permissão, diretório, JSON ilegível em disco), **Then** mensagem de erro legível e exit 2.

### US3 — Adaptador fino, sem novas dependências e SDD rastreável (P2)
A I/O fica exclusivamente no exemplo; a biblioteca continua pura. O ciclo segue a ordem normativa com R4 aprovado antes de R7 e roteiro de testes para o orquestrador.

**Why this priority**: preserva a Constituição II e a governança do hub; evita que o gate vire uma segunda implementação do contrato.

**Independent Test**: inspecionar o diff — somente o novo exemplo e os artefatos do ciclo — e conferir ausência de `unwrap`/`expect`/`unsafe`, de novas dependências e de alterações em `Cargo.toml`.

**Acceptance Scenarios**:
1. **Given** o código entregue, **When** inspecionado, **Then** `conferir` não é alterado e nenhuma função de I/O aparece na biblioteca.
2. **Given** as evidências de `2026-11-22T16:54:17Z`, **When** registradas, **Then** `./verify.sh` aparece com `ALL STAGES OK` e 41/41 testes, 5 gates PASS e CI run `44952738240` `success`, sem reexecução neste ciclo.
3. **Given** o roteiro de testes, **When** executado pelo orquestrador, **Then** 3 execuções do caso aceito produzem saídas idênticas comparadas por `diff`.

## Requirements

### Functional Requirements
- **FR-001**: fornecer `crates/sim/examples/verificar_manifesto.rs` com a assinatura `verificar_manifesto <manifesto.json> <regras.cfg> [module_id] [versao_local]`; os dois primeiros argumentos são obrigatórios; `module_id` usa `insurgencia_contratos::MODULO_ID` e `versao_local` usa `PROTOCOL_VERSION` como padrão; argumentos em número inválido, `versao_local` não interpretável como `u8` ou argumento não UTF-8 produzem mensagem legível na stderr e exit 2.
- **FR-002**: manter toda I/O no adaptador: ler o manifesto como texto (`fs::read_to_string`) e as regras como bytes (`fs::read`); `NotFound` no arquivo de regras é traduzido a `hash_regras = None` para devolver `REGRAS_AUSENTES` pelo contrato; qualquer outra falha de leitura (inclusive a do manifesto) produz mensagem legível e exit 2. A biblioteca `contratos` não é alterada e não faz I/O.
- **FR-003**: calcular o SHA-256 do conteúdo do arquivo de regras com `sha2 = "=0.10.9"` e converter os bytes para hexadecimal minúsculo por formatação manual (tabela `b"0123456789abcdef"`, `String::with_capacity`), sem crate `hex` e sem nova dependência; nenhum `Cargo.toml` ou `Cargo.lock` é alterado.
- **FR-004**: chamar `insurgencia_contratos::manifesto::conferir(json, module_id, versao_local, hash_regras)` e traduzir o resultado: sucesso → `COMPAT_OK module_id=<id> content_hash=<hash>` em stdout, exit 0; recusa → `COMPAT_FALHA <codigo>` em stderr com `ErroCompatibilidade::codigo()`, exit 1. A ordem formato → versão → módulo → 64 hex → regras → hash permanece a de `conferir`; o CLI não a reordena nem a reimplementa.
- **FR-005**: higiene Rust: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `.unwrap()`/`.expect()`/`unsafe`/`async`, `Result`/`ExitCode` em vez de pânico, `max_width=100`, nomes e commentários em pt-BR, nenhum `#![allow(...)]` amplo, nenhuma dependência nova e nenhuma alteração em `crates/contratos`, `crates/sim/src` ou `manifesto.json`.
- **FR-006**: escrever `specs/034-gate-manifesto-cli/{spec,plan,analyze,tasks,converge}.md`, `checklists/requirements.md`, os 5 artefatos de assess, `validate.md` (R4), `release.md` (R7 `1.0.0`) e `test-roadmap.md`; cobrir os 8 casos do roteiro (uso inválido, aceito com hash real, manifesto inválido, versão, módulo, regras ausentes, hash divergente e 3 execuções idênticas por `diff`), manter `converge.md` append-only, `analyze.md` READ-ONLY, R4 antes de R7, rollback documentado e nenhum build/teste/gate/commit/push/PR; toda execução pendente é marcada **[NV]**.

### Key Entities
- **Argumentos do adaptador**: caminhos do manifesto e das regras, `module_id` e `versao_local` opcionais; fronteira entre mundo externo e contrato.
- **Hash das regras**: `Option<String>` com SHA-256 hex minúsculo do bytes do arquivo; `None` significa "arquivo de regras ausente".
- **Manifesto validado**: `ManifestoCompatibilidade` devolvido por `conferir`; fonte dos campos impressos em `COMPAT_OK`.
- **Código de recusa**: literal de `ErroCompatibilidade::codigo()`; único vocabulário de falha do CLI.

## Clarificações
- "Manifesto ausente/inválido → `MANIFEST_INVALID`" significa manifesto com JSON malformado ou campos obrigatórios ausentes; um **caminho** de manifesto inexistente é falha de leitura (exit 2), distinto de recusa de contrato.
- "Arquivo de regras ausente → `REGRAS_AUSENTES`" é condição de domínio mapeada a `None` no adaptador; não é tratada como falha de I/O (exit 2).
- A seção "Contexto e Orçamento" do template é documental e não é lida por gates; nenhum manifesto de orçamento é criado neste ciclo.

## Success Criteria
- **SC-001**: 8/8 casos do roteiro produzem o literal e o exit code esperados (0 aceito, 1 recusa, 2 uso/leitura).
- **SC-002**: no caso aceito, o hash impresso é igual ao SHA-256 calculado a partir do arquivo de regras temporário (comparação textual, minúsculas).
- **SC-003**: 3 execuções idênticas do caso aceito produzem `diff` vazio e o mesmo exit code.
- **SC-004**: nenhuma dependência nova e nenhuma alteração em `Cargo.toml`, `Cargo.lock`, `crates/contratos`, `crates/sim/src` ou `manifesto.json` (diff limitado ao exemplo e aos artefatos do ciclo).
- **SC-005**: 0 ocorrências de `unwrap`, `expect`, `unsafe` ou `async` no arquivo novo; `forbid(unsafe_code)` e `deny(clippy::pedantic)` presentes; linhas com no máximo 100 colunas.
- **SC-006**: SDD completo na ordem normativa, 14 artefatos escritos + 1 arquivo de código, `analyze.md` READ-ONLY, `converge.md` append-only com primeira entrada `2026-11-22`, R4 aprovado antes de R7 `1.0.0`, rollback documentado e nenhum build/teste/gate/commit/push/PR executado.

## Assumptions
- Orquestrador executa o roteiro de testes, consulta as evidências remotas e atualiza o baseline de converge (o arquivo novo ainda não está baselined); neste ciclo nada disso é feito.
- `sha2`, `thiserror` e `insurgencia-contratos` permanecem disponíveis em `crates/sim` com as versões fixadas atuais.
- Ambiente é headless; nenhuma janela, engine, rede ou relógio é usado pelo exemplo.
- Evidências de `2026-11-22T16:54:17Z` e CI run `44952738240` são coladas como observadas, sem reexecução.

## Out of scope
Segunda implementação da validação, I/O na biblioteca, dependências novas, alteração de `Cargo.toml`/`Cargo.lock`, `crates/contratos`, `crates/sim/src`, `manifesto.json`, `specs/011`, GDScript, factory `b1deab1` (somente fonte), `docs/GDD*`, baselines, `.gitignore`, `STATE.md`, `PROGRESSO.md`, ledger, CI, 193 paths untracked alheios, janelas, credenciais, deploy/remote e qualquer build/teste/gate/commit/push/PR. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. Ela apenas torna o artefato visível para quem cria a spec, sem depender de imposição automática. Para specs deste hub, o padrão é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.
