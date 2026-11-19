# Contract: Bootstrap local dos baselines de governança

**Versão do contrato**: `1.0.0` | **Created**: 2026-11-19 | **Fase**: F1 (Phase 1 — Contracts)

**Escopo**: define a forma exata dos três baselines em `docs/governance/`, os comandos locais que os produzem, as regras de validação, o ratchet de `stale`, os cenários de regressão, a proveniência e o rollback documental. A origem autoritativa do conteúdo é `spec.md`; as decisões de desenho estão em `research.md` e as entidades em `data-model.md`. Nenhuma lógica, limiar, schema, caminho padrão ou regex dos três scripts é alterado. Nenhum gate é executado neste contrato: comandos e saídas são especificações do procedimento, e a saída executada com data/hora pertence à portaria de evidência (R4). Nenhum efeito remoto é afirmado; tudo permanece `[NV]`.

## 1. Identificadores e caminhos canônicos

| ID | Tipo | Significado | Localização |
|---|---|---|---|
| `BL-CONV` | Baseline | Prefixo SHA256 e tamanho aceito de `converge.md`. | `docs/governance/converge-append-only-baseline.json` |
| `BL-COMP` | Baseline | Artefatos faltantes aceitos por feature. | `docs/governance/sdd-completeness-baseline.json` |
| `BL-UNIQ` | Baseline | IDs `T###` duplicados aceitos por feature. | `docs/governance/task-id-uniqueness-baseline.json` |
| `G-CONV` | Gate | Comparador de `converge.md` append-only. | `scripts/ci/check_converge_append_only.py` |
| `G-COMP` | Gate | Verificador de completude SDD. | `scripts/ci/check_sdd_completeness.py` |
| `G-UNIQ` | Gate | Verificador de unicidade de `T###`. | `scripts/ci/check_task_id_uniqueness.py` |
| `DEC-R1` | Decisão | Veredito `go`, opção A, de 2026-11-19. | `.specify/assessments/baseline-governanca/decision.md:5,21-25` |
| `DIV-1` | Dívida | Ausência de baseline local registrada. | `.specify/memory/STATE.md:50` |

**Regra de caminho**: os três destinos são exatamente os valores padrão de `--baseline` nos scripts (`G-CONV:18`, `G-COMP:24`, `G-UNIQ:16`). Nenhum caminho alternativo é gravado; nenhum diretório novo é criado.

**Regra de origem**: a factory é fonte do aparato, não autoridade dos baselines. Nenhuma chave, valor, spec ou configuração da factory entra nos três JSON (spec FR-005, SC-005).

## 2. Formatos exatos dos baselines

### 2.1 `BL-CONV` — convergência

```json
{
  "_provenance": {
    "origem": "alvo-local",
    "metodo": "scripts/ci/check_converge_append_only.py --bootstrap",
    "data": "YYYY-MM-DD",
    "factory": "nao-utilizada"
  },
  "enforce_unbaselined": true,
  "files": {
    "specs/<NNN-slug>/converge.md": {
      "sha256": "<hexdigest de 64 caracteres>",
      "bytes": 1234
    }
  },
  "schema_version": "087.1.0"
}
```

| Chave | Origem no script | Obrigatória | Regra |
|---|---|---|---|
| `schema_version` | `G-CONV:60` | Sim | Literal `087.1.0`. |
| `enforce_unbaselined` | `G-CONV:45,60` | Sim | `true`; recusa arquivo presente fora de `files`. |
| `files` | `G-CONV:25-32,40-46` | Sim | Mapa de caminho relativo → `{sha256, bytes}`; vazio quando não há `converge.md`. |
| `_provenance` | spec FR-004 | Sim (feature) | Ignorado pelo carregador. |

No estado documentado, `files` é `{}` (`docs/PROGRESSO.md:68,106`); nenhuma entrada é inventada.

### 2.2 `BL-COMP` — completude

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

