# Quickstart: Bootstrap e verificação dos baselines locais (feature 003)

**Feature**: `003-baseline-governanca` | **Created**: 2026-11-19 | **Fase**: F1 (Phase 1 — Validation guide)

**Tipo**: procedimento executável e determinístico, somente local. Este guia contém comandos dos três gates já existentes e as saídas esperadas extraídas do código e do estado documentado. Não contém código novo, compilação, build, pipeline, migração, serviço externo, alteração de scripts nem efeito remoto. Nenhuma saída é afirmada como executada neste documento: a colagem da saída executada com data/hora pertence à portaria de evidência (R4).

## Pré-requisitos

- Checkout local do repositório do alvo com os três scripts em `scripts/ci/` e o diretório `docs/governance/` disponíveis.
- Python 3 com biblioteca padrão; nenhum `pip install`, engine, banco, credencial ou acesso de rede.
- Leitura da spec (`FR-001`–`FR-012`, `SC-001`–`SC-008`, `## Clarifications`), do `plan.md`, do `data-model.md` e de `contracts/baseline-bootstrap-contract.md`.
- Os três arquivos de baseline **ainda não gravados** antes das etapas 2 e 4 (a derivação depende de `baseline = {}`).

## Comandos de setup

**Não aplicável como instalação**: não há ambiente, runtime, biblioteca, engine, banco ou migração a instalar. A única preparação é conferir que os três scripts existem e que os baselines ainda não estão materializados:

```bash
ls scripts/ci/check_sdd_completeness.py scripts/ci/check_converge_append_only.py scripts/ci/check_task_id_uniqueness.py
ls docs/governance/
```

A listagem de `docs/governance/` é leitura, não etapa de configuração de execução. Nenhum comando de instalação é fornecido para evitar sugerir dependência inexistente.

## Procedimento de bootstrap

### Etapa 1 — `BL-CONV`: gerar e gravar o baseline de convergência

```bash
python3 scripts/ci/check_converge_append_only.py --bootstrap
```

Saída esperada (derivada de `check_converge_append_only.py:59-62`; estado sem `converge.md`):

```json
{
  "enforce_unbaselined": true,
  "files": {},
  "schema_version": "087.1.0"
}
```

Gravar essa saída em `docs/governance/converge-append-only-baseline.json` e acrescentar o campo `_provenance` com `origem=alvo-local`, `metodo=scripts/ci/check_converge_append_only.py --bootstrap`, `data=YYYY-MM-DD` e `factory=nao-utilizada`. Se existirem `converge.md`, o gerador os lista em `files` com `sha256` e `bytes`; nenhum é reescrito. Referência: contrato §2.1, §3.1; spec FR-001, FR-004.

### Etapa 2 — `BL-COMP`: capturar o inventário antes de gravar

```bash
python3 scripts/ci/check_sdd_completeness.py --json
```

Executar com `docs/governance/sdd-completeness-baseline.json` ainda ausente. Saída esperada (derivada de `check_sdd_completeness.py:112-143`):

```json
{
  "baselined_features": 0,
  "features_with_gaps": 1,
  "ok": false,
  "regressions": {
    "003-baseline-governanca": ["release", "validate"]
  },
  "schema": "047.2.0",
  "spec_dirs": 3,
  "stale_baseline": {},
  "strict_from": 88
}
```

O campo `regressions` é o inventário completo porque `baseline = {}` quando o arquivo não existe (`check_sdd_completeness.py:109-110`). Referência: contrato §3.1, §3.3; spec FR-002.

### Etapa 3 — `BL-COMP`: gravar o baseline

Gravar em `docs/governance/sdd-completeness-baseline.json`:

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

Regra: `allow_missing := regressions` da etapa 2, sem acrescentar ou remover chaves. `release` e `validate` são portarias futuras (R7 e R4) e, portanto, dívida legítima conforme a spec `## Clarifications` Q3. Referência: contrato §2.2, §3.2; spec FR-002, FR-010.

