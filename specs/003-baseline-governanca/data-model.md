# Data Model: Baselines locais de governança

**Feature**: `003-baseline-governanca` | **Created**: 2026-11-19 | **Fase**: F1 (Phase 1 — Design)

**Origem**: spec `## Key Entities` (Baseline local, Entrada do estado corrente, Proveniência local, Regressão controlada, Evidência de gate), `## Requirements` (FR-001–FR-012), `## Success Criteria` (SC-001–SC-008) e decisões D1–D10 de `plan.md`. Os formatos são extraídos literalmente de `scripts/ci/check_*.py`; ver `research.md RQ-1–RQ-3`.

**Premissa**: as "entidades" abaixo são três arquivos JSON versionados em `docs/governance/` e registros documentais de apoio, não tabelas, objetos de runtime ou contratos de API de rede. Não há persistência de runtime, migração, chave estrangeira, concorrência ou ciclo de vida executável. Todo campo listado como obrigatório deve estar presente na materialização.

## Visão geral

| Entidade | Identidade | Finalidade | Origem |
|---|---|---|---|
| Baseline de convergência | `docs/governance/converge-append-only-baseline.json` | Fixar prefixo SHA256 e tamanho aceito de cada `converge.md`. | spec FR-001; `check_converge_append_only.py:18,25-32` |
| Baseline de completude | `docs/governance/sdd-completeness-baseline.json` | Registrar artefatos faltantes aceitos por feature. | spec FR-002; `check_sdd_completeness.py:24,80-90` |
| Baseline de unicidade | `docs/governance/task-id-uniqueness-baseline.json` | Registrar IDs `T###` duplicados aceitos por feature. | spec FR-003; `check_task_id_uniqueness.py:16,43-53` |
| Entrada do estado corrente | Chave do mapa de `compute_actual`/`regressions` | Única autoridade no ato da bootstrap; distingue baseline honesto de inventado. | spec Key Entities; research RQ-4, RQ-5 |
| Proveniência local | `_provenance` (presente nos três JSON) | Declarar origem no alvo e não uso da factory. | spec FR-004; research RQ-6 |
| Evidência de gate | Artefato R4 `.specify/validations/baseline-governanca/validate.md` (saída executada com data/hora) | Guardar saída executada com data/hora; estado remoto `[NV]`. | spec FR-012; SC-007; contrato §6 |

```text
scripts/ci/check_*.py --bootstrap / --json (sem baseline)
        │
        ▼
Entrada do estado corrente (compute_actual → regressions)
        │  transformação determinística (contrato §3)
        ▼
Baseline de convergência ── files{sha256,bytes} + _provenance
Baseline de completude   ── allow_missing{feature:[artefato]} + _provenance
Baseline de unicidade    ── allow_duplicates{feature:["duplicate_task:T###"]} + _provenance
        │
        ▼
Gates executados com baseline → PASS (0 regressões, 0 stale)
        │
        ▼
Evidência de gate (saída colada com data/hora na portaria R4) — remoto [NV]
```

## Entidade Baseline de convergência

**Arquivo canônico**: `docs/governance/converge-append-only-baseline.json` (valor padrão de `--baseline` em `check_converge_append_only.py:18`).

| Campo | Tipo | Obrigatório | Origem no script | Regra de validação |
|---|---|---|---|---|
| `schema_version` | Literal `087.1.0` | Sim | `check_converge_append_only.py:60` | Imutável; emitido pelo `--bootstrap`. A saída do gate em modo comparação também reporta `schema:"087.1.0"` (`:110`). |
| `enforce_unbaselined` | Booleano | Sim | `:45,60` | Deve ser `true`; recusa `converge.md` presente mas fora do mapa `files`. |
| `files` | Mapa `<caminho relativo>` → objeto | Sim | `:25-32,40-46` | Cada valor exige `sha256` (str) e `bytes` (int); chaves fora deste formato são descartadas pelo carregador. |
| `files.<rel>.sha256` | Hex SHA256 | Sim | `:21-22,31` | Hexdigest dos bytes do arquivo no ato da bootstrap; comparado ao SHA256 do prefixo de `bytes` do arquivo corrente. |
| `files.<rel>.bytes` | Inteiro ≥ 0 | Sim | `:29-31,78-89` | Define o prefixo comparado; o arquivo corrente deve ter tamanho ≥ este valor. |
| `_provenance` | Objeto | Sim (feature) | spec FR-004; research RQ-6 | Campo extra ignorado pelo carregador; ver §Entidade Proveniência local. |

