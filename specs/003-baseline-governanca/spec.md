# Feature Specification: Baselines locais de governança

**Feature Branch**: `003-baseline-governanca`

**Created**: 2026-11-19

**Status**: Draft

**Input**: Decisão R1 de 2026-11-19 (veredito `go`, opção A — bootstrap local integral) registrada em `.specify/assessments/baseline-governanca/decision.md`; dívida 1 de `.specify/memory/STATE.md:50`. Materializar três baselines LOCAIS de governança a partir do estado corrente do alvo, nunca copiando da factory, para que os gates deixem de ser "verdes vazios" e passem a detectar regressão.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Bootstrap local do baseline de convergência (Priority: P1)

O mantenedor que executa os gates locais precisa que `converge.md` tenha uma linha de base derivada do próprio alvo, com hash e tamanho registrados, para que uma reescrita, um truncamento, um apagamento ou um arquivo novo deixem de passar despercebidos. Hoje nenhum arquivo de baseline existe em `docs/governance/` e o gate reporta `baselined=0`. A feature materializa o JSON no caminho que o próprio script lê, usando o gerador `--bootstrap` nativo, sem tocar na lógica do gate.

**Why this priority**: é o único dos três gates com gerador determinístico nativo e o de menor ambiguidade; entrega a prova de que a origem é o estado local e estabelece o padrão de proveniência para os outros dois.

**Independent Test**: basta executar o gerador `--bootstrap`, gravar a saída em `docs/governance/converge-append-only-baseline.json` e executar o gate com esse baseline para obter PASS com `baselined_files` igual ao número de `converge.md` presentes, zero reescritos, zero ausentes, zero não baselinados e zero `stale` — independentemente dos outros dois baselines.

**Acceptance Scenarios**:

1. **Given** que nenhum arquivo `docs/governance/converge-append-only-baseline.json` existe, **When** o mantenedor executa o gerador `--bootstrap` do gate de convergência sobre o alvo e grava a saída no caminho padrão, **Then** o arquivo contém `schema_version="087.1.0"`, `enforce_unbaselined=true` e um mapa `files` com `sha256` e `bytes` para cada `specs/<NNN>-*/converge.md` efetivamente presente.
2. **Given** o baseline materializado no cenário 1, **When** o gate de convergência é executado com ele, **Then** retorna PASS (exit 0) com `baselined_files` igual à contagem de `converge.md`, `rewritten_or_truncated`, `missing`, `unbaselined` e `stale_baseline` todos vazios.
3. **Given** uma regressão controlada — um `converge.md` reescrito ou truncado, ou um `converge.md` novo não baselinado — **When** o gate é executado com o mesmo baseline, **Then** retorna FAIL (exit 1) apontando o arquivo em `rewritten_or_truncated` ou `unbaselined`.

---

### User Story 2 — Bootstrap local dos baselines de completude e unicidade (Priority: P1)

O auditor de governança precisa que os gates de completude SDD e de unicidade de IDs tenham linhas de base derivadas do estado corrente do alvo, para que um artefato faltante fora do aceito ou um `T###` duplicado novo sejam recusados. Os dois scripts não expõem gerador `--bootstrap`; a derivação é feita a partir do próprio inventário que cada script calcula, obtido com `(--json)` sem baseline, e gravado nos formatos que os scripts já leem. Nenhuma chave de spec da factory é adotada.

**Why this priority**: fecha os dois terços restantes da dívida 1 e ativa as verificações de `stale` e de regressão nesses gates, que hoje emitem PASS sem lastro.

**Independent Test**: basta derivar as duas entradas do inventário local, gravar os dois JSON nos caminhos padrão e executar cada gate com seu baseline, obtendo PASS com zero regressões e zero `stale` em cada um — sem depender do baseline de convergência.

**Acceptance Scenarios**:

