# Decision — Clones como marketing

- **Slug**: sb-21-clones-como-marketing
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | Clones são documentados no caso estudado, mas a incidência no projeto é externa e **[NV]**. |
| Força da evidência | adequate | Gamemeca e o postmortem Scale the Depths/AUTOMATON são sólidos; atribuição ao vídeo **[NV]**. |
| Valor vs. inação | adequate | Sem registro, a resposta vira assertiva; o valor é reativo, não gera tração por si. |
| Viabilidade / Appetite | strong | Registro datado em commits/releases; sem serviço externo no caminho crítico. |
| Alinhamento estratégico | strong | Defesa documental casa com I/VI e, sendo reativa, com a tese IV ("fork é feature"). |
| Postura de risco | weak | Fabricar o incidente é infalsificável e eticamente reprovável; risco mitigado por proibição explícita. |

## Racional
ADAPT: adota-se apenas a forma legítima do caso — registro datado de autoria e, **se o incidente ocorrer**, resposta pública com evidência. Nunca provocar, fabricar ou simular um clone (isso é infalsificável, reprovável e sem caso documentado). O que é transferível não é a narrativa de marketing, mas a capacidade de provar anterioridade sem depender de plataforma externa. Alcance em ZH e efeitos de imprensa permanecem **[NV]** até verificação ao vivo.

## Condição/responsável
Condição: nenhum incidente fabricado; registro apenas no fluxo de release/hash já existente. Responsável: SE pela resposta pública; o registro é automático via commits/tags. Status de CI/merge é **[NV]** até verificação ao vivo (VI).

## Handoff
Implementar registro cronológico de autoria/proveniência (commits, hashes, datas, capturas íntegras) nas releases; não criar clone nem incidente; responder somente com evidência se o incidente real ocorrer; manter saída de LLM informativa e nenhum serviço externo no caminho crítico.
