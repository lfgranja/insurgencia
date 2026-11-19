# Phase 0 Research: Baselines locais de governança

**Feature**: `003-baseline-governanca` | **Created**: 2026-11-19 | **Fase**: F0 (Phase 0 — Outline & Research)

**Input**: `spec.md` (origem autoritativa), `.specify/assessments/baseline-governanca/{intake,research,problem,concept,decision}.md`, `.specify/memory/constitution.md`, `docs/governance/sdd-phase-contracts.md`, `scripts/ci/check_sdd_completeness.py`, `scripts/ci/check_converge_append_only.py`, `scripts/ci/check_task_id_uniqueness.py`, `.specify/memory/STATE.md` e `docs/PROGRESSO.md`.

**Política de fontes**: somente fontes locais do alvo, com caminho e linha quando aplicável. Nenhuma URL foi aberta, nenhuma fonte externa ou baseline da factory foi consultado e nenhuma referência externa foi inventada. Toda afirmação sobre estado remoto, CI, merge, aprovação registrada ou proteção de branch permanece `[NV]`. Os formatos abaixo foram extraídos por leitura direta do código dos três gates, não de documentação de terceiros.

## Metodologia

1. Extrair da spec e do Technical Context as decisões de desenho necessárias — nenhum marcador de esclarecimento pendente (spec `## Clarifications`: três respostas em 2026-11-19, uma por vez).
2. Triangular cada decisão com pelo menos duas fontes locais independentes quando disponíveis (código do gate + assessment; código do gate + STATE/PROGRESSO; contrato de fase + constituição).
3. Extrair literalmente dos scripts os schemas, caminhos, regex, limiares e regras de `regressions`/`stale`, registrando a linha de origem.
4. Registrar alternativas consideradas e por que foram rejeitadas, sem executar gates, sem antecipar materialização, tarefas ou efeito remoto.

Formato adotado em cada decisão: **Decisão**, **Rationale**, **Alternativas consideradas**, **Fontes locais**.

## Decisões de pesquisa

### RQ-1 — Formato e local do baseline de convergência

- **Decisão**: gravar `docs/governance/converge-append-only-baseline.json` com `{"schema_version":"087.1.0","enforce_unbaselined":true,"files":{"<caminho relativo>":{"sha256":"<hex>","bytes":N}}}`. `compute_actual` varre `specs/[0-9][0-9][0-9]-*/converge.md`, lê bytes e registra `sha256` (hexdigest) e `bytes` (comprimento). O gerador `--bootstrap` imprime exatamente esse objeto com `json.dumps(..., indent=2, sort_keys=True)` e retorna 0.
- **Rationale**: é o formato que o próprio `load_baseline` consome; `enforce_unbaselined` default `true` recusa arquivo novo não baselinado. PASS exige mesmo SHA256 no prefixo de `bytes` e tamanho ≥ baseline; apêndices são permitidos.
- **Alternativas consideradas**: (a) copiar baseline da factory — rejeitado porque entradas órfãs viram `missing`/`stale` e contrariam a regra "factory é fonte, não autoridade"; (b) escrever um formato simplificado sem `bytes` — rejeitado porque `load_baseline` exige `sha256` e `bytes`; (c) desligar `enforce_unbaselined` — rejeitado porque deixaria `converge.md` novo passar sem baseline (FR-001, D8).
- **Fontes locais**: `scripts/ci/check_converge_append_only.py:17-18,21-32,35-46,49-62,72-108,109-125`; spec FR-001, FR-006; `.specify/assessments/baseline-governanca/research.md:32`.

### RQ-2 — Formato e semântica do baseline de completude SDD

- **Decisão**: gravar `docs/governance/sdd-completeness-baseline.json` com `{"allow_missing":{"<NNN-slug>":["<artefato>",…]}}`. Artefatos possíveis: `spec.md`, `plan.md`, `tasks.md`, `checklists`, `assessment`, `validate`, `release`; para specs com número ≥ 88, também `analyze.md` e `converge.md` (`strict_from=88`). O resumo do gate informa `schema:"047.2.0"`, `baselined_features=len(baseline)`, `features_with_gaps=len(actual)`, `spec_dirs` e `strict_from`.
- **Rationale**: `load_baseline` lê somente `allow_missing`; as chaves são o nome do diretório `NNN-slug` (não o slug isolado), e os valores são listas de artefatos. `stale` dispara quando o artefato passou a existir ou a feature saiu de `specs/`.
- **Alternativas consideradas**: (a) baselinar por slug em vez de `NNN-slug` — rejeitado porque as chaves de `compute_actual` e de `spec_names` usam `s.name`; (b) alterar `strict_from` para incluir a feature 003 — rejeitado por FR-011 (limiares imutáveis); (c) baselinar todos os artefatos de todas as specs — rejeitado por criar `stale` imediato e distorcer a contagem.
- **Fontes locais**: `scripts/ci/check_sdd_completeness.py:24-31,33-47,50-77,80-90,112-141,145-156`; spec FR-002, FR-007; `.specify/assessments/baseline-governanca/research.md:31`.

