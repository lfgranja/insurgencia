# Define — Combate e Fôlego

- **Slug**: `combate-folego`
- **Created**: 2026-11-23T07:35Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §10.1:517-523, §10.6:680, §16.3:1505-1509, `STATE.md` ciclo 047, constituição v1.2.2)

A ação `atacar_forte` já está em `ACOES_PERMITIDAS`, mas não produz efeito: não há débito de recurso, validação de alvo, dano ou estado de combate no Núcleo. Ao mesmo tempo, o GDD §10.1:517 define Fôlego como reserva física que se regenera por tempo — "não é mana (energia mágica) nem recurso sobrenatural" — e o jogador não carrega `folego`, `folego_ticks` nem `combate_ticks`. Jogador e Núcleo não compartilham uma verdade verificável sobre cansaço, acerto ou duração do combate, e a distinção documental do GDD permanece apenas declarada. O recorte resolve isso com uma fatia small: um ataque, um recurso e um HUD, sem IA, projéteis ou rede e **sem promover nenhum valor de combate a cânone** — todos os números abaixo são `[HIPÓTESE]` (§10.1) ou escolha de protótipo (contrato da sessão).

## Affected Users & Stakeholders

- **Users**: jogadores da Comuna (Guerreiro/Milícia), que precisam que `atacar_forte` tenha efeito previsível e que o Fôlego seja gasto e recuperado em tempo conhecido (GDD §10.1:517-521; §10.6:680); jogadores que recebem o ataque, que precisam de dano determinístico e de uma recusa explícita quando o alvo é inválido.
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de disciplina de tick, débito atômico e `hash_estado` estável; SWE/orquestrador — precisa de uma fatia testável sem mudança de contrato, protocolo ou stack; designer — precisa que `[HIPÓTESE]` continue `[HIPÓTESE]` e que a objeção de §16.3:1509 ("em vez de combate puro") seja respeitada pelo escopo mínimo.

## Goals

- Dar efeito a `atacar_forte` em `crates/sim/src/lib.rs`: validar Fôlego (**custo 30 — escolha de protótipo**) e alvo no ponto/alcance (**2 m — `[HIPÓTESE]` §10.1:523**), debitar o custo e aplicar **dano 10 — escolha de protótipo** com `saturating_sub`.
- Manter `folego` (**máximo 100 — `[HIPÓTESE]` §10.1:519**), `folego_ticks` (acumulador) e `combate_ticks` (**duração 90 ticks — escolha de protótipo**) no `Jogador`, com todos os campos incluídos no `hash_estado`.
- Regenerar em `fase_finish`: **+1 a cada 3 ticks** fora de combate (derivado de **10/s — `[HIPÓTESE]` §10.1:521**) e **+1 a cada 6 ticks** em combate (derivado de **5/s — `[HIPÓTESE]` §10.1:521**), sem duplo crédito no tick do ataque.
- Garantir no máximo **um** ataque por jogador por tick via `BTreeSet<u64>`, sem duplicatas e sem consumo parcial em caso de recusa.
- Expor recusas explícitas (`CodigoRecusa::FolegoInsuficiente` / `AlvoInvalido`), tecla `Q` just-pressed com `seq`/`request_id` monotônicos no cliente e HUD com `Fôlego`/`Vida`.
- Manter compatibilidade: nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`; nenhum `async`, rede ou Lightyear; constituição v1.2.2 com R1 `go`, R4 antes de R7 (`1.0.0-local`) e **R6 N/A**.

## Non-Goals

- IA de inimigos, Súditos, projéteis, colisão, física, channeling de **0,4 s** ou recarga de **6 s** (GDD §10.6:680, `[HIPÓTESE]`), múltiplas armas e balanceamento — o recorte é um ataque, não "combate puro" (§16.3:1509).
- Promover qualquer número a cânone: 100, 10/s, 5/s e 2 m são `[HIPÓTESE]` (§10.1:519-523); custo 30, dano 10 e 90 ticks são escolhas de protótipo do contrato; os valores de "Ataque Forte" em §10.6 também são `[HIPÓTESE]` e não são reaproveitados.
- Mapa, ilhas, desafios, condição de vitória, Nós, Resistência Ideológica, Propaganda e Súditos (ciclo 046) — não reabertos.
- Netcode, lobby, matchmaking, predição/reconciliação, `async`, sockets ou Lightyear (ADR-004); rede ficou em 10/625 no scorecard.
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`; adotar R6 ou simular emissão de issues.

