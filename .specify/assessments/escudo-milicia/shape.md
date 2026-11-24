# Shape — Escudo de Milícia (W do Guerreiro)

- **Slug**: `escudo-milicia`
- **Created**: 2026-11-23T22:00Z
- **Recommended option**: Opção A — Escudo de Milícia no Núcleo + cliente (rota única de dano)
- **Scorecard único do ciclo**: A **960/3125** (4×4×5×3×4); B 216/3125 (4×3×3×2×3); C 50/3125 (5×2×5×1×1, inelegível por ADR-004); rastreabilidade 053/054 0 (já fechada, fora da disputa). Mesmos valores de `intake.md`, `research.md` e `decision.md`, sem pontuação paralela.

## Options

### Option A — Escudo de Milícia no Núcleo + cliente (recomendada)

- **Sketch**: o jogador pressiona `V` e ergue um escudo que absorve até 250 de dano por 150 ticks (5 s), com recarga de 420 ticks (14 s), custo 25 Fôlego e alcance 3 m, aplicável a si mesmo ou a um aliado — nunca aos dois, nunca acumulado. No Núcleo, quatro campos em `Jogador` guardam absorção restante, duração, recarga e alvo; uma única função `aplicar_dano_a_jogador` substitui as duas subtrações diretas de vida em PvP e dano de PNJ, e o hash passa a serializar os quatro campos. No cliente, a tecla `V` gera o comando e a HUD passa a mostrar `Escudo: N (ticks)`. O efeito é determinístico e auditável por teste, sem relógio externo.
- **Appetite**: `small`/`medium` (dias) — seis constantes (uma reservada), quatro campos, uma ação validada, uma rota de dano, um avanço de ticks, um segmento de hash, um handler de tecla, uma linha de HUD e cinco testes; sem contrato, protocolo, Cargo, stack, rede, mapa ou CI novos.
- **Trade-offs**: ganha a primeira habilidade W jogável e, mais importante, unifica o dano ao jogador em uma rota auditável — PvP e PNJ passam a compartilhar a mesma regra de absorção, piso zero e `combate_ticks`. Impacto **4** (defesa com regra no GDD e dois consumidores nomeados), confiança **4** (valores e rotas lidos; efeito renderizado **[NV]**), fechamento **5** (jogador do cliente + sim determinístico consomem o resultado), esforço⁻¹ **3** (mudança real no sim e no cliente, com 5 testes novos), risco⁻¹ **4** (nenhuma mudança de stack/protocolo/rede; risco concentrado na rota única, mitigado pelas fronteiras testadas). O +15% contínuo do GDD fica de fora e é registrado como reserva documentada: o custo é honestidade sobre o corte, não dívida escondida.
- **Rabbit holes**: promover os números `[HIPÓTESE]` a cânone ou editar o GDD; implementar o +15% contra gás, canalização de 0,3 s, proteção ao Artesão ou efeito visual 3D; somar 250 + 250 em vez de atribuir; decrementar duração/recarga duas vezes no mesmo tick; aplicar absorção no atacante em vez da vítima; deixar uma das duas rotas de dano direto sem conversão; esquecer `escudo_alvo`/`escudo_recarga_ticks` no hash; usar `HashMap`, RNG, `Instant`, `unwrap`/`expect`/`#[allow]`; avançar monotônicos em comando recusado; conflitar `V` com `W`/`T`/`E`/`Q`; prometer captura visual real com janela minimizada; adotar R6 ou promover `dev→main`.

### Option B — Mapa/rotas de PVE com spawn por ilha

- **Sketch**: introduzir mapa, rotas e spawn determinístico por ilha para dar contexto de desafio ao combate, substituindo os quatro PNJs em posições fixas.
- **Appetite**: `large` (semanas) — geometria de mapa, topologia de ilhas, spawn, rotas e critérios de vitória ainda `[HIPÓTESE]`.
- **Trade-offs**: impacto **4**, confiança **3**, fechamento **3**, esforço⁻¹ **2**, risco⁻¹ **3** → **216/3125**. É frente legítima do backlog (BF-003), mas depende de regras de mapa não fechadas e não melhora a verificabilidade da defesa do jogador; abre escopo de design muito maior que o corte de uma habilidade.
- **Rabbit holes**: topologia de ilhas, spawn, rotas, Nós, Resistência Ideológica, "combate puro" fora do MVP, e a dependência de servidor/Rede para Carlos consumidor.

### Option C — Rede autoritativa real (Lightyear ou emenda de ADR-004)

