# Feature Specification: PNJs defensores com PV e dano determinístico

**Branch**: `053-pnj-defensores` (base `4975fd4`) · **Created**: 2026-11-23 · **Status**: Draft · **R1**: `go` · **Sessão**: `20261123T155300Z-pnj-defensores`, `insurgencia - pnjs defensores`.

**Âncora**: decisão 80/20 do ciclo 053 registrada em `.specify/memory/STATE.md` (2026-11-23T15:53Z): **PNJs defensores (PV + dano determinístico) = 4×2×2×3 = 48/625**, escolhida sobre **novas habilidades W/E/R 4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. O desempate com novas habilidades W/E/R cabe a esta frente: GDD §8.1:361 e §8.4/§8.5 já descrevem guardas/patrulhas como oposição PVE, e o protótipo só precisa de entidades determinísticas e de uma ação sobre o estado, sem inventar habilidade, contrato ou stack. Pela convenção de GDD §8.1:351, efetivo, dano e PV ausentes das fontes são `[HIPÓTESE]`: 4 guardas, 8 dmg/s por 3 s (24 pontos) e a patrulha de §8.5 são lidos e rotulados, nunca promovidos a cânone. O GDD é lido, nunca reescrito.

## User Scenarios

### US1 — Entidade PNJ e registro determinístico (P1)

O jogador passa a enfrentar guardas/patrulhas como entidades observáveis no estado autoritativo: `pub struct Pnj { id, posicao, pv, ativo }`, `Mundo.pnjs: BTreeMap<u64, Pnj>` e `Mundo::adicionar_pnj(id, posicao)`, que insere com `pv = PV_PNJ = 24` e `ativo = true`. O efetivo padrão do protótipo é **4 PNJs** (`TOTAL_PNJS_PADRAO = 4`), derivado de GDD §8.1:361/§8.5 e rotulado `[HIPÓTESE]`/protótipo: sem mapa, o cliente standalone injeta quatro posições fixas na inicialização, sem aleatoriedade, I/O, relógio externo ou segundo jogador. A estrutura é `Clone + Copy + Debug` com `///` em todos os campos, mantém a ordem estável por `id` e não carrega contador próprio de ataque — a cadência vem do `tick` global (US4). Nenhuma habilidade, contrato, mensagem de protocolo, crate ou dependência nova.

**Why this priority**: sem a entidade e o registro, nenhum dano, ataque ou contagem de PNJs no HUD é observável no hash determinístico; é a base de todas as outras histórias.

**Independent Test**: inspecionar `crates/sim/src/lib.rs` (`Pnj`, `Mundo.pnjs`, `Mundo::adicionar_pnj`, constantes rotuladas) e confirmar quatro inserções com `pv = 24`, `ativo = true`, ordem por `id` e ausência de `Instant`/`SystemTime`/aleatoriedade.

**Acceptance Scenarios**:
1. **Given** um `Mundo` recém-criado, **When** `adicionar_pnj` é chamado para os ids 1..4 em posições fixas, **Then** `pnjs` contém quatro entradas com `pv = 24` e `ativo = true`, em ordem crescente de `id`, e o hash muda em relação ao mundo sem PNJs.
2. **Given** um `id` já presente, **When** `adicionar_pnj` é chamado novamente, **Then** a entrada é reiniciada para `pv = 24`/`ativo = true` de forma determinística (sem duplicata, sem pânico).
3. **Given** a spec, **When** os números são lidos, **Then** 4 PNJs, 24 PV e todas as demais constantes aparecem rotulados `[HIPÓTESE]`/protótipo, sem promoção a cânone e sem reescrita do GDD.

### US2 — Dano do jogador a PNJ (P1)

`Mundo::aplicar_dano_a_pnj(id, dano) -> bool` aplica o dano do jogador — reutilizando `DANO_ATAQUE = 10` (`[HIPÓTESE]`/protótipo da spec 047) — com `saturating_sub`, marca `ativo = pv > 0` e retorna `true` apenas quando o PNJ passa a inativo nesta chamada. Dano não positivo ou `id` inexistente é no-op. Diferente de barricadas e pontes, a entrada **não é removida** quando os PV chegam a zero: ela permanece com `ativo = false` para que a morte seja observável no `hash_estado` e o HUD possa contar somente os ativos. Um PNJ inativo não é alvo válido e não ataca (US4).

