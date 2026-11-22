# Feature Specification: Estado verificado e escalonamento 45

**Branch**: `037-estado-escalonamento-45` (base `efb4d4b`) · **Created**: 2026-11-22 · **Status**: Draft · **R1**: go · **Sessão**: `20261122T225813Z-estado-escalonamento`

**Âncora**: HEAD `efb4d4b`, `origin/dev=efb4d4b`, `origin/main=c126b6e`, PR #44 mesclado em dev às 21:44:38Z, PR #45 `dev → main` OPEN/green/ACR sem blockers (comentário `7086327685`) na fila SWE, constituição do alvo v1.0.0 (ratificada 2026-11-19) e prompt v4.6 (cita v1.2.0 + R6). Ciclo **documental**: nenhuma promoção de `main`, nenhuma adopção de R6.

## User Scenarios

### US1 — Estado verificado registrado (P1)
Auditora/sessão seguinte lêem os 14 artefatos e encontram HEADs, gates e contagens do estado observado às 22:58–23:01Z.

**Why this priority**: sem esse registro, o estado verificado se perde entre notas e a honestidade das evidências (Princípio VI) se degrada.

**Independent Test**: ler `intake.md`/`research.md`/`converge.md` e comparar com as evidências da sessão.

**Acceptance Scenarios**:
1. **Given** HEAD `efb4d4b` e `origin/main=c126b6e`, **When** os artefatos são lidos, **Then** ambos aparecem com carimbo 22:58–23:01Z.
2. **Given** `verify.sh` 41/41 e 5 gates PASS observados, **When** citados, **Then** constam como observados, com **[NV]** para qualquer reexecução.

### US2 — Escalonamento do PR #45 sem mesclar `main` (P1)
SWE recebe o PR #45 escalonado (OPEN, green, ACR sem blockers, comentário `7086327685`, na fila) e a decisão explícita de não mesclar `main`.

**Why this priority**: é o único movimento pendente; mesclar fora de tempo contrariaria a autoridade do SWE.

**Independent Test**: ler `research.md`/`decision.md` e confirmar a decisão.

**Acceptance Scenarios**:
1. **Given** PR #45 green e sem blockers, **When** o ciclo é revisado, **Then** a decisão registrada é escalonar ao SWE e **não** mesclar `main`.
2. **Given** PR #44 mesclado às 21:44:38Z (`efb4d4b`), **When** citado, **Then** aparece como fato observado que encerra o ciclo 036.

### US3 — Decisão de governança (P1)
Auditora confere que a constituição v1.0.0 do alvo prevalece sobre o prompt v4.6, que R6 não foi adotado e que a divergência não é dívida.

**Why this priority**: adotar v1.2.0 sem ratificação quebraria o Princípio III e a ordem normativa do alvo.

**Independent Test**: ler `analyze.md`/`validate.md` e o trecho da constituição em `research.md`.

**Acceptance Scenarios**:
1. **Given** prompt v4.6 cita `tasks` antes de `analyze` e R6, **When** os artefatos são revisados, **Then** `analyze` precede `tasks` e não há R6.
2. **Given** `speckit.github.taskstoissues.md` exige `resolve-tasks.sh` ausente, **When** o ciclo é revisado, **Then** nenhuma emissão de issues é feita e a divergência consta como decisão de governança, não dívida.

### US4 — Fronteira, roteiro e não-execução (P2)
Auditora confere os 14 artefatos na ordem do alvo, o roteiro antes do PR e a ausência de build/gate/commit/PR.

**Why this priority**: preserva a Constituição II/VI e a rastreabilidade do hub.

**Independent Test**: listar os 14 artefatos e ler `test-roadmap.md`, `validate.md`, `release.md`.

**Acceptance Scenarios**:
1. **Given** o ciclo 037, **When** revisado, **Then** `analyze.md` precede `tasks.md`, `validate.md` (R4) aprova antes de `release.md` (R7 `1.0.0`) e nenhum `converge.md` existente foi editado.
2. **Given** o roteiro, **When** lido, **Then** contém 6–10 passos numerados, superfície documental/CLI, capturas de terminal, `automatizável?`, rollback, passo de comparar notas vs HEAD/contagens e passo de confirmar não-adoção de R6.

## Requirements

