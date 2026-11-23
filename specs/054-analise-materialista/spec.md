# Feature Specification: Análise Materialista — marca determinística em PNJ

**Branch**: `054-habilidades-wer` (base `5d2919`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T165500Z-analise-materialista`, `insurgencia - análise materialista`.

**Âncora**: GDD §10.2:554-559 — **E — Análise Materialista** do Intelectual: nível **3**, **20 Fôlego**, execução **0,3 s**, **12 m** de alcance (alvo único), **recarga 15 s**, **−25% de resistência a dano por 5 s** e **marcas de dois Intelectuais no mesmo alvo não se acumulam** (§10.2:559). A decisão 80/20 do ciclo 054, registrada em `.specify/memory/STATE.md` (2026-11-23T16:55Z), escolheu **Análise Materialista 4×2×2×3 = 48/625** sobre **demais W/E/R 4×2×1×3 = 24/625** e **rede 5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE), critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. O corte determinístico implementa **apenas a marca em `Pnj` ativo** no estado autoritativo, reaproveitando `atacar_pnj`, `DANO_ATAQUE`, `BTreeMap`, `ValidacaoComandos` e o relógio por `tick`; a execução de 0,3 s, a visibilidade de estruturas por 10 s, jogadores, estruturas, o bônus da Sabotagem Estrutural, canalização, magia, RNG, mapa, protocolo, Cargo, stack, rede, Lightyear, sockets, Cargo e as habilidades W/R ficam **fora de escopo**. **Todos os números do corte são `[HIPÓTESE]`/protótipo** (convenção de rotulação de GDD §8.1:351 e §10.1), nunca cânone: o GDD é lido, nunca reescrito.

## User Scenarios

### US1 — Estado autoritativo da marca e do custo (P1)

O jogador passa a observar a marca no estado determinístico: `Pnj.marca_ticks: i64` (`0` = sem marca) e `Jogador.analise_recarga_ticks: i64` (`0` = sem recarga) entram em `crates/sim/src/lib.rs` com `///` em cada campo e são inicializados em zero por `adicionar_pnj`/`adicionar_jogador`, inclusive quando uma entrada existente é reinjetada. As constantes `[HIPÓTESE]` `CUSTO_ANALISE_MATERIALISTA = 20`, `ALCANCE_ANALISE_MATERIALISTA_M = 12.0`, `TICKS_ANALISE_MATERIALISTA = 150` (5 s a `PASSO_SEGUNDOS = 1/30`), `RECARGA_ANALISE_MATERIALISTA_TICKS = 450` (15 s) e `BONUS_DANO_ANALISE_PERCENT = 25` são `pub const` documentadas no sim. Nenhuma estrutura paralela, `HashMap`, uuid, RNG, relógio externo, crate, contrato ou protocolo novo: a marca vive no `Pnj` do `BTreeMap` e a recarga no `Jogador` do `BTreeMap`, preservando a ordem estável por `id`/`peer_id`.

**Why this priority**: sem os campos e as constantes, marca, bônus, recarga, HUD e hash não têm representação observável nem comparável em replays; é a base das demais histórias.

**Independent Test**: inspecionar `Pnj`, `Jogador`, `adicionar_pnj`, `adicionar_jogador` e as cinco constantes; conferir `0` nos campos novos após criação/reinjeção, rótulo `[HIPÓTESE]` em todas elas e ausência de `Instant`/`SystemTime`/aleatoriedade.

**Acceptance Scenarios**:
1. **Given** um `Mundo` recém-criado, **When** `adicionar_pnj(1, (0.0, 0.0))` e `adicionar_jogador(1)` são chamados, **Then** `marca_ticks == 0` e `analise_recarga_ticks == 0`, sem alterar os campos pré-existentes.
2. **Given** um PNJ marcado e um jogador em recarga, **When** `adicionar_pnj`/`adicionar_jogador` são chamados com o mesmo id/peer, **Then** ambos os campos voltam a zero, de forma determinística e sem pânico.
3. **Given** a spec, **When** os números são lidos, **Then** 3, 20, 12 m, 25%, 5 s/150 ticks e 15 s/450 ticks aparecem associados a GDD §10.2:554-559 e rotulados `[HIPÓTESE]`/derivação de protótipo, sem promoção a cânone.

### US2 — Ação `analise_materialista` no nível 3 (P1)

Uma ação nova entra no núcleo: `"analise_materialista"` em `ACOES_PERMITIDAS` e `nivel_minimo_para_acao` devolve **3** para ela, mantendo intactos o mapa Q1/W2/E3/R6 das ações existentes e o `_ => 1` residual. `ValidacaoComandos::validar` exige `destino` na posição de um PNJ **ativo**, distância euclidiana entre jogador e PNJ `<= ALCANCE_ANALISE_MATERIALISTA_M` (12 m), `folego >= CUSTO_ANALISE_MATERIALISTA` (20) e `analise_recarga_ticks == 0`. A precedência das guardas é preservada: `PeerNaoAutorizado` → `SeqOuRequestReplay` → `AcaoDesconhecida` → `AcaoBloqueada` (nível) → `DestinoInvalido` → `FolegoInsuficiente` → `AnaliseEmRecarga` (variante `[HIPÓTESE]`/protótipo acrescentada a `CodigoRecusa` apenas para tornar a recarga observável em `ultima_recusa`/`hash_estado`). Em `fase_skills`, o despacho usa cota de **uma análise por atacante por tick**, em `BTreeSet<u64>`, e `seq`/`request_id` só avançam se o comando for aceito. Não há habilidade W/E/R, custo de canalização, magia, RNG, efeito de visibilidade, sinal ou campo de protocolo novo.

**Why this priority**: é a interação jogável do ciclo — liga a tecla `E` ao estado determinístico sem criar habilidade, contrato ou stack.

**Independent Test**: inspecionar `ACOES_PERMITIDAS`, `nivel_minimo_para_acao`, `ValidacaoComandos::validar` e o despacho em `fase_skills`; validar nível 2 recusado, destino ausente/PNJ inativo/fora de 12 m, fôlego 19, recarga 450 e dois comandos do mesmo atacante no mesmo tick.

**Acceptance Scenarios**:
1. **Given** `nivel = 3`, PNJ ativo no `destino`, jogador a exatamente 12 m, fôlego 20 e recarga 0, **When** o comando é validado, **Then** é aceito; os limiares Q1/W2/E3/R6 das ações existentes permanecem inalterados.
2. **Given** `nivel = 2`, **When** `analise_materialista` é validada, **Then** o resultado é `Err(CodigoRecusa::AcaoBloqueada)` e nenhum estado muda — inclusive quando o destino é inválido.
3. **Given** destino ausente, PNJ inativo/inexistente no ponto ou distância 12,01 m, **When** o comando é validado, **Then** `Err(CodigoRecusa::DestinoInvalido)`; com destino válido e fôlego 19, `FolegoInsuficiente`; com fôlego suficiente e recarga > 0, `AnaliseEmRecarga`.
4. **Given** dois comandos `analise_materialista` do mesmo atacante no mesmo tick com alvos válidos, **When** `fase_skills` roda, **Then** no máximo um é aplicado (cota por atacante/tick), sem debitar 40 de fôlego nem marcar dois PNJs.

### US3 — Aplicação, não-acumulação e relógio por tick (P1)

`Mundo::aplicar_analise_materialista(&mut self, comando: &Comando, analises: &mut BTreeSet<u64>) -> bool` debita 20 de fôlego com `saturating_sub`, escreve `pnj.marca_ticks = TICKS_ANALISE_MATERIALISTA` (**atribuição, nunca soma**) e `jogador.analise_recarga_ticks = RECARGA_ANALISE_MATERIALISTA_TICKS`, e retorna `true` somente quando todos os efeitos foram aplicados. Uma reaplicação por outro atacante, com recarga própria zerada, **reinicia** a marca para 150 ticks — nunca acumula para 300. A cadência é inteiramente determinística: `avancar_tick` chama `avancar_ticks_analise()` logo após `fase_prepare()` e antes de `fase_skills()`, decrementando em um as marcas de todos os PNJs em ordem de `id` e as recargas de todos os jogadores em ordem de `peer_id`, saturando em zero. Assim, a marca aplicada no tick `T` vale 150 durante o tick `T` e zera no tick `T + 150`; a recarga iniada em `T` zera em `T + 450`. Não há decremento duplo no tick de aplicação, `Instant`, `sleep`, relógio externo, I/O ou contador em estrutura nova.

**Why this priority**: garante os critérios de aceitação de GDD §10.2:557 e 559 (5 s de efeito, sem acumulação) e mantém replays idênticos sem relógio de parede.

**Independent Test**: inspecionar `aplicar_analise_materialista`, `avancar_ticks_analise` e `avancar_tick`; percorrer os ticks `T`, `T+149`, `T+150`, `T+449` e `T+450`, conferindo marca 150→1→0, recarga 450→1→0 e o novo comando aceito em `T+450`.

**Acceptance Scenarios**:
1. **Given** fôlego 100 e recarga 0, **When** a análise é aceita no tick `T`, **Then** fôlego 80, `marca_ticks = 150`, `analise_recarga_ticks = 450`, `seq`/`request_id` atualizados e `ultima_recusa` inalterada.
2. **Given** um PNJ com `marca_ticks = 40` e outro atacante elegível, **When** a análise é reaplicada no mesmo PNJ, **Then** `marca_ticks = 150` (não 190), sem soma nem extensão adicional.
3. **Given** um comando recusado por fôlego, recarga ou cota, **When** o tick avança, **Then** nenhum campo muda, `seq`/`request_id` não avançam e a cota do atacante não é consumida por essa recusa.
4. **Given** os ticks `T+150` e `T+450`, **When** o relógio por tick é avançado, **Then** marca e recarga chegam a zero nesses ticks exatos, respectivamente, e nenhum valor negativo aparece.

### US4 — Bônus de dano no PNJ marcado (P1)

`Mundo::aplicar_dano_a_pnj` passa a aplicar `+25%` quando `marca_ticks > 0`, com aritmética inteira: `dano + dano * BONUS_DANO_ANALISE_PERCENT / 100`. Com `DANO_ATAQUE = 10`, o dano efetivo é **12** (PV 24→12); sem marca, permanece 10 (24→14). O cálculo ocorre uma vez por chamada aceita, antes de `saturating_sub(...).max(0)`, preservando piso zero, a transição `ativo = pv > 0` e a manutenção da entrada no `BTreeMap`. Marca expirada (`marca_ticks == 0`) não dá bônus, e o dano base de jogadores, Súditos, barricadas, pontes e Nós permanece intacto.

**Why this priority**: é o efeito observável da habilidade no combate PVE e o critério de aceitação de GDD §10.2:557 traduzido para o passo fixo de 30 Hz.

**Independent Test**: inspecionar `aplicar_dano_a_pnj` e aplicar dano 10 a um PNJ sem marca (24→14) e a um marcado (24→12), além de dano letal e dano a PNJ já inativo, comparando PV, `ativo`, retorno e hash.

**Acceptance Scenarios**:
1. **Given** um PNJ com 24 PV e `marca_ticks > 0`, **When** o jogador aplica dano 10, **Then** os PV vão a 12 e o retorno é `false`.
2. **Given** um PNJ sem marca e 24 PV, **When** o jogador aplica dano 10, **Then** os PV vão a 14 — o comportamento da spec 053 permanece inalterado.
3. **Given** um PNJ marcado com 8 PV, **When** o jogador aplica dano 10 (efetivo 12), **Then** `pv = 0`, `ativo = false`, a entrada permanece no mapa e o retorno é `true`, sem valor negativo.
4. **Given** `marca_ticks == 0`, `dano <= 0` ou `id` inexistente, **When** `aplicar_dano_a_pnj` é chamado, **Then** não há bônus nem outra mudança de estado, sem pânico.

### US5 — Cliente `E`, HUD `Análise: N`, hash e fronteira headless (P1)

`texto_hud` em `crates/cliente/src/main.rs` acrescenta `Análise: N`, com `N = simulacao.0.pnjs.values().filter(|p| p.ativo && p.marca_ticks > 0).count()` lido do espelho autoritativo, preservando **todas** as linhas e teclas existentes (`WASD`/setas, `B`/`P`/`C`/`G`/`R`/`F`/`Q`/`T`). Um handler com `KeyCode::KeyE` e `just_pressed` agenda `analise_materialista` no PNJ ativo mais próximo cuja distância ao jogador seja `<= ALCANCE_ANALISE_MATERIALISTA_M` (12 m), com desempate por menor `id` (padrão análogo a `posicao_pnj_mais_proximo`); sem alvo dentro de 12 m, nenhum comando é agendado e `seq`/`request_id` locais não avançam. `E` é livre no cliente (`W` continua sendo movimento) e nenhuma tecla W/R é acrescentada. Em `hash_estado`, `serializar_estruturas` passa a incluir `marca_ticks` no segmento de PNJs (ordem de `id`) e `serializar_jogadores` inclui `analise_recarga_ticks` (ordem de `peer_id`), mantendo todos os segmentos atuais (tick, epoch, match_id, jogadores, estruturas, Nós, resistência, resultado, `ultima_recusa`). A execução é headless: nenhuma janela normal, cliente somente com `--minimizado`.

**Why this priority**: sem o HUD o jogador não vê a marca; sem a tecla a ação não é jogável; sem o hash as marcas não são comparáveis em replays; sem a fronteira explícita, a frente viraria porta de entrada para protocolo e stack novos.

**Independent Test**: inspecionar `texto_hud`, o handler de `KeyE`, o helper de seleção de alvo, `serializar_estruturas`, `serializar_jogadores` e `hash_estado`; comparar hashes antes/depois de marcar, reaplicar e expirar, e conferir paths alterados.

**Acceptance Scenarios**:
1. **Given** quatro PNJs ativos, dois marcados e um inativo marcado, **When** `texto_hud` é montado, **Then** aparece `Análise: 2` ao lado de todas as linhas existentes, inclusive `PNJs: 4`, e a dica de teclas existente é acrescida de `E para analisar materialmente`.
2. **Given** um PNJ a 12 m e outro a 12,01 m, **When** `E` é pressionada, **Then** o comando é agendado somente para o PNJ a 12 m; sem PNJ ativo em alcance, nada é agendado.
3. **Given** dois replays com o mesmo input, **When** `hash_estado` é comparado, **Then** as strings são idênticas; mudar apenas `marca_ticks` ou `analise_recarga_ticks` altera o hash de forma previsível e nenhum segmento existente desaparece.
4. **Given** o estado final do ciclo, **When** os paths alterados são listados, **Then** aparecem somente os artefatos SDD e os paths de código atribuídos (sim, testes da sim e `crates/cliente/src/main.rs`), sem contratos, protocolo, Cargo, CI ou rede; nenhuma janela normal é aberta e o cliente roda apenas com `--minimizado`.

### US6 — Testes, governança e estilo (P2)

R1 é `go`; a constituição v1.2.2 coloca `tasks` antes de `analyze` (READ-ONLY), `converge` como append-only e R4 (validate) antes de R7 (release `1.0.0-local`); **R6 é N/A** — se invocada, PARE e escale ao SWE, sem issue simulada, sem `gh` improvisado e sem alegar execução. Cinco testes novos entram em `crates/sim/tests/simulacao.rs` (marca e bônus 10→12, não-acumulação, recarga 450, alcance 12 m, hash), preservando a suíte existente de **131 testes** (87 deles nesse arquivo). O código segue `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100` e `///` em itens públicos (o atributo de lint `#[expect(clippy::..., reason = "...")]`, já usado no cliente, não é chamada de método e permanece permitido). Nenhum número do protótipo é promovido a cânone. Execução de gates, build, testes, formatadores, commits, push e PR **[NV]** para este agente, assim como capturas visuais de HUD neste alvo headless.

**Why this priority**: preserva a cadeia de rastreabilidade do hub e impede que a frente vire dívida de automação, mudança de stack ou regra canônica inventada.

**Independent Test**: inspecionar os artefatos, a ordem das portarias, os nomes dos cinco testes e o diff de código, conferindo rótulos `[HIPÓTESE]`, R6 N/A e os atributos de lint.

**Acceptance Scenarios**:
1. **Given** os artefatos, **When** a governança é revisada, **Then** R1 `go`, R6 N/A, `tasks` antes de `analyze`, `converge` append-only e R4 antes de R7 `1.0.0-local` aparecem explicitamente, com scorecard **48/625**.
2. **Given** os testes, **When** a suíte é inspecionada, **Then** há exatamente cinco testes novos nomeados e os 131 existentes permanecem sem afrouxamento; nenhum deles depende de relógio externo, RNG, I/O ou janela gráfica.
3. **Given** o código, **When** o estilo é conferido, **Then** não há `unsafe`, `unwrap`/`.expect()`/`#[allow]` nem linha acima de 100 colunas; execução e capturas continuam **[NV]** até o orquestrador.

## Edge Cases

- `analise_materialista` com `destino = None`, em PNJ inativo/inexistente ou a mais de 12 m: `DestinoInvalido`; nenhum fôlego é debitado, nenhuma marca escrita, `seq`/`request_id` não avançam e a cota não é consumida.
- Jogador de nível 2 ou inferior: `AcaoBloqueada` precede todas as recusas específicas, inclusive destino, fôlego e recarga; nível 3 é o mínimo e nenhum limiar Q1/W2/E3/R6 existente é alterado.
- Fôlego 19: `FolegoInsuficiente`, sem débito parcial. Recarga `> 0`: `AnaliseEmRecarga`; a recarga só zera no tick `T+450`, nunca antes. Comandos recusados não atualizam `ultima_recusa` no despacho — apenas `ValidacaoComandos::validar` devolve o código, e o gate de nível do núcleo continua atualizando `ultima_recusa` como hoje.
- Dois comandos válidos do mesmo atacante no mesmo tick: cota de um por atacante/tick; o segundo não debita fôlego, não marca outro PNJ e não avança monotônicos. A cota só é consumida por comando efetivamente aceito.
- Reaplicação no mesmo PNJ por atacante diferente: `marca_ticks = 150` (atribuição), nunca `+=`; o efeito não se acumula nem se estende. A recarga é por jogador, de modo que cada Intelectual precisa da sua própria recarga zerada.
- Decremento no tick de aplicação: `avancar_ticks_analise()` roda **antes** de `fase_skills()`, então a marca 150 e a recarga 450 escritas no tick `T` só começam a decrescer em `T+1`, evitando decremento duplo no tick de aplicação.
- Ordem do tick: `fase_prepare()` → `avancar_ticks_analise()` → `fase_skills()` (dano do jogador com bônus) → `fase_defesa()` → `fase_ataque_pnjs()` → demais fases existentes. A marca expira no início do tick `T+150`, antes de qualquer dano nesse tick.
- PNJ morto com marca ativa: o dano subsequente satura em zero e a entrada permanece no mapa; `marca_ticks` continua a decrescer até zero e aparece no hash, mas nenhum comando novo pode marcar um PNJ inativo. Nenhum valor negativo de PV, marca ou recarga é produzido.
- Dano letal com marca: `10 + 10*25/100 = 12` aplicado a 8 PV resulta em `pv = 0`, `ativo = false` e retorno `true`; o piso zero da spec 053 é preservado e nenhum dano fracionário é introduzido.
- Alcance exatamente 12 m: aceito (comparação `<=` com `hypot`, sem tolerância de arredondamento); 12,01 m recusado. A distância é medida entre a posição do jogador e a posição do PNJ, não entre o jogador e sua própria posição.
- Seleção do alvo no cliente: nenhum PNJ ativo dentro de 12 m → nenhum comando agendado e nenhum incremento de `seq`/`request_id` local; desempate por menor `id` quando dois PNJs estão na mesma distância.
- Teclas: `E` é a única acrescentada; `W`/setas continuam movendo, e `T` continua atacando. Nenhuma tecla W/R é introduzida e nenhuma habilidade W/R é implementada.
- HUD sem surface automatizável: o alvo é headless, cliente somente com `--minimizado`; a checagem automatizável é inspeção de `texto_hud` e comparação de hash. Captura visual real da linha `Análise:` fica **[NV]** até verificação humana.
- R6 invocada por engano: PARE, escale ao SWE, sem issue simulada e sem `gh` improvisado; dívida de automação registrada, não executada.

## Requirements

### Functional Requirements

- **FR-001 (estado e constantes rotuladas)**: em `crates/sim/src/lib.rs`, `pub struct Pnj` ganha `pub marca_ticks: i64` (`/// Ticks restantes de marca; zero significa sem marca`) e `pub struct Jogador` ganha `pub analise_recarga_ticks: i64` (`/// Ticks restantes de recarga de analise materialista`), ambos `i64` para casar com o relógio de ticks, documentados, inicializados em zero em `adicionar_pnj`/`adicionar_jogador` e zerados quando a entrada é reinjetada. As constantes `pub const CUSTO_ANALISE_MATERIALISTA: i64 = 20`, `ALCANCE_ANALISE_MATERIALISTA_M: f64 = 12.0`, `TICKS_ANALISE_MATERIALISTA: i64 = 150`, `RECARGA_ANALISE_MATERIALISTA_TICKS: i64 = 450` e `BONUS_DANO_ANALISE_PERCENT: i64 = 25` são documentadas com `///` e rótulo `[HIPÓTESE]`/derivação de GDD §10.2:554-559 (5 s × 30 Hz = 150; 15 s × 30 Hz = 450), nunca cânone: sem estrutura paralela, `HashMap`, RNG, I/O, relógio externo, crate, contrato ou protocolo novo.
- **FR-002 (ação e validação)**: `ACOES_PERMITIDAS` inclui `"analise_materialista"` e `nivel_minimo_para_acao` devolve `3` para essa ação, preservando Q1/W2/E3/R6 e o `_ => 1` das demais. `ValidacaoComandos::validar` exige `destino` na posição de PNJ ativo (reutilizando `pnj_id_ativo_em_ponto`) com distância euclidiana `<= ALCANCE_ANALISE_MATERIALISTA_M` (12 m), `folego >= CUSTO_ANALISE_MATERIALISTA` (20) e `analise_recarga_ticks == 0`, na ordem `PeerNaoAutorizado` → `SeqOuRequestReplay` → `AcaoDesconhecida` → `AcaoBloqueada` → `DestinoInvalido` → `FolegoInsuficiente` → `AnaliseEmRecarga`. A variante `[HIPÓTESE]` `CodigoRecusa::AnaliseEmRecarga` é acrescentada apenas ao enum da sim, sem contrato, `PROTOCOL_VERSION` ou campo de rede. Em `fase_skills`, o despacho em `aplicar_analise_materialista` usa cota de um por atacante por tick (`BTreeSet<u64>`) e retorna `true` somente em comando aceito — condição para `seq`/`request_id` avançarem — sem habilidade, magia, canalização, efeito de visibilidade, sinal ou stack novos.
- **FR-003 (aplicação, não-acumulação e decaimento)**: um comando aceito debita 20 de fôlego com `saturating_sub`, atribui `marca_ticks = TICKS_ANALISE_MATERIALISTA` (nunca `+=`) ao PNJ ativo no `destino` e `analise_recarga_ticks = RECARGA_ANALISE_MATERIALISTA_TICKS` ao atacante, tornando a reaplicação por outro atacante um reinício para 150 ticks, nunca uma soma. `avancar_tick` chama `avancar_ticks_analise()` após `fase_prepare()` e antes de `fase_skills()`, decrementando em um as marcas em ordem de `id` e as recargas em ordem de `peer_id`, saturando em zero; a marca escrita no tick `T` some no tick `T+150` e a recarga no tick `T+450`. Nenhum `Instant`, `SystemTime`, `sleep`, I/O, RNG, decremento duplo ou contador em estrutura nova é usado.
- **FR-004 (bônus de dano)**: `Mundo::aplicar_dano_a_pnj` calcula, uma vez por chamada com `dano > 0` e `id` existente, `dano_efetivo = dano + dano * BONUS_DANO_ANALISE_PERCENT / 100` quando `marca_ticks > 0`, e `dano` caso contrário; aplica `pv = pv.saturating_sub(dano_efetivo).max(0)`, atualiza `ativo = pv > 0`, retorna `true` somente na transição para inativo e mantém a entrada no `BTreeMap`. Com dano base 10 o resultado é 12 (24→12); sem marca continua 14 (24→14). O dano base de jogadores, Súditos, barricadas, pontes e Nós e a fórmula `DANO_ATAQUE` permanecem intactos, sem `f64` de dano e sem dupla aplicação do bônus.
- **FR-005 (cliente, HUD, hash e fronteira)**: `texto_hud` acrescenta `Análise: N`, com `N` contendo somente PNJs `ativo && marca_ticks > 0` do espelho autoritativo, preservando todas as linhas e teclas existentes; um handler `KeyCode::KeyE` com `just_pressed` agenda `analise_materialista` no PNJ ativo mais próximo dentro de 12 m (desempate por menor `id`), sem comando quando não há alvo. `serializar_estruturas` inclui `marca_ticks` no segmento `pnjs` em ordem de `id` e `serializar_jogadores` inclui `analise_recarga_ticks` em ordem de `peer_id`, preservando todos os demais segmentos e a comparabilidade entre replays. Nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `concurrency.jsonl`, `STATE.md` ou GDD; nenhuma janela normal — cliente somente com `--minimizado`.
- **FR-006 (testes, governança e estilo)**: cinco testes novos e nomeados em `crates/sim/tests/simulacao.rs` cobrem marca e bônus 10→12 (`analise_materialista_marca_e_bonus_de_dano`), não-acumulação (`analise_materialista_nao_acumula_marca`), recarga 450 (`analise_materialista_recarga_450_ticks`), alcance 12 m (`analise_materialista_alcance_12_m`) e hash (`hash_estado_cobre_analise_materialista`), sem alterar os 131 testes existentes. `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100`, `///` em itens públicos (atributo de lint `#[expect(...)]` permitido); R1 `go`, R6 **N/A** por v1.2.2 (PARE se invocada, sem issue simulada e sem `gh` improvisado), `tasks` antes de `analyze` (READ-ONLY), `converge` append-only, R4 antes de R7 `1.0.0-local`; gates/build/testes/formatadores/commit/push/PR e capturas visuais **[NV]** para este agente.

### Key Entities

- **`Pnj.marca_ticks`**: contador `i64` no `BTreeMap` de PNJs; `0` = sem marca, `> 0` = bônus de 25% ativo, escrito por atribuição e decrementado uma vez por tick.
- **`Jogador.analise_recarga_ticks`**: contador `i64` por `peer_id`, `0` = pronta para usar; escrito como 450 no comando aceito e decrementado uma vez por tick.
- **Constantes `[HIPÓTESE]`**: `20` Fôlego, `12.0` m, `150` ticks (5 s), `450` ticks (15 s) e `25%`, derivadas de GDD §10.2:554-559 e de `PASSO_SEGUNDOS = 1/30`, nunca cânone.
- **`analise_materialista`**: ação de nível 3 no sim, com `destino` em PNJ ativo a no máximo 12 m, cota por atacante/tick, custo 20 e recarga por jogador; não é uma habilidade W/E/R nem um campo de protocolo.
- **`aplicar_analise_materialista` / `avancar_ticks_analise`**: ponto de aplicação atômica e ponto único de decaimento determinístico, ambos no sim e sem relógio externo.
- **`aplicar_dano_a_pnj`**: ponto de incidência do bônus inteiro (10→12), com piso zero e transição para inativo preservadas.
- **`serializar_estruturas` / `serializar_jogadores` / `hash_estado`**: serialização estável que passa a cobrir marca e recarga sem perder segmentos existentes.
- **`texto_hud` / handler de `KeyE`**: linha `Análise: N`, seleção do PNJ mais próximo dentro de 12 m e fronteira headless (`--minimizado`), com captura visual **[NV]**.

## Success Criteria

- **SC-001 (estado e rótulos)**: `Pnj.marca_ticks` e `Jogador.analise_recarga_ticks` existem, são documentados e começam em zero, inclusive após reinjeção; as cinco constantes têm os valores 20/12.0/150/450/25 especificados e todas aparecem rotuladas `[HIPÓTESE]`/derivação nos artefatos, sem promoção a cânone.
- **SC-002 (ação/validação)**: `analise_materialista` é aceita no nível 3 com PNJ ativo a no máximo 12 m, fôlego 20 e recarga 0; nível 2, destino ausente/sem PNJ ativo/fora de 12 m, fôlego 19 e recarga > 0 devolvem, respectivamente, `AcaoBloqueada`, `DestinoInvalido`, `FolegoInsuficiente` e `AnaliseEmRecarga`; no máximo uma análise por atacante/tick e monotônicos avançados só em comando aceito.
- **SC-003 (aplicação/decaimento)**: o comando aceito debita 20, escreve marca 150 e recarga 450; reaplicação reinicia para 150 em vez de somar; marcas e recargas decrescem uma vez por tick, em ordem de `id`/`peer_id`, zerando nos ticks `T+150`/`T+450`, sem `Instant`, RNG ou decremento duplo.
- **SC-004 (bônus)**: um PNJ marcado recebe 12 de dano do ataque base 10 (24→12); sem marca recebe 10 (24→14); dano letal com marca satura em zero, mantém a entrada e retorna `true`; nenhum outro dano básico ou número canônico muda.
- **SC-005 (cliente/hash/fronteira)**: `texto_hud` acrescenta `Análise: N` contando só PNJs marcados ativos e `KeyE` agenda a ação no alvo elegível mais próximo sem conflito com `W`/`T`, preservando todas as linhas/teclas; `hash_estado` cobre marca e recarga, é idêntico entre replays iguais e muda de forma previsível quando apenas um desses campos muda; contratos, protocolo, Cargo, manifesto, CI e redes permanecem intactos, com cliente apenas `--minimizado`.
- **SC-006 (governança/testes/estilo)**: R1 `go`, R6 N/A, `tasks` antes de `analyze` (READ-ONLY), `converge` append-only, R4 antes de R7 `1.0.0-local`; cinco testes novos nomeados cobrindo marca/10→12, não-acumulação, 450 ticks, 12 m e hash, com os 131 existentes preservados; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`; execução e capturas **[NV]** até o orquestrador.

## Clarificações

- **A regra lida é a de GDD §10.2:554-559**: E do Intelectual, nível 3, 20 Fôlego, 12 m, −25% por 5 s, não-acumulação. A execução de 0,3 s, a visibilidade de estruturas por 10 s, o bônus da Sabotagem Estrutural e a aplicação a jogadores/estruturas são **fora do corte**; nenhum número é promovido a cânone e o GDD não é reescrito.
- **150 e 450 ticks são derivações**, não números canônicos independentes: 5 s × 30 Hz (`PASSO_SEGUNDOS = 1/30`) e 15 s × 30 Hz. O bônus de 25% e o custo de 20 vêm da regra lida, e 12 m é o alcance do alvo único; todos são rotulados `[HIPÓTESE]`/protótipo.
- **A marca é um contador, não um carimbo de tempo**: `marca_ticks = 150` escrito por atribuição e decrementado por tick, sem `Instant`, `SystemTime`, `sleep` ou relógio externo. Reaplicação **reinicia** para 150 (critério §10.2:559), nunca acumula para 300.
- **O decaimento acontece antes das habilidades no tick**, em `fase_prepare` → `avancar_ticks_analise` → `fase_skills`, para que o tick de aplicação não sofra decremento duplo; a marca vale durante o tick `T` e expira no início de `T+150`.
- **A recarga é por jogador, a marca é por PNJ**: dois atacantes podem marcar o mesmo PNJ (sem acumulação), mas cada um só pode tornar a marcar após seus próprios 450 ticks. `AnaliseEmRecarga` é a recusa `[HIPÓTESE]` que torna isso observável em `ultima_recusa`/hash, sem contrato de rede.
- **Ação, não habilidade de slot**: `analise_materialista` vive em `ACOES_PERMITIDAS`, na validação e no despacho da sim; nenhuma habilidade W/E/R, custo extra, ícone, canalização, magia, efeito de visibilidade, mapa ou progressão de papel é acrescentada. `E` é apenas a tecla livre escolhida no cliente; `W` continua movimento.
- **Efeito restrito a PNJs ativos**: só PNJs ativos podem ser marcados; o bônus incide em `aplicar_dano_a_pnj` quando `marca_ticks > 0`, preservando o piso zero da spec 053. Nenhum jogador, Súdito, barricada, ponte ou Nó é marcado.
- **Sem mapa, visibilidade ou RNG**: a escolha do alvo no cliente é pelo PNJ ativo mais próximo dentro de 12 m (desempate por menor `id`); não há linha de visão, rota, coordenada canônica, aleatoriedade ou I/O.
- **R6 é N/A** por constituição v1.2.2: fora da sequência executável até `resolve-tasks.sh` e o adaptador de issues existirem; se invocada, PARE e escale ao SWE, sem simular issue, sem `gh` improvisado e sem alegar execução.
- **R3 = 0** clarificações em aberto: as lacunas (0,3 s, visibilidade, alvos, posições) são simplificações explícitas de protótipo, não ambiguidades que bloqueiem a especificação.
- Execução de gates/build/testes/formatadores e capturas visuais não têm surface automatizável neste agente: ficam **[NV]** até o orquestrador; o cliente é executado somente com `--minimizado`, nunca em janela normal.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira em `specs/054-analise-materialista/`; agentes de implementação editam `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs` e `crates/cliente/src/main.rs` na etapa 12; o orquestrador executa gates, build, testes, o roteiro e as verificações finais.
- A sim já tem `PASSO_SEGUNDOS = 1/30`, `Mundo` com `BTreeMap`s determinísticos, `adicionar_pnj`/`adicionar_jogador`, `Pnj { id, posicao, pv, ativo, ataque_ticks }`, `aplicar_dano_a_pnj` com piso zero, `atacar_pnj` com cota por atacante/tick, `ACOES_PERMITIDAS`, `nivel_minimo_para_acao` (Q1/W2/E3/R6), `ValidacaoComandos::validar`, `CodigoRecusa`, `fase_prepare`/`fase_skills`/`fase_defesa`/`fase_ataque_pnjs`/`avancar_tick`, `serializar_estruturas`/`hash_estado` e `DANO_ATAQUE = 10`. O cliente já tem `texto_hud`, `posicao_pnj_mais_proximo`, `solicitar_atacar_pnj`, o espelho `Mundo`, `PEER_ID` e o padrão de handlers com `just_pressed`. O ciclo acrescenta apenas os campos, as constantes, a ação, o bônus, o decaimento, o segmento de hash, a linha de HUD, a tecla `E` e os cinco testes, sem crate novo.
- A suíte Rust existente tem 131 testes (87 em `crates/sim/tests/simulacao.rs`); os cinco novos são acrescentados sem modificar os existentes.
- Ambiente headless: nenhuma janela gráfica utilizável, rede, credencial ou comando `gh` é usado na redação destes artefatos; capturas visuais e execução ao vivo ficam **[NV]**.
- O scorecard **48/625** é a escolha do ciclo 054 em `.specify/memory/STATE.md` (2026-11-23T16:55Z); W/R e as demais E ficam para ciclos posteriores e a rede segue bloqueada por ADR-004 até decisão da SE.

## Out of scope

Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, GDD e documentos derivados; `Lightyear`, `async`, sockets, TCP, servidor, segundo jogador humano; execução de 0,3 s, canalização, silêncio, magia, RNG, chance, visibilidade através de terreno, linha de visão, mapa, rotas, coordenadas canônicas, item, equipamento, inventário; marcas em jogadores, Súditos, barricadas, pontes ou Nós; bônus na Sabotagem Estrutural, em estruturas ou em regras de vitória/derrota; novas habilidades W/R, custos, alcances, recargas, papéis ou progressão; teclas W/R; promoção de qualquer número a cânone; execução de gates, build, testes, formatadores, commits, push e PR; abertura de janela normal (cliente somente com `--minimizado`).

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: Análise Materialista (marca determinística em PNJ ativo) **4×2×2×3 = 48/625**, contra demais W/E/R **4×2×1×3 = 24/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. Registro em `.specify/memory/STATE.md` (2026-11-23T16:55Z). Nenhum manifesto é criado e nenhum número do protótipo é promovido a cânone.
