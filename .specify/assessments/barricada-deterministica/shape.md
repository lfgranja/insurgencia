# Shape — Barricada determinística de materiais

- **Slug**: `barricada-deterministica`
- **Created**: 2026-11-23T02:43Z
- **Recommended option**: Opção A — Barricada determinística

## Options

### Option A — Barricada determinística (recomendada)

- **Sketch**: o jogador pressiona `B` no ponto onde está; a ação só começa se houver 20 Materiais, e a canalização de 1,5 s termina em 45 ticks com uma parede de 200 PV que bloqueia passagem e projéteis por 12 s. Mover-se cancela; nova barricada além de duas remove a mais antiga; o estado resultante é integralmente coberto pelo hash. Jogador enxerga Materiais, Barreiras e progresso de Construindo no HUD.
- **Appetite**: `small` (dias) — fatia limitada ao Núcleo Rust e ao cliente/HUD, sem contrato, protocolo ou rede novos.
- **Trade-offs**: ganha a primeira habilidade verificável, determinismo auditável e fidelidade aos valores canônicos; sacrifica Fôlego/alcance/recarga/dimensões (hipóteses) e qualquer variação de ponte. Risco principal é a disciplina de tick (uma canalização por jogador por tick) e a ordem estável do hash — ambos cobertos por contrato explícito.
- **Rabbit holes**: refatorar `fase_skills` para múltiplas canalizações simultâneas; tentar modelar colisão/passagem completa de projéteis (fora do recorte); reabrir o rótulo `[HIPÓTESE]` de §16.3/H4; tocar em `crates/contratos` ou `PROTOCOL_VERSION` para "acomodar" o comando.

### Option B — Adiar para netcode

- **Sketch**: postergar a habilidade até que sincronização cliente-servidor esteja pronta, implementando primeiro transporte, predição e reconciliação.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de uma fatia de Núcleo para infraestrutura de rede.
- **Trade-offs**: ganha base de rede antecipada; sacrifica qualquer fatia jogável e verificável no curto prazo. Risco: sincronizar um estado que ainda não existe deterministicamente; GDD §16.3:1490 pede Núcleo isolado, não rede primeiro.
- **Rabbit holes**: lobby/matchmaking (explicitamente fora do MVP), predição de canalização e resolução de conflitos de concorrência de comandos.

### Option C — Ponte provisória

- **Sketch**: implementar primeiro a variante de 15 Materiais usada em emboscadas na Rota Inferior/Delta do Comércio, reaproveitando o caminho de canalização.
- **Appetite**: `small` (dias), mas com valor de design menor.
- **Trade-offs**: ganha reuso de código com a barricada; sacrifica o lastro canônico — GDD §10.3:574 marca a ponte como `[SUPOSIÇÃO]` e §9.3:475 só lhe dá o custo de 15. Risco: produto com números não canônicos e sem critérios SC próprios.
- **Rabbit holes**: tratamento de travessia de ilhas e regras de emboscada, que puxam mapa e IA para dentro do recorte.

### Option D — Combatar primeiro

- **Sketch**: investir a fatia em combate (Ataque Forte do Guerreiro, tiros, dano), deixando a construção para depois.
- **Appetite**: `medium` (semanas).
- **Trade-offs**: ganha feedback imediato de ação; sacrifica a tese do MVP. GDD §16.3:1509 é explícito: provar jogabilidade baseada em desafios de ilha, "em vez de combate puro". Risco: invalidar a premissa que o projeto quer testar.
- **Rabbit holes**: balanceamento de armas, recarga e IA — nenhum com valor canônico para a barricada.

## Recommendation

Escolher **Opção A**. É a única que satisfaz simultaneamente as metas de `define.md`: valores canônicos fechados (confiança 5), efeito verificável por `hash_estado` (impacto 5) e fronteira pequena sem contrato/protocolo novos (risco 4). B inverte a dependência determinismo→rede, C carece de lastro canônico e D contradiz o objetivo do MVP. 80/20: **400/625** contra 108/625 (C), 48/625 (D) e 10/625 (B).

## Out of Scope (for the recommended option)

- Fôlego, alcance, recarga e dimensões; ponte provisória e demais habilidades.
- Netcode, lobby, matchmaking, combate e IA.
- Reabertura do conflito §16.3/H4; adoção de R6; emissão simulada de issues.
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` e qualquer código-fonte neste assessment.

## Assumptions to Validate

- A conversão 30 ticks/s (1,5 s = 45 ticks; 12 s = 360 ticks) é a do relógio corrente do Núcleo — confirmar no `/speckit.specify`.
- `Comando.destino` comporta o ponto de construção sem estender o contrato de protocolo.
- `ponto_igual` é a comparação canônica de posição e tolera a rodada de floats do cliente.
- Inserir/remover barreiras em `BTreeMap<u64, Barreira>` com `proxima_barreira_id` monotônico preserva a ordem do hash entre clientes.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; alternativas B/C/D são as mesmas rejeitadas em `research.md` e não recebem rastro de implementação. T001..T014 pertencem ao handoff em `decision.md`, com definição canônica em `specs/040-barricada-deterministica/`.
