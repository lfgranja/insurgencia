# Concept — Como dar efeito observável ao Ato de Vandalismo do Agitador

**Data:** 2026-11-25 · **Opção recomendada:** Opção A — token + validação + ramo em `fase_skills` + tecla `H`

## Contexto

O GDD §10.4 descreve o Ato de Vandalismo com critérios de aceitação e números `[HIPÓTESE]` (120 de dano, 1,5 s de silêncio, 20 Fôlego, 2 m, 7 s de recarga — `docs/GDD - Insurgência.md:604-612`), e o §10.8 define o Agitador como função de infiltração e silêncio (`:744-757`). No núcleo, porém, `ACOES_PERMITIDAS` tem 14 tokens sem `vandalismo`, `fase_skills` não tem ramo correspondente e o cliente usa Q para `atacar_forte` (`crates/sim/src/lib.rs:222-236,920-1010`; `crates/cliente/src/main.rs:823-831`). A decisão 80/20 do ciclo 061 escolheu BF-002c com 288/3125 (`4×4×2×3×3`) e fixou DoD(0.9) com comportamento, consumidor, comandos de prova e backlog (`.specify/memory/STATE.md:606-609`). O problema tem impacto 4 e confiança 4; a lacuna não é documental, é de comportamento observável. O padrão já entregue no ciclo 060 para `sinalizar_sincronia` — token, validação, ramo determinístico, consumidor, hash e prova — é o único que fecha o gap sem mover a fronteira de governança.

## Opções consideradas

1. **Opção A — token + validação + ramo em `fase_skills` + tecla `H` — escolhida.**
   - **Sketch**: acrescentar `vandalismo` como 15º token de `ACOES_PERMITIDAS` (exige destino), validar `alvo`/alcance `ALCANCE_CONTATO_M` (2 m), Fôlego (20) e recarga (210 ticks), num mapa de nível compatível com Q (`_ => 1`, sem inventar papel). Em `fase_skills`, um ramo deduplicado aplica 120 de dano e `silenciado_ticks = 45` ao alvo e cancela a canalização em curso antes de `fase_finish`; o novo estado entra em `hash_estado` em ordem estável. Recusas preservam `seq`/`request_id`/`ultima_recusa`. No cliente, tecla protótipo `H` (Q já é `atacar_forte`) enfileira o comando com destino e mostra dica na HUD; a CLI aceita a mesma linha. Todos os números rotulados `[HIPÓTESE]`, sem alterar GDD/contrato/stack/CI.
   - **Appetite**: small (dias) — um token, validação, um ramo de fase, um contador no alvo, uma tecla e hashes/baselines atualizados; orçamento, não medição.
   - **Trade-offs**: ganha comportamento determinístico, reprodutível por CLI e coberto por hash, usando a autoridade do `Mundo`; sacrifica a pureza do catálogo de 14 ações e exige atualizar golden hashes e testes de CLI/cliente. Não entrega partida real, papéis ou itens — o GDD permanece fonte, não meta implementada.
   - **Rabbit holes**: ordem de aplicação do dano frente a `fase_defesa`/`fase_finish`; quais contadores de canalização zerar; decremento do silêncio e da recarga; colisão da tecla `H` com atalhos existentes; atualização de baselines de hash sem mascarar divergência real.

2. **Opção B — apenas adicionar o token à whitelist — rejeitada.**
   - **Sketch**: incluir `vandalismo` em `ACOES_PERMITIDAS` e nada mais; o comando passa na whitelist, mas nenhum ramo de `fase_skills` produz efeito e nenhuma tecla o dispara.
   - **Appetite**: trivial (horas).
   - **Trade-offs**: aparência de cobertura sem comportamento — exatamente o padrão de BF-015b antes do ciclo 060, em que o token `sinalizar_sincronia` era aceito e descartado no núcleo.
   - **Rabbit holes**: testes poderiam passar apenas com "comando aceito", deixando dano/silêncio/recusa sem prova; o hash não mudaria com o efeito; a CLI e o cliente continuariam sem consumidor real; o backlog não poderia ser fechado com honestidade.
   - **Rejeição**: **não fecha o gap do DoD(0.9)** — o critério de comportamento exige dano, silêncio, cancelamento de canalização, recusa que preserva monotônicos e hash cobrindo o novo estado, além de consumidor (jogador Bevy + CLI). É a mesma armadilha que o ciclo 060 corrigiu ao dar efeito a `sinalizar_sincronia`.

## Recomendação

**Opção A.** É a única que satisfaz simultaneamente os quatro itens do DoD(0.9): comportamento observável no núcleo determinístico, consumidor no cliente Bevy e na CLI, comandos de prova (`sim`, `servidor`, `cliente`, `./verify.sh`) e fechamento de BF-002c. A Opção B é rejeitada por ser uma mudança cosmética de whitelist: sem ramo em `fase_skills`, sem estado no hash e sem tecla, o efeito documentado no GDD continua inexistente no protótipo e o backlog não pode ser fechado. A recomendação é de conceito: implementação, testes, baselines de hash e preenchimento da spec pertencem ao handoff `/speckit.specify`, não a esta etapa.

## Fora de escopo (opção recomendada)

- Nesta etapa: qualquer token, validação, ramo, tecla, HUD, spec, plano, task, código, gate, teste, build, formatação, stage, commit ou push; escrita fora dos cinco artefatos de assessment.
- No ciclo de implementação: mudança de GDD, cânone, topologia de ilhas, papéis/funções no núcleo, sistema de itens, rede/ADR-004, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI/stack; promoção `dev→main`, PR #76, deploy, force-push; R6 (N/A por constituição v1.2.2 — PARE e escale, sem issue simulada).
- Não se afirma nesta etapa compilação, execução de gate, paridade de hash, comportamento em partida real ou desempenho — tudo permanece **[NV]**.

## Suposições a validar

- Os números do GDD convertem para o protótipo sob `[HIPÓTESE]`: 1,5 s × 30 Hz = 45 ticks de silêncio; 7 s × 30 Hz = 210 ticks de recarga; 2 m = `ALCANCE_CONTATO_M`; 20 Fôlego; 120 de dano. Nenhum valor é promovido a cânone.
- O estado de silêncio pode ser representado sem alterar o contrato de rede (campo no `Jogador`) e incluído no `hash_estado` em ordem estável, com baselines atualizados no mesmo conjunto de mudanças.
- A canalização a cancelar usa contadores já existentes (`propaganda_*`, `construcao_*`, `ponte_*` ou equivalente) e a ordem das fases permite aplicar dano/silêncio antes de `fase_finish` sem duplo crédito por tick.
- A tecla `H` está livre e não colide com `B,C,P,G,Q,R,F,T,E,Y,V`, setas ou `Espaço`; a dica na HUD é texto de protótipo.
- "Função Agitadora" é rótulo de produto; o núcleo não tem campo de papel e o gate de nível (`_ => 1`) é o único critério de acesso implementado — confirmar na spec que nenhuma regra canônica de função está sendo inventada.
