# Feature Specification: Validação do Servidor Headless — whitelist única e recusa determinística

**Branch**: `feature/056-servidor-validacao` (base `0bc2c25`) · **Created**: 2026-11-24 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261124T102734Z-servidor-validacao`, `insurgencia - validacao do servidor headless`.

**Âncora**: `AGENTS.md` (crates/servidor: headless `--ticks N` CLI, lê `peer_id acao seq request_id [x y]` da stdin, **valida antes de emitir**, agenda em ordem e imprime `<tick>\t<sha256>` por tick; sem sockets, async ou Lightyear) e a auditoria [EV] de `2026-11-24T10:29Z` em `.specify/memory/STATE.md`. O Núcleo tem **13 ações canônicas** em `ACOES_PERMITIDAS` (`crates/sim/src/lib.rs:188-202`), `ValidacaoComandos::validar` (`sim:2098`) e mapa de níveis Q1/W2/E3/R6 (`sim:214-220`), mas **nenhum binário chama o validador**; o servidor headless mantém whitelist própria de **6 ações** (`servidor/src/main.rs:16-24`), aridade 4/6 e `exige_destino` hardcoded para 3 ações (`:83-116`). A decisão 80/20 do ciclo 056, registrada em `.specify/memory/STATE.md` (2026-11-24T10:29Z), escolheu **`servidor-validacao` (BF-011) com 4×5×4×4×4 = 1280/3125** contra HUD `hash_estado` **1125/3125** (BF-014), cliente `mover` **240/3125** (BF-012), contratos consumidos **100/3125** (BF-013) e rede **50/3125 — inelegível** (BF-001, autorização ADR-004). Critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵ = 3125.

**Rastreabilidade canônica (eco dos artefatos do ciclo)**: FR-001 whitelist única derivada das 13 ações canônicas e `acao_exige_destino` pública no sim; FR-002 parser do servidor usa a API pública para aridade e coordenadas finitas; FR-003 `ValidacaoComandos::validar` antes de `agendar_comando`, com visão local de `seq`/`request_id` para replay no lote e zero mutação do `Mundo` na recusa; FR-004 recusa determinística (exit 2, stdout vazia, stderr `linha N: <CodigoRecusa>`) sem tocar `ultima_recusa`; FR-005 formato `<tick>\t<sha256>`, hash inicial e fixture `cli.rs:19` compatíveis; FR-006 cinco testes novos (141 → 146), estilo Rust, fronteira e governança. SC-001..SC-006 seguem a mesma ordem.

## User Scenarios

### US1 — API pública do sim: única lista de ações e exigência de destino (P1)

O sim passa a expor `pub const ACOES_PERMITIDAS: &[&str]` (as mesmas 13 ações de hoje, na mesma ordem: `mover`, `sinalizar_sincronia`, `sinalizar_alavanca`, `coletar_materiais`, `construir_barreira`, `atacar_forte`, `construir_ponte`, `propaganda`, `propaganda_no`, `atacar_no`, `atacar_pnj`, `analise_materialista`, `escudo`) e `pub fn acao_exige_destino(acao: &str) -> bool`, documentados com `///` como referência estável para validação e consumidores headless. A função devolve `false` **somente** para `sinalizar_sincronia` e `sinalizar_alavanca` (2 ações) e `true` para as outras **11**. Nenhuma regra de `ValidacaoComandos::validar`, nenhum limiar de `nivel_minimo_para_acao`, nenhum `CodigoRecusa` e nenhum campo `ultima_recusa` é alterado: a mudança é exclusivamente de visibilidade e de helper derivado da lista canônica — nunca uma segunda tabela de ações.

**Why this priority**: sem uma referência pública, o servidor é obrigado a espelhar a whitelist e a regra de destino à mão — exatamente a divergência 6 versus 13 que motivou o ciclo. É a base das demais histórias.

**Independent Test**: inspecionar `crates/sim/src/lib.rs:187-230`; conferir `pub` em `ACOES_PERMITIDAS`, 13 entradas na ordem canônica, `///` em ambos os itens e `acao_exige_destino` devolvendo `false` apenas para as duas sinalizações; verificar que o corpo de `validar` e `ultima_recusa` não mudaram.

