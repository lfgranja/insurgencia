# Research — Armadilha Improvisada (E do Artesão)

**Data**: 2026-11-24T12:15Z · **Slug**: `armadilha-improvisada` · Evidências lidas do contrato da sessão, da decisão 80/20 das 12:15Z, do GDD, da constituição v1.2.2 e do checkout em `661890f`; **nenhum comando de gate, build, teste, git remoto ou GitHub foi executado**. Evidência confidence overall: **high** para regra, constantes, rotas e fronteira; **[NV]** para efeito renderizado, verde de gates e nomes finais de T001..T012.

## Scorecard único do ciclo (repetido, sem pontuação paralela)

| Candidato | I × C × Fch × E⁻¹ × R⁻¹ ÷ 3125 | Resultado |
|:--|:--|:--|
| `armadilha-improvisada` (BF-002b) | 5×4×4×3×3 = 720/3125 | **720/3125** |
| Mapa/rotas de PVE (BF-003) | 5×3×3×2×3 = 270/3125 | 270/3125 |
| HUD `hash_estado` (BF-014) | 2×5×1×5×5 = 250/3125 | 250/3125 |
| Cliente `mover` (BF-012) | 4×3×3×2×2 = 144/3125 | 144/3125 |
| `contratos` consumidos (BF-013) | 5×2×5×1×2 = 100/3125 | 100/3125 |
| Rede (BF-001) | 5×2×5×1×1 = 50/3125 — inelegível (ADR-004) | 50/3125 |

## Usuários e demanda

- [source: `.specify/memory/STATE.md`, decisão `2026-11-24T12:15Z`, lida] Consumidor 1: **jogador do Artesão no cliente standalone**, que passa a ver `Armadilhas: N`, a armar artefatos com `E` e a sofrer/observar imobilização de inimigos por 60 ticks; consumidor 2: **os testes de integração do Núcleo determinístico**, que passam a auditar custo, recusa, disparo, imunidade, expiração, cota e hash. São consumidores fora dos artefatos do ciclo — passam o filtro eliminatório. (confidence: high)
- [source: `docs/GDD - Insurgência.md` §10.3:588-593, lido] Critérios de aceitação escritos no GDD: "O primeiro inimigo que entra no raio fica imobilizado por 2 s, sem receber dano mágico"; "Aliados e Súditos aliados atravessam a armadilha sem ativá-la"; "A armadilha expira após 60 s mesmo sem ter sido ativada". Demanda documentada, não observada em partida: o corte cobre aliados/Súditos por `equipe`, mas não implementa dano mágico, Súditos inimigos com regra própria nem os Desafios de Ilha citados no parágrafo. (confidence: high para o texto; **[NV]** para comportamento jogado)

## Prior art

- [source: `.specify/memory/STATE.md`, decisão 12:15Z, lida] BF-002b é o primeiro incremento **nomeado** do elenco Q/W/E/R (BF-002b..v) com âncora canônica em GDD §10.3:588-593; BF-002a (Escudo de Milícia), Q (`atacar_forte`) e E da Intelectual (`analise_materialista`) já foram entregues. O padrão de ciclos anteriores — constante `[HIPÓTESE]`, campo no estado, ação validada no sim, tick determinístico, hash, HUD e tecla — é reaproveitado, não reinventado. (confidence: high)
- [source: `specs/055-escudo-milicia/tasks.md`, `specs/054-analise-materialista/tasks.md` e `specs/056-servidor-validacao/spec.md`, lidos] Ciclos 054-056 entregaram 128→136→141→146 testes com o mesmo desenho: `BTreeMap` indexado por `id`, validação centralizada no sim, relógio por tick em `avancar_tick`, segmento estável em `serializar_jogadores`/`hash_estado`, whitelist derivada de `ACOES_PERMITIDAS` no CLI e linha de HUD. A armadilha segue esse precedente; nenhuma biblioteca, stack ou protocolo novo. (confidence: high)
- [source: `crates/sim/src/lib.rs:190-220,768-790,1065-1080`, lido] `ACOES_PERMITIDAS` já é pública e o mapa Q1/W2/E3/R6 já reserva o nível 3 para `analise_materialista`; `avancar_tick` já tem o par `avancar_analises_materialistas()` → `avancar_escudos()` antes de `fase_skills()`, e `avancar_escudos()` mostra o padrão de decremento por tick em ordem de `BTreeMap` com limpeza na transição para zero. A novidade é `avancar_armadilhas()` inserida no mesmo ponto da cadeia — não um novo motor de ticks. (confidence: high)
- [source: `.specify/assessments/alvo-treino/decision.md` e `.specify/assessments/servidor-validacao/decision.md`, lidos] A limitação do cliente standalone (um único `PEER_ID = 1`) já foi contornada com scaffolding não-canônico; a imobilização precisa de inimigo no espelho autoritativo para ser observada, portanto o roteiro de verificação deve usar o mesmo scaffolding quando aplicável. A whitelist do servidor headless deriva de `ACOES_PERMITIDAS`, então acrescentar `armadilha` ao sim a torna aceita também no CLI sem nova tabela. (confidence: medium — consequência de leitura estrutural, não executada)

