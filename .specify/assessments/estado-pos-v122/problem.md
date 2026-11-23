# Problem — Estado verificado pós-v1.2.2

**Data**: 2026-11-23T01:30Z

O repositório chegou ao pós-v1.2.2 com `dev` em `e010363`, `main` em `8b0b044`, gates verdes (41/41) e PR #48 `dev → main` OPEN, green e com ACR sem blockers parado na fila SWE. Simultaneamente, cinco documentos normativos (`OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md`, `STATE.md`) seguem ancorando o snapshot antigo `efb4d4b`/PR #45 em trechos finais, e o checkout compartilhado reporta 238 assessments/13 research contra as contagens versionadas corretas (37/65/37/37/0/10). Sem um registro documental consolidado, o estado verificado fica disperso entre notas de sessão, trechos defasados e comentários de ACR; a decisão de **não mesclar `main`**, a **suspensão de R6 por decisão ratificada** e a ordem v1.2.2 (`tasks` antes de `analyze`) poderiam ser reinterpretadas como pendência, dívida ou autorização para simular issues. O problema é de **rastreabilidade, deriva e governança**, não de código.

## Critérios de sucesso observáveis
1. Os 14 artefatos SDD registram HEADs (`e010363`/`8b0b044`), PR #48 escalonado ao SWE (comentário `7089052846`) e a decisão explícita de não mesclar `main`, com carimbo 01:29–01:31Z.
2. Gates observados (41/41 + gates Python PASS), deploy ausente no workflow, factory `b1deab1` (`origin/dev 05b977f`) somente fonte e inventário (1 worktree, 0 stashes, 24 untracked) são citados com método e ressalva **[NV]** onde não houve reexecução.
3. A deriva documental dos cinco arquivos é **registrada** (âncora antiga `efb4d4b`/PR #45) sem editá-los — fora da fronteira — e as contagens versionadas corretas (37/65/37/37/0/10) são distinguidas do checkout compartilhado (238/13, untracked históricos).
4. A constituição v1.2.2 do alvo é aplicada: `analyze` READ-ONLY **depois** de `tasks`; R1 `go`; R4 antes de R7 `1.0.0`; `converge` append-only. **R6 suspensa por decisão ratificada**, fora do fluxo, com PARE se invocada e nenhuma issue simulada.
5. `test-roadmap.md` escrito antes do PR: 10 passos numerados, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback, passo de conferir PR #48 green, passo de comparar notas vs HEAD/contagens e passo de confirmar R6 fora do fluxo.
6. Nenhum build/gate/formatador/teste/commit/push/PR executado; fronteira exata respeitada; alterações preexistentes fora da fronteira preservadas.

## Custo de inação
Sem este registro, o PR #48 green e sem blockers poderia ser mesclado por engano (contrariando a competência do SWE), a deriva dos cinco documentos poderia se propagar como estado vigente, R6 poderia ser invocada sem adaptador (com issues simuladas), e a ordem v1.2.2 poderia ser invertida por hábito do ciclo 037 — com dano à honestidade das evidências do hub (Princípio VI) e à governança ratificada (Princípio III).

## Fora de escopo
Promover `dev → main`/mesclar PR #48; corrigir `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` ou `STATE.md`; reexecutar gates/build/capturas; transplantar da factory `b1deab1`; adotar R6 como regra ativa, invocar `taskstoissues`/`resolve-tasks.sh` ou simular issues; editar `converge.md` existente; mexer em `docs/GDD*`, baselines, ledger, CI, `feature.json`, paths untracked alheios, janelas, deploy, assinatura GPG, commits/push/PRs; coletar materiais. As alterações preexistentes em `.specify/feature.json` e `docs/PROGRESSO.md` pertencem ao usuário e não são tocadas.
