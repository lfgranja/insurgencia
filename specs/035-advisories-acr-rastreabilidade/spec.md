# Feature Specification: Fechamento de advisories de ACR e rastreabilidade documental pós-034/041

**Branch**: `035-advisories-acr-rastreabilidade` (base `f9d87e0`) · **Created**: 2026-11-22 · **Status**: Draft · **R1**: go

**Âncora obrigatória**: PRs #39/#40/#41 (comentários de ACR `7082836571` / `7082907820` / `7082916777`), artefatos do ciclo 034 (`specs/034-gate-manifesto-cli/`, `.specify/validations/gate-manifesto-cli/evidence.md`), `.specify/memory/STATE.md`, `.specify/feature.json`, `.gitignore` e `crates/sim/examples/verificar_manifesto.rs`. Este ciclo **não redefine** `conferir` (011/033): apenas corrige documentação e a linha de saída do adaptador.

## User Scenarios

### US1 — Fechar os advisories de ACR com rastreabilidade (P1)
Uma pessoa responsável pelo hub consulta os comentários dos PRs #39/#40/#41 e encontra cada advisory (3+4+3 = 10) fechado e rastreado até uma correção na fonte ou uma decisão documentada, sem blockers pendentes.

**Why this priority**: é o valor central — transformar revisões dispersas em fechamento verificável, sem reabrir contratos já estabilizados.

**Independent Test**: listar os três comentários de ACR e conferir que os 10 advisories têm destino registrado nos artefatos do ciclo e nenhum marcador de blocker.

**Acceptance Scenarios**:
1. **Given** os 10 advisories dos comentários `7082836571` (3), `7082907820` (4) e `7082916777` (3), **When** o fechamento é registrado, **Then** cada um aparece rastreado em `research.md`/`converge.md` e nenhum é classificado como blocker.
2. **Given** o PR #40 (`dev→main`) `OPEN` no head `f9d87e0`, **When** o ciclo é encerrado, **Then** nenhum merge em `main` é feito e a fila SWE é preservada.

### US2 — Restaurar a deriva documental pós-034/041 (P1)
Uma auditora lê `STATE.md`, `evidence.md` e os artefatos 034 e encontra afirmações que batem com o HEAD real, em vez das quatro divergências observadas em `18:31–18:34Z`.

**Why this priority**: sem evidências reproduzíveis, gates e auditorias futuras perdem a base factual.

**Independent Test**: comparar cada afirmação com a fonte lida diretamente (contagens, âncora, número de capturas, status do converge, conteúdo de `.gitignore`).

**Acceptance Scenarios**:
1. **Given** `STATE.md` com 33/236/33/33 e âncora `specs/033`, **When** corrigido, **Then** registra 34 specs / 237 assessments / 34 validations / 34 releases / 0 bugs / 13 research, coerente com `.specify/feature.json → specs/034`.
2. **Given** `evidence.md` com "quatro execuções" e "converge pendente", **When** corrigido, **Then** registra cinco execuções/capturas e converge baselinado (`17/17`).
3. **Given** artefatos 034 afirmando `.gitignore` intacto, **When** corrigidos, **Then** passam a registrar que `.gitignore:5-6` contém `/.specify/queue/`, sem remover a regra legítima.

### US3 — Saída canônica do CLI e governança do SDD (P2)
Uma pessoa ou gate de CI executa `verificar_manifesto` com um manifesto cujo `content_hash` está em maiúsculas; o CLI aceita (contrato inalterado) e imprime o hash calculado em minúsculas. O ciclo segue a ordem normativa com rastreabilidade completa.

**Why this priority**: garante determinismo textual da saída e preserva a Constituição II e a governança do hub.

**Independent Test**: gerar regras temporárias, calcular `sha256sum` (minúsculo), gerar manifesto com `content_hash` em maiúsculas equivalente e executar o CLI; observar `COMPAT_OK` com o hash minúsculo.

**Acceptance Scenarios**:
1. **Given** manifesto com `content_hash` maiúsculo e regras com o mesmo hash minúsculo, **When** o CLI roda, **Then** imprime `COMPAT_OK module_id=insurgencia.exemplo content_hash=<hash-minúsculo>` em stdout, exit 0 — sem imprimir as maiúsculas do manifesto.
2. **Given** manifesto com `content_hash` já minúsculo, **When** o CLI roda, **Then** imprime o mesmo hash minúsculo (nenhuma regressão).
3. **Given** o SDD do ciclo, **When** inspecionado, **Then** `analyze.md` (READ-ONLY) vem antes de `tasks.md`, R4 é aprovado antes de R7 `1.0.0`, `converge.md` é append-only e o roteiro de testes foi escrito antes do PR com o passo do `content_hash` maiúsculo.

## Requirements

