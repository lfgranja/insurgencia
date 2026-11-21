# Feature Specification: Instrumentação de Métricas no Núcleo — 20 Partidas (`crates/metricas`)

**Feature Branch**: `feature/024-instrumentacao-metricas` · **Created**: 2026-11-21T01:03Z · **Status**: Draft · **Data:** 2026-11-21

**Input**: `.specify/feature.json` aponta `specs/024-instrumentacao-metricas`. `.specify/memory/STATE.md` (estado vivo 2026-11-21T00:22Z) reorientou a prioridade da SE para o MVP jogável single-player, cujos itens 2 e 3 são, respectivamente, **duas partidas consecutivas, uma com cliente modificado (GDD:1567)** e **instrumentação no Núcleo + 20 partidas com as métricas da §17 (GDD:1605–1622)**. A spec 023 entregou o cliente greybox; os crates `insurgencia-contratos` (`crates/contratos`), `insurgencia-sim` (`crates/sim`, Núcleo) e `insurgencia-cliente` (`crates/cliente`) já estão mesclados. A GDD §17 exige que a instrumentação registre eventos **no Núcleo, nunca apenas no cliente**, e que as metas sejam recalibradas após 20 partidas instrumentadas. Gate Rust de referência: `verify.sh` (`verify.sh: ALL STAGES OK`). Nenhuma medição, execução de `cargo`, abertura de janela, stage, commit ou push foi observada nesta etapa; todo resultado permanece **[NV]**.

## User Scenarios & Testing *(mandatory)*

As quatro histórias abaixo são ordenadas por dependência estrutural. Cada uma é testável de forma independente por leitura dos artefatos; nenhuma pressupõe toolchain instalada, gate executado, partida rodada ou alegação de execução observada nesta etapa.

### User Story 1 — Eventos de telemetria registrados no Núcleo (Priority: P1)

Como mantenedor do Insurgência, preciso que o Núcleo (`crates/sim`) registre capturas de ilha, tentativas de sincronia e divergências cliente-servidor como eventos de telemetria com definição operacional e fonte de dados declaradas, para que as métricas da GDD §17 tenham origem auditável e não dependam do cliente.

**Why this priority**: é o pré-requisito das demais histórias — sem os três eventos no Núcleo, não há como calcular mediana/p95, reproduzir a partida modificada ou gerar o relatório (GDD §17: "O Núcleo grava capturas, tentativas de sincronia e divergências cliente-servidor antes da Fase 4").

**Independent Test**: ler a interface de eventos em `crates/metricas` e os pontos de emissão em `crates/sim`, confirmando três variedades de evento (`Captura`, `TentativaSincronia`, `DivergenciaClienteServidor`), cada uma com rótulo de partida, tempo monotônico desde o início da partida e fonte declarada; nenhum comando é executado nesta etapa.

**Acceptance Scenarios**:

1. **Given** o Núcleo instrumentado, **When** uma ilha de rota é capturada, **Then** o Núcleo emite um evento `Captura` com `partida_id`, `ilha`, `tempo_desde_inicio` e `fonte = "nucleo"`, e o cliente não é a fonte do registro (Q1).
2. **Given** a janela de 5 s da Ilha Meio, **When** duas Ativações são recebidas, **Then** o Núcleo emite uma `TentativaSincronia` por tentativa com desfecho `sucesso` ou `recusa`, e a taxa da §17 é derivada desses eventos.
3. **Given** um cliente que forja inputs, **When** o servidor corrige ou recusa o comando, **Then** o Núcleo emite `DivergenciaClienteServidor` com `comando`, `valor_enviado`, `valor_corrigido` e `resultado`, sem aceitar estado pronto (Q3).

---

### User Story 2 — Registro de 20 partidas com as 5 métricas da §17 (Priority: P1)

Como avaliador da tese, preciso de 20 registros de partida com mediana e p95 das cinco métricas instrumentadas — tempo até primeira captura, taxa de conclusão do desafio sincronizado, latência percebida, taxa de partidas concluídas e taxa de divergência cliente-servidor —, para que as metas `[HIPÓTESE]` da GDD §17 sejam recalibradas com amostra mínima em vez de sorte.

