# Assessment — 027-lowpoly-real

**Data**: 2026-11-21 · **Resultado**: GO (correção de defeito visual).

## Contexto
O ciclo 026 entregou 3D com primitivas (cubo/cone/prisma), mas a SE reportou: "Continua apenas greybox, nada de low-poly". A distinção GDD §13.2 é: greybox = primitivas para testar mecânicas; low-poly = geometria irregular com poucos polígonos.

## Impacto × confiança × facilidade ÷ risco
|Item|I|C|F|R|Score|
|---|---|---|---|---|---|
|Terreno irregular|3|3|3|1|27|
|Ilhas com relevo|3|3|2|1|18|
|Árvores/rochas|3|3|2|1|18|
|Avatar corporificado|2|3|2|1|12|
|Correção B0004|3|3|3|1|27|

**Decisão**: GO. Implementar malhas procedurais irregulares + elementos de cenário, sem assets externos (host fora da allowlist, licença CC0 exigiria auditoria).