**Why this priority**: é o mecanismo que torna o combate jogável e verificável; sem ele, a ação de US3 não teria efeito determinístico nem o contador do HUD faria sentido.

**Independent Test**: inspecionar `aplicar_dano_a_pnj` e a suíte determinística; aplicar 0, negativo, 10 (24→14), 14 (14→0) e dano extra a um PNJ já inativo, comparando PV, `ativo`, retorno e hash.

**Acceptance Scenarios**:
1. **Given** um PNJ com `pv = 24`, **When** o jogador aplica 10 de dano, **Then** `pv = 14`, `ativo = true`, o retorno é `false` e o hash muda de forma previsível.
2. **Given** `pv = 14`, **When** são aplicados 14 de dano, **Then** `pv = 0`, `ativo = false`, o retorno é `true` e a entrada permanece no mapa (não removida).
3. **Given** `dano = 0`, `dano = -1` ou `id = 99`, **When** `aplicar_dano_a_pnj` é chamado, **Then** nenhum PNJ muda e o hash permanece idêntico, sem pânico.

### US3 — Ação `atacar_pnj` desbloqueada no nível 1 (P1)

Uma ação nova entra no núcleo e no cliente: `atacar_pnj`, com `destino` na posição do PNJ. `ACOES_PERMITIDAS` ganha a entrada `"atacar_pnj"`; `nivel_minimo_para_acao` devolve 1 por padrão (`_ => 1`), então a ação é aceita já no nível 1 e se soma às ações livres de gate (`mover`, sinais e coleta). `ValidacaoComandos::validar` aceita `destino` coincidente com a posição de um PNJ **ativo** (`ponto_igual`) e devolve `DestinoInvalido` quando o destino está ausente, fora do ponto ou o PNJ está inativo/inexistente. Em `fase_skills`, o despacho usa a mesma cota de um ataque por atacante por tick de `atacar_forte`, chama `aplicar_dano_a_pnj` com `DANO_ATAQUE = 10` e só atualiza `seq`/`request_id` quando o comando é efetivamente processado. Nenhuma habilidade W/E/R é criada, nenhum custo de Fôlego, alcance ou recarga novo é introduzido e nenhum campo de protocolo é acrescentado.

**Why this priority**: é a interação jogável do ciclo — liga a tecla `T` ao estado determinístico sem custo de implementação de habilidades.

**Independent Test**: inspecionar `ACOES_PERMITIDAS`, `nivel_minimo_para_acao`, `ValidacaoComandos::validar`, o despacho em `fase_skills` e o tratamento de `KeyCode::KeyT`; validar nível 1, destino válido/inválido, PNJ inativo e dois comandos do mesmo atacante no mesmo tick.

**Acceptance Scenarios**:
1. **Given** `nivel = 1` e um PNJ ativo no destino, **When** `atacar_pnj` é validado, **Then** é aceito e, no despacho, o PNJ perde 10 PV; `atacar_forte`, construções, propaganda e `atacar_no` mantêm seus limiares Q1/W2/E3/R6 inalterados.
2. **Given** `destino = None` ou um destino sem PNJ ativo, **When** `atacar_pnj` é validado, **Then** o resultado é `Err(CodigoRecusa::DestinoInvalido)` e nenhum PV muda.
3. **Given** dois comandos `atacar_pnj` do mesmo atacante no mesmo tick com destinos válidos, **When** `fase_skills` roda, **Then** no máximo um é aplicado (cota por atacante/tick), sem efeito duplo.
4. **Given** o cliente final, **When** o tratamento de `KeyCode` é inspecionado, **Then** `KeyT` agenda `atacar_pnj` no PNJ mais próximo do jogador (desempate por menor `id`) e nenhuma tecla W/E/R é acrescentada.

### US4 — Ataque determinístico do PNJ ao jogador (P1)

