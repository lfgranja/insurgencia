# Research — Estado verificado pós-v1.2.2

**Data**: 2026-11-23T01:30Z · Evidências observadas às 01:29–01:31Z, coladas sem reexecução.

## Evidência primária
- [source: git, 01:29Z] HEAD/`dev`=`e010363`, `main`=`8b0b044`; worktree isolado `.specify/queue/worktrees/doc-drift-038/`, branch `038-estado-pos-v122`, base `e010363`. 1 worktree, 0 stashes, 24 untracked preservados.
- [source: GitHub, 01:29Z] 0 PRs abertos para `dev`. PR **#48** `dev → main` OPEN, green, ACR sem blockers (comentário `7089052846`), na fila SWE. Decisão: **não mesclar `main`** — a promoção/deploy pertence ao SWE; a constituição v1.2.2 limita o merge autônomo a `dev`.
- [source: gates observados, 01:30Z] Gates Python **PASS**; **41/41** testes. Nada reexecutado neste worktree — qualquer reexecução permanece **[NV]** (Princípio VI).
- [source: `.specify/memory/constitution.md`, lido] **Version 1.2.2** (ratificada 2026-11-19, última emenda 2026-11-23). Princípio III: sequência executável `00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release`. `analyze` é READ-ONLY e vem **depois** de `tasks`; `converge` é append-only; R1 exige `go`; R4 precede R7. **R6 (`taskstoissues`) está ratificada como intenção, mas fora da sequência executável** enquanto `.specify/extensions/github/scripts/bash/resolve-tasks.sh` e o adaptador de issues não existem; se um ciclo a invocar sem a ferramenta, **PARE e escale ao SWE**, nunca simule issue, nunca use `gh` improvisado e nunca alegue execução. Inclusão de R6 exige emenda ratificada após a ferramenta existir.
- [source: observação 01:30Z] **Deploy ausente no workflow** — dívida previamente escalada e aberta; não é corrigida neste ciclo (fora da fronteira).
- [source: factory, 01:30Z] Factory `b1deab1` (`origin/dev 05b977f`) apenas fonte — nada a transplantar.
- [source: deriva documental, 01:31Z] `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` e `STATE.md` ainda ancoram, em trechos finais, o snapshot antigo `efb4d4b`/PR #45 (ciclo 037). O estado vigente é `e010363`/`8b0b044`/PR #48.
- [source: contagens versionadas, 01:31Z] Árvore versionada (`origin/dev`): **37 specs, 65 assessments, 37 validations, 37 releases, 0 bugs, 10 research**. O checkout compartilhado mostra **238/13** porque inclui 173 diretórios untracked históricos (`ast-*`, `cor-*`, `doc-*`, `srv-*`, `ev-*`) e 3 notas `2026-11-20-*` não versionadas, preservadas por decisão registrada. O stub/artefatos 038 são novos e não versionados neste worktree.

## Interpretação
O estado está **verificado e estável** em `e010363`, com gates verdes e PR #48 green sem blockers na fila SWE. O valor imediato (375/625) está em consolidar esse estado e registrar a deriva documental e a governança v1.2.2 — não em coletar materiais (180/625) nem em promover `main`. A constituição v1.2.2 é a do próprio alvo (ratificada), ao contrário do ciclo 037: não há conflito de versão a resolver, mas há uma **suspensão de R6 por decisão ratificada** que deve ser registrada em toda a cadeia de artefatos, com PARE se invocada e nenhuma simulação de issues. A ordem executável agora coloca `analyze` **depois** de `tasks` — inversão em relação ao ciclo 037 que deve ser refletida em plano, tasks, analyze e roteiro.

## Alternativas rejeitadas
1. Mesclar `main` para "limpar" o PR #48 — rejeitada: promoção/deploy é competência do SWE; constituição v1.2.2 limita merge autônomo a `dev`.
2. Coletar materiais — rejeitada pelo 80/20 (180/625): confiança/facilidade/segurança menores e fora da fronteira.
3. Adotar R6 como regra ativa ou invocar `taskstoissues`/`resolve-tasks.sh` — rejeitada: R6 está fora da sequência executável até a ferramenta existir; PARE e escale; nunca simular issues.
4. Editar `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` ou `STATE.md` para corrigir a deriva — rejeitada: fora da fronteira exata; a deriva é **registrada**, não corrigida neste ciclo.
5. Reordenar o fluxo como no ciclo 037 (`analyze` antes de `tasks`) — rejeitada: a constituição v1.2.2 do alvo exige `tasks` antes de `analyze` (READ-ONLY).

## Rastreabilidade
Fatos colados como observados às 01:29–01:31Z, sem reexecução. Nenhum comando de gate, build, git remoto, GitHub, merge ou emissão de issue foi executado por este agente. Tudo o que depende de execução ou decisão externa (CI final do PR #48, merge pelo SWE, deploy, confirmação da SE, execução do roteiro) permanece **[NV]**. Factory `b1deab1` é somente fonte. As alterações preexistentes em `.specify/feature.json` e `docs/PROGRESSO.md` não foram tocadas.
