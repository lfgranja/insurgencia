# Feature Specification: Godot headless benchmark

**Feature Branch**: `feature/006-godot-headless-benchmark`

**Created**: 2026-11-20

**Status**: Draft

**Input**: pedido do mantenedor em 2026-11-20, com veredito R1 `go` da decisão registrada em `decision.md` (Opção A), evidência citada em `research.md` e autorização explícita para instalar release oficial verificada e executar o benchmark headless. A carga é sintética: nenhum artefato desta feature a descreve como benchmark 5v5 real.

## User Scenarios & Testing *(mandatory)*

As histórias abaixo são ordenadas por prioridade e cada uma é independentemente testável.

### User Story 1 — Release oficial identificada e integridade provada (Priority: P1)

O mantenedor precisa saber qual release da engine foi usada e provar que o artefato não divergiu entre a publicação upstream e o uso no alvo. Hoje o alvo não registra engine instalada; sem release nomeada e sem comparação de checksum, a origem do binário seria uma suposição.

**Why this priority**: é a condição de confiança para tudo o que vem depois — sem proveniência e integridade, a identidade do binário e a medição perdem valor auditável.

**Independent Test**: registrar a release `4.7.2-stable` e o par esperado/calculado do checksum, conferindo a igualdade que produziu `CHECKSUM_OK` em 2026-11-20T00:43:52Z; nenhum outro artefato é necessário para esse teste.

**Acceptance Scenarios**:

1. **Given** o pedido de instalar a engine no alvo, **When** a release é identificada, **Then** o valor `4.7.2-stable` aparece rastreado à consulta `gh api repos/godotengine/godot/releases/tags/4.7.2-stable`.
2. **Given** o valor esperado `9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65` (128-hex; exibição: `9aa00f7a...c65`) na saída de `grep 'Godot_v4.7.2-stable_linux.x86_64.zip' SHA512-SUMS.txt`, **When** `sha512sum Godot_v4.7.2-stable_linux.x86_64.zip` é calculado, **Then** o valor calculado é idêntico ao esperado e o resultado `CHECKSUM_OK` é registrado com data/hora 2026-11-20T00:43:52Z.
3. **Given** divergência entre esperado e calculado, **When** a comparação é registrada, **Then** o resultado é uma falha explícita e nenhuma identidade de binário é afirmada a partir daquele artefato.

---

### User Story 2 — Identidade do binário instalado (Priority: P1)

O mantenedor precisa que a versão efetivamente instalada seja registrada pela string que o próprio executável reporta, e não deduzida do nome do arquivo ou da release baixada.

**Why this priority**: fixa a identidade da ferramenta que produziu a medição; sem ela, comparações futuras não têm âncora.

**Independent Test**: registrar a string `4.7.2.stable.official.ed1daf0bf` com data/hora 2026-11-20T00:44:11Z, independentemente da execução do benchmark.

**Acceptance Scenarios**:

1. **Given** um artefato de release íntegro, **When** o binário é instalado e consultado, **Then** a string reportada `4.7.2.stable.official.ed1daf0bf` é registrada com comando e data/hora.
2. **Given** a string registrada, **When** a evidência é revisada, **Then** a identidade da engine é aquela string, e nenhuma versão é inferida apenas do nome do arquivo.

---

### User Story 3 — Medição headless da carga sintética (Priority: P1)

Quem for implementar a simulação precisa de uma linha de base do custo por tick obtida com a carga já versionada em `tests/benchmark/headless_tick.gd`, com estatísticas suficientes para mostrar a dispersão.

**Why this priority**: é o número que permite detectar regressões futuras; sem ele, o desempenho da simulação permanece uma incógnita.

**Independent Test**: registrar 600 ticks × 64 entidades, 3 amostras iniciais (9,43 / 15,50 / 17,48 µs/tick) em 2026-11-20T00:44:35Z e 7 amostras (25,49 / 42,95 / 22,71 / 27,64 / 39,45 / 39,99 / 37,48 µs/tick) em 2026-11-20T00:44:48Z, obtidas com `$G --headless --path tests/benchmark --script headless_tick.gd`; mediana 37,48 µs/tick, mínimo 22,71, máximo 42,95 e `accumulator=-297.626094` — sem depender da história 1 ou 2 para ser verificado.

