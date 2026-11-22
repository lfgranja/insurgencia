# Intake — HUD com acento e fonte livre

**Data**: 2026-11-22T21:15Z · **Slug**: `hud-acento-fonte-livre` · **Origem**: pedido explícito do SWE na sessão `20261122T211548Z-hud-acento-fonte`: “Agora quero o Ê sim.”

## Ideia
Reverter o paliativo ASCII do ciclo 029 e restaurar a grafia correta no HUD do cliente low-poly: `INSURGÊNCIA — low-poly` e `Limites: X +/-580 · Y +/-310`, usando uma fonte livre embarcada e rastreável (cobertura de U+00CA, U+2014 e U+00B7), sem tofu `▯`.

## Evidências de entrada (observadas às 21:15–21:16Z)
- Worktree `.specify/queue/worktrees/hud-acento-036/`, branch `036-hud-acento-fonte-livre`, base/HEAD `7c5c0f3`; 0 modificações rastreadas, 24 untracked (22 históricos preservados + captura + diretório), 0 PRs abertos.
- `verify.sh` e gates Python verdes anteriormente: 41/41 testes, SDD `35/0`, DAG `332/34/0`, converge `18/18`, hub-docs `0`.
- Captura em display virtual (Xvfb + lavapipe) `docs/evidencias/hud-confirmacao-visual-2026-11-22.png`: HUD ASCII atual, legível, porém com grafia incorreta.
- PR #40 mesclado pelo SWE às 20:52:35Z (merge commit `c126b6e`; `origin/main=c126b6e`) — encerra o [NV] de promoção `dev → main` das sessões anteriores.
- Factory `b1deab1` (`origin/dev` `8ae9f52`) contém apenas a fonte; nada a transplantar.

## Restrição da SE (herdada do ciclo 029)
Agente SEMPRE roda o cliente com `--minimizado`; NUNCA abre janela normal no desktop da SE. Inspeção visual fica com a SE; capturas de verificação devem usar display virtual isolado (Xvfb).