### RQ-3 — Formato e semântica do baseline de unicidade de IDs

- **Decisão**: gravar `docs/governance/task-id-uniqueness-baseline.json` com `{"allow_duplicates":{"<NNN-slug>":["duplicate_task:T###",…]}}`; `allow_missing` é aceito como legado pelo carregador. O resumo do gate informa `schema:"087.1.0"`, `tasks_files`, `specs_with_duplicates=len(actual)` e `baselined_specs=len(baseline)`. A regex é `^\s*-\s*\[[ xX]\]\s+(T\d{3,})\b`, aplicada linha a linha de cada `tasks.md`.
- **Rationale**: `load_baseline` normaliza valores para listas ordenadas de strings; as entradas de `compute_actual` usam o prefixo `duplicate_task:` seguido do ID. `stale` dispara quando a duplicata deixou de existir ou a spec foi removida.
- **Alternativas consideradas**: (a) registrar IDs sem o prefixo `duplicate_task:` — rejeitado porque a comparação em `main` é por string exata; (b) usar `allow_missing` como chave canônica — rejeitado porque `allow_duplicates` tem precedência no carregador e é o nome semanticamente correto; (c) gerar entradas para specs sem duplicata — rejeitado porque criaria `stale` imediato.
- **Fontes locais**: `scripts/ci/check_task_id_uniqueness.py:15-17,20-39,43-53,64-82,86-111`; spec FR-003, FR-008; `.specify/assessments/baseline-governanca/research.md:33`.

### RQ-4 — Estratégia de bootstrap para os gates sem gerador nativo

- **Decisão**: para completude e unicidade, executar o gate com `--json` **antes** de o arquivo de baseline existir. Nesse estado, `load_baseline` não é chamado e `baseline = {}`; consequentemente `regressions` contém exatamente o inventário calculado por `compute_actual`. A transformação é: `allow_missing := regressions` (completude) e `allow_duplicates := regressions` (unicidade). Depois de gravar o JSON, executar novamente para confirmar `regressions={}` e `stale_baseline={}`.
- **Rationale**: é uma derivação determinística, local e auditável, sem alterar os scripts. Usar um caminho de baseline inexistente também funcionaria, mas exigiria um arquivo artificial; a ordem "antes/depois" é mais simples e não inventa caminhos.
- **Alternativas consideradas**: (a) adicionar `--bootstrap` aos dois scripts — rejeitado por FR-011 e SC-006 (zero alterações); (b) transcrever manualmente as contagens de STATE/PROGRESSO — rejeitado porque são divergentes e não trazem as listas de entradas; (c) copiar a estrutura da factory — rejeitado por risco de `stale` e soberania do alvo.
- **Fontes locais**: `check_sdd_completeness.py:109-119,132-141`; `check_task_id_uniqueness.py:64-73,86-94`; spec FR-002, FR-003, SC-002; D1, D4.

### RQ-5 — Autoridade do estado corrente sobre contagens históricas

- **Decisão**: o estado produzido pelos scripts no ato da bootstrap é a única autoridade. Registros com `spec_dirs=0`, `1` e `2` são contexto datado. No estado esperado da materialização: três diretórios numerados, `003-baseline-governanca` com `release` e `validate` ausentes, nenhum `converge.md` e nenhum `T###` duplicado.
- **Rationale**: constituição I (determinismo) e VI (verificação honesta); decisão R1 exige derivação do alvo. Contagens antigas não identificam quais entradas baselinar.
- **Alternativas consideradas**: (a) congelar `spec_dirs=2` como autoridade — rejeitado porque a feature 003 já existe; (b) aceitar a média das contagens — rejeitado por não ser determinístico; (c) baselinar sem executar nada — rejeitado por reproduzir o "verde vazio".
- **Fontes locais**: spec `## Clarifications` Q1; `.specify/assessments/baseline-governanca/research.md:34,47`; `docs/PROGRESSO.md:33-36,87,106`; `.specify/memory/STATE.md:40-43`; constituição I e VI.

### RQ-6 — Proveniência sem alterar os scripts

