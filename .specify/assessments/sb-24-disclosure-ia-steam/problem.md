# Problem — Disclosure IA na Steam (sb-24)

**Data**: 2026-11-21

## Problema concreto no projeto
A regra atual da Steam dispensa disclosure de ferramentas de código e cobre só conteúdo consumido pelo jogador. Seguir esse mínimo seria **tecnicamente conforme e strategicamente errado** para Insurgência: a tese do projeto é inspecionabilidade (I/VI) e soberania do jogador (IV). Hoje não existe uma nota de proveniência que diga o que é procedural, o que é CC0 com fonte, o que é feito por humano e o que veio de IA revisada. Sem ela, qualquer afirmação pública sobre autoria fica solta — e o caso Slotbound mostra como uma descrição mais estreita do que a autodescrição anterior gera acusação de desonestidade, mesmo sem walk-back real.

## Sucesso observável
1. Existe nota de proveniência **mais ampla que o mínimo da Steam**, cobrindo: procedural, CC0 com fonte, humano, IA revisada.
2. Nenhum "walk-back": nada é editado para esconder afirmação pública anterior; a nota é estável e datada.
3. A nota vive em documentação versionada (`docs/adr/`, docs do projeto), sem depender de Steam page — que é **[NV]** até existir.
4. Alegações sobre o caso Slotbound distinguem escopo de retratação e mantêm **[NV]** onde o arquivo foi consultado apenas indiretamente.
5. Nada é afirmado como "verificado" sem comando executado (VI); saída de LLM é informativa.

## Custo de não agir
Sem nota de proveniência, o projeto perde a chance de transformar transparência em vantagem e fica vulnerável à mesma leitura de opacidade que atingiu Slotbound — com o agravo de que aqui a identidade *é* verificação. Adotar o mínimo da Steam seria, na prática, um walk-back encoberto.
