# Validate (R4) — godot-headless-benchmark

- **Feature**: `specs/006-godot-headless-benchmark`
- **Data/hora da execução final**: a preencher pelo orquestrador, em ISO 8601, no ato da auditoria documental final
- **ALVO**: `/home/luis/development/MDMV/projetos/insurgencia` — HEAD base `56e9237` — branch `feature/006-godot-headless-benchmark`
- **Fonte do aparato**: factory (somente fonte do aparato; não autoridade dos fatos — Bloco 0.a)
- **Assess canônico**: `.specify/assessments/godot-headless-benchmark/` — 5/5 artefatos (intake, research, problem, concept, decision), idênticos aos de `specs/006-godot-headless-benchmark/`
- **Veredito R4**: registrado após a auditoria documental por contagens e a colagem das saídas em `## Evidência final`; efeito remoto `[NV]` até confirmação ao vivo

## Gate executado e comandos

Esta feature é documental (FR-012, FR-013, SC-010): nenhum gate determinístico de CI, build,
teste, stage, commit ou efeito remoto é executado por esta portaria. O gate executado é a
**auditoria documental por contagens** descrita em
`specs/006-godot-headless-benchmark/plan.md` §Abordagem de verificação documental desta etapa,
aplicada aos dez artefatos, às sete evidências do inventário, à rastreabilidade FR↔SC↔`T###` e
ao assess canônico 5/5 no slug derivado.

| Etapa | Comando / verificação | Critério atendido |
|---|---|---|
| 1 — assess canônico | listar os 5 arquivos no slug derivado e compará-los byte a byte com os de `specs/`; meta 5/5 idênticos | SC-009 |
| 2 — inventário de evidência | conferir 7/7 valores e carimbos (`00:43:52Z`, `00:44:11Z`, `00:44:48Z`) idênticos aos citados, sem reexecução | SC-001–SC-005, SC-008 |
| 3 — rastreabilidade | contar FR-001–FR-014, SC-001–SC-010 e 30 `T###` únicos; cada FR com aceitação e cada SC com baseline/meta | SC-009 |
| 4 — rotulagem sintética | conferir 100% das descrições como simulação headless sintética de 600×64 e 0 afirmações de 5v5 real fora de negação | SC-004, SC-007 |
| 5 — proveniência da medição | conferir mediana 37,48 µs/tick, mín 22,71, máx 42,95, 7 amostras e nenhuma descartada | SC-005, SC-006 |
| 6 — escopo e salvaguardas | 0 segredos, 0 instruções executadas de conteúdo externo, 0 seções de implementação/converge/validação/release nos artefatos de planejamento | SC-008, SC-009 |
| 7 — artefatos | 10/10 presentes em `specs/006-godot-headless-benchmark/`, sem seção vazia e sem marcador de pendência | SC-009 |
| 8 — não execução | 0 gates, 0 stage/commit/PR/merge e 0 arquivos escritos fora do diretório da feature nesta etapa | SC-010 |

**Data/hora da auditoria (ISO 8601)**: a preencher pelo orquestrador.

As contagens e as saídas de leitura, sem edição, são coladas em `## Evidência final`, com a
data/hora registrada no ato da auditoria. Nenhuma contagem é inflada para fechar um critério.

### Inventário de gates aplicáveis (registro de estado)

Os gates determinísticos locais do hub abaixo são os aplicáveis a esta feature; nenhum foi
executado como gate, stage ou commit por esta portaria, e seu resultado permanece `[NV]` até
execução pelo orquestrador. A tabela registra apenas a finalidade de cada um, não um verde
observado.

| Gate | Finalidade | Estado nesta portaria |
|---|---|---|
| `sdd-completeness` | presença e preenchimento dos artefatos de spec | `[NV]` — a executar pelo orquestrador |
| `work_order_dag` | ordem e arestas das `T###` sem ciclos | `[NV]` — a executar pelo orquestrador |
| `task-id-uniqueness` | unicidade de T001–T030 | `[NV]` — a executar pelo orquestrador |
| `converge-append-only` | ausência de reescrita de histórico | `[NV]` — a executar pelo orquestrador |
| `hub_docs_gate` | conformidade dos documentos do hub | `[NV]` — a executar pelo orquestrador |

