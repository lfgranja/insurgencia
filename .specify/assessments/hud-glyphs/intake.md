# Intake — HUD glyphs ausentes

**Data**: 2026-11-21T04:59Z · **Slug**: `hud-glyphs` · **Origem**: imagem anexada pela SE às 04:33Z mostrando a cena low-poly.

## Ideia
O HUD do cliente exibe caracteres substituídos por "▯" (tofu): `INSURG▯NCIA ▯ low-poly` (deveria ser `INSURGÊNCIA — low-poly`) e `Limites: X +/-580 ▯ Y +/-310` (deveria ser `·`). A fonte padrão embutida do Bevy aparenta não ter esses glyphs.

## Restrição da SE (04:33Z)
Agente SEMPRE roda o cliente com `--minimizado`; NUNCA abrir janela normal durante trabalho de agente. Verificação visual fica com a SE.
