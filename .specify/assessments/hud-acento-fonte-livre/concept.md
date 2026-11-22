# Concept — HUD com acento e fonte livre

**Data**: 2026-11-22T21:17Z

## Opções
1. **Embarcar Noto Sans Regular (OFL 1.1) e restaurar as strings** (recomendada): copiar o TTF de `/usr/share/fonts/google-noto/` para `assets/fonts/`, anexar o texto integral da licença, registrar proveniência/hash, carregar via `AssetServer` e aplicar `font` no `TextFont` do HUD; `Ê`, `—` e `·` voltam sem tofu. Reversível e auditável.
2. Manter o paliativo ASCII do ciclo 029 — rejeitada: o SWE pediu explicitamente o `Ê`.
3. Usar a fonte do sistema por caminho absoluto sem embarcar — rejeitada: não reproduzível nem rastreável em outro host.
4. Alterar a fonte padrão/atlas do Bevy — rejeitada: acoplamento a internals da engine, sem proveniência.
5. Corrigir apenas parte dos glifos — rejeitada: descumpre o pedido e deixa o HUD inconsistente.

## Decisão (R1: go)
Opção 1. Impacto 5 × confiança 4 × facilidade 3 × segurança 4 ÷ 625 = 240/625 (80/20 registrado; escolha do usuário tem prioridade). Handoff: agente de código aplica fonte + strings + licença/proveniência; este agente escreve somente os artefatos SDD. R3: 0 clarificações — pedido, fonte e licença já estão definidos pelas evidências.

## Salvaguardas
- Cobertura Unicode confirmada por `fc-match` para U+00CA/U+2014/U+00B7 antes de embarcar.
- SHA-256 do TTF e da licença registrados em `docs/governance/font-provenance.md` e verificáveis no roteiro de testes.
- Nenhuma dependência Rust nova; nenhuma janela normal aberta; capturas apenas em Xvfb isolado.
- Histórico do ciclo 029 preservado (append-only); nada é apagado ou reescrito.