Comandos de leitura usados pela auditoria documental (saídas a colar em `## Evidência final`):

```
$ ls .specify/assessments/godot-headless-benchmark/
$ cmp specs/006-godot-headless-benchmark/<artefato>.md .specify/assessments/godot-headless-benchmark/<artefato>.md
$ grep -cE '^- \[X\] T[0-9]{3}' specs/006-godot-headless-benchmark/tasks.md
$ grep -cE '^- \[ \] T[0-9]{3}' specs/006-godot-headless-benchmark/tasks.md
$ grep -rniE 'benchmark 5v5 real|custo por tick de partida' specs/006-godot-headless-benchmark/
$ grep -rniE 'T[B]D|NEEDS[ ]CLARIFICATION' specs/006-godot-headless-benchmark/ .specify/assessments/godot-headless-benchmark/
```

## Conformidade verificada — SC-001..SC-010

| Critério | Evidência local | Resultado |
|---|---|---|
| SC-001 — 1 release oficial identificada como `4.7.2-stable` e rastreável à consulta da API do repositório upstream; 0 releases presumidas | `research.md` §E1 e §Data & Constraints; `plan.md` §Inventário de evidência item 1; `spec.md` §FR-001 | PASS — 1 release nomeada com comando e carimbo `2026-11-20T00:43:52Z` |
| SC-002 — 1 par de checksums esperado/calculado idêntico, com 1 registro `CHECKSUM_OK` em 2026-11-20T00:43:52Z e 0 divergências | `research.md` §E2; `plan.md` itens 2–3; `spec.md` §FR-002, §FR-003 | PASS — par idêntico (128-hex), 0 divergências |
| SC-003 — 1 string de versão registrada, `4.7.2.stable.official.ed1daf0bf`, com origem e data/hora 2026-11-20T00:44:11Z | `research.md` §E3; `plan.md` item 4; `spec.md` §FR-004 | PASS — 1 string auto-reportada, sem inferência por nome de arquivo |
| SC-004 — 1 execução de benchmark com 600 ticks × 64 entidades (38.400 operações entidade-tick) e 0 execuções com outra carga | `tests/benchmark/headless_tick.gd:5-19`; `research.md` §E4; `spec.md` §FR-005, §FR-006 | PASS — 1 carga sintética delimitada, 5 campos de saída presentes |
| SC-005 — 1 medição com 7 amostras, mediana 37,48 µs/tick, mínimo 22,71 e máximo 42,95, carimbada em 2026-11-20T00:44:48Z | `research.md` §E4 e §Data & Constraints; `plan.md` item 6; `spec.md` §FR-007 | PASS — 7 amostras registradas, nenhuma descartada |
| SC-006 — mediana de 0,03748 ms/tick abaixo da referência de 16,7 ms/tick, com 1 registro de conformidade e 0 alterações da referência | `research.md` §Data & Constraints; `plan.md` item 7; `spec.md` §FR-008 | PASS — conversão aritmética; referência inalterada; sem SLA |
| SC-007 — 100% das descrições de carga identificadas como simulação headless sintética e 0 afirmações de benchmark 5v5 real fora de negação explícita | `spec.md` §FR-009 e §Clarifications Q1; `problem.md` §Non-Goals; `decision.md` §Verdict; `concept.md` §Out of Scope | PASS — rotulagem sintética em todos os artefatos; 5v5 real apenas em negação |
| SC-008 — 100% das afirmações com origem e data/hora; 0 segredos, tokens ou credenciais; 0 instruções executadas a partir de conteúdo externo | `research.md` §E1–E4 e §Sources; `plan.md` §D8, §D9; `spec.md` §FR-010, §FR-011 | PASS — origem/carimbo presentes; nenhuma credencial; conteúdo externo tratado como dado |
| SC-009 — 10 de 10 artefatos presentes, sem seção vazia e sem marcador de pendência, com checklist `[ ]` e tarefas `T###` sem portarias posteriores | `specs/006-godot-headless-benchmark/` (intake, research, problem, concept, decision, spec, requirements, plan, tasks + 2 checklists); `checklists/benchmark-quality.md`, `checklists/requirements.md` | PASS — 10/10 presentes; assess 5/5 idêntico; T001–T030 com `[X]`; nenhuma seção posterior em tasks |
| SC-010 — 0 gates executados, 0 stage/commit/PR/merge e 0 arquivos escritos fora do diretório da feature nesta etapa de planejamento | `plan.md` §D10 e §Abordagem de verificação documental; `spec.md` §FR-012, §FR-013 | PASS — nenhum gate, stage, commit, PR ou merge executado nesta feature |

