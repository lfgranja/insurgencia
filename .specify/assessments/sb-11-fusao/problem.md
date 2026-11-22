# Problem — Como usar fusão sem criar hiper-unidade/carry

**Data**: 2026-11-21

## Problema

O intake registra "mecânica de fusão combinada com slots" (https://youtu.be/Z4ayMTXnkoQ, `[UNVERIFIED — fetch skipped]`). A Steam confirma que Slotbound faz "grow your army by absorbing others … ascend into a stronger unit". Para **este** projeto, fundir **unidades** criaria uma hiper-unidade que substitui a coordenação por poder individual — exatamente o "campeão solitário"/"carry" proibido pela Constituição V e pelo GDD §4.4/§5.1. A nota fit-local (achado 4) classifica o conflito como estrutural. O problema concreto é: **como aproveitar o que a fusão tem de bom (ordem, crescimento, feedback tátil repetido) sem importar a corrida de power-level que o gênero vende — e sem atrair a economia de mobile F2P saturada (3 líderes com 99% da receita)**.

## Sucesso observável

- A fusão combina apenas **materiais e papéis/funções complementares** (ex.: matéria-prima → bem de tier superior para a Ilha Central), nunca heróis/unidades em uma superunidade.
- Existe **teto de poder por ator** e **registro auditável** de cada fusão (o que entrou, o que saiu, seed/commit), verificável no `sim` determinístico — sem serviço externo (IV).
- A condição de vitória permanece **converter/sustentar por coordenação** (GDD §4.4), nunca "soma de desempenhos independentes" ou "carry".
- O feedback da fusão sinaliza **causa** (o que mudou no estado material), não apenas acumulação — clareza causal (VI) e legibilidade no HUD (coordenar com `sb-15`).
- Nenhum elenco de campeões, nenhuma economia premium, nenhuma dependência de serviço externo; ADR-004 e regra `--minimizado` preservadas.

## Custo de não agir

Rejeitar a fusão por completo perderia um mecanismo com apelo material para o Pilar II (ação/materialismo) e a chance de tornar a cadeia produtiva da Ilha Central legível e satisfatória. O custo de adotá-la "como está" é alto: conflito com V/§4.4, risco de hiper-unidade, saturação de mercado (AppMagic: "dead-end" em Merge-3/5) e complexidade de HUD (Riot: "complexity ceiling"). A adaptação tem custo baixo: o `sim` já registra estado canônico (`hash_estado`) sobre o qual a fusão pode ser auditada.

## Stakeholders e restrições

- **Afetados**: mantenedor/SE (autoridade de design); Núcleo/`insurgencia-sim` (estado auditável); jogadores (coordenação vs. carry).
- **Restrições**: constituição II (ação material), IV (sem serviço externo), V (sem campeão solitário/carry), VI (verificação honesta); GDD §4.4 e §5.1; ADR-004 (Bevy+Lightyear fixo); regra da janela (`--minimizado`); CI verde autoriza merge, nunca auto-merge cego; saída de LLM informativa.
- **Fora de escopo**: criar elenco de unidades/heróis, hiper-unidade, economia de mobile F2P ou serviço de fusão nesta etapa; tocar crates/gates/testes/docs fora destes 4 artefatos.

## Perguntas abertas

- `[NV]` Quais materiais/funções do GDD são complementares o bastante para uma cadeia de fusão coerente com a Ilha Central?
- `[NV]` Como expressar "teto de poder por ator" no `sim` sem transformar a fusão em número opaco?
- `[NV]` Como dar feedback de causalidade no HUD sem estourar o teto de complexidade (coordenar com `sb-15`)?
