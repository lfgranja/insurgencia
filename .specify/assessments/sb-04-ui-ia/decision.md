# Decision — UI IA (sb-04)

- **Slug**: sb-04-ui-ia
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | A afirmação do vídeo é hipótese [NV]; o HUD local já resolve a função sem IA. |
| Força da evidência | strong | Código local lido diretamente; políticas Steam/itch.io, USCO/D.C. Circuit e estudo C2PA são primárias. |
| Valor vs. inação | weak | Nenhum ganho funcional: `format!` determinístico já atende; valor é só evitar regressão. |
| Viabilidade/Appetite | strong | Manter e versionar temas é trivial; não exige sistema novo. |
| Alinhamento estratégico | strong | Preserva determinismo (I), soberania (IV) e verificação honesta (VI). |
| Postura de risco | adequate | Risco de disclosure/copyright evitado ao não gerar UI em runtime e ao isentar algoritmo autocontido. |

## Racional
ADAPT porque a interface "gerada por IA" não agrega valor onde o HUD já é puro `format!` determinístico e auditável: adotá-la em runtime traria dependência externa, titularidade incerta e disclosure sem contrapartida. A adaptação mantém o HUD textual, concentra cores/strings em temas versionados e admite IA apenas como referência offline cujo resultado revisado é versionado e atribuído. Nenhuma dependência externa, nenhuma troca de engine (ADR-004 intacto); saída de LLM é informativa e gates determinísticos permanecem a autoridade.

## Condição/responsável
Responsável: SE com o fluxo SDD, no adaptador de apresentação (`atualizar_texto`). Toda mudança de HUD passa por `verify.sh` + 5 gates; inspeção visual sob `--minimizado`. Se IA for usada como referência de layout, o resultado só entra após revisão humana e inventário de proveniência (constituição VI).

## Handoff
- **Problema**: risco de substituir um HUD determinístico e isento de disclosure por UI gerada por modelo, opaca e dependente de serviço externo.
- **Abordagem**: HUD em `format!` sobre estado da sim + temas versionados; IA apenas como referência offline, nunca em runtime.
- **In scope**: extrair cores/strings do HUD para tabela de temas versionada; verificar texto sob `--minimizado`; inventário se houver resultado inspirado por IA.
- **Out of scope**: geração de UI em runtime, modelo remoto no caminho crítico, "100% IA", disclosure generativo para algoritmo autocontido, troca de engine.
- **Métricas**: HUD reproduzível para o mesmo estado; `verify.sh` ALL STAGES OK + 5/5 gates; nenhuma dependência remota **[NV até verificação ao vivo]**.
- **Perguntas remanescentes**: quais cores/strings merecem virar tema versionado? A afirmação do intake consta no post original ou só em material promocional [NV]? Houve de fato revisão humana na UI alegada [NV]?
