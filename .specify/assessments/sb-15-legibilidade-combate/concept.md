# Concept — Legibilidade do combate (sb-15)

**Data**: 2026-11-21

## Opções
1. **Ligar `avancar_tick` e iterar só no adaptador de apresentação (recomendada).** Conectar o `Mundo` do cliente ao tick determinístico do `sim` e então ajustar marcadores, ordem dos campos, destaque de ilha e telegrafia em `sincronizar_avatar`/`atualizar_texto`. Preserva determinismo, não cria dependências e é auditável por diff e `hash_estado`. Coerente com ADAPT.
2. **Reescrever regras/estado da sim para "facilitar" o HUD.** Melhoraria a exibição à custa da autoridade do `sim` e do princípio portas/adaptadores (constituição II); risco de quebrar os 34 testes. Rejeitada.
3. **Adotar o pacote de "juice" visual da demo (partículas, animações, assets).** Exige janela para avaliar **[NV]**, depende de assets externos e contradiz a regra da janela e a soberania. Rejeitada neste ciclo.

## Recomendação (ADAPT)
Opção 1, em duas etapas ordenadas: (i) ligar `avancar_tick` com `PASSO_SEGUNDOS = 1/30`; (ii) só então iterar HUD. ADAPT e não ADOPT: o princípio de legibilidade é válido, mas a forma depende do pré-requisito local e de verificação sob `--minimizado`.
