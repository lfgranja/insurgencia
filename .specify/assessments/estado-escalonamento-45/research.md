# Research — Estado verificado e escalonamento 45

**Data**: 2026-11-22T22:59Z

## Evidência primária (observada às 22:58–23:01Z; não reexecutada)
- [source: git, 22:58Z] HEAD=`efb4d4b`, `origin/dev=efb4d4b`, `origin/main=c126b6e`. **`c126b6e` é o merge commit do PR #40** que promoveu `dev → main` às 20:52:35Z no ciclo 036 — `main` está um commit atrás de `dev`.
- [source: GitHub, 22:59Z] 0 PRs abertos para `dev`. PR **#45** `dev → main` OPEN, green; ACR sem blockers, comentário `7086327685`; **na fila SWE**. Decisão: **não mesclar main** neste ciclo — a promoção pertence ao SWE.
- [source: git, 22:58Z] PR #44 mesclado em dev às 21:44:38Z, resultando em `efb4d4b`. É o merge do ciclo 036.
- [source: specs/036 + converge, lidos] Ciclo 036 entregou `INSURGÊNCIA — low-poly` e `Limites: X +/-580 · Y +/-310` com Noto Sans Regular (OFL 1.1, SHA-256 `478c558e…3823`, 621572 bytes) embarcada; blocker P1 da ACR (`BEVY_ASSET_ROOT`) corrigido em `01ff4d0` via `caminho_assets()`/`AssetPlugin.file_path`; captura final **sem env externo** (SHA-256 `e1769d91…8fa1`); evidência **11/11 PASS**.
- [source: gates observados, 22:59Z] `./verify.sh` → `ALL STAGES OK`, 41/41 testes. 5 gates Python → PASS (SDD, DAG, task-id, converge append-only, hub-docs).
- [source: `find` somente leitura neste worktree, 23:00Z] 36 diretórios de spec (excluindo o stub 037), 36 diretórios de validação, 36 arquivos de release, 0 itens `*bug*`. As contagens de assessments (238) e research (13) são as registradas na nota de estado da sessão; `find` local contabiliza 300 arquivos `.md` em `.specify/assessments` e 10 em `docs/research` — divergência de snapshot/método entre a árvore observada e este worktree, registrada conforme Princípio VI (verificação honesta), sem reexecução.
- [source: `.specify/memory/constitution.md`, lido, 23:00Z] **Version 1.0.0**, ratificada em 2026-11-19, fonte factory HEAD `b1deab1`. Princípio III: `intake → research → define → shape → decide → specify → clarify → plan → checklist → analyze → tasks → implement → converge → validate → release`; `converge.md` append-only; R1 exige `go`, R4 precede R7. **Não há R6 na constituição do alvo.**
- [source: prompt v4.6, citado no contexto da sessão] Cita constituição v1.2.0 (`tasks` antes de `analyze`, R6 obrigatório) e o comando `speckit.github.taskstoissues.md` exige `resolve-tasks.sh`, ausente no alvo. Bloco 0.a da sessão: vale a governança do alvo.

## Interpretação
O estado está **verificado e estável**: `dev` e `origin/dev` coincidem em `efb4d4b`, os gates estão verdes e o ciclo 036 está fechado com evidência 11/11. O único movimento pendente é a promoção `dev → main` via PR #45, que já passou pela ACR sem blockers e aguarda o SWE — escalonar esse fato é mais valioso (375/625) do que implementar paridade de `construir_barreira` (60/625). A divergência de versão constitucional não é um problema a corrigir: o alvo ratificou v1.0.0 e o prompt é apenas uma fonte externa; adotar R6 sem ratificação seria quebrar o Princípio III.

## Alternativas rejeitadas
1. Mesclar `main` para "limpar" o PR #45 — rejeitada: promoção pertence ao SWE; faria o agente agir fora da autoridade.
2. Implementar paridade `construir_barreira` — rejeitada pelo 80/20 (60/625) e por envolver código, contrariando o escopo documental.
3. Adotar R6 e reordenar `tasks` antes de `analyze` para seguir o prompt v4.6 — rejeitada: a constituição do alvo (v1.0.0, ratificada) não tem R6 e exige `analyze` antes de `tasks`.
4. Registrar a divergência de governança como dívida técnica — rejeitada: Bloco 0.a a classifica como decisão de governança; dívida implicaria correção futura de algo que está correto no alvo.
5. Usar `speckit.github.taskstoissues.md` — rejeitada: depende de `resolve-tasks.sh`, ausente; nenhuma emissão de issues é feita neste ciclo.

## Rastreabilidade
Fatos colados como observados, sem reexecução. Nenhum comando de gate, build, git remoto ou GitHub foi executado por este agente. Tudo o que depende de execução ou decisão externa (CI final do PR #45, merge pelo SWE, confirmação da SE) permanece **[NV]**. Factory `b1deab1` é somente fonte.