**Why this priority**: é o entregável 3 da prioridade da SE e a condição explícita da GDD §17 ("As metas são revisadas uma única vez após 20 partidas"); o número 20 é a amostra mínima declarada, não um limite aberto.

**Independent Test**: ler os registros e o agregador em `crates/metricas` e confirmar 20 de 20 registros, cada um com as cinco métricas e com mediana/p95 calculados a partir dos eventos de US1; nenhuma partida é executada nesta etapa.

**Acceptance Scenarios**:

1. **Given** os eventos de 20 partidas, **When** o agregador é acionado, **Then** existem exatamente 20 registros com `partida_id` único e as cinco métricas da §17 preenchidas, sem valor copiado, ausente ou inferido sem fonte (SC-002, SC-003).
2. **Given** os 20 registros, **When** as métricas são agregadas, **Then** mediana e p95 são calculados para tempo até primeira captura e latência percebida (RTT), e as taxas de sincronia, partidas concluídas e divergência são razões declaradas, todas com meta `[HIPÓTESE]` registrada (SC-003).
3. **Given** métricas fora dos limiares `[HIPÓTESE]`, **When** o relatório é gerado, **Then** o desvio aparece como observação datada, sem alterar a meta na especificação – a recalibração é única, após as 20 partidas (Q2).

---

### User Story 3 — Duas partidas consecutivas, uma com cliente modificado (Priority: P1)

Como avaliador do fechamento do MVP, preciso que os critérios de aceitação passem em duas partidas consecutivas, uma delas com cliente modificado que forja inputs, com as divergências registradas e corrigidas pelo servidor autoritativo, para que GDD:1567 seja satisfeito sem aceitar estado pronto.

**Why this priority**: é o entregável 2 da prioridade da SE e a própria definição de "MVP fechado" em GDD:1567; depende de US1 para existir e é testável isoladamente pela sequência das duas partidas e pelo registro de divergência.

**Independent Test**: ler o roteiro das duas partidas e os eventos persistidos, confirmando sequência consecutiva (partida N e N+1), cliente modificado em exatamente uma delas, inputs forjados recusados/corrigidos e ao menos uma `DivergenciaClienteServidor` com `resultado = "corrigido"` ou `"recusado"`; nenhuma partida é executada nesta etapa.

**Acceptance Scenarios**:

1. **Given** o roteiro de fechamento, **When** as duas partidas são executadas em sequência, **Then** a partida N é limpa e a N+1 usa cliente modificado, com 2 de 2 partidas concluídas e nenhuma rodada interveniente (GDD:1567, SC-004).
2. **Given** o cliente modificado na segunda partida, **When** ele envia posição pronta, dano declarado ou captura sem condição prévia, **Then** o servidor recusa ou corrige, o Núcleo registra a divergência e a ilha permanece neutra quando não há condição (GDD:1178, 1192, 1516, 1638; Q3).
3. **Given** as divergências registradas, **When** a partida modificada termina, **Then** o relatório lista as divergências por cliente para auditoria, sem exigir acesso ao equipamento do jogador, e nenhuma divergência fica sem correção (GDD §18, SC-004).

---

### User Story 4 — Relatório datado das 20 partidas (Priority: P2)

Como mantenedor do hub, preciso de um relatório datado que consolide os 20 registros, as cinco métricas com mediana/p95 e o desfecho das duas partidas consecutivas, para que a revisão única das metas `[HIPÓTESE]` seja rastreável por data e fonte.

**Why this priority**: é o entregável documental que torna as medições auditáveis; depende de US1–US3 para ter conteúdo, mas é verificável isoladamente pela presença da data, dos 20 registros, das 5 métricas e das referências GDD.

**Independent Test**: ler `docs/metricas/2026-11-21-relatorio-instrumentacao.md` e confirmar data/hora no cabeçalho, 20 registros referenciados, 5 métricas com definição operacional e fonte, o par de partidas consecutivas e a explicitação de que toda meta permanece `[HIPÓTESE]` até a recalibração; nenhum comando é executado nesta etapa.

**Acceptance Scenarios**:

