# Shape — Combate e Fôlego

- **Slug**: `combate-folego`
- **Created**: 2026-11-23T07:35Z
- **Recommended option**: Opção A — `atacar_forte` com efeito e Fôlego observável

## Options

### Option A — `atacar_forte` com efeito e Fôlego observável (recomendada)

- **Sketch**: em `crates/sim/src/lib.rs`, o `Jogador` passa a carregar `folego` (**máximo 100 — `[HIPÓTESE]` §10.1:519**), `folego_ticks` (acumulador) e `combate_ticks` (**90 ticks — escolha de protótipo**), todos no `hash_estado`. A ação `atacar_forte`, já em `ACOES_PERMITIDAS`, valida Fôlego (**custo 30 — escolha de protótipo**) e alvo no ponto/alcance (**2 m — `[HIPÓTESE]` §10.1:523**), debita o custo e aplica **dano 10 — escolha de protótipo** com `saturating_sub`. Um `BTreeSet<u64>` garante no máximo um ataque por jogador por tick; recusas usam `CodigoRecusa::FolegoInsuficiente` / `AlvoInvalido`. Em `fase_finish`, a regeneração credita **+1 a cada 3 ticks** fora de combate (de **10/s — `[HIPÓTESE]` §10.1:521**) e **+1 a cada 6 ticks** com `combate_ticks > 0` (de **5/s — `[HIPÓTESE]` §10.1:521**). No cliente, a tecla `Q` agenda a ação e o HUD exibe `Fôlego`/`Vida`.
- **Appetite**: `small` (dias) — uma ação já existente ganha efeito, três campos no `Jogador`, um acumulador em `fase_finish`, duas recusas e uma linha de HUD; sem `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, rede ou dependência nova.
- **Trade-offs**: ganha a última frente de produto sem mudança de stack (`STATE.md:351`) com estado auditável por `hash_estado`, respeitando o nome canônico do protótipo "Ataque Forte" (§10.6:680) sem adotar seus números hipotéticos. Sacrifica sistema de combate completo — IA, projéteis, colisão, balanceamento — justamente o que o GDD §16.3:1509 coloca fora do MVP. Confiança **2** e facilidade **2** registram que a ação existe mas não tem efeito e que todo número é `[HIPÓTESE]`/`escolha de protótipo`; risco invertido **3** reflete débito atômico, disciplina de tick, acumulador e hash a verificar.
- **Rabbit holes**: adotar os números de §10.6 (custo 20, 2,5 m, 150 de dano, recarga de 6 s — todos `[HIPÓTESE]`) no lugar dos valores do contrato; promover 100/10/5/2 m a cânone; implementar channeling, IA de Súditos, projéteis ou colisão; duplicar ataque quando o mesmo jogador aparece duas vezes no comando; creditar regeneração no tick do ataque; transformar a ação em pacote de protocolo.

### Option B — Súditos/Propaganda (rejeitada no ciclo 046)

- **Sketch**: retomar a conversão determinística de Súdito por Propaganda (`adicionar_sudito`, 120 ticks de exposição, reversão em 90 ticks).
- **Appetite**: `small` (dias) — também cabe em uma fatia, e já foi entregue como handoff no ciclo 046.
- **Trade-offs**: empatou com A em 48/625 (4×2×2×3) e venceu o desempate naquele ciclo porque GDD §9.2:464 é regra de aceitação escrita. O contrato 047 inverte o desempate e escolhe combate; reabrir B significaria desfazer uma decisão já tomada e um rastro FR/SC/T já entregue. Não é rejeição por pontuação — é rejeição por escopo de ciclo.
- **Rabbit holes**: reavaliar §9.2:464, spawn/raio/funções `[HIPÓTESE]`, Nós e Resistência Ideológica.

### Option C — Rede / Lightyear primeira

- **Sketch**: postergar a mecânica até transporte, predição e reconciliação cliente-servidor estarem prontos.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de fatia de Núcleo para infraestrutura de rede.
- **Trade-offs**: impacto 5, mas facilidade 1 e risco invertido 1 → **10/625**; ADR-004 bloqueia a troca até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`). Sincronizaria um Fôlego que ainda não é determinístico.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), predição de dano, resolução de concorrência de comandos, troca de engine sem novo ADR.

