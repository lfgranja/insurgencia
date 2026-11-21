# Concept: Hotfixes rápidos

- **Slug**: sb-22-hotfixes-rapidos
- **Created**: 2026-11-21
- **Recommended option**: Option A — Patches pequenos com gate verde e notas versionadas

## Options

### Option A — Patches pequenos com gate verde e notas versionadas

- **Sketch**: cada correção vira um patch mínimo com tag semver; notas curtas (o que mudou, por quê, como verificar) vão para CHANGELOG e GitHub Releases; `verify.sh` é executado e sua saída colada antes do merge autônomo autorizado por CI verde. Triagem por impacto — bloqueante > regra > cosmético — com limite explícito de tempo para leitura de reviews e resposta a poucas, sem pedir avaliação dentro do jogo. Não se promete prazo de hotfix; promete-se rastreabilidade.
- **Appetite**: small — já existem `verify.sh`, CI e canal `gh`; falta apenas disciplina documental.
- **Trade-offs**: velocidade um pouco menor que "publicar direto", trocada por proveniência e detecção de regressão; notas exigem escrita mínima a cada tag. Não há custo de plataforma nem SDK.
- **Rabbit holes**: transformar "9 updates em 3 dias" em meta de contagem; publicar com "verde" pressuposto; usar Update Visibility Rounds em cada patch; responder toda review para parecer responsivo.

### Option B — Hotfix direto sem gate, com correção depois

- **Sketch**: publicar a correção imediatamente e rodar o gate posteriormente.
- **Appetite**: mínimo no curto prazo.
- **Trade-offs**: maximiza velocidade aparente, mas inverte a constituição VI (verde afirmado sem comando executado), cria risco de regressão e destrói a rastreabilidade que diferencia o projeto do caso estudado.
- **Rabbit holes**: chamar "pequeno" a qualquer mudança para pular o gate; confiar em CI remoto não observado.

### Option C — SLA de resposta a todas as reviews

- **Sketch**: compromisso público de responder cada avaliação e corrigir tudo em prazo fixo.
- **Appetite**: large e recorrente.
- **Trade-offs**: parece responsivo, mas a própria Valve desaconselha resposta integral e alerta para limitar tempo de leitura; com 1.598 reviews e score de 79%, é trabalho sem evidência de retorno e insustentável para dev solo.
- **Rabbit holes**: confundir engajamento com verificação; tratar "fila de reviews" como fila de tarefas obrigatórias.

## Recommendation

Option A, coerente com o veredito ADOPT: adota-se a prática observada no caso — correções pequenas e frequentes, nascidas de relatórios — com o guardrail que o caso não precisou declarar e que a constituição exige: notas em CHANGELOG/GitHub Releases, tag semver e `verify.sh` executado antes do merge. Não se adota a promessa de hotfix cego nem a resposta integral a reviews. O canal CI→`gh` já existe; a mudança é de disciplina, não de infraestrutura. Score de reviews e números de vendas permanecem observações de terceiros `[NV]`, nunca critério de verde.

## Out of Scope (for the recommended option)

- Promessa de prazo/SLA para hotfixes.
- Steam page, App ID, Update Visibility Rounds ou publicação em loja.
- Responder toda review ou pedir avaliação dentro do jogo.
- Publicar sem saída executada de `verify.sh` e sem notas versionadas.
- Alterações em gates, CI, simulação ou constituição; uso de vendas/score como prova local.

## Assumptions to Validate

- O fluxo CI verde → merge via `gh` continua exigindo a saída local de `verify.sh` como evidência (conferir no ato do merge).
- Tags e notas no GitHub Releases são suficientes como CHANGELOG versionado, sem segundo arquivo a manter.
- [NEEDS CLARIFICATION: responsável por tag/notas e limite de tempo para triagem de reviews.]
- [NEEDS CLARIFICATION: CHANGELOG único ou notas por release (decisão de formato, não de canal).]

## Nota de fronteira

Nove updates em três dias são um fato sobre outro projeto, não uma meta de cadência. O que se exporta é o comportamento — patch pequeno, origem no relato, nota pública — ancorado no gate local que a Insurgência já possui.