A oposição PVE passa a devolver dano: `DANO_ATAQUE_PNJ = 8`, `TICKS_ATAQUE_PNJ = 90` (3 s a 30 Hz) e `ALCANCE_ATAQUE_PNJ_M = 2.0`, todos `[HIPÓTESE]`/derivação de GDD §8.1:361 (8 dmg/s por 3 s) e do alcance de contato existente. A resolução acontece em `fase_defesa`, depois de `fase_skills` e antes de `fase_controle_ilhas`, somente quando `tick % TICKS_ATAQUE_PNJ == 0`: para cada PNJ `ativo` em ordem de `id`, o primeiro jogador dentro de 2 m em ordem de `peer_id` sofre 8 de dano (`vida.saturating_sub(8).max(0)`, sem vida negativa). A cadência usa o `tick` global — sem contador em `Pnj`, sem `Instant`, `sleep`, aleatoriedade ou I/O. Um PNJ morto no mesmo tick (`ativo = false` após US2) não ataca; nenhum sinal, efeito ou habilidade nova é emitida.

**Why this priority**: sem a reação do PNJ, o combate seria unidirecional e o dano de 8 por 3 s de GDD §8.1 não seria observável no estado determinístico.

**Independent Test**: inspecionar as constantes, `fase_defesa` e `avancar_tick`; percorrer ticks 0, 1, 89, 90, 180 e conferir quem leva dano, a saturação em zero e a ordem de desempate.

**Acceptance Scenarios**:
1. **Given** um PNJ ativo e um jogador a 2 m ou menos, **When** o tick é múltiplo positivo de 90 (ex.: 90), **Then** a vida do jogador cai 8 (100→92); nos ticks 1 e 89 não há dano.
2. **Given** dois jogadores dentro do alcance, **When** o PNJ ataca no tick 90, **Then** apenas o de menor `peer_id` leva 8 de dano; critério de desempate determinístico.
3. **Given** `vida = 3`, **When** o PNJ ataca, **Then** `vida = 0` (nunca negativa); com `vida = 0` o dano não altera o valor.
4. **Given** um PNJ com `ativo = false`, **When** o tick 90 é processado, **Then** ele não ataca e o hash reflete apenas `pv`/`ativo`, sem segmento de ataque.

### US5 — HUD, hash e fronteira determinística (P1)

`texto_hud` em `crates/cliente/src/main.rs` passa a incluir `PNJs: N`, contando `simulacao.0.pnjs.values().filter(|pnj| pnj.ativo).count()` a partir do espelho autoritativo. O cliente trata `KeyCode::KeyT` com `just_pressed`, agendando `atacar_pnj` com `destino` na posição do PNJ ativo mais próximo (mesmo padrão de `posicao_no_mais_proximo`, com desempate por menor `id`); sem PNJ ativo, nenhum comando é agendado. Todas as linhas e teclas existentes (`WASD`/`B`/`P`/`C`/`G`/`R`/`F`/`Q`) são preservadas. `serializar_estruturas` (ou segmento equivalente em `hash_estado`) passa a cobrir `pnjs` com `id:posicao:pv:ativo` em ordem de `id`, mantendo todos os segmentos atuais (tick, epoch, match_id, jogadores, estruturas, nós, resistência, resultado, `ultima_recusa`). Nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `concurrency.jsonl` ou `STATE.md`.

**Why this priority**: sem o HUD o jogador não vê a oposição; sem o hash as mudanças não são comparáveis em replays; sem a fronteira explícita, uma frente de PVE viraria porta de entrada para protocolo e stack novos.

**Independent Test**: inspecionar `texto_hud`, `solicitar_ataque_pnj`, `posicao_pnj_mais_proximo`, `serializar_estruturas` e `hash_estado`; comparar hashes antes/depois de dano e de morte de PNJ; conferir `git status --short`.

**Acceptance Scenarios**:
1. **Given** quatro PNJs ativos e um inativo, **When** `texto_hud` é montado, **Then** aparece `PNJs: 4` ao lado de `Posicao`, `Ilha`, `Função`, `Nível`, `Limites`, `Suprimentos`, `Materiais`, `Fôlego`, `Vida`, `Alvo`, `Autogestão`, `Barreiras`, `Pontes`, `Súditos`, `Ilhas`, `Sinais`, `Resistência`, `Nós`, `Resultado`, `Construindo`, `Coleta` e da dica de teclas existente, acrescida de `T para atacar PNJ`.
2. **Given** dois replays com o mesmo input, **When** `hash_estado` é comparado, **Then** as strings são idênticas; mudar apenas `pv` ou `ativo` de um PNJ altera o hash de forma previsível e nenhum segmento existente desaparece.
3. **Given** o estado final do ciclo, **When** os arquivos alterados são listados, **Then** aparecem somente os 4 artefatos SDD e os paths de código atribuídos (sim, testes da sim e `crates/cliente/src/main.rs`), sem contratos, protocolo, Cargo, CI ou rede.

