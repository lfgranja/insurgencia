# Feature Specification: HUD com acento e fonte livre

**Branch**: `036-hud-acento-fonte-livre` (base `7c5c0f3`) · **Created**: 2026-11-22 · **Status**: Draft · **R1**: go

**Âncora**: pedido explícito do SWE (“Agora quero o Ê sim”), `crates/cliente/src/main.rs:458,459-462,555`, captura `docs/evidencias/hud-confirmacao-visual-2026-11-22.png` (Xvfb + lavapipe), ciclo 029 (paliativo ASCII), `/usr/share/fonts/google-noto/NotoSans-Regular.ttf`, SIL OFL 1.1 e PR #40 mesclado às 20:52:35Z (`c126b6e`). Este ciclo **supera** o paliativo do ciclo 029 sem apagar seu histórico.

## User Scenarios

### US1 — Grafia correta do nome do jogo (P1)
A SE vê `INSURGÊNCIA — low-poly` no HUD, com `Ê` e `—` renderizados, sem `▯`.

**Why this priority**: é o pedido literal do SWE e a identidade do produto (nome do jogo corrompido desde 029).

**Independent Test**: compilar/executar o cliente em `--minimizado`, capturar em Xvfb e inspecionar o título; ou inspecionar as duas linhas de `main.rs`.

**Acceptance Scenarios**:
1. **Given** o HUD atual ASCII (`INSURGENCIA - low-poly`), **When** a fonte com os glifos é carregada e o texto restaurado, **Then** a captura mostra `INSURGÊNCIA — low-poly` sem tofu.
2. **Given** a fonte padrão do Bevy sem U+00CA/U+2014, **When** o `TextFont` recebe a fonte embarcada, **Then** nenhum `▯` aparece no título.

### US2 — Separador `·` nos limites (P1)
A SE vê `Limites: X +/-580 · Y +/-310` no HUD dinâmico, com ponto médio real.

**Why this priority**: completa a grafia pedida; o `/` do ciclo 029 é um paliativo visível.

**Independent Test**: capturar o texto atualizado com `atualizar_texto` e conferir o caractere U+00B7.

**Acceptance Scenarios**:
1. **Given** `format!` com `/` em `atualizar_texto`, **When** substituído por `·`, **Then** a captura mostra o ponto médio e nenhum `▯`.

### US3 — Fonte livre embarcada e rastreável (P2)
Auditora confere proveniência, hash e licença da fonte e os acha registrados no repositório.

**Why this priority**: sem rastreabilidade, a correção não é soberana nem reproduzível em outro host.

**Independent Test**: comparar `sha256sum` do TTF e da licença com os valores registrados e ler `docs/governance/font-provenance.md`.

**Acceptance Scenarios**:
1. **Given** `/usr/share/fonts/google-noto/NotoSans-Regular.ttf` (621572 bytes), **When** copiado para `assets/fonts/` e verificado, **Then** SHA-256 = `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823`.
2. **Given** a licença SIL OFL 1.1, **When** inspecionado `assets/fonts/OFL.txt`, **Then** contém o texto integral e a proveniência registra URL primária e cobertura Unicode.

### US4 — Governança e não-execução (P2)
Auditora confere os 14 artefatos SDD na ordem normativa, com `analyze` antes de `tasks`, R4 antes de R7 e nenhum build/gate/commit executado pelo agente SDD.

**Why this priority**: preserva a Constituição II e a honestidade das evidências do hub.

**Independent Test**: listar os artefatos e ler as seções de rastreabilidade/rollback.

**Acceptance Scenarios**:
1. **Given** o ciclo 036, **When** os artefatos são revisados, **Then** `analyze.md` precede `tasks.md`, `validate.md` (R4) aprova antes de `release.md` (R7 `1.0.0`) e tudo o que não foi executado está **[NV]**.

## Requirements

