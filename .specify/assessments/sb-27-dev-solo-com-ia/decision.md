# Decision — Dev solo com IA

- **Slug**: sb-27-dev-solo-com-ia
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

| Critério | Avaliação | Justificativa (1 linha) |
|---|---|---|
| Validade do problema | strong | GDD §3.2 define engenheiros sem estúdio como público; barreira real de entrada. |
| Força da evidência | adequate | Caso Slotbound é n=1 e autodeclarado [NV]; risco de comprehension debt tem RCT. |
| Valor vs. inação | adequate | Inclusão alinhada à tese, condicionada a checagem; sem promessa de 100K. |
| Viabilidade/Appetite | strong | Ferramentas open-source opcionais e `verify.sh` já disponíveis. |
| Alinhamento estratégico | strong | IV (sem provedor), III (docs append-only), V (decisão humana). |
| Postura de risco | adequate | Mitigado por comprehension check; risco residual de delegação total. |

## Racional
O relato de sucesso é n=1, com 6–12 meses de trabalho, assets comprados, nenhum teste automatizado e números de vendas autodeclarados [NV]. O censo de 53.597 jogos mostra que a amostra é sobrevivente. Na Insurgência, IA é via de entrada legítima para o público do GDD §3.2, mas somente com "generation-then-comprehension": o iniciante explica o módulo, passa por `verify.sh` e registra decisões em docs append-only (III). Nenhuma dependência de provedor (IV) e nenhuma promessa de 100K/3 dias (ver sb-28).

## Condição/responsável
Responsável: SE e o iniciante integrador, com revisor separado (sb-08). Condição: checagem de compreensão por diff (explicar o módulo) + `verify.sh` verde; ferramentas IA opcionais e open-source; decisões registradas em docs append-only.

## Handoff
Registrar no fluxo SDD que dev iniciante é bem-vindo com IA, condicionado a comprehension check; vincular ao handoff de sb-08 (revisor separado + `proptest`); manter ADR-004, `--minimizado` e a proibição de `.unwrap()`/`.expect()`.