| Chave | Origem no script | Obrigatória | Regra |
|---|---|---|---|
| `allow_missing` | `G-COMP:80-90` | Sim | Mapa de `<NNN-slug>` → lista de artefatos; chave é o nome do diretório, não o slug isolado. |
| Artefatos | `G-COMP:28-30,59-74` | Condicional | `spec.md`, `plan.md`, `tasks.md`, `checklists`, `assessment`, `validate`, `release`; `analyze.md`/`converge.md` apenas para número ≥ 88. |
| `_provenance` | spec FR-004 | Sim (feature) | Ignorado pelo carregador. |

O resumo do gate reporta `schema:"047.2.0"` e `strict_from:88`; esses valores **não** são campos do arquivo (spec FR-011; data-model §Campos não aplicáveis).

### 2.3 `BL-UNIQ` — unicidade

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

| Chave | Origem no script | Obrigatória | Regra |
|---|---|---|---|
| `allow_duplicates` | `G-UNIQ:48-53` | Sim | Mapa de `<NNN-slug>` → lista de strings `duplicate_task:T###`; tem precedência sobre `allow_missing`. |
| `allow_missing` | `G-UNIQ:48` | Não (legado) | Aceito apenas na ausência de `allow_duplicates`; não usado na materialização. |
| `_provenance` | spec FR-004 | Sim (feature) | Ignorado pelo carregador. |

Regex canônica (`G-UNIQ:17`): `^\s*-\s*\[[ xX]\]\s+(T\d{3,})\b`. O resumo reporta `schema:"087.1.0"`, também **não** campo do arquivo.

## 3. Procedimento de bootstrap

### 3.1 Ordem obrigatória

1. **Convergência (`BL-CONV`)**: executar `python3 scripts/ci/check_converge_append_only.py --bootstrap`; a saída é o JSON completo (schema_version, enforce_unbaselined e files). Gravar em `docs/governance/converge-append-only-baseline.json` e acrescentar `_provenance`. O gerador não lê baseline, então a ordem em relação aos outros dois é livre.
2. **Completude (`BL-COMP`) — antes de gravar**: com `docs/governance/sdd-completeness-baseline.json` ainda ausente, executar `python3 scripts/ci/check_sdd_completeness.py --json`. Como o baseline não existe, `load_baseline` não é chamado e `regressions` é exatamente o inventário `compute_actual`.
3. **Completude — gravação**: gravar `{"allow_missing": <regressions do passo 2>, "_provenance": <...>}` no caminho canônico.
4. **Unicidade (`BL-UNIQ`) — antes de gravar**: com `docs/governance/task-id-uniqueness-baseline.json` ainda ausente, executar `python3 scripts/ci/check_task_id_uniqueness.py --json` e capturar `regressions`.
5. **Unicidade — gravação**: gravar `{"allow_duplicates": <regressions do passo 4>, "_provenance": <...>}` no caminho canônico.
6. **Confirmação**: executar novamente os três gates com `--baseline` apontando para os arquivos gravados; esperar `ok:true`, `regressions`/`rewritten_or_truncated`/`missing`/`unbaselined`/`stale_baseline` vazios e contagens coerentes.

**Regra da ordem**: os passos 2 e 4 devem ocorrer **antes** da gravação de seus respectivos baselines. Depois da gravação, `regressions` esvazia por construção e o inventário não pode mais ser lido diretamente do resumo.

### 3.2 Transformações determinísticas

| Gate | Inventário (sem baseline) | Transformação para o arquivo |
|---|---|---|
| `G-CONV` | `compute_actual(root)` → `{rel: {sha256, bytes}}` | Saída literal de `--bootstrap`; acrescentar `_provenance`. |
| `G-COMP` | `summary["regressions"]` → `{feature: [artefato, …]}` | `allow_missing := summary["regressions"]`; acrescentar `_provenance`. |
| `G-UNIQ` | `summary["regressions"]` → `{feature: ["duplicate_task:T###", …]}` | `allow_duplicates := summary["regressions"]`; acrescentar `_provenance`. |

Nenhuma transformação inventa, remove ou renomeia chaves. Nenhuma chave da factory é admitida (§1).