### US6 — Governança, rotulação e estilo (P2)

R1 é `go`; a sequência da constituição v1.2.2 coloca `tasks` antes de `analyze` (READ-ONLY) e R4 (validate) antes de R7 (release `1.0.0-local`); **R6 é N/A** — se invocada, PARE e escale ao SWE, sem issue simulada, sem `gh` improvisado e sem alegar execução. O código segue `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100` e `///` em itens públicos (o atributo de lint `#[expect(clippy::..., reason = "...")]`, já usado no cliente, não é chamada de método e permanece permitido). Nenhum número do protótipo é promovido a cânone: 4 PNJs, 24 PV, dano 10 do jogador, dano 8 do PNJ, 90 ticks e alcance 2 m são `[HIPÓTESE]`/derivação. Execução de gates, build, testes, formatadores, commits, push e PR **[NV]** para este agente de documentação.

**Why this priority**: preserva a cadeia de rastreabilidade do hub e impede que uma frente de PVE vire dívida de automação, mudança de stack ou regra canônica inventada.

**Independent Test**: inspecionar os 4 artefatos, a ordem das portarias e o diff de código, conferindo rótulos `[HIPÓTESE]`, R6 N/A e os atributos de lint.

**Acceptance Scenarios**:
1. **Given** os 4 artefatos, **When** a governança é revisada, **Then** R1 `go`, R6 N/A, `tasks` antes de `analyze` e R4 antes de R7 `1.0.0-local` aparecem explicitamente, com scorecard **48/625**.
2. **Given** o código, **When** o estilo é conferido, **Then** não há `unsafe`, `unwrap`/`.expect()`/`#[allow]`, e as linhas respeitam 100 colunas.
3. **Given** os números do protótipo, **When** a spec é lida, **Then** 4/24/10/8/90/2 m aparecem rotulados como `[HIPÓTESE]`/derivação, e novas habilidades W/E/R, rede e mapa permanecem fora de escopo.

## Edge Cases

- `aplicar_dano_a_pnj` com `dano <= 0` ou `id` inexistente: no-op silencioso, sem alterar PV, `ativo`, hash ou fila de sinais.
- PV atinge exatamente 0: `ativo = false`, entrada **mantida** no `BTreeMap` (ao contrário de barreiras/ponte), HUD conta somente ativos e o hash registra a morte; `adicionar_pnj` com o mesmo `id` reativa e restaura 24 PV.
- `destino` ausente ou em posição de PNJ inativo/inexistente: `ValidacaoComandos::validar` devolve `DestinoInvalido`; um comando recusado não consome cota de ataque, não altera PV nem atualiza `seq`/`request_id`.
- Dois comandos `atacar_pnj` do mesmo atacante no mesmo tick: a cota de um ataque por atacante por tick impede dano duplo; a cota só é consumida por comando válido.
- Ordem do tick: `fase_skills` (dano do jogador) roda antes de `fase_defesa` (dano do PNJ). Um PNJ morto no tick 90 não contra-ataca no tick 90; o jogador que o matou ainda pode sofrer dano de outro PNJ ativo no mesmo tick.
- Cadência em `tick % 90 == 0`: o tick 0 também satisfaz a condição; para evitar dano na inicialização, a resolução exige `tick > 0` **ou** a presença prévia de jogador e PNJ no alcance — a regra adotada é `tick > 0 && tick % 90 == 0`, documentada e coberta no roteiro.
- Alcance exatamente 2 m: aceito (`<= ALCANCE_ATAQUE_PNJ_M`), sem tolerância de arredondamento adicional; comparação por distância euclidiana, determinística. `ponto_igual` usa `total_cmp` para o destino da ação do jogador.
- Vida do jogador em 3 com dano 8: satura em 0 (`saturating_sub(...).max(0)`); dano posterior a 0 não produz valor negativo nem sinal novo.
- Múltiplos jogadores no alcance do mesmo PNJ: apenas o de menor `peer_id` é atingido; múltiplos PNJs no alcance do mesmo jogador aplicam dano em ordem de `id`, cada qual uma vez por tick de ataque.
- Sem PNJ ativo: `T` não agenda comando (o helper devolve `None`), `PNJs: 0` aparece no HUD e nenhum ataque é resolvido.
- HUD sem surface automatizável: o alvo é headless; a checagem automatizável é inspeção de `texto_hud` e comparação de hash. Captura visual real da linha `PNJs:` fica **[NV]** até verificação humana.
- R6 invocada por engano: PARE, escale ao SWE, sem issue simulada e sem `gh` improvisado; dívida de automação registrada, não executada.

