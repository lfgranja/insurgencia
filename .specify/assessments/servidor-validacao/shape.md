# Shape — Validação do servidor headless (CLI autoritativo)

- **Slug**: `servidor-validacao`
- **Created**: 2026-11-24T10:29Z
- **Recommended option**: Opção A — whitelist derivada do Núcleo + validação na fronteira do CLI
- **Scorecard único do ciclo**: A **1280/3125** (4×5×4×4×4); B 1125/3125 (3×5×3×5×5); C 240/3125 (5×3×4×2×2); D 100/3125 (5×2×5×1×2); rede 50/3125 (5×2×5×1×1, inelegível por ADR-004 e fora da disputa). Mesmos valores de `intake.md`, `research.md` e `decision.md`, sem pontuação paralela.

## Options

### Option A — Whitelist derivada do Núcleo + validação na fronteira do CLI (recomendada)

- **Sketch**: o sim expõe `pub const ACOES_PERMITIDAS` (as mesmas 13 ações) e `pub fn acao_exige_destino` (`true` para 11; `false` só para `sinalizar_sincronia` e `sinalizar_alavanca`). O servidor apaga sua whitelist de 6 ações e sua tabela de 3 ações com destino, passa a derivar aridade e exigência de coordenadas finitas da API pública e chama `ValidacaoComandos::validar` em cada linha **antes** de `agendar_comando`. Um `seq`/`request_id` local cobre o replay dentro do lote; a validação não toca o `Mundo`. Recusa = `exit 2`, `stdout` vazia, `stderr linha N: <CodigoRecusa>`. O formato `<tick>\t<sha256>` e o hash inicial ficam intactos. A fixture `cli.rs:19` vai a `0 0`; os testes de parser com `mover (3.5,-4)` não são alterados.
- **Appetite**: `small` — duas API públicas no sim, remoção de duas tabelas no servidor, uma chamada de validação no laço de leitura, uma visão local de monotônicos, um ajuste de fixture e cinco testes; sem contrato, protocolo, `Cargo`, stack, rede, CI ou relógio novo.
- **Trade-offs**: ganha a paridade autoritativa entre CLI e Núcleo e uma recusa com o código que o Núcleo já define, sem criar regra. Impacto **4** (autoridade da fronteira observável por automação), confiança **5** (whitelist, validador, parser, formato e riscos lidos no checkout), fechamento **4** (usuário/automação do CLI e testes nomeados; sem consumidor de UI), esforço⁻¹ **4** (escopo real mas local, sem stack nova), risco⁻¹ **4** (risco concentrado em `ultima_recusa`/hash e no replay do lote, mitigado por leitura-only e pelos testes; fronteira de stack preservada). Custo aceito: comandos antes aceitos pelo parser passam a ser recusados pelo Núcleo com `CodigoRecusa` legítimo — é a correção pretendida, registrada como mudança observável na spec.
- **Rabbit holes**: alterar `validar`, `CodigoRecusa` ou `ultima_recusa` "para facilitar"; transformar `acao_exige_destino` em nova tabela de regras; validar mutando o `Mundo` para registrar a recusa; esquecer a visão local de `seq`/`request_id` e deixar replay do lote passar; reescrever os testes de parser com `mover (3.5,-4)`; trocar `<tick>\t<sha256>` por outro formato; tocar contratos, `PROTOCOL_VERSION`, `Cargo`, CI, rede ou async; adotar R6 ou promover `dev→main`.

### Option B — HUD republica `hash_estado` (BF-014)

- **Sketch**: fazer o cliente standalone chamar `hash_estado` e exibir o resumo na HUD, fechando o loop de paridade visual entre o sim e o jogador.
- **Appetite**: `small`/`medium` — handler de tecla/linha de HUD no cliente e um teste de renderização estrutural; sem mudança no sim.
- **Trade-offs**: impacto **3**, confiança **5**, fechamento **3**, esforço⁻¹ **5**, risco⁻¹ **5** → **1125/3125**. É a segunda melhor pontuação e tem risco menor que A, mas não resolve a divergência de aceitação na `stdin`: o jogador veria o hash sem que a fronteira CLI deixe de aceitar comandos inválidos. A captura visual real com janela minimizada também fica **[NV]**.
- **Rabbit holes**: prometer screenshot sem surface automatizável; duplicar formatação do hash no cliente; confundir paridade visual com autoridade de validação.

### Option C — Cliente agenda `mover` em vez de injetar posição (BF-012)

- **Sketch**: trocar a injeção direta de posição feita por `sincronizar_sim` no cliente pelo agendamento do comando `mover`, eliminando o unit mismatch entre Cliente e Servidor.
- **Appetite**: `medium` — mexe no caminho de sincronização do cliente, depende do limite de 1 m do Núcleo e precisa de coordenação com BF-011.
- **Trade-offs**: impacto **5**, confiança **3**, fechamento **4**, esforço⁻¹ **2**, risco⁻¹ **2** → **240/3125**. Resolve uma inconsistência maior de jogabilidade, mas hoje o CLI sequer valida `mover`; implementar a correção no cliente antes da paridade da fronteira inverteria a dependência e aumentaria o risco de divergência.
- **Rabbit holes**: reescrever `sincronizar_sim`, alterar limites de movimento, predição/reconciliação, redes e stack (fora do corte).

### Option D — `contratos` consumido por binários (BF-013)

