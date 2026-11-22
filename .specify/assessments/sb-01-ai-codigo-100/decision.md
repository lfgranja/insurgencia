# Decision — Código 100% IA

- **Slug**: sb-01-ai-codigo-100
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

| Critério | Avaliação | Justificativa (1 linha) |
|---|---|---|
| Validade do problema | strong | Risco real e documentado de comprehension debt se "100%" virar meta de processo. |
| Força da evidência | strong | Posts originais + RCTs (Anthropic, METR, GitHub); "100%" permanece infalsificável [NV]. |
| Valor vs. inação | adequate | Preserva auditabilidade sem custo novo; evita dívida de compreensão. |
| Viabilidade/Appetite | strong | `verify.sh` e gates já vigentes; basta explicitar a fronteira. |
| Alinhamento estratégico | strong | Constituição I (determinismo), IV (soberania) e VI (verificação honesta). |
| Postura de risco | strong | Mudança de política reversível; nenhuma dependência de provedor adicionada. |

## Racional
A tese do vídeo é testemunho autodeclarado, não evidência auditável [NV]. O próprio autor relata delegação feature a feature com verificação humana e ~1 ano de trabalho, não geração autônoma. Na Insurgência, a autoria pode ser mista, mas a autoridade é determinística: `verify.sh` (check → clippy `-D warnings` → nextest → fmt) e um humano que consiga explicar o módulo. Nenhum KPI de percentual de código IA; ADR-004 (Bevy+Lightyear) e `#![forbid(unsafe_code)]`/`#![deny(clippy::pedantic)]` permanecem.

## Condição/responsável
Responsável: SE (Garry Seto) e o agente que integra cada diff. Condição: nenhuma meta "100% IA" em spec/KPI; todo diff com gate verde e explicação do módulo antes de merge; ferramentas IA opcionais, saída sempre informativa.

## Handoff
Registrar a fronteira "IA propõe, gate + humano dispõem" na norma de código/SDD; exigir `verify.sh` ALL STAGES OK por diff; manter `contratos`/`sim` livres de SDK de provedor; preservar `--minimizado` e a proibição de `.unwrap()`/`.expect()`.