1. **Given** que `docs/governance/sdd-completeness-baseline.json` não existe e que o inventário local aponta a feature `003-baseline-governanca` com os artefatos de portarias futuras ainda ausentes, **When** o auditor deriva o baseline de `allow_missing` a partir das regressões calculadas sem baseline e grava o JSON, **Then** o gate de completude retorna PASS com `baselined_features>0`, `regressions` vazio e `stale_baseline` vazio.
2. **Given** que `docs/governance/task-id-uniqueness-baseline.json` não existe e que o inventário local não aponta duplicatas, **When** o auditor deriva o baseline de `allow_duplicates` a partir das regressões calculadas sem baseline e grava o JSON com mapa vazio, **Then** o gate de unicidade retorna PASS com `baselined_specs=0`, `regressions` vazio e `stale_baseline` vazio.
3. **Given** um `T###` duplicado artificial introduzido em uma `tasks.md`, **When** o gate de unicidade é executado com o baseline derivado, **Then** retorna FAIL (exit 1) com o ID em `regressions`; dado um artefato obrigatório acrescentado fora do baseline, o gate de completude retorna FAIL com `stale_baseline` apontando a entrada satisfeita.

---

### User Story 3 — Evidenciar a origem local e a integridade dos três gates (Priority: P2)

Quem mantém e audita o aparato SDD precisa distinguir, na própria saída, um gate que passa por não-regressão de um gate que passa por ausência de estado, e precisa fazer isso sem herdar chaves, formatos ou decisões da factory. A feature registra proveniência local explícita nos três baselines, preserva os scripts intactos e deixa a saída executada com data/hora pronta para a portaria de evidência, mantendo remoto, CI, merge e proteção como `[NV]`.

**Why this priority**: sem proveniência e sem evidência colada, o baseline materializado reproduziria o "verde vazio" que se quer eliminar e correria o risco de ser confundido com uma cópia da factory.

**Independent Test**: basta ler os três JSON e a saída dos três gates, contendo zero chaves da factory, campo de proveniência local nos três e saída PASS com contadores coerentes, sem executar build, teste ou efeito remoto.

**Acceptance Scenarios**:

1. **Given** os três baselines materializados, **When** um auditor lê os três arquivos, **Then** encontra exatamente três caminhos em `docs/governance/`, nenhuma chave de spec da factory, nenhuma entrada órfã e campo de proveniência local declarando origem no alvo, método, data e não uso da factory.
2. **Given** os três gates executados com seus baselines, **When** a evidência é preparada para a portaria de evidência, **Then** as três saídas executadas estão coladas com data/hora e nenhuma afirmação de CI, merge, publicação, aprovação ou proteção de branch é feita — todas rotuladas `[NV]`.
3. **Given** um cenário de regressão controlada nos três gates, **When** as saídas são comparadas, **Then** os três recusam a regressão antes da portaria de evidência, comprovando que o ratchet está ativo e não apenas declarado.

---

### Edge Cases

- Se o estado corrente estiver limpo para um gate (nenhum `converge.md`, nenhuma duplicata), o baseline DEVE ser materializado com mapa vazio e `baselined=0`, sem inventar entradas para inflar a contagem; a dívida fecha pela presença do arquivo local, pelo schema conhecido e pela proveniência, não por uma contagem artificial. A prova DEVE ser: (i) o arquivo presente no caminho canônico, (ii) a saída executada do gate colada em R4 com data/hora e (iii) a recusa observada de `RG-2` (novo `converge.md` com `files={}`) e `RG-4` (`T###` duplicado com `allow_duplicates={}`); `RG-1`/`RG-5` são CONDICIONAIS e não se aplicam a mapas vazios (contrato §4.3, §8 item 4).
- Se uma chave de spec removida do alvo ou uma chave de spec da factory aparecer em qualquer baseline, o gate a classifica como `stale`; nenhuma dessas chaves DEVE ser copiada, mesmo que a factory seja a fonte do aparato.
- Se contagens históricas divergirem (`spec_dirs=0`, `1` ou `2`), o estado produzido pelos próprios scripts no ato da bootstrap DEVE prevalecer; registros datados são contexto, não autoridade.
- Se um gap atual for artefato de portaria ainda não executada neste ciclo (por exemplo `validate` e `release` da própria feature 003), ele DEVE ser registrado como aceito; se for artefato que a fase já deveria ter produzido (`spec.md`, `plan.md`, `tasks.md`, `checklists`, `assessment`), DEVE ser corrigido antes de virar baseline.
- Se o gerador `--bootstrap` de converge incluir um campo de proveniência acrescentado depois da geração, os três gates DEVEM continuar lendo apenas as chaves de schema, ignorando campos extras; nenhum script DEVE ser alterado para acomodá-lo.
- Se um `converge.md` crescer por apêndice legítimo, o gate DEVE continuar PASS desde que o prefixo de `bytes` registrados mantenha o mesmo SHA256; um apêndice não é re-baselinação.
- Se um item do baseline deixar de ser necessário porque o artefato passou a existir ou a spec foi removida, o gate DEVE sinalizar `stale`, exigindo re-baselinação explícita em vez de silêncio.
- Se CI, merge, publicação, aprovação em plataforma ou proteção de branch forem citados, DEVEM permanecer `[NV]`; nenhuma aprovação remota é pré-requisito para materializar baselines locais.
- Se a saída dos gates não puder ser colada com data/hora, a feature NÃO DEVE ser afirmada como concluída: verde sem evidência é falha de honestidade.

