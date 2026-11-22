# Contrato de remediação do analyze — Feature 002 (2026-11-19)

Fonte: relatório de analyze (READ-ONLY) com 4 achados: F1 MEDIUM, F2 MEDIUM, F3 LOW, F4 MEDIUM.

| ID | Decisão canônica |
|---|---|
| F1 | Morada única de `EM-002-1.0.0` e do campo `contagens` = `data-model.md §Entidade Registro de emenda`. O contrato `contracts/gdd-amendment-contract.md §5` permanece apenas como regra de formato (SemVer/changelog/rollback) e NÃO deve conter a entidade. Corrigir `data-model.md:17,116`, `quickstart.md` etapa 12 e `tasks.md` T022 para apontar a morada correta. |
| F2 | Manter a afirmação de que o GDD é o único artefato normativo de origem alterado. Tarefas que hoje dizem "Registrar/Fixar/Consolidar/Revisar o registro" em `data-model.md`/contrato devem ser reescritas como **conferência por leitura, sem alterar** esses artefatos (o conteúdo já existe no planejamento). Nenhuma tarefa desta feature escreve fora do GDD. |
| F3 | Diferenciar T007 (fixar o formato no contrato, na fase Foundational) de T026 (revisão final de conformidade do registro já preenchido, na fase Polish). Sem renumerar, sem apagar tarefas. |
| F4 | SC-003 passa a exigir: "100% das verificações documentais executadas neste ciclo retornam resultado observável; os gates determinísticos locais pertencem à portaria R4 (validate), não a tarefas desta feature". Ajustar spec SC-003, `checklists/requirements.md` CHK010, contrato e quickstart para a mesma redação. Nenhum verde remoto ou antecipado. |

Depois das correções, **rodar analyze novamente** (somente leitura). Sem CRITICAL, a implementação
só prossegue após essa revalidação.
