# Concept — Fundir materiais/funções, não unidades

**Data**: 2026-11-21 · **Opção recomendada**: Opção 1 — cadeia produtiva complementar com teto de poder e registro auditável (ADAPT).

## Contexto

O intake propõe fusão combinada com slots. A Steam confirma que Slotbound "absorve outras unidades" e as funde em uma "unidade mais forte" — hiper-unidade/carry, proibido pela Constituição V e GDD §4.4/§5.1. A literatura de merge mostra que o que retém é ordem, crescimento e feedback tátil repetido, não "números maiores", e que o subgênero é saturado (AppMagic) e estressante (GDC 2023). No repo, o `sim` já tem estado canônico (`hash_estado`) para auditoria. Veredito fixado: **ADAPT** — guardrail: **fundir apenas materiais/funções complementares, com teto de poder e sem hiper-unidade/carry**.

## Opções consideradas

1. **Cadeia produtiva: fundir materiais e papéis complementares com teto de poder — recomendada (ADAPT).**
   - **Sketch**: no `insurgencia-sim`, combinar matérias-primas (ou funções de diferentes ilhas) em bens de tier superior para a Ilha Central. Cada fusão tem **entradas/saídas auditáveis** via estado canônico (`hash_estado` + semente/commit), **teto de poder por ator** (ninguém vira superunidade) e **feedback que sinaliza causa** (o que mudou materialmente), não acumulação. A vitória segue por coordenação. O transformador de `sb-10` pode fornecer variedade de materiais; o HUD de `sb-15` torna a cadeia legível. Ausência de elenco de campeões e de economia premium.
   - **Appetite**: small a medium — reaproveita o `sim` determinístico e o gate `verify.sh`; custo em definir receitas, tetos e exposição no HUD.
   - **Trade-offs**: ganha Pilar II (ação material), cadeia produtiva legível e coerência com a condição de vitória (converter, não explodir); sacrifica o "power fantasy" do merge. Riscos: complexidade (Riot: "complexity ceiling") e acumulação disfarçada — mitigados por teto de poder e registro auditável.
   - **Rabbit holes**: transformar "material" em "unidade disfarçada" com níveis; permitir carry por acumulação de tier; importar economia mobile F2P; feedback que esconde a causa (juice como ruído).
   - **Por que atende**: é a alternativa 2 da nota game-design — conecta fusão ao Pilar II e à condição de vitória sem criar campeão solitário, com o mínimo atrito com o Núcleo atual.

2. **Fundir unidades em hiper-unidade (como Slotbound) — rejeitada.**
   - **Sketch**: absorver unidades aliadas e "ascender" a uma unidade mais forte, com níveis e poder acumulado.
   - **Appetite**: alto — exige elenco, progressão e balanceamento de poder.
   - **Trade-offs**: entrega o "power fantasy" reconhecível; em troca nega a coordenação (V), cria carry e contraria GDD §4.4/§5.1 diretamente.
   - **Rejeição**: é exatamente o "campeão solitário" que a constituição e o GDD proíbem; não é uma questão de estética.

3. **Rejeitar toda fusão — rejeitada como decisão.**
   - **Sketch**: eliminar o tema por completo.
   - **Appetite**: zero.
   - **Trade-offs**: elimina risco de carry, mas perde um mecanismo com apelo material (ordem/crescimento) e a chance de tornar a cadeia produtiva satisfatória.
   - **Rejeição**: opção "não fazer"; a Opção 1 captura o valor sem o risco.

## Recomendação

**Opção 1**, coerente com **ADAPT**: fundir **materiais/funções complementares** em bens de tier superior para a Ilha Central, com **teto de poder por ator** e **registro auditável** no `sim`, sem hiper-unidade e sem carry. Detalhes (receitas, tetos, feedback, integração com `sb-10`/`sb-15`) pertencem ao handoff `/speckit.specify`. Nenhuma implementação ou teste nesta etapa.

## Fora de escopo

- Nesta etapa: qualquer receita de fusão, tabela de tiers, UI, crate, gate, teste, build, janela ou commit; escrita fora dos 4 artefatos deste slug.
- No handoff: elenco de campeões/heróis, hiper-unidade, "ascensão" de unidade, carry, economia premium, RNG opaco, serviço externo (IV), alteração na condição de vitória do GDD, estética de caça-níquel (tratada em `sb-10`).
- `[NV]`: nenhuma implementação/medição de fusão; vídeo original não inspecionado; conteúdo de fontes ZH integralmente não lido.

## Suposições a validar

- Fundir materiais (não unidades) entrega a satisfação de "crescimento" sem power fantasy — hipótese a validar por teste de legibilidade, não por opinião (Constituição VI).
- O teto de poder por ator pode ser expresso no `sim` como invariante verificável, não como número opaco.
- A cadeia produtiva resultante cabe no HUD sem estourar a complexidade (Riot); se não couber, signposting progressivo.
