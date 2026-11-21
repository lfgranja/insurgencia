# Problem Definition: Demo no itch.io

- **Slug**: sb-16-demo-itch-io
- **Created**: 2026-11-21
- **Inputs used**: intake.md + research.md (steam-gtm, alternativas, fit-local)

## Problem Statement

A Insurgência não tem hoje um artefato executável e inspecionável, versionado e publicado, que qualquer terceiro possa baixar, rodar e conferir: o `insurgencia-sim` é determinístico e headless, mas não possui ponto de entrada de demonstração nem distribuição. Ao mesmo tempo, a narrativa do vídeo atribui o sucesso do caso estudado a "uma demo grátis no itch.io". Copiar apenas o canal, sem o mecanismo (seed/comando reprodutível, tag, checksum, changelog), repetiria o erro de tratar um outlier como receita e criaria dependência proprietary no caminho crítico — exatamente o que a constituição IV veta. Este problema é concretamente local: falta uma ponte entre a simulação já auditável e uma publicação verificável.

## Affected Users & Stakeholders

- **Users**: engenheiros sem estúdio (GDD §3.2), que precisam ver a tese rodando antes de confiar nela — [source: fit-local evidência local GDD §3.2]
- **Users**: quem recebe um link de demo e precisa reproduzir o mesmo resultado sem Steam, SDK ou conta — [source: alternativas ALT-8/9]
- **Stakeholders**: mantenedores do gate `verify.sh`, que devem continuar sendo a autoridade (constituição I/VI) — [source: fit-local achado 6]
- **Stakeholders**: a SE, que hoje trabalha com janelas minimizadas e não pode depender de um binário que abra janela — [source: fit-local evidência `AGENTS.md`]

## Goals

- Publicar uma demonstração curta e determinística derivada apenas de `insurgencia_sim`, sem `DefaultPlugins` e sem App ID.
- Fazer o exemplo passar no gate existente (`--all-targets`) sem novo job de CI.
- Versionar a publicação: tag semver, checksum, changelog datado e espelho em canal soberano (GitHub Releases primário, itch.io como descoberta).
- Tornar a execução reproduceível por terceiros com um único comando e saída idêntica.

## Non-Goals

- Criar Steam page, App ID ou integrar Steamworks SDK no caminho crítico.
- Transformar itch.io em fonte única de progresso, partida ou identidade.
- Perseguir meta de wishlists ou copiar os números do caso (100k/3 dias) como referência.
- Abrir janela gráfica, mesmo minimizada, durante o trabalho da SE.
- Alterar a simulação, os gates ou as regras da constituição.

## Success Metrics

- `crates/sim/examples/demo_deterministica.rs` compilado e coberto por `verify.sh` (baseline: arquivo inexistente).
- Duas execuções do exemplo, em máquinas distintas, produzem o mesmo `hash_estado()` para a mesma seed (baseline: não medido).
- Uma release versionada com binário/artefato + checksum + notas publicada (baseline: nenhuma).
- Downloads da release via API do GitHub e execuções do exemplo contabilizados como sinal de interesse (baseline: zero; GDD §17 mede verificação da tese, não wishlists).
- Nenhum serviço externo no caminho crítico da demo (baseline: satisfeito pela sim headless).

## Cost of Inaction

Sem demo versionada, a equipe não tem como mostrar nem falsificar a própria tese: a narrativa GTM permanece emprestada de um outlier, a proveniência do código não fica ancorada em releases datadas (fraca contra clones) e qualquer publicação futura em loja começará sem artefato validado. O custo não é "não ter um vídeo bonito", é perder o único difusor barato e auditável que o repo já pode produzir hoje.

## Open Questions

- [NEEDS CLARIFICATION: qual sessão/cenário mínimo da sim serve como demo sem expor detalhes ainda instáveis do GDD?]
- [NEEDS CLARIFICATION: a conta itch.io será operada pela SE ou o espelho fica só no GitHub Releases até decisão de marca?]
- [NEEDS CLARIFICATION: qual seed fixa e qual formato de saída (hash por tick) serão congelados como contrato da demo?]