### 3.3 Saídas esperadas no estado documentado

| Comando | Campos relevantes da saída esperada |
|---|---|
| `check_converge_append_only.py --bootstrap` | `schema_version="087.1.0"`, `enforce_unbaselined=true`, `files={}` (nenhum `converge.md`). |
| `check_sdd_completeness.py --json` (sem baseline) | `schema="047.2.0"`, `ok=false`, `features_with_gaps=1`, `regressions={"003-baseline-governanca":["release","validate"]}`, `baselined_features=0`, `stale_baseline={}`, `strict_from=88`. |
| `check_task_id_uniqueness.py --json` (sem baseline) | `schema="087.1.0"`, `ok=true`, `specs_with_duplicates=0`, `regressions={}`, `baselined_specs=0`, `stale_baseline={}`. |
| Três gates com baseline gravado | `ok=true` nos três; `baselined_files=0`, `baselined_features=1`, `baselined_specs=0`; todas as listas de achados vazias; exit 0. |

Essas saídas são derivadas da leitura dos scripts e do estado documentado; nenhuma foi executada nesta fase e nenhuma é afirmada como obtida (spec FR-012; constituição VI).

## 4. Validação, ratchet e cenários de regressão

### 4.1 Regras de validação por gate

| Gate | PASS exige | FAIL quando |
|---|---|---|
| `G-CONV` | Mesmo SHA256 no prefixo de `bytes`; tamanho atual ≥ `bytes`; nenhum `missing`, `unbaselined`, `stale` | `rewritten`, `truncated`, arquivo ausente, arquivo novo não baselinado (com `enforce_unbaselined=true`) |
| `G-COMP` | Nenhuma falta fora de `allow_missing`; nenhum artefato baselinado que já existe; nenhuma feature removida no baseline | `regressions` não vazio ou `stale_baseline` não vazio |
| `G-UNIQ` | Nenhuma duplicata fora de `allow_duplicates`; nenhuma duplicata baselinada já corrigida; nenhuma spec removida no baseline | `regressions` não vazio ou `stale_baseline` não vazio |

### 4.2 Ratchet

Toda entrada do baseline é um estado **aceito**, não uma isenção permanente. Se a condição desaparecer (artefato criado, duplicata corrigida, feature removida), o gate sinaliza `stale_baseline` e a entrada deve ser removida por re-baselinação explícita. Isso impede que dívida corrigida continue contando como coberta (spec D9; FR-005–FR-008).

### 4.3 Cenários de regressão controlada

| ID | Alteração artificial | Gate que recusa | Assinatura esperada | Condição no estado documentado |
|---|---|---|---|---|
| `RG-1` | Reescrever ou truncar um `converge.md` baselinado | `G-CONV` | `rewritten_or_truncated` com `reason` `rewritten`/`truncated`; exit 1 | **CONDICIONAL** — exige ao menos um `converge.md` em `files`; no estado atual `files={}`, não há alvo baselinado |
| `RG-2` | Criar `converge.md` novo sem re-baselinar | `G-CONV` | `unbaselined` com o caminho; exit 1 | **EXECUTÁVEL AGORA** — `enforce_unbaselined=true` recusa mesmo com `files={}` |
| `RG-3` | Acrescentar artefato obrigatório sem atualizar `allow_missing` | `G-COMP` | `stale_baseline` apontando artefato satisfeito; exit 1 | **EXECUTÁVEL AGORA** — `allow_missing` contém `release`/`validate` |
| `RG-4` | Introduzir `T###` duplicado em uma `tasks.md` | `G-UNIQ` | `regressions` com `duplicate_task:T###`; exit 1 | **EXECUTÁVEL AGORA** — detecta duplicata mesmo com `allow_duplicates={}` |
| `RG-5` | Corrigir duplicata baselinada sem re-baselinar | `G-UNIQ` | `stale_baseline` apontando a entrada; exit 1 | **CONDICIONAL** — exige `allow_duplicates` não vazio; no estado atual `{}`, não há entrada a corrigir |

