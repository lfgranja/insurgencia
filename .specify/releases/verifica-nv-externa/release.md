# Release (R7) — verifica-nv-externa

- **Versão**: 1.0.0 (SemVer)
- **Data**: 2026-11-19
- **Feature**: `specs/004-verifica-nv-externa`
- **R4**: registrado em `.specify/validations/verifica-nv-externa/validate.md` antes deste release
- **Branch**: `feature/004-verifica-nv-externa` (HEAD base `f5a6b32`) — sem merge, sem auto-merge, sem force-push
- **Remote/CI/PR/proteções**: `[NV]` até confirmação ao vivo via `gh`

## Changelog (1.0.0)

| ID | Mudança | Origem |
|---|---|---|
| FR-001 | 12 fatos NV-A01–NV-A12 resolvidos nos destinos previstos; rótulo `[NV]` removido apenas nesses fatos | `spec.md` §FR-001; `plan.md` §Inventory |
| FR-002 | Cada substituição com título, URL e data de acesso 2026-11-19 de ≥2 fontes independentes no próprio destino | `plan.md` §D2, §Templates; `research.md` §Síntese |
| FR-003 | Ressalva de escopo de cada fato preservada no destino (Godot, Stride #1666, Lightyear, Defold, Dviglo, Fyrox) | `spec.md` §FR-003; T007–T025 |
| FR-004 | Nenhuma afirmação de benchmark, build, protótipo, execução de engine, custo por tick ou falha empírica; Stride #1666 como inferência documental | `spec.md` §Clarifications Q1; `research-evidence/stride-fyrox.md` |
| FR-005 | Marcadores de classe C preservados integralmente, inclusive ausência de fundação neutra em LÖVE/Haxe/OpenFL em `monogame_love_haxe.md:23` | T026–T029; `spec.md` §FR-005 |
| FR-006 | Marcadores de classe B preservados integralmente (CI, merge, publicação, aprovação remota, proteção de branch) | T030–T031; `spec.md` §FR-006 |
| FR-007 | Nota de superação append-only em `docs/avaliacao-engines/pesquisa/**`, sem reescrita do corpo datado | T007–T009, T013–T015, T020–T021; `plan.md` §D5 |
| FR-008 | Substituição no corpo dos documentos integrados, mantendo o restante do texto inalterado | T010–T012, T016–T019, T022–T025 |
| FR-009 | Nenhuma nota, vetor C1–C10, índice ponderado, score ou recomendação recalculado; divergência do Fyrox corrigida como fato, vetor/índice inalterados | `plan.md` §D6; T017, T036 |
| FR-010 | Nenhum arquivo de `docs/governance/**`, ADR, `STATE.md`, `PROGRESSO.md` ou `converge.md` alterado; contagem canônica A=11/B=92/C=157 preservada como registro datado | T035; `spec.md` §FR-010 |
| FR-011 | Toda afirmação não coberta pelos três arquivos de evidência permanece `[NV]`; nenhuma URL nova sem segunda fonte já registrada | T036; `research.md` §Síntese |
| FR-012 | Data de acesso 2026-11-19 distinguida das datas de versão/publicação das fontes (21/07/2025, 02/12/2024, 29/03/2026 etc.) | `plan.md` §Templates |
| FR-013 | Inventário de FR-013 com ID NV-Axx, arquivo/linha de destino, ressalva e fontes, permitindo auditoria por contagem | `research.md` §Síntese; `plan.md` §Inventory |
| FR-014 | Nenhum gate, stage ou commit executado; nenhuma escrita fora de `specs/004-verifica-nv-externa/` nesta feature de planejamento | T035; `spec.md` §FR-014 |
| T001–T036 | 36 tarefas de inventário, substituição, notas de superação, salvaguardas e auditoria concluídas (`[X]`) | `specs/004-verifica-nv-externa/tasks.md` |
| SC-001–SC-010 | Conformidade registrada no R4: 12/12 fatos, 0 sem duas fontes, 0 benchmarks, 100% B/C preservado, 100% snapshots com nota e 0 linhas reescritas, 0 scores alterados, 10/10 artefatos | `.specify/validations/verifica-nv-externa/validate.md` |
| CHK001–CHK020, CHK001–CHK031 | Rastreabilidade FR↔SC↔CHK nos dois checklists (20/20 e 31/31 itens rastreáveis; marcadores reviewer-owned permanecem `[ ]`) | `checklists/requirements.md`; `checklists/nv-externa-quality.md` |
| DEC-R1 | Veredito `go` da opção A (substituição cirúrgica com ressalvas e notas de superação) | `.specify/assessments/verifica-nv-externa/decision.md` |
| BUG-### | Nenhum bug reportado neste ciclo | — (nenhum registro `BUG-###` no ciclo) |

## Entregáveis versionados

- `.specify/validations/verifica-nv-externa/validate.md` — R4 com `## Evidência final` para o orquestrador colar as saídas.
- Documentos integrados com substituição no corpo: `docs/AVALIACAO-COMPARATIVA-ENGINES.md`, `docs/avaliacao-engines/01-matriz.md`, `docs/avaliacao-engines/02-analise-por-opcao.md`, `docs/avaliacao-engines/03-recomendacao.md`.
- Snapshots históricos com nota de superação append-only (corpo datado byte-intacto): `docs/avaliacao-engines/pesquisa/alternativas.md`, `godot.md`, `defold_flax_stride.md`, `libgdx_urho_panda.md`, `licencas_lockin.md`, `bevy.md`.

## Plano de rollback (documental)

Procedimento aplicável exclusivamente às substituições e às notas de superação introduzidas; a feature
produz somente edições documentais (nenhum código, runtime, banco, migração, binário, configuração de
CI ou serviço externo).

1. Registrar o achado com IDs FR e, quando existirem, `T###` e `BUG-###` (gatilhos: ressalva de
   FR-003 perdida, fonte ausente ou sem data, alegação empírica introduzida, marcador B/C removido,
   linha de snapshot reescrita, score recalculado ou afirmação remota sem confirmação ao vivo).
2. Reverter a substituição ou a nota ao texto anterior em novo ciclo documentado, com nova versão
   SemVer e data — nunca apagar ou reescrever o histórico.
3. Para snapshots históricos, remover somente a nota de superação em ciclo de correção, preservando
   byte-for-byte o corpo datado, inclusive a linha `Data: 19/11/2026`.
4. Reconfirmar que as notas de superação periódicas e as ressalvas restantes permanecem, e que os
   marcadores de classe B/C continuam integralmente preservados.
5. Reconfirmar 0 alterações em notas, vetores C1–C10, índices e recomendações; se a divergência do
   Fyrox tiver sido registrada, manter v1.0.0/RC1/RC2 com vetor/índice inalterados.
6. Reexecutar a auditoria documental por contagens e conferir as saídas coladas no R4; registrar a
   reversão no changelog com versão, data e IDs FR/`T###`/`BUG-###`.

**Exemplo de reversão**: restauração da redação anterior da afirmação de marca Godot em
`02-analise-por-opcao.md` ou do corpo datado de `pesquisa/godot.md`, preservando o histórico e a
versão atual como registro auditável.

**Proibido**: apagar histórico, `force-push`, merge/auto-merge, reescrever snapshots ou governança,
recalcular scores, afirmar CI/merge/aprovação obtidos. **Não aplicável**: rollback de runtime, banco,
migração, binário, configuração de CI ou serviço externo — a feature é exclusivamente documental.

## Restrições

- Nenhum merge, auto-merge, force-push ou deleção de branch executada.
- Nenhuma afirmação de CI/check verde remoto; `[NV]` até confirmação ao vivo.
- Nenhuma portaria posterior a tasks (implement, converge, validate, release) executada como gate,
  stage ou commit por esta feature de planejamento.
- Nenhum código, dependência, engine, serviço externo ou emenda da norma externa introduzido.
- A contagem canônica A=11/B=92/C=157 e a contagem local de 12 fatos permanecem separadas e íntegras.
