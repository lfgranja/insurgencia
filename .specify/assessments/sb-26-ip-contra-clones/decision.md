# Decision — IP contra clones

- **Slug**: sb-26-ip-contra-clones
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | Clones são documentados, mas a tese "fork é feature" redefine o que é problema aqui. |
| Força da evidência | adequate | Gamemeca e leis chinesas são primárias; acórdãos coreanos lidos só via artigo **[NV]**. |
| Valor vs. inação | strong | Registro cronológico custa quase zero e dá prova de anterioridade sem litígio. |
| Viabilidade / Appetite | strong | commits/hashes/datas/capturas já cabem no fluxo de release; sem serviço externo. |
| Alinhamento estratégico | strong | A parte defensiva casa com IV e VI; a ofensiva contradiz IV e é REJECT. |
| Postura de risco | adequate | Risco concentrado na via ofensiva (jurisdição, custo **[NV]**); contido pela proibição explícita. |

## Racional
ADAPT **apenas na parte defensiva**: adotar registro cronológico de autoria/proveniência — commits, hashes, datas e capturas íntegras — como prova de anterioridade. **REJECT**: litígio ou denúncia ofensiva contra forks, que contradiz a constituição IV ("fork é feature") e troca soberania por dependência de jurisdição externa. **DEFER**: denúncia em WeChat/Douyin, que depende de contato local e telefone chinês (gargalo documentado por Gamemeca), não sendo caminho crítico. Mecanismos legais existem (CAC arts. 14–17; Lei de Direitos Autorais arts. 10/53/54), mas acórdãos coreanos foram lidos só via artigo **[NV]**.

## Condição/responsável
Condição: nenhuma ação ofensiva sem nova decisão que revogue a tese IV; denúncia em plataforma chinesa só com contato local. Responsável: SE por qualquer ação pública; registro é automático via commits/tags. Custos/jurisdição e CI/merge permanecem **[NV]** até verificação ao vivo (VI).

## Handoff
Implementar registro cronológico de autoria/proveniência (commits, hashes, datas, capturas íntegras) no fluxo de release; **não** iniciar litígio nem denúncia ofensiva contra forks; manter denúncia em WeChat/Douyin como **DEFER** até contato local; saída de LLM informativa, nenhum serviço externo no caminho crítico.