**Acceptance Scenarios**:
1. **Given** um consumidor externo (`crates/servidor`), **When** ele importa `insurgencia_sim::{acao_exige_destino, ACOES_PERMITIDAS}`, **Then** ambos compilam sem re-export, alias ou cópia local, e a lista tem exatamente 13 entradas na ordem do contrato.
2. **Given** as 13 ações canônicas, **When** `acao_exige_destino` é chamada para cada uma, **Then** 11 devolvem `true` e apenas `sinalizar_sincronia`/`sinalizar_alavanca` devolvem `false`, sem depender de ordem, caixa ou whitespace.
3. **Given** o estado autoritativo, **When** a nova API é usada, **Then** nenhuma regra de validação, nenhum código de recusa, nenhum nível e nenhuma escrita em `ultima_recusa` é modificado; o hash de `Mundo::new()` permanece idêntico.

### US2 — Parser do servidor deriva aridade e destino do Núcleo (P1)

`crates/servidor/src/main.rs` remove a whitelist local de 6 ações (`:16-24`) e o `matches!` hardcoded de `:99`, passando a usar `ACOES_PERMITIDAS.contains(&acao)` para o filtro de ação e `acao_exige_destino(acao)` para decidir entre aridade 4 (sem coordenadas) e 6 (com `x y`). As mensagens estáveis de erro do parser são preservadas: `linha N: ação desconhecida: <acao>`, `linha N: esperado 'peer_id acao seq request_id [x y]'`, `linha N: ação exige coordenadas x e y`, `linha N: ação não aceita coordenadas` e `linha N: {x|y} inválido` (somente valores finitos, via `is_finite`). A ordem de processamento, o parsing de `peer_id`/`seq`/`request_id`, o descarte de linhas em branco e a numeração 1-based das linhas permanecem os mesmos.

**Why this priority**: é a correção estrutural da divergência — a whitelist deixa de ser uma cópia defasada e passa a ser derivada da única lista canônica, cobrindo as 7 ações antes invisíveis no CLI.

**Independent Test**: inspecionar `analisar_linha` e `analisar_coordenada`; alimentar uma linha por cada uma das 13 ações, uma ação desconhecida (`voar`), uma ação com destino sem coordenadas, uma sem destino com coordenadas extras e coordenadas `NaN`/`inf`; conferir aridade, mensagens e nenhuma whitelist local remanescente.

**Acceptance Scenarios**:
1. **Given** as 13 ações canônicas, **When** cada linha `7 <acao> 1 1 [x y]` é analisada com a aridade correta, **Then** todas são aceitas pelo parser; ações com destino exigem 6 campos e `sinalizar_*` exige 4.
2. **Given** `7 sinalizar_sincronia 1 1 0.5 0.0`, **When** a linha é analisada, **Then** o erro é `linha N: ação não aceita coordenadas`, mantendo o teste `entrada_malformada_retorna_exit_2_sem_saida`.
3. **Given** `7 mover 1 1` ou `7 mover 1 1 NaN 0.0`, **When** a linha é analisada, **Then** o erro é `linha N: ação exige coordenadas x e y` ou `linha N: x inválido`, respectivamente; nenhum comando chega ao `Mundo`.

### US3 — Validar cada comando antes de agendar, sem mutar o estado (P1)

Em `executar`, depois de criar os jogadores e **antes** de cada `mundo.agendar_comando(...)`, o servidor chama `ValidacaoComandos::validar(&mundo, peer_id, &acao, seq, request_id, destino)`. Como todos os comandos do lote são validados antes do primeiro `avancar_tick`, o `seq`/`request_id` do `Mundo` ainda não avançou: o servidor mantém uma **visão local** (por `peer_id`) dos últimos `seq`/`request_id` aceitos, atualizando-a somente após um comando validado com sucesso, para que um segundo comando do mesmo par no mesmo lote com valores repetidos seja recusado como `SeqOuRequestReplay`. Nenhum comando recusado é agendado, nenhum `Mundo` é mutado em recusa — em particular `ultima_recusa` permanece `None`, porque esse campo entra em `hash_estado` (`sim:1927-1934`). As regras do validador, o gate de nível do núcleo em `fase_skills` e a ordem das fases do tick não são alterados.

**Why this priority**: converte o validador existente em consumidor executável sem mudar o Núcleo; impede que o CLI aceite intenções inválidas e que uma recusa contamine o hash por `ultima_recusa`.

**Independent Test**: inspecionar o loop de validação em `executar`; testar comando válido seguido de replay do mesmo `peer_id`, dois peers com `seq` independentes, ação de nível superior ao do jogador (`AcaoBloqueada`) e comando com destino inválido; conferir que a primeira linha de stdout continua igual a `Mundo::new().hash_estado()` quando nada é agendado.

