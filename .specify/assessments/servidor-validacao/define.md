# Define — Validação do servidor headless (CLI autoritativo)

- **Slug**: `servidor-validacao`
- **Created**: 2026-11-24T10:29Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (decisão 80/20 e DoD em `.specify/memory/STATE.md:493-536`; `crates/sim/src/lib.rs`; `crates/servidor/src/main.rs`; `crates/servidor/tests/cli.rs`; constituição v1.2.2)
- **Scorecard único do ciclo**: `servidor-validacao` **1280/3125** (4×5×4×4×4); HUD `hash_estado` 1125/3125 (3×5×3×5×5); cliente `mover` 240/3125 (5×3×4×2×2); contratos consumidos 100/3125 (5×2×5×1×2); rede 50/3125 (5×2×5×1×1, inelegível por ADR-004). Nenhuma pontuação paralela.

## Problem Statement

O binário `insurgencia-servidor` aceita e agenda comandos com base numa whitelist **própria** de 6 ações (`servidor/src/main.rs:16-24`) e numa tabela local de 3 ações que exigem `destino` (`:99`), enquanto o Núcleo determinístico define 13 ações canônicas numa constante **privada** (`sim/src/lib.rs:188-202`) e já possui `ValidacaoComandos::validar` (`:2087`), **não chamada por nenhum binário**. O resultado é um contrato de aceitação duplo e divergente: o CLI pode aceitar o que o Núcleo recusaria e recusar o que o Núcleo aceitaria, e a recusa não chega ao consumidor com o `CodigoRecusa` autoritativo. Qualquer correção ingênua que valide mutando o `Mundo` altera `ultima_recusa`, que entra no SHA-256 (`sim:1916-1947`), quebrando o hash inicial e o determinismo. O problema é de **autoridade e paridade da fronteira CLI**, não de nova regra, protocolo ou transporte.

## Affected Users & Stakeholders

- **Users**: usuário/automação do CLI headless — precisam que `stdin` com as 13 ações canônicas seja aceita e que uma recusa chegue como `exit 2`, `stdout` vazia e `stderr linha N: <CodigoRecusa>`, sem mudar o formato `<tick>\t<sha256>`; quem inspeciona a sim — precisa que a validação seja a mesma em ambos os lados, com whitelist única e sem mutação de `ultima_recusa`.
- **Stakeholders**: mantenedores do sim — precisam de `pub const ACOES_PERMITIDAS` e `pub fn acao_exige_destino` como API pública estável, sem tocar nas regras de `validar`; mantenedores do servidor — precisam remover a whitelist local e a tabela hardcoded de `destino`, mantendo visão local de `seq`/`request_id` para replay dentro do lote; autor do teste `cli.rs` — precisa da fixture `coletar_materiais 1 1 0 0` e dos testes de parser com `mover (3.5,-4)` preservados; SWE/orquestrador — precisa de um recorte verificável por `cargo test -p insurgencia-servidor --test cli`, sondas `printf ... | cargo run`, `./verify.sh` e CI `gates`+`rust`, sem promoção `dev→main` nem deploy.

## Goals