### Functional Requirements
- **FR-001**: `crates/cliente/src/main.rs` (`configurar`) deve usar `Text::new("INSURGÊNCIA — low-poly")` (U+00CA e U+2014).
- **FR-002**: `crates/cliente/src/main.rs` (`atualizar_texto`) deve gerar `"INSURGÊNCIA — low-poly\n...Limites: X +/-{LIMITE_X:.0} · Y +/-{LIMITE_Y:.0}\n..."` (U+00B7), preservando posição, ilha e controles.
- **FR-003**: carregar `assets/fonts/NotoSans-Regular.ttf` via `AssetServer` (`Handle<Font>`) e aplicar `font: fonte.into()` no `TextFont` do HUD; arquivo com 621572 bytes e SHA-256 `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823`; nenhuma dependência `Cargo` nova.
- **FR-004**: incluir texto integral da licença SIL Open Font License 1.1 em `assets/fonts/OFL.txt` e proveniência em `docs/governance/font-provenance.md` (origem, tamanho, SHA-256, URL primária https://github.com/notofonts/noto-fonts/blob/main/LICENSE, cobertura U+00CA/U+2014/U+00B7).
- **FR-005**: nenhum `▯` no HUD; cobertura dos três glifos comprovada por `fc-match`/`charset` da fonte e por captura visual **[NV]** até o orquestrador.
- **FR-006**: respeitar a regra da janela — agente nunca abre janela normal; verificações visuais usam Xvfb isolado, sem janela no desktop da SE; confirmação final da SE permanece **[NV]**.
- **FR-007**: escrever os 14 artefatos SDD na ordem normativa (`analyze` READ-ONLY **antes** de `tasks`; `converge.md` append-only; R4 aprovado antes de R7 `1.0.0` com changelog FR/T e rollback).
- **FR-008**: escrever `.specify/validations/hud-acento-fonte-livre/test-roadmap.md` **antes do PR** com escopo/FR/T, pré-condições, passos numerados (ação → resultado esperado), superfície CLI/visual isolada, plano de capturas (Xvfb, sem janela no desktop), coluna `automatizável?`, rollback, passo explícito dos três glifos e ausência de `▯`, e passo de hash/licença da fonte.
- **FR-009**: manter fronteira e não-execução: nenhum build/teste/gate/formatador/commit/push/PR; não tocar paths fora da fronteira; factory `b1deab1` somente fonte; histórico do ciclo 029 e PR #40 (`c126b6e`) preservados como fatos observados; pendências **[NV]**.

### Key Entities
- **Glifo HUD**: caractere unicode do título/limites; estado = renderizado ou `▯` (tofu).
- **Fonte embarcada**: TTF livre versionado, identificado por SHA-256 e licença OFL 1.1.
- **Proveniência**: registro que liga o binário embarcado à origem, ao hash, à licença e à cobertura Unicode.
- **Captura Xvfb**: evidência visual isolada, sem janela no desktop da SE; rotulada **[NV]** até execução.

## Clarificações
- “Agora quero o Ê sim” revoga o paliativo ASCII do ciclo 029 para **todos** os três caracteres (`Ê`, `—`, `·`), não apenas para o `Ê`.
- A fonte é copiada de pacote já instalado no sistema; não há download de rede nem dependência Rust nova.
- `docs/evidencias/hud-confirmacao-visual-2026-11-22.png` está na árvore principal e é evidência da grafia ASCII anterior — fora da fronteira deste agente.
- A seção “Contexto e Orçamento” do template é documental e não lida por gates; nenhum manifesto é criado neste ciclo (fora de fronteira).

## Success Criteria
- **SC-001**: captura (quando executada) mostra `INSURGÊNCIA — low-poly` e `·` nos limites, sem `▯`.
- **SC-002**: SHA-256 do TTF confere `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823`; licença OFL 1.1 integral e proveniência presentes.
- **SC-003**: `verify.sh` ALL STAGES OK (41/41) e 5 gates PASS após a mudança **[NV]** até execução pelo orquestrador.
- **SC-004**: 14 artefatos na ordem normativa, `analyze` antes de `tasks`, R4 antes de R7 `1.0.0`, rollback documentado, nada executado por este agente.
- **SC-005**: nenhuma janela normal aberta por agente; capturas apenas em Xvfb isolado; confirmação visual final da SE **[NV]**.
- **SC-006**: PR #40 mesclado (`c126b6e`, 20:52:35Z) registrado como fato observado; histórico 029 intacto.

## Assumptions
- O agente de código aplica a implementação (fonte, strings, licença e proveniência); este agente escreve somente os 14 artefatos SDD.
- Orquestrador executa `verify.sh`, gates, CI e captura Xvfb; a SE faz a confirmação visual final.
- Ambiente headless; nenhuma janela, rede ou credencial é usada por este agente.
- Evidências de 21:15–21:18Z são coladas como observadas, sem reexecução.

## Out of scope
Alterações em contratos de simulação/manifesto, `Cargo.toml`/`Cargo.lock`, `docs/GDD*`, baselines pré-existentes, ledger, CI, `STATE.md`/`PROGRESSO.md`/`feature.json`, factory `b1deab1` (somente fonte), paths untracked alheios, janelas, deploy, assinatura GPG e qualquer build/teste/gate/formatador/commit/push/PR. O que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.