## Requirements *(mandatory)*

> **Exceção documental justificada — comandos em FR-001–FR-003**: a regra geral que mantém a spec livre de prescrição de comandos é aqui excepcionada de forma explícita e delimitada. Nesta feature o comando **é** o mecanismo do próprio objeto: um baseline é definido como a saída de `python3 scripts/ci/check_*.py` executada sobre o alvo. Suprimir os comandos de FR-001–FR-003 tornaria indeterminado o objeto da feature; por isso eles constam como parte da definição do requisito, não como procedimento de implementação nem como promessa de execução. A saída executada com data/hora pertence à portaria de evidência (R4) e nenhum verde é afirmado nesta spec. Ver também `## Contexto e Orçamento`.

### Functional Requirements

- **FR-001**: O aparato DEVE materializar `docs/governance/converge-append-only-baseline.json` a partir da saída de `python3 scripts/ci/check_converge_append_only.py --bootstrap`, preservando `schema_version="087.1.0"`, `enforce_unbaselined=true` e o mapa `files` com `sha256` e `bytes` para cada `specs/<NNN>-*/converge.md` presente no alvo.
- **FR-002**: O aparato DEVE materializar `docs/governance/sdd-completeness-baseline.json` cuja chave `allow_missing` contém exatamente as entradas de `regressions` calculadas por `python3 scripts/ci/check_sdd_completeness.py --json` executado sem baseline, derivando o estado do próprio alvo; no ato da materialização, os scripts são a autoridade sobre contagens históricas divergentes (ver `## Clarifications`).
- **FR-003**: O aparato DEVE materializar `docs/governance/task-id-uniqueness-baseline.json` cuja chave `allow_duplicates` contém exatamente as entradas de `regressions` calculadas por `python3 scripts/ci/check_task_id_uniqueness.py --json` executado sem baseline, sem inventar nenhuma entrada `duplicate_task:T###`.
- **FR-004**: Cada um dos três baselines DEVE registrar proveniência local explícita por meio de campo `_provenance` declarando origem no alvo, método de geração, data e não uso da factory; o campo DEVE ser ignorado pelos gates, que leem apenas as chaves de schema.
- **FR-005**: Nenhum baseline DEVE conter chave de spec da factory, chave de spec removida do alvo ou entrada sem contrapartida no estado corrente; a factory é fonte do aparato, não autoridade dos baselines.
- **FR-006**: O gate de convergência DEVE retornar PASS (exit 0) com `rewritten_or_truncated`, `missing`, `unbaselined` e `stale_baseline` vazios e `baselined_files` igual ao número de `converge.md` presentes.
- **FR-007**: O gate de completude DEVE retornar PASS (exit 0) com `regressions` e `stale_baseline` vazios e `baselined_features` igual ao número de features com gaps aceitos no estado corrente.
- **FR-008**: O gate de unicidade DEVE retornar PASS (exit 0) com `regressions` e `stale_baseline` vazios e `baselined_specs` igual ao número de specs com duplicatas aceitas no estado corrente.
- **FR-009**: Quando o estado corrente não contém entradas para um gate, o baseline correspondente DEVE ser materializado com mapa vazio e `baselined=0`, sem inventar entradas; a dívida 1 fecha por presença do arquivo local, schema conhecido, proveniência e saída colada em R4, com `RG-2`/`RG-4` recusados como prova executável do ratchet, enquanto `RG-1`/`RG-5` permanecem CONDICIONAIS até existir entrada baselinada (contrato §4.3, §8 item 4; ver `## Clarifications`).
- **FR-010**: Um gap ou duplicata SÓ DEVE ser registrado como aceito quando decorre de portaria ainda não executada no ciclo ou de dívida local datada e rastreável; ausência de artefato exigido por fase já concluída, chave órfã ou chave da factory DEVE ser corrigida antes de virar entrada de baseline (ver `## Clarifications`).
- **FR-011**: Nenhuma lógica, limiar, schema, caminho padrão ou regex dos scripts `scripts/ci/check_sdd_completeness.py`, `scripts/ci/check_converge_append_only.py` e `scripts/ci/check_task_id_uniqueness.py` DEVE ser alterado.
- **FR-012**: A saída executada dos três gates, com data/hora, DEVE ser colada como evidência na portaria de evidência; CI, merge, publicação, aprovação e proteção de branch DEVEM permanecer `[NV]`.