**Acceptance Scenarios**:
1. **Given** `7 coletar_materiais 1 1 0 0` com jogador na origem, **When** o comando é validado, **Then** `Ok(())` e ele é agendado uma única vez, na ordem da linha.
2. **Given** duas linhas do mesmo peer com `seq = 1`, **When** a segunda é validada no mesmo lote, **Then** o resultado é `Err(CodigoRecusa::SeqOuRequestReplay)`, nada é agendado para ela e o processo termina com exit 2 sem stdout.
3. **Given** peers 7 e 8 com `seq = 1` cada, **When** ambos são validados, **Then** a visão local é por `peer_id` e nenhum é recusado por replay alheio.
4. **Given** qualquer recusa na validação, **When** o processo termina, **Then** `ultima_recusa` não foi escrita e nenhum campo do `Mundo` foi alterado — apenas a visão local do CLI é consultada.

### US4 — Recusa determinística na fronteira (P1)

Quando `ValidacaoComandos::validar` devolve `Err(codigo)`, o servidor escreve na **stderr** exatamente `linha N: <CodigoRecusa>` (o `Display` do `thiserror` de `CodigoRecusa`, p. ex. `linha 3: destino ausente ou fora do ponto`, `linha 2: seq ou request_id em replay`, `linha 4: movimento fora do limite`) e retorna `ExitCode::from(2)`. A **stdout permanece vazia**: nenhuma linha `<tick>\t<sha256>` é emitida quando qualquer comando do lote é recusado. Erros de argumentos e de parsing continuam com suas mensagens estáveis atuais e o mesmo exit 2, sem misturar com o formato de recusa de validação.

**Why this priority**: é o contrato observável pelo consumidor (usuário/automação do CLI) e pelo teste nomeado `comando_recusado_pela_validacao_retorna_exit_2_sem_stdout_e_com_codigo_na_stderr`.

**Independent Test**: executar o binário com uma linha válida seguida de `7 mover 1 1 3.5 -4`; conferir exit 2, stdout vazia e stderr começando com `linha 2: movimento fora do limite`; repetir para `coletar_materiais` em `(0.5, 0.5)` (`DestinoInvalido`) e para replay (`SeqOuRequestReplay`).

**Acceptance Scenarios**:
1. **Given** um lote com ao menos um comando recusado, **When** o processo termina, **Then** `status.code() == Some(2)`, `stdout.is_empty()` e a stderr contém `linha N: <CodigoRecusa>` com `N` igual ao número da linha 1-based da entrada.
2. **Given** a mesma entrada recusada em duas execuções, **When** as saídas são comparadas, **Then** stdout e stderr são byte a byte idênticas, sem relógio, RNG ou ordem de mapa não determinística.
3. **Given** erros de argumentos (`--ticks 0`) e de parsing (`voar`, aridade errada), **When** o processo termina, **Then** continuam com exit 2/stdout vazia e mensagem estável, sem prefixo de código de recusa inventado.

### US5 — Formato `<tick>\t<sha256>`, hash inicial e fixture compatíveis (P1)

O laço de emissão permanece `for tick in 0..ticks { writeln!(saida, "{tick}\t{}", mundo.hash_estado())?; mundo.avancar_tick(); }` — uma linha por tick, `<tick>` decimal começando em 0, TAB, 64 hex lowercase, hash **antes** de avançar o tick. A fixture `ENTRADA_VALIDA` em `crates/servidor/tests/cli.rs:19` é ajustada de `coletar_materiais 1 1 0.5 0.5` para `coletar_materiais 1 1 0 0`, pois o validador do Núcleo exige destino exatamente igual à posição do jogador e os jogadores são criados na origem; sem esse ajuste a entrada "válida" passaria a ser recusada com `DestinoInvalido`. Os testes de parser que usam `mover (3.5,-4)` em `servidor/src/main.rs` **não são tocados**: eles exercem somente `analisar_linha`, e a validação é uma etapa posterior. Nenhum teste fixa um hash de entrada não vazia como constante; apenas `primeira_linha_com_stdin_vazia_e_o_estado_inicial` ancora o hash inicial ao de `Mundo::new()`, e ele é preservado.

**Why this priority**: garante que a nova validação não mude o protocolo de saída nem quebre a suíte existente por incompatibilidade de fixture — risco identificado explicitamente na auditoria [EV].

**Independent Test**: comparar a primeira linha de stdout com stdin vazia contra `Mundo::new().hash_estado()`; executar duas vezes com a fixture ajustada e comparar stdout; inspecionar que o trecho de `writeln!`/`avancar_tick` e os testes de parser com `(3.5, -4)` estão intactos.

