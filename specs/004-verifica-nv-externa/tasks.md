# Tasks: Verifica NV externa

**Feature**: `004-verifica-nv-externa` — ver [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md).
**Convenção**: `- [ ] T### [P?] [USn?] descrição com caminho`. Nenhuma tarefa de portaria posterior a tasks (implement, converge, validate ou release) é incluída.

## Phase 1: Setup

- [X] T001 Registrar o inventário de 12 fatos NV-A01–NV-A12 com fontes e ressalvas em `specs/004-verifica-nv-externa/research.md` (síntese e fontes primárias); os destinos arquivo/linha ficam registrados em `specs/004-verifica-nv-externa/plan.md`, conforme FR-013
- [X] T002 Registrar a matriz de citações (título, URL, data e ≥2 fontes por fato) em `specs/004-verifica-nv-externa/plan.md`
- [X] T003 Registrar os templates de citação auditável e de nota de superação append-only em `specs/004-verifica-nv-externa/plan.md`

## Phase 2: Foundational

- [X] T004 Fixar a classificação A/B/C e a lista literal de marcadores preservados em `specs/004-verifica-nv-externa/plan.md`
- [X] T005 Fixar a regra "nota em `docs/avaliacao-engines/pesquisa/**`, substituição nos demais documentos" em `specs/004-verifica-nv-externa/plan.md`
- [X] T006 Definir o procedimento de auditoria por contagens (12 fatos, 0 sem duas fontes, 0 benchmarks, 0 classes B/C alteradas) em `specs/004-verifica-nv-externa/plan.md`

## Phase 3: User Story 1 — Godot: marca e licença com ressalva (P1)

**Goal**: NV-A01 e NV-A02 com escopo completo e duas fontes em cada destino.
**Independent Test**: ler os destinos e confirmar as quatro hipóteses automáticas, a regra do fork comercial e o MIT sem EULA nos canais oficiais, com citações datadas.

- [X] T007 [P] [US1] Acrescentar nota de superação com NV-A01/NV-A02 (política v. 21/07/2025 + Governance + artigo 02/12/2024; `LICENSE.txt` + `/license` + "Complying with licenses") sem reescrever o corpo em `docs/avaliacao-engines/pesquisa/licencas_lockin.md`
- [X] T008 [P] [US1] Acrescentar nota de superação com NV-A01/NV-A02 e a ressalva das quatro hipóteses automáticas sem alterar o corpo datado em `docs/avaliacao-engines/pesquisa/godot.md`
- [X] T009 [P] [US1] Acrescentar nota de superação com NV-A01/NV-A02 sobre o fork Redot e a marca, preservando as linhas 9 e 24 em `docs/avaliacao-engines/pesquisa/alternativas.md`
- [X] T010 [US1] Substituir a afirmação de marca em NV-A01 nas linhas 38 e 91 com política de marcas + governança, sem tocar nos marcadores de classe C em `docs/avaliacao-engines/03-recomendacao.md`
- [X] T011 [US1] Substituir a afirmação C1 de Godot na linha 11 com a ressalva de escopo e duas fontes em `docs/avaliacao-engines/02-analise-por-opcao.md`
- [X] T012 [US1] Substituir a afirmação C1 de Godot na linha 241 com a ressalva de escopo e duas fontes em `docs/AVALIACAO-COMPARATIVA-ENGINES.md`

## Phase 4: User Story 2 — Stride, Fyrox, Urho3D/Dviglo e Defold (P1)

**Goal**: NV-A03–NV-A08 com ressalvas de natureza documental, licença e escopo de conta.
**Independent Test**: ler os destinos e confirmar "sem fix registrado, não testado empiricamente", Dviglo MIT com `license: null` explicado, nenhuma conta Defold no escopo e versões do Fyrox corrigidas sem mudar notas.