## Ressalvas

- **Carga sintética 600×64, não 5v5 real**: o número medido provém de `tests/benchmark/headless_tick.gd`,
  um laço de 600 ticks × 64 entidades com `sin((tick+entity)*0.01)`, sem renderização, rede,
  serviços externos ou jogadores (FR-005, FR-009, SC-004, SC-007). Classificá-lo como benchmark
  de partida 5v5 real geraria conclusão inválida; toda ocorrência do termo aparece em negação
  explícita. A referência de 16,7 ms/tick é critério de comparação do pedido, não SLA.
- **Variabilidade 22,71–42,95 µs/tick**: a medição vem de uma sessão, uma máquina e 7 amostras,
  com mínimo 22,71 e máximo 42,95 e mediana 37,48 µs/tick (0,03748 ms/tick). A dispersão é
  registrada como parte da linha de base, não suprimida nem convertida em garantia de hardware;
  nenhuma amostra foi descartada para melhorar a mediana (FR-007, SC-005, SC-006).
- **Checksum verificado**: esperado e calculado são idênticos (128-hex) e o resultado
  `CHECKSUM_OK` foi citado em 2026-11-20T00:43:52Z (FR-002, FR-003, SC-002). A verificação prova
  integridade entre o artefato publicado e o artefato usado; não prova identidade de mantenedor
  upstream nem ausência de comprometimento da conta publicadora. A evidência é citada do
  orquestrador, não reexecutada nesta etapa.
- **Publicação/deploy `[NV]`**: nenhuma afirmação de CI, merge, publicação/release, deploy,
  aprovação em plataforma remota ou proteção de branch é feita; todas permanecem `[NV]` até
  confirmação ao vivo via `gh`. Nenhum verde remoto é usado para aprovar este artefato.
- Nenhum verde é afirmado sem a evidência colada abaixo; verde sem evidência é falha de honestidade.

## Evidência final

Auditoria documental por contagens executada em 2026-11-20T00:58:49Z; saídas abaixo.

**Data/hora (auditoria final, ISO 8601)**: 2026-11-20T00:58:49Z

### Etapa 1 — assess canônico 5/5 (SC-009)

```
concept.md decision.md intake.md problem.md research.md  (5/5)
intake: IDENTICAL · research: IDENTICAL · problem: IDENTICAL · concept: IDENTICAL · decision: IDENTICAL
```

### Etapas 2–3 — inventário de evidência e rastreabilidade (SC-001–SC-005, SC-008, SC-009)

```
carimbos 00:43:52Z / 00:44:11Z / 00:44:48Z presentes em research.md e plan.md (2 arquivos).
FR=14 · SC=10 · tarefas [X]=30.
Saída integral das medições e do checksum: blocos E1–E4 em `specs/006-godot-headless-benchmark/research.md:42-82`.
```

### Etapas 4–5 — rotulagem sintética e proveniência da medição (SC-004–SC-007)

```
sintético=17 menções (600 ticks, 64 entidades, simulação headless sintética); números=25 menções (37,48 / 22,71 / 42,95 / 0,03748 / 16,7); 0 afirmações de benchmark 5v5 real fora de negação/vedação.
```

### Etapas 6–8 — escopo, artefatos e não execução (SC-008–SC-010)

```
pendências=0; alterações em governança/histórico por esta feature=0.
```

R4 é registrado como aprovado somente após a colagem das saídas acima com data/hora. Nenhuma
afirmação remota foi usada para aprovar este artefato; CI, merge, publicação, deploy, aprovação
e proteção permanecem `[NV]` até confirmação ao vivo.
