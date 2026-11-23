# Shape — Ponte provisória determinística

- **Slug**: `ponte-provisoria`
- **Created**: 2026-11-23T06:25Z
- **Recommended option**: Opção A — Ponte provisória determinística

## Options

### Option A — Ponte provisória determinística (recomendada)

- **Sketch**: nova ação `construir_ponte` em `ACOES_PERMITIDAS` da sim (nenhuma mudança em `crates/contratos` ou `PROTOCOL_VERSION`). A validação exige `destino == posicao` e saldo ≥ `CUSTO_PONTE_MATERIAIS` (15); sem saldo, a ação é recusada **sem consumo parcial** e o débito de 15 Materiais é atômico. A ponte é a struct pública `Ponte { id, dono, posicao, pv, ticks_restantes }` em `BTreeMap<u64, Ponte>` em `Mundo`, com constantes `CUSTO_PONTE_MATERIAIS = 15`, `PV_PONTE = 60`, `TICKS_DURACAO_PONTE = 1350` (45 s a 1/30 s) e `TICKS_CONSTRUCAO_PONTE = 45` (**escolha de protótipo, não valor canônico**). A construção consome 45 ticks determinísticos; a ponte expira ao completar 1350 ticks ou é removida quando `pv == 0`; todas as Pontes entram em `hash_estado` em ordem estável. O HUD mostra a contagem de Pontes e associa a tecla `P` à construção.
- **Appetite**: `small` (dias) — uma ação nova, uma struct em `BTreeMap`, quatro constantes, validação/débito atômico, expiração por tick, hash e duas linhas de HUD, reaproveitando o padrão de coleta/construção já entregue; sem contrato, protocolo, dependência ou rede novos.
- **Trade-offs**: ganha o próximo recurso com valores canônicos fechados (15 Materiais, 60 PV, 45 s) e efeito verificável por `hash_estado`; sacrifica o **atalho de 8 s entre ilhas**, que depende de mapa/ilhas e fica **fora de escopo, registrado como limitação**. O tempo de construção de 45 ticks é mantido como escolha de protótipo, não promovido a regra do GDD. Risco principal: débito atômico sem consumo parcial, validação `destino == posicao`, um tick de construção/expiração por ponte e ordem estável do hash — cobertos por contrato explícito.
- **Rabbit holes**: modelar mapa/ilhas Delta-1/Delta-2, travessia ou o atalho de 8 s "para bater o número do GDD"; introduzir emboscadas, IA ou dano de projéteis; negociar 15 Materiais/60 PV/45 s; transformar os 45 ticks de construção em valor canônico; tocar em `crates/contratos` ou `PROTOCOL_VERSION`; introduzir Lightyear/async/sockets "para sincronizar a ponte".

### Option B — Súditos/Propaganda

- **Sketch**: implementar primeiro a conversão de Nós e a Resistência Ideológica (§6.2:275), tornando a vitória por conversão observável antes da ponte.
- **Appetite**: `medium` (semanas) — exige três Nós, Propaganda canalizada de 20 s e o Intelectual.
- **Trade-offs**: ganha o objetivo final da partida; sacrifica o lastro canônico — Nós e 100 pontos são `[HIPÓTESE]`. Risco: construir sobre números não fechados e sem o recurso de logística que o Delta-3 pressupõe.
- **Rabbit holes**: canalização de Propaganda, locomotiva de Súditos, reconquista e relógio de 20 min.

### Option C — Combate/Fôlego

- **Sketch**: reabrir Fôlego, alcance, recarga e dimensões das habilidades para extrair um recorte jogável de combate antes da ponte.
- **Appetite**: `medium` (semanas) — Fôlego, alcance, recarga e dimensões são `[HIPÓTESE]` (GDD §10.3:574-579) e exigem IA, dano e física.
- **Trade-offs**: ganha pressão de combate; sacrifica a regra canônica — a ponte já tem custo, PV e duração fechados, enquanto estes números não têm. Risco: virada de escopo para sistema de combate inteiro.
- **Rabbit holes**: IA de inimigos, projéteis, balanceamento de dano, recarga e hierarquia de funções.

### Option D — Rede (Lightyear/async/sockets)

- **Sketch**: sincronizar a construção da ponte cliente-servidor antes de debitar Materiais, introduzindo transporte autoritativo.
- **Appetite**: `large` (meses) — muda a natureza da fatia de Núcleo para infraestrutura de rede.
- **Trade-offs**: ganha base de rede antecipada; sacrifica qualquer retorno verificável no curto prazo. **Bloqueada por ADR-004 até decisão da SE**: Lightyear está ausente de `Cargo.toml`/`Cargo.lock` e nenhuma troca adicional de engine é permitida sem novo ADR (ADR-004:53).
- **Rabbit holes**: predição do débito, reconciliação de ticks de construção/expiração, lobby e matchmaking (fora do MVP).

## Recommendation

Escolher **Opção A**. É a única que satisfaz simultaneamente as metas de `define.md`: valores canônicos fechados (GDD §9.3:475,483; §8.4:407), efeito verificável por `hash_estado` e fronteira pequena sobre o padrão de construção já entregue. Confiança 3 porque o uso em emboscadas é `[SUPOSIÇÃO]` (§10.3:574) e o atalho de 8 s fica fora de escopo; facilidade 3 pela validação `destino == posicao`, débito atômico e expiração por tick; risco invertido 4 pela disciplina exigida. B e C constroem sobre `[HIPÓTESE]`, D está bloqueada por ADR-004. 80/20: **108/625** contra 48/625 (B), 48/625 (C) e 10/625 (D).

## Out of Scope (for the recommended option)

- **Atalho de 8 s entre ilhas** — depende de mapa/ilhas; fora de escopo, registrado como limitação.
- **Tempo de construção de 45 ticks tratado como canônico** — é escolha de protótipo, não valor do GDD.
- Mapa, ilhas, rotas comerciais, emboscadas, IA, combate, Fôlego/alcance/recarga/dimensões.
- Súditos, Nós de Controle, Resistência Ideológica, Propaganda e condição de vitória completa.
- Netcode, lobby, matchmaking, Lightyear, async e sockets; adoção de R6 (N/A por v1.2.2).
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` e qualquer código-fonte neste assessment.

## Assumptions to Validate

- O relógio corrente do Núcleo é 30 ticks/s, de modo que 45 s = **1350 ticks** — confirmar no `/speckit.specify`.
- `ACOES_PERMITIDAS` aceita a nova ação `construir_ponte` sem extensão do contrato de protocolo.
- `Mundo` expõe `BTreeMap<u64, Ponte>` e `Comando.destino`/`posicao` são comparáveis pelo predicado canônico de posição, sem tocar em `crates/contratos`.
- O saldo de Materiais é legível e debitável de forma atômica; recusa com saldo < 15 não consome nada.
- Inserir/ler/remover `Ponte` no `BTreeMap<u64, Ponte>` preserva a ordem do `hash_estado` entre clientes, e um único tick de expiração por ponte evita dupla remoção.
- A tecla `P` e a contagem de Pontes são acrescentadas ao HUD sem outra dependência de entrada.
- **Confirmar como protótipo**: `TICKS_CONSTRUCAO_PONTE = 45` é escolha de implementação, não número canônico; **manter fora de escopo** o atalho de 8 s.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; alternativas B/C/D são as mesmas rejeitadas em `research.md` e não recebem rastro de implementação. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica em `specs/045-ponte-provisoria/`. **R6 N/A** por constituição v1.2.2, registrado em todas as opções como fora de Escopo.
