# Contratos de Fase SDD — Insurgência (alvo)

Fonte: Const. III+AGENTS.md+checker. Ordem (nenhuma portaria pulada): intake→research→define→shape→decide→specify→clarify→plan→checklist→analyze→tasks→implement→converge→validate→release. `k`=`excecao-linguagem-licenca-nucleo`; **A**=`.specify/assessments/<k>/`, **S**=`specs/001-<k>/`, **V**=`.specify/validations/<k>/`, **R**=`.specify/releases/<k>/`.

## Assess (saída em A)

1. **intake** → `A/intake.md`. Campos: Slug, Created, Source, Type(7 opções), Idea, Restated, Origin, Unknowns. Gate: ideia bruta→research/define; só captura. Proibido: avaliar, solucionar, inventar origem, segredos, executar URLs, sobrescrever sem confirmação.
2. **research** → `A/research.md`. Campos: Slug, Created, confidence(high|medium|low), Users&Demand, Prior Art, Market, Data&Constraints, **Evidence Against(sempre)**, Gaps, Sources(URL/host/policy). Gate: intake→define. Proibido: decidir/desenhar, ASSUMPTION como evidência, modificar código, fetch fora da allowlist.
3. **define** → `A/problem.md`. Campos: Slug, Created, Inputs, Problem Statement, Users&Stakeholders, Goals, Non-Goals, Success Metrics+baseline, Cost of Inaction, Open Questions. Gate: intake/research→shape. Proibido: espaço da solução(features/API/modelos/tarefas), inventar usuários/métricas, sobrescrever sem confirmação.
4. **shape** → `A/concept.md`. Campos: Slug, Created, Recommended option, 2–3 Options(Sketch/Appetite S|M|L/Trade-offs/Rabbit holes), Recommendation, Out of Scope, Assumptions. Gate: **exige `problem.md`**→decide. Proibido: spec/arquitetura/API/tarefas, appetite sem evidência, forçar vencedor("none" válido).
5. **decide** → `A/decision.md`. Campos: Slug, Decided, Verdict(go|needs-clarification|kill), Artifacts, Scorecard(problem validity, evidence strength, value/inaction, feasibility, strategic fit, risk posture ∈ strong|adequate|weak|unknown), Rationale, ramos clarify/go. **R1: go exige validity≥adequate, evidence≥adequate(nunca weak/unknown) e opção recomendada; `problem.md` obrigatório, `concept.md` esperado(sem ele→needs-clarification). Só go→specify.** Proibido: over-claim go, escrever spec, enterrar kill.

## Entrega (saída em S)

