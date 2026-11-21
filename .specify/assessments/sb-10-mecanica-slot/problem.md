# Problem — Como obter variedade por run sem importar aposta/opacidade

**Data**: 2026-11-21

## Problema

O intake descreve a slot machine como "mecânica central, surfando tendência de roguelikes de aposta" (https://youtu.be/Z4ayMTXnkoQ, `[UNVERIFIED — fetch skipped]`). A Steam confirma que Slotbound gira um 3x3 para invocar unidades e fundi-las. Para **este** projeto, adotar a mecânica "como está" colidiria com a Constituição IV (azar/opacidade como autoridade, dependência de backend de odds) e V + GDD §2.6/§4.4 (vitória por sorte, não por coordenação). O problema concreto é: **como preservar o valor da mecânica — variedade por run, rerolls com custo, conversão de sorte em habilidade — sem trazer a estética de cassino e sem quebrar o determinismo auditável** que o `insurgencia-sim` já oferece (`fator`, `misturar`, `hash_estado`).

## Sucesso observável

- A "rolagem" é um **transformador determinístico de materiais/funções**: seed registrada, odds inspecionáveis no código, pool sem reposição e reroll com custo conhecido — nunca aposta, nunca prêmio em dinheiro, nunca "near-miss" (Lost Garden).
- A decisão do jogador vem **antes** da amostra (RNG pré-ação): o jogador escolhe com base em odds claras; a amostra não substitui a agência (Riot/Costikyan).
- O resultado é reproduzível: mesma semente → mesma sequência, verificável por `hash_estado()` e por teste no `sim` (sem build executado nesta etapa `[NV]`).
- Nenhum serviço externo fornece odds ou resultados (IV); nenhuma vitória depende de "sortudo" (V); a estética não imita caça-níquel (sem rolos, luzes, som de aposta).
- Regra da janela (`--minimizado`) e ADR-004 (Bevy+Lightyear) preservadas.

## Custo de não agir

Não reformular o tema deixa o projeto sem uma resposta explícita ao "por que não slot?" e, ao mesmo tempo, abre espaço para que alguém importe o caça-níquel por moda (a onda pós-Balatro é rápida — 36氪 lista 4 domínios em ~2 anos `[NV]` quanto ao texto integral). O custo de adotar a mecânica crua é alto: conflito com IV/V, passivo de disclosure/ regulação (o componente de apostas é lido como "原罪" na análise ZH da 机核) e risco de opacidade que a literatura chama de "black hat". O custo de **adaptar** é baixo: o RNG determinístico já existe no `sim`; falta apenas defini-lo como transformador com odds claras.

## Stakeholders e restrições

- **Afetados**: mantenedor/SE (autoridade de design); Núcleo/`insurgencia-sim` (determinismo); jogadores (agência e clareza).
- **Restrições**: constituição I (determinismo), IV (soberania, sem serviço externo), V (ação coletiva, sem vitória por sorte), VI (verificação honesta); GDD §2.6 (`alea` como contraste), §4.4 (sem soma de desempenhos/carry); ADR-004; regra da janela; CI verde autoriza merge, nunca auto-merge cego; saída de LLM informativa.
- **Fora de escopo**: implementar rolagem, tabelas de odds, UI de slot ou aposta nesta etapa; tocar crates/gates/testes/docs fora destes 4 artefatos.

## Perguntas abertas

- `[NV]` Onde o transformador entra no loop: materialização de supply na Ilha Central ou cadeia produtiva (coordenar com `sb-11`)?
- `[NV]` Como expor odds "inspecionáveis" ao jogador sem estourar o teto de complexidade do HUD (Riot: "complexity ceiling"; related `sb-15`)?
- `[NV]` Qual custo de reroll é compatível com soberania do jogador (sem moeda premium, sem pressão de monetização)?
