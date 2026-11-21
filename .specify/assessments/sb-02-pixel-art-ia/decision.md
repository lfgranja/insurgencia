# Decision — Pixel art IA (sb-02)

- **Slug**: sb-02-pixel-art-ia
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | A afirmação do vídeo é hipótese [NV], mas a ausência de inventário de proveniência é risco real e local. |
| Força da evidência | strong | USCO/D.C. Circuit, forense 32×32, NoiseShift/SD-πXL, estudo C2PA e ToS do PixelLab são primárias lidas. |
| Valor vs. inação | adequate | A estética procedural já existe; o ganho é disciplina de rastreabilidade, não arte nova. |
| Viabilidade/Appetite | strong | Gerador determinístico local e `COPYRIGHT.txt` já estão no repo; custo baixo. |
| Alinhamento estratégico | strong | Reforça soberania (IV) e verificação honesta (VI); coerente com `ast-20/21`, `doc-11/12`. |
| Postura de risco | adequate | Risco jurídico/ToS mitigado por CC0/humano + ausência de serviço externo no caminho crítico. |

## Racional
ADAPT porque o valor estético da pixel art é legítimo, mas a forma alegada no vídeo (geração remota como fonte majoritária) não é: copyright de saída IA é incerto, ferramentas como PixelLab impõem dependência e proveniência embutida não sobrevive ao pipeline. A adaptação mantém a estética e troca a origem por gerador determinístico local/CC0/humano com inventário verificado. Nenhuma dependência externa, nenhuma troca de engine (ADR-004 intacta) e nenhuma meta de "100%" — gates determinísticos permanecem a autoridade.

## Condição/responsável
Responsável: SE com o fluxo SDD. Toda arte nova só entra após registro de proveniência e aprovação via `verify.sh` + 5 gates; saída de LLM é informativa. Nenhuma janela normal (regra `--minimizado`); checagem de licença no ato do download de qualquer pack CC0.

## Handoff
- **Problema**: ausência de fonte determinística e de inventário de proveniência para sprites, com risco de dependência externa e titularidade incerta.
- **Abordagem**: gerador local seed+paleta+hash ou CC0/humano contratado; inventário em `docs/evidencias/` e `COPYRIGHT.txt`.
- **In scope**: estender `malha`/`achatar`/`CORES_TERRENO` a sprites; registrar licença SPDX, ferramenta, seed/prompt, hash e data; verificação por `hash_estado` reprodutível.
- **Out of scope**: PixelLab/MCP no caminho crítico, alegar "100% IA", geração em runtime, EXIF/C2PA como fonte da verdade, troca de engine.
- **Métricas**: mesmo seed + paleta → mesmo hash; `verify.sh` ALL STAGES OK + 5/5 gates; nenhuma dependência remota no build **[NV até verificação ao vivo]**.
- **Perguntas remanescentes**: qual subconjunto de sprites realmente precisa de arte dedicada versus facetas procedurais? O inventário existe em `docs/evidencias/` com o formato esperado? A redação viva da página da Steam foi auditada [NV]?