### Key Entities *(include if feature involves data)*

- **Baseline local**: arquivo JSON em `docs/governance/`, um por gate; identidade pelo caminho canônico; campos de schema lidos pelos scripts (`allow_missing`, `files`/`schema_version`/`enforce_unbaselined`, `allow_duplicates`) mais o campo de proveniência ignorado por eles; usado para classificar cada achado do estado corrente como coberto, regressão ou `stale`.
- **Entrada do estado corrente**: item calculado por cada script sobre o alvo — feature com artefatos faltantes, `converge.md` com hash/tamanho, spec com `T###` duplicado; é a única fonte autoritativa no ato da bootstrap e o que distingue um baseline honesto de um inventado.
- **Proveniência local**: registro, em cada baseline, de origem no alvo, método, data e não uso da factory; torna auditável a afirmação de que o baseline não foi copiado.
- **Regressão controlada**: alteração artificial e isolada (gap, reescrita/truncamento de `converge.md`, `T###` duplicado) usada para provar que o ratchet de cada gate está ativo antes da portaria de evidência.
- **Evidência de gate**: saída executada de cada um dos três checks com data/hora, colada na portaria de evidência; estado remoto permanece `[NV]`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 3 de 3 arquivos de baseline presentes em `docs/governance/` (baseline: 0 de 3), cada um um JSON válido com as chaves exigidas pelo respectivo gate; meta 3 de 3.
- **SC-002**: 100% das entradas do estado corrente aparecem no baseline correspondente (no estado documentado: completude 1 de 1 feature; convergência 0 de 0 arquivos; unicidade 0 de 0 specs), com zero entradas inventadas e zero chaves da factory.
- **SC-003**: 3 de 3 gates retornam PASS (exit 0) quando executados com seus baselines locais, com zero regressões e zero `stale_baseline` em cada um (baseline: `baselined=0` nos três).
- **SC-004**: `baselined>0` em todo gate cujo estado corrente contém entradas — no estado documentado, `sdd-completeness` com `baselined_features=1` para `003-baseline-governanca` (`release`, `validate`); nos gates sem entradas, `baselined=0` só é aceito com arquivo materializado e proveniência registrada.
- **SC-005**: zero chaves de spec da factory e zero entradas órfãs nos três baselines (baseline: risco registrado em `.specify/memory/STATE.md:50`; meta 0).
- **SC-006**: zero alterações em lógica, limiares, schemas, caminhos padrão e regex dos três scripts (baseline 0; meta 0), verificável por comparação textual.
- **SC-007**: 100% (3 de 3) das saídas executadas dos gates coladas com data/hora na portaria de evidência e zero afirmações de CI, merge, publicação, aprovação ou proteção de branch como obtidas — todas `[NV]`.
- **SC-008**: um cenário de regressão controlado por gate é recusado por 3 de 3 gates antes da portaria de evidência (baseline: sem evidência de detecção; meta 3 de 3 recusas). No estado documentado, os cenários executáveis são `RG-2` (`G-CONV`), `RG-3` (`G-COMP`) e `RG-4` (`G-UNIQ`); `RG-1` e `RG-5` são CONDICIONAIS a baselines com entradas e não entram nessa contagem (contrato §4.3).

