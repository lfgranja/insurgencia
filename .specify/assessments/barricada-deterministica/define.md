# Define — Barricada determinística de materiais

- **Slug**: `barricada-deterministica`
- **Created**: 2026-11-23T02:43Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §9.3/§10.3/§16.3/§20.2)

## Problem Statement

Sem uma construção de barricada determinística no Núcleo, a primeira habilidade ativa do Artesão não existe como mecânica verificável: não há como provar, no estado simulado, que 20 Materiais foram debitados uma única vez, que a canalização de 1,5 s respeita o relógio de ticks, ou que a parede de 200 PV desaparece após 12 s. Jogador e servidor não compartilham uma verdade observável sobre a ação, e o pilar Materialismo Histórico (a economia precede e condiciona a ação militar, GDD §9.3:485) fica apenas declarado no documento de design.

## Affected Users & Stakeholders

- **Users**: jogadores na função Artesão — precisam converter Materiais coletados em controle temporário de corredor (GDD §10.3:574); jogadores das demais funções — precisam de uma parede previsível para proteger canalizações (GDD §8.5/§16.3).
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de determinismo e `hash_estado` estável; SWE/orquestrador — precisa de uma fatia testável sem mudança de protocolo; designer — precisa que os valores canônicos não sejam reabertos.

## Goals

- Tornar observável a sequência completa: posição do jogador → canalização → débito atômico → barreira → expiração.
- Preservar os valores canônicos 20 Materiais / 45 ticks / 200 PV / 360 ticks / máx. 2 por jogador sem renegociação.
- Garantir determinismo por tick: no máximo uma canalização avançada por jogador por tick, recusa sem consumo parcial e cancelamento por movimento.
- Refletir integralmente o novo estado em `hash_estado`, em ordem estável.
- Manter compatibilidade: nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`.

## Non-Goals

- Fôlego, alcance, recarga e dimensões da parede — `[HIPÓTESE]` no GDD §10.3:574.
- Ponte provisória, Linha de Defesa, Oficina de Campo e qualquer outra habilidade.
- Netcode, lobby, matchmaking, predição ou reconciliação de cliente.
- Combate, dano de armas, IA de inimigos e física de projéteis além da colisão com a parede já prevista no GDD.
- Reabrir o conflito de rótulo §16.3/H4, adotar R6 ou simular emissão de issues.

## Success Metrics

- Recusa por saldo insuficiente consome **0** Materiais (baseline: comportamento inexistente — **[NV]** até execução).
- Canalização completa em exatamente **45 ticks**, com revalidação de saldo ao completar (baseline: **[NV]**).
- No máximo **1** canalização por jogador por tick; mover o jogador zera `construcao_ticks` e `construcao_ponto` (baseline: **[NV]**).
- Débito de **20** Materiais é atômico; a 2ª barricada força a remoção da mais antiga, mantendo no máximo **2** por jogador (baseline: **[NV]**).
- Cada barreira persiste **360 ticks** com **200 PV** e todas aparecem em `hash_estado` na ordem de `BTreeMap` (baseline: **[NV]**).
- Cliente agenda a ação apenas com tecla `B` just-pressed e `seq`/`request_id` monotônicos; HUD exibe Materiais, Barreiras e Construindo (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, o Artesão segue sem habilidade implementada: o ciclo de Materiais não fecha em ação militar, o desafio "Defender Barricada" (GDD §10.3:574) permanece sem suporte e não há superfície determinística para validar Núcleo + Módulo antes de qualquer trabalho de rede. O custo é a perda da única fatia com valores canônicos prontos para verificação — não apenas a ausência de uma parede.

## Open Questions

- Nenhuma clarificação de design: os valores canônicos estão fechados em §9.3/§10.3 e o contrato fixa a API (R3 = 0 clarificações).
- **[NV]** Conformidade de R4/R7 depende de gates não executados neste assessment.
- **[NV]** O comportamento de `hash_estado` e da tabela de barreiras precisa de verificação em tempo de execução pelo orquestrador.

## Rastreabilidade (escopo do problema)

- FR-001 → meta de constantes/valores canônicos; FR-002 → meta de estado do jogador e recusa; FR-003 → meta de comando e validação; FR-004 → meta de comportamento da fase; FR-005 → meta de determinismo/hash; FR-006 → meta de entrada e HUD.
- SC-001..SC-006 → `Success Metrics` acima; T001..T014 → entregáveis especificados em `specs/040-barricada-deterministica/`, fora desta fronteira.