- [X] T013 [P] [US2] Acrescentar nota de superação com NV-A03/NV-A04/NV-A08 (API da issue + timeline + release notes 4.4; API do repositório + releases + blog; download + manual + FAQ), sem reescrever as linhas 11, 52 e 53 em `docs/avaliacao-engines/pesquisa/defold_flax_stride.md`
- [X] T014 [P] [US2] Acrescentar nota de superação com NV-A06/NV-A07 (API Urho3D + site About + LICENSE; README Dviglo + `licenses/dviglo/LICENSE` + API) na linha 30, sem reescrever o corpo em `docs/avaliacao-engines/pesquisa/licencas_lockin.md`
- [X] T015 [P] [US2] Acrescentar nota de superação com NV-A06/NV-A07 na linha 8 (Urho3D arquivado 25/01/2023; Dviglo MIT e último push 09/03/2024), preservando os marcadores de classe C nas linhas 13–25 em `docs/avaliacao-engines/pesquisa/libgdx_urho_panda.md`
- [X] T016 [US2] Substituir "sem correção na 4.4" na linha 41 por "issue aberta após 4.4.3, sem fix registrado, não testado empiricamente" com API da issue + release notes 4.4 em `docs/avaliacao-engines/02-analise-por-opcao.md`
- [X] T017 [US2] Corrigir a versão do Fyrox na linha 116 para v1.0.0 (29/03/2026), RC1 (21/10/2025) e RC2 (09/02/2026), MIT e repositório ativo, com API do repositório + `LICENSE.md` + API de releases, sem alterar vetor, índice ou nota em `docs/avaliacao-engines/02-analise-por-opcao.md`
- [X] T018 [US2] Substituir a menção à issue #1666 na linha 83 com a ressalva documental e duas fontes em `docs/avaliacao-engines/01-matriz.md`
- [X] T019 [US2] Substituir a menção a Urho3D/Dviglo na linha 148 (arquivado 2023; Dviglo parado 09/03/2024) com API + site + licenças em `docs/AVALIACAO-COMPARATIVA-ENGINES.md`

## Phase 5: User Story 3 — Lightyear ↔ Bevy: matriz, book e cadência (P1)

**Goal**: NV-A09–NV-A12 com a tabela upstream, o estado WIP do book, a ressalva do ciclo 0.19 e a dupla licença.
**Independent Test**: ler os destinos e confirmar 0.28–0.30→0.19 e 0.31→0.20, "collection of notes", ciclo 0.19 sem generalização e `MIT OR Apache-2.0`, com duas fontes cada.

- [X] T020 [P] [US3] Acrescentar nota de superação com NV-A09/NV-A10/NV-A11/NV-A12 (README + crates.io + `Cargo.toml` + `introduction.md` + `SUMMARY.md` + Releases + licenças), sem reescrever as linhas 11, 19, 23 e 26 em `docs/avaliacao-engines/pesquisa/bevy.md`
- [X] T021 [P] [US3] Acrescentar nota de superação com NV-A12 na linha 36 (`Cargo.toml` + `LICENSE-MIT`/`LICENSE-APACHE` + crates.io; ressalva da UI parcial do GitHub) em `docs/avaliacao-engines/pesquisa/licencas_lockin.md`
- [X] T022 [US3] Substituir a cadência na linha 45 por "observação do ciclo 0.19, não lei geral; 1 a 13 publicações por ciclo; Releases defasada", com crates.io + Releases + README em `docs/avaliacao-engines/02-analise-por-opcao.md`
- [X] T023 [US3] Substituir matriz, book e cadência na linha 80 com README + crates.io + `Cargo.toml` + book, mantendo os marcadores de classe C intactos em `docs/avaliacao-engines/01-matriz.md`
- [X] T024 [US3] Substituir matriz, book e cadência na linha 155 (O3 C5/C8) com as fontes e a ressalva do ciclo 0.19, sem alterar notas em `docs/AVALIACAO-COMPARATIVA-ENGINES.md`
- [X] T025 [US3] Substituir a cadência na linha 88 com a ressalva do ciclo 0.19 e duas fontes, preservando os marcadores de classe C das linhas 83–92 em `docs/avaliacao-engines/03-recomendacao.md`

## Phase 6: User Story 4 — Salvaguardas e verificação documental (P2)

**Goal**: nenhuma salvaguarda apagada, nenhum empirismo introduzido, histórico preservado, contagens fechadas.
**Independent Test**: percorrer as listas de FR-005/FR-006/FR-007 e confirmar 100% de preservação e 0 linhas reescritas em snapshots.