## Assumptions

- O estado autoritativo no ato da materialização é o produzido pelos próprios scripts sobre o alvo, não as contagens históricas divergentes de `docs/PROGRESSO.md` e `.specify/memory/STATE.md`.
- `docs/governance/` continua sendo o diretório autoritativo de baselines (valor padrão de `--baseline` nos três scripts) e os schemas atuais permanecem estáveis até a materialização.
- O repositório é documental: nenhuma engine, runtime, banco, telemetria, build, CI ou serviço externo é materializado neste ciclo.
- Os três scripts são lidos como autoridade de formato e não são alterados; campos extras de proveniência são tolerados porque os carregadores leem apenas chaves nomeadas.
- Artefatos de portarias ainda não executadas neste ciclo (`validate`/R4 e `release`/R7) são dívida legítima e baselinável para a feature 003; artefatos exigidos pelas fases já concluídas não são.
- Um baseline com mapa vazio é honesto quando o estado corrente não tem entradas, e o valor materializado está em proveniência, schema local e ratchet ativo, não em contagem inflada.
- A autorização explícita do pedido e o veredito `go` da decisão R1 autorizam a materialização; aprovação em plataforma remota permanece `[NV]`.
- A execução dos gates para produzir evidência com data/hora pertence à portaria de evidência do ciclo completo; os cenários e comandos são descritos nesta feature, mas nenhum verde remoto é afirmado.
- Nenhum segredo, token ou credencial é necessário ou registrado em qualquer baseline.

## Clarifications

### Session 2026-11-19

- Q: Qual é o estado exato do alvo no ato da materialização, dadas as contagens históricas divergentes `spec_dirs=0/1/2`? → A: Os próprios scripts, executados no ato da bootstrap com `--json` e sem baseline, são a fonte autoritativa; as contagens históricas são contexto datado, não autoridade. Estado esperado no ato da materialização: três diretórios numerados, `003-baseline-governanca` com os artefatos `release` e `validate` ainda ausentes (portarias futuras), nenhum `converge.md` e nenhum `T###` duplicado.
- Q: Um baseline com mapas vazios satisfaz a dívida 1 ou exige ao menos uma entrada por gate? → A: Satisfaz quando o estado corrente não tem entradas; a dívida 1 fecha com a presença do arquivo local, do schema conhecido, da proveniência e da saída colada, nunca com entradas inventadas. No estado atual, convergência e unicidade têm mapas vazios e completude tem uma entrada legítima.
- Q: Que critério separa gap legítimo (a baselinar) de defeito a corrigir antes de virar baseline? → A: É legítimo o gap decorrente de portaria ainda não executada no ciclo ou de dívida local datada e rastreável; é defeito a ausência de artefato que a própria fase já deveria ter produzido (`spec.md`, `plan.md`, `tasks.md`, `checklists`, `assessment`) ou qualquer chave órfã/da factory. Aplicando ao estado atual, `release` e `validate` de `003-baseline-governanca` são legítimos; qualquer outra entrada exigiria correção prévia.

Nenhuma pergunta foi feita ao usuário: as três ambiguidades foram resolvidas uma por vez por evidência local (decisão R1, scripts de gate, STATE, PROGRESSO e artefatos da feature 003). Nenhum marcador de esclarecimento pendente permaneceu e R3 foi respeitado (≤3 esclarecimentos, um por vez).

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.

Esta feature produz exclusivamente três artefatos JSON locais em `docs/governance/` e não gera manifesto de orçamento de contexto nem saídas com handles de contexto. Diferentemente de specs anteriores, os comandos dos gates determinísticos locais são parte do escopo desta feature — são o mecanismo de bootstrap e de verificação —, mas a colagem da saída executada com data/hora pertence à portaria de evidência (R4), não a esta spec. A citação de comandos em FR-001–FR-003 é **exceção documental justificada**: o comando é o mecanismo do objeto da feature, conforme nota em `## Requirements`. Nenhum efeito remoto, CI, merge, publicação ou proteção de branch é afirmado; tudo permanece `[NV]` até confirmação ao vivo.
