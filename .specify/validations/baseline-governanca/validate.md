# Validate (R4) — baseline-governanca

- **Feature**: `specs/003-baseline-governanca`
- **Data/hora da execução final**: a preencher pelo orquestrador, em ISO 8601, no ato da execução dos gates
- **ALVO**: `/home/luis/development/MDMV/projetos/insurgencia` — HEAD base `a5e4131` — branch `feature/003-baseline-governanca`
- **Fonte do aparato**: factory (somente fonte do aparato; não autoridade dos baselines — Bloco 0.a)
- **Veredito R4**: registrado após a execução dos três gates locais e a colagem da saída em `## Evidência final`; efeito remoto `[NV]` até confirmação ao vivo

## Gates executados e comandos

Os três gates abaixo foram executados localmente com os baselines canônicos de `docs/governance/`.
A saída integral, sem edição, de cada um é colada em `## Evidência final`, com a data/hora
registrada no ato da execução.

| Gate | Baseline | Comando |
|---|---|---|
| `G-COMP` | `docs/governance/sdd-completeness-baseline.json` | `python3 scripts/ci/check_sdd_completeness.py --baseline docs/governance/sdd-completeness-baseline.json` |
| `G-CONV` | `docs/governance/converge-append-only-baseline.json` | `python3 scripts/ci/check_converge_append_only.py --baseline docs/governance/converge-append-only-baseline.json` |
| `G-UNIQ` | `docs/governance/task-id-uniqueness-baseline.json` | `python3 scripts/ci/check_task_id_uniqueness.py --baseline docs/governance/task-id-uniqueness-baseline.json` |

**Data/hora da execução (ISO 8601)**: a preencher pelo orquestrador.

## Conformidade verificada — SC-001..SC-008

| Critério | Evidência local | Resultado |
|---|---|---|
| SC-001 — 3 de 3 baselines presentes em `docs/governance/` | `sdd-completeness-baseline.json`, `converge-append-only-baseline.json`, `task-id-uniqueness-baseline.json` | PASS — 3/3 presentes, JSON válidos com as chaves de contrato §2 |
| SC-002 — 100% das entradas do estado corrente no baseline; 0 inventadas, 0 da factory | inventário vazio dos três gates + `_provenance` | PASS — mapas vazios refletem o estado corrente; 0 entradas inventadas |
| SC-003 — 3 de 3 gates PASS (exit 0), 0 regressões, 0 `stale_baseline` | saída dos três gates em `## Evidência final` | ver `## Evidência final` (contagens e exit codes colados) |
| SC-004 — `baselined>0` só onde há entradas; mapas vazios honestos | `allow_missing={}`, `files={}`, `allow_duplicates={}` | PASS com ressalva — `baselined=0` aceito: arquivo materializado, proveniência registrada e saída colada |
| SC-005 — 0 chaves de spec da factory, 0 entradas órfãs | comparação com o inventário de `scripts/ci/` | PASS — 0 chaves da factory, 0 órfãs |
| SC-006 — 0 alterações em lógica/limiares/schemas/caminhos/regex dos 3 scripts | `sha256sum` vs. contrato §8.1 (`146fd007…`, `a51a5c6e…`, `b6c5e9ca…`) | PASS — 3/3 digests iguais ao registro de imutabilidade |
| SC-007 — 3 de 3 saídas coladas com data/hora; remoto `[NV]` | campo `## Evidência final` | campo preparado para colagem pelo orquestrador; CI/merge/publicação/aprovação/proteção `[NV]` |
| SC-008 — cenário de ratchet recusado por 3 de 3 gates | contrato §4.3; contagens/exit codes registrados em tarefas | RG-2/RG-3/RG-4 executáveis e recusados; RG-1/RG-5 CONDICIONAIS (ver Ressalvas) |

## Ressalvas

- **Mapas vazios são honestos**: `allow_missing={}`, `files={}` e `allow_duplicates={}` refletem o
  estado corrente verificado (nenhum `converge.md`, nenhuma duplicata `T###`, nenhuma lacuna
  baselinável de portaria já concluída). A dívida 1 fecha pela presença do arquivo local, do schema
  conhecido, da proveniência `alvo-local` e da saída colada — não por contagem inflada. Release e
  validate da própria feature 003 são artefatos de portaria R4/R7, não lacuna baselinável.
- **RG-1 e RG-5 são CONDICIONAIS e não foram executados**: dependem de uma entrada baselinada que
  não existe neste estado (`converge.md` em `files` para RG-1; `duplicate_task:T###` em
  `allow_duplicates` para RG-5). Não são afirmados como executados. A prova de ratchet executável
  neste estado é dada por RG-2 (`G-CONV`), RG-3 (`G-COMP`) e RG-4 (`G-UNIQ`), com recusa e exit 1.
  Os cenários foram aplicados de forma isolada e reversível, sem tocar em arquivos reais nem alterar
  os scripts.
- **Apenas contagens e exit codes em tarefas/artefatos de planejamento**: `tasks.md`,
  `data-model.md` e contrato §4.3 registram somente contagens e exit codes. A saída integral com
  data/hora pertence exclusivamente a este artefato R4, em `## Evidência final`.
- **`[NV]` — remoto**: nenhuma afirmação de CI, merge, publicação, aprovação em plataforma ou
  proteção de branch é feita; todas permanecem `[NV]` até confirmação ao vivo.
- Nenhum verde é afirmado sem a saída colada abaixo; verde sem evidência é falha de honestidade.

## Evidência final

Execução realizada em 2026-11-19T23:37:11Z (comandos com `--baseline` executados em seguida, 23:38Z). Saídas integrais abaixo.

**Data/hora (execução final, ISO 8601)**: 2026-11-19T23:37:11Z

### G-COMP — completude SDD

```
$ python3 scripts/ci/check_sdd_completeness.py --baseline docs/governance/sdd-completeness-baseline.json
sdd-completeness: PASS (spec_dirs=3 gaps=0 baselined=0 strict_from=88)
exit=0
```

### G-CONV — convergência append-only

```
$ python3 scripts/ci/check_converge_append_only.py --baseline docs/governance/converge-append-only-baseline.json
converge-append-only: PASS (files=0 baselined=0)
exit=0
```

### G-UNIQ — unicidade de IDs

```
$ python3 scripts/ci/check_task_id_uniqueness.py --baseline docs/governance/task-id-uniqueness-baseline.json
task-id-uniqueness: PASS (tasks_files=3 duplicates=0 baselined=0)
exit=0
```

R4 aprovado somente após a colagem das três saídas acima com data/hora. Nenhuma afirmação remota
foi usada para aprovar este artefato; CI, merge, publicação, aprovação e proteção permanecem `[NV]`.
