# Implementation Plan — 037-estado-escalonamento-45

**Ordem normativa (do alvo, constituição v1.0.0)**: assess.intake → research → define(problem) → shape(concept) → decide(`go`) → specify → clarify → plan → checklist → **revisar checklist** → **analyze (READ-ONLY)** → tasks → implement → converge (append-only) → validate (R4) → release (R7). `analyze` executa **antes** de `tasks`; R4 aprova **antes** de R7 `1.0.0`. Não há R6: o prompt v4.6 (v1.2.0) é fonte externa e não ratificada no alvo (Bloco 0.a).

## Passos

1. **Assess**: intake (`22:58Z`, sessão `20261122T225813Z-estado-escalonamento`), research com fontes e 80/20 (375/625 vs. 60/625), problem, concept (5 opções) e decision com `Veredito: go` (R1), 0 clarificações (R3), precedência da constituição v1.0.0 registrada como decisão de governança.
2. **Specify/clarify**: substituir o stub por `spec.md` com US1–US4, FR-001..FR-009, SC-001..SC-006; clarificar escalonar ≠ mesclar, divergência ≠ dívida e contagens com ressalva.
3. **Plan**: este arquivo; fronteira explícita (só 14 artefatos), sem build/gate/commit.
4. **Checklist**: um item por FR em `checklists/requirements.md`; revisão registrada na própria seção “Revisão” e aprovada para `analyze`. `[x]` = revisão de requisitos, não implementação.
5. **Analyze (READ-ONLY)**: `analyze.md` com achados A1..A8, convergência e veredito; nenhuma edição de spec/plano/checklist/código nesta etapa; executa **antes** de `tasks`.
6. **Tasks**: `tasks.md` com T-IDs únicos e DAG linear; T018 (implement/promoção) pertence ao SWE/orquestrador e fica **[NV]**.
7. **Roteiro de testes (antes do PR)**: `.specify/validations/estado-escalonamento-45/test-roadmap.md` — escopo/FR/T, pré-condições, 6–10 passos numerados, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback; passos explícitos de notas vs HEAD/contagens e de confirmação de não-adoção de R6.
8. **Converge (append-only)**: primeira entrada em `specs/037-estado-escalonamento-45/converge.md`, datada, sem tocar `converge.md` de outros ciclos.
9. **Validate (R4)**: `validate.md` aprovando documentalmente, com toda execução pendente marcada **[NV]**.
10. **Release (R7)**: `release.md` versão `1.0.0`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4.

## Handoff ao SWE / orquestrador (fora da fronteira SDD)
- PR #45 (`dev → main`) está OPEN, green, ACR sem blockers (comentário `7086327685`) e na fila SWE: **não mesclar `main` neste ciclo**; promoção e acompanhamento da ACR pertencem ao SWE.
- PR #44 mesclado às 21:44:38Z (`efb4d4b`) encerra o ciclo 036; blocker P1 `BEVY_ASSET_ROOT` corrigido em `01ff4d0`; captura final sem env e evidência 11/11 — fatos observados, não reexecutados.
- Factory `b1deab1` (`origin/dev b3b0c9f4`) é somente fonte; nada a transplantar.

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)
- Comparar notas deste ciclo com HEAD/contagens ao vivo (`git rev-parse`, `find`, gates) — passo 9 do roteiro.
- Confirmar que R6 não foi adotado e que a ordem do alvo foi mantida — passo 10 do roteiro.
- CI/PR e merge do PR #45 ficam com o SWE; nenhum agente mescla `main`.

## Riscos e mitigação
| Risco | Mitigação |
|---|---|
| PR #45 mesclado por engano | Decisão explícita de escalonar, não mesclar (FR-001/US2); handoff ao SWE |
| R6/v1.2.0 adotados sem ratificação | Bloco 0.a; constituição v1.0.0 lida; passo 10 do roteiro |
| Divergência tratada como dívida | Registrada como decisão de governança (FR-004) |
| Contagens afirmadas sem verificação | `find` somente leitura; divergência de snapshot registrada (FR-003) |
| Ordem normativa invertida | `analyze` antes de `tasks`; R4 antes de R7; checklist revisado |
| Afirmar verde sem execução | Tudo o que não foi observado marcado **[NV]** (Princípio VI) |
| Editar converge alheio | `converge.md` novo, append-only, na fronteira 037 |
