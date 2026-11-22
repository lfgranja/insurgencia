# Concept — Transformar "slot" em transformador determinístico

**Data**: 2026-11-21 · **Opção recomendada**: Opção 1 — transformador semeado com odds inspecionáveis, sem aposta (ADAPT).

## Contexto

O intake propõe a slot como mecânica central "surfando tendência de roguelikes de aposta". A literatura mostra que o que retém não é o caça-níquel em si, mas a conversão de sorte em habilidade (CloverPit rerolls baratos, 目押し em Slots & Daggers, "decisão clara" em Balatro) e a aleatoriedade exposta/controlada (Costikyan, Lost Garden, Riot). No repo, `fator`/`misturar`/`hash_estado` já dão RNG determinístico auditável. GDD §2.6 usa `alea` como contraste ao `agon`; Constituição V proíbe vitória por sorte. Veredito fixado: **ADAPT** — e o guardrail é explícito: **só como transformador semeado, odds inspecionáveis, sem aposta/estética de cassino**.

## Opções consideradas

1. **Transformador determinístico de materiais/funções, semeado e auditável — recomendada (ADAPT).**
   - **Sketch**: uma operação no `insurgencia-sim` que consome materiais conhecidos e produz um resultado amostrado de um **pool sem reposição** com **odds explícitas no código** e **seed registrada**. RNG é **pré-ação** (o jogador decide com base em odds antes de amostrar), reroll tem custo conhecido, e a sequência é reproduzível por `hash_estado()`. Sem rolos, luzes ou som de aposta — a apresentação é a de uma cadeia produtiva/materialização, não de cassino. Vitória continua por coordenação (Proposta 1 de `sb-11` complementar).
   - **Appetite**: small a medium — reaproveita o RNG existente no `sim`; `verify.sh --all-targets` cobre testes/example sem novo job; custo em definir odds/custo e expor clareza no HUD.
   - **Trade-offs**: ganha variedade por run e auditoria determinística (I/IV/VI); sacrifica o "brilho" imediato do caça-níquel. Risco: odds opacas ou near-miss reintroduzidos na implementação — mitigados por critério de aceite ("odds inspecionáveis", "sem aposta").
   - **Rabbit holes**: chamar "slot" ao transformador e importar a estética junto; RNG pós-ação; tabela de probabilidades escondida em backend; reroll sem custo que vira grinding; complexidade excessiva no HUD (Riot).
   - **Por que atende**: mantém o valor da mecânica (variedade + habilidade) sob as restrições I/IV/V/VI — é o caminho de menor atrito com o Núcleo Rust/Bevy atual (nota game-design, alternativa 1).

2. **Importar a slot machine "como está" (3x3, rolos, prêmios) — rejeitada.**
   - **Sketch**: implementar giro 3x3 que concede unidades/prêmios, com tabela de odds e animação de rolos.
   - **Appetite**: médio, mas exige backend/odds service e UI de apostas.
   - **Trade-offs**: entrega o "hook" reconhecível; em troca traz opacidade, dependência externa (IV), vitória por sorte (V) e estética de cassino — "black hat" na terminologia de Lost Garden.
   - **Rejeição**: contraria IV/V e GDD §2.6/§4.4; a análise ZH chama o componente de apostas de "原罪".

3. **Rejeitar toda aleatoriedade — rejeitada como decisão.**
   - **Sketch**: eliminar RNG, tudo fixo por semente constante.
   - **Appetite**: zero.
   - **Trade-offs**: máxima previsibilidade, mas perde variedade por run e a própria textura de simulação material que o GDD admite como contraste (`alea`).
   - **Rejeição**: confunde "sem aposta" com "sem variação"; o RNG determinístico já existe e pode ser usado com semântica (imprevisibilidade de terreno/clima/material, não "prêmio").

## Recomendação

**Opção 1**, coerente com **ADAPT**: transformar a ideia em um transformador semeado com odds inspecionáveis, RNG pré-ação, pool sem reposição, reroll com custo conhecido e **zero aposta/estética de cassino**. A vitória permanece por coordenação. Detalhes (odds numéricas, custo de reroll, exposição no HUD, integração com `sb-11`) pertencem ao handoff `/speckit.specify`. Nenhuma implementação ou teste nesta etapa.

## Fora de escopo

- Nesta etapa: qualquer tabela de odds, RNG, UI de rolos, crate, gate, teste, build, janela ou commit; escrita fora dos 4 artefatos deste slug.
- No handoff: aposta, prêmio financeiro, near-miss, odds opacas, RNG pós-ação, backend de probabilidades, estética de cassino, moeda premium, dependência de serviço externo (IV), alteração em ADR-004.
- `[NV]`: nenhuma medição de legibilidade/odds executada; números de vendas do gênero são outliers/autorrelato; conteúdo ZH integral não lido.

## Suposições a validar

- Um transformador com pool sem reposição e odds expostas entrega variedade suficiente sem "juice" de cassino — hipótese a validar por teste de legibilidade (Constituição VI).
- O custo de reroll pode ser expresso em materiais do jogo (não moeda premium), coerente com soberania do jogador.
- A exposição das odds cabe no teto de complexidade do HUD (coordenar com `sb-15`); se não couber, o signposting progressivo do Balatro é a referência.
