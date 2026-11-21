# Decision — HUD glyphs ausentes

**Data**: 2026-11-21T04:59Z · **R1**: `go`

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Tofu visível na imagem da SE: nome do jogo corrompido. |
| Força da evidência | strong | Imagem anexada + linhas exatas em `main.rs:458,555`. |
| Valor vs. inação | adequate | Cosmético, mas afeta identidade do produto. |
| Viabilidade | strong | Duas strings; sem assets. |
| Alinhamento estratégico | strong | Soberania (sem asset externo), verificação honesta. |
| Postura de risco | strong | Mudança textual reversível, zero lógica. |

**Veredito**: `go` — Opção 1 (ASCII seguro). R3: 0 clarificações (regra da janela já explícita na interjeição da SE).
