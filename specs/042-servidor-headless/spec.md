# Feature Specification: Servidor headless determinístico

**Branch**: `042-servidor-headless` (base `b099093`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T043000Z-servidor-headless`, `insurgencia - servidor headless deterministico`.

**Âncora**: decisão 80/20 do ciclo 042 registrada em `.specify/memory/STATE.md` (2026-11-23T04:30Z): servidor headless determinístico (stdin → ticks → hash) **4×4×4×4 = 256/625**, contra dívida documental restante **2×5×5×5 = 250/625**, combate/Fôlego `[HIPÓTESE]` **4×2×2×3 = 48/625** e rede com Lightyear (dependência ausente) **5×2×1×1 = 10/625**; critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. O core `insurgencia-sim` já expõe `Mundo`, `Comando`, `adicionar_jogador`, `agendar_comando`, `avancar_tick`, `hash_estado` e as seis ações de `ACOES_PERMITIDAS`; este ciclo acrescenta **somente** o binário que dirige esse core a partir da stdin. Constituição **v1.2.2 do alvo**: **R6 é N/A** — fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE**, nunca simular issue, nunca `gh` improvisado; registrar dívida de automação. R1 `go`; R4 antes de R7 `1.0.0-local`.

## User Scenarios

### US1 — Executável headless sem nova dependência (P1)

Qualquer terceiro executa `cargo run -p insurgencia-servidor -- --ticks N < entrada.txt` e obtém o estado autoritativo do `insurgencia-sim` como uma sequência de hashes, sem janela, relógio, aleatoriedade, async, socket ou Lightyear.

**Why this priority**: é a autoridade determinística mínima que falta após o core Rust: o mesmo input produz o mesmo estado observável, em qualquer máquina, sem dependência de engine ou protocolo de rede (Princípios III/VI).

**Independent Test**: compilar e executar o binário com `--ticks 1` e stdin vazia; conferir uma única linha `0\t<64 hex>` e nenhuma biblioteca externa nova em `crates/servidor/Cargo.toml` ou `Cargo.toml` da workspace.

**Acceptance Scenarios**:

1. **Given** o workspace no baseline `b099093`, **When** o membro `crates/servidor` é adicionado e o binário compila, **Then** ele depende apenas de `insurgencia-sim` e `Cargo.lock` é gerado pelo Cargo, nunca editado à mão.
2. **Given** uma execução com stdin vazia e `--ticks 1`, **When** o processo termina, **Then** imprime exatamente `0\t<hash sha256>` e nada mais na stdout, sem abrir janela nem conexão.

### US2 — Parser determinístico da stdin (P1)

Um comando por linha, com tokens separados por whitespace: `peer_id acao seq request_id [x y]`. `peer_id` é `u64`, `seq` e `request_id` são `i64`, `x`/`y` são `f64` finitos. As coordenadas são **obrigatórias** para `mover`, `coletar_materiais` e `construir_barreira` e **proibidas** para `sinalizar_sincronia`, `sinalizar_alavanca` e `atacar_forte`.

**Why this priority**: é a fronteira entre entrada legível pelo core e erro silencioso; uma reinterpretação de linha malformada gera estados divergentes sem aviso.

**Independent Test**: testar o parser com as seis ações conhecidas, com coordenadas ausentes/extras e com uma ação desconhecida; conferir os erros determinísticos e a ausência de pânico.

**Acceptance Scenarios**:

1. **Given** a linha `7 mover 1 1 0.5 0.0`, **When** analisada, **Then** produz um `Comando` com `peer_id = 7`, `acao = "mover"`, `seq = 1`, `request_id = 1`, `destino = Some((0.5, 0.0))`.
2. **Given** a linha `7 sinalizar_sincronia 1 1 0.5 0.0`, **When** analisada, **Then** é rejeitada com erro determinístico: ação conhecida não aceita coordenadas.
3. **Given** a linha `7 voar 1 1`, **When** analisada, **Then** é rejeitada como ação desconhecida com código de saída 2, sem pânico.

### US3 — Agendamento em ordem antes do tick 0 (P1)

Todos os comandos válidos da stdin são convertidos em `Comando` e enfileirados via `agendar_comando` na ordem exata de leitura, antes do primeiro `avancar_tick`. Cada `peer_id` visto pela primeira vez é registrado com `adicionar_jogador`; os campos `peer_id`, `acao`, `seq`, `request_id` e `destino` são preservados integralmente.

**Why this priority**: a ordem de agendamento e a preservação dos monotônicos são parte do contrato do core; reordenar, deduplicar ou descartar comandos no binário quebraria a reprodução.

**Independent Test**: enviar três comandos de peers distintos em ordem conhecida e conferir que o hash por tick é idêntico entre duas execuções e que nenhum comando é reorganizado.

**Acceptance Scenarios**:

1. **Given** três linhas válidas lidas em ordem A, B, C, **When** a stdin é fechada, **Then** a fila de pendentes contém A, B, C nessa ordem antes do tick 0.
2. **Given** um `peer_id` ainda não registrado no `Mundo`, **When** seu primeiro comando é lido, **Then** `adicionar_jogador` o registra antes do agendamento, sem inventar autorização ou estado extra.

### US4 — Um hash SHA-256 por tick (P1)

A stdout contém exatamente `N` linhas `<tick>\t<hash>`, com `tick` de `0` a `N-1` e o hash canônico de 64 caracteres hexadecimais retornado por `Mundo::hash_estado()` no início de cada tick; sem cabeçalho, legenda, cor, carimbo de data/hora ou saída de diagnóstico na stdout.

**Why this priority**: é o produto observável do ciclo — a assinatura do estado que permite comparar duas execuções com `diff` e detectar divergência.

**Independent Test**: executar com `--ticks 3`, contar 3 linhas, conferir o padrão `^[0-9]+\t[0-9a-f]{64}$` e comparar duas execuções com `diff`.

**Acceptance Scenarios**:

1. **Given** `--ticks 3` e comandos válidos, **When** o processo termina com sucesso, **Then** a stdout tem as linhas `0\t<hash0>`, `1\t<hash1>` e `2\t<hash2>`, nessa ordem.
2. **Given** a mesma entrada em duas execuções, **When** as saídas são comparadas, **Then** o `diff` é vazio porque nenhum relógio, rede, aleatoriedade ou ordem de iteração não determinística participa do resultado.

### US5 — Erro determinístico, sem pânico (P2)

Argumentos inválidos (`--ticks` ausente, zero, negativo ou não inteiro) e qualquer linha malformada, com ação desconhecida ou aridade de coordenadas incorreta, encerram o processo com código de saída **2**, mensagem estável na stderr (com o número da linha, 1-based, quando aplicável) e nenhuma linha na stdout.

**Why this priority**: falha ruidosa e precoce é preferível a estado divergente; erro determinístico também é verificável por teste de integração.

**Independent Test**: executar com `--ticks 0`, com uma linha `foo` e com coordenadas proibidas; conferir exit 2, stderr estável e stdout vazia em cada caso.

**Acceptance Scenarios**:

1. **Given** a linha 4 com tokens insuficientes, **When** o parser a encontra, **Then** a execução para antes do tick 0, imprime erro que identifica a linha 4 e retorna exit 2.
2. **Given** um erro de argumento ou de parsing, **When** o processo termina, **Then** não há `panic!`, `unwrap`, `expect` ou saída parcial na stdout.

### US6 — Estilo, testes e rastreabilidade (P2)

O crate segue `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, erros via `Result`/`ExitCode` e documentação `///` em itens públicos. Testes unitários cobrem o parser; o teste de integração da CLI compara duas execuções, conta linhas e rejeita entrada inválida.

**Why this priority**: sem essas provas, o determinismo é afirmação, não evidência; o roteiro de verificação e o R4 dependem delas.

**Independent Test**: inspecionar os atributos do crate e rodar os testes unitários e de integração (pelo orquestrador, **[NV]** neste agente de documentação).

**Acceptance Scenarios**:

1. **Given** o código do crate, **When** inspecionado, **Then** não há `unsafe`, `unwrap`, `expect`, `#[allow]` nem item público sem `///`.
2. **Given** os testes, **When** executados, **Then** cobrem identidade de saída, contagem de `N` linhas e exit 2 para entrada inválida.

### Edge Cases

- `--ticks` ausente, `0`, negativo, não inteiro ou com texto extra: erro determinístico, exit 2, stdout vazia.
- stdin vazia: `N` linhas com o hash inicial do `Mundo`, sem nenhum comando aplicado.
- Linha em branco (exceto o terminador final), tokens em número incorreto ou campo numérico inválido: erro determinístico com número da linha.
- `x`/`y` não finitos (`NaN`, `inf`) ou com mais de dois valores: erro determinístico, sem ataques de arredondamento implícito.
- Coordenadas presentes em `sinalizar_sincronia`, `sinalizar_alavanca` ou `atacar_forte`: erro determinístico; coordenadas ausentes em `mover`, `coletar_materiais` ou `construir_barreira`: erro determinístico.
- Ação fora das seis conhecidas: erro determinístico, mesmo que os demais campos estejam bem formados.
- `peer_id` repetido ou `seq`/`request_id` repetidos/decrescentes: o parser não deduplica e não inventa recusa; a validação semântica permanece no core `insurgencia-sim`, fora da fronteira deste ciclo.

## Requirements

### Functional Requirements

- **FR-001**: criar o crate `crates/servidor` com o binário `insurgencia-servidor`, dependendo **apenas** de `insurgencia-sim`; nenhuma dependência externa nova, nenhum `async`, socket, Lightyear, janela ou relógio. O `Cargo.toml` da workspace ganha o membro; `Cargo.lock` é gerado pelo Cargo, nunca editado manualmente.
- **FR-002**: aceitar a opção `--ticks N` com `N` inteiro positivo; argumento ausente, não inteiro, zero ou negativo produz erro determinístico e código de saída 2, sem pânico e sem qualquer linha na stdout.
- **FR-003**: ler a stdin linha a linha, com um comando por linha no formato `peer_id acao seq request_id [x y]` (`u64`, string de ação, `i64`, `i64`, `f64` finito). Coordenadas são obrigatórias para `mover`, `coletar_materiais` e `construir_barreira` e proibidas para `sinalizar_sincronia`, `sinalizar_alavanca` e `atacar_forte`. Linha malformada ou ação desconhecida gera erro determinístico na stderr (com número da linha, 1-based, quando aplicável), exit 2, nenhuma saída na stdout e nenhum pânico.
- **FR-004**: converter cada linha válida em `Comando` e chamá-lo via `agendar_comando` na ordem exata de leitura **antes do tick 0**, preservando `peer_id`, `acao`, `seq`, `request_id` e `destino`; registrar cada `peer_id` com `adicionar_jogador` na primeira aparição; não reordenar, deduplicar, filtrar ou reescrever comandos no binário.
- **FR-005**: emitir exatamente `N` linhas na stdout, uma por tick, no formato `<tick>\t<hash sha256>` com `tick` de `0` a `N-1` e o hash canônico de 64 caracteres hexadecimais de `Mundo::hash_estado()` no início de cada tick; sem cabeçalho, ruído ou registros de data/hora. Duas execuções com a mesma entrada devem produzir bytes idênticos.
- **FR-006**: estilo e evidências obrigatórios: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, erros via `Result`/`ExitCode`, documentação `///` em itens públicos; testes unitários do parser e teste de integração da CLI cobrindo saída idêntica entre duas execuções, contagem de linhas e rejeição de entrada inválida. R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A** por v1.2.2; gates, build, testes, formatadores, commit, push e PR permanecem **[NV]** para o agente de documentação.

### Key Entities

- **`insurgencia-servidor`**: binário do novo crate `crates/servidor`, dirigido por argumento `--ticks` e pela stdin; sem estado próprio além do `Mundo` do core.
- **`Comando`**: estrutura do `insurgencia-sim` com `peer_id`, `acao`, `seq`, `request_id` e `destino: Option<(f64, f64)>`; produzida pelo parser sem perda de campos.
- **`Mundo`**: estado autoritativo do core; usado via `new`, `adicionar_jogador`, `agendar_comando`, `avancar_tick` e `hash_estado` sem reimplementar regras de simulate.
- **Hash canônico**: `Mundo::hash_estado()`, SHA-256 já serializado no core; a linha de saída é `<tick>\t<hash>` e nenhum outro hashing é introduzido.
- **Erro determinístico**: tipo local, convertido em mensagem estável na stderr e `ExitCode` 2; nunca `panic!`.

## Clarificações

- As seis ações conhecidas são as de `ACOES_PERMITIDAS` no `insurgencia-sim`: `mover`, `sinalizar_sincronia`, `sinalizar_alavanca`, `coletar_materiais`, `construir_barreira` e `atacar_forte`. Qualquer outra string é "ação desconhecida" (exit 2).
- O hash exibido na linha `t` é o snapshot do estado **no início** do tick `t`, depois de todos os comandos terem sido enfileirados. Os comandos pendentes são consumidos pelo primeiro `avancar_tick()`, que leva o `Mundo` do tick 0 ao tick 1; por isso a última linha impressa é `N-1`.
- "Erro determinístico" significa mensagem fixa, sem carimbo de data/hora, caminhos absolutos, endereços de memória ou detalhes de depuração; a contagem de linha é 1-based e a primeira falha encerra a leitura.
- O parser valida **forma** (tipos, aridade, coordenadas finitas e proibidas/obrigatórias) e **nome da ação**; não aplica `ValidacaoComandos::validar` nem inventa recusas semânticas (replay, distância, saldo). Essas regras continuam no core `insurgencia-sim` e não são reabertas neste ciclo.
- `peer_id` é registrado na primeira aparição para que o comando tenha efeito no `Mundo`; a ordem de iteração do `BTreeMap` do core já é determinística e nenhuma autorização externa é introduzida.
- `Cargo.lock` é artefato gerado pelo Cargo: nenhum agente edita o arquivo manualmente.
- **R6 é N/A** por constituição v1.2.2: não é dívida executada, nem regra ativa, nem simulação de issue; se invocada, PARE e escale ao SWE. A dívida de automação (`resolve-tasks.sh`/adaptador) fica registrada, sem `gh` improvisado.
- A seção "Contexto e Orçamento" do template é documental e não lida por gates; nenhum manifesto é criado neste ciclo (fora de fronteira).

## Success Criteria

- **SC-001**: `cargo run -p insurgencia-servidor -- --ticks 5 < entrada.txt` termina com exit 0 e imprime exatamente 5 linhas, `0..4`, cada uma no formato `<tick>\t<64 hex>`; nenhuma linha extra, cabeçalho ou ruído na stdout.
- **SC-002**: duas execuções com a mesma entrada e o mesmo `--ticks` produzem stdout byte a byte idêntico (`diff` vazio); nenhum relógio, socket, aleatoriedade ou iteração não determinística influencia o resultado.
- **SC-003**: `--ticks` inválido e cada classe de entrada inválida (linha malformada, ação desconhecida, coordenadas ausentes ou proibidas, número não finito) produzem erro determinístico na stderr, exit 2, stdout vazia e nenhum pânico.
- **SC-004**: `crates/servidor/Cargo.toml` declara somente `insurgencia-sim`; workspace inclui o novo membro; não há `async`, socket, Lightyear ou nova dependência externa; `Cargo.lock` foi gerado pelo Cargo.
- **SC-005**: testes unitários do parser e teste de integração da CLI (identidade entre duas execuções, contagem de `N` linhas, rejeição de entrada inválida); código com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, `Result`/`ExitCode` e `///` em itens públicos.
- **SC-006**: 6 FR e 6 SC; T001..T012 únicos em DAG; plano com passos 00→26, checklist com `[x]` = revisão de requisitos, roteiro de 10 passos, R6 N/A e fronteira explícita; scorecard 256/625 e alternativas 250/625, 48/625 e 10/625; gates/build/testes **[NV]** até execução pelo orquestrador.

## Assumptions

- O baseline é `b099093`, branch `042-servidor-headless`, worktree `.specify/queue/worktrees/servidor-headless-042/`, sessão `20261123T043000Z-servidor-headless`.
- A API pública de `insurgencia-sim` já existente é suficiente; nenhuma alteração no core, em `crates/contratos`, `PROTOCOL_VERSION`, manifesto ou CI é necessária.
- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam o código Rust na etapa 12; o orquestrador executa gates, build e verificações finais.
- Ambiente headless: nenhuma janela, rede, credencial ou comando `gh` é usado na redação destes artefatos.
- A sequência executável da constituição v1.2.2 vai de `00` a `26`, com `27 validate` imediatamente antes de `26 release`; `tasks` precede `analyze` (READ-ONLY).

## Out of scope

Async, sockets, Lightyear, netcode, TCP/UDP, janela, engine, credenciais, deploy, novas dependências externas, alteração em `crates/contratos`, `PROTOCOL_VERSION`, manifesto ou CI, reabertura de regras semânticas do core, adoção de R6, `taskstoissues`, `resolve-tasks.sh` ou simulação de issue. Tudo o que não foi executado permanece **[NV]**; nenhum commit, push ou PR é feito por este agente.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = **625**, decidido em 2026-11-23T04:30Z:

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Servidor headless determinístico (stdin → ticks → hash) | 4 | 4 | 4 | 4 | **256/625** |
| Dívida documental restante | 2 | 5 | 5 | 5 | 250/625 |
| Combate/Fôlego (`[HIPÓTESE]`) | 4 | 2 | 2 | 3 | 48/625 |
| Rede com Lightyear (dependência ausente) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `servidor-headless`. É o maior ganho de autoridade sem dependência externa: um binário que possui `Mundo`, consome comandos de entrada determinística e emite o hash por tick. Prepara o caminho para rede sem introduzir Lightyear, async ou sockets neste ciclo. R6 permanece **N/A** por constituição v1.2.2.