## Contexto e custo da inação

- [source: `.specify/memory/STATE.md`, BF-002b..v e BF-003, lidos] Sem o corte, as habilidades restantes permanecem apenas no GDD; o Artesão continua sem controle de área observável e o Estado não tem como provar imobilização, imunidade de aliados ou expiração por `hash_estado`. A alternativa de maior impacto bruto (rede, 5) está bloqueada por ADR-004; mapa/rotas PVE (também 5) dependem de regras de spawn e geometria ainda `[HIPÓTESE]` e de frente muito maior. (confidence: high)
- [source: `.specify/memory/STATE.md`, reescalonamento de BF-014, lido] O diagnóstico de republicar `hash_estado` na HUD foi reavaliado de 1125 para 250/3125: não muda comportamento de jogo. Escolher a armadilha evita gastar o ciclo em observabilidade sem efeito e entrega uma regra com consumidor jogador. (confidence: high)
- [source: sequência dos ciclos 052-056, lida] A cadeia jogável (ataque, alvo, Fôlego, PNJs, escudo, validação do CLI) já foi fechada; a armadilha é a próxima fatia de habilidade com efeito sobre o próprio movimento do jogador. Inação deixaria `mover` incondicional no cliente e nenhuma resposta autoritativa a controle de área. (confidence: medium — inferência de sequência, não medição de uso)

## Dados e restrições

- [source: `docs/GDD - Insurgência.md` §10.3:588-593, lido] Números `[HIPÓTESE]`: 25 Fôlego, 5 Materiais, 1 s de execução, 3 m, raio 1 m, 2 s de imobilização, 60 s de duração, máximo 3, recarga 14 s. O corte registra 25/5/3/1/30/1/60/1800/420 como escolhas de protótipo derivadas de `PASSO_SEGUNDOS = 1/30`, sem promover nada a cânone e sem alterar o GDD. (confidence: high)
- [source: `crates/sim/src/lib.rs:15,190-220,281-360,462-510`, lido] `PASSO_SEGUNDOS = 1/30` → 1 s = **30 ticks**, 2 s = **60 ticks**, 60 s = **1800 ticks**, 14 s = **420 ticks**; `Jogador: Clone + Copy + Debug` comporta `equipe: u8` e dois campos `i64`; `Mundo` já usa `BTreeMap<u64, ...>` com `proxima_*_id` para barricadas e pontes — o mesmo desenho serve para `armadilhas`/`proxima_armadilha_id`. (confidence: high)
- [source: `crates/sim/src/lib.rs:768-790,1065-1080,1903-1949,1953-1991,2000-2085`, lido] `avancar_tick` fixa o ponto de inserção; `avancar_escudos()` é o modelo de decremento e limpeza; `hash_estado` usa SHA-256 sobre segmentos estáveis e `serializar_jogadores` em ordem de `peer_id`; `CodigoRecusa` e `ValidacaoComandos` concentram as recusas. A validação da armadilha deve seguir a ordem destino/alcance → Fôlego → Materiais → recarga, com `CodigoRecusa::ArmadilhaEmRecarga`, e avançar `seq`/`request_id` apenas em comando aceito. (confidence: high)
- [source: `crates/cliente/src/main.rs:612,923-955,1042,1123-1221`, lido] `KeyCode::KeyE` hoje chama `solicitar_analise_materialista`; o despacho passa a depender de `funcao == Funcao::Artesao` (`sim:246-266`), mantendo `analise_materialista` para as demais funções. `texto_hud` tem **99 linhas**, então a linha `Armadilhas: N` precisa entrar sem ultrapassar 100 linhas; `mover_jogador` e `sincronizar_sim` precisam consultar `imobilizado_ticks` no espelho autoritativo. (confidence: high)
- [source: `.specify/memory/STATE.md`, DoD 12:15Z, lida] Comandos que provam: `cargo test -p insurgencia-sim --test simulacao`, `cargo test -p insurgencia-cliente --lib`, `./verify.sh` remoto `gates`+`rust`, roteiro 10/10 em `.specify/validations/armadilha-improvisada/`, inventário **151 testes**. Nenhum foi executado neste assessment. (confidence: high para o contrato; **[NV]** para o resultado)
- [source: `.specify/memory/constitution.md:23-26,48-50,66-79`, lida] Constituição v1.2.2: R1 `go`; R4 antes de R7 `1.0.0-local`; `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A/bloqueado** (BF-007), PARE se invocada, nunca issue simulada, nunca `gh` improvisado; promoção `dev→main` e deploy são SWE-exclusivos. (confidence: high)
- [source: contrato da sessão, imutável] Rust `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; sem contratos, protocolo, Cargo, CI, stack, rede ou alteração do GDD canônico. (confidence: high)

## Evidências contrárias à ideia