### Functional Requirements
- **FR-001**: consolidar e fechar os 10 advisories de ACR dos PRs #39/#40/#41 (comentários `7082836571`, `7082907820`, `7082916777`), todos sem blockers, com rastreabilidade por PR/comentário nos artefatos do ciclo; nenhuma redefinição de `conferir`, de `specs/011` ou do contrato de manifesto.
- **FR-002**: corrigir a deriva de `.specify/memory/STATE.md` para 34 specs / 237 assessments / 34 validations / 34 releases / 0 bugs / 13 research, com âncora coerente com `.specify/feature.json` (ciclo 034 registrado), substituindo as contagens 33/236/33/33 e a âncora `specs/033`.
- **FR-003**: corrigir `.specify/validations/gate-manifesto-cli/evidence.md` para registrar cinco execuções/capturas no passo 1 (e não quatro) e converge baselinado (`17/17`, não "pendente"), preservando as capturas e o resto do relatório.
- **FR-004**: corrigir a afirmação de `.gitignore` intacto nos artefatos 034 para registrar que `.gitignore:5-6` contém `/.specify/queue/`, sem remover a regra — a alegação atual é falsa.
- **FR-005**: corrigir `crates/sim/examples/verificar_manifesto.rs:135-136` para imprimir o hash calculado em minúsculas (`hash_regras`) em `COMPAT_OK`, em vez de `manifesto.content_hash`; manter a aceitação de `content_hash` maiúsculo via `eq_ignore_ascii_case` em `conferir`, a ordem formato → versão → módulo → 64 hex → regras → hash, os exit codes 0/1/2 e a higiene Rust (sem `unwrap`/`expect`/`unsafe`/`async`, `forbid(unsafe_code)`, `deny(clippy::pedantic)`). Nenhuma alteração em `crates/contratos`.
- **FR-006**: escrever `specs/035-advisories-acr-rastreabilidade/{spec,plan,analyze,tasks,converge}.md`, `checklists/requirements.md`, os 5 artefatos de assess, `validate.md` (R4), `release.md` (R7 `1.0.0`) e `test-roadmap.md`; ordem normativa com `analyze` (READ-ONLY) **antes** de `tasks`, `converge.md` append-only, R4 aprovado antes de R7, changelog FR/T e rollback documentado.
- **FR-007**: escrever `.specify/validations/advisories-acr-rastreabilidade/test-roadmap.md` **antes do PR** com escopo/FR/T, pré-condições, passos numerados (ação → resultado esperado), superfície CLI/docs, capturas de terminal/JSON, coluna `automatizável?` e rollback, incluindo um passo explícito para `content_hash` maiúsculo no manifesto ser aceito e imprimir o hash minúsculo canônico.
- **FR-008**: manter a fronteira e a não-executação: nenhum build/teste/gate/commit/push/PR, nenhum merge em `main` (PR #40 fica na fila SWE), factory `b1deab1` somente fonte, 194 paths untracked alheios, `docs/GDD*`, ledger, CI e baselines pré-existentes intactos; toda execução pendente marcada **[NV]**.

### Key Entities
- **Advisory ACR**: finding de revisão identificado por PR + comentário; estado = sem blockers / fechado com correção na fonte.
- **Divergência documental**: par afirmação-em-arquivo vs. valor-real-no-HEAD; resolvida corrigindo a fonte, nunca a expectativa.
- **Hash canônico**: SHA-256 hex minúsculo calculado a partir das regras; única forma impressa em `COMPAT_OK`, mesmo quando o manifesto traz maiúsculas.
- **Roteiro de testes**: contrato de verificação escrito antes do PR, com rastreabilidade FR/T e passo determinístico do hash.

## Clarificações
- "Fechar advisories" significa registrar destino rastreável; não significa reavaliar o contrato de 011/033 nem reabrir discussões já decididas.
- "Corrigir a fonte" aplica-se aos documentos e a uma linha de saída do CLI; a regra `/.specify/queue/` do `.gitignore` é legítima e **não** deve ser removida.
- "Hash canônico" é o valor calculado em minúsculas; o `content_hash` do manifesto pode permanecer em maiúsculas no arquivo, mas não é o que se imprime.
- A seção "Contexto e Orçamento" do template é documental e não é lida por gates; nenhum manifesto de orçamento é criado neste ciclo.

## Success Criteria
- **SC-001**: 10/10 advisories (3+4+3) rastreados e fechados, 0 blockers, nenhum merge em `main`.
- **SC-002**: 4/4 divergências corrigidas na fonte (STATE.md, evidence.md, artefatos 034 e saída do CLI), com os valores observados no HEAD.
- **SC-003**: com `content_hash` maiúsculo no manifesto, o CLI imprime o hash minúsculo canônico e aceita exit 0; com hash já minúsculo, saída idêntica.
- **SC-004**: 14 artefatos na ordem normativa, `analyze.md` READ-ONLY antes de `tasks.md`, `converge.md` append-only, R4 aprovado antes de R7 `1.0.0`, rollback documentado e nenhum build/teste/gate/commit/push/PR executado.
- **SC-005**: evidências de `2026-11-22T18:32:17Z` (`ALL STAGES OK`, 41/41, 5 gates) e PR #40 `OPEN` (`gates`/`rust` pass) coladas como observadas, sem reexecução.

## Assumptions
- Orquestrador/agente de código aplica as quatro correções na fonte, executa o roteiro e mantém PR #40 na fila SWE; este agente escreve somente os 14 artefatos SDD.
- `conferir` continua aceitando maiúsculas via `eq_ignore_ascii_case`; `hash_regras` continua disponível no adaptador para impressão.
- Ambiente headless; nenhuma janela, engine, rede ou relógio é usado.
- Evidências de `18:32:17Z` e CI de PR #40 são coladas como observadas, sem reexecução.

## Out of scope
Redefinir `conferir`/011; remover regra de `.gitignore`; instalar Godot ou paridade Rust↔Godot (autorização externa; 80/625); mesclar PR #40 em `main`; factory `b1deab1` (somente fonte), `docs/GDD*`, ledger, CI, baselines pré-existentes, 194 paths untracked alheios, janelas, credenciais, deploy/remote e qualquer build/teste/gate/commit/push/PR. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. Ela apenas torna o artefato visível para quem cria a spec, sem depender de imposição automática. Para specs deste hub, o padrão é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.
