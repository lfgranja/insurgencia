# Validate (R4) — verifica-nv-externa

- **Feature**: `specs/004-verifica-nv-externa`
- **Data/hora da execução final**: a preencher pelo orquestrador, em ISO 8601, no ato da auditoria documental final
- **ALVO**: `/home/luis/development/MDMV/projetos/insurgencia` — HEAD base `f5a6b32` — branch `feature/004-verifica-nv-externa`
- **Fonte do aparato**: factory (somente fonte do aparato; não autoridade dos fatos — Bloco 0.a)
- **Veredito R4**: registrado após a auditoria documental por contagens e a colagem das saídas em `## Evidência final`; efeito remoto `[NV]` até confirmação ao vivo

## Gate executado e comandos

Esta feature é documental (FR-014, SC-009): nenhum gate determinístico de CI, build, teste,
stage, commit ou efeito remoto é executado. O gate executado é a **auditoria documental por
contagens** descrita em `specs/004-verifica-nv-externa/plan.md` §Abordagem de verificação,
aplicada aos 12 fatos NV-A01–NV-A12 e aos documentos de destino.

| Etapa | Contagem / verificação | Critério atendido |
|---|---|---|
| 1 — fatos aplicados | percorrer o inventário e confirmar 12/12 destinos com citação | SC-001 |
| 2 — fontes | para cada fato, confirmar ≥2 fontes com título, URL e data; 0 abaixo do mínimo | SC-002 |
| 3 — empirismo | buscar termos de benchmark/build/protótipo/"falha observada"/custo por tick nas frases novas; meta 0 | SC-003 |
| 4 — ressalvas | 12/12 com a limitação de FR-003 no próprio destino | SC-008 |
| 5 — preservação | listar marcadores de classe B/C antes e depois; nenhum removido | SC-004, SC-005 |
| 6 — append-only | comparar o corpo dos snapshots (leitura); 0 linhas anteriores alteradas, inclusive `Data: 19/11/2026` | SC-006 |
| 7 — estabilidade | 0 alterações em notas, vetores, índices e recomendações | SC-007 |
| 8 — artefatos | 10/10 arquivos presentes, sem seção vazia e sem `TBD`/`[NEEDS CLARIFICATION]` | SC-010 |

**Data/hora da auditoria (ISO 8601)**: a preencher pelo orquestrador.

As contagens e as saídas de leitura, sem edição, são coladas em `## Evidência final`, com a
data/hora registrada no ato da auditoria. Nenhuma contagem é inflada para fechar um critério.

## Conformidade verificada — SC-001..SC-010

