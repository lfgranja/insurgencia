# Decision — HUD com acento e fonte livre

**Data**: 2026-11-22T21:17Z · **R1**: `go` · **R3**: 0 clarificações

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Pedido explícito do SWE (“Agora quero o Ê sim”); HUD atual com grafia incorreta confirmada por captura e código. |
| Força da evidência | strong | `main.rs:458,555` lidos; captura Xvfb; TTF e `fc-match` para U+00CA/U+2014/U+00B7; licença OFL 1.1 integral. |
| Valor vs. inação | strong | Restaura identidade do produto sem tofu; inação contraria decisão do usuário. |
| Viabilidade | strong | Fonte já instalada no sistema; `AssetServer`/`TextFont` existem; nenhuma dependência Rust nova. |
| Alinhamento estratégico | strong | Soberania e rastreabilidade: asset livre, hash e licença versionados; histórico 029 preservado. |
| Postura de risco | strong | Mudança local e reversível; risco tipográfico mitigado pela captura em Xvfb e pelo roteiro de testes. |

**Veredito: go** — Opção 1 (embarcar Noto Sans Regular OFL 1.1 e restaurar `Ê`, `—`, `·`). 80/20: impacto 5 × confiança 4 × facilidade 3 × segurança 4 ÷ 625 = 240/625, com a escolha do usuário como critério prioritário.

Condições: (1) fonte, hash e licença rastreados; (2) captura Xvfb com os três glifos e sem `▯`; (3) `verify.sh` 41/41 e 5 gates PASS **[NV]**; (4) regra da janela preservada; (5) 14 artefatos SDD na ordem normativa, `analyze` antes de `tasks`, R4 antes de R7, nada executado por este agente. Handoff: agente de código aplica a implementação; orquestrador executa roteiro, gates e promoção.
