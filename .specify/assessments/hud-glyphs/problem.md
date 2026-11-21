# Problem — HUD glyphs ausentes

**Data**: 2026-11-21T04:59Z

O HUD do cliente low-poly exibe "tofu" (▯) no lugar de `Ê`, `—` e `·`, confirmado por imagem da SE. Isso degrada a legibilidade e a identidade do produto (nome do jogo aparece corrompido). A causa é a fonte padrão do Bevy sem esses glyphs — não é erro de lógica.

## Critérios de sucesso
1. Strings do HUD usam apenas ASCII seguro (`INSURGENCIA - low-poly`, ` / `).
2. `verify.sh` ALL STAGES OK (34 testes) e 5/5 gates PASS.
3. Nenhuma fonte/asset externo adicionado.
4. Regra da janela respeitada: nenhuma janela normal aberta por agente.
5. Inspeção visual final fica com a SE **[NV]** até confirmação.
