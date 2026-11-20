# Feature Specification: Evidência auditável do gate `make check` e registro do não verificado

**Feature Branch**: `012-evidencia-make-check` · **Created**: 2026-11-20 · **Status**: Draft · **Data:** 2026-11-20T03:19Z

**Input**: Task 6 de `docs/plans/2026-11-19-ciclo-80-20-omoba.md` (linhas 1219–1304), literal.

## User Scenarios & Testing

### User Story 1 — Porta única de verificação (P1)
Um único comando `make check` executa, em sequência e no mesmo processo, as cinco suítes `test_contrato`, `test_servidor`, `test_validacao`, `test_ilha_sincronia`, `test_manifesto` e o gate `check-compat`, parando no primeiro erro. No cenário atual imprime `OK test_contrato`, `OK test_servidor`, `OK test_validacao`, `OK test_ilha_sincronia`, `OK test_manifesto`, `COMPAT_OK module_id=insurgencia.exemplo` e termina com status 0, sem nenhuma linha `FAIL` ou `COMPAT_FALHA`.

**Why this priority**: é a fatia mínima que transforma cinco verificações dispersas numa porta repetível e auditável (doc-16, ast-01), destravando a evidência das Tasks anteriores sem serviço externo.

**Independent Test**: `make check` → sequência literal acima, código de saída `0`, nenhuma linha `FAIL`/`COMPAT_FALHA`.

### User Story 2 — Evidência com cabeçalho fixo e não verificado explícito (P1)
`docs/progress/evidence.csv` registra 9 colunas na ordem literal `id,criterio,verdict,source,command,result,confidence,unverified,recorded_at`, com pelo menos uma linha por critério CE-1..CE-5. Enquanto o comando de uma linha não é executado, `verdict` permanece `NAO_VERIFICADO`, `result` e `recorded_at` vazios e `unverified` não vazio; nunca é registrado sucesso antes da execução observada.

**Why this priority**: sem o cabeçalho literal e a marcação explícita do não observado, a evidência pareceria completa quando ainda não foi comandada (doc-38, doc-30, doc-44).

**Independent Test**: comparar a primeira linha do CSV com o cabeçalho literal e executar `grep -c -E 'NAO_VERIFICADO|NAO_VERIFICADO' docs/progress/evidence.csv` → valor `≥ 1`.

### User Story 3 — Registro datado ligado aos ids da evidência (P2)
`docs/progress/2026-11-19-ciclo-80-20.md` existe como registro datado do ciclo, referencia os ids `EV-20261119-001` a `EV-20261119-007`, fixa o estado `ilha_concluida`, declara o fora do escopo (GDD §16.3/§16.4) e o marco de reavaliação ao final de `make check`.

**Why this priority**: liga o histórico datado (doc-31) à evidência comandada, permitindo reconstruir o que foi verificado e o que ficou pendente sem reinterpretar intenção (CE-5.5).

**Independent Test**: `test -f docs/progress/2026-11-19-ciclo-80-20.md` → código de saída `0`, com o corpo citando `docs/progress/evidence.csv` e os ids.

### Edge Cases
- Linha ainda não comandada → `verdict=NAO_VERIFICADO`, `result` e `recorded_at` vazios, `unverified` preenchido; `confidence` só passa a `high`/`medium`/`low` após saída observada.
- Testes de loopback (CE-2.1/CE-2.5 e CE-3.6) permanecem `NAO_VERIFICADO` até execução local: `SERVIDOR_PRONTO porta=7777` foi observado, mas não implica cobertura integral da árvore canônica (39/39) nem aceitação automática dessas linhas.
- `make check` para no primeiro erro: se uma suíte falhar, as subsequentes não são executadas e nenhuma linha final de sucesso é impressa.
- Campo `result` com vírgulas ou aspas → valor entre aspas no CSV, preservando as 9 colunas.
- A linha agregada CE-5.1 (`EV-20261119-007`) é preenchida com `make check 2>&1 | tail -1` **somente após** a execução; a saída é copiada literalmente, sem reescrita.
- Saídas pertencem à portaria de validação R4 (com data/hora e **[NV]** para o não observado) e versão/changelog/rollback à R7; nenhum desses artefatos é escrito por esta especificação.

## Requirements

### Functional Requirements

