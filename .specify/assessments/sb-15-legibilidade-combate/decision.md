# Decision — Legibilidade do combate (sb-15)

- **Slug**: sb-15-legibilidade-combate
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Sem `avancar_tick` no cliente, não há combate observável; depois do tick, causas precisam de apresentação. |
| Força da evidência | strong | Riot e GMTK são fontes primárias sobre telegrafia/complexidade; mapeamento do seam é leitura direta do código. |
| Valor vs. inação | adequate | Alto valor potencial, mas condicionado ao pré-requisito do tick; hoje o ganho é vacuo. |
| Viabilidade/Appetite | adequate | Duas etapas pequenas, porém serializadas: ligar tick antes de mexer no HUD. |
| Alinhamento estratégico | strong | Mantém sim autoritativa e adaptador puro (II), determinismo (I) e verificação honesta (VI). |
| Postura de risco | adequate | Risco de mexer na sim e quebrar determinismo; mitigado por escopo restrito ao adaptador e gates. |

## Racional
ADAPT: a legibilidade é critério de agência comprovado (Riot: efeitos previsíveis e "jogáveis em volta"; teto de complexidade; signposting progressivo), mas a versão do vídeo é relato não verificado **[NV]** e o cliente local ainda não roda combate. A adaptação é ligar `avancar_tick` e iterar **apenas** no adaptador de apresentação — nunca na sim — com captura sob `--minimizado`. Não se troca engine (ADR-004) nem se acrescenta asset externo.

## Condição/responsável
Responsável: SE via fluxo SDD; a mudança do cliente depende de aprovação por `verify.sh` (check → clippy → nextest → fmt) e 5 gates. CI verde autoriza merge; auto-merge cego proibido. Inspeção visual final com a SE **[NV]**.

## Handoff
- **Problema**: cliente espelha estado sem avançar a sim; inexistência de eventos de combate legíveis.
- **Abordagem**: ligar `Mundo::avancar_tick` a `PASSO_SEGUNDOS`, depois refinar `sincronizar_avatar`/`atualizar_texto` para telegrafia, ordem dos campos, destaque de ilha e causas numeradas.
- **In scope**: adaptador de apresentação (HUD/Text), captura determinística sob `--minimizado`, diff de HUD entre versões.
- **Out of scope**: alterações nas regras do `sim`, RNG novo, assets/fontes externas, janela normal, juízos de estética não verificáveis.
- **Métricas**: `hash_estado()` idêntico para a mesma semente antes/depois da apresentação; causas exibidas para cada evento; 34 testes + 5 gates PASS **[NV até execução ao vivo]**.
- **Perguntas remanescentes**: quais eventos de combate telegrafar primeiro? Quem valida a leitura visual (SE) sem abrir janela? As atualizações citadas no intake têm fonte primária [NV]?