- Publicar no sim `pub const ACOES_PERMITIDAS: &[&str]` com as mesmas 13 entradas e a mesma ordem de `sim:188-202`, e acrescentar `pub fn acao_exige_destino(acao: &str) -> bool` — `true` para 11 ações, `false` **apenas** para `sinalizar_sincronia` e `sinalizar_alavanca`.
- **Não alterar** nenhuma regra de `ValidacaoComandos::validar`, nenhum `CodigoRecusa`, nenhum `nivel_minimo_para_acao` e nenhum campo `ultima_recusa` — a mudança é de visibilidade e de chamada, não de política.
- Remover a whitelist local de 6 ações do servidor; o parser passa a usar `acao_exige_destino` para decidir aridade (4 ou 6 campos) e a exigir coordenadas `f64` finitas exatamente para as 11 ações com destino, rejeitando coordenadas extras nas 2 sem destino.
- Validar cada linha com `ValidacaoComandos::validar` **antes** de `agendar_comando`, usando o `Mundo` como estado autoritativo e mantendo uma visão local de `seq`/`request_id` para detectar replay entre comandos do mesmo lote (o `Mundo` só atualiza esses monotônicos no tick seguinte).
- Recusa determinística: `exit 2`, `stdout` vazia, `stderr` `linha N: <CodigoRecusa>`; a validação não escreve no `Mundo`, de modo que o formato `<tick>\t<sha256>` e o hash inicial com `stdin` vazia permanecem idênticos aos atuais.
- Ajustar **apenas** a fixture `crates/servidor/tests/cli.rs:19` de `coletar_materiais 1 1 0.5 0.5` para `coletar_materiais 1 1 0 0`; não tocar testes de parser que usam `mover (3.5,-4)`.
- Cinco testes determinísticos nomeados — `todas_as_treze_acoes_do_sim_sao_aceitas_pela_whitelist_do_cli`, `acoes_com_destino_exigem_coordenadas_finitas_e_rejeitam_coordenadas_extras`, `comando_recusado_pela_validacao_retorna_exit_2_sem_stdout_e_com_codigo_na_stderr`, `comandos_validados_preservam_formato_tick_hash_e_determinismo`, `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial` — elevando 141 → **146** testes, sem afrouxar os existentes.
- Estilo Rust `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`, documentação `///` em itens públicos; rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012 e constituição v1.2.2: R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, **R6 N/A**.

## Non-Goals

- Alterar regras de validação, prioridade de recusas, `CodigoRecusa`, níveis, limites de movimento ou `ultima_recusa`; criar nova regra de destino fora de `acao_exige_destino`.
- Alterar o formato `<tick>\t<sha256>`, o hash inicial, o `ExitCode::from(2)`, o parser de argumentos `--ticks N` ou a emissão antes de `avancar_tick`.
- Reescrever testes de parser com `mover (3.5,-4)`; tocar em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, CI, stack, rede, sockets, async, Lightyear, mapa ou cliente.
- Mudanças em `STATE.md`, `AGENTS.md`, GDD, specs, validações, releases, `.specify/queue/concurrency.jsonl` ou qualquer código-fonte neste assessment; fronteira de escrita restrita aos 5 artefatos de assess.
- Executar gates, build, testes, formatadores, `./verify.sh`, sondas, commit, push ou PR; adotar R6, simular issue ou usar `gh` improvisado; promover `dev→main` ou disparar deploy (SWE-exclusivos).

## Success Metrics

- **SC-001** — As 13 ações de `ACOES_PERMITIDAS` são aceitas pela whitelist derivada do CLI, e nenhuma ação fora dela entra no parser sem recusa. Teste `todas_as_treze_acoes_do_sim_sao_aceitas_pela_whitelist_do_cli`. (baseline: whitelist local de 6 — **[NV]** até verificação)
- **SC-002** — As 11 ações com destino exigem coordenadas finitas e as 2 sem destino rejeitam coordenadas extras; `NaN`/`inf` não são aceitos. Teste `acoes_com_destino_exigem_coordenadas_finitas_e_rejeitam_coordenadas_extras`. (baseline: `exige_destino` hardcoded para 3 ações — **[NV]**)
- **SC-003** — Um comando recusado por `ValidacaoComandos::validar` termina com `exit 2`, `stdout` vazia e `stderr linha N: <CodigoRecusa>`, sem agendar e sem mutar o estado. Teste `comando_recusado_pela_validacao_retorna_exit_2_sem_stdout_e_com_codigo_na_stderr`. (baseline: nenhum binário chama o validador — **[NV]**)
- **SC-004** — Comandos válidos preservam o formato `<tick>\t<sha256>` tick a tick e duas execuções com a mesma entrada produzem saídas idênticas; a nova validação não introduz relógio, RNG ou ordem instável. Teste `comandos_validados_preservam_formato_tick_hash_e_determinismo`. (baseline: formato já estável — **[NV]**)
- **SC-005** — Com `stdin` vazia, a primeira linha continua sendo exatamente o hash do estado inicial do `Mundo`, inclusive com `ultima_recusa` None. Teste `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial`. (baseline: já observado no binário atual — **[NV]**)
- **SC-006** — Fronteira e governança: a fixture `cli.rs:19` passa a usar `0 0`, os testes de parser com `mover (3.5,-4)` permanecem, o total sobe para **146** testes e nada muda em contratos/protocolo/Cargo/CI/stack/rede; estilo Rust respeitado, R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, **R6 N/A**. (baseline: 141 testes e fixture inválida — **[NV]** até verificação)