**Forma canônica no estado documentado** (nenhum `converge.md` presente, conforme `docs/PROGRESSO.md:68,106`):

```json
{
  "_provenance": {
    "origem": "alvo-local",
    "metodo": "scripts/ci/check_converge_append_only.py --bootstrap",
    "data": "YYYY-MM-DD",
    "factory": "nao-utilizada"
  },
  "enforce_unbaselined": true,
  "files": {},
  "schema_version": "087.1.0"
}
```

**Regras de validação da entidade**:
1. `compute_actual` varre `specs/[0-9][0-9][0-9]-*/converge.md`; nenhum outro caminho entra em `files`.
2. PASS exige, para cada entrada: mesmo SHA256 no prefixo de `bytes`, tamanho atual ≥ `bytes`, nenhum arquivo ausente, nenhum não baselinado e nenhum `stale` (`:72-108`).
3. Apêndice legítimo preserva o prefixo e é aceito; reescrita ou truncamento produzem `rewritten_or_truncated` com `reason` `rewritten`/`truncated`.
4. Mapa vazio é válido quando não há `converge.md`; `baselined_files=0` nesse caso, sem entrada inventada (spec FR-009).

**Transições de estado**: `Ausente` → `Bootstrapado` (saída de `--bootstrap` gravada com `_provenance`) → `Verificado` (gate retorna PASS com zero achados); `Verificado` → `Rebaselinado` (novo `converge.md` legítimo entra em `files`); qualquer reescrita/truncamento leva a `Regressao`, nunca a uma reescrita silenciosa do prefixo.

## Entidade Baseline de completude

**Arquivo canônico**: `docs/governance/sdd-completeness-baseline.json` (`check_sdd_completeness.py:24`).

| Campo | Tipo | Obrigatório | Origem no script | Regra de validação |
|---|---|---|---|---|
| `allow_missing` | Mapa `<NNN-slug>` → lista de artefatos | Sim | `:80-90` | Chave é o nome do diretório em `specs/`; valores são strings exatas dos artefatos em `REQ_SPEC_FILES`, `REQ_DERIVED` e, quando aplicável, `STRICT_SPEC_FILES`. |
| `allow_missing.<feature>[]` | Enum de artefato | Condicional | `:28-30,59-74` | `spec.md`, `plan.md`, `tasks.md`, `checklists`, `assessment`, `validate`, `release`; `analyze.md`/`converge.md` só para número ≥ `strict_from` (88). |
| `_provenance` | Objeto | Sim (feature) | spec FR-004 | Ignorado pelo carregador. |

**Artefatos e origem da verificação** (`:28-30,50-77`):

| Artefato | Verificação em `compute_actual` |
|---|---|
| `spec.md`, `plan.md`, `tasks.md` | Presença como arquivo em `specs/<NNN-slug>/`. |
| `checklists` | Diretório `checklists/` com ao menos um `.md`. |
| `assessment`, `validate`, `release` | Diretório com o slug da feature em `.specify/assessments/`, `.specify/validations/` e `.specify/releases/`. |
| `analyze.md`, `converge.md` | Exigidos somente quando o número da spec ≥ `strict_from` (padrão 88). |

**Forma canônica no estado documentado** (feature 003 com `validate` e `release` ainda ausentes; ver spec `## Clarifications` Q1/Q3):

```json
{
  "_provenance": {
    "origem": "alvo-local",
    "metodo": "scripts/ci/check_sdd_completeness.py --json (sem baseline) → regressions",
    "data": "YYYY-MM-DD",
    "factory": "nao-utilizada"
  },
  "allow_missing": {
    "003-baseline-governanca": ["release", "validate"]
  }
}
```