- [X] T026 [P] [US4] Confirmar a preservação integral dos marcadores de classe C nas linhas 17, 24, 29 e 35–39 em `docs/avaliacao-engines/pesquisa/godot.md`
- [X] T027 [P] [US4] Confirmar a preservação integral dos marcadores de classe C nas linhas 3, 24 e 32 (benchmark e custo por tick) em `docs/avaliacao-engines/pesquisa/bevy.md`
- [X] T028 [P] [US4] Confirmar a preservação integral da matriz de capacidades `[NV]` nas linhas 21–28 em `docs/avaliacao-engines/pesquisa/netcode_modularidade.md`
- [X] T029 [US4] Confirmar a preservação integral dos marcadores de classe C nas linhas 20 e 83–92 em `docs/avaliacao-engines/03-recomendacao.md`, nas linhas 19, 29, 37, 63, 65, 75, 86 e 97 em `docs/avaliacao-engines/02-analise-por-opcao.md` e na linha 23 em `docs/avaliacao-engines/pesquisa/monogame_love_haxe.md` (ausência de fundação neutra em LÖVE/Haxe/OpenFL; nenhum apagado), antes de fechar SC-004
- [X] T030 [P] [US4] Confirmar a preservação integral dos marcadores de classe B/C nas linhas 309, 317 e 334–346 em `docs/AVALIACAO-STACK-MDMV.md`
- [X] T031 [P] [US4] Confirmar a preservação integral dos marcadores de classe B/C em `docs/avaliacao-stack/02-aroeira.md` e `docs/avaliacao-stack/03-insurgencia.md`
- [X] T032 [US4] Registrar as contagens finais (12 fatos, 0 sem duas fontes, 0 benchmarks, 100% B/C preservado, 0 linhas de snapshot reescritas) em `specs/004-verifica-nv-externa/plan.md` — discharge: FR-013 e SC-001–SC-006

## Phase 7: Polish & cross-cutting concerns

- [X] T033 [P] Auditar os 10 artefatos de spec/checklist em `specs/004-verifica-nv-externa/` (intake, research, problem, concept, decision, spec, requirements, plan, nv-externa-quality, tasks — `research-evidence/**` não entra nesta contagem) quanto a seção vazia, `TBD` ou `[NEEDS CLARIFICATION]` pendente — discharge: SC-010
- [X] T034 [P] Auditar a rastreabilidade FR↔SC↔CHK em `specs/004-verifica-nv-externa/checklists/requirements.md` e `specs/004-verifica-nv-externa/checklists/nv-externa-quality.md` — discharge: cobertura FR-001–FR-014 ↔ SC-001–SC-010 nos checklists
- [X] T035 [P] Consolidar o registro documental de fechamento (contagens, divergências registradas e nenhuma portaria posterior executada) em `specs/004-verifica-nv-externa/plan.md` — discharge: verificações negativas FR-010, FR-014 e SC-009
- [X] T036 [US4] Verificação negativa final: confirmar FR-011 (toda afirmação não coberta pelos três arquivos de evidência permanece `[NV]`, e nenhuma URL nova introduzida sem segunda fonte já registrada em `research.md`), SC-007 (0 notas, vetores, índices, scores e recomendações alterados) e SC-008 (100% das substituições com a ressalva de FR-003 no próprio destino); registrar o resultado em `specs/004-verifica-nv-externa/plan.md`

## Dependencies

- Setup (T001–T003) → Foundational (T004–T006) → histórias 1, 2 e 3 (independentes entre si) → história 4 (depende das substituições) → Polish (T033–T036).
- T007–T012, T013–T019 e T020–T025 podem rodar em paralelo entre histórias; dentro de cada história, as edições em snapshots e em documentos integrados também são paralelas quando em arquivos distintos.
- T026–T032 dependem da conclusão das respectivas substituições; T033–T036 dependem de todas as fases anteriores.

## Parallel Execution Examples

- **Onda 1**: T001 + T002 + T003 (inventário, matriz de fontes e templates em arquivos distintos da spec).
- **Onda 2**: T007 + T008 + T009 (três snapshots de Godot) em paralelo; simultaneamente T013 + T014 + T015 (snapshots de Stride/Dviglo) e T020 + T021 (snapshots de Bevy/Lightyear).
- **Onda 3**: T010 + T011 + T012 (integrados de Godot), T016 + T017 + T018 + T019 (integrados de Stride/Fyrox), T022 + T023 + T024 + T025 (integrados de Bevy/Lightyear).
- **Onda 4**: T026 + T027 + T028 + T030 + T031 (preservação em arquivos distintos) e T029 (após T010/T025).

## Implementation Strategy

- MVP documental: User Story 1 (Godot) entrega o primeiro lote de fatos com ressalva e valida o template de citação antes das histórias 2 e 3.
- Incremental: cada história completa um cluster temático e pode ser auditada isoladamente; a história 4 fecha a verificação das salvaguardas.
- Nenhuma tarefa executa gates, build, teste, stage, commit ou efeito remoto; a verificação é leitura e contagem conforme `plan.md` §Abordagem de verificação.