### Etapa 4 — `BL-UNIQ`: capturar o inventário antes de gravar

```bash
python3 scripts/ci/check_task_id_uniqueness.py --json
```

Executar com `docs/governance/task-id-uniqueness-baseline.json` ainda ausente. Saída esperada (derivada de `check_task_id_uniqueness.py:86-96`):

```json
{
  "baselined_specs": 0,
  "ok": true,
  "regressions": {},
  "schema": "087.1.0",
  "specs_with_duplicates": 0,
  "stale_baseline": {},
  "tasks_files": 3
}
```

`regressions={}` porque o estado corrente não tem `T###` duplicado. O campo `tasks_files` acompanha a quantidade de `tasks.md` presentes no ato da execução. Referência: contrato §2.3, §3.3; spec FR-003.

### Etapa 5 — `BL-UNIQ`: gravar o baseline

Gravar em `docs/governance/task-id-uniqueness-baseline.json`:

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

Mapa vazio é honesto: nenhuma entrada é inventada para inflar `baselined_specs`, que permanece 0; a prova de que não é um "verde vazio" é o arquivo presente em `docs/governance/`, a saída executada colada em R4 com data/hora e a recusa observada de `RG-2` (convergência) e `RG-4` (unicidade) — `RG-1`/`RG-5` são CONDICIONAIS e não se aplicam (spec FR-009, SC-004; contrato §4.3, §8 item 4). Referência: contrato §2.3; spec FR-003, FR-009.

### Etapa 6 — Confirmar os três gates com os baselines gravados

```bash
python3 scripts/ci/check_converge_append_only.py --baseline docs/governance/converge-append-only-baseline.json
python3 scripts/ci/check_sdd_completeness.py --json --baseline docs/governance/sdd-completeness-baseline.json
python3 scripts/ci/check_task_id_uniqueness.py --json --baseline docs/governance/task-id-uniqueness-baseline.json
```

Resultado esperado: `ok=true` nos três; `baselined_files=0`, `baselined_features=1`, `baselined_specs=0`; `regressions`, `rewritten_or_truncated`, `missing`, `unbaselined` e `stale_baseline` vazios; exit 0. O modo texto dos gates imprime linhas `...: PASS (...)` com os mesmos contadores. Referência: contrato §4.1; spec SC-003, SC-004.

## Cenários de verificação do ratchet

Cada cenário é isolado, observado e revertido; nenhum altera os scripts nem é afirmado como verde. Referência: contrato §4.3; spec SC-008.

| ID | Ação de verificação | Resultado esperado | Condição no estado documentado |
|---|---|---|---|
| `RG-1` | Reescrever ou truncar um `converge.md` baselinado e rodar `G-CONV` | FAIL, `rewritten_or_truncated` com `reason` `rewritten`/`truncated`, exit 1 | **CONDICIONAL** — exige `converge.md` em `files`; com `files={}`, não há alvo baselinado |
| `RG-2` | Criar `converge.md` novo sem re-baselinar e rodar `G-CONV` | FAIL, `unbaselined` com o caminho, exit 1 | **EXECUTÁVEL AGORA** — `enforce_unbaselined=true` recusa mesmo com `files={}` |
| `RG-3` | Criar artefato `release` para a feature 003 sem remover a entrada e rodar `G-COMP` | FAIL, `stale_baseline` apontando o artefato satisfeito, exit 1 | **EXECUTÁVEL AGORA** — `allow_missing` já contém `release`/`validate` |
| `RG-4` | Introduzir `T###` duplicado em uma `tasks.md` e rodar `G-UNIQ` | FAIL, `regressions` com `duplicate_task:T###`, exit 1 | **EXECUTÁVEL AGORA** — detecta duplicata mesmo com `allow_duplicates={}` |
| `RG-5` | Corrigir duplicata baselinada sem re-baselinar e rodar `G-UNIQ` | FAIL, `stale_baseline` apontando a entrada, exit 1 | **CONDICIONAL** — exige `allow_duplicates` não vazio; com `{}`, não há entrada a corrigir |

