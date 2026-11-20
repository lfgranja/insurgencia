# Problem Definition: Substituir marcadores `[NV]` de pesquisa por fatos verificados ao vivo

- **Slug**: verifica-nv-externa
- **Created**: 2026-11-19
- **Inputs used**: intake.md | research.md | user input (pedido do mantenedor)

## Problem Statement

O mantenedor e o auditor do hub leem, nos documentos integrados de avaliação de engines e de stack, rótulos `[NV]` sobre fatos que já foram verificados ao vivo em 19/11/2026 e registrados em três arquivos de evidência, sem que essa evidência tenha sido aplicada ao texto; ao mesmo tempo, uma substituição ingênua apagaria salvaguardas metodológicas legítimas e converteria evidência documental em alegação de benchmark. O problema é de confiabilidade e rastreabilidade documental, não de execução de engines.

## Affected Users & Stakeholders

- **Users**: mantenedor do hub — precisa que o documento reflita o que já se sabe, com citação auditável, sem reler três arquivos de evidência para cada afirmação.
- **Users**: auditor/revisor de governança — precisa contar marcadores resolvidos e verificar que nenhum fato ficou sem duas fontes e nenhum `[NV]` metodológico foi apagado.
- **Stakeholders**: maintainer `lfgranja` — decidiu a resolução de `[NV]` em 19/11/2026 e pediu a aplicação com ressalvas.
- **Stakeholders**: leitores das avaliações (time de produto/engenharia) — usam as notas e recomendações para escolher stack; uma ressalva perdida pode induzir decisão errada.

## Goals

- Aplicar os 12 fatos resolvíveis (NV-A01–NV-A12) aos documentos de destino, com título, URL, data e pelo menos duas fontes por fato.
- Preservar a ressalva de escopo de cada fato (quatro hipóteses automáticas da marca Godot; natureza documental de Stride #1666; ciclo 0.19 da cadência Lightyear; escopo de download/instalação/uso do Defold).
- Manter integralmente os marcadores de classe B (execução externa) e classe C (salvaguarda metodológica).
- Diferenciar documento integrado (substituição no corpo) de snapshot histórico (nota de superação append-only, sem reescrita).
- Permitir verificação por contagem: 12 fatos resolvidos, 0 fatos sem duas fontes, 0 alegações de benchmark, 0 seções vazias nos artefatos.

## Non-Goals

- Executar builds, protótipos, benchmarks ou qualquer engine; medir custo por tick ou desempenho 5v5+bots.
- Reescrever o corpo de snapshots históricos datados (`docs/avaliacao-engines/pesquisa/**`) ou de histórico em `docs/PROGRESSO.md`, ADRs, `converge.md` e `docs/governance/**`.
- Recalcular notas, índices ponderados, scores ou recomendações das avaliações em razão da substituição.
- Alterar a contagem canônica de classes (A=11, B=92, C=157) registrada em governança.
- Resolver marcadores fora do escopo (reconexão que restaura sessão, hash no handshake, serviços proprietários, CI/merge/proteção de branch, publicação/deploy).
- Rodar gates, fazer stage ou commit; executar esta feature de planejamento além da escrita dos artefatos.
- Buscar novas URLs: toda evidência vem dos três arquivos já coletados em 19/11/2026.

## Success Metrics

- Fatos resolvíveis aplicados: 12 de 12 (baseline: 0 de 12 aplicados aos documentos; meta 12 de 12).
- Fatos substituídos sem pelo menos duas fontes independentes com título, URL e data: 0 (baseline: risco implícito em marcadores sem citação; meta 0).
- Alegações novas de benchmark/build/protótipo/execução de engine: 0 (baseline: 0 nesta rodada; meta 0).
- Marcadores de classe C e B preservados: 100% dos listados em FR-005 e FR-006 (baseline: presentes; meta nenhum removido).
- Snapshots históricos com nota de superação e zero linhas reescritas no corpo: 100% (baseline: sem notas de superação; meta 100%).
- Notas, índices e recomendações alterados pela substituição: 0 (baseline: 0; meta 0).
- Artefatos da feature presentes, sem seção vazia e sem `TBD`/`[NEEDS CLARIFICATION]` pendente: 10 de 10 (baseline: 1 artefato de template; meta 10 de 10).

## Cost of Inaction

Os documentos integrados continuam a marcar como não verificado fatos já provados em fonte primária, forçando cada leitor a consultar os três arquivos de evidência e tornando a avaliação menos auditável; pior, qualquer atualização futura feita sem as ressalvas poderia transformar "issue aberta sem fix registrado" em "falha observada" e "4 versões no ciclo 0.19" em regra geral, contaminando decisões de stack com alegações empíricas não sustentadas. A dívida também deixa a resolução de 19/11/2026 incompleta: o contrato de governança autorizou a conversão, mas o texto integrado ainda não a reflete.

## Open Questions

- Resolvido: sim — a ressalva de Stride #1666 é "sem correção registrada na 4.4.3" (estado `open` após 4.4.3 e nenhum fix registrado na issue ou nas notas de release), documental e não empírica (`research-evidence/stride-fyrox.md:13`).
- Resolvido: sim — `docs/avaliacao-engines/pesquisa/**` é snapshot histórico (nota de superação append-only) e os demais documentos nomeados em FR-008 são integrados (substituição no corpo), conforme `docs/governance/nv-resolution-2026-11-19.md:20-22`.
- Resolvido: sim — "4 versões por ciclo do Bevy" vale apenas para o ciclo 0.19 (0.28.0, 0.29.0, 0.30.0, 0.30.1), não como regra geral (`research-evidence/lightyear-bevy.md:43`).

As três perguntas são respondidas uma a vez em `spec.md` / `## Clarifications` / `### Session 2026-11-19`, com base em `research-evidence/stride-fyrox.md:13`, `docs/governance/nv-resolution-2026-11-19.md:20-22` e `research-evidence/lightyear-bevy.md:43`.