### Functional Requirements
- **FR-001**: registrar HEAD `efb4d4b`, `origin/dev=efb4d4b`, `origin/main=c126b6e`, 0 PRs para dev e PR #45 (`dev → main`) OPEN/green/ACR sem blockers (comentário `7086327685`) na fila SWE, com decisão explícita de **não mesclar `main`**.
- **FR-002**: registrar PR #44 mesclado em dev às 21:44:38Z (`efb4d4b`), encerrando o ciclo 036 (HUD acentuado, Noto Sans OFL 1.1, blocker P1 `BEVY_ASSET_ROOT` corrigido em `01ff4d0`, captura final sem env, evidência 11/11).
- **FR-003**: citar `./verify.sh` ALL STAGES OK 41/41 e 5 gates Python PASS, e as contagens versionadas (37 specs, 65 assessments, 37 validations, 37 releases, 0 bugs, 10 research), confirmando por `find` somente leitura o subset verificável neste worktree e registrando a divergência de snapshot sem reexecução: o checkout compartilhado mostra 238 assessments e 13 research por incluir 173 diretórios históricos untracked e 3 notas untracked, preservados e não versionados.
- **FR-004**: registrar decisão de governança: constituição v1.0.0 do alvo (ratificada 2026-11-19) prevalece sobre prompt v4.6 (v1.2.0, `tasks` antes de `analyze`, R6 obrigatório) pelo Bloco 0.a; divergência é **decisão, não dívida**; R6 **não** adotado; `speckit.github.taskstoissues.md`/`resolve-tasks.sh` ausente e não usado.
- **FR-005**: documentar o 80/20: documentação/escalonamento 3×5×5×5 = 375/625 escolhido sobre paridade `construir_barreira` 5×3×2×2 = 60/625.
- **FR-006**: seguir a ordem normativa **do alvo**: assess.intake → research → define → shape → decide(go) → specify → clarify → plan → checklist → revisar checklist → **analyze (READ-ONLY)** → **tasks** → implement → converge (append-only) → validate (R4) → release (R7). R1 `go`; R4 antes de R7 `1.0.0`.
- **FR-007**: escrever exatamente os 14 artefatos da fronteira, sem editar `converge.md` existente e sem tocar paths fora dela.
- **FR-008**: escrever `.specify/validations/estado-escalonamento-45/test-roadmap.md` **antes do PR**: 6–10 passos numerados, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback, passo explícito de comparar notas vs HEAD/contagens e passo de confirmar que R6 não foi adotado sem ratificação.
- **FR-009**: não executar gates/build/formatador/teste, não fazer commit/push/PR, não mesclar `main`, factory `b1deab1` somente fonte; pendências **[NV]**; nada afirmado como verde sem comando executado.

### Key Entities
- **Estado verificado**: conjunto de fatos observados às 22:58–23:01Z (HEADs, PRs, gates, contagens), colados com fonte e carimbo.
- **Escalonamento PR #45**: registro de que a promoção `dev → main` está pronta e aguarda o SWE; não é uma ação de merge.
- **Decisão de governança**: precedência da constituição v1.0.0 do alvo sobre prompt externo; distinta de dívida técnica.
- **Roteiro de testes**: plano documental com passos, superfície, capturas de terminal e rollback, escrito antes do PR.

## Clarificações
- "Não mesclar main" é decisão deste ciclo: o PR #45 pertence ao SWE e está na fila; escalonar ≠ promover.
- A divergência constitucional é **decisão de governança** (Bloco 0.a), não dívida; R6 não é adotado sem ratificação do alvo.
- Contagens versionadas de assessments/research são 65/10 (`find` somente leitura no worktree); o checkout compartilhado mostra 238/13 por incluir 173 diretórios históricos untracked e 3 notas untracked, preservados e não versionados; registrado sem reexecução, conforme Princípio VI.
- A seção "Contexto e Orçamento" do template é documental e não lida por gates; nenhum manifesto é criado (fora de fronteira).

## Success Criteria
- **SC-001**: 14 artefatos contêm HEADs, PR #44/#45, gates e contagens com carimbo 22:58–23:01Z.
- **SC-002**: Decisão de não mesclar `main` e precedência da constituição v1.0.0 explícitas; R6 ausente; nenhuma dívida registrada pela divergência.
- **SC-003**: `analyze.md` precede `tasks.md`; R4 aprovado antes de R7 `1.0.0`; `converge.md` novo e append-only; nenhum converge existente tocado.
- **SC-004**: `test-roadmap.md` escrito antes do PR com 6–10 passos, superfície documental/CLI, capturas, `automatizável?`, rollback, passos de notas vs HEAD/contagens e de não-adoção de R6.
- **SC-005**: Nenhum build/gate/formatador/teste/commit/push/PR executado; tudo o que depende de execução externa **[NV]**.
- **SC-006**: Fronteira exata respeitada; factory `b1deab1` somente fonte; histórico do ciclo 036 e PR #40 (`c126b6e`) preservados.

## Assumptions
- Este agente escreve somente os 14 artefatos SDD; SWE decide PR #45; orquestrador executa verificações futuras.
- Ambiente headless; nenhuma janela, rede ou credencial é usada por este agente.
- Evidências de 22:58–23:01Z são coladas como observadas, sem reexecução; `find` somente leitura é usado apenas para confirmar contagens.

## Out of scope
Promoção `dev → main`/merge do PR #45, paridade `construir_barreira`, adoção de R6 ou reordenação do fluxo, `taskstoissues`/`resolve-tasks.sh`, edição de `converge.md` existente, `docs/GDD*`, baselines, ledger, CI, `STATE.md`/`PROGRESSO.md`/`feature.json`, janelas, deploy, assinatura GPG, factory `b1deab1` (somente fonte), paths untracked alheios e qualquer build/teste/gate/formatador/commit/push/PR. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.