**Ressalva do estado documentado**: com `files={}` em `BL-CONV` e `allow_duplicates={}` em `BL-UNIQ`, `RG-1` e `RG-5` são **CONDICIONAIS** — dependem de entrada baselinada e só se aplicam após re-baselinação legítima. A prova executável imediata de que o ratchet está ativo é dada por `RG-2` e `RG-4`; `RG-3` também é executável porque `allow_missing` já contém `release`/`validate`. A evidência colada em R4 registra apenas os cenários executáveis; nenhum CONDICIONAL é afirmado como executado no estado atual (contrato §4.3, §8 item 6; spec SC-008, FR-009).

## Critérios de encerramento

- As etapas 1–6 produzem três JSON válidos nos caminhos canônicos, com `_provenance` e zero chaves da factory.
- Os três gates retornam PASS com zero regressões e zero `stale_baseline` quando executados com os baselines gravados.
- `baselined>0` em todo gate cujo estado corrente tem entradas; nos demais o mapa permanece vazio com `baselined=0`, e a prova é arquivo presente + saída colada em R4 + recusa de `RG-2`/`RG-4` (`RG-1`/`RG-5` são CONDICIONAIS; contrato §4.3, §8 item 4).
- Cada cenário executável no estado atual (`RG-2`, `RG-3`, `RG-4`) é recusado pelo gate correspondente antes da portaria de evidência; `RG-1` e `RG-5` são CONDICIONAIS e assim registrados no contrato §4.3.
- Nenhum script é alterado; cada um dos três arquivos é conferido por `sha256sum` contra o registro de imutabilidade do contrato §8.1 (SC-006; `T025`/`T027`), registrando zero divergências.
- A saída executada, com data/hora, é colada na portaria de evidência (R4); CI, merge, publicação, aprovação e proteção de branch permanecem `[NV]`.
- Se qualquer contagem divergir, a execução para e a divergência é registrada com o ID FR/SC correspondente ou `BUG-###`; nenhum resultado é maquiado.

## Comandos de teste, build e gate — escopo e limite

**Escopo**: os comandos de gate acima são o mecanismo de bootstrap e de verificação desta feature (spec FR-001–FR-003, SC-003, SC-008). **Limite**: nenhuma saída é afirmada como executada neste guia, e a portaria de evidência (R4) é quem cola a saída com data/hora. Não há build, teste automatizado, pipeline, compilação ou serviço externo neste ciclo. Nenhum verde remoto é afirmado; todo efeito remoto permanece `[NV]` até confirmação ao vivo.

## Campos não aplicáveis ao procedimento (com motivo)

| Campo | Declaração | Motivo |
|---|---|---|
| Tempo de execução, timeout, taxa de sucesso de comandos | **Não aplicável** | Os gates são determinísticos e locais; nenhum SLA ou benchmark externo foi medido. |
| Ambiente de teste, fixtures, dados de entrada sintéticos | **Não aplicável** | A entrada é o próprio repositório; os cenários de regressão são alterações isoladas e revertidas, não fixtures persistentes. |
| Credenciais, tokens, acesso remoto | **Não aplicável** | Nenhuma integração remota; norma de trabalho proíbe segredos e status remoto é `[NV]`. |
| Saída de console colada | **Não aplicável nesta fase** | Somente a portaria R4 produz evidência com saída e data/hora; este guia descreve saídas esperadas, não obtidas. |
| Versão de engine ou linguagem do projeto | **Não aplicável** | Python 3 é a linguagem dos gates existentes, não uma dependência introduzida pela feature. |
| `--bootstrap` nos gates de completude e unicidade | **Não aplicável** | Esses scripts não expõem a opção; a derivação é feita por `regressions` do `--json` sem baseline (contrato §3). |