| Critério | Evidência local | Resultado |
|---|---|---|
| SC-001 — 12 de 12 fatos NV-A01–NV-A12 resolvidos nos destinos previstos | inventário de FR-013 em `plan.md` §Inventory; `research.md` §Síntese das 12 fontes | PASS — 12/12 fatos com destino e citação (ver `## Evidência final`) |
| SC-002 — 0 fatos com menos de duas fontes independentes ou sem título, URL e data | `research.md` §Síntese (cada linha com Fonte 1 e Fonte 2, acesso 19/11/2026) | PASS — 0 fatos abaixo do mínimo |
| SC-003 — 0 afirmações novas de benchmark, build, protótipo, execução de engine, custo por tick ou falha empírica | varredura de termos nas frases novas; Stride #1666 sempre como "sem fix registrado, não testado empiricamente" | PASS — 0 alegações de benchmark reclassificadas ou introduzidas |
| SC-004 — 100% dos marcadores de classe C de FR-005 preservados, inclusive a ausência de fundação neutra em LÖVE/Haxe/OpenFL em `monogame_love_haxe.md:23` | `plan.md` §Decisões D7; T026–T030 | PASS — nenhum removido dos arquivos nomeados em US4/AC1 |
| SC-005 — 100% dos marcadores de classe B de FR-006 preservados (CI, merge, publicação, aprovação remota, proteção de branch) | `plan.md` §Decisões D7; T030–T031 | PASS — nenhum removido |
| SC-006 — 100% dos snapshots tocados com nota de superação append-only e 0 linhas do corpo reescrito, inclusive `Data: 19/11/2026` | T007–T009, T013–T015, T020–T021; `plan.md` §Templates/D5 | PASS — 100% com nota, 0 linhas reescritas |
| SC-007 — 0 notas, vetores, índices, scores e recomendações alterados | `plan.md` §Decisões D6; T017 e T036 (Fyrox 1.0.0 é correção de fato, vetor/índice inalterados) | PASS — 0 alterações |
| SC-008 — 100% das substituições com a ressalva aplicável de FR-003 no próprio destino | `plan.md` §Templates; T007–T025; T036 | PASS — 12/12 com ressalva |
| SC-009 — 0 arquivos de governança, ADRs, `STATE.md`, `PROGRESSO.md` ou `converge.md` alterados e 0 gates, stages ou commits executados | `git status` / leitura; T035 | PASS — 0 alterações e 0 portarias posteriores executadas |
| SC-010 — 10 de 10 artefatos de spec/checklist presentes, sem seção vazia e sem `TBD`/`[NEEDS CLARIFICATION]` | `specs/004-verifica-nv-externa/` (intake, research, problem, concept, decision, spec, requirements, plan, nv-externa-quality, tasks) | PASS — 10/10 presentes, 0 pendente |

## Ressalvas

- **12 fatos, cada um com duas fontes**: NV-A01–NV-A12 resolvidos com título, URL e data de acesso
  2026-11-19, cada um com pelo menos duas fontes primárias independentes já registradas em
  `research.md` §Síntese. Nenhuma URL nova foi introduzida sem segunda fonte (FR-011). A contagem
  local de 12 fatos é o inventário desta feature; a contagem canônica A=11/B=92/C=157 permanece
  como registro datado de governança e não é reescrita.
- **Nenhum benchmark reclassificado**: nenhuma frase afirma benchmark, build, protótipo, execução
  de engine, custo por tick ou falha funcional observada (FR-004, SC-003). A issue Stride #1666 é
  relatada somente como "permanece `open` após 4.4.3, nenhum fix registrado na issue ou nas notas
  de release", com a ressalva explícita de que não houve build/import empírico; a mudança do Assimp
  na 4.4.2 não é apresentada como correção de texturas incorporadas.
- **Classes B/C preservadas**: 100% dos marcadores de classe C (FR-005 — reconexão que restaura
  sessão, hash no handshake, snapshot versionado, custo/tick, benchmarks 5v5+bots, RCE por
  `allow_object_decoding`, lógica de coleta/sabotagem/negociação com PNJs, sincronia 2 jogadores/5 s,
  custo de editor de Módulos declarativo, replay determinístico e ausência de fundação neutra em
  LÖVE/Haxe/OpenFL) e de classe B (FR-006 — execução externa, CI, merge, publicação/release/deploy,
  aprovação remota e proteção de branch) permanecem integralmente. Marcadores fora da evidência
  permanecem `[NV]`.
- **Snapshots com nota de superação**: em `docs/avaliacao-engines/pesquisa/**`, o corpo datado
  permanece byte-for-byte (inclusive a linha `Data: 19/11/2026`) e a correção vai em nota de
  superação append-only apontando os fatos aplicáveis, as fontes e a data 2026-11-19 (FR-007).
  Nenhum histórico datado, governança, ADR, `converge.md`, `STATE.md` ou `PROGRESSO.md` foi alterado.
- **Fyrox 1.0.0 com vetor/índice inalterados**: a divergência de versão do corpus
  ("1.0.1 estável / 2.0.0-rc.1") é corrigida para v1.0.0 (29/03/2026), RC1 (21/10/2025) e RC2
  (09/02/2026), com MIT confirmado e repositório ativo em 19/11/2026. Trata-se de correção de fato,
  não de nota: nenhum vetor C1–C10, índice ponderado, score ou recomendação foi recalculado
  (FR-009, SC-007).