- **Decisão**: acrescentar campo `_provenance` aos três JSON com origem `alvo-local`, método de geração, data ISO 8601 e `factory: nao-utilizada`. Os carregadores leem apenas chaves nomeadas (`allow_missing`, `files`/`schema_version`/`enforce_unbaselined`, `allow_duplicates`/`allow_missing`), portanto ignoram o campo extra.
- **Rationale**: atende FR-004 e transforma "não copiamos da factory" em afirmação inspecionável no próprio artefato, sem tocar na lógica dos gates (FR-011).
- **Alternativas consideradas**: (a) gravar proveniência só em `docs/PROGRESSO.md` — rejeitado porque o baseline viajaria sem sua origem; (b) exigir novo campo nos schemas dos scripts — rejeitado por SC-006; (c) usar comentário no JSON — rejeitado porque JSON não admite comentários e o parsing falharia.
- **Fontes locais**: `check_converge_append_only.py:35-46`; `check_sdd_completeness.py:80-90`; `check_task_id_uniqueness.py:43-53`; spec FR-004, FR-011; D3.

### RQ-7 — Honestidade de mapas vazios

- **Decisão**: quando `compute_actual` não produz entradas, o baseline é gravado com mapa vazio (`{}`) e `baselined=0`, sem inventar itens. A dívida 1 fecha pela presença do arquivo local, pelo schema conhecido, pela proveniência e pela saída colada.
- **Rationale**: `Evidence Against` do assessment mostra que inflar a contagem registrando gaps reais como aceitos reduz a sensibilidade do gate. Honestidade prevalece sobre aparência de cobertura.
- **Alternativas consideradas**: (a) criar uma entrada fictícia por gate para mostrar `baselined>0` — rejeitado como rabbit hole explícito no shape; (b) adiar os gates limpos — rejeitado porque deixa a dívida aberta; (c) transformar `baselined=0` em FAIL alterando o script — rejeitado por FR-011.
- **Fontes locais**: spec `## Clarifications` Q2, FR-009, SC-004; `.specify/assessments/baseline-governanca/concept.md:13-14,32`; `.specify/assessments/baseline-governanca/research.md:40-41`; D6.

### RQ-8 — Critério gap legítimo versus defeito

- **Decisão**: gap/duplicata é legítimo e baselinável somente quando decorre de portaria ainda não executada no ciclo (`validate`/R4, `release`/R7) ou de dívida local datada e rastreável. É defeito a corrigir antes de baselinar a ausência de artefato exigido por fase já concluída (`spec.md`, `plan.md`, `tasks.md`, `checklists`, `assessment`), qualquer chave órfã e qualquer chave da factory. Aplicando ao estado atual: `release` e `validate` de `003-baseline-governanca` são legítimos.
- **Rationale**: resolve a ambiguidade sem transformar dívida tolerada em anuência nem baselinar erro de materialização; a classificação é verificável por presença/ausência de arquivo e por registro datado.
- **Alternativas consideradas**: (a) baselinar todo gap corrente — rejeitado por reduzir sensibilidade; (b) corrigir todo gap antes de baselinar — rejeitado porque R4/R7 são portarias posteriores e não podem ser antecipadas; (c) decidir caso a caso sem critério — rejeitado por não ser auditável.
- **Fontes locais**: spec `## Clarifications` Q3, FR-010, Edge Cases; `.specify/assessments/baseline-governanca/research.md:41,49`; contrato de fase itens 26-27; D5.

### RQ-9 — Imutabilidade dos scripts e ratchet ativo

- **Decisão**: nenhuma lógica, limiar, schema, caminho padrão ou regex é alterado. O ratchet é ativado pelo próprio uso do baseline: entradas satisfeitas ou features removidas passam a `stale_baseline`, e regressões aparecem em `regressions`/`rewritten_or_truncated`/`missing`/`unbaselined`.
- **Rationale**: o aparato já implementa a detecção; a feature apenas fornece o estado aceito. Alterar os scripts destruiria a comparabilidade histórica e violaria SC-006.
- **Alternativas consideradas**: (a) reescrever os gates para aceitar proveniência — rejeitado por FR-011; (b) desabilitar `stale` para reduzir ruído — rejeitado porque anula o ratchet; (c) substituir os gates por outro mecanismo — rejeitado por determinismo e soberania do aparato local.
- **Fontes locais**: spec FR-011, SC-006, FR-005–FR-008; `check_sdd_completeness.py:120-131`; `check_task_id_uniqueness.py:74-84`; `check_converge_append_only.py:100-108`; D7, D9.

### RQ-10 — Evidência e honestidade de status

