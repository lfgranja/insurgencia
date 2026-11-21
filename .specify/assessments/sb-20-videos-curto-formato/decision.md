# Decision — Vídeos de curto formato (sb-20)

- **Slug**: sb-20-videos-curto-formato
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | "Shorts bem editados" não é dor local; o que falta é evidência visual reproduzível do MVP. |
| Força da evidência | adequate | Dados contraditórios e de fontes com interesse; primárias são só as regras de plataforma e as ferramentas de captura. |
| Valor vs. inação | strong | Clipes a partir da sim são reutilizáveis em devlogs/README/releases, com custo marginal baixo. |
| Viabilidade/Appetite | strong | Sim headless + HUD em `format!` + FFmpeg `-bitexact`; nenhuma conta necessária para produzir. |
| Alinhamento estratégico | strong | Captura rotulada preserva soberania (IV) e verificação honesta (VI); fork como feature, não marketing de clones. |
| Postura de risco | adequate | Risco de otimizar views e de disclosure; mitigado por UTM/72 h, rótulo e captura da sim real. |

## Racional
ADAPT: a forma do vídeo muda. Em vez de editar promoção com IA (premissa inferida, não observada), o projeto produz clipes a partir de quadros determinísticos da simulação real, com semente, comando e hash publicados — sem abrir janela normal e sem serviço externo no caminho de verificação. Short-form é teste mensurável por conversão UTM em 72 h, não meta de vaidade. Postar em plataforma fica **DEFER** até existir conta/canal; a produção do artefato é imediata. Nenhum código de engine é tocado (ADR-004), nenhum hack de IA é usado como prova de sucesso alheio.

## Condição/responsável
Responsável: SE, via fluxo SDD, para o pipeline de captura; publicação depende de conta/canal e de checagem de disclosure (coordenar com DisclosureLegalEvidence). CI verde autoriza merge; saída de LLM é informativa e nunca autoritativa.

## Handoff
- **Problema**: ausência de clipes honestos e reproduzíveis do estado real do jogo; pressão de receita de short-form sem evidência local.
- **Abordagem**: captura determinística (exemplo headless/sim ou `--minimizado`), FFmpeg `-bitexact`, metadados suprimidos, `sha256` + semente + comando anexados; material sintético rotulado; medir por UTM em 72 h.
- **In scope**: pipeline local de captura, devlogs versionados, GIF/APNG derivado da sim, rótulos de proveniência.
- **Out of scope**: postar em qualquer plataforma até existir conta (DEFER), voz/stills IA, campanha paga, otimização por views, perseguição de clones.
- **Métricas**: reprodução byte-exata do clipe para a mesma semente; `sha256` publicado; conversões por UTM em 72 h quando houver conta; 34 testes + 5 gates PASS **[NV até execução ao vivo]**.
- **Perguntas remanescentes**: qual canal/conta será usada (se alguma)? O encode bitexact é estável entre versões do FFmpeg [NV]? Como rotular cada quadro/voz conforme a plataforma escolhida?