**Regras de validação da entidade**:
1. A chave `allow_missing` é derivada de `regressions` com baseline ausente; cada entrada corresponde a um gap real do inventário (spec SC-002).
2. `baselined_features = len(baseline)` — no estado documentado, 1; no estado limpo, 0 sem entrada inventada.
3. `stale_baseline` aponta artefato que passou a existir ou feature removida; exige re-baselinação explícita (`:120-129`).
4. `strict_from` permanece 88; a feature 003 (número 3) não exige `analyze.md`/`converge.md` (spec FR-011).

**Transições de estado**: `Ausente` → `Bootstrapado` → `Verificado` (PASS com zero regressões e zero `stale`); `Verificado` → `Rebaselinado` (artefato passou a existir: entrada removida para evitar `stale`) ou → `Regressao` (artefato obrigatório novo fora do baseline).

## Entidade Baseline de unicidade

**Arquivo canônico**: `docs/governance/task-id-uniqueness-baseline.json` (`check_task_id_uniqueness.py:16`).

| Campo | Tipo | Obrigatório | Origem no script | Regra de validação |
|---|---|---|---|---|
| `allow_duplicates` | Mapa `<NNN-slug>` → lista de strings | Sim | `:48-53` | Precede `allow_missing` no carregador; valores ordenados pelo carregador. |
| `allow_missing` | Mapa equivalente | Não (legado) | `:48` | Aceito somente na ausência de `allow_duplicates`; não usado na materialização. |
| `allow_duplicates.<feature>[]` | `duplicate_task:T###` | Condicional | `:17,39` | Prefixo obrigatório; `T###` com pelo menos três dígitos segundo a regex `^\s*-\s*\[[ xX]\]\s+(T\d{3,})\b`. |
| `_provenance` | Objeto | Sim (feature) | spec FR-004 | Ignorado pelo carregador. |

**Forma canônica no estado documentado** (nenhuma duplicata detectada):

```json
{
  "_provenance": {
    "origem": "alvo-local",
    "metodo": "scripts/ci/check_task_id_uniqueness.py --json (sem baseline) → regressions",
    "data": "YYYY-MM-DD",
    "factory": "nao-utilizada"
  },
  "allow_duplicates": {}
}
```

**Regras de validação da entidade**:
1. `compute_actual` lê cada `tasks.md`, aplica a regex e conta IDs com `collections.Counter`; apenas duplicatas entram no mapa (`:20-39`).
2. `baselined_specs = len(baseline)` — 0 no estado documentado; mapa vazio é honesto (spec FR-008, FR-009).
3. `stale_baseline` aponta duplicata corrigida ou spec removida; a entrada deve ser retirada (`:74-82`).
4. Nenhuma entrada `duplicate_task:T###` é inventada para inflar a contagem (spec FR-003, SC-002).

**Transições de estado**: `Ausente` → `Bootstrapado` → `Verificado` (zero regressões e zero `stale`); `Verificado` → `Rebaselinado` (duplicata corrigida: entrada removida) ou → `Regressao` (novo ID duplicado fora do baseline).

## Entidade Entrada do estado corrente

| Campo | Tipo | Obrigatório | Regra de validação |
|---|---|---|---|
| `fonte` | Literal | Sim | Um dos três scripts, executado sobre o alvo; nenhuma fonte remota ou da factory. |
| `chave` | Texto | Sim | Nome do diretório `NNN-slug` (completude/unicidade) ou caminho relativo do `converge.md` (convergência). |
| `valor` | Lista ou objeto | Sim | Artefatos faltantes, `duplicate_task:T###`, ou par `sha256`/`bytes`. |
| `classificacao` | Enum | Sim | `Legitima` (portaria futura ou dívida datada) ou `Defeito` (artefato de fase concluída, chave órfã ou da factory). |
| `destino` | Enum | Sim | `Baselinar` se `Legitima`; `Corrigir` se `Defeito`. |

**Regra de validação**: nenhuma entrada é copiada de registro histórico; a classificação segue a spec `## Clarifications` Q3 e a research RQ-8. Chamado de "regressões" no resumo dos gates quando o baseline está ausente.

