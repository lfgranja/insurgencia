# Feature Specification: Estado verificado pós-v1.2.2

**Branch**: `038-estado-pos-v122` (base `e010363`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: go · **Sessão**: `20261123T012952Z-estado-pos-v122`

**Âncora**: HEAD/`dev` `e010363`, `main` `8b0b044`; PR #48 `dev → main` OPEN, green, ACR sem blockers (comentário `7089052846`) na fila SWE; constituição **v1.2.2** do alvo (ratificada 2026-11-19, última emenda 2026-11-23); 0 PRs para dev. Ciclo **documental**: registra o estado observado às 01:29–01:31Z e a deriva de cinco documentos; não promove `main`, não edita documentos derivados, não executa nada. **R6 suspensa por decisão ratificada** (fora do fluxo; PARE se invocada; nunca simular issues).

## User Scenarios

### US1 — Estado verificado registrado (P1)
Auditoria/sessão seguinte leem os 14 artefatos e encontram HEADs, gates, PR e contagens do estado observado às 01:29–01:31Z.

**Why this priority**: sem esse registro, o estado verificado se perde entre notas e trechos defasados, degradando a honestidade das evidências (Princípio VI).

**Independent Test**: ler `intake.md`/`research.md`/`converge.md` e comparar com as evidências da sessão.

**Acceptance Scenarios**:
1. **Given** HEAD/`dev`=`e010363` e `main`=`8b0b044`, **When** os artefatos são lidos, **Then** ambos aparecem com carimbo 01:29–01:31Z.
2. **Given** 41/41 testes e gates Python PASS observados, **When** citados, **Then** constam como observados, com **[NV]** para qualquer reexecução.

### US2 — Escalonamento do PR #48 sem mesclar `main` (P1)
SWE recebe o PR #48 escalonado (OPEN, green, ACR sem blockers, comentário `7089052846`, na fila) e a decisão explícita de não mesclar `main`.

**Why this priority**: é o único movimento pendente; a constituição v1.2.2 limita o merge autônomo a `dev` e a promoção/deploy pertence ao SWE.

**Independent Test**: ler `research.md`/`decision.md` e confirmar a decisão.

**Acceptance Scenarios**:
1. **Given** PR #48 green e sem blockers, **When** o ciclo é revisado, **Then** a decisão registrada é escalonar ao SWE e **não** mesclar `main`.
2. **Given** deploy ausente no workflow, **When** citado, **Then** aparece como dívida já escalada e aberta, não corrigida neste ciclo.

### US3 — Deriva documental registrada (P1)
Auditoria confere que a deriva em `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` e `STATE.md` (âncora antiga `efb4d4b`/PR #45) está registrada sem que os arquivos sejam editados, e que as contagens versionadas corretas (37/65/37/37/0/10) são distintas do checkout compartilhado (238/13).

**Why this priority**: impedir que o snapshot antigo seja tomado como vigente e que contagens não versionadas sejam afirmadas como reais.

**Independent Test**: ler `research.md`/`analyze.md` e conferir os cinco caminhos e as duas séries de contagens.

**Acceptance Scenarios**:
1. **Given** os cinco documentos ancoram `efb4d4b`/PR #45, **When** o ciclo é revisado, **Then** a deriva consta como registrada, com decisão de não edição por fronteira.
2. **Given** o checkout compartilhado mostra 238/13, **When** as contagens são citadas, **Then** as versionadas (37/65/37/37/0/10) aparecem como corretas e a diferença é explicada por 173 diretórios e 3 notas untracked preservadas.

### US4 — Governança v1.2.2 e R6 suspensa (P1)
Auditoria confere a ordem executável da constituição v1.2.2, com `tasks` antes de `analyze` (READ-ONLY), R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only, e a suspensão de R6 por decisão ratificada.

**Why this priority**: adotar R6 sem adaptador e sem emenda, ou repetir a ordem do ciclo 037, quebraria o Princípio III.

**Independent Test**: ler `plan.md`, `tasks.md`, `analyze.md`, `validate.md` e o trecho da constituição em `research.md`.

**Acceptance Scenarios**:
1. **Given** a constituição v1.2.2, **When** os artefatos são revisados, **Then** `tasks` precede `analyze` e R4 precede R7 `1.0.0`.
2. **Given** R6 ratificada como intenção mas sem `resolve-tasks.sh`/adaptador, **When** o ciclo é revisado, **Then** consta como suspensa, com PARE se invocada, nenhuma issue simulada e nenhum `taskstoissues` usado.

### US5 — Fronteira, roteiro e não-execução (P2)
Auditoria confere os 14 artefatos, o roteiro escrito antes do PR e a ausência de build/gate/commit/PR.

**Why this priority**: preserva os Princípios II/VI e a rastreabilidade do hub.

**Independent Test**: listar os 14 artefatos e ler `test-roadmap.md`, `validate.md`, `release.md`.

**Acceptance Scenarios**:
1. **Given** o ciclo 038, **When** revisado, **Then** nenhum `converge.md` existente foi editado e nenhuma alteração preexistente fora da fronteira foi tocada.
2. **Given** o roteiro, **When** lido, **Then** contém 10 passos numerados, superfície documental/CLI, capturas de terminal, `automatizável?`, rollback, passo de conferir PR #48 green, passo de notas vs HEAD/contagens e passo de confirmar R6 fora do fluxo.

## Requirements

### Functional Requirements
- **FR-001**: registrar HEAD/`dev` `e010363`, `main` `8b0b044`, 0 PRs para dev e PR #48 (`dev → main`) OPEN/green/ACR sem blockers (comentário `7089052846`) na fila SWE, com decisão explícita de **não mesclar `main`** (promoção e deploy são competência do SWE).
- **FR-002**: registrar gates Python PASS e 41/41 testes observados, **deploy ausente no workflow** (dívida escalada e aberta) e inventário (1 worktree, 0 stashes, 24 untracked preservados), com **[NV]** para reexecução.
- **FR-003**: registrar a deriva documental em `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` e `STATE.md` (âncora `efb4d4b`/PR #45 em trechos finais) **sem editá-los**, e as contagens versionadas corretas (37 specs, 65 assessments, 37 validations, 37 releases, 0 bugs, 10 research), distinguindo-as do checkout compartilhado (238/13) por 173 diretórios untracked históricos e 3 notas `2026-11-20-*` preservadas.
- **FR-004**: registrar a constituição **v1.2.2 do alvo** (ratificada 2026-11-19, última emenda 2026-11-23): merge autônomo só em `dev`, promoção/deploy ao SWE, ordem executável `00 → 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09 → revisão → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release`, `analyze` READ-ONLY **depois** de `tasks`, R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only.
- **FR-005**: registrar **R6 suspensa por decisão ratificada**: ratificada como intenção em v1.2.0, mas fora da sequência executável até `resolve-tasks.sh` e o adaptador de issues existirem; se invocada, **PARE e escale ao SWE**; nunca simular issue, nunca usar `gh` improvisado, nunca alegar execução; inclusão exige emenda ratificada.
- **FR-006**: documentar o 80/20: documentação pós-v1.2.2 3×5×5×5 = 375/625 escolhida sobre coletar materiais 5×4×3×3 = 180/625.
- **FR-007**: escrever exatamente os 14 artefatos da fronteira, sem editar `converge.md` existente, sem tocar os cinco documentos com deriva e sem tocar alterações preexistentes fora da fronteira.
- **FR-008**: escrever `.specify/validations/estado-pos-v122/test-roadmap.md` **antes do PR**: 10 passos numerados, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback, passo explícito de conferir PR #48 green, passo de comparar notas vs HEAD/contagens e passo de confirmar R6 fora do fluxo.
- **FR-009**: não executar gates/build/formatador/teste, não fazer commit/push/PR, não mesclar `main`, factory `b1deab1` (`origin/dev 05b977f`) somente fonte; pendências **[NV]**; nada afirmado como verde sem comando executado.

### Key Entities
- **Estado verificado**: fatos observados às 01:29–01:31Z (HEADs, PR, gates, contagens, inventário), colados com fonte e carimbo.
- **Deriva documental**: descompasso entre os trechos finais de cinco documentos normativos e o estado vigente; registrado, não corrigido neste ciclo.
- **Escalonamento PR #48**: registro de que a promoção `dev → main` está pronta e aguarda o SWE; não é ação de merge.
- **Suspensão de R6**: decisão ratificada que mantém `taskstoissues` fora do fluxo até a ferramenta existir; distinta de dívida e de não-ratificação.
- **Roteiro de testes**: plano documental com passos, superfície, capturas e rollback, escrito antes do PR.

## Clarificações
- "Não mesclar `main`" é decisão deste ciclo: o PR #48 pertence ao SWE e está na fila; escalonar ≠ promover. Deploy ausente no workflow é dívida já escalada, não correção deste ciclo.
- A deriva dos cinco documentos é registrada, não emendada: os arquivos ficam fora da fronteira exata.
- R6 está **ratificada, mas suspensa**: não é dívida nem não-ratificação; se invocada sem `resolve-tasks.sh`/adaptador, PARE e escale — nunca simular issues.
- Contagens versionadas são 37/65/37/37/0/10; o checkout compartilhado mostra 238/13 por incluir 173 diretórios untracked históricos e 3 notas untracked, preservados e não versionados.
- A ordem executável é a da constituição v1.2.2 do alvo (`tasks` antes de `analyze`) — diferente do ciclo 037; nenhuma inversão por hábito.
- Alterações preexistentes em `.specify/feature.json` e `docs/PROGRESSO.md` pertencem ao usuário e não são tocadas.
- A seção "Contexto e Orçamento" do template é documental e não lida por gates; nenhum manifesto é criado (fora de fronteira).

## Success Criteria
- **SC-001**: 14 artefatos contêm HEADs, PR #48, gates e contagens com carimbo 01:29–01:31Z.
- **SC-002**: Decisão de não mesclar `main` explícita; deriva registrada sem edição; contagens versionadas distintas das do checkout compartilhado.
- **SC-003**: Ordem v1.2.2 respeitada (`tasks` antes de `analyze` READ-ONLY); R4 antes de R7 `1.0.0`; `converge.md` novo e append-only.
- **SC-004**: R6 suspensa por decisão ratificada, com PARE se invocada e nenhuma issue simulada.
- **SC-005**: `test-roadmap.md` escrito antes do PR com 10 passos, capturas, `automatizável?`, rollback, passos de PR #48 green, notas vs HEAD/contagens e R6 fora do fluxo.
- **SC-006**: Nenhum build/gate/formatador/teste/commit/push/PR executado; tudo o que depende de execução externa **[NV]**; fronteira exata respeitada.

## Assumptions
- Este agente escreve somente os 14 artefatos SDD; SWE decide PR #48 e deploy; orquestrador executa verificações futuras.
- Ambiente headless; nenhuma janela, rede ou credencial é usada por este agente.
- Evidências de 01:29–01:31Z são coladas como observadas, sem reexecução; nenhum comando de gate, build, git remoto ou GitHub é executado.

## Out of scope
Promoção `dev → main`/merge do PR #48; edição de `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` e `STATE.md`; reexecução de gates/build/capturas; coleta de materiais; adoção de R6 como regra ativa, `taskstoissues`/`resolve-tasks.sh` ou simulação de issues; edição de `converge.md` existente; `docs/GDD*`, baselines, ledger, CI, `feature.json`, paths untracked alheios, janelas, deploy, assinatura GPG, factory `b1deab1` (somente fonte), commits/push/PRs e alterações preexistentes fora da fronteira. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.
