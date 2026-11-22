# Checklist de Requisitos — 037-estado-escalonamento-45

- [x] FR-001 HEADs `efb4d4b`/`c126b6e`, 0 PRs para dev, PR #45 OPEN/green/ACR sem blockers (comentário `7086327685`) na fila SWE, decisão de **não mesclar `main`**
- [x] FR-002 PR #44 mesclado às 21:44:38Z (`efb4d4b`); ciclo 036 (Noto Sans OFL 1.1, `BEVY_ASSET_ROOT` corrigido em `01ff4d0`, captura sem env, evidência 11/11)
- [x] FR-003 `verify.sh` 41/41 + 5 gates PASS e contagens (36/238/36/36/0/13) com `find` somente leitura e ressalva de snapshot **[NV]**
- [x] FR-004 constituição v1.0.0 do alvo prevalece (Bloco 0.a); divergência = decisão de governança, não dívida; R6 não adotado; `taskstoissues`/`resolve-tasks.sh` não usados
- [x] FR-005 80/20 documentado: documentação/escalonamento 375/625 > paridade `construir_barreira` 60/625
- [x] FR-006 ordem normativa do alvo: `analyze` (READ-ONLY) antes de `tasks`; R4 antes de R7 `1.0.0`; R1 `go`
- [x] FR-007 14 artefatos na fronteira exata; nenhum `converge.md` existente editado; nada fora da fronteira
- [x] FR-008 `test-roadmap.md` antes do PR: 6–10 passos, superfície documental/CLI, capturas de terminal, `automatizável?`, rollback, passo notas vs HEAD/contagens e passo de não-adoção de R6
- [x] FR-009 nenhum build/gate/formatador/teste/commit/push/PR; não mesclar `main`; factory `b1deab1` somente fonte; pendências **[NV]**

## Revisão
Requisitos verificáveis por inspeção das evidências coladas, da constituição lida e do roteiro de testes. Checklist `[x]` = revisão de requisitos, **não** execução de build/gate/teste nem implementação. Conflito avaliado e resolvido na fonte: o prompt v4.6 pede `tasks` antes de `analyze` e R6 (constituição v1.2.0), mas o alvo ratificou v1.0.0 — Bloco 0.a faz prevalecer a governança do alvo; a divergência é registrada como **decisão**, não dívida, e nenhuma ratificação de R6 é inventada. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0). Evidências de `22:58–23:01Z` coladas como observadas; `find` somente leitura confirma subset das contagens e a divergência de snapshot é registrada sem reexecução (Princípio VI). PR #45 permanece na fila SWE: escalonar não é mesclar. Aprovado para `analyze`.
