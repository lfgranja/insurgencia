# Define — PNJs defensores com PV e dano determinístico

- **Slug**: `pnj-defensores`
- **Created**: 2026-11-23T15:53Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §8.1:357,361,363 e §8.4/§8.5, STATE.md ciclo 053, constituição v1.2.2)
- **Scorecard congelado** (`STATE.md`, 2026-11-23T15:53Z): PNJs defensores **48/625** (4×2×2×3), novas habilidades W/E/R 48/625 (4×2×2×3), rede 10/625 (5×2×1×1); critério impacto × confiança × facilidade ÷ risco, denominador 5⁴ = **625**. Empate 48/625 desempatado pelo contrato da sessão e pelo lastro do GDD §8.1:361.

## Problem Statement

O GDD §8.1:361 descreve forças de segurança e uma penalidade de 8 dmg/s por 3 s — 24 pontos — como **`[HIPÓTESE]`**, e §8.4/§8.5 dão o lastro qualitativo de patrulhas e "Negociação com PNJs". No Núcleo Rust verificado, porém, `Mundo` (`:368-403`) não tem campo `pnjs`, não existe `struct Pnj`, não há `adicionar_pnj` nem `aplicar_dano_a_pnj`, e `ACOES_PERMITIDAS` (`:137-148`) não contém `atacar_pnj`. O jogador pode atacar (`atacar_forte` com 10 de dano, alcance 2 m) e já tem papéis/níveis (ciclo 052), mas **não há adversário com PV** que reaja ao golpe nem que devolva dano em alcance: o PVE do GDD existe só como parágrafo, sem verdade observável, sem relógio determinístico e sem linha no cliente. Sem isso, qualquer ciclo futuro que queira "defensor de ilha", "patrulha" ou "pressão de gás" precisaria reconstruir entidade, dano, tick e hash desde zero.

## Affected Users & Stakeholders

- **Users**: jogadores da Comuna — precisam de um adversário cuja resistência seja legível (`PNJs: N` ativos) e cuja reação seja previsível (8 de dano a cada 3 s dentro de 2 m), para que atacar com `T` tenha efeito observável e não seja um comando vazio; jogadores que testam limites — precisam que atacar fora de alcance ou um PNJ já inativo não altere o estado.
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de entidade em `BTreeMap`, API de dano com guardas, relógio por tick e cobertura em `hash_estado` sem quebrar replay; SWE/orquestrador — precisa de uma fatia testável sem mudança de contrato, protocolo, Cargo ou stack; designer — precisa que 24 PV, 8 dmg, 90 ticks e 2 m continuem `[HIPÓTESE]`/protótipo, e que as habilidades completas W/E/R e o escalonamento 4→8/×5 não sejam incorporados "de graça".

## Goals

- Introduzir `Pnj { id, posicao, pv, ativo }` e `Mundo.pnjs: BTreeMap<u64, Pnj>` em `crates/sim/src/lib.rs`, em ordem estável de `id`, com os campos públicos necessários para inspeção determinística.
- Fornecer `adicionar_pnj` (registro de defensor ativo com PV inicial **24**, `[HIPÓTESE]`/protótipo) e `aplicar_dano_a_pnj` (dano ≤ 0, `id` inexistente ou PNJ inativo não alteram estado; PV nunca negativo; `ativo = false` quando PV ≤ 0).
- Expor a ação `atacar_pnj` no nível **1**, com `destino` na posição do PNJ, reaproveitando `DANO_ATAQUE = 10`, `ponto_igual`, `ALCANCE_CONTATO_M = 2.0` e a dedup de um ataque por atacante por tick.
- Fazer o defensor revidar **8** a cada **90 ticks** (3 s a 30 Hz, `[HIPÓTESE]`) ao jogador dentro de **2 m**, em ordem estável de `id`, usando o tick determinístico da sim — sem relógio de parede.
- Refletir `Mundo.pnjs` (incluindo `pv` e `ativo`) em `hash_estado`, em posição fixa e independente de formatação de texto.
- Exibir no cliente `PNJs: N` ativos e introduzir a tecla `T` para agendar `atacar_pnj` com `destino` na posição do PNJ, reaproveitando `seq`/`request_id` do fluxo existente.
- Preservar a compatibilidade: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, dependências, `async`, sockets ou Lightyear; estilo `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`.

## Non-Goals

- Novas habilidades W/E/R do GDD §10.2–§10.5 — Oficina de Campo, Análise Materialista, Plano Mestre, Fuga pelas Sombras, Espalhar Rumores, Sabotagem Estrutural etc. O contrato e o `STATE.md` as empurram para o próximo ciclo; `T` é apenas o gatilho de `atacar_pnj`, não uma habilidade nova.
- Escalonamento de defensores por aliado (4 → 8) ou a variante de estresse `×5` da Rota Superior, seções §8.1:357,363; mapa, ilhas, spawn por ilha, navegação, tabelas de aggro, rotas e penalidade de gás lacrimogêneo.
- Rebalanceamento de dano, custo, PV, Fôlego, alcance, recarga ou duração das ações consolidadas dos ciclos 015/045/046/047/048/050/052 — os números são reaproveitados, não renegociados.
- Reabrir papéis, níveis, `TICKS_POR_NIVEL = 900`, `NIVEL_MAXIMO = 12` ou o mapa Q1/W2/E3/R6 do ciclo 052; `atacar_pnj` é nível 1 e usa a infraestrutura existente como dada.
- Promover 24 PV, 8 dmg, 90 ticks, 2 m ou o dano 10 a cânone: todos permanecem **`[HIPÓTESE]`/protótipo**.
- Netcode, lobby, matchmaking, predição/reconciliação, `async`, sockets, Lightyear (ADR-004) e qualquer alteração de protocolo ou contrato.
- Adotar R6, simular emissão de issues ou reabrir o empate 48/625 com novas habilidades W/E/R.