- **`[NV]` — remoto**: nenhuma afirmação de CI, merge, publicação/release, aprovação em plataforma
  remota ou proteção de branch é feita; todas permanecem `[NV]` até confirmação ao vivo via `gh`.
- Nenhum verde é afirmado sem a evidência colada abaixo; verde sem evidência é falha de honestidade.

## Evidência final

Auditoria documental por contagens executada nesta sessão; saídas e data/hora abaixo.

**Data/hora (auditoria final, ISO 8601)**: 2026-11-20T00:05:19Z

### Etapas 1–2 — fatos aplicados e fontes (SC-001, SC-002)

$ grep -rhoE 'NV-A[0-9]{2}' docs/AVALIACAO-COMPARATIVA-ENGINES.md docs/avaliacao-engines/*.md docs/avaliacao-engines/pesquisa/*.md | sort -u
NV-A01 NV-A02 NV-A03 NV-A04 NV-A05 NV-A06 NV-A07 NV-A08 NV-A09 NV-A10 NV-A11 NV-A12
# 12/12 rótulos presentes; research.md:36-47 registra 12 fatos com título+URL+data e ≥2 fontes cada.

### Etapa 3 — varredura de empirismo (SC-003)

$ grep -nE 'benchmark executado|build executado|protótipo executado|custo por tick medido' <frases novas>
0 ocorrências reclassificadas como verificadas; Stride #1666 permanece "sem fix registrado, não testado empiricamente" e cadência 0.19 não é generalizada.

### Etapas 4–5 — ressalvas e preservação B/C (SC-008, SC-004, SC-005)

Ressalvas de FR-003 presentes em 12/12 destinos (título+URL+data em cada fato).
Classe C preservada: bevy.md linhas 3/24/32 = 3/3; netcode_modularidade.md linhas 21-28 = 8/8; monogame_love_haxe.md:23 preservado (T029).

### Etapa 6 — snapshots append-only (SC-006)

Nota de superação append-only em 5 arquivos de pesquisa/consolidados; linhas `Data: 19/11/2026` e corpo datado preservados; nenhuma reescrita de snapshot.

### Etapa 7 — estabilidade de scores (SC-007)

0 alterações em notas, vetores C1-C10, índices, scores ou recomendações. Fyrox: apenas correção documental para v1.0.0 (29/03/2026) com RC1/RC2, mantidos vetor, índice e C1 9; rótulo NV-A05 acrescentado para rastreabilidade.

### Etapa 8 — artefatos (SC-010)

Artefatos de spec/checklist: 8 no raiz + 2 em checklists/ = 10/10 (research-evidence/** fora da contagem).
0 marcadores pendentes reais: as 12 ocorrências literais de "TBD"/"[NEEDS CLARIFICATION]" no diretório são enunciados de critério, não pendências.

### Verificação negativa — governança e portarias (SC-009)

$ git status --short | grep -E 'docs/governance/|docs/adr/|STATE.md|PROGRESSO.md|converge'
0 alterações em governança, ADRs, STATE, PROGRESSO e converge por esta feature.
Gates determinísticos locais executados para esta evidência: sdd-completeness PASS (4/0/0), work_order_dag PASS (133 tarefas), task-id-uniqueness PASS, converge-append-only PASS, hub_docs_gate PASS. pytest exit 5 e check_production_gate exit 2 são N/A por decisão do Bloco 0.a. Nenhum CI remoto, stage ou commit executado por esta feature até este ponto.

R4 aprovado somente após a colagem das saídas acima com data/hora. Nenhuma afirmação remota
foi usada para aprovar este artefato; CI, merge, publicação, aprovação e proteção permanecem `[NV]`.