## Success Metrics

- Com Fôlego suficiente e alvo no ponto/alcance (**2 m — `[HIPÓTESE]`**), `atacar_forte` debita **30 (escolha de protótipo)** e aplica **10 de dano (escolha de protótipo)** via `saturating_sub`, sem valor negativo (baseline: efeito inexistente — **[NV]** até execução).
- Com Fôlego inferior a **30 (escolha de protótipo)**, o ataque é recusado com `FolegoInsuficiente`, sem dano e sem débito parcial (baseline: **[NV]**).
- Alvo fora do ponto/alcance (**2 m — `[HIPÓTESE]`**) é recusado com `AlvoInvalido`, sem consumo de Fôlego (baseline: **[NV]**).
- No máximo **1** ataque por jogador por tick, mesmo com comandos repetidos (`BTreeSet<u64>` sem duplicatas) (baseline: **[NV]**).
- Fora de combate, **+1 Fôlego a cada 3 ticks**; com `combate_ticks > 0` (**janela de 90 ticks — escolha de protótipo**), **+1 a cada 6 ticks**; a janela expira no tick correto, sem regeneração acelerada indevida (baseline: **[NV]**).
- `folego`, `folego_ticks` e `combate_ticks` de todos os jogadores aparecem em `hash_estado` em ordem estável; cliente agenda com `Q` e HUD exibe `Fôlego`/`Vida` (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, `atacar_forte` segue como nome sem efeito e o GDD §10.1:517-523 continua sem recipiente observável: não há reserva física, contador de regeneração nem relógio de combate no `Jogador`. Qualquer ciclo futuro que precise de cansaço ou acerto determinístico teria de reconstruir débito atômico, disciplina de tick e hash desde zero — e a última frente de produto sem mudança de stack (`STATE.md:351`) ficaria sem fechamento verificável. O custo é a perda de uma fatia pequena e auditável, não a ausência de um sistema de combate completo.

## Open Questions

- Nenhuma clarificação de design pendente no contrato: os números foram fornecidos como hipóteses/escolhas de protótipo e não devem ser renegociados (R3 = 0 clarificações).
- **[NV]** Confirmar no `/speckit.specify` como o ponto do alvo é validado para o alcance de **2 m (`[HIPÓTESE]`)** sem geometria de projétil; quais variantes de `CodigoRecusa` já existem; se `atacar_forte`/`BTreeSet<u64>` cabem em `ACOES_PERMITIDAS` sem tocar em contrato/protocolo.
- **[NV]** Confirmar que a recusa é sempre sem consumo parcial e que `fase_finish` não credita regeneração no mesmo tick do ataque.
- **[NV]** Conformidade de R4/R7 (`1.0.0-local`) depende de gates não executados neste assessment.
- Fora de escopo por decisão, não lacuna: IA, projéteis, colisão, balanceamento e os demais itens de `Non-Goals`.

## Rastreabilidade (escopo do problema)

- FR-001 → constantes documentadas e rotuladas (100, 10/s, 5/s, 2 m `[HIPÓTESE]`; 30, 10, 90 ticks escolha de protótipo); FR-002 → `folego`, `folego_ticks`, `combate_ticks` no `Jogador` e no `hash_estado`; FR-003 → efeito de `atacar_forte` com validação de alvo/alcance e Fôlego; FR-004 → dano `saturating_sub`, débito atômico e `BTreeSet<u64>`; FR-005 → regeneração em `fase_finish` com acumulador e janela de 90 ticks; FR-006 → recusas, tecla `Q`, HUD `Fôlego`/`Vida` e fronteira inalterada.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis do handoff em `decision.md`, com definição canônica prevista em `specs/047-combate-folego/` (template no momento da leitura).
- **R6 é N/A** por constituição v1.2.2 e não entra neste rastro. Todo valor de combate aqui citado é `[HIPÓTESE]`/`escolha de protótipo`; nada é cânone.