## Success Metrics

- Um PNJ registrado com PV inicial **24** começa `ativo = true` e é refletido em `Mundo.pnjs` e em `hash_estado` (baseline: estado inexistente — **[NV]** até verificação).
- `atacar_pnj` é aceito no **nível 1** e, com `destino` na posição do PNJ e jogador a ≤ 2 m, reduz o PV de **24 para 14** (dano **10**); fora de alcance não altera PV nem gasta efeito (baseline: **[NV]**).
- Dano ≤ 0, `id` inexistente e PNJ inativo não mudam estado; PV nunca fica negativo e, ao chegar a zero, `ativo = false` — a entrada permanece no mapa com o `id` auditável (baseline: **[NV]**).
- O defensor ataca **8** a cada **90 ticks** ao jogador a ≤ 2 m: o tick 89 não dispara, o 90.º dispara uma vez; fora de 2 m não há dano ao jogador (baseline: **[NV]**).
- Um único ataque do jogador por atacante por tick e um único contra-ataque por PNJ por janela de 90 ticks; `hash_estado` muda com `pv`/`ativo` e é idêntico para estados idênticos (baseline: **[NV]**).
- A HUD exibe `PNJs: N` com a contagem de ativos e a tecla `T` agenda `atacar_pnj` com `destino` na posição do PNJ; nenhuma habilidade W/E/R nova, nenhum campo de protocolo novo (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, o PVE do GDD segue sem tradução em estado: o jogador tem `atacar_forte` e um alvo de treino não-canônico no cliente, mas nenhum adversário com PV que reaja, nenhum dano devolvido e nenhuma contagem de defensores ativos. Qualquer ciclo de ilhas, patrulhas ou desafios que queira "4 forças de segurança" ou "24 pontos de dano" precisaria criar entidade, API de dano, relógio, hash e HUD ao mesmo tempo — junto com mapa e escalonamento —, perdendo a oportunidade de validar a oposição em isolamento. O custo é a ausência da única forma escrita no GDD de tornar a pressão do ambiente verificável; não é a ausência das habilidades completas W/E/R.

## Open Questions

- R3 = 0 clarificações de design pendentes: o contrato fixou campos, constantes, ação, HUD e tecla; o que segue é validação de especificação, não renegociação.
- **[NV]** Confirmar no `/speckit.specify` se `Pnj` guarda `posicao` como `(f64, f64)` (o tipo de `Jogador.posicao` e do `Comando.destino`) e se `adicionar_pnj` recebe `(id, posicao)` com PV inicial implícito de 24.
- **[NV]** Confirmar se a janela de contra-ataque é contada por PNJ (`Pnj` com campo de ticks) ou por `tick % 90` global; a primeira opção é a recomendação por permitir defensores registrados em ticks diferentes.
- **[NV]** Confirmar a posição de `Mundo.pnjs` no `hash_estado` e a ordem de `atacar_pnj` entre as validações de `ValidacaoComandos`.
- **[NV]** Confirmar se a tecla `T` deve escolher o PNJ mais próximo, o primeiro por `id` ou usar uma posição fixa de destino — a especificação decide; o contrato só exige "`destino` na posição do PNJ".
- **[NV]** Conformidade de R4/R7 (`1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** Comportamento de dano, inatividade, relógio e HUD precisa de verificação em tempo de execução pelo orquestrador.
- Fora de escopo por decisão, não lacuna: habilidades completas W/E/R, escalonamento 4→8/×5, mapa/ilhas e o alvo de treino do ciclo 048 — todos com outro rótulo ou outra frente.

## Rastreabilidade (escopo do problema)

- FR-001 → `Pnj { id, posicao, pv, ativo }` e `Mundo.pnjs: BTreeMap<u64, Pnj>`; FR-002 → `adicionar_pnj` com PV inicial 24 `[HIPÓTESE]`; FR-003 → `aplicar_dano_a_pnj` (guardas, clamp e `ativo = false`) com dano do jogador 10; FR-004 → `atacar_pnj` nível 1 em `ACOES_PERMITIDAS`, `destino` na posição do PNJ, alcance 2 m e dedup; FR-005 → contra-ataque de 8 a cada 90 ticks a jogador em 2 m, integrado a `avancar_tick` e `hash_estado`; FR-006 → HUD `PNJs: N` ativos e tecla `T`, sem novas habilidades W/E/R, contrato, protocolo, Cargo, stack ou rede.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis do handoff em `decision.md`, com definição canônica prevista em `specs/053-pnj-defensores/` (hoje em template).
- Todos os números (24, 8, 90, 2 m, 10) são `[HIPÓTESE]`/protótipo. **R6 é N/A** por constituição v1.2.2 e não entra neste rastro.
