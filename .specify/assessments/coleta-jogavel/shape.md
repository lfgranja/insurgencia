# Shape — Coleta jogável no cliente

- **Slug**: `coleta-jogavel`
- **Created**: 2026-11-23T03:54Z
- **Recommended option**: Opção A — Coleta jogável no cliente

## Options

### Option A — Coleta jogável no cliente (recomendada)

- **Sketch**: um comando válido `coletar_materiais` fixa o ponto e inicia o auto-avanço: a coleta progride 1 tick por jogador a cada tick seguinte enquanto o jogador permanecer no mesmo ponto, e credita +3 Suprimentos e +2 Materiais ao completar 180 ticks (6 s a 1/30 s). `mover` para posição diferente zera o parcial; comando e auto-avanço no mesmo tick não contam duas vezes (`BTreeSet<u64>`, padrão do ciclo 040). No cliente, `C` em just pressed agenda o comando com `destino` = posição atual e ids monotônicos; o HUD mostra Suprimentos e progresso de Coleta ao lado de Materiais/Barreiras/Construindo; o doc comment de `Comando.destino` passa a citar `mover`, `coletar_materiais` e `construir_barreira`.
- **Appetite**: `small` (dias) — uma rotina de auto-avanço espelhando `avancar_construcoes_automaticas`, uma entrada de teclado, dois elementos de HUD e um doc comment; sem contrato, protocolo ou rede novos.
- **Trade-offs**: ganha a fatia 80/20 com valor canônico fechado, verificável por `hash_estado` e jogável de ponta a ponta; sacrifica apenas o escopo de "primeiro implementar rede/ponte". Risco principal é a disciplina de tick (um avanço por jogador por tick, sem dupla contagem) e a ordem estável do hash — ambos cobertos pelo padrão já existente no Núcleo.
- **Rabbit holes**: refatorar `fase_skills` para múltiplos pontos de coleta simultâneos por jogador; criar entidade "ponto neutro" no contrato; transformar a tecla `C` em sistema de ações genérico; reabrir os números de GDD §7.1; tocar em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml` para "acomodar" o comando.

### Option B — Só corrigir o doc comment de `destino`

- **Sketch**: entregar apenas a correção documental de `Comando.destino`, citando `mover`, `coletar_materiais` e `construir_barreira`, sem mexer na jogabilidade.
- **Appetite**: `xs` (horas) — uma linha de doc comment.
- **Trade-offs**: ganha exatidão documental e risco mínimo (fator 5); sacrifica todo o impacto (fator 2) — a regra canônica continua avançando só com comando por tick e sem entrada/HUD. É uma condição inclusa na Opção A, não uma entrega que fecha o ciclo.
- **Rabbit holes**: transformar a correção em auditoria documental ampla do contrato; mexer na semântica de `destino` além do comentário.

### Option C — Ponte provisória

- **Sketch**: implementar primeiro a variante de 15 Materiais usada em emboscadas, reaproveitando o caminho de canalização do ciclo 040.
- **Appetite**: `small` (dias), mas com valor de design menor.
- **Trade-offs**: ganha reuso de código com a barricada; sacrifica o lastro canônico — GDD §10.3:574 marca a ponte como `[SUPOSIÇÃO]` e §9.3:475 só lhe dá o custo de 15, sem critérios SC próprios. Risco: produto com números não canônicos e fora da Fase 1 do pulso.
- **Rabbit holes**: travessia de ilhas, regras de emboscada e mapa/IA entrando no recorte.

### Option D — Servidor + Lightyear (netcode)

- **Sketch**: postergar a jogabilidade até transporte, predição e reconciliação cliente-servidor estarem prontos.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de fatia de Núcleo/cliente para infraestrutura de rede.
- **Trade-offs**: ganha base de rede antecipada; sacrifica qualquer fatia jogável no curto prazo. Risco máximo: sincronizar um estado que ainda não tem auto-avanço determinístico nem entrada; GDD §16.3:1490 pede Núcleo isolado, não rede primeiro.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), predição de progresso de coleta e resolução de concorrência de comandos.

## Recommendation

Escolher **Opção A**. É a única que satisfaz simultaneamente as metas de `define.md`: regra canônica fechada (confiança 5), efeito verificável por `hash_estado` (impacto 5), fronteira pequena sem contrato/protocolo novos (facilidade 5) e risco controlado pelo padrão `BTreeSet` já auditado (fator 4). B é necessária, mas de impacto 2 e insuficiente sozinha; C não tem lastro canônico; D inverte a dependência determinismo→jogabilidade→rede. Scorecard único do ciclo: **500/625** contra 250/625 (B), 108/625 (C) e 10/625 (D).

## Out of Scope (for the recommended option)

- Reabertura da regra §7.1, ponto neutro como entidade de contrato, ponte, barricada, combate, emboscada, dano e IA.
- Netcode, lobby, matchmaking, predição, reconciliação e servidor Lightyear.
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` e qualquer código-fonte neste assessment.
- R6 (N/A por constituição v1.2.2), issues simuladas e `gh` improvisado; gates, build, testes e formatadores.

## Assumptions to Validate

- O relógio corrente do Núcleo é 30 Hz (`PASSO_SEGUNDOS = 1/30`), logo 6 s = 180 ticks — confirmado em `crates/sim/src/lib.rs`, a validar na especificação.
- `avancar_coletas_automaticas` deve espelhar `avancar_construcoes_automaticas`: varredura de `jogadores` em ordem de `BTreeMap`, cancels por ponto/saldo, `BTreeSet<u64>` local e crédito atômico ao atingir `TICKS_COLETA`.
- `mover` para posição diferente já zera `coleta_ticks`/`coleta_ponto`; o auto-avanço não deve reintroduzir o acúmulo após o cancelamento no mesmo tick.
- `Comando.destino` comporta a posição atual do cliente sem extensão de protocolo, e `seq`/`request_id` monotônicos são gerados pelo recurso local já previsto no contrato.
- A ordem de serialização de `hash_estado` permanece estável entre clientes.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; alternativas B/C/D são as mesmas rejeitadas em `research.md` e não recebem rastro de implementação. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica em `specs/041-coleta-jogavel/`. R6 = N/A (constituição v1.2.2).
