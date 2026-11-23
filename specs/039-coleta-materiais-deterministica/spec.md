# Feature Specification: Coleta determinística de materiais

**Branch**: `039-coleta-materiais-deterministica` (base `990ab06`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T015356Z-coleta-materiais`, `insurgencia - coleta deterministica de materiais`.

**Âncora**: GDD §7.1 (pulso de 30 s, Fase 1 "Coletar") e `crates/sim/src/lib.rs` lidos nesta sessão; contrato técnico auditado pelo orquestrador. Constituição **v1.2.2 do alvo** (ratificada 2026-11-19, última emenda 2026-11-23). Este ciclo é **documental**: escreve os 14 artefatos SDD; a implementação em `crates/sim/src/lib.rs`, gates e PR ficam ao SWE/orquestrador e permanecem **[NV]**. **R6 fora do fluxo** por decisão ratificada — PARE se invocada, nunca simular issues.

## User Scenarios

### US1 — Regra canônica de coleta registrada (P1)
Auditoria/sessão seguinte leem os artefatos e encontram a regra do GDD §7.1 sem diluição: 6 s (180 ticks a 1/30 s) em ponto neutro → exatamente +3 Suprimentos e +2 Materiais; sair antes zera o acúmulo parcial.

**Why this priority**: é a regra de aceitação canônica do pulso de 30 s; sem ela o sim não tem recompensa de Fase 1 e o Pilar de Ação Coletiva não se concretiza.

**Independent Test**: ler `intake.md`/`research.md`/`spec.md` e confrontar com GDD §7.1 (linhas ~299–314).

**Acceptance Scenarios**:
1. **Given** o GDD §7.1, **When** os artefatos são lidos, **Then** constam 180 ticks a 1/30 s e a recompensa exata +3/+2.
2. **Given** uma saída antes de 180 ticks, **When** o contrato é lido, **Then** o acúmulo parcial aparece como zerado, nunca creditado.

### US2 — Contrato determinístico rastreável (P1)
O contrato técnico auditado está registrado: `TICKS_COLETA=180`; campos `materiais`, `suprimentos`, `coleta_ticks`, `coleta_ponto` em `Jogador`; `ValidacaoComandos::validar` recusa `DestinoInvalido` para `coletar_materiais` sem destino ou com destino ≠ posição; `fase_skills` acumula e credita atomicamente; `hash_estado` inclui os novos campos. `Comando.destino` é o ID do ponto neutro, sem nova mensagem.

**Why this priority**: toca o estado autoritativo; validação, crédito atômico e hash refletivo são o que mantêm determinismo entre pares.

**Independent Test**: ler `plan.md`/`tasks.md`/`test-roadmap.md` e confrontar com a API atual de `lib.rs`.

**Acceptance Scenarios**:
1. **Given** `coletar_materiais` sem destino, **When** a validação é descrita, **Then** o resultado é `DestinoInvalido`.
2. **Given** dois estados com estoques/acúmulo distintos, **When** `hash_estado` é descrito, **Then** os resumos divergem porque os quatro campos entram na serialização.

### US3 — Fronteira e protocolo preservados (P1)
Auditoria confirma que a mudança futura se restringe a `crates/sim/src/lib.rs`: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, GUI, rede, GDScript, dano ou barricada; nenhuma entidade "ponto neutro" inventada no protocolo.

**Why this priority**: `"coletar_materiais"` já está em `ACOES_PERMITIDAS` e `destino` já existe em `Comando`; mudar protocolo seria custo e risco desnecessários.

**Independent Test**: ler `plan.md`/`analyze.md`/`release.md` e checar a lista de "fora de escopo".

**Acceptance Scenarios**:
1. **Given** a fronteira do ciclo, **When** revisada, **Then** os 14 artefatos são documentais e nenhum arquivo de código foi editado por este agente.
2. **Given** o desenho futuro, **When** descrito, **Then** usa `Comando.destino` como ID do ponto e não altera `PROTOCOL_VERSION`.

### US4 — Governança v1.2.2 e R6 fora do fluxo (P1)
A cadeia respeita `... 09 checklist → revisão → 10 tasks → 11 analyze (READ-ONLY) → 12 implement → 13 converge → 27 validate → 26 release`, R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only, e registra R6 suspensa por decisão ratificada.

**Why this priority**: é a constituição do próprio alvo; inverter a ordem ou ativar R6 quebraria a governança ratificada.

**Independent Test**: ler `plan.md`, `tasks.md`, `analyze.md`, `validate.md` e o trecho da constituição em `research.md`.

**Acceptance Scenarios**:
1. **Given** a ordem v1.2.2, **When** os artefatos são revisados, **Then** `tasks` precede `analyze` e R4 precede R7.
2. **Given** R6 sem `resolve-tasks.sh`/adaptador, **When** o ciclo é revisado, **Then** consta como suspensa, com PARE se invocada e nenhuma issue simulada.

### US5 — Roteiro pré-PR, 80/20 e não-execução (P2)
`test-roadmap.md` é escrito antes do PR com 10 passos e os casos 180 ticks, saída antes, recusa sem destino, hash divergente, ausência de consumo parcial e determinismo; o 80/20 (180/625) é documentado; nada é executado.

**Why this priority**: preserva verificabilidade prévia e honestidade das evidências (Princípio VI).

**Independent Test**: ler `test-roadmap.md`, `intake.md` e `validate.md`; listar os 14 artefatos.

**Acceptance Scenarios**:
1. **Given** o roteiro, **When** lido, **Then** tem 10 passos numerados, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback e os seis casos.
2. **Given** este agente, **When** o ciclo termina, **Then** nenhum build/gate/formatador/teste/commit/push/PR foi executado e `.specify/memory/STATE.md` permanece intacto.

## Requirements

### Functional Requirements
- **FR-001**: registrar a regra canônica do GDD §7.1: 6 s = 180 ticks a `PASSO_SEGUNDOS=1/30` em ponto neutro rendem exatamente +3 Suprimentos e +2 Materiais; sair antes zera o acúmulo parcial.
- **FR-002**: registrar o contrato técnico auditado: `TICKS_COLETA=180`; campos `materiais`, `suprimentos`, `coleta_ticks`, `coleta_ponto` em `Jogador`; recusa `DestinoInvalido` para `coletar_materiais` sem destino ou com destino ≠ posição; acúmulo e crédito atômicos em `fase_skills`; `hash_estado` inclui os novos campos; `Comando.destino` como ID do ponto sem nova mensagem.
- **FR-003**: restringir a implementação futura a `crates/sim/src/lib.rs`; nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, GDScript, GUI, rede, dano, barricada ou R6; nenhuma entidade de protocolo para ponto neutro.
- **FR-004**: aplicar a constituição v1.2.2 do alvo — ordem `00 → 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09 → revisão → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release`, `analyze` READ-ONLY depois de `tasks`, R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only; **R6 suspensa por decisão ratificada**, fora do fluxo, PARE se invocada, nenhuma issue simulada.
- **FR-005**: documentar o 80/20: coleta de materiais 5×4×3×3 = 180/625 escolhida porque a fila documental está esgotada e este é o menor incremento de produto com valor canônico; documentação pós-v1.2.2 (375/625) já executada no ciclo 038.
- **FR-006**: escrever exatamente os 14 artefatos da fronteira, substituindo o stub de `spec.md`; não editar `.specify/memory/STATE.md` (memória local do SWE, não versionada), `converge.md` de outros ciclos, nem a alteração preexistente em `.specify/feature.json`.
- **FR-007**: escrever `.specify/validations/coleta-materiais-deterministica/test-roadmap.md` **antes do PR**: 10 passos numerados, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback e os casos 180 ticks, saída antes, recusa sem destino, hash divergente, ausência de consumo parcial e determinismo.
- **FR-008**: não executar gates/build/formatador/testes/commit/push/PR/merge/deploy; nada afirmado como verde sem comando executado; pendências **[NV]**; implementação de código e verificação ficam ao SWE/orquestrador.

### Key Entities
- **Regra de coleta**: 6 s/180 ticks em ponto neutro, recompensa +3 Suprimentos/+2 Materiais, acúmulo parcial zerado na saída (GDD §7.1).
- **Ponto neutro (no sim)**: identificado por `Comando.destino`; não é nova entidade de protocolo.
- **Estado do jogador**: `Jogador` com posição, vida, seq, request_id e, após a implementação, materiais, suprimentos, coleta_ticks, coleta_ponto.
- **Hash canônico**: `hash_estado` SHA-256 sobre tick, epoch, match_id e jogadores; deve incluir os novos campos para preservar determinismo.
- **Roteiro de testes**: plano documental pré-PR com 10 passos, casos obrigatórios e rollback.

## Clarificações
- "Ponto neutro" não existe como entidade no sim; o ID é o `destino` do comando, já presente no contrato — não há nova mensagem nem bump de `PROTOCOL_VERSION`.
- "Zerar acúmulo parcial" significa não creditar recursos e reiniciar o contador quando o jogador deixa o ponto antes de 180 ticks; o crédito é atômico ao completar o prazo.
- "Recusa sem destino" cobre tanto `destino=None` quanto destino diferente da posição atual do jogador, ambos com `DestinoInvalido`.
- "Hash divergente" é requisito: estados com estoques/acúmulo diferentes devem produzir resumos diferentes; omitir os campos novos do hash é defeito.
- "Não-execução" vale para este agente: a implementação em `lib.rs` e a execução do roteiro são **[NV]** e pertencem ao SWE/orquestrador.
- R6 está **ratificada, mas suspensa**: se invocada sem `resolve-tasks.sh`/adaptador, PARE e escale; nunca simular issue.
- A seção "Contexto e Orçamento" do template é documental e não lida por gates; nenhum manifesto é criado (fora de fronteira).

## Success Criteria
- **SC-001**: 14 artefatos contêm a regra canônica 180 ticks/6 s → +3/+2 e a perda do acúmulo parcial na saída antecipada.
- **SC-002**: Contrato técnico refletido com fidelidade (campos, validação, crédito atômico, hash, destino como ID do ponto) e sem mudança de protocolo.
- **SC-003**: Ordem v1.2.2 respeitada (`tasks` antes de `analyze` READ-ONLY), R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only e R6 fora do fluxo.
- **SC-004**: 80/20 documentado (180/625 escolhido; fila documental esgotada) com rastreabilidade no intake/concept/decision.
- **SC-005**: `test-roadmap.md` escrito antes do PR com 10 passos, capturas, `automatizável?`, rollback e os seis casos obrigatórios.
- **SC-006**: Nenhum build/gate/formatador/teste/commit/push/PR/merge/deploy executado; `[NV]` em toda pendência; fronteira exata e arquivos alheios preservados.

## Assumptions
- Este agente escreve somente os 14 artefatos SDD; SWE/orquestrador implementa `lib.rs`, executa gates/roteiro e decide PR.
- Ambiente headless; nenhuma janela, rede ou credencial é usada por este agente.
- Evidências de GDD/`lib.rs` e da constituição são coladas como lidas nesta sessão, sem reexecução de gates; nada é afirmado como verde.

## Out of scope
Implementação em `crates/sim/src/lib.rs` por este agente; `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`; barricada, dano/emboscada, GUI, rede, GDScript; entidade "ponto neutro" no protocolo; execução de gates/build/testes/formatador/commit/push/PR/merge/deploy; adoção de R6, `taskstoissues`/`resolve-tasks.sh` ou simulação de issues; edição de `converge.md` existente, `.specify/memory/STATE.md`, `docs/GDD*`, baselines, ledger, CI, `feature.json` ou paths untracked alheios. Tudo o que não foi executado permanece **[NV]**.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*
> Seção aditiva e documental: nenhum gate lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.