- **Sketch**: postergar a habilidade até existir transporte, predição e reconciliação cliente-servidor, com peers remotos reais como alvo e autoridade.
- **Appetite**: `large` (meses) — troca de natureza do trabalho, de habilidade jogável para infraestrutura de rede.
- **Trade-offs**: impacto **5**, confiança **2**, fechamento **5**, esforço⁻¹ **1**, risco⁻¹ **1** → **50/3125 — inelegível**. ADR-004 bloqueia a troca até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`); sincronizaria uma fatia cuja verificação local ainda não foi fechada, invertendo a dependência verificabilidade → sincronização.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), predição de absorção, resolução de concorrência de comandos, NAT, troca de engine sem novo ADR, credenciais e infra (BF-009).

### Fora da disputa — Rastreabilidade 053/054 (BF-008)

- **Sketch**: marcar tasks e promover release já fechadas. Impacto **2**, confiança **5**, fechamento **0**, esforço⁻¹ **5**, risco⁻¹ **5** → **0**. Fator fechamento zero elimina a opção: o trabalho foi corrigido em `d7f4103` e manter como fechado não é entrega nova. Não é uma opção de shape, apenas registro do filtro eliminatório.

## Recommendation

Escolher **Opção A**. É a única que satisfaz as metas de `define.md` com os dois consumidores nomeados e sem depender de autorização externa: o jogador vê `Escudo: N (ticks)` e a sim passa a ter uma rota única de dano ao jogador, auditável por hash e por cinco testes nomeados. Impacto **4** (W do Guerreiro, regra no GDD), confiança **4** (código e números lidos; **[NV]** só no efeito renderizado), fechamento **5** (cliente + sim), esforço⁻¹ **3** (escopo real, porém local), risco⁻¹ **4** (fronteira de stack preservada). B (216/3125) é frente de design maior e não endereça a defesa verificável; C (50/3125) está bloqueada por ADR-004 e inverte a dependência; a rastreabilidade já fechada tem fechamento 0 e não compete. A recomendação é **go** com R1 registrado, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local` e **R6 N/A** — e com a condição explícita de que nada foi executado neste assessment.

## Out of Scope (for the recommended option)

- Promoção de números a cânone, edição do GDD, execução de 0,3 s, +15% de resistência a dano contínuo, gás lacrimogêneo, Sabotagem Estrutural, Artesão/entrantes, efeito visual 3D, canalização, RNG, `Instant`, mapa, ilhas e spawn por ilha.
- Netcode, Lightyear, sockets, async, lobby, matchmaking, predição/reconciliação, contratos, `PROTOCOL_VERSION`, `Cargo.toml`, CI ou stack nova (ADR-004 permanece com a SE).
- `STATE.md`, `AGENTS.md`, specs, validações, releases, `.specify/queue/concurrency.jsonl` e qualquer código-fonte neste assessment; **R6** (N/A por constituição v1.2.2), issues simuladas e `gh` improvisado; promoção `dev→main` e deploy (SWE-exclusivos).
- Gates, build, testes, formatadores, `./verify.sh`, CI, commit, push e PR neste assessment; captura visual real onde não houver surface automatizável (**[NV]**).

## Assumptions to Validate

- `PASSO_SEGUNDOS = 1/30` (`sim:15`) converte 5 s em **150 ticks** e 14 s em **420 ticks** sem arredondamento; validar que os decrementos ocorrem uma vez por tick em `avancar_tick` e que o tick de aplicação não consome indevidamente um tick de duração.
- `Jogador: Clone + Copy + Debug` aceita os quatro campos sem quebrar construtores, `Default` ou comparações; `escudo_alvo: Option<u64>` com `None`/`0` representa ausência de alvo.
- `ACOES_PERMITIDAS` e `nivel_minimo_para_acao` vivem no sim: acrescentar `"escudo"` com nível **2** não exige contrato, protocolo ou Cargo — confirmar a ordem de precedência das recusas em `ValidacaoComandos::validar` e que monotônicos avançam só em comando aceito.
- A absorção incide sobre a **vítima** de `aplicar_ataque` e do ataque de PNJ, consome `escudo_pv` primeiro e aplica só o excedente em `vida` com piso zero, preservando `combate_ticks` e a ordem das fases.
- Um segundo `escudo` **atribui** 250/150/420 (nunca `+=`), cobre self **ou** aliado a ≤ 3 m, e a reaplicação só é possível com recarga 0; critério de desempate entre aliados no cliente é determinístico e não abre clarificação de produto.
- `serializar_jogadores`/`hash_estado` incluem os quatro campos em ordem de `peer_id` do `BTreeMap`, sem `HashMap`, e dois mundos idênticos mantêm o mesmo hash.
- Os cinco testes nomeados cobrem 250/PNJ/420/150/hash e elevam 136 → **141** sem afrouxar os existentes; `./verify.sh`, cinco gates, roteiro 10/10 e CI `gates`+`rust` permanecem **[NV]** até execução pelo orquestrador.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 (regra hipotética / constantes+campos / ação nível 2+validação / rota única+aplicação+expiração+recarga / hash / cliente+testes+estilo+governança) e SC-001..SC-006. B e C são as alternativas de `research.md` e não recebem rastro de implementação neste ciclo; a rastreabilidade já fechada tem fechamento 0 e também não recebe rastro. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica em `specs/055-escudo-milicia/tasks.md`. R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**; nada executado, todo resultado de verificação **[NV]**.