## Cost of Inaction

O CLI headless continua com um contrato de aceitação paralelo e mais estreito que o do Núcleo: as 7 ações ausentes da whitelist local nunca chegam ao validador, e as 6 presentes são agendadas sem nunca serem validadas. Quem consome a `stdin` não consegue distinguir "ação desconhecida do CLI" de "`CodigoRecusa` do Núcleo", e um futuro validador chamado sem cuidado pode alterar `ultima_recusa` e corromper o hash. O custo é a permanência de BF-011 aberto enquanto as alternativas reais exigem autorização (rede, ADR-004), fechamento menor (HUD) ou esforço maior com risco de unit mismatch (cliente `mover`). Corrigir a fronteira agora é barato porque a regra já existe e só precisa ser exposta e chamada.

## Open Questions

- R3 = **0 clarificações**: nenhuma pergunta de produto bloqueia a especificação; o contrato resolve ações, exceções de `destino`, precedência da validação, recusa e formato de saída.
- **[NV]** Nomes definitivos de FR-001..FR-006 / SC-001..SC-006 / T001..T012 são os de `specs/056-servidor-validacao/spec.md` e `tasks.md` (em construção); o rastro abaixo registra o eco deste assessment.
- **[NV]** Renderização textual do `<CodigoRecusa>` na `stderr` (Display do `#[error]` vs. nome da variante) e as mensagens exatas do parser para aridade/coordenadas — a spec fixa; este assessment registra apenas o formato `linha N: <CodigoRecusa>`.
- **[NV]** A visão local de `seq`/`request_id` deve ser derivada somente do lote lido, sem persistência entre execuções e sem sobrescrever a autoridade do `Mundo`; confirmar na spec que a checagem de replay local não duplica nem contradiz `SeqOuRequestReplay`.
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates e roteiro não executados neste assessment.

## Rastreabilidade (escopo do problema)

- FR-001 → whitelist única derivada de `ACOES_PERMITIDAS` e exposição pública no sim; FR-002 → `acao_exige_destino` (11 com destino; `sinalizar_sincronia` e `sinalizar_alavanca` sem); FR-003 → parser do servidor sem whitelist local, aridade e coordenadas finitas derivadas; FR-004 → `ValidacaoComandos::validar` antes de `agendar_comando`, visão local de `seq`/`request_id`, sem mutar `ultima_recusa`; FR-005 → recusa `exit 2`/`stdout` vazia/`stderr linha N: <CodigoRecusa>` e fixture `0 0`; FR-006 → formato `<tick>\t<sha256>`, hash inicial, cinco testes, estilo Rust e governança.
- SC-001..SC-006 → Success Metrics acima (cada um com teste nomeado em SC-001..SC-005; SC-006 com fronteira, fixture e verificação); T001..T012 → entregáveis em `specs/056-servidor-validacao/tasks.md`, fora desta fronteira de escrita.
- R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, **R6 N/A** por constituição v1.2.2 (BF-007). Nada executado; todo resultado de verificação permanece **[NV]**.
