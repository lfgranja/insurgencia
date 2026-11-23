# Shape — Defesa de barricada (autogestão)

- **Slug**: `defesa-barricada`
- **Created**: 2026-11-23T05:47Z
- **Recommended option**: Opção A — Defesa determinística de barricada

## Options

### Option A — Defesa determinística de barricada (recomendada)

- **Sketch**: a cada tick, para cada barreira com `pv > 0` e `ticks_restantes > 0` cujo dono está no mesmo ponto (`ponto_igual`), `defesa_ticks` avança 1; ao completar 300 ticks (10 s), o dono recebe +1 em `pontos_autogestao` e o contador zera. Se o dono sai, a barricada é destruída ou expira, o contador zera sem crédito. O HUD mostra `Autogestão`. Nenhuma nova entrada do jogador é necessária — a defesa é derivada da posição já existente.
- **Appetite**: `small` (dias) — dois campos, uma fase determinística e uma linha no HUD, sobre a barricada 040 já pronta; sem contrato, protocolo, dependência ou rede novos.
- **Trade-offs**: ganha a recompensa canônica tornada observável, determinismo auditável por `hash_estado` e continuidade com o ciclo 040; sacrifica os papéis Guerreiro/Artesão (simplificação dono-defensor) e qualquer sistema de Súditos/Nós de Controle. Risco principal é a disciplina de tick (um incremento por barreira por tick) e a ordem estável do hash — ambos cobertos por contrato explícito.
- **Rabbit holes**: modelar perseguição à emboscada, funções distintas de defensor ou reforço do Artesão; estender a phase de 8 s para 10 s "para bater o número" (o número é a recompensa, não o pacing); introduzir Lightyear/async/sockets "para sincronizar a defesa"; tocar em `crates/contratos` ou `PROTOCOL_VERSION`.

### Option B — Súditos/Propaganda

- **Sketch**: implementar primeiro a conversão de Nós e a Resistência Ideológica (§6.2:275), tornando a vitória por conversão observável antes da defesa.
- **Appetite**: `medium` (semanas) — exige três Nós, Propaganda canalizada de 20 s e o Intelectual.
- **Trade-offs**: ganha o objetivo final da partida; sacrifica o lastro canônico — os 3 Nós e os 100 pontos são `[HIPÓTESE]`. Risco: construir sobre números não fechados e sem a barricada que a fase Defender pressupõe.
- **Rabbit holes**: canalização de Propaganda, locomotiva de Súditos, regras de reconquista e relógio de 20 min.

### Option C — Ponte provisória

- **Sketch**: usar a mesma base de barricada para a variante de 15 Materiais em emboscadas da Rota Inferior/Delta do Comércio.
- **Appetite**: `small` (dias), mas com valor de design menor.
- **Trade-offs**: ganha reuso do caminho de construção; sacrifica o lastro canônico — GDD §10.3:574 marca a ponte como `[SUPOSIÇÃO]` e não há recompensa de autogestão associada. Risco: produto sem critério SC próprio de defesa.
- **Rabbit holes**: travessia de ilhas e regras de emboscada, que puxam mapa e IA para dentro do recorte.

### Option D — Rede (Lightyear/async/sockets)

- **Sketch**: sincronizar a defesa da barricada cliente-servidor antes de creditar pontos, introduzindo transporte autoritativo.
- **Appetite**: `large` (meses) — muda a natureza da fatia de Núcleo para infraestrutura de rede.
- **Trade-offs**: ganha base de rede antecipada; sacrifica qualquer retorno verificável no curto prazo. **Bloqueada por ADR-004 até decisão da SE**: Lightyear está ausente de `Cargo.toml`/`Cargo.lock` e nenhuma troca adicional de engine é permitida sem novo ADR (ADR-004:53).
- **Rabbit holes**: predição de contagem de ticks, reconciliação de crédito, lobby e matchmaking (fora do MVP).

## Recommendation

Escolher **Opção A**. É a única que satisfaz simultaneamente as metas de `define.md`: recompensa canônica fechada (GDD §7.1:306, confiança 4 — o valor é canônico, mas a presença do defensor é simplificada), efeito verificável por `hash_estado` (impacto 4) e fronteira pequena sobre a barricada 040 (facilidade 4, risco invertido 4 pela disciplina de tick). B constrói sobre `[HIPÓTESE]`, C sobre `[SUPOSIÇÃO]` sem recompensa própria e D está bloqueada por ADR-004. 80/20: **256/625** contra 108/625 (C), 48/625 (B) e 10/625 (D).

## Out of Scope (for the recommended option)

- Papéis Guerreiro/Artesão e reforço; Súditos, Nós de Controle, Resistência Ideológica e condição de vitória completa.
- Ponte provisória, Fôlego/alcance/recarga/dimensões, combate, IA e física de projéteis.
- Netcode, lobby, matchmaking, Lightyear, async e sockets; adoção de R6 (N/A por v1.2.2).
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` e qualquer código-fonte neste assessment.

## Assumptions to Validate

- O relógio corrente do Núcleo é 30 ticks/s, de modo que 10 s = **300 ticks** — confirmar no `/speckit.specify`.
- `Barreira` do ciclo 040 expõe `pv`, `ticks_restantes`, `dono` e `posicao` de modo a avaliar "intacta" e "mesmo ponto" sem estender o contrato de protocolo.
- `ponto_igual` é a comparação canônica de posição e serve para o teste dono↔barricada.
- Inserir/ler `defesa_ticks` em `BTreeMap<u64, Barreira>` e `pontos_autogestao` em `Jogador` preserva a ordem do `hash_estado` entre clientes.
- Em 360 ticks de vida da barricada cabe no máximo 1 crédito (300 ticks); o crédito seguinte depende de nova barricada.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; alternativas B/C/D são as mesmas rejeitadas em `research.md` e não recebem rastro de implementação. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica em `specs/044-defesa-barricada/`. **R6 N/A** por constituição v1.2.2, registrado em todas as opções como fora de Escopo.