**Acceptance Scenarios**:

1. **Given** o arquivo `tests/benchmark/headless_tick.gd` com `TICKS=600` e `ENTITIES=64`, **When** o benchmark headless é executado, **Then** a saída traz `ticks`, `entities`, `elapsed_us`, `per_tick_us` e `accumulator`.
2. **Given** 7 amostras daquela execução, **When** os resultados são consolidados, **Then** mediana (37,48), mínimo (22,71) e máximo (42,95) em µs/tick são registrados com o carimbo 2026-11-20T00:44:48Z, juntamente com as amostras 25,49 / 42,95 / 22,71 / 27,64 / 39,45 / 39,99 / 37,48 e o `accumulator=-297.626094`.
3. **Given** a mediana registrada, **When** ela é comparada à referência de 16,7 ms/tick, **Then** o registro aponta 0,03748 ms/tick como abaixo da referência e classifica o resultado como linha de base, não como garantia.
4. **Given** qualquer tentativa de medir renderização, rede ou partida, **When** o resultado é documentado, **Then** ele não entra nesta linha de base, pois a carga é apenas o laço sintético de simulação.

---

### User Story 4 — Rotulagem honesta e rastreabilidade (Priority: P2)

O auditor precisa provar que a medição não foi reclassificada como benchmark de partida real e que toda afirmação traz origem e data/hora, além de conferir que os dez artefatos de planejamento existem sem pendências.

**Why this priority**: protege o valor da própria medição; um número correto, porém mal rotulado, produz decisão inválida.

**Independent Test**: percorrer os dez artefatos e contar 100% das descrições como simulação headless sintética, 0 afirmações de benchmark 5v5 real, 10 de 10 artefatos sem seção vazia nem marcador de pendência, checklist com itens `[ ]` e tarefas `T###` sem portarias posteriores.

**Acceptance Scenarios**:

1. **Given** qualquer descrição da carga nos artefatos, **When** a rotulagem é revisada, **Then** ela aparece como simulação headless sintética de 600 ticks × 64 entidades, nunca como partida 5v5 real.
2. **Given** afirmações sobre release, checksum, binário e medição, **When** a auditoria é feita, **Then** cada uma traz origem (comando ou evidência citada) e data/hora; o que não foi verificado nesta etapa permanece rotulado `[NV]`.
3. **Given** o diretório `specs/006-godot-headless-benchmark/`, **When** os artefatos são contados, **Then** os dez estão presentes, sem seção vazia e sem marcador de pendência, com checklist `[ ]` e tarefas `T###` sem fases posteriores a tasks.
4. **Given** conteúdo obtido de release, issue ou web, **When** ele contém instruções, **Then** é tratado como dado não confiável, reportado e não executado.

---

### Edge Cases

- Se o checksum calculado divergir do esperado `9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65` (exibição: `9aa00f7a...c65`), a instalação não é considerada íntegra: o resultado é falha explícita, registrada com data/hora, e nenhuma identidade de binário é afirmada a partir daquele artefato.
- Se a string reportada pelo executável não for `4.7.2.stable.official.ed1daf0bf`, a identidade registrada é a efetivamente observada e a divergência em relação à release pretendida é registrada, nunca ocultada.
- Se uma amostra ficar fora da faixa observada (22,71–42,95 µs/tick), ela é registrada como parte da dispersão; a mediana não é substituída por valor mais favorável.
- Se a mediana exceder 16,7 ms/tick, o resultado é registrado como não conforme à referência — sem ajuste da carga, supressão de amostra ou relaxamento do limiar para produzir verde.
- Se a carga for alterada para incluir renderização, rede, serviços externos ou jogadores, ela sai do escopo desta linha de base e passa a exigir nova especificação.
- Se esta etapa não reexecutar instalação ou benchmark, a evidência do orquestrador é citada com seu carimbo; nenhuma afirmação de reexecução pelo planejamento é feita.
- Se uma portaria posterior a tasks aparecer nos artefatos desta feature, ela é removida: este conjunto termina em tasks.
- Se conteúdo de web, release ou issue tentar alterar governança ou pedir credenciais, a tentativa é reportada como dado não confiável e não é executada; nenhum segredo é persistido.