**Acceptance Scenarios**:
1. **Given** stdin vazia e `--ticks 4`, **When** o binário roda, **Then** 4 linhas `<tick>\t<sha256>` são emitidas e a primeira é exatamente `0\t{hash de Mundo::new()}`.
2. **Given** a fixture ajustada para `coletar_materiais 1 1 0 0`, **When** os testes de formato/determinismo rodam, **Then** todos terminam com exit 0, stderr vazia e hashes idênticos entre execuções.
3. **Given** qualquer entrada válida, **When** a saída é inspecionada, **Then** há exatamente um TAB por linha, o tick é monotônico a partir de 0 e o hash tem 64 caracteres hexadecimais; nenhum campo novo, JSON ou prefixo é acrescentado.

### US6 — Testes, estilo e governança (P2)

Cinco testes novos e nomeados entram em `crates/servidor/tests/cli.rs`: `todas_as_treze_acoes_do_sim_sao_aceitas_pela_whitelist_do_cli`, `acoes_com_destino_exigem_coordenadas_finitas_e_rejeitam_coordenadas_extras`, `comando_recusado_pela_validacao_retorna_exit_2_sem_stdout_e_com_codigo_na_stderr`, `comandos_validados_preservam_formato_tick_hash_e_determinismo` e `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial`. Os **141 testes existentes** são preservados sem afrouxamento, resultando em **146**. O código mantém `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, ausência de `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100` e `///` em itens públicos. R1 é `go`; a constituição v1.2.2 coloca `tasks` antes de `analyze` (READ-ONLY), `converge` como append-only e R4 (validate) antes de R7 (release `1.0.0-local`); **R6 é N/A** — se invocada, PARE e escale ao SWE, sem issue simulada, sem `gh` improvisado e sem alegar execução. Promoção `dev→main` e deploy são SWE-exclusivos.

**Why this priority**: preserva a cadeia de rastreabilidade do hub e mantém a frente como correção determinística, sem dívida de automação, stack, protocolo ou CI.

**Independent Test**: inspecionar os artefatos, a ordem das portarias, os nomes dos cinco testes e o diff de código; conferir 141 → 146, ausência de `unsafe`/`unwrap`/`expect`/`#[allow]` e nenhuma mudança em contratos/protocolo/Cargo/CI/rede.

**Acceptance Scenarios**:
1. **Given** os artefatos, **When** a governança é revisada, **Then** R1 `go`, R6 N/A (BF-007), `tasks` antes de `analyze`, `converge` append-only e R4 antes de R7 `1.0.0-local` aparecem explicitamente, com scorecard **1280/3125**.
2. **Given** a suíte, **When** inspecionada, **Then** há exatamente cinco testes novos com os nomes canônicos e os 141 existentes permanecem intactos; nenhum depende de relógio externo, RNG, I/O de arquivo, rede ou janela gráfica.
3. **Given** o código, **When** o estilo é conferido, **Then** não há `unsafe`, `unwrap`/`.expect()`/`#[allow]` nem linha acima de 100 colunas; execução, gates e capturas ficam **[NV]** até o orquestrador.

## Edge Cases