- **Sketch**: ligar o módulo `contratos`, hoje desconectado, ao sim e ao servidor para compartilhar tipos e constantes.
- **Appetite**: `large` — toca fronteira de módulo, `PROTOCOL_VERSION` e possivelmente `Cargo`.
- **Trade-offs**: impacto **5**, confiança **2**, fechamento **5**, esforço⁻¹ **1**, risco⁻¹ **2** → **100/3125**. É infraestrutura de contrato, não correção de comportamento observável na `stdin`; o contrato da sessão veta mudanças de contrato/protocolo/Cargo neste ciclo.
- **Rabbit holes**: versionamento de protocolo, migração de tipos, quebra de compatibilidade com consumidores existentes.

### Fora da disputa — Rede autoritativa real (BF-001)

- **Sketch**: transporte, predição e reconciliação com peers remotos reais (ADR-004/Lightyear).
- **Trade-offs**: impacto **5**, confiança **2**, fechamento **5**, esforço⁻¹ **1**, risco⁻¹ **1** → **50/3125 — inelegível**. ADR-004 e a ausência de `lightyear` em `Cargo.toml`/`Cargo.lock` bloqueiam a troca até decisão da SE; é dívida de autorização (BF-001), não opção de shape deste ciclo.

## Recommendation

Escolher **Opção A**. É a única que satisfaz as metas de `define.md` com os dois consumidores nomeados e sem depender de autorização externa: o usuário/automação do CLI passa a ver a decisão do Núcleo e o código `CodigoRecusa`, e o validador deixa de ser órfão, sem mudar regra, hash ou formato. Impacto **4**, confiança **5** (código lido), fechamento **4** (CLI + testes), esforço⁻¹ **4** e risco⁻¹ **4** (sem contrato/protocolo/Cargo/CI/stack/rede). B (1125/3125) melhora a visibilidade do hash, mas não a autoridade da fronteira; C (240/3125) e D (100/3125) invertem a dependência ou exigem mudanças vetadas; a rede (50/3125) é inelegível por ADR-004. A recomendação é consistente com a decisão 80/20 das `2026-11-24T10:29Z` em `.specify/memory/STATE.md:520-536` e com a DoD ali registrada.

## Out of Scope (for the recommended option)

- Mudanças em `ValidacaoComandos::validar`, `CodigoRecusa`, níveis, limites, `ultima_recusa`, `hash_estado`, protocolo, contratos, `Cargo.toml`/`Cargo.lock`, CI, stack, rede, sockets, async, Lightyear, mapa ou cliente.
- Reescrita dos testes de parser com `mover (3.5,-4)`; promessa de saída diferente de `<tick>\t<sha256>` ou de hash inicial diferente do atual.
- `STATE.md`, `AGENTS.md`, GDD, specs, validações, releases, `.specify/queue/concurrency.jsonl` e qualquer código-fonte neste assessment; **R6** (N/A por constituição v1.2.2, BF-007), issues simuladas e `gh` improvisado; promoção `dev→main` e deploy (SWE-exclusivos).
- Gates, build, testes, formatadores, `./verify.sh`, sondas, CI, commit, push e PR neste assessment; todo resultado de verificação permanece **[NV]**.

## Assumptions to Validate

- `ACOES_PERMITIDAS` mantém as 13 entradas e a ordem de `sim:188-202` quando passa a `pub const`, e `acao_exige_destino` devolve `false` exatamente para `sinalizar_sincronia` e `sinalizar_alavanca` (11 `true`).
- `ValidacaoComandos::validar` permanece com a assinatura `(&Mundo, u64, &str, i64, i64, Option<(f64, f64)>)` de `sim:2087-2094` e continua imutável quanto ao mundo; a chamada na fronteira não introduz mutação de `ultima_recusa`.
- O `Mundo` não atualiza `seq`/`request_id` no agendamento (`sim:764-766`), apenas na execução do tick seguinte (`sim:809-820`); a visão local no servidor cobre somente o lote lido e não substitui a regra de `SeqOuRequestReplay`.
- `analisar_coordenada` (`servidor:66-75`) continua a rejeitar valores não finitos; a aridade 4/6 passa a ser decidida por `acao_exige_destino`, sem aceitar coordenadas extras nas 2 ações sem destino.
- A fixture `cli.rs:19` com `coletar_materiais 1 1 0 0` satisfaz a regra de `coletar_materiais` do Núcleo (destino igual ao ponto do jogador), e os testes de parser com `mover (3.5,-4)` continuam válidos como testes de **sintaxe**, não de validação.
- Os cinco testes nomeados cobrem whitelist, aridade/coordenadas, recusa exit/stdout/stderr, formato/determinismo e hash inicial, elevando 141 → **146** sem afrouxar os existentes; `./verify.sh`, CI `gates`+`rust`, roteiro 10/10, R4 e R7 permanecem **[NV]** até execução pelo orquestrador.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 (whitelist pública / `acao_exige_destino` / parser derivado / validação antes do agendamento com visão local / recusa e fixture / formato, testes, estilo e governança) e SC-001..SC-006. B, C e D são as alternativas de `research.md` e não recebem rastro de implementação neste ciclo; a rede (50/3125) é inelegível por ADR-004. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica em `specs/056-servidor-validacao/tasks.md`. R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**; nada executado, todo resultado de verificação **[NV]**.