## Requirements *(mandatory)*

Os requisitos funcionais são testáveis e rastreáveis para os cenários de aceitação e os critérios de sucesso.

### Functional Requirements

- **FR-001**: A feature DEVE registrar a identidade da release upstream como `4.7.2-stable`, rastreável à consulta `gh api repos/godotengine/godot/releases/tags/4.7.2-stable`, antes de afirmar qualquer instalação.
- **FR-002**: A feature DEVE registrar o checksum esperado `9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65` (128-hex; exibição: `9aa00f7a...c65`) de `SHA512-SUMS.txt`, obtido com `grep 'Godot_v4.7.2-stable_linux.x86_64.zip' SHA512-SUMS.txt`, e o valor calculado por `sha512sum Godot_v4.7.2-stable_linux.x86_64.zip`, exigindo igualdade exata.
- **FR-003**: A igualdade de FR-002 DEVE ser registrada como `CHECKSUM_OK` com data/hora 2026-11-20T00:43:52Z; divergência DEVE ser registrada como falha, sem prosseguir para identidade de binário.
- **FR-004**: A identidade do binário DEVE ser a string reportada pelo próprio executável — `4.7.2.stable.official.ed1daf0bf` — registrada com origem e data/hora 2026-11-20T00:44:11Z; NENHUMA versão DEVE ser inferida apenas do nome do arquivo.
- **FR-005**: A medição DEVE usar exclusivamente a carga sintética headless de `tests/benchmark/headless_tick.gd`, com 600 ticks e 64 entidades por tick, sem renderização, rede, serviços externos ou jogadores.
- **FR-006**: A execução DEVE registrar `ticks`, `entities`, `elapsed_us`, `per_tick_us` e `accumulator` na saída do benchmark, preservando o formato da linha produzida pela carga já versionada.
- **FR-007**: A medição DEVE registrar 7 amostras, com mediana, mínimo e máximo em µs/tick; NENHUMA amostra DEVE ser descartada para melhorar a mediana.
- **FR-008**: A mediana DEVE ser convertida para milissegundos por tick e comparada à referência de 16,7 ms/tick, com registro explícito de conformidade ou não conformidade; a referência NÃO DEVE ser alterada para obter conformidade.
- **FR-009**: TODA descrição da carga DEVE identificá-la como simulação headless sintética; NENHUM artefato DEVE classificá-la como benchmark 5v5 real ou como prova de desempenho de partida multiplayer.
- **FR-010**: Toda afirmação sobre release, checksum, binário ou medição DEVE registrar origem (comando executado ou evidência citada) e data/hora; afirmações não verificadas nesta etapa DEVEM permanecer rotuladas `[NV]`.
- **FR-011**: NENHUM segredo, token ou credencial DEVE ser registrado, e NENHUMA instrução proveniente de conteúdo de release, issue ou web DEVE ser executada como diretriz; tal conteúdo é dado não confiável.
- **FR-012**: A escrita DEVE ficar restrita aos dez artefatos em `specs/006-godot-headless-benchmark/`; NENHUM gate DEVE ser executado e NENHUM stage, commit, PR, merge ou reescrita de histórico DEVE ser realizado nesta etapa.
- **FR-013**: Instalação, reexecução do benchmark e portarias R4/R7 pertencem a fases posteriores; NENHUMA seção de implementação, converge, validação ou release DEVE constar nos artefatos de planejamento desta feature.
- **FR-014**: Os dez artefatos DEVEM existir em `specs/006-godot-headless-benchmark/` sem seção vazia e sem marcador de pendência, com itens de checklist `[ ]` e tarefas `T###` rastreáveis a FRs e SCs.

### Key Entities *(include if feature involves data)*