- Ação fora das 13 canônicas: o parser a recusa com `linha N: ação desconhecida: <acao>` antes da validação; o guard `AcaoDesconhecida` de `ValidacaoComandos::validar` permanece no sim como única fonte de verdade, ainda que inalcançável pelo CLI.
- Ações com destino (11): ausência de `x y` → `linha N: ação exige coordenadas x e y`; ações sem destino (`sinalizar_*`) com 6 campos → `linha N: ação não aceita coordenadas`. Nenhuma ação muda de aridade por acidente de `matches!` hardcoded.
- Coordenadas `NaN`, `+inf`, `-inf` ou texto não numérico: `analisar_coordenada` recusa com `linha N: {x|y} inválido`; somente `is_finite()` é aceito, preservando o teste `entrada_malformada_retorna_exit_2_sem_saida`.
- Replay no lote: `seq`/`request_id` do `Mundo` só avançam em `fase_skills`, depois do `avancar_tick`; por isso a visão local do CLI por `peer_id` é necessária. Sem ela, duas linhas do mesmo peer com `seq = 1` passariam juntas. Peers diferentes têm contadores independentes.
- Recusa nunca muta o estado: `ultima_recusa` entra em `hash_estado` (`sim:1927-1934`); chamar `validar` em um comando inválido **não** escreve em `Mundo`. Se o gate de nível de `fase_skills` recusar um comando agendado diretamente, isso é comportamento preexistente do núcleo e permanece fora desta fronteira.
- Fixture `cli.rs:19`: `coletar_materiais 1 1 0.5 0.5` é inválida para o Núcleo (destino diferente da posição do jogador na origem) e deve virar `0 0`; sem a troca, os testes de formato/determinismo falhariam com `DestinoInvalido`. Os testes de parser com `mover (3.5,-4)` em `main.rs:224-226` permanecem — `(3.5, -4)` excede `LIMITE_MOVIMENTO_M = 1.0`, mas `analisar_linha` não valida distância.
- `mover` em distância maior que 1 m vindo da stdin: aceito pelo parser (aridade/coordenadas finitas) e recusado pelo validador com `CodigoRecusa::MovimentoForaDoLimite` → `linha N: movimento fora do limite`; o teste de parser com `(3.5, -4)` continua verde porque não passa pela validação.
- Combinações de recusa do Núcleo: `AcaoBloqueada` (nível Q1/W2/E3/R6), `AlvoInvalido`, `FolegoInsuficiente`, `AnaliseEmRecarga`, `EscudoEmRecarga`, `DestinoInvalido`, `SaldoInsuficiente` — todas são propagadas como `linha N: <CodigoRecusa>` sem interpretação ou reescrita pelo servidor; a ordem das guardas é a do sim.
- Ordem de validação dentro do lote: a validação ocorre na ordem das linhas; a primeira recusa encerra o processo, portanto não há emissão parcial de stdout. Linhas em branco continuam sendo ignoradas e não consomem numeração de comando, mas contam para o `N` das mensagens.
- Stdin vazia: nenhum comando, nenhuma recusa, `Mundo::new()` intacto; a primeira linha de stdout é o hash inicial — equator de `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial`.
- Argumentos: só `--ticks N` com `N > 0`; forma inválida mantém exit 2/stdout vazia e mensagem estável, sem conflito com o formato `linha N: <CodigoRecusa>`.
- Formato de saída: `<tick>\t<sha256>` antes de `avancar_tick`, um TAB, 64 hex lowercase; nenhum campo de `ultima_recusa`, nenhum JSON, nenhum prefixo e nenhuma linha extra são emitidos.
- Fronteira: nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md` ou `.specify/memory/STATE.md`; nenhum binário cliente é executado em janela normal e nenhuma captura visual é fabricada.
- R6 invocada por engano: PARE, escale ao SWE, sem issue simulada e sem `gh` improvisado; promoção `dev→main` e deploy não são executados neste ciclo.

## Requirements

### Functional Requirements

- **FR-001 (API pública do sim)**: em `crates/sim/src/lib.rs`, `ACOES_PERMITIDAS` passa a ser `pub const ACOES_PERMITIDAS: &[&str]` com as mesmas 13 ações canônicas na ordem atual (`sim:190-204`) e `///` documentando a referência estável para validação/consumidores headless. É acrescentado `#[must_use] pub fn acao_exige_destino(acao: &str) -> bool`, também com `///`, que devolve `false` apenas para `sinalizar_sincronia` e `sinalizar_alavanca` e `true` para as outras 11 ações. Nenhuma regra de `ValidacaoComandos::validar`, nenhum `CodigoRecusa`, nenhum limiar de `nivel_minimo_para_acao`, nenhum campo de estado e nenhuma escrita em `ultima_recusa` é alterada; nenhuma segunda lista de ações é introduzida em outro crate.
- **FR-002 (parser derivado)**: `crates/servidor/src/main.rs` remove a whitelist local de 6 ações (`:16-24`) e o `matches!(acao, "mover" | "coletar_materiais" | "construir_barreira")` de `:99`, usando `ACOES_PERMITIDAS.contains(&acao)` e `acao_exige_destino(acao)` da API pública. A aridade é 4 para as 2 ações sem destino e 6 para as 11 com destino; `analisar_coordenada` continua exigindo `f64` finito; mensagens de erro, numeração 1-based, descarte de linhas em branco e ordem de leitura da stdin permanecem idênticos. Os testes de parser que usam `mover (3.5,-4)` não são modificados.
- **FR-003 (validação antes de agendar)**: em `executar`, cada comando é submetido a `ValidacaoComandos::validar(&mundo, peer_id, &acao, seq, request_id, destino)` depois de registrados os peers e antes de `mundo.agendar_comando(Comando { ... })`. O servidor mantém visão local, por `peer_id`, do último `seq`/`request_id` aceitos no lote e a atualiza somente em sucesso, de modo que replay intra-lote seja recusado antes do agendamento. Comando recusado não é agendado e não muta o `Mundo`: `ultima_recusa` permanece `None`, os campos dos jogadores e a fila de pendentes não mudam. A ordem das guardas e as regras do validador são as do sim, sem reimplementação no CLI.
- **FR-004 (recusa determinística)**: qualquer `Err(CodigoRecusa)` gera stderr `linha N: <CodigoRecusa>` usando o `Display` do erro tipado do sim, `ExitCode::from(2)` e stdout vazia; a primeira recusa interrompe o processamento, sem saída parcial. Argumentos e erros de parsing conservam exit 2/stdout vazia e suas mensagens estáveis atuais. Duas execuções com a mesma entrada recusada produzem stderr e stdout byte a byte idênticas, sem relógio, RNG, I/O de arquivo ou rede.
- **FR-005 (formato, hash e fixture)**: o laço de saída continua `writeln!(saida, "{tick}\t{}", mundo.hash_estado())` seguido de `mundo.avancar_tick()`, imprimindo uma linha `<tick>\t<sha256>` por tick, com hash de 64 hex lowercase calculado **antes** do avanço. A fixture `ENTRADA_VALIDA` em `crates/servidor/tests/cli.rs:19` muda de `coletar_materiais 1 1 0.5 0.5` para `coletar_materiais 1 1 0 0`, preservando os demais peers/ações. A primeira linha com stdin vazia continua igual a `0\t{hash de Mundo::new()}`; nenhum hash de entrada não vazia é fixado como constante e nenhuma mudança de serialização, `ultima_recusa` ou formato é permitida.
- **FR-006 (testes, estilo, fronteira e governança)**: cinco testes nomeados em `crates/servidor/tests/cli.rs` — `todas_as_treze_acoes_do_sim_sao_aceitas_pela_whitelist_do_cli`, `acoes_com_destino_exigem_coordenadas_finitas_e_rejeitam_coordenadas_extras`, `comando_recusado_pela_validacao_retorna_exit_2_sem_stdout_e_com_codigo_na_stderr`, `comandos_validados_preservam_formato_tick_hash_e_determinismo` e `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial` — preservando os 141 existentes (**146** ao final). Código e testes seguem `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100` e `///` em itens públicos; nenhum teste usa relógio externo, RNG, arquivo temporário, rede ou janela. R1 `go`, R6 N/A/BF-007 (PARE se invocada, nunca simular issue, nunca `gh` improvisado), `tasks` antes de `analyze` (READ-ONLY), `converge` append-only, R4 antes de R7 `1.0.0-local`, promoção `dev→main` e deploy SWE-exclusivos; nenhum binário roda janela normal (cliente apenas `--minimizado`) e toda execução/gate/captura fica **[NV]** até o orquestrador. Subagentes não executam gates, build, testes, formatadores, commit, push ou PR.