Cada cenário é isolado e revertido após a observação; nenhum deles altera os scripts nem é registrado como verde. Os cenários provam que o ratchet está ativo antes da portaria de evidência (spec SC-008).

**Ressalva do estado documentado**: com `BL-CONV.files={}` e `BL-UNIQ.allow_duplicates={}`, `RG-1` e `RG-5` são **CONDICIONAIS** — dependem de uma entrada baselinada que ainda não existe (`converge.md` baselinado para `RG-1`; duplicata baselinada para `RG-5`) e só se aplicam após re-baselinação legítima. A prova executável imediata de que o ratchet está ativo neste estado é dada por **`RG-2`** (`G-CONV` recusa `converge.md` novo via `enforce_unbaselined=true`, mesmo com `files` vazio) e **`RG-4`** (`G-UNIQ` recusa `T###` duplicado como `regressions`, mesmo com `allow_duplicates` vazio); `RG-3` também é executável porque `allow_missing` já contém `release`/`validate`. Nenhum cenário CONDICIONAL DEVE ser afirmado como executado no estado atual: a evidência colada em R4 registra `RG-2`, `RG-3` e `RG-4` como prova observável (spec SC-008; quickstart §Cenários de verificação do ratchet).

## 5. Versionamento, changelog e rollback documental

### 5.1 Versão do conjunto

- `1.0.0`: primeira materialização dos três baselines locais.
- **MAJOR** (`2.0.0`, …): mudança de schema lido, de caminho canônico, de regra de ratchet ou remoção de um dos três baselines.
- **MINOR** (`1.1.0`, …): acréscimo ou remoção de entrada legítima em qualquer baseline, sem mudar formato.
- **PATCH** (`1.0.1`, …): correção de `_provenance`, de data ou de pontuação, sem mudar entradas nem schemas.

Os schemas lidos (`087.1.0`/`047.2.0`) e o `schema_version` de `BL-CONV` são fixados pelos scripts e **não** seguem esta versão documental.

### 5.2 Changelog

Cada entrada registra versão, data `YYYY-MM-DD`, descrição, IDs FR afetados e, quando houver, IDs `T###` e `BUG-###`. Forma documental: `### [1.0.0] — 2026-11-19` seguida de itens com os IDs. Nenhuma versão é afirmada como publicada em tag remota, mesclada ou aprovada por CI; todo efeito remoto permanece `[NV]`.

### 5.3 Rollback documental

**Gatilhos**: chave da factory ou órfã detectada; regressão controlada não recusada; `stale` persistente após re-baselinação incorreta; alteração indevida de script; entrada que transforma dívida em anuência sem registro datado; afirmação de efeito remoto sem confirmação ao vivo.

**Procedimento**:

1. Registrar o achado com IDs FR e, quando existirem, `T###` e `BUG-###`.
2. Criar novo commit que restaure o JSON anterior do baseline afetado, com nova versão SemVer e data; nunca apagar ou reescrever o histórico.
3. Reconfirmar os outros dois baselines intactos e os três scripts inalterados (comparação textual).
4. Reexecutar o gate afetado e conferir zero achados na saída colada.
5. Registrar entrada no changelog com versão, data e IDs.
6. Marcar `[NV]` qualquer efeito remoto até confirmação ao vivo.

**Proibido**: apagar histórico, `force-push`, auto-merge, alterar scripts, copiar da factory, afirmar CI/merge/aprovação obtidos.

**Não aplicável**: rollback de runtime, banco, migração, binário, configuração de CI ou serviço externo — motivo: a feature produz somente três JSON versionados.

## 6. Evidência e limite de autoridade

| Campo | Regra |
|---|---|
| Comando | Um dos três comandos de gate com o respectivo `--baseline`. |
| Saída | Colada integralmente, sem edição, na portaria de evidência (R4). |
| Data/hora | Registrada no ato da execução, em formato ISO 8601. |
| Contadores | `baselined*`, contagens de achados e `ok`/exit code coerentes com a saída. |
| Estado remoto | CI, merge, publicação, aprovação e proteção de branch permanecem `[NV]` (constituição VI). |

