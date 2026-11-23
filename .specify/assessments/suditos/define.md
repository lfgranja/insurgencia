# Define — Súditos e Propaganda

- **Slug**: `suditos`
- **Created**: 2026-11-23T06:58Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §9.2:457-467, STATE.md ciclo 046, constituição v1.2.2)

## Problem Statement

Sem conversão determinística de Súditos no Núcleo, a única regra de aceitação escrita para eles (GDD §9.2:464 — 4 s de Propaganda interrompem o ataque a estruturas e iniciam panfletagem; dano de jogador reverte em 3 s) permanece apenas declarada: não há estado observável que diga há quanto tempo um Súdito foi exposto, se ele está convertido, nem há quantos ticks a reversão será concluída. Jogador e Núcleo não compartilham uma verdade verificável sobre a influência ideológica, e o pilar Materialismo Histórico / Ação Coletiva (§9.2:469) fica restrito à prosa de design — sem depender de spawn, ilhas ou funções ainda `[HIPÓTESE]`.

## Affected Users & Stakeholders

- **Users**: jogadores da Comuna (Intelectual) — precisam que a exposição à Propaganda produza um efeito previsível sobre Súditos inimigos (GDD §9.2:459,464); jogadores que atacam — precisam que o dano reverta a conversão em tempo conhecido (§9.2:464).
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de determinismo por tick e `hash_estado` estável; SWE/orquestrador — precisa de uma fatia testável sem mudança de contrato, protocolo ou stack; designer — precisa que os valores canônicos não sejam reabertos e que as hipóteses continuem rotuladas.

## Goals

- Tornar observável a sequência completa: injeção determinística (`adicionar_sudito(id, posicao)`) → exposição via `propaganda` → **120 ticks** → `convertido = true` (para de atacar estruturas, passa a panfletar) → dano (`aplicar_dano_a_sudito(id)`) → **90 ticks** → `convertido = false`.
- Preservar os valores canônicos 4 s = 120 ticks e 3 s = 90 ticks sem renegociação, com a conversão de tempo fixada no contrato da sessão.
- Garantir disciplina de tick: no máximo **um** avanço de `propaganda_ticks` ou `reversao_ticks` por súdito por tick, usando `BTreeSet<u64>` no comando de auto-avanço, sem duplicatas.
- Refletir integralmente `id`, `posicao`, `convertido`, `propaganda_ticks` e `reversao_ticks` de todos os súditos em `hash_estado`, em ordem estável de `BTreeMap`.
- Manter compatibilidade: nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`; nenhum `async`, rede ou Lightyear.
- Registrar explicitamente como fora de escopo tudo o que o GDD marca `[HIPÓTESE]`: spawn por pulso de 30 s, raio de 6 m, funções trabalhador/propagandista/miliciano, limite de 3 Súditos, permanência de 60 s e renda do trabalhador.

## Non-Goals

- Spawn automático por pulso, contagem de Súditos por ilha, Rota Central, posse/conversão de ilhas e mapa — `[HIPÓTESE]` em GDD §9.2:461.
- Raio de área de 6 m da Propaganda e limite de 3 Súditos por aplicação — `[HIPÓTESE]` em §9.2:461; a exposição é informada por ids no comando, não por geometria.
- Funções trabalhador/propagandista/miliciano, conservação de função, renda por 60 s e permanência de 60 s — `[HIPÓTESE]` em §9.2:461,463-467.
- Conversão de Nós, Resistência Ideológica e Propaganda canalizada de 20 s — `[HIPÓTESE]` em §6.2:275; fora do recorte desde o ciclo 044.
- IA de Súditos, ataque real a estruturas, projéteis, colisão, dano de armas, Fôlego/alcance/recarga/dimensões (§10.3:574-579) e combate em geral.
- Netcode, lobby, matchmaking, predição/reconciliação, `async`, sockets ou Lightyear (ADR-004).
- Reabrir o empate 48/625 com Combate/Fôlego, adotar R6 ou simular emissão de issues.

## Success Metrics

- Aos **120 ticks** de exposição ininterrupta, `convertido` passa a `true` e o súdito para de atacar estruturas / passa a panfletar (baseline: comportamento inexistente — **[NV]** até execução).
- **119 ticks** não convertem; a conversão ocorre exatamente no tick em que 120 é atingido, sem conversão antecipada (baseline: **[NV]**).
- `aplicar_dano_a_sudito(id)` inicia a reversão: aos **90 ticks** `convertido` volta a `false` e o súdito retoma o ataque a estruturas (baseline: **[NV]**).
- **89 ticks** não revertem; a reversão ocorre exatamente no tick em que 90 é atingido (baseline: **[NV]**).
- No máximo **1** avanço de contador por súdito por tick, com `BTreeSet<u64>` sem ids duplicados; todos os campos aparecem em `hash_estado` na ordem de `BTreeMap` (baseline: **[NV]**).
- Cliente agenda `propaganda` com tecla `G` just-pressed e `seq`/`request_id` monotônicos via recurso local; HUD exibe a contagem de convertidos (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, a regra §9.2:464 segue sem recipiente observável: não há contador de exposição, estado de conversão nem relógio de reversão no `Mundo`. Qualquer ciclo futuro que dependa de "Súdito convertido" (Nós, Resistência Ideológica, renda do trabalhador) precisaria reconstruir o determinismo desde zero, e a tese do GDD de que a influência avança pelo trabalho e pela persuasão (§9.2:469) continuaria sem verificação possível no Núcleo. O custo é a perda da única parte de Súditos ancorada como critério de aceitação — não a ausência de spawn.

## Open Questions

- Nenhuma clarificação de design pendente: a regra é §9.2:464 e o contrato fixa API e conversão de ticks (R3 = 0 clarificações).
- **[NV]** Confirmar no `/speckit.specify` que o relógio corrente é 30 ticks/s e que `propaganda`/`BTreeSet<u64>` cabem em `ACOES_PERMITIDAS` sem tocar em contratos/protocolo.
- **[NV]** Conformidade de R4/R7 (`1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** Comportamento de `hash_estado` e da tabela de súditos precisa de verificação em tempo de execução pelo orquestrador.
- Fora de escopo por decisão, não lacuna: spawn/raio/funções e demais `[HIPÓTESE]` listadas em `Non-Goals`.

## Rastreabilidade (escopo do problema)

- FR-001 → constantes 120/90; FR-002 → struct `Sudito` e `BTreeMap` no `Mundo`; FR-003 → `adicionar_sudito` e `aplicar_dano_a_sudito`; FR-004 → ação `propaganda` e auto-avanço com `BTreeSet<u64>`; FR-005 → cobertura em `hash_estado`; FR-006 → tecla `G` e HUD de convertidos.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis do handoff em `decision.md`, com definição canônica prevista em `specs/046-suditos/` (hoje em template).
- R6 é **N/A** por constituição v1.2.2 e não entra neste rastro.