### Key Entities

- **`ACOES_PERMITIDAS` (sim)**: lista canônica de 13 ações, agora pública; única fonte de whitelist para o CLI, sem espelhamento local.
- **`acao_exige_destino` (sim)**: helper público derivado da lista — `false` para `sinalizar_sincronia`/`sinalizar_alavanca`, `true` para as outras 11; define a aridade 4/6 do parser.
- **`ValidacaoComandos::validar`**: validador isolado já existente (`sim:2098`), agora chamado pelo binário; devolve o primeiro `CodigoRecusa` aplicável, sem efeito colateral no `Mundo`.
- **`CodigoRecusa`**: enum `thiserror` do sim; seu `Display` é o texto estável usado na stderr como `linha N: <CodigoRecusa>`.
- **Visão local de `seq`/`request_id`**: mapa mantido no CLI, por `peer_id`, para detectar replay dentro do lote antes do `avancar_tick`; não é campo do `Mundo` e não entra no hash.
- **`ultima_recusa`**: campo do `Jogador` que participa de `hash_estado`; permanece `None` nas recusas da fronteira, garantindo hash inicial e determinismo inalterados.
- **LinhaComando**: estrutura interna do parser (`peer_id`, `acao`, `seq`, `request_id`, `destino`) — permanece como está, sem campos novos.
- **Formato `<tick>\t<sha256>`**: protocolo de stdout do CLI, imutável, com hash emitido antes de `avancar_tick`.
- **Fixture `ENTRADA_VALIDA`**: entrada canônica dos testes de integração; compatível com o validador após a troca `0.5 0.5` → `0 0`.

## Success Criteria

