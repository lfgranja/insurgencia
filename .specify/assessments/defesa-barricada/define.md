# Define — Defesa de barricada (autogestão)

- **Slug**: `defesa-barricada`
- **Created**: 2026-11-23T05:47Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §6.2/§7.1/§10.3, STATE.md:304-315, constituição v1.2.2, ADR-004)

## Problem Statement

A barricada determinística entregue no ciclo 040 (200 PV, 360 ticks, coberta por `hash_estado`) não rende hoje nenhum efeito observável: o GDD §7.1:306 fixa a recompensa canônica — **+1 ponto de autogestão a cada 10 s de barricada intacta** —, mas nada no Núcleo mede o tempo sob defesa nem credita o jogador. Sem isso, a condição de vitória "autogestão, não demolição" (§6.2:273) e a fase Defender do loop de 30 s permanecem apenas declaradas no documento de design: não há verdade observável sobre quanto tempo a barricada foi defendida, se o defensor estava no ponto, ou se o acúmulo foi perdido sem crédito. Materialismo Histórico e Ação Coletiva ficam sem fechamento mecânico entre a obra (a barricada) e o objetivo (a autogestão).

## Affected Users & Stakeholders

- **Users**: jogadores que defenderam a barricada — precisam ver o tempo de posse convertido em pontos de autogestão no HUD (GDD §7.1:306); equipe — precisa de um retorno visível pelo ato de "segurar o ponto", distinto de perseguir a emboscada.
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de determinismo por tick e `hash_estado` estável também para os novos campos; SWE/orquestrador — precisa de uma fatia testável sem mudança de protocolo, dependência ou stack; designer — precisa que a regra canônica de 10 s e a simplificação dono-defensor não sejam reabertas.

## Goals

- Medir 300 ticks (10 s a 1/30 s) de barricada intacta com o dono no mesmo ponto e creditar **+1** em `Jogador.pontos_autogestao`, zerando o contador ao creditar.
- Zerar `Barreira.defesa_ticks` **sem crédito** se o dono sai do ponto, se a barricada é destruída (`pv == 0`) ou se expira (`ticks_restantes == 0`).
- Manter determinismo: no máximo um incremento de `defesa_ticks` por barricada por tick, em fase própria, com ordem estável no hash.
- Refletir integralmente `pontos_autogestao` e `defesa_ticks` em `hash_estado`.
- Exibir `Autogestão` no HUD como retorno observável ao jogador.
- Preservar compatibilidade: nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`; nenhum Lightyear, async ou sockets.

## Non-Goals

- Papéis distintos Guerreiro/Artesão na defesa — simplificação registrada: o dono no mesmo ponto é o defensor (STATE.md:315).
- Condição de vitória completa: 3 Nós de Controle, Resistência Ideológica, Propaganda de 20 s e Súditos — `[HIPÓTESE]` em GDD §6.2:275.
- Ponte provisória, Oficina de Campo, Linha de Defesa e demais habilidades; Fôlego/alcance/recarga/dimensões da barricada (`[HIPÓTESE]`).
- Netcode, lobby, matchmaking, predição ou reconciliação — rede bloqueada por ADR-004 até decisão da SE.
- Combate, IA de inimigos, dano a armas e física de projéteis além do estado `pv` da barricada 040.
- Adotar R6 neste ciclo: **R6 N/A** por constituição v1.2.2, sem issue simulada nem `gh` improvisado.

## Success Metrics

- A cada **300 ticks** de barricada intacta com dono no mesmo ponto, `pontos_autogestao` aumenta exatamente **+1** e `defesa_ticks` volta a **0** (baseline: comportamento inexistente — **[NV]** até execução).
- Aos **299 ticks** o crédito é **0**; não existe acúmulo parcial de pontos (baseline: **[NV]**).
- Sair do ponto zera `defesa_ticks` no mesmo tick em que a condição falha, sem crédito (baseline: **[NV]**).
- Destruição (`pv == 0`) ou expiração (`ticks_restantes == 0`) zera `defesa_ticks` sem crédito (baseline: **[NV]**).
- No máximo **1** incremento de `defesa_ticks` por barricada por tick; `pontos_autogestao` e todos os `defesa_ticks` aparecem em `hash_estado` em ordem estável (baseline: **[NV]**).
- HUD exibe `Autogestão` com o total creditado; em 360 ticks de vida da barricada, no máximo **1** crédito (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, o ciclo 040 entrega uma barricada que não retroalimenta a condição de vitória: o loop de 30 s tem fase Defender sem recompensa mecânica, a tese "autogestão, não demolição" segue apenas documental e não há superfície determinística para validar Núcleo + Módulo antes de qualquer trabalho de rede. O custo é a perda do fechamento entre a obra coletiva e o objetivo — não apenas a ausência de um contador no HUD.

## Open Questions

- Nenhuma clarificação de design: os 10 s são canônicos em GDD §7.1:306 e a conversão para 300 ticks é determinística (R3 = 0 clarificações).
- **[NV]** Conformidade de R4/R7 depende de gates não executados neste assessment; **R6 N/A** por v1.2.2.
- **[NV]** O comportamento do `hash_estado` com os novos campos e a ordem de iteração das barricadas precisam de verificação em tempo de execução pelo orquestrador.
- Registrado, não reaberto: "tempo alvo 8 s" na tabela §7.1:306 é pacing; a regra de recompensa de 10 s prevalece.

## Rastreabilidade (escopo do problema)

- FR-001 → constantes canônicas (300 ticks; reuso de 200 PV / 360 ticks); FR-002 → campos `pontos_autogestao` e `defesa_ticks`; FR-003 → condição de barricada intacta e dono no mesmo ponto; FR-004 → fase determinística de incremento e crédito; FR-005 → zeração sem crédito; FR-006 → HUD `Autogestão` e fronteira de compatibilidade.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis especificados em `specs/044-defesa-barricada/`, fora desta fronteira.