## Requirements

### Functional Requirements

- **FR-001 (entidade PNJ e registro)**: em `crates/sim/src/lib.rs`, `pub struct Pnj { pub id: u64, pub posicao: (f64, f64), pub pv: i64, pub ativo: bool }` é documentada com `///` em cada campo e deriva `Clone, Copy, Debug`; `Mundo.pnjs: BTreeMap<u64, Pnj>` mantém a ordem estável por `id`; `Mundo::adicionar_pnj(&mut self, id: u64, posicao: (f64, f64))` insere com `pv = PV_PNJ = 24` e `ativo = true`, reiniciando uma entrada existente com o mesmo `id`. `TOTAL_PNJS_PADRAO: usize = 4` e todas as constantes são `[HIPÓTESE]`/protótipo de GDD §8.1:361 e §8.4/§8.5 (convenção de rotulação em §8.1:351), nunca cânone: 4 guardas/patrulha e 24 PV são derivações de protótipo sem mapa, sem aleatoriedade, I/O, relógio externo, crate, contrato ou protocolo novo.
- **FR-002 (dano do jogador a PNJ)**: `pub const PV_PNJ: i64 = 24` (`[HIPÓTESE]`: 8 dmg/s × 3 s em GDD §8.1:361) e `Mundo::aplicar_dano_a_pnj(&mut self, id: u64, dano: i64) -> bool` aplica `dano` com `saturating_sub`, atualiza `ativo = pv > 0` e retorna `true` somente na transição para inativo; `dano <= 0` e `id` inexistente são no-op. A entrada permanece no `BTreeMap` após a morte, tornando o óbito observável no `hash_estado` e distinto de "nunca existiu". O dano do jogador reutiliza `DANO_ATAQUE = 10` (`[HIPÓTESE]`/protótipo da spec 047), sem novo número canônico.
- **FR-003 (ação `atacar_pnj` no nível 1)**: `ACOES_PERMITIDAS` ganha a entrada `"atacar_pnj"`; `nivel_minimo_para_acao` mantém `_ => 1`, portanto a ação está disponível no nível 1 sem alterar o mapa Q1/W2/E3/R6 das ações existentes. `ValidacaoComandos::validar` aceita `destino` coincidente com a posição de um PNJ ativo (`ponto_igual`) e devolve `DestinoInvalido` para destino ausente, fora do ponto, PNJ inativo ou inexistente, preservando a precedência de `PeerNaoAutorizado`, `SeqOuRequestReplay`, `AcaoDesconhecida` e `AcaoBloqueada`. Em `fase_skills`, o despacho usa a cota de um ataque por atacante por tick, chama `aplicar_dano_a_pnj(..., DANO_ATAQUE)` e atualiza `seq`/`request_id` apenas para comandos processados. Nenhuma habilidade W/E/R, Fôlego, alcance, recarga, sinal, comando de protocolo ou efeito novo é criado; `T` é apenas o gatilho local (FR-005).
- **FR-004 (ataque do PNJ ao jogador)**: `DANO_ATAQUE_PNJ: i64 = 8`, `TICKS_ATAQUE_PNJ: i64 = 90` (3 s a 30 Hz) e `ALCANCE_ATAQUE_PNJ_M: f64 = 2.0` são documentados como `[HIPÓTESE]`/derivação de GDD §8.1:361 (8 dmg/s por 3 s, 24 pontos) e do `ALCANCE_CONTATO_M` existente. Em `fase_defesa`, após `fase_skills` e antes de `fase_controle_ilhas`, quando `tick > 0 && tick % TICKS_ATAQUE_PNJ == 0`, cada PNJ `ativo` em ordem de `id` aplica 8 de dano ao primeiro jogador dentro de 2 m em ordem de `peer_id`, com `vida.saturating_sub(8).max(0)`. A cadência vem do `tick` global: `Pnj` não ganha contador, e não há `Instant`, `SystemTime`, `sleep`, aleatoriedade, I/O, sinal ou habilidade nova. PNJ inativo não ataca.
- **FR-005 (HUD, hash e fronteira determinística)**: `texto_hud` em `crates/cliente/src/main.rs` acrescenta `PNJs: N`, com `N = simulacao.0.pnjs.values().filter(|p| p.ativo).count()` lido do espelho autoritativo. Um handler de `KeyCode::KeyT` com `just_pressed` agenda `atacar_pnj` no `destino` do PNJ ativo mais próximo do jogador (desempate por menor `id`, padrão análogo a `posicao_no_mais_proximo`); sem alvo, nenhum comando é agendado. Todas as linhas e teclas existentes (`WASD`/`B`/`P`/`C`/`G`/`R`/`F`/`Q`) são preservadas e nenhuma tecla W/E/R é introduzida — `T` é a única tecla nova. `serializar_estruturas`/`hash_estado` inclui segmento `pnjs` com `id:posicao:pv:ativo` em ordem de `id`, preservando todos os segmentos atuais e a comparabilidade entre replays. Nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `concurrency.jsonl` ou `STATE.md`.
- **FR-006 (governança e estilo)**: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap()`/`.expect()`/`#[allow]`, `max_width = 100`, `///` em itens públicos (atributo de lint `#[expect(...)]` permitido); R1 `go`, R6 **N/A** por v1.2.2 (PARE se invocada, sem issue simulada e sem `gh` improvisado), `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`; commits em PT-BR com staging explícito; gates/build/testes/formatadores/commit/push/PR **[NV]** para este agente.