1. **Given** o relatório materializado, **When** ele é lido, **Then** contém data/hora (2026-11-21), as referências GDD:1567 e GDD:1605–1622, 20 registros, 5 métricas e 0 valores sem fonte declarada (SC-005).
2. **Given** o fechamento do ciclo, **When** a verificação é feita, **Then** `verify.sh` termina com `verify.sh: ALL STAGES OK` e os cinco gates determinísticos permanecem PASS e inalterados, sem modificar `Makefile`, baselines ou `evidence.csv` (SC-001, SC-005).

---

### Edge Cases

- **Partida sem captura**: tempo até primeira captura não tem amostra; o registro mantém o campo vazio com razão declarada, e mediana/p95 são calculados somente sobre partidas com captura — nenhum valor zero é inventado.
- **Janela de 5 s sem tentativa**: razão `0/0` é registrada como `sem_amostra`, não como 0% nem como 100%.
- **RTT ausente**: se o cliente não reportar RTT, o campo fica `sem_amostra`; o Núcleo registra o valor recebido, nunca o estima (Q1).
- **Divergência já corrigida**: uma segunda correção idêntica no mesmo comando é deduplicada por `partida_id + comando + sequência`, evitando inflar a taxa de divergência.
- **Empate na mediana/p95**: p95 é calculado por interpolação determinística sobre a amostra ordenada, sem aleatoriedade e sem relógio de parede; a mesma entrada produz a mesma saída.
- **Cliente modificado sem divergência**: se nenhum input forjado for detectado, o desfecho é registrado como `0 divergências` e a partida não é descartada — a ausência é dado, não falha.
- **Servidores comunitários**: nenhuma métrica depende de servidor comunitário, federação ou forks publicados; "servidor ativo" e "módulo publicado" ficam fora deste ciclo (Q4, DEFER do STATE.md).
- **Alegação de execução**: `cargo`, gates, partidas, `verify.sh` ou janela ditos executados nesta etapa → não observado; permanece **[NV]** (SC-001–SC-005).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O Núcleo (`crates/sim`) DEVE registrar, como eventos de telemetria, capturas de ilha, tentativas de sincronia na janela de 5 s e divergências cliente-servidor, cada um com `partida_id`, sequência monotônica, tempo desde o início da partida e `fonte = "nucleo"`. O cliente NÃO DEVE ser a fonte do registro — ele apenas origina o input ou reporta RTT, que o Núcleo recebe e grava. Nenhum evento de rede, snapshot, persistência remota ou autoridade externa é introduzido (US1, Q1).
- **FR-002**: Um novo crate `crates/metricas` (pacote `insurgencia-metricas`) DEVE ser materializado no workspace `members = ["crates/*"]` com `edition = "2024"`, contendo os tipos de evento, o agregador determinístico de mediana/p95 e o escritor de registros/relatório. O crate DEVE depender apenas dos tipos necessários já presentes no workspace, sem adicionar dependência de rede, banco, HTTP, OTLP ou Uptime Kuma (US1, US2, Q4).
- **FR-003**: Toda métrica incluída no relatório DEVE ter definição operacional e fonte de dados declaradas, na forma da tabela da GDD §17; nenhuma métrica entra no relatório sem esses dois campos (US2, US4).
- **FR-004**: O agregador DEVE produzir exatamente 5 métricas — (M1) tempo até primeira captura de ilha, com mediana e p95; (M2) taxa de conclusão do desafio sincronizado; (M3) latência percebida, com RTT mediano e p95 e percentual de respostas "sem atraso perceptível"; (M4) taxa de partidas concluídas; (M5) taxa de divergência cliente-servidor — cada uma com meta `[HIPÓTESE]`: M1 mediana ≤ 90 s e p95 ≤ 150 s; M2 ≥ 60% e ≤ 90%; M3 RTT mediano ≤ 80 ms em servidor local e p95 ≤ 250 ms, com ≥ 90% de respostas positivas; M4 ≥ 50%; M5 ≤ 1% das partidas com divergência não corrigida. As métricas "Servidores comunitários ativos" e "Diversidade de módulos e forks publicados" da §17 ficam DEFER e NÃO entram no conjunto de 5 (Q2, Q4).
- **FR-005**: O registro DEVE conter exatamente 20 partidas com `partida_id` único, sem duplicata, sem registro inferido e sem valor copiado de outra partida; o número 20 é a amostra mínima da GDD §17 e a meta é revisada uma única vez ao final (US2, SC-002).
- **FR-006**: As duas partidas de fechamento DEVEM ser consecutivas (N e N+1, sem rodada interveniente), uma delas com cliente modificado que simula **inputs forjados** — posição pronta, dano declarado acima do real ou captura sem condição prévia. O servidor DEVE recusar ou corrigir cada input, o Núcleo DEVE registrar a divergência correspondente, e nenhuma divergência DEVE ficar sem correção (GDD:1178, 1192, 1516, 1567, 1638; US3, Q3).
- **FR-007**: O relatório `docs/metricas/2026-11-21-relatorio-instrumentacao.md` DEVE ser datado (data/hora), conter os 20 registros, as 5 métricas com mediana/p95 e definição operacional/fonte, o desfecho das duas partidas consecutivas, as referências GDD:1567 e GDD:1605–1622 e a afirmação explícita de que toda meta permanece `[HIPÓTESE]` até a recalibração única. Nenhum artefato R4/R7 é escrito nesta etapa de especificação (US4, SC-005).
- **FR-008**: Todo código Rust adicionado DEVE declarar `unsafe_code = "forbid"`, `clippy::pedantic = "deny"` e `unwrap_used`/`expect_used` em `deny`, tratando falhas com `Result<T, E>`, `?` e `thiserror` onde aplicável, sem alterar `Cargo.toml`/`clippy.toml` para desativar lint (única exceção: `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado). `verify.sh` DEVE permanecer a única fonte de verdade do gate local, com quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — terminando com `verify.sh: ALL STAGES OK`. Os cinco gates determinísticos, `Makefile`, baselines e `evidence.csv` NÃO DEVEM ser alterados.

### Key Entities *(include if feature involves data)*

- **Evento de telemetria**: registro emitido pelo Núcleo com variedade (`Captura`, `TentativaSincronia`, `DivergenciaClienteServidor`), `partida_id`, sequência, tempo monotônico e fonte declarada; base de todas as métricas.
- **Registro de partida**: entrada única por partida (20 no total), com identificador, eventos associados e as cinco métricas calculadas.
- **Métrica**: definida por nome, definição operacional, fonte, valor observado (mediana/p95 ou taxa) e meta `[HIPÓTESE]`; M1–M5 conforme FR-004.
- **Cliente modificado**: cliente de teste que forja inputs (não uma posição autoritativa); o servidor permanece autoritativo e registra a correção/recusa como divergência.
- **Relatório datado**: documento `docs/metricas/2026-11-21-relatorio-instrumentacao.md` que consolida registros, métricas, partidas consecutivas e referências normativas.
- **Crate `insurgencia-metricas`**: unidade em `crates/metricas`, membro do workspace, sem rede, responsável por tipos de evento, agregação determinística e escrita do relatório.

## Success Criteria *(mandatory)*

Critérios a aferir na implementação e em R4. Nenhum comando de `cargo`, gate, teste, build ou formatador foi executado nesta etapa de especificação; as metas abaixo são alvos, não resultados observados aqui.

### Measurable Outcomes

- **SC-001**: Na implementação/R4, `verify.sh` termina com `verify.sh: ALL STAGES OK` e 4 de 4 estágios OK na ordem `cargo check --locked --workspace --all-targets` → `cargo clippy --workspace --all-targets --all-features -- -D warnings` → `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) → `cargo fmt --all -- --check`, com 0 estágios vermelhos e 0 lints desativados; os 5 gates determinísticos alvo permanecem PASS e inalterados. (baseline: estado vivo 5/5 PASS e `verify.sh` ALL STAGES OK; meta: 4 de 4 + 5 de 5; nesta etapa: **[NV]**)
- **SC-002**: Existem 20 de 20 registros de partida com `partida_id` único, 0 duplicatas e 0 registros inferidos, cada um com os eventos de US1 associados. (baseline: nenhuma instrumentação; meta: 20 de 20; nesta etapa: **[NV]**)
- **SC-003**: 5 de 5 métricas (M1–M5) estão presentes em 100% dos 20 registros, com definição operacional e fonte declaradas e mediana/p95 calculados por agregação determinística quando aplicável — 0 métricas sem fonte, 0 valores copiados e 0 campos `sem_amostra` tratados como zero. (baseline: 0 métricas instrumentadas; meta: 5 de 5; nesta etapa: **[NV]**)
- **SC-004**: 2 de 2 partidas consecutivas registradas, com exatamente 1 cliente modificado, inputs forjados recusados/corrigidos e ≥ 1 `DivergenciaClienteServidor` com `resultado = "corrigido"` ou `"recusado"`, 0 divergências sem correção e 0 aceitação de estado pronto. (baseline: nenhuma partida de fechamento instrumentada; meta: 2 de 2; nesta etapa: **[NV]**)
- **SC-005**: Exatamente 1 relatório datado (`docs/metricas/2026-11-21-relatorio-instrumentacao.md`) com data/hora, 20 registros, 5 métricas, o par consecutivo, as referências GDD:1567 e GDD:1605–1622 e a ressalva `[HIPÓTESE]`, com `Makefile`, baselines e `evidence.csv` inalterados. (baseline: nenhum relatório; meta: 1 de 1; nesta etapa: **[NV]**)

