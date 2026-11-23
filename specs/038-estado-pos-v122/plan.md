# Implementation Plan — 038-estado-pos-v122

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze (READ-ONLY) → 12 implement → 13 converge → 27 validate → 26 release. **`analyze` executa DEPOIS de `tasks`** (ordem oposta à do ciclo 037); R1 `go`; R4 precede R7 `1.0.0`; `converge` append-only. **R6 está suspensa por decisão ratificada**: fora da sequência executável até `resolve-tasks.sh`/adaptador existirem; PARE se invocada, nunca simular issues.

## Passos

1. **00 Constituição**: constituição v1.2.2 lida e aplicada; sequência executável acima; R6 suspensa (não executada, não simulada).
2. **01 Intake**: sessão `20261123T012952Z-estado-pos-v122`, 80/20 (375/625 vs. 180/625), evidências de 01:29–01:31Z.
3. **02 Research**: HEADs/PR/gates/contagens/inventário lidos, deriva dos cinco documentos registrada, constituição v1.2.2 e R6 citadas com fonte.
4. **03 Define (problem)**: rastreabilidade, deriva e governança; critérios observáveis; custo de inação.
5. **04 Shape (concept)**: 6 opções, recomendação documental, 80/20 = 375/625.
6. **05 Decide**: scorecard 6/6 strong, `Veredito: go` (R1), 0 clarificações (R3), R6 suspensa por decisão ratificada.
7. **06 Specify / 07 Clarify**: substituir o stub por `spec.md` com US1–US5, FR-001..FR-009, SC-001..SC-006; clarificar escalonar ≠ mesclar, deriva ≠ edição, R6 suspensa ≠ dívida, contagens versionadas ≠ checkout compartilhado, ordem v1.2.2 ≠ ordem 037.
8. **08 Plan**: este arquivo; fronteira explícita (só 14 artefatos), sem build/gate/commit.
9. **09 Checklist**: um item por FR em `checklists/requirements.md`; revisão registrada na seção "Revisão" e aprovada para `tasks`. `[x]` = revisão de requisitos, não implementação.
10. **10 Tasks**: `tasks.md` com T-IDs únicos e DAG linear; `analyze` vem **depois** de `tasks` na ordem; T019 (promoção/SWE) fica **[NV]** e fora da fronteira.
11. **11 Analyze (READ-ONLY)**: `analyze.md` com achados A1..A8, convergência e veredito; nenhuma edição de spec/plano/checklist/código nesta etapa; executa **depois** de `tasks`, conforme v1.2.2.
12. **12 Implement (documental)**: redação dos demais artefatos SDD na fronteira, incluindo o roteiro de testes antes do PR; nenhum build/gate/teste.
13. **13 Converge (append-only)**: primeira entrada em `specs/038-estado-pos-v122/converge.md`, datada, sem tocar `converge.md` de outros ciclos.
14. **27 Validate (R4)**: `validate.md` aprovando documentalmente, com toda execução pendente marcada **[NV]**; R4 antes de R7.
15. **26 Release (R7)**: `release.md` versão `1.0.0`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4.

## Roteiro de testes (antes do PR)
`.specify/validations/estado-pos-v122/test-roadmap.md` — escopo/FR/T, pré-condições, 10 passos numerados, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback. Passos explícitos: conferir PR #48 green (comentário `7089052846`); comparar notas vs HEAD/contagens; confirmar que R6 está fora do fluxo por decisão ratificada (PARE se invocada, nenhuma issue simulada).

## Handoff ao SWE / orquestrador (fora da fronteira SDD)
- PR #48 (`dev → main`) está OPEN, green, ACR sem blockers (comentário `7089052846`) e na fila SWE: **não mesclar `main` neste ciclo**; promoção, deploy e acompanhamento da ACR pertencem ao SWE. Deploy ausente no workflow é dívida já escalada e aberta.
- Deriva em `OBJETIVO.md`, `ROADMAP.md`, `docs/research/index.md`, `docs/PROGRESSO.md` e `STATE.md` (âncora `efb4d4b`/PR #45) fica **registrada, não corrigida** — os arquivos estão fora da fronteira. Contagens versionadas corretas: 37/65/37/37/0/10; 238/13 do checkout compartilhado inclui untracked históricos preservados.
- Factory `b1deab1` (`origin/dev 05b977f`) é somente fonte; nada a transplantar. R6 permanece suspensa até emenda ratificada após a ferramenta existir.

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)
- Comparar notas com HEAD/contagens ao vivo (`git rev-parse`, `find`, gates) e com os trechos finais dos cinco documentos com deriva.
- Confirmar que PR #48 segue green e que R6 não foi adotada nem simulada; ordem v1.2.2 mantida (`tasks` antes de `analyze`).

## Riscos e mitigação
| Risco | Mitigação |
|---|---|
| PR #48 mesclado por engano | Decisão explícita de escalonar, não mesclar (FR-001/US2); handoff ao SWE |
| Deriva propagada como estado vigente | FR-003 registra os cinco documentos e a âncora antiga sem editá-los |
| Contagens não versionadas afirmadas como reais | Contagens versionadas (37/65/37/37/0/10) vs. 238/13 explicadas (FR-003) |
| R6 adotada ou issue simulada | FR-005: suspensão ratificada, PARE se invocada, `taskstoissues` não usado |
| Ordem invertida por hábito do ciclo 037 | Plano e tasks: `tasks` antes de `analyze` (READ-ONLY); R4 antes de R7 |
| Editar converge alheio ou alterações preexistentes | `converge.md` novo; fronteira de 14 artefatos; `feature.json`/`PROGRESSO.md` intactos |
| Afirmar verde sem execução | Tudo o que não foi observado marcado **[NV]** (Princípio VI) |
