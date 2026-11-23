# Checklist de Requisitos — 038-estado-pos-v122

- [x] FR-001 HEAD/`dev` `e010363`, `main` `8b0b044`, 0 PRs para dev, PR #48 OPEN/green/ACR sem blockers (comentário `7089052846`) na fila SWE, decisão de **não mesclar `main`**
- [x] FR-002 gates Python PASS e 41/41 testes observados; deploy ausente no workflow (dívida escalada); 1 worktree, 0 stashes, 24 untracked preservados; reexecução **[NV]**
- [x] FR-003 deriva registrada em `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md`, `STATE.md` (âncora `efb4d4b`/PR #45) sem edição; contagens versionadas 37/65/37/37/0/10 vs. 238/13 do checkout compartilhado (untracked históricos)
- [x] FR-004 constituição v1.2.2 do alvo aplicada: merge autônomo só em `dev`, promoção/deploy ao SWE, `analyze` READ-ONLY **depois** de `tasks`, R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only
- [x] FR-005 R6 suspensa por decisão ratificada: fora do fluxo até `resolve-tasks.sh`/adaptador, PARE se invocada, nenhuma issue simulada, nenhum `taskstoissues`/`gh` improvisado
- [x] FR-006 80/20 documentado: documentação pós-v1.2.2 375/625 > coletar materiais 180/625
- [x] FR-007 14 artefatos na fronteira exata; nenhum `converge.md` existente editado; cinco documentos com deriva intactos; alterações preexistentes fora da fronteira preservadas
- [x] FR-008 `test-roadmap.md` antes do PR: 10 passos, superfície documental/CLI, capturas de terminal, `automatizável?`, rollback, passo de PR #48 green, passo de notas vs HEAD/contagens e passo de R6 fora do fluxo
- [x] FR-009 nenhum build/gate/formatador/teste/commit/push/PR; `main` não mesclado; factory `b1deab1` (`origin/dev 05b977f`) somente fonte; pendências **[NV]**

## Revisão
Requisitos verificáveis por inspeção das evidências coladas (01:29–01:31Z), da constituição v1.2.2 lida e do roteiro de testes. Checklist `[x]` = revisão de requisitos, **não** execução de build/gate/teste nem implementação. Ao contrário do ciclo 037, não há conflito de versão constitucional: o alvo ratificou v1.2.2 e a ordem executável coloca `tasks` antes de `analyze`; o hábito do ciclo 037 é explicitamente rejeitado na fonte. R6 não é tratada como dívida nem como regra ativa: está ratificada como intenção, mas suspensa por decisão ratificada até a ferramenta existir — PARE se invocada, nenhuma issue simulada. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0). Evidências coladas como observadas; reexecuções e decisões externas permanecem **[NV]** (Princípio VI). Aprovado para `tasks`.