- **FR-001**: O `Makefile` expõe o alvo `check` como `.PHONY`, com `GODOT ?= godot`, `GODOT_FLAGS := --headless --path . --script` e dependências `test-contrato test-servidor test-validacao test-ilha test-manifesto check-compat` nessa ordem; a receita para no primeiro erro e não depende de rede, conta, toolchain Cargo ou toolchain .NET.
- **FR-002**: `docs/progress/evidence.csv` tem cabeçalho literal de 9 colunas nesta ordem: `id,criterio,verdict,source,command,result,confidence,unverified,recorded_at`; contém pelo menos uma linha por critério CE-1..CE-5, com ids `EV-20261119-001` a `EV-20261119-007`, `command` executável e `source` apontando para o script ou arquivo de origem correspondente.
- **FR-003**: `docs/progress/2026-11-19-ciclo-80-20.md` é o registro datado exigido por CE-5.5: título com a data do ciclo, referência explícita a `docs/progress/evidence.csv` e aos ids `EV-20261119-001`..`EV-20261119-007`, estado persistido `ilha_concluida`, fora do escopo declarado (GDD §16.3/§16.4: condição completa de vitória da Ilha Central, elenco Artesão/Guerreiro e partidas 5v5) e marco de reavaliação após `make check`.
- **FR-004**: O alvo `check` reexecuta os sete scripts listados como `Test:` na Task 6 — `tests/test_contrato.gd`, `tests/test_servidor.gd`, `tests/test_validacao.gd`, `tests/test_ilha_sincronia.gd`, `tests/test_manifesto.gd`, `tests/test_loopback.gd`, `tests/test_loopback_ilha.gd` — sem editar nenhum deles; os alvos `test-contrato`, `test-servidor`, `test-validacao`, `test-ilha`, `test-manifesto` e `check-compat` invocam os cinco primeiros e o verificador de compatibilidade com o binário fixado e as flags `--headless --path . --script`.
- **FR-005**: Toda linha de `evidence.csv` começa com `verdict=NAO_VERIFICADO` e `unverified` não vazio; `verdict` só muda quando o comando de `command` for executado e a saída observada registrada em `result`, passando `confidence` a `high`/`medium`/`low` e `unverified` a vazio apenas no caso `PASS`. Nenhuma linha registra sucesso antes da execução, e `grep -c` do marcador retorna valor `≥ 1`.
- **FR-006**: Registrar as saídas executadas (com data/hora e marcação **[NV]** para o não observado) na portaria de validação R4; versão, changelog com FR/T e rollback em R7. Nenhum artefato R4/R7 é escrito nesta especificação.

### Key Entities

- **Evidência (`EV-20261119-001`..`EV-20261119-007`)**: linha do CSV que liga um critério CE a um `source`, um `command`, um `verdict`, uma `confidence` e um carimbo `recorded_at`.
- **Porta `make check`**: sequência ordenada de alvos `.PHONY` que falha rápido e produz a saída agregada usada na linha CE-5.1.
- **Registro datado**: arquivo `docs/progress/2026-11-19-ciclo-80-20.md`, âncora legível do ciclo e dos ids de evidência.

## Success Criteria

- **SC-001**: `make check` → sequência `OK test_contrato`, `OK test_servidor`, `OK test_validacao`, `OK test_ilha_sincronia`, `OK test_manifesto`, `COMPAT_OK module_id=insurgencia.exemplo`, código de saída `0` e nenhuma linha `FAIL` ou `COMPAT_FALHA`. Contagens observadas nesta sessão: `test_contrato` 16 `PASS`, `test_servidor` 6, `test_validacao` 9, `test_ilha_sincronia` 14 e `test_manifesto` 7; `SERVIDOR_PRONTO porta=7777` e `COMPAT_OK` também observados.
- **SC-002**: A primeira linha de `docs/progress/evidence.csv` é exatamente `id,criterio,verdict,source,command,result,confidence,unverified,recorded_at`, com as 9 colunas na ordem literal e nenhuma renomeação.
- **SC-003**: `grep -c -E 'NAO_VERIFICADO|NAO_VERIFICADO' docs/progress/evidence.csv` → valor `≥ 1`, garantindo a marcação explícita do não verificado mesmo após execuções parciais.
- **SC-004**: `test -f docs/progress/2026-11-19-ciclo-80-20.md` → código de saída `0` (registro datado presente) e o corpo referencia os ids de evidência do ciclo.
- **SC-005**: Os 5 gates determinísticos do alvo terminam `PASS` (observado nesta sessão); as saídas são registradas com data/hora na portaria R4 na fase própria, sem escrita de artefatos R4/R7 aqui.

## Out of scope
Artefatos das portarias R4/R7, escrita de `Makefile` ou `evidence.csv` nesta etapa de especificação, sondagem de lobby/worker, publicação de release, partidas 5v5, Ilha Meio, vitória da Ilha Central, elenco Artesão/Guerreiro, deploy, remote, CI remoto, factory, alteração de baselines, gates ou configuração de CI. Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) é feita por este ciclo; o que não foi executado permanece **[NV]**.