### Key Entities

- **`Pnj`**: entidade determinística `{ id, posicao, pv, ativo }`, `Clone + Copy + Debug`, rotulada `[HIPÓTESE]`/protótipo; sem contador de ataque — a cadência é o `tick` global.
- **`Mundo.pnjs` / `Mundo::adicionar_pnj`**: registro `BTreeMap<u64, Pnj>` em ordem de `id` e ponto de inserção com `pv = 24`/`ativo = true` (4 PNJs no protótipo).
- **`Mundo::aplicar_dano_a_pnj`**: aplicação de dano do jogador (`DANO_ATAQUE = 10`) com `saturating_sub`, transição `ativo = pv > 0` e manutenção da entrada para observabilidade no hash.
- **`atacar_pnj`**: ação de nível 1 com `destino` na posição do PNJ ativo; entra em `ACOES_PERMITIDAS`, na validação e no despacho de `fase_skills`, sem habilidade, tecla W/E/R ou campo de protocolo novo.
- **`DANO_ATAQUE_PNJ` / `TICKS_ATAQUE_PNJ` / `ALCANCE_ATAQUE_PNJ_M`**: constantes `8`, `90` e `2.0`, derivadas/hipotéticas de GDD §8.1:361; usadas em `fase_defesa` com `tick > 0 && tick % 90 == 0`.
- **`fase_defesa`**: fase determinística de contra-ataque, depois de `fase_skills` e antes de `fase_controle_ilhas`, em ordem de `id` dos PNJs e `peer_id` dos jogadores.
- **`serializar_estruturas` / `hash_estado`**: serialização estável que passa a cobrir `pnjs`, preservando todos os segmentos e a comparabilidade entre replays.
- **`texto_hud` / handler de `KeyT`**: linha `PNJs: N` e gatilho local da ação, sem novas habilidades W/E/R e com captura visual **[NV]** neste alvo headless.

## Success Criteria

