# Analyze (READ-ONLY) — 036-hud-acento-fonte-livre

**Data:** 2026-11-22T21:20Z · **Escopo:** spec, plano e checklist — nenhuma edição de artefato, código, baseline ou documento-fonte nesta etapa. **Posição:** executa **antes** de `tasks`.

## Achados

| # | Achado | Severidade | Correção na fonte |
|:--|:--|:--|:--|
| A1 | Restaurar as strings sem trocar a fonte reintroduz o tofu `▯` do ciclo 029 | alta | FR-001/FR-002/FR-003: fonte embarcada e `TextFont.font` são pré-condição das strings |
| A2 | Paliativo ASCII poderia ser mantido por “segurança”, contrariando o SWE | alta | FR-001..FR-005/Clarificações: a escolha do usuário revoga o paliativo para os três glifos; cobertura provada por `fc-match` |
| A3 | Fonte do sistema por caminho absoluto não seria reproduzível/auditável | alta | FR-003/FR-004: copiar o TTF para `assets/fonts/`, registrar SHA-256, licença integral e URL primária |
| A4 | Ordem normativa poderia ser invertida (`tasks` antes de `analyze`, R7 antes de R4) | alta | Plano passos 5–10 e `tasks.md`: `analyze` (READ-ONLY) antes de `tasks`; R4 antes de R7 `1.0.0` |
| A5 | Risco de afirmar verde para `verify.sh`/gates/captura sem tê-los executado | alta | FR-009/Pressupostos: pendências **[NV]**; apenas evidências observadas às `21:15–21:18Z` são coladas |
| A6 | Regra da janela da SE poderia ser violada por inspeção visual direta | média | FR-006/roteiro: cliente `--minimizado`, capturas somente em Xvfb isolado, sem janela no desktop |
| A7 | Roteiro de testes escrito depois do PR perderia valor de verificação prévia | média | FR-008/T014: escrever `test-roadmap.md` antes do PR, com passos dos glifos, ausência de `▯`, hash e licença |
| A8 | Tentação de transplantar a fonte da factory `b1deab1` ou tocar documentos alheios | média | FR-009/Out of scope: factory é somente fonte; `STATE.md`, `PROGRESSO.md`, `feature.json`, baselines e paths untracked alheios intactos |

## Convergência
Não há conflito entre spec, plano e checklist: os três glifos, a fonte e a licença estão alinhados em FR-001..FR-009 e SC-001..SC-006. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0). R1 `go` presente com scorecard 6/6 strong e 80/20 = 240/625. A captura prévia (`docs/evidencias/hud-confirmacao-visual-2026-11-22.png`) prova que o ASCII era legível; `fc-match` prova que Noto Sans cobre U+00CA/U+2014/U+00B7. O PR #40 mesclado (`c126b6e`, 20:52:35Z) é fato observado, não pendência. Nenhuma execução de `verify.sh`, gates, build, formatador, commit, push ou PR aconteceu nesta etapa, e nada foi escrito fora da fronteira de 14 artefatos. A implementação observada no worktree às `21:18Z` pertence ao agente de código e permanece **[NV]** até verificação pelo orquestrador. R4 precede R7; release `1.0.0` com rollback documentado (nunca executado).

## Veredito
Prosseguir para `tasks` e, em seguida, para a redação do roteiro, do `converge.md` (primeira entrada append-only), de `validate.md` (R4) e de `release.md` (R7), com A1–A8 tratados como requisitos de governança e fronteira. Factory `b1deab1` permanece somente fonte; histórico do ciclo 029, PR #40 mesclado, `docs/GDD*`, baselines, `STATE.md`, `PROGRESSO.md`, `feature.json`, ledger, CI e paths untracked alheios intactos por este agente. `./verify.sh`, gates e a captura com acento permanecem **[NV]** até execução observada. PASS.