## Recommendation

Escolher **Opção A**. Ela satisfaz as metas de `define.md` com a menor fronteira: impacto **4** (ação sem efeito vira comportamento verificável por `hash_estado`), confiança **2** (todos os números são `[HIPÓTESE]`/`escolha de protótipo`, e o GDD §16.3:1509 prefere desafios de ilha a "combate puro"), facilidade **2** (ação existe, efeito não) e risco invertido **3** (débito atômico, um ataque por tick, acumulador e hash exigem verificação). B empatou em 48/625 mas perdeu o desempate por decisão de ciclo; C inverte a dependência determinismo → sincronização e está bloqueada por ADR-004. 80/20: **48/625** contra 10/625 (C); B é registrada como o 48/625 do ciclo anterior, não como candidato reaberto.

A recomendação **não** propõe promover nenhum número a cânone nem implementar um sistema de combate: o recorte é um ataque com recurso observável, com os valores de protótipo explicitamente rotulados.

## Out of Scope (for the recommended option)

- `[HIPÓTESE]` GDD §10.1:519-523: promover 100 de Fôlego máximo, 10/s, 5/s ou 2 m a valor canônico — permanecem rótulos documentais.
- `[HIPÓTESE]` GDD §10.6:680-701: custo 20, 2,5 m, 150 de dano, 6 s de recarga, channeling, Escudo de Milícia, Carga de Piquete e Linha de Defesa; escolha de protótipo 30/10/90 prevalece nesta fatia.
- IA, Súditos, projéteis, colisão, física, balanceamento, mapa/ilhas, desafios e condição de vitória; "combate puro" (§16.3:1509) não é o MVP.
- Súditos/Propaganda e seu rastro FR/SC/T (ciclo 046) — não reabertos.
- Netcode, lobby, matchmaking, `async`, sockets, Lightyear; mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`.
- **R6** (N/A por constituição v1.2.2): nenhuma issue simulada, nenhum `gh` improvisado.

## Assumptions to Validate

- A base é 30 ticks/s: **90 ticks = 3 s** (escolha de protótipo); **10/s → +1 a cada 3 ticks** e **5/s → +1 a cada 6 ticks** (`[HIPÓTESE]`), sem arredondamento. Confirmar no `/speckit.specify`.
- `atacar_forte` cabe em `ACOES_PERMITIDAS` e o `BTreeSet<u64>` de atacantes viaja no comando sem estender `crates/contratos` ou `PROTOCOL_VERSION`.
- A validação "no ponto/alcance" usa o alcance contato de **2 m (`[HIPÓTESE]`)** e não exige geometria de projétil; `CodigoRecusa::AlvoInvalido` é a recusa quando o alvo não está no ponto.
- `CodigoRecusa::FolegoInsuficiente` recusa sem débito parcial; o custo **30 (escolha de protótipo)** é debitado apenas no ataque aceito.
- `fase_finish` não credita regeneração no mesmo tick do ataque, e `combate_ticks` decai de **90 (escolha de protótipo)** a zero sem reiniciar o acumulador.
- `hash_estado` cobre `folego`, `folego_ticks` e `combate_ticks` de todos os jogadores em ordem estável.
- Cliente usa tecla `Q` just-pressed com `seq`/`request_id` monotônicos via recurso local; HUD exibe `Fôlego`/`Vida`.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; B e C são as alternativas de `research.md` e não recebem rastro de implementação neste ciclo. T001..T012 pertencem ao handoff em `decision.md`. Todo valor de combate aqui citado é `[HIPÓTESE]`/`escolha de protótipo`. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa.
