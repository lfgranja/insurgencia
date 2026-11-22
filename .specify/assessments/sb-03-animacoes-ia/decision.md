# Decision — Animações IA (sb-03)

- **Slug**: sb-03-animacoes-ia
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | Avatar hoje estático é fato local; a atribuição das animações à IA é hipótese [NV]. |
| Força da evidência | strong | ToS/docs PixelLab, SD-πXL, NoiseShift e forense/copyright são primárias lidas; fit local lido diretamente. |
| Valor vs. inação | adequate | Animação por estado agrega legibilidade, mas não é pré-requisito do caminho crítico. |
| Viabilidade/Appetite | strong | `avancar_tick`/`hash_estado`/tempo já existem; falta tabela de frames no adaptador. |
| Alinhamento estratégico | strong | Derivar animação do estado reforça determinismo (I), soberania (IV) e verificação honesta (VI). |
| Postura de risco | adequate | Risco de dependência remota e copyright mitigado por IA apenas offline + inventário. |

## Racional
ADAPT porque "animações geradas por IA" descreve uma forma (modelo remoto, saída irreproduzível) que o Insurgência não pode adotar sem violar IV/VI, mas a intenção — avatar animado — é legítima. A adaptação troca a origem por frames versionados derivados do estado da simulação, com sprites locais inventariados. Nenhuma IA em runtime, nenhuma dependência externa, nenhuma troca de engine (ADR-004 intacto); a saída de LLM continua informativa e os gates determinísticos mandam.

## Condição/responsável
Responsável: SE com o fluxo SDD, no seam `Mundo → apresentação`. Toda animação entra como tabela de frames versionada e passa por `verify.sh` + 5 gates; inspeção visual, se houver, sob `--minimizado`. Modelo/serviço de IA, se usado na autoria, deve ter prompt/seed/ToS registrados antes de versionar (constituição VI).

## Handoff
- **Problema**: avatar estático e ausência de tabelas de animação determinísticas, com risco de importar geração remota opaca.
- **Abordagem**: frames indexados por `hash_estado`/tick/tempo no adaptador de apresentação; sprites locais (determinísticos ou CC0/humano inventariado).
- **In scope**: tabela de frames versionada consumindo o estado da sim; inventário de proveniência dos sprites; verificação por mesmo seed/tick → mesmo frame.
- **Out of scope**: chamada a modelo em runtime, PixelLab/MCP no caminho crítico, "100% IA", EXIF/C2PA como fonte da verdade, troca de engine.
- **Métricas**: reprodução idêntica de frame para a mesma semente/tick; `verify.sh` ALL STAGES OK + 5/5 gates; nenhuma dependência remota **[NV até verificação ao vivo]**.
- **Perguntas remanescentes**: quais estados do `sim` devem dirigir cada animação? O cliente já ligou `avancar_tick` (pré-requisito compartilhado com sb-15) [NV]? Quais animações são realmente alegadas no vídeo [NV]?
