# Problem Definition: Runway de wishlists

- **Slug**: sb-18-wishlist-runway
- **Created**: 2026-11-21
- **Inputs used**: intake.md + research.md (steam-gtm, alternativas, fit-local)

## Problem Statement

O resumo do vídeo propõe importar uma sequência de números — 591 wishlists por quatro meses, 40 mil em duas semanas, 100 mil antes do launch, ~4,5 meses de marketing — como se fosse um plano de runway para a Insurgência. Três desses quatro pontos não têm fonte primária (`[NV]`: "591 em março" era ~590 em julho; "40k em 2 semanas" é documentado como 50k em 17 dias; "4,5 meses" não foi localizado). Mesmo o número confirmado é inútil como meta local: não existe Steam page cujas wishlists possam ser medidas, e a própria Valve afirma que wishlists não são fator de visibilidade nem permitem prever vendas. O problema real é planejar com números emprestados e não falsificáveis, quando o projeto já tem métricas de verificação da tese (GDD §17) e distribuição de custo zero.

## Affected Users & Stakeholders

- **Users**: a SE e o time, que precisam de um horizonte de execução honesto até o próximo marco — [source: GDD §17 via fit-local]
- **Users**: engenheiros sem estúdio (GDD §3.2), que não têm receita de outlier para financiar 4,5 meses de campanha — [source: alternativas ALT-8/9]
- **Stakeholders**: autoridade de gate, que não pode aceitar "wishlist subiu" como prova de valor sem comando executado (constituição VI) — [source: fit-local achado 6]
- **Stakeholders**: quem decide orçamento, hoje sem registro local de runway próprio — [source: steam-gtm C10 `[NV]`]

## Goals

- Substituir números emprestados por um runway registrado localmente: orçamento, custo de CI/arte, prazo até o próximo marco verificável.
- Estabelecer métricas de interesse sob controle do projeto (downloads de releases, execuções da demo determinística), não wishlists de terceiros.
- Explicitar a data de reavaliação e as condições sob as quais qualquer plataforma externa poderia ser considerada.
- Rotular `[NV]` tudo o que não foi verificado, preservando honestidade de evidência.

## Non-Goals

- Adotar 4,5 meses, 591/40k/100k ou qualquer número do caso como meta.
- Criar Steam page ou perseguir wishlists para "ativar algoritmo" (mecânica negada pela Valve).
- Transformar estimativas de conversão de terceiros em previsão de receita.
- Declarar verde financeiro, de CI remoto ou de vendas sem comando executado e fonte primária.
- Alterar GDD §17, gates ou a constituição; redefinir o que conta como sucesso do projeto.

## Success Metrics

- Runway próprio registrado com orçamento, custos e marco seguinte (baseline: inexistente; única referência disponível é `[NV]`).
- Zero metas de wishlist em documentos de planejamento (baseline: proposta importada do vídeo).
- Downloads de releases via API do GitHub e execuções do exemplo demo contabilizados como sinal de interesse (baseline: zero).
- Reavaliação datada antes de qualquer decisão de plataforma paga (baseline: sem data).
- Nenhum número `[NV]` citado como evidência de decisão (baseline: quatro números do resumo, três `[NV]`).

## Cost of Inaction

Manter os números emprestados cria metas que o repo não consegue medir e incentiva serviços externos como "solução" para um problema de evidência. O projeto passa a otimizar wishlists — que a Valve diz não contarem — em vez de verificação da tese; o runway vira ficção financeira de outro dev. O custo concreto é decisão mal informada, não a ausência de uma planilha.

## Open Questions

- [NEEDS CLARIFICATION: qual orçamento real e qual prazo até o próximo marco do plano de trabalho da SE?]
- [NEEDS CLARIFICATION: quais contadores locais (GitHub API, execuções da demo) serão registrados como linha de base?]
- [NEEDS CLARIFICATION: em que data e sob quais critérios a decisão de plataforma paga é reaberta?]