- **SC-001 (whitelist única)**: `ACOES_PERMITIDAS` é pública no sim com 13 ações na ordem canônica e `acao_exige_destino` devolve `true` para 11 e `false` para 2; o servidor não contém whitelist local nem `matches!` hardcoded de destino, e as 13 ações são aceitas pelo parser. Teste `todas_as_treze_acoes_do_sim_sao_aceitas_pela_whitelist_do_cli`.
- **SC-002 (aridade e finitude)**: ações com destino exigem coordenadas finitas e recusam coordenadas extras nas ações sem destino; `NaN`/`inf` e aridade errada geram exit 2/stdout vazia com mensagem `linha N: ...`; testes de parser com `mover (3.5,-4)` permanecem intactos. Teste `acoes_com_destino_exigem_coordenadas_finitas_e_rejeitam_coordenadas_extras`.
- **SC-003 (recusa sem estado)**: comando recusado por `ValidacaoComandos::validar` (destino, replay intra-lote, nível, fôlego, saldo, recarga) termina com exit 2, stdout vazia e stderr `linha N: <CodigoRecusa>`, sem escrever `ultima_recusa` nem agendar; duas execuções idênticas produzem a mesma saída. Teste `comando_recusado_pela_validacao_retorna_exit_2_sem_stdout_e_com_codigo_na_stderr`.
- **SC-004 (formato e determinismo)**: comandos validados preservam exatamente uma linha `<tick>\t<sha256>` por tick, hash de 64 hex antes do avanço e stdout idêntica entre execuções; a fixture `cli.rs:19` usa `0 0` e todos os testes de formato/determinismo passam. Teste `comandos_validados_preservam_formato_tick_hash_e_determinismo`.
- **SC-005 (hash inicial)**: com stdin vazia, a primeira linha continua `0\t{hash de Mundo::new()}`, com 4 linhas para `--ticks 4` e nenhuma recusa; `ultima_recusa` não é escrita pela fronteira. Teste `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial`.
- **SC-006 (suíte, estilo e governança)**: `./verify.sh` espera **146 testes** (141 preservados + 5), com estilo Rust v1.2.2 respeitado, roteiro de 10 passos em `.specify/validations/servidor-validacao/`, R1 `go`, R6 N/A, `tasks` antes de `analyze`, R4 antes de R7 `1.0.0-local`, promoção/deploy SWE-exclusivos e nenhuma mudança em contrato/protocolo/Cargo/CI/stack/rede; execução e capturas **[NV]** até o orquestrador.

## Clarificações

