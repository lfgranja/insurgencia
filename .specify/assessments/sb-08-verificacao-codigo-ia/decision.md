# Decision — Verificação determinística de código IA

- **Slug**: sb-08-verificacao-codigo-ia
- **Decided**: 2026-11-21
- **Veredito**: ADOPT

| Critério | Avaliação | Justificativa (1 linha) |
|---|---|---|
| Validade do problema | strong | Comprehension debt é o risco melhor documentado e não é medido por DORA/cobertura. |
| Força da evidência | strong | RCT Anthropic + CRUST-Bench + PGS + `verify.sh` local lido diretamente. |
| Valor vs. inação | strong | Maior alavanca local (27,0); transforma saída informativa em evidência auditável. |
| Viabilidade/Appetite | strong | Gates e sim headless já existem; custo baixo e incremental. |
| Alinhamento estratégico | strong | Constituição I/VI (determinismo e verificação honesta) e III (decisões documentadas). |
| Postura de risco | strong | Reforço aditivo; nenhuma dependência de provedor, revisão sem poder de alterar código. |

## Racional
`verify.sh` (check → clippy `-D warnings` → nextest → fmt, `--all-targets`) já é o oráculo autoritativo; saída de LLM é informativa (I/VI). O que falta é fechar o elo humano e de propriedades: revisor separado do autor (evitando autoengano), `proptest` em `contratos`/`sim` e teste de determinismo via `hash_estado`. Isso casa com CRUST-Bench (compilador como oráculo barato) e com o padrão "gerar e explicar" (86% de compreensão). Os 34 testes atuais são base, não meta.

## Condição/responsável
Responsável: SE e o agente revisor (distinto do autor do diff). Condição: todo diff com `verify.sh` ALL STAGES OK + explicação do módulo; revisor sob `--minimizado`, sem permissão para alterar código; `proptest` e invariantes de determinismo aplicados a `contratos`/`sim`.

## Handoff
Adicionar checklist de comprehension debt ao review; designar agente revisor separado; escrever `proptest` para invariantes de domínio e teste `hash_estado` (mesma seed → mesmo estado); manter ADR-004, `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]` e a proibição de `.unwrap()`/`.expect()`.
