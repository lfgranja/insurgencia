# Problem — Tendência gambling-roguelike (sb-12)

**Data**: 2026-11-21

O intake propõe perseguir uma "tendência de mercado" exemplificada por Balatro. Para **este** projeto o problema não é ausência dessa mecânica: é a pressão para importar uma identidade de caça-níquel (giro, aposta, progressão por unidade poderosa) num jogo cuja tese é coordenação material e soberania do jogador. O GDD §2.6 já usa `alea` apenas como contraste; adotar a tendência como direção deslocaria o produto do `agon` cooperativo para sorte/espetáculo e colocaria o tempo de desenvolvimento na corrida de replicação pós-Balatro, onde os números são assimétricos e o *outlier* é confundido com regra.

## Sucesso observável
- Nenhuma estética, rótulo ou sistema de aposta entra no GDD, no cliente ou nos specs (verificação por `git diff` e gates).
- A variedade por run, quando existir, continua vinda de semente registrada e odds inspecionáveis (sb-14), não de giro opaco.
- A tendência permanece registrada apenas como dado de mercado em `docs/research/`, sem virar requisito de produto.

## Custo de não agir
Se a decisão for evitada, a ideia volta em cada ciclo de planejamento como "oportunidade perdida", consumindo tempo de design em benchmark de cassino e corre o risco de gerar uma feature que a constituição V e o GDD §4.4/§5.1 proíbem — retrabalho caro depois de implementado.

**[NV]** Não há evidência de que essa tendência influencie o público-alvo do Insurgência (engenheiros sem estúdio, GDD §3.2).
