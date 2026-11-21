# Concept — Endurecimento do check `rust` na proteção de `dev`

- **Opção 1 (recomendada)**: documentar o ciclo 032 (assess → spec → plan → tasks → checklist → analyze READ-ONLY → converge append-only → validate R4 → release R7) colando literalmente as evidências datadas já coletadas — merge do PR #36 e mutação da proteção de `dev` ocorreram sob autorização explícita do SE. Nenhum build/gate/commit, nenhuma reexecução, fronteira restrita de 12 artefatos.
- **Opção 2**: reexecutar `gh`/`verify.sh`/gates e refazer o merge e a mutação antes de escrever. Rejeitada — o merge (#36) e a mutação já foram executados e verificados por GET em `2026-11-21T20:25:48Z`; repetir seria redundante e poderia conflitar com o estado já mutado.
- **Opção 3**: estender a autorização a deploy/publicação e assinatura GPG no mesmo ciclo. Rejeitada — fora da autorização do SE; esses itens permanecem **[NV]/DEFER**.

**Recomendação**: Opção 1. Registrar a proteção resultante `contexts=["gates","rust"]` e o merge como entradas datadas, preservando o histórico. R4 precede R7; release `1.0.0` com changelog FR-001..FR-006 + T-IDs e rollback documentado (restaurar `contexts=["gates"]` via `gh api -X PUT` com o payload anterior — nunca executar).