- O GDD inclui "sem dano mágico", Desafio de Ilha e recarga de 14 s; o corte cobre somente a mecânica de imobilização e a imunidade por `equipe`. Os efeitos de ilha e qualquer dano ficam de fora: quem esperar o critério completo de §10.3 verá só o núcleo da regra. [source: GDD §10.3:588-593 + STATE 12:15Z] (confidence: high)
- `equipe` ainda não existe em `Jogador`: imunidade de aliados depende de um campo novo e de sua inicialização determinística em `Jogador::new`/criação; uma atribuição implícita (por exemplo, usar `peer_id`) mudaria a regra e violaria o contrato. [source: `crates/sim/src/lib.rs:281-360,540-570`] (confidence: high)
- A imobilização cruza duas frentes: a sim precisa recusar `mover` sem tocar posição nem monotônicos, e o cliente precisa deixar de agendar/injetar movimento enquanto `imobilizado_ticks > 0`. Qualquer divergência entre as duas resulta em jogador "parado" no cliente mas movido no Núcleo (ou vice-versa). É o maior risco técnico do corte. [source: `crates/cliente/src/main.rs:612,1042`; contrato] (confidence: high)
- `texto_hud` já ocupa 99 linhas; acrescentar uma linha sem compactar outra quebra a restrição `<100 linhas` do contrato. [source: `crates/cliente/src/main.rs:1123-1221`] (confidence: high)
- Mapa/rotas (270), HUD hash (250), cliente `mover` (144) e contratos (100) são opções legítimas, mas pontuam menos: as duas primeiras não entregam uma habilidade jogável neste ciclo, e as duas últimas são correções de plumbing/diagnóstico. Rede (50) é inelegível por autorização ADR-004. (confidence: high)

## Lacunas e perguntas em aberto

- R3 = **0 clarificações**: não há pergunta de produto bloqueante; o contrato resolve constantes, conversão em ticks, ordem de validação, fase do tick, imunidade por `equipe`, tecla `E`, bloqueio local e formato `Armadilhas: N`.
- **[NV]** Nomes definitivos de FR/SC/T em `specs/057-armadilha-improvisada/spec.md` e `tasks.md` (em construção); o rastro abaixo é o eco deste assessment, não a redação canônica.
- **[NV]** Verificação ao vivo: gates, `cargo test`, `cargo fmt`, `./verify.sh` (151), roteiro 10/10, CI e R4/R7 — nenhum executado aqui.
- **[NV]** Idem para renderização da HUD, para a ausência de divergência de hash entre mundos idênticos após a inclusão dos campos e das armadilhas, e para a contagem exata de linhas de `texto_hud` após a inclusão de `Armadilhas: N`.

## Rastreabilidade

- FR-001 (regra e números `[HIPÓTESE]`, sem alterar GDD) ↔ GDD §10.3:588-593 + decisão 12:15Z; FR-002 (`Armadilha` e campos em `Mundo`/`Jogador`) ↔ `sim:281-360,462-510`; FR-003 (ação `armadilha` nível 3, custo 25+5, alcance 3 m, validação ordenada, recusa e cota) ↔ `sim:190-220,1953-1991,2000-2085`; FR-004 (`avancar_armadilhas()` após `avancar_escudos()` e antes de `fase_skills()`, disparo por `equipe`, imobilização, expiração e limite) ↔ `sim:768-790,1065-1080`; FR-005 (`mover` ignorado e hash cobrindo campos/armadilhas em ordem de `id`) ↔ `sim:1903-1949`; FR-006 (cliente `E` por `Funcao`, bloqueio local, HUD, 5 testes, estilo e governança) ↔ contrato da sessão + constituição v1.2.2.
- SC-001..SC-006 em `define.md`; T001..T012 em `decision.md` (nomes finais em `specs/057-armadilha-improvisada/tasks.md`). R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**. Nada executado; todo resultado de verificação permanece **[NV]**.

## Fontes

Todas as fontes são artefatos locais do worktree lidos em modo somente-leitura; nenhuma URL foi buscada e nenhuma página externa foi citada (logo, não há sanitização de URL a registrar).

- `.specify/memory/STATE.md` (auditoria, backlog, decisão 80/20 e DoD do ciclo 057, 2026-11-24T12:15Z) — host: local, policy: allowlisted (artefato do repo).
- `docs/GDD - Insurgência.md` §10.3:588-593 — host: local, policy: allowlisted.
- `.specify/memory/constitution.md` v1.2.2 — host: local, policy: allowlisted.
- `crates/sim/src/lib.rs` e `crates/cliente/src/main.rs` no checkout `661890f` — host: local, policy: allowlisted.
- `specs/054-analise-materialista/tasks.md`, `specs/055-escudo-milicia/tasks.md`, `specs/056-servidor-validacao/spec.md`, `.specify/assessments/alvo-treino/decision.md` e `.specify/assessments/servidor-validacao/decision.md` — host: local, policy: allowlisted.
