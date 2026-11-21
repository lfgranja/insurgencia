# Problem — Vídeos de curto formato (sb-20)

**Data**: 2026-11-21

O intake repete a receita genérica "vídeos curtos bem editados, incluindo histórico de desenvolvimento e clones". Para o Insurgência, o problema não é falta de edição: é a ausência de **evidência visual reproduzível e honesta** do estado real da simulação. Os dados de short-form são contraditórios e vêm de fontes com interesse comercial; gravar a janela normal viola a regra SE (`--minimizado` sempre), e apresentar stills/voz sintéticos como jogo real feriria a verificação honesta (art. VI). Clones, além disso, são externos e a tese FOSS trata fork como feature — não como tropa de marketing.

## Sucesso observável
- Clipes gerados de quadros determinísticos (semente + commit registrados), com `sha256` publicado, sem janela normal.
- Todo material sintético, se existir, está rotulado; nenhum quadro é passado por jogo real.
- Cada publicação tem link UTM rastreável e decisão medida em 72 h por conversões, não por views.
- `verify.sh` ALL STAGES OK (34 testes) e 5/5 gates PASS; publicação em plataforma só após existir conta **[NV]**.

## Custo de não agir
Sem captura determinística, o projeto perde a chance de reutilizar o mesmo artefato em devlogs, README, ADRs e releases — e fica tentado a comprar a narrativa de "viralização" de fornecedores. O custo de produzir o clipe é baixo; o custo de perseguir views como KPI é desvio de foco e risco de disclosure.