## Entidade Proveniência local

Presente como `_provenance` nos três JSON e ignorada pelos carregadores (research RQ-6).

| Campo | Tipo | Obrigatório | Regra de validação |
|---|---|---|---|
| `origem` | Literal `alvo-local` | Sim | Nenhuma origem externa, remota ou da factory. |
| `metodo` | Texto | Sim | Comando exato que gerou o baseline (`--bootstrap` para convergência; `--json` sem baseline → `regressions` para os outros dois). |
| `data` | Data ISO 8601 (`YYYY-MM-DD`) | Sim | Data da materialização, coerente com a evidência. |
| `factory` | Literal `nao-utilizada` | Sim | Afirmação inspecionável de que a factory não foi copiada. |

**Regra de validação**: a presença do campo não altera os schemas lidos; nenhum script é modificado para acomodá-lo (spec FR-004, FR-011, SC-006).

## Entidade Evidência de gate

**Artefato canônico**: `.specify/validations/baseline-governanca/validate.md` (R4). Esta entidade modela exclusivamente o **artefato R4**, e não o registro provisório mantido em `tasks.md`/neste `data-model.md` durante `tasks/implement`. No artefato R4, os campos `saida` e `data_hora` são obrigatórios.

| Campo | Tipo | Obrigatório | Regra de validação |
|---|---|---|---|
| `gate` | Enum | Sim | `sdd-completeness`, `converge-append-only`, `task-id-uniqueness`. |
| `comando` | Texto | Sim | Comando executado com o baseline correspondente. |
| `saida` | Texto | Sim | Saída executada colada, integralmente, sem edição. |
| `data_hora` | Data/hora ISO 8601 | Sim | Registrada no ato da execução. |
| `contadores` | Objeto | Sim | `baselined*`, contagens de achados e `ok`/exit code coerentes com a saída. |
| `estado_remoto` | Literal `[NV]` | Sim | CI, merge, publicação, aprovação e proteção não são afirmados como obtidos. |

**Regra de validação**: registros provisórios durante tasks/implement contêm apenas contagens e exit codes; `saida`/`data_hora` pertencem exclusivamente ao artefato R4.

**Transições de estado**: `Pendente` → `Provisória` (durante `tasks/implement`: apenas contagens e exit codes observados, sem `saida` colada nem `data_hora`, com efeito remoto `[NV]`) → `Arquivada` (materialização do artefato R4 em `.specify/validations/baseline-governanca/validate.md`, com `saida` integral colada sem edição e `data_hora` registrada no ato da execução). Nenhuma transição afirma efeito remoto.

## Campos não aplicáveis a entidades documentais (com motivo)

| Campo usual de modelo de dados | Declaração | Motivo |
|---|---|---|
| Chave primária autoincremental, FK, índice | **Não aplicável** | A identidade é o caminho do arquivo ou a chave literal do mapa JSON; não há banco. |
| Migração de esquema, DDL, seed | **Não aplicável** | Nenhuma estrutura persistente é alterada; os schemas são fixados pelos scripts e leem arquivos já existentes. |
| DTO, payload de API, contrato HTTP/gRPC | **Não aplicável** | Não há interface de rede; o contrato descreve formatos de arquivo e comandos locais. |
| Estado de runtime, cache, TTL, concorrência | **Não aplicável** | Os baselines são lidos por processos determinísticos de curta duração, sem thread, request ou sessão. |
| Métricas de latência, vazão, SLA | **Não aplicável** | Os critérios são contagens, exit code e presença/ausência de chaves (SC-001–SC-008). |
| Campos de segredo, token ou credencial | **Não aplicável** | Norma de trabalho proíbe segredos; nenhuma integração remota existe neste ciclo. |
| `schema_version` nos baselines de completude e unicidade | **Não aplicável como campo de arquivo** | Esses scripts não gravam nem leem tal campo no JSON; o schema `047.2.0`/`087.1.0` aparece apenas no resumo emitido pelo gate. Somente o baseline de convergência tem `schema_version` no arquivo. |