- **A âncora é a auditoria [EV] de `2026-11-24T10:29Z` e as regras do `AGENTS.md` para o CLI headless**, não uma nova regra de jogo: o objetivo é paridade entre a whitelist do servidor e as 13 ações canônicas do sim, mais chamada efetiva do validador antes do agendamento. Nenhuma regra de validação, nenhum número de gameplay e nenhum `CodigoRecusa` é inventado.
- **`acao_exige_destino` é derivada da lista, não uma nova política**: as 2 exceções (`sinalizar_sincronia`, `sinalizar_alavanca`) são as únicas ações sem destino; as outras 11 exigem `x y`. A aridade do parser deixa de ser hardcoded e passa a seguir essa função.
- **Aridade e validação são etapas distintas**: `analisar_linha` só decide forma (4 ou 6 campos, números finitos, ação conhecida); distância, nível, fôlego, saldo, recarga e replay são decididos por `ValidacaoComandos::validar`. Por isso `mover (3.5,-4)` continua válido como teste de parser e é recusado se vier da stdin.
- **Visão local de monotonização é necessária por causa do tick**: `agendar_comando` apenas empilha; `seq`/`request_id` avançam em `fase_skills`, depois do `avancar_tick`. Como toda a validação do lote acontece antes do primeiro tick, o CLI mantém seu próprio último par aceito por `peer_id`, sem tocar no `Mundo`.
- **Recusa não é efeito de jogo**: `ValidacaoComandos::validar` não muta estado; em especial `ultima_recusa` (que entra no hash) permanece `None`. O gate de nível preexistente em `fase_skills` continua como está, sem reforço ou remoção.
- **O texto da recusa é o `Display` do erro tipado**: `linha N: <CodigoRecusa>` não é uma tabela nova de mensagens no servidor; alterações futuras de redação pertencem ao `thiserror` do sim. Erros de parsing/argumentos mantêm suas mensagens atuais e não são reclassificados como `CodigoRecusa`.
- **Risco de compatibilidade da fixture é explícito**: `cli.rs:19` usa `coletar_materiais 1 1 0.5 0.5`, inválido para o Núcleo, porque os jogadores começam em `(0, 0)` e o validador exige destino igual à posição. A troca para `0 0` é a única edição permitida em `ENTRADA_VALIDA`; nenhum teste existente é afrouxado e nenhum hash de entrada não vazia é recalculado como constante.
- **Nenhum protocolo novo**: a saída `<tick>\t<sha256>` e o hash inicial são imutáveis; nenhum campo de recusa, JSON, prefixo ou linha extra entra na stdout. `PROTOCOL_VERSION`, contratos, manifesto, Cargo, CI e rede ficam fora.
- **Fronteira de escrita**: este agente escreve somente os 4 artefatos SDD em `specs/056-servidor-validacao/`; assess/validate/release e código pertencem a etapas e agentes próprios.
- **R6 é N/A** por constituição v1.2.2 e BF-007: fora da sequência executável até `resolve-tasks.sh` e adaptador de issues existirem; se invocada, PARE e escale ao SWE, sem simular issue, sem `gh` improvisado e sem alegar execução. Promoção `dev→main` e deploy também são SWE-exclusivos.
- **R3 = 0** clarificações em aberto: as divergências conhecidas (whitelist, fixture, replay intra-lote) têm solução determinada pela auditoria [EV] e pelo contrato do ciclo; nenhuma bloqueia a especificação.
- Execução de gates/build/testes/formatadores e capturas não têm surface automatizável neste agente: ficam **[NV]** até o orquestrador, sem janela normal.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira em `specs/056-servidor-validacao/` no worktree `.specify/queue/worktrees/servidor-056/` (branch `feature/056-servidor-validacao`, base `0bc2c25`); agentes de implementação editam `crates/sim/src/lib.rs`, `crates/servidor/src/main.rs` e `crates/servidor/tests/cli.rs` na etapa 12; o orquestrador executa gates, build, testes, o roteiro e as verificações finais.
- A sim já tem `ACOES_PERMITIDAS` (13 ações), `validar` em `ValidacaoComandos`, `CodigoRecusa` com `Display` via `thiserror`, `Mundo::new()`, `adicionar_jogador`, `agendar_comando`, `hash_estado` incluindo `ultima_recusa`, `LIMITE_MOVIMENTO_M = 1.0` e mapa de níveis Q1/W2/E3/R6; a auditoria [EV] confirma que nenhum binário chama o validador hoje.
- O servidor já tem `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, parser 4/6 campos com `is_finite`, `LinhaComando`, emissão `<tick>\t<sha256>` antes de `avancar_tick`, `main()` com `ExitCode::from(2)` e 6 testes unitários em `main.rs` mais 5 de integração em `tests/cli.rs`.
- A suíte Rust existente tem **141 testes** ancorados em `0bc2c25`; os cinco novos são acrescentados sem modificar os existentes (exceto a fixture `cli.rs:19`), resultando em **146**.
- Ambiente headless: nenhuma janela gráfica, rede, credencial ou comando `gh` é usado na redação destes artefatos; execução, capturas e resultados de gates ficam **[NV]**.
- O scorecard **1280/3125** é a escolha do ciclo 056 em `.specify/memory/STATE.md` (2026-11-24T10:29Z); o fechamento de BF-011 é registrado no backlog do mesmo documento, sem reabrir frentes alheias.

## Out of scope

Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, GDD e documentos derivados; Lightyear, `async`, sockets, TCP/UDP, lobby, 5v5, segundo jogador humano real ou partida entre clientes; novas ações, novos códigos de recusa, novas regras de validação, novos níveis, mudança de `ultima_recusa` ou de serialização de hash; janela gráfica normal, cliente Bevy, HUD, teclas, capturas visuais fabricadas; implementação da R6, criação de issue, `gh` improvisado, promoção `dev→main`, deploy, build/gate/teste/formatador/commit/push/PR executados por este agente; reescrita de artefatos assess/validate/release ou de testes existentes além da fixture `cli.rs:19`.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: `servidor-validacao` **4×5×4×4×4 = 1280/3125**, contra HUD `hash_estado` **1125/3125** (BF-014), cliente `mover` **240/3125** (BF-012), contratos consumidos **100/3125** (BF-013) e rede **50/3125** (BF-001 — inelegível por autorização/ADR-004); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, escala 1–5, denominador 5⁵ = 3125. Registro em `.specify/memory/STATE.md` (2026-11-24T10:29Z). Nenhum manifesto é criado e nenhuma frente anterior é reaberta.
