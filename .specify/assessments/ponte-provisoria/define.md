# Define — Ponte provisória determinística

- **Slug**: `ponte-provisoria`
- **Created**: 2026-11-23T06:25Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §8.4/§9.3/§10.3, STATE.md:317-328, constituição v1.2.2, ADR-004)

## Problem Statement

A ponte provisória tem regras canônicas fechadas no GDD — **15 Materiais** (§9.3:475,483), **60 PV** e **45 s de duração** (§8.4:407) — mas não existe hoje como estado determinístico observável no Núcleo Rust. Não há ação `construir_ponte`, não há débito atômico de Materiais com recusa sem consumo parcial, não há objeto com 60 PV que expire ao completar 1350 ticks a 1/30 s, e o HUD não mostra quantas Pontes estão ativas. Sem isso, a travessia do Delta-3 (Ilha da Barca) e o uso da ponte em emboscadas permanecem apenas declarados no documento de design (§10.3:574 os marca como `[SUPOSIÇÃO]`), sem verdade observável sobre custo, posse, integridade ou expiração. Coleta de Materiais e construção seguem sem fechamento mecânico entre recurso gasto e objeto temporário entregue.

## Affected Users & Stakeholders

- **Users**: jogadores que coletam Materiais e constroem — precisam ver a ação `construir_ponte` debitar exatamente 15 Materiais, a ponter aparecer com 60 PV e o contador do HUD refletir as Pontes ativas; equipe — precisa de um retorno visível pelo ato de construir, com recusa clara quando não há saldo.
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de determinismo por tick e `hash_estado` estável também para `Ponte`; SWE/orquestrador — precisa de uma fatia testável sem mudança de protocolo, dependência ou stack; designer — precisa que 15 Materiais / 60 PV / 45 s não sejam renegociados, e que o atalho de 8 s e os 45 ticks de construção não sejam confundidos com regras canônicas.

## Goals

- Introduzir `construir_ponte` em `ACOES_PERMITIDAS` da sim, sem alterar `crates/contratos` nem `PROTOCOL_VERSION`.
- Fixar as constantes `CUSTO_PONTE_MATERIAIS = 15`, `PV_PONTE = 60`, `TICKS_DURACAO_PONTE = 1350` (45 s a 1/30 s) e `TICKS_CONSTRUCAO_PONTE = 45` (escolha de protótipo, não valor canônico).
- Representar a ponte como struct pública `Ponte { id, dono, posicao, pv, ticks_restantes }` em `BTreeMap<u64, Ponte>` em `Mundo`.
- Validar `destino == posicao` e saldo ≥ 15 Materiais; debitar 15 de forma **atômica**, com recusa **sem consumo parcial** quando não há saldo.
- Aplicar 60 PV iniciais e expiração determinística por `ticks_restantes` (1350 ticks), removendo a ponte também quando `pv == 0`.
- Refletir integralmente todas as Pontes em `hash_estado`, em ordem estável.
- Exibir contagem de Pontes no HUD e associar a tecla `P` à ação de construção.
- Preservar as regras Rust: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`; sem async, rede ou Lightyear.

## Non-Goals

- **Atalho de 8 s entre ilhas**: depende de mapa/ilhas (§8.4:407); **fora de escopo**, registrado como limitação.
- **Tempo de construção como regra canônica**: os 45 ticks são escolha de protótipo, não valor do GDD; não promover, não renegociar.
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`; Lightyear, async, sockets, netcode, lobby ou matchmaking — rede bloqueada por ADR-004 até decisão da SE.
- Súditos, Nós de Controle, Resistência Ideológica, Propaganda e condição de vitória completa — `[HIPÓTESE]` em GDD §6.2:275.
- Combate, IA de inimigos, dano a armas, física de projéteis, Fôlego/alcance/recarga/dimensões (`[HIPÓTESE]`) e os papéis de emboscada do Delta — fora do recorte.
- Mapa, ilhas Delta-1/Delta-2, rotas comerciais e travessia — o atalho de 8 s não é implementado.
- Adotar R6 neste ciclo: **R6 N/A** por constituição v1.2.2, sem issue simulada nem `gh` improvisado.

## Success Metrics

- **SC-001**: `construir_ponte` com `destino == posicao` e saldo ≥ 15 debita **exatamente 15 Materiais** e cria uma `Ponte` com **60 PV** e **1350 ticks_restantes** (baseline: comportamento inexistente — **[NV]** até execução).
- **SC-002**: com saldo inferior a 15, a ação é **recusada**, **nenhum** Material é consumido (sem consumo parcial) e nenhuma `Ponte` é criada (baseline: **[NV]**).
- **SC-003**: com `destino != posicao`, a ação é **recusada sem débito** e sem criação de Ponte (baseline: **[NV]**).
- **SC-004**: a Ponte desaparece do `BTreeMap` quando `pv == 0` ou quando `ticks_restantes` chega a 0 após 1350 ticks, sem duplo evento de remoção (baseline: **[NV]**).
- **SC-005**: todas as Pontes aparecem em `hash_estado` em ordem estável; no máximo **1** tick de construção e **1** tick de expiração por ponte, sem duplo débito (baseline: **[NV]**).
- **SC-006**: o HUD exibe a **contagem de Pontes** ativas e a tecla `P` dispara `construir_ponte`, sem tocar em `crates/contratos`, `PROTOCOL_VERSION` ou rede (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, o hub tem um recurso com preço, PV e duração já canônicos no GDD que nenhum sistema do Núcleo materializa: o jogador não pode gastar 15 Materiais numa ponte, não vê 60 PV nem contagem, e não há superfície determinística para validar construção, débito atômico e expiração antes de qualquer trabalho de mapa, emboscada ou rede. O custo é a perda do fechamento entre coleta de Materiais e objeto temporário — não apenas a ausência de um contador no HUD.

## Open Questions

- Nenhuma clarificação de design: 15 Materiais, 60 PV e 45 s são canônicos (GDD §9.3:475,483; §8.4:407) e a conversão para 1350 ticks é determinística (R3 = 0 clarificações).
- **[NV]** Conformidade de R4/R7 depende de gates não executados neste assessment; **R4 antes de R7 (`1.0.0-local`)**; **R6 N/A** por v1.2.2.
- **[NV]** O comportamento do `hash_estado` com `Ponte` em `BTreeMap<u64, Ponte>`, a ordem de iteração e a validação `destino == posicao` precisam de verificação em tempo de execução pelo orquestrador.
- Registrados, não reabertos: **atalho de 8 s fora de escopo** (limitação de mapa/ilhas) e **45 ticks de construção como escolha de protótipo**, não valor canônico.

## Rastreabilidade (escopo do problema)

- FR-001 → constantes canônicas (15 Materiais, 60 PV, 1350 ticks; 45 ticks de construção como protótipo); FR-002 → ação `construir_ponte` em `ACOES_PERMITIDAS`; FR-003 → struct `Ponte` em `BTreeMap<u64, Ponte>` em `Mundo`; FR-004 → validação `destino == posicao`, saldo e débito atômico sem consumo parcial; FR-005 → expiração determinística, remoção por `pv == 0` e cobertura em `hash_estado`; FR-006 → HUD com contagem de Pontes e tecla `P`, fronteira de compatibilidade.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis especificados em `specs/045-ponte-provisoria/`, fora desta fronteira; **R6 N/A** por constituição v1.2.2, sem tarefa.