6. **specify** → `S/spec.md`+`S/checklists/requirements.md`+`.specify/feature.json`. Campos: User Scenarios, Functional Requirements(testáveis), Success Criteria(mensuráveis,tech-agnostic), Key Entities, Assumptions; ≤3 `[NEEDS CLARIFICATION]`. Gate: após R1→clarify/plan; 1 feature/invocação. Proibido: HOW, checklists na spec, >3 marcadores.
7. **clarify** → `S/spec.md`: `## Clarifications`(`### Session YYYY-MM-DD`,`- Q: … → A: …`)+revalida `requirements.md`. Campos: Clear/Partial/Missing; 1 pergunta/vez. **Gate: antes de plan; R3=≤3, um por vez** (portaria prevalece sobre default 5). Proibido: stack/tasks, criar spec, exceder R3.
8. **plan** → `S/plan.md`+`S/research.md`(F0)+`S/data-model.md`+`S/contracts/`+`S/quickstart.md`(F1). Campos: Technical Context, Constitution Check(reavaliado), decisões. Gate: spec esclarecida; ERRO em violação injustificada/clarify pendente→checklist/tasks. Proibido: código/migrações no quickstart, seguir com gate falho.
9. **checklist** → `S/checklists/<domain>.md`(`- [ ] CHK### …`,≥80% rastreáveis), **append-only**. Testa QUALIDADE DOS REQUISITOS, não implementação. Gate: lê spec/plan/tasks→analyze/tasks. Proibido: itens Verify/Test/Confirm/Check+comportamento, marcar `[x]`, apagar/substituir existentes(exceto `requirements.md`).
10. **analyze** → relatório **em memória, sem arquivo**(ID/Categoria/Severidade CRITICAL→LOW/Local/Sumário/Recomendação; ≤50; cobertura, métricas); ≥088:`S/analyze.md`. **Gate: STRICTLY READ-ONLY, só após tasks.md; constituição=autoridade(conflito=CRITICAL).** Proibido: modificar arquivos, remediação sem aprovação, alucinar seções, verde sem evidência.
11. **tasks** → `S/tasks.md`. Formato `- [ ] T### [P?] [USn?] descrição com caminho`; fases Setup→Foundational→User Stories(P1…)→Polish; testes só se pedidos. Gate: plan+spec→analyze/implement. Proibido: sem checkbox/ID/caminho, fora de user stories, ID duplicado.
12. **implement** → código+`S/tasks.md` com `[X]`. Gate: checklists=gate **read-only**; o STOP por item `[ ]` aplica-se somente ao checklist de domínio em revisão humana de requisitos, não a `requirements.md` (ciclo próprio) nem a bloqueio automático da implementação documental. Exige spec/plan/tasks; por fase, TDD, marca `[X]`. Proibido: modificar checklists, prosseguir após falha não-paralela, pular validate/release, concluir sem validação.
13. **converge** → **APPEND-ONLY** em `S/tasks.md`: `## Phase N: Convergence` com `- [ ] T### <desc> per <source-ref> (<gap-type>)`; sem achados=inalterado byte-for-byte. Gap-types: missing|partial|contradicts|unrequested. **Gate: só após implement no tasks.md atual→implement/validate; ≥088:`S/converge.md`.** Proibido: modificar spec/plan, reescrever/apagar tarefas, mexer em código, git/diff.

## Evidência — ARTEFATOS, não comandos

14. **validate (R4)** — Artefato `V/`. Campos: gate executado, saída colada, data/hora, conformidade; remoto/CI/merge=`[NV]` até confirmação. **Gate: R4 aprovado antes de R7.** Proibido: verde sem comando, esconder divergência documento↔código.
15. **release (R7)** — Artefato `R/`. Campos: **SemVer+changelog com IDs FR/T/bug+rollback**. **Gate: exige R4 aprovado.** Proibido: release sem R4/SemVer/changelog/rollback, auto-merge/force-push, CI/merge verde sem confirmação.

## Ordem obrigatória (prefixos = caminhos exatos)

| # | Fase | Saída |
|---|------|-------|
| 1 | intake | `A/intake.md` |
| 2 | research | `A/research.md` |
| 3 | define | `A/problem.md` |
| 4 | shape | `A/concept.md` |
| 5 | decide | `A/decision.md` — **R1=`go`** |
| 6 | specify | `S/spec.md`(+`checklists/requirements.md`,`.specify/feature.json`) |
| 7 | clarify | `S/spec.md`(atualizado) — **R3≤3**, antes de plan |
| 8 | plan | `S/plan.md`(+`research.md`,`data-model.md`,`contracts/`,`quickstart.md`) |
| 9 | checklist | `S/checklists/<domain>.md` — append-only,`[ ]` |
| 10 | analyze | memória(≥088:`S/analyze.md`) — **read-only**, após tasks |
| 11 | tasks | `S/tasks.md` — `T### [P?] [USn?]` |
| 12 | implement | `S/tasks.md`(`[X]`)+código — checklists=gate |
| 13 | converge | `S/tasks.md`(`## Phase N: Convergence`;≥088:`S/converge.md`) — **append-only** |
| 14 | validate | `V/` — **R4 aprovado** |
| 15 | release | `R/` — **R7**: SemVer+changelog FR/T/bug+rollback; R4 antes de R7 |

≥088: `analyze.md`+`converge.md` obrigatórios na spec; <088(caso `001`) legado, mas fases 10 e 13 executam.
