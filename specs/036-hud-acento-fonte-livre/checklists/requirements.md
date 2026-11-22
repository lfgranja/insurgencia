# Checklist de Requisitos — 036-hud-acento-fonte-livre

- [x] FR-001 `Text::new("INSURGÊNCIA — low-poly")` no `configurar` (U+00CA, U+2014)
- [x] FR-002 `format!` em `atualizar_texto` com `·` (U+00B7) em `Limites: X +/-{LIMITE_X:.0} · Y +/-{LIMITE_Y:.0}`
- [x] FR-003 fonte embarcada `assets/fonts/NotoSans-Regular.ttf` (621572 bytes, SHA-256 `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823`) via `AssetServer`/`Handle<Font>` no `TextFont`; sem deps novas
- [x] FR-004 OFL 1.1 integral em `assets/fonts/OFL.txt` + proveniência em `docs/governance/font-provenance.md` (origem/hash/URL/cobertura)
- [x] FR-005 nenhum `▯`; cobertura dos três glifos por `fc-match` e captura Xvfb **[NV]** até execução
- [x] FR-006 regra da janela preservada; cliente `--minimizado`, capturas em Xvfb isolado; SE confirma visualmente **[NV]**
- [x] FR-007 14 artefatos SDD na ordem normativa; `analyze` READ-ONLY antes de `tasks`; `converge` append-only; R4 antes de R7 `1.0.0`; changelog FR/T e rollback
- [x] FR-008 `test-roadmap.md` antes do PR: escopo/FR/T, pré-condições, passos numerados, superfície CLI/visual isolada, capturas Xvfb, coluna `automatizável?`, rollback, passos dos glifos/sem `▯` e hash/licença
- [x] FR-009 fronteira e não-execução: nenhum build/gate/formatador/commit/push/PR; factory `b1deab1` somente fonte; histórico 029 e PR #40 (`c126b6e`) preservados; pendências **[NV]**

## Revisão
Requisitos verificáveis por inspeção das duas strings, do hash da fonte, do texto da licença e do roteiro de testes. Checklist `[x]` = revisão de requisitos, **não** execução de build/gate/teste nem implementação. Conflito avaliado e resolvido na fonte: o paliativo ASCII do ciclo 029 (seguro, porém graficamente incorreto) é substituído por fonte livre embarcada, atendendo ao pedido explícito do SWE sem reintroduzir tofu — a captura prévia comprova que o ASCII era legível, e o `fc-match` comprova que Noto Sans cobre os três glifos. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0). Evidências de `21:15–21:18Z` coladas como observadas; `verify.sh`, gates, captura com acento, CI e merge permanecem **[NV]** até o orquestrador. Aprovado para `analyze` e, depois, para `tasks`. Factory `b1deab1` somente fonte; `docs/GDD*`, baselines, `STATE.md`, `PROGRESSO.md`, `feature.json`, ledger, CI e paths untracked alheios intactos por este agente.