- **Decisão**: os comandos e saídas esperadas são descritos no quickstart; nenhuma saída é afirmada como executada nesta fase. A colagem da saída executada com data/hora pertence à portaria de evidência (R4). CI, merge, publicação, aprovação e proteção de branch permanecem `[NV]`.
- **Rationale**: constituição VI e contrato de fase exigem artefato com comando executado e saída colada, não intenção. Rotular `[NV]` preserva a honestidade sem bloquear a materialização local.
- **Alternativas consideradas**: (a) afirmar PASS sem execução — rejeitado por violar constituição VI; (b) exigir CI ou merge antes de materializar — rejeitado porque nada disso é pré-requisito e está `[NV]`; (c) omitir o procedimento do quickstart — rejeitado porque o contrato de fase exige guia validável.
- **Fontes locais**: spec FR-012, SC-007, Assumptions; `docs/governance/sdd-phase-contracts.md:24-27`; constituição VI; D10.

## Evidência contrária e tratamento local

| Risco apontado pela evidência local | Tratamento neste desenho |
|---|---|
| Estado limpo gera mapas vazios e `baselined=0`, reproduzindo o sintoma que se quer eliminar. | Mapa vazio é honesto; o valor está em arquivo local, schema, proveniência e ratchet ativo, com saída colada (RQ-7). |
| Registrar gaps atuais como aceitos transforma dívida tolerada em dívida aceita. | Critério de RQ-8 limita o aceite a portarias futuras e dívida datada; defeitos são corrigidos antes. |
| Copiar da factory cria entradas órfãs punidas como `stale`/`missing`. | Derivação exclusiva do estado corrente e campo `_provenance` com `factory: nao-utilizada` (RQ-5, RQ-6). |
| Contagens históricas divergem (`spec_dirs=0/1/2`). | Scripts no ato da bootstrap são a autoridade; divergência registrada em `## Clarifications` (RQ-5). |
| Incluir comandos no quickstart poderia antecipar portaria de evidência. | Comandos são descritos como procedimento; nenhuma saída é afirmada como obtida e R4 permanece posterior (RQ-10). |
| Campos extras no JSON poderiam quebrar o parsing dos gates. | `load_baseline` lê chaves nomeadas e ignora o resto; nenhum script é alterado (RQ-6, RQ-9). |

## Fontes consultadas (somente locais)

| Fonte local | Uso |
|---|---|
| `specs/003-baseline-governanca/spec.md` | Requisitos, clarificações, critérios de sucesso e premissas. |
| `.specify/assessments/baseline-governanca/{intake,research,problem,concept,decision}.md` | Evidência de origem, opções, veredito R1 e desvios históricos. |
| `scripts/ci/check_converge_append_only.py:17-18,21-32,35-46,49-62,72-125` | Formato `files`/`sha256`/`bytes`, `--bootstrap`, `enforce_unbaselined`, regras de PASS/FAIL e contadores. |
| `scripts/ci/check_sdd_completeness.py:24-31,33-47,50-77,80-90,93-156` | Formato `allow_missing`, inventário de artefatos, `strict_from=88`, schema `047.2.0` e regras de `stale`. |
| `scripts/ci/check_task_id_uniqueness.py:15-17,20-39,43-53,56-111` | Formato `allow_duplicates`, regex `T###`, schema `087.1.0` e contadores. |
| `.specify/memory/constitution.md` | Princípios I–VI, determinismo e verificação honesta. |
| `docs/governance/sdd-phase-contracts.md:11,17,24-27` | Ordem de portarias, fase plan e exigência de evidência em R4. |
| `.specify/memory/STATE.md:36-50` | Dívida 1, saídas com `baselined=0` e ressalva sobre a factory. |
| `docs/PROGRESSO.md:33-36,68,87,96,106` | Contagens divergentes, `files=0`, classificação dos "verdes vazios". |

## Campos não aplicáveis a esta pesquisa (com motivo)

| Campo | Declaração | Motivo |
|---|---|---|
| Benchmarks externos, mercado ou pesquisa de usuário | **Não aplicável** | Política `local-only`; números externos seriam evidência inventada. |
| URL, host ou política de allowlist acionada | **Não aplicável** | Nenhuma URL foi aberta; nenhuma fonte remota é necessária. |
| Resultado de gate, build, teste, lint ou compilação | **Não aplicável nesta fase** | Nenhum comando foi executado; saída colada com data/hora pertence à portaria de evidência. |
| Alteração de schema, limiar, regex ou caminho dos scripts | **Não aplicável (vetado)** | FR-011 e SC-006 exigem zero alterações; os formatos já são suportados. |
| Código, engine, banco, telemetria, CI ou migração | **Não aplicável** | A feature materializa apenas três JSON versionados. |
| Redação final dos três baselines | **Não aplicável nesta fase** | F0 registra decisões e formatos; a materialização ocorre na fase de implementação, conforme o contrato. |
