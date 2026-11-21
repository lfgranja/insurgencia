# Problem — RNG, juice e balance (sb-14)

**Data**: 2026-11-21

O intake afirma que o nicho prefere "loop viciante, sistemas, balance e juice" a arte artesanal e que as críticas negativas se concentram em RNG. No Insurgência, o problema concreto é outro: a simulação já é determinística e semeada (`fator`, `misturar`, `hash_estado`), mas nada garante que a aleatoriedade seja **legível** — o jogador/fork não consegue ver por que um resultado ocorreu, se a amostra veio antes da decisão e se o feedback sinaliza causa. "Mais RNG" e "mais juice" resolveriam o sintoma errado: opacidade e ruído de feedback são exatamente o que a literatura condena e o que a constituição I/VI impede verificar.

## Sucesso observável
- Toda amostra aleatória é reproduzível a partir de semente registrada; `hash_estado()` confere o mesmo traço para a mesma semente.
- Efeitos aleatórios são positivos ou neutros, amostrados **antes** da decisão relevante, com odds inspecionáveis no HUD.
- Cada manifestação de "juice" (cor, marcador, som quando existir) aponta para uma causa numerável da simulação; nada esconde informação necessária à decisão.
- `verify.sh` ALL STAGES OK (34 testes) e 5/5 gates PASS; nenhuma janela normal aberta (regra `--minimizado`).

## Custo de não agir
Sem exposição de semente/odds e sem feedback por causa, qualquer discussão de "balance" vira opinião: forks divergem, replay não é auditável e o RNG vira bode expiatório de decisões de design — desperdiçando a vantagem competitiva do determinismo.

**[NV]** Preferência real dos jogadores e supostas críticas ao RNG no Slotbound não medidas localmente.
