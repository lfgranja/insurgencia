# Concept — Fechamento de rastreabilidade (ciclos 028–030)

- **Opção 1 (recomendada)**: correção documental cirúrgica com fronteiras disjuntas — A escreve os 13 artefatos do ciclo 031 (assess → spec → plan → tasks → checklist → analyze → converge → validate → release); B aplica as correções nos artefatos existentes (marcação de tarefas, errata de redação, appends datados, `AGENTS.md`). Nenhum build/gate/commit; evidências já coletadas são coladas, não reexecutadas.
- **Opção 2**: reexecutar todos os gates e o `gh` ao vivo antes de escrever, e gerar um único agente para tudo. Mais custo, risco de mexer em trabalho alheio e violação da regra de que "[x] em checklist = revisão de requisitos, não implementação".
- **Opção 3**: reescrever os `converge.md` existentes para refletir o estado atual. Rejeitada — `converge.md` é **append-only**; entradas antigas nunca são reescritas ou renumeradas.

**Recomendação**: Opção 1. Registrar a errata e as evidências como entradas datadas (`2026-11-21T13:23Z`), preservando o histórico. R4 precede R7; release `1.0.0` com changelog FR-001..FR-006 + T-IDs e rollback documentado.