Nenhum verde é afirmado neste contrato. A execução dos comandos e a colagem da saída com data/hora pertencem à portaria de evidência, posterior a esta fase de planejamento.

## 7. Campos e operações não aplicáveis ao contrato (com motivo)

| Campo/operação | Declaração | Motivo |
|---|---|---|
| Endpoint, esquema HTTP/gRPC, autenticação de API | **Não aplicável** | Nenhuma interface de rede existe; o contrato descreve arquivos e comandos locais. |
| Novo parser ou validador automático | **Não aplicável (vetado)** | Os três gates já validam; FR-011 e SC-006 proíbem alteração ou substituição. |
| Campos de segredo, token ou credencial | **Não aplicável** | Nenhuma integração remota; norma de trabalho proíbe segredos. |
| `--bootstrap` em `G-COMP` e `G-UNIQ` | **Não aplicável** | Esses scripts não expõem a opção; a derivação usa `regressions` do `--json` sem baseline (§3). |
| Resultado de release | **Não aplicável nesta fase** | R4 precede R7; ambas são portarias posteriores. |
| Modificação de `converge.md` | **Não aplicável (vetado)** | A feature registra o estado atual; `converge.md` é append-only e nenhum arquivo é reescrito. |

## 8. Critérios de conformidade

1. Os três arquivos existem em `docs/governance/` com as chaves exatas de §2 e `_provenance` presente (SC-001, SC-004).
2. Cada entrada do baseline corresponde a uma entrada real do estado corrente; zero entradas inventadas e zero chaves da factory (SC-002, SC-005).
3. Os três gates retornam PASS (exit 0) com zero regressões e zero `stale_baseline` (SC-003).
4. `baselined>0` em todo gate com entradas no estado corrente; nos demais, o mapa vazio permanece com `baselined=0` e é honesto — a prova é o arquivo presente, a saída colada em R4 e a recusa de `RG-2`/`RG-4` (SC-004, SC-008).
5. Os três scripts permanecem byte-idênticos: nenhuma lógica, limiar, schema, caminho ou regex alterado; a conferência é feita contra os SHA256 de §8.1 por `T027` (SC-006).
6. Cenários de ratchet executáveis neste estado (`RG-2`, `RG-3`, `RG-4`) são recusados pelos gates correspondentes antes da portaria de evidência; `RG-1` e `RG-5` são CONDICIONAIS e assim registrados em §4.3 (SC-008).
7. Saída executada com data/hora colada na portaria de evidência; nenhum efeito remoto afirmado (SC-007).
8. Versão, changelog e eventual rollback seguem §5, com IDs FR e, quando houver, `T###`/`BUG-###`.

### 8.1 Base de imutabilidade dos scripts (SHA256)

Registro canônico dos digests atuais dos três scripts, obtidos por `sha256sum` sobre o alvo em 2026-11-19. Toda conferência de imutabilidade — item 5 acima, `T027` — DEVE comparar contra este registro; nenhum outro valor é admitido como referência de "versão original".

| Script | SHA256 (64 caracteres hexadecimais) |
|---|---|
| `scripts/ci/check_sdd_completeness.py` | `146fd007c2adeacad7117df0e2577e25d110f101fbb9fcc8cd718fc30c9946a2` |
| `scripts/ci/check_converge_append_only.py` | `a51a5c6e096c7e7d52833bfd2310e815701f6c19c08bb54d741ff2580b5157fa` |
| `scripts/ci/check_task_id_uniqueness.py` | `b6c5e9cacb8af7b442e9f1c920b5435c8f14f2fec35f1115607fa8334e59a420` |

Qualquer divergência entre o digest observado e o registro é achado de imutabilidade, registrado com ID FR/SC ou `BUG-###` conforme §5.3; nenhuma re-baselinação de script é permitida (FR-011, SC-006). O registro é documental e não altera os scripts.