- **SC-001 (entidade e registro)**: `Pnj` expõe exatamente `id`, `posicao`, `pv` e `ativo` documentados; `adicionar_pnj` insere 4 entradas com `pv = 24`/`ativo = true` em ordem de `id`, reinserção reinicia o estado e o hash muda de forma previsível. Todos os números aparecem rotulados `[HIPÓTESE]`/protótipo nos 4 artefatos, sem promoção a cânone.
- **SC-002 (dano do jogador)**: `aplicar_dano_a_pnj` com dano 10 reduz 24→14 mantendo `ativo`, o dano seguinte leva a 0 e a `false` com retorno `true`, a entrada permanece no mapa, e `dano <= 0`/`id` inexistente são no-op com hash inalterado — sempre sem pânico.
- **SC-003 (ação)**: `atacar_pnj` é aceito no nível 1 com `destino` em PNJ ativo e aplica 10 de dano; destino ausente/fora do ponto ou PNJ inativo devolve `DestinoInvalido`; no máximo um ataque por atacante por tick; `ACOES_PERMITIDAS`, os limiares Q1/W2/E3/R6 e as teclas existentes permanecem intactos — nenhuma habilidade nova é introduzida.
- **SC-004 (contra-ataque)**: com PNJ ativo e jogador a ≤2 m, os ticks 90, 180 e 270 aplicam 8 de dano (100→92→84→76), os ticks 1 e 89 não aplicam; desempate por menor `peer_id`, saturação em 0, nenhum dano de PNJ inativo e nenhum contador em `Pnj`.
- **SC-005 (HUD/hash/fronteira)**: `texto_hud` acrescenta `PNJs: N` contando apenas ativos e `KeyT` agenda `atacar_pnj` no PNJ ativo mais próximo, preservando todas as linhas e teclas; `hash_estado` cobre `pnjs` em ordem de `id`, é idêntico entre replays com o mesmo input e muda de forma previsível quando apenas `pv`/`ativo` mudam; contratos, protocolo, Cargo, manifesto, CI e redes permanecem intactos.
- **SC-006 (governança/estilo)**: R1 `go`, R6 N/A, `tasks` antes de `analyze` (READ-ONLY), R4 antes de R7 `1.0.0-local`; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`.expect()`/`#[allow]`, `max_width = 100`; execução, capturas visuais e publicação **[NV]** até o orquestrador.

## Clarificações

- **Guardas, patrulhas e os números do protótipo** vêm de GDD §8.1:351,361 e §8.4/§8.5, mas **todos são `[HIPÓTESE]`**: 4 forças de segurança por ilha, 8 dmg/s por 3 s (24 pontos) e PV de combatente 100. O protótipo os adota como regras de simulação, sem promovê-los a cânone e sem reescrever o GDD.
- **24 PV do PNJ** é a soma hipotética 8 × 3 s de GDD §8.1:361; **dano 10 do jogador** reutiliza `DANO_ATAQUE` da spec 047; **8 a cada 90 ticks** é a conversão de 8 dmg/s por 3 s para o passo fixo de 30 Hz do protótipo; **2 m** reaproveita o alcance de contato existente. Todas são derivações rotuladas, não novo número canônico.
- **A cadência do ataque usa o `tick` global**, com guarda `tick > 0`, e não um contador em `Pnj`: isso mantém a estrutura exigida pelo protótipo (`id`, `posicao`, `pv`, `ativo`) e torna o ataque reproduzível em replays. Ticks 90, 180 e 270 são os três primeiros disparos.
- **`Pnj` inativo permanece no mapa**: ao contrário de barricadas e pontes, a morte não remove a entrada; `ativo = false` é o estado observável no hash e o filtro do HUD. `adicionar_pnj` com o mesmo `id` restaura `pv = 24` e `ativo = true`.
- **`atacar_pnj` é uma ação nova no núcleo, não uma habilidade W/E/R**: não há custo, alcance, recarga, ícone, sinal ou efeito novo. As habilidades reais de GDD §10.2–10.6 e as teclas W/E/R continuam diferidas. A única tecla acrescentada é `T`, no cliente.
- **Sem mapa, ilha ou coordenada canônica**: as quatro posições são injetadas no cliente standalone como simplificação de protótipo; nenhuma rota, coordenada de ilha, item, equipamento ou inventário é introduzido.
- **`DestinoInvalido` tem prioridade sobre recusas específicas de `atacar_pnj`**, mas nunca sobre `PeerNaoAutorizado`, `SeqOuRequestReplay`, `AcaoDesconhecida` ou `AcaoBloqueada`: a ordem das guardas é parte do contrato observável.
- **R6 é N/A** por constituição v1.2.2: fora da sequência executável até `resolve-tasks.sh`/adaptador existirem; se invocada, PARE e escale ao SWE, sem simular issue, sem `gh` improvisado e sem alegar execução.
- **R3 = 0** clarificações em aberto: o GDD fornece a moldura (guardas/patrulhas, dano, PV) e as lacunas (posições, cadência exata, contagem) são simplificações explícitas de protótipo, não ambiguidades.
- Execução de gates/build/testes/formatadores e capturas visuais não têm surface automatizável neste agente: ficam **[NV]** até o orquestrador.

