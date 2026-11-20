# Problem Definition: Godot headless benchmark

- **Slug**: godot-headless-benchmark
- **Created**: 2026-11-20
- **Inputs used**: intake.md | research.md | pedido do mantenedor 2026-11-20

## Problem Statement

O alvo não tem engine instalada nem qualquer medição de desempenho versionada, de modo que não há como provar que o binário usado provém de release oficial íntegra nem comparar o custo de um tick de simulação antes e depois de mudanças futuras. Sem identidade de release, par de checksums e medição com data/hora, a adoção da engine repousaria em confiança informal e o desempenho da simulação continuaria sendo uma incógnita — com o risco adicional de uma carga sintética headless ser lida como prova de uma partida multiplayer real.

## Affected Users & Stakeholders

- **Users**: mantenedor que instala e opera a engine — precisa de um binário cuja origem e integridade possam ser demonstradas; quem for implementar a simulação de jogo — precisa de uma linha de base de custo por tick para detectar regressões.
- **Stakeholders**: auditores do hub, que exigem comando, saída e data/hora antes de aceitar qualquer afirmação como verde; a governança do projeto, cuja constituição exige determinismo (I), ausência de dependência proprietária no caminho crítico (IV) e verificação honesta (VI).

## Goals

- Fixar a identidade da release oficial usada como base da instalação.
- Provar a integridade do artefato por comparação entre checksum esperado e calculado, com data/hora do resultado.
- Registrar a identidade reportada pelo próprio binário instalado.
- Medir o custo por tick da carga sintética headless já versionada, com estatísticas de dispersão.
- Julgar a medição contra uma referência explícita e registrá-la como linha de base, não como garantia.
- Impedir que a carga sintética seja descrita como benchmark 5v5 real.
- Entregar o planejamento em dez artefatos rastreáveis, com requisitos funcionais testáveis e critérios de sucesso mensuráveis.

## Non-Goals

- Reclassificar a medição como benchmark de partida 5v5 real ou como prova de desempenho de jogo multiplayer.
- Medir renderização, rede, serviços externos, inteligência artificial de jogadores ou qualquer carga além do laço sintético definido em `tests/benchmark/headless_tick.gd`.
- Definir arquitetura de produto, regras de jogo, rampa de conteúdo ou modelo de dados de partida.
- Transformar a mediana medida em SLA, garantia de hardware ou meta de produto.
- Executar gates, stage, commit, abertura de PR, merge ou reescrita de histórico nesta etapa de planejamento.
- Escrever portarias posteriores a tasks (implement, converge, validate, release) neste conjunto de artefatos.

## Success Metrics

- Identidade da release: 1 release oficial identificada como `4.7.2-stable`, rastreável à consulta da API do repositório upstream (baseline: nenhuma; meta 1).
- Integridade: 1 par de checksums esperado/calculado idêntico, resultando em `CHECKSUM_OK` em 2026-11-20T00:43:52Z; 0 divergências (baseline: 0; meta 1 par, 0 divergências).
- Binário: 1 string de versão reportada pelo executável, `4.7.2.stable.official.ed1daf0bf`, registrada com data/hora 2026-11-20T00:44:11Z (baseline: nenhuma; meta 1).
- Carga: 1 execução de 600 ticks × 64 entidades, sem outra configuração de carga (baseline: nenhuma medição; meta 1 execução com esses parâmetros).
- Medição: mediana de 37,48 µs/tick em 7 amostras, com mínimo 22,71 e máximo 42,95 registrados em 2026-11-20T00:44:48Z (baseline: nenhuma; meta 1 medição completa).
- Comparação: mediana de 0,03748 ms/tick abaixo da referência de 16,7 ms/tick, com 1 registro de conformidade (baseline: referência definida no pedido; meta conforme).
- Rotulagem: 100% das descrições identificadas como simulação headless sintética; 0 afirmações de benchmark 5v5 real fora de contextualização negativa (baseline: 0; meta 100% e 0).
- Artefatos: 10 de 10 presentes em `specs/006-godot-headless-benchmark/`, sem seção vazia e sem marcador de pendência, com checklist `[ ]` e tarefas `T###` sem portarias posteriores (baseline: 1 template de spec; meta 10 de 10).

## Cost of Inaction

Sem release identificada e sem verificação de checksum, a instalação da engine dependeria de binário de proveniência não demonstrada, contrariando o princípio de verificação honesta. Sem medição versionada, não haveria linha de base para julgar regressões futuras no custo da simulação. Sem rotulagem explícita, uma medição de laço sintético poderia ser citada como desempenho de partida real e levar a decisões de design inválidas. O custo é a perda de auditabilidade e de comparabilidade, não apenas a ausência de um número.

## Open Questions

- Pergunta 1 — como classificar a carga para não induzir conclusão de partida real? Respondida em `spec.md` / `## Clarifications` / Q1: como simulação headless sintética, com vedação explícita à reclassificação.
- Pergunta 2 — qual referência usar e quais estatísticas registrar? Respondida em `spec.md` / `## Clarifications` / Q2: referência de 16,7 ms/tick, com mediana, mínimo, máximo e contagem de amostras.
- Pergunta 3 — como tratar a evidência já obtida pelo orquestrador nesta etapa? Respondida em `spec.md` / `## Clarifications` / Q3: como fatos citados com origem e carimbo, sem reexecução e sem portarias posteriores a tasks.
