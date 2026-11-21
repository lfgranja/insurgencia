# Problem — Fechamento de rastreabilidade (ciclos 028–030)

**Problema**: os artefatos SDD dos ciclos 028–030 divergem do estado observável em 2026-11-21 — tarefas já entregues (PRs #34 e #35 mesclados, CI verde) permanecem marcadas como `[ ]`; a expressão "`verify.sh --all-targets`" é sintaticamente inválida e foi replicada em oito arquivos; `AGENTS.md` afirma Lightyear sem ressalva de que a dependência não existe nas manifestações; e o estado real dos gates/CI não está registrado de forma datada e auditável.

**Sucesso observável**: rastreabilidade fechada (T004–T008 de 029 e T011 de 028 marcados com nota datada e evidências), zero ocorrências da forma inválida do comando nos arquivos designados, `AGENTS.md` preciso sobre Lightyear, entradas datadas em `converge.md`/`PROGRESSO.md`, e evidências datadas coladas nos artefatos 031 — tudo sem alterar código, gates, constituição ou trabalho alheio.

**Custo de não agir**: um auditor que seguir os artefatos concluiria, erradamente, que trabalho mesclado ainda está pendente, que o comando de verificação aceita uma flag inexistente e que o projeto depende de Lightyear; a cadeia de evidências entre requisito, tarefa e verificação fica quebrada exatamente nos ciclos mais recentes.