## Assumptions

- O agente de documentação escreve somente os 4 artefatos da fronteira; agentes de implementação editam `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs` e `crates/cliente/src/main.rs` na etapa 12; o orquestrador executa gates, build, testes e verificações finais.
- A sim já tem `PASSO_SEGUNDOS = 1 / 30`, `Mundo` com `BTreeMap`s determinísticos, `adicionar_jogador`, `fase_prepare`, `fase_skills`, `fase_defesa`, `fase_controle_ilhas`, `fase_vitoria`, `fase_finish`, `avancar_tick`, `ACOES_PERMITIDAS`, `ValidacaoComandos::validar`, `CodigoRecusa`, `serializar_estruturas`, `hash_estado`, `aplicar_dano_a_sudito`/`a_barreira`/`a_ponte`, `DANO_ATAQUE = 10`, `ALCANCE_CONTATO_M = 2.0`, `nivel_minimo_para_acao` e a cota de ataque por atacante/tick. O ciclo acrescenta `Pnj`, `Mundo.pnjs`, `adicionar_pnj`, `aplicar_dano_a_pnj`, a ação `atacar_pnj`, o ataque em `fase_defesa` e o segmento `pnjs` no hash, sem crate novo. O cliente já tem `texto_hud`, `atualizar_texto`, `posicao_no_mais_proximo`, o espelho `Mundo`, `PEER_ID` e o padrão de handlers de tecla.
- Ambiente headless: nenhuma janela gráfica utilizável, rede, credencial ou comando `gh` é usado na redação destes artefatos; capturas visuais e execução ao vivo ficam **[NV]**.
- O scorecard **48/625** é a escolha do ciclo 053: papéis/níveis foi fechado no ciclo 052; novas habilidades W/E/R empatam em 48/625 e ficam para o próximo ciclo, e a rede (10/625) segue bloqueada por ADR-004 até decisão da SE.

## Out of scope

Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, manifesto, CI, rede, `AGENTS.md`, `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md` e documentos derivados; `Lightyear`, `async`, sockets, TCP, servidor, segundo jogador humano; novas habilidades W/E/R (GDD §10.2–10.6), custos/alcances/recargas/efeitos novos, seleção de papel e progressão nova; mapa, rotas, coordenadas de ilha canônicas, items, equipamento, inventário, economia nova, Plano Mestre e regras de vitória/derrota dos ciclos 049–051; promoção de qualquer número a cânone; execução de gates, build, testes, formatadores, commits, push e PR.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Esta seção é aditiva e **documental**: nenhum gate determinístico lê templates. O padrão do hub é copiar `.specify/templates/context-budget-manifest-template.json` para `examples/context-budget-manifest.json` e validá-lo com `scripts/ci/check_context_budget.py`; como o manifesto é declarativo e opt-in, nenhum arquivo dessa natureza é criado neste ciclo (fora de fronteira). Honestidade dos rótulos e limites de autoridade estão em `docs/governance/context-budget-manifest.md`.

**80/20**: PNJs defensores (PV + dano determinístico) **4×2×2×3 = 48/625**, contra novas habilidades W/E/R **4×2×2×3 = 48/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); critério impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625. Registro em `.specify/memory/STATE.md` (2026-11-23T15:53Z). Nenhum manifesto é criado e nenhum número do protótipo é promovido a cânone.