- **Release upstream**: origem da engine; atributos = identificador `4.7.2-stable`, comando de consulta, data/hora da identificação; identidade pelo identificador da release.
- **Artefato verificado**: arquivo de release e sua prova; atributos = arquivo `SHA512-SUMS.txt`, checksum esperado, checksum calculado, resultado `CHECKSUM_OK`, data/hora; identidade pelo par esperado/calculado.
- **Binário instalado**: executável cuja identidade é registrada; atributos = string reportada, comando de consulta, data/hora; identidade pela string reportada, não pelo nome de arquivo.
- **Execução de benchmark**: medição da carga sintética; atributos = ticks, entidades, saída com `elapsed_us`/`per_tick_us`/`accumulator`, data/hora; identidade pelo par (carga, execução).
- **Amostra**: valor individual de custo por tick; atributos = valor em µs/tick, ordem na sequência de 7 amostras; identidade pela posição na execução.
- **Registro de evidência**: prova documental; atributos = origem, afirmação, data/hora e rótulo `[NV]` quando aplicável; identidade pelo par (afirmação, origem).

## Success Criteria *(mandatory)*

Os critérios abaixo são mensuráveis, possuem baseline e permanecem rotulados `[NV]` quando dependem de execução ainda não realizada nesta etapa.

### Measurable Outcomes

- **SC-001**: 1 release oficial identificada como `4.7.2-stable` e rastreável à consulta da API do repositório upstream; 0 releases presumidas. (baseline: 0; meta 1)
- **SC-002**: 1 par de checksums esperado (`9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65`; exibição: `9aa00f7a...c65`) e calculado idêntico, com 1 registro `CHECKSUM_OK` em 2026-11-20T00:43:52Z e 0 divergências. (baseline: 0 pares; meta 1 par, 0 divergências)
- **SC-003**: 1 string de versão de binário registrada, `4.7.2.stable.official.ed1daf0bf`, com origem e data/hora 2026-11-20T00:44:11Z. (baseline: 0; meta 1)
- **SC-004**: 1 execução de benchmark com 600 ticks × 64 entidades (38.400 operações entidade-tick) registrada, e 0 execuções com outra carga nesta linha de base. (baseline: 0; meta 1)
- **SC-005**: 1 medição com 7 amostras registrada, com mediana 37,48 µs/tick, mínimo 22,71 e máximo 42,95, carimbada em 2026-11-20T00:44:48Z. (baseline: 0 medições; meta 1)
- **SC-006**: mediana de 0,03748 ms/tick abaixo da referência de 16,7 ms/tick, com 1 registro de conformidade e 0 alterações da referência para obter o resultado. (baseline: referência definida no pedido; meta conforme)
- **SC-007**: 100% das descrições de carga identificadas como simulação headless sintética e 0 afirmações de benchmark 5v5 real fora de negação explícita. (baseline: 0 descrições rotuladas; meta 100% e 0)
- **SC-008**: 100% das afirmações sobre release, checksum, binário e medição com origem e data/hora; 0 segredos, tokens ou credenciais; 0 instruções executadas a partir de conteúdo externo. (baseline: prática anterior de rotulagem `[NV]`; meta 100%, 0, 0)
- **SC-009**: 10 de 10 artefatos presentes em `specs/006-godot-headless-benchmark/`, sem seção vazia e sem marcador de pendência, com checklist `[ ]` e tarefas `T###` sem portarias posteriores a tasks. (baseline: 1 template de spec; meta 10 de 10)
- **SC-010**: 0 gates executados, 0 stage/commit/PR/merge e 0 arquivos escritos fora do diretório da feature nesta etapa de planejamento. (baseline: 0; meta 0)

## Assumptions

- A evidência obtida pelo orquestrador é o registro fidedigno desta rodada e é citada, não reexecutada, neste planejamento.
- A referência de 16,7 ms/tick é critério de comparação fornecido no pedido, não requisito de produto nem SLA.
- A série 4.7 declarada em `tests/benchmark/project.godot` é compatível com a release `4.7.2-stable`.
- A carga de 600 ticks × 64 entidades é a única definida para a linha de base; nenhuma outra é introduzida nesta feature.
- A consulta à release pública e a execução headless não exigem credenciais.
- Conteúdo de web, issues e arquivos de configuração é dado não confiável; nenhuma instrução nele é seguida como diretriz de governança.