## Clarifications

Resoluções das quatro ambiguidades detectadas no texto de entrada, com a fonte local que as autoriza:

- **Q1 — Onde fica a instrumentação?** No **Núcleo (`crates/sim`)**, nunca apenas no cliente. O cliente pode originar input e reportar RTT, mas quem registra o evento é o Núcleo; nenhum registro é confiado ao cliente. Fonte: GDD §17 ("A instrumentação registra eventos no Núcleo, nunca apenas no cliente") e STATE.md (stack Rust com `crates/sim` já mesclado).
- **Q2 — Qual é o valor das metas?** Toda meta é **[HIPOTESE]** e permanece marcada assim nesta especificação; os limiares (mediana ≤ 90 s / p95 ≤ 150 s; 60–90%; RTT ≤ 80 ms local, p95 ≤ 250 ms, ≥ 90% positivas; ≥ 50%; ≤ 1% de divergência) são alvos de recalibração, não resultado medido. A revisão ocorre uma única vez após as 20 partidas. Fonte: GDD §17 e GDD:1685–1690 (H8–H10).
- **Q3 — O que significa "cliente modificado"?** Um cliente de teste que **simula inputs forjados** — posição pronta, dano declarado acima do real ou captura sem condição prévia — para verificar que o servidor autoritativo recusa/corrige e que o Núcleo registra a divergência. Não é leitura de memória, não é driver de kernel e não é modificação do Núcleo. Fonte: GDD:1178, 1192, 1516, 1567, 1638 e critérios anti-trapaça da GDD §18.
- **Q4 — Há servidores comunitários neste ciclo?** **Não.** Nenhuma métrica depende de servidor comunitário, federação ou forks publicados; "Servidores comunitários ativos" e "Diversidade de módulos e forks publicados" ficam DEFER, e o aborto do servidor de referência também está fora de escopo. Nenhum pipeline OTLP/Uptime Kuma é introduzido: o relatório datado é o substituto documental. Fonte: STATE.md (DEFER explícito itens 5 e 6), GDD §17 e docs/avaliacao-stack (N45/Uptime Kuma).

## Assumptions

- A ADR-004 (Accepted, 20/11/2026) fixa Bevy 0.20.1 e toolchain 1.99.1; os crates `contratos`, `sim` e `cliente` permanecem intactos em suas responsabilidades — este ciclo acrescenta `crates/metricas` e pontos de emissão no Núcleo, sem trocar engine, sem Rust no Núcleo Godot histórico e sem reabrir ADR-001.
- O workspace raiz já declara `members = ["crates/*"]` e `resolver = "2"`; `verify.sh` é a única fonte de verdade do gate Rust e os cinco gates determinísticos, `Makefile`, baselines e `evidence.csv` não são modificados.
- As 20 partidas e o par consecutivo pertencem à implementação/R4; nenhuma foi executada nesta etapa. Toda saída permanece **[NV]** até registro com data/hora em R4, que precede R7 (SemVer, changelog com IDs FR/T e rollback).
- A amostra de 20 partidas usa execução determinística e repetível, sem relógio de parede e sem aleatoriedade; o tempo registrado é monotônico desde o início da partida, não data/hora do sistema.

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
