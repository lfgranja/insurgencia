# Decision — Pixel art mascara artefatos (sb-13)

- **Slug**: sb-13-pixel-art-mascara-artefatos
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | A tese do vídeo é especulação [NV]; o problema real local é documentar a estética por razões próprias. |
| Força da evidência | strong | Forense 32×32 a 98,65%, NoiseShift, SD-πXL e código local são primárias lidas. |
| Valor vs. inação | adequate | Não exige trabalho novo; evita o custo de adotar uma justificativa infalsificável. |
| Viabilidade/Appetite | strong | Low-poly procedural (`malha`/`achatar`, `CORES_TERRENO`) já está implementado. |
| Alinhamento estratégico | strong | Recusar ocultação reforça verificação honesta (VI) e proveniência (`ast-20/21`, `doc-11/12`). |
| Postura de risco | adequate | Risco de regresso à justificativa de máscara mitigado por documentação explícita e gates. |

## Racional
ADAPT porque a intuição por trás da tese — usar poucos detalhes — produz de fato uma estética funcional, mas a **justificativa** de "mascarar artefatos de IA" não sobrevive ao escrutínio: é opinião do youtuber, é contrariada por forense a 32×32 e por NoiseShift/SD-πXL, e é infalsificável, ferindo a constituição VI. Mantemos o low-poly facetado por legibilidade, custo e determinismo, com inspeção e inventário — nunca como ocultação. Nenhuma dependência externa, nenhuma troca de engine (ADR-004 intacto); gates determinísticos permanecem a autoridade sobre a qualidade.

## Condição/responsável
Responsável: SE com o fluxo SDD. Toda documentação da estética deve citar legibilidade/determinismo, nunca "esconder falhas"; qualquer arte passa por inventário de proveniência e por `verify.sh` + 5 gates antes de versionar. Inspeção visual sob `--minimizado`; saída de LLM permanece informativa (constituição VI).

## Handoff
- **Problema**: risco de adotar uma justificativa infalsificável para a estética low-poly, trocando verificação por ocultação.
- **Abordagem**: manter facetas procedurais documentadas por legibilidade/determinismo; rejeitar explicitamente "mascarar artefatos".
- **In scope**: registrar as razões da estética na documentação do cliente; inventário de proveniência de qualquer arte; verificação por `hash_estado` e gates.
- **Out of scope**: justificativa de ocultação, "100% IA", usar baixa resolução como escudo de detectabilidade, dependência de IA remota, troca de engine.
- **Métricas**: estética reproduzível por mesmo seed/estado; `verify.sh` ALL STAGES OK + 5/5 gates; nenhuma alegação de "máscara" na documentação **[NV até verificação ao vivo]**.
- **Perguntas remanescentes**: a estética low-poly está documentada com as razões corretas em `docs/GDD`? Existe algum artefato visível que motive revisão (inspeção sob `--minimizado`) [NV]? A tese do vídeo foi lida apenas na transcrição — confirmar fonte se necessário [NV]?