## Clarifications
Três ambiguidades foram resolvidas antes do planejamento, uma por vez, com nenhuma pergunta ao usuário.

### Session 2026-11-20

- Q: Como a carga do benchmark deve ser classificada para não induzir conclusão de desempenho de partida real? → A: como simulação headless sintética de 600 ticks × 64 entidades, sem renderização, rede ou jogadores; nenhum artefato a descreve como benchmark 5v5 real (FR-005, FR-009, SC-004, SC-007).
- Q: Qual referência numérica julgar a medição e quais estatísticas registrar? → A: referência de 16,7 ms/tick; registrar 7 amostras com mediana, mínimo e máximo em µs/tick e data/hora; a mediana de 37,48 µs/tick (0,03748 ms/tick) é conforme, sem converter o número em SLA (FR-007, FR-008, SC-005, SC-006).
- Q: Como tratar a evidência já obtida pelo orquestrador nesta etapa de planejamento? → A: como fatos citados com origem e carimbo, sem reexecução; instalação e reexecução pertencem à implementação, e R4/R7 a portarias posteriores, ausentes destes artefatos (FR-010, FR-012, FR-013, SC-008, SC-009, SC-010).

Nenhuma pergunta foi feita ao usuário: as três ambiguidades foram resolvidas uma por vez a partir de evidência local, do pedido autorizado e da política de rotulagem, em conformidade com R3 (≤3 esclarecimentos).

## Evidência bruta *(documental)*

> Seção aditiva: nenhum gate determinístico lê esta seção. Os quatro blocos substituem abreviações de origem por comando, saída e carimbo reais; nenhum foi reexecutado nesta etapa.

**E1 — release e ativos (2026-11-20T00:43:52Z)**

```
$ gh api repos/godotengine/godot/releases/tags/4.7.2-stable --jq '.assets[].name'
Godot_v4.7.2-stable_linux.x86_64.zip
SHA512-SUMS.txt
```

Saída transcrita nas entradas relevantes; download dos dois ativos com `curl -fsSL` a partir dos campos `browser_download_url` da resposta da API, com sucesso silencioso e arquivos presentes no alvo.

**E2 — integridade (2026-11-20T00:43:52Z)**

```
$ grep 'Godot_v4.7.2-stable_linux.x86_64.zip' SHA512-SUMS.txt
9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65  Godot_v4.7.2-stable_linux.x86_64.zip
$ sha512sum Godot_v4.7.2-stable_linux.x86_64.zip
9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65  Godot_v4.7.2-stable_linux.x86_64.zip
```

Comparação esperado == calculado → `CHECKSUM_OK`.

**E3 — identidade do binário (2026-11-20T00:44:11Z)**

```
$ Godot_v4.7.2-stable_linux.x86_64 --headless --version
4.7.2.stable.official.ed1daf0bf
```

**E4 — carga sintética (2026-11-20T00:44:35Z e 2026-11-20T00:44:48Z)**

```
$ G=Godot_v4.7.2-stable_linux.x86_64
$ $G --headless --path tests/benchmark --script headless_tick.gd
```

- 2026-11-20T00:44:35Z — 3 amostras iniciais: 9,43 / 15,50 / 17,48 µs/tick.
- 2026-11-20T00:44:48Z — 7 amostras: 25,49 / 42,95 / 22,71 / 27,64 / 39,45 / 39,99 / 37,48 µs/tick; mediana 37,48 µs/tick; trecho da linha `BENCHMARK`: `accumulator=-297.626094`.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico lê esta seção.

Esta feature não produz engine de jogo, API, serviço, banco ou migração: ela fixa identidade de release, integridade de artefato, identidade de binário e uma linha de base de custo por tick, com evidência auditável. Não é exigido manifesto de orçamento de contexto nem saídas com handles. As decisões técnicas ficam consolidadas em `plan.md` e as formas de prova em `checklists/requirements.md`, `checklists/benchmark-quality.md` e `tasks.md`. Nenhuma afirmação de execução de gates, stage, commit, merge ou portaria posterior a tasks é feita aqui; o que depende de verificação ao vivo permanece rotulado `[NV]` até comando executado com saída colada e data/hora.
