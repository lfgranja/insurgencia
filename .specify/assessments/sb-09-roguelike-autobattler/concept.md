# Concept — Destino do auto-battler no Insurgência

**Data**: 2026-11-21 · **Opção recomendada**: Opção 3 — diferir até existir coordenação real; preservar a ideia como replay de ação coletiva (DEFER).

## Contexto

A Steam confirma o gênero de Slotbound (auto-battler single-player com slot e fusão), e a Riot define que TFT deliberadamente **não** recompensa coordenação — o modo solo carrega, junto, a negação do Pilar IV e o risco de "campeão solitário" (Constituição V / GDD §4.4, §5.1). O repo não tem servidor determinístico em produção nem 5v5, e o cliente não chama `avancar_tick` (nota fit-local). Veredito fixado: **DEFER** — não reabrir o veredito, apenas justificar.

## Opções consideradas

1. **Adotar o auto-battler solo como núcleo — rejeitada por conflito estrutural.**
   - **Sketch**: importar o loop "girar slot → invocar unidades → fundir → auto-resolver", com progressão por campeões, espelhando Slotbound.
   - **Appetite**: alto — exige reescrita da condição de vitória, backend de unidades/leaderboard e novo loop de UI.
   - **Trade-offs**: traz o gênero "quente" para o produto; em troca contraria IV/V, cria dependência de serviço externo e repete a corrida de power-level que a literatura (Riot) e o GDD condenam.
   - **Rejeição**: "vitória por soma de desempenhos independentes" é explicitamente proibida no GDD §4.4; os números de mercado são outliers `[NV]`.

2. **Adaptar o auto-battler como modo solo "inspecionável" — rejeitada por ainda ser solitário.**
   - **Sketch**: manter a auto-resolução, mas com semente auditável e odds explícitas, removendo apenas o caça-níquel.
   - **Appetite**: médio — reaproveita `hash_estado()`/`fator()` do `sim`, mas ainda exige modo solo.
   - **Trade-offs**: melhora transparência (I/VI), porém não resolve a ausência de ação coletiva (IV/V); a transparência não cria coordenação onde ela não existe.
   - **Rejeição**: a auditoria é necessária, mas não suficiente; sem servidor/5v5, o modo continua sendo "herói solitário auditável".

3. **Diferir até existir coordenação real; preservar como replay de ação coletiva — recomendada (DEFER).**
   - **Sketch**: não implementar agora. Registrar a intenção: quando houver servidor determinístico/headless e sessão 5v5, a auto-resolução pode entrar como **relatório legível** das ações coletivas já executadas (funções + materialização), com semente e `hash_estado()` — nunca como modo solo. É a alternativa 3 da nota game-design.
   - **Appetite**: zero neste ciclo; custo futuro depende do pré-requisito de plataforma/servidor.
   - **Trade-offs**: preserva coerência com IV/V e evita retrabalho; sacrifica uma "feature vistosa" imediata.
   - **Por que atende**: respeita a precedência I→III→II→IV do GDD e o veredito fixado; transforma um conflito em pré-requisito explícito, não em negação permanente.

## Recomendação

**Opção 3**, coerente com **DEFER**. Nenhuma implementação neste ciclo. A ideia fica registrada como possibilidade futura condicionada a (a) servidor determinístico/headless, (b) sessão 5v5, (c) forma de auto-resolução que **reporte coordenação** e não a substitua. Reabrir o tema antes disso exigiria novo assessment — não é decisão desta etapa.

## Fora de escopo

- Nesta etapa: qualquer modo solo, auto-battler, servidor, 5v5, código, crates, gates, testes, build ou publicação; escrita fora dos 4 artefatos deste slug.
- No retorno futuro (se aprovado): "campeão solitário", "carry", "vitória por soma de desempenhos independentes", RNG opaco, backend proprietário (IV) e estética de caça-níquel (tratada em `sb-10`/`sb-12`).
- `[NV]`: nenhum comportamento de auto-battler executado, nenhum servidor/5v5 no alvo, números de vendas autorrelato.

## Suposições a validar

- O pré-requisito (servidor determinístico + 5v5) aparece em ciclo futuro — até lá, `DEFER` permanece.
- "Replay de coordenação" pode ser construído sobre `avancar_tick` + `hash_estado()` sem virar corrida de power-level — hipótese a validar quando o pré-requisito existir.
- A precedência I→III→II→IV do GDD §4 continua norma de design (não reaberta aqui).
