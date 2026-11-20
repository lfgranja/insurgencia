# Implementation Plan: Godot headless benchmark

**Feature**: `006-godot-headless-benchmark` — fixar release oficial e integridade da engine Godot no alvo, registrar a identidade do binário instalado e medir o custo por tick da carga sintética headless versionada, com rotulagem que impeça leitura como partida 5v5 real.
**Input**: [spec.md](spec.md), [research.md](research.md), [decision.md](decision.md) (R1 `go`, Opção A), `AGENTS.md`, `.specify/memory/constitution.md`, `docs/governance/sdd-phase-contracts.md`.

## Technical Context

| Campo | Valor |
|---|---|
| Tipo de feature | Verificação de proveniência/integridade de runtime e medição documental de desempenho; sem código de produto |
| Release upstream | `4.7.2-stable`, identificada por `gh api repos/godotengine/godot/releases/tags/4.7.2-stable --jq '.assets[].name'` → `Godot_v4.7.2-stable_linux.x86_64.zip` e `SHA512-SUMS.txt`; download com `curl -fsSL` |
| Prova de integridade | `grep 'Godot_v4.7.2-stable_linux.x86_64.zip' SHA512-SUMS.txt` → esperado `9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65` (exibição: `9aa00f7a...c65`); `sha512sum` local idêntico → `CHECKSUM_OK` em 2026-11-20T00:43:52Z |
| Identidade do binário | `Godot_v4.7.2-stable_linux.x86_64 --headless --version` → `4.7.2.stable.official.ed1daf0bf` em 2026-11-20T00:44:11Z |
| Carga de medição | `tests/benchmark/headless_tick.gd`: 600 ticks × 64 entidades, `sin((tick+entity)*0.01)`, sem renderização, rede ou serviços externos |
| Configuração do projeto de benchmark | `tests/benchmark/project.godot`: `config/name="Insurgencia Headless Benchmark"`, `config/features=PackedStringArray("4.7")`, `run/main_scene=""` |
| Resultado registrado | `$G --headless --path tests/benchmark --script headless_tick.gd`: 3 amostras iniciais (9,43 / 15,50 / 17,48) em 2026-11-20T00:44:35Z e 7 amostras (25,49 / 42,95 / 22,71 / 27,64 / 39,45 / 39,99 / 37,48) em 2026-11-20T00:44:48Z; mediana 37,48 µs/tick; `accumulator=-297.626094` |
| Referência de comparação | 16,7 ms/tick (orçamento de quadro a 60 FPS); mediana equivalente a 0,03748 ms/tick |
| Unidade de evidência | Origem (comando ou evidência citada) + afirmação + data/hora; `[NV]` até confirmação ao vivo |
| Artefatos de design | Consolidados neste `plan.md` e em `research.md`; `data-model.md`, `contracts/` e `quickstart.md` não se aplicam — não há API externa, dados persistidos nem comando de produto a documentar |
| Escrita nesta etapa | Restrita a `specs/006-godot-headless-benchmark/`; nenhum gate executado, nenhum stage, commit, PR ou merge |

Esclarecimentos pendentes no contexto: nenhum. As três ambiguidades foram resolvidas em `spec.md` / `## Clarifications` / `### Session 2026-11-20`, antes deste plano (R3 respeitado).

## Constitution Check

| Princípio / norma | Reavaliação | Status |
|---|---|---|
| I — Determinismo primeiro | Release, checksum, versão e medição são resultados de comandos registrados, com valores e carimbos citados (FR-001–FR-008) | PASS |
| II — Adaptadores e portas | Não há código de domínio nem chamada a SDK/provedor; a consulta à release é comando pontual e o benchmark é carga local, sem acoplar a simulação a serviço externo | PASS |
| III — Fluxo SDD ordenado | intake→research→define→shape→decide(`go`)→specify→clarify→plan→checklist→tasks, nesta ordem, sem portaria posterior a tasks | PASS |
| IV — Soberania do jogador | Release pública verificada por checksum; nenhuma dependência proprietária no caminho crítico da simulação | PASS |
| V — Materialismo e ação coletiva | Fora do escopo de produto; nenhuma regra, habilidade ou resultado de partida é alterado | PASS |
| VI — Verificação honesta | Evidência do orquestrador é citada, não reexecutada; natureza sintética é explicitada; nenhum verde é afirmado sem origem e data/hora (FR-009–FR-011) | PASS |
| Norma — gates do alvo | Nenhum gate é executado nesta etapa; nenhum gate da factory é adotado (FR-012) | PASS |
| Norma — commits/branch/PR | Nenhum stage, commit, PR, merge ou reescrita de histórico nesta etapa (FR-012, SC-010) | PASS |
| Contratos de fase (§checklist/tasks) | Checklists com `- [ ] CHK###` e rastreabilidade ≥80%; tasks com `- [ ] T### [P?] [USn?]` e caminho, terminando em tasks | PASS |

Nenhuma violação injustificada. A reexecução da instalação e do benchmark e as portarias R4/R7 pertencem a fases posteriores, não a este plano.

## Decisões de design

- **D1 — Release nomeada**: Opção A de `concept.md`. A identidade `4.7.2-stable` vem da consulta à API upstream e é registrada antes de qualquer afirmação de instalação. Justificativa: FR-001, SC-001.
- **D2 — Integridade por par de checksums**: esperado e calculado são registrados juntos; a igualdade produz `CHECKSUM_OK` com carimbo, e divergência é falha explícita. Justificativa: FR-002, FR-003, SC-002.
- **D3 — Identidade auto-reportada**: a versão do binário é a string que o executável reporta, não deduzida do nome do arquivo. Justificativa: FR-004, SC-003.
- **D4 — Carga fixa**: o benchmark usa somente `tests/benchmark/headless_tick.gd`, com `TICKS=600` e `ENTITIES=64`; nenhuma extensão para partida, renderização ou rede. Justificativa: FR-005, FR-006, SC-004.
- **D5 — Dispersão registrada**: 7 amostras com mediana, mínimo e máximo; nenhuma amostra é descartada para melhorar o número. Justificativa: FR-007, SC-005.
- **D6 — Referência como comparação, não meta**: 16,7 ms/tick é o critério de comparação; a mediana de 0,03748 ms/tick é conforme, sem virar SLA e sem relaxamento do limiar. Justificativa: FR-008, SC-006, Clarificação Q2.
- **D7 — Rotulagem sintética obrigatória**: toda descrição identifica a carga como simulação headless sintética; reclassificação como 5v5 real é vedada em todos os artefatos. Justificativa: FR-009, SC-007, Clarificação Q1.
- **D8 — Evidência com origem e carimbo**: cada afirmação traz comando ou evidência citada e data/hora; o que não foi verificado nesta etapa permanece `[NV]`. Justificativa: FR-010, FR-013, SC-008; `docs/governance/sdd-phase-contracts.md §Evidence`.
- **D9 — Dado externo não confiável**: conteúdo de release, issue ou web é tratado como dado; nenhuma instrução nele é executada e nenhum segredo é persistido. Justificativa: FR-011, SC-008; `AGENTS.md:41-45`.
- **D10 — Escopo de escrita e término em tasks**: somente os dez artefatos em `specs/006-godot-headless-benchmark/`; nenhum gate, stage, commit, PR ou portaria posterior a tasks nesta etapa. Justificativa: FR-012, FR-013, FR-014, SC-009, SC-010.

## Inventário de evidência

| # | Valor | Origem | Carimbo | Rastreio |
|---|---|---|---|---|
| 1 | `4.7.2-stable`; `gh api repos/godotengine/godot/releases/tags/4.7.2-stable --jq '.assets[].name'` → `Godot_v4.7.2-stable_linux.x86_64.zip`, `SHA512-SUMS.txt`; `curl -fsSL` | E1 | 2026-11-20T00:43:52Z | FR-001, SC-001 |
| 2 | esperado `9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65` (exibição: `9aa00f7a...c65`) via `grep 'Godot_v4.7.2-stable_linux.x86_64.zip' SHA512-SUMS.txt` | E2 | 2026-11-20T00:43:52Z | FR-002, SC-002 |
| 3 | calculado idêntico via `sha512sum Godot_v4.7.2-stable_linux.x86_64.zip` → `CHECKSUM_OK` | E2 | 2026-11-20T00:43:52Z | FR-003, SC-002 |
| 4 | `4.7.2.stable.official.ed1daf0bf`, via `Godot_v4.7.2-stable_linux.x86_64 --headless --version` | E3 | 2026-11-20T00:44:11Z | FR-004, SC-003 |
| 5 | 600 ticks × 64 entidades; `sin((tick+entity)*0.01)` | `tests/benchmark/headless_tick.gd:5-19` | 2026-11-20 | FR-005, FR-006, SC-004 |
| 6 | mediana 37,48; mín 22,71; máx 42,95 µs/tick em 7 amostras, com `accumulator=-297.626094`; 3 amostras iniciais 9,43 / 15,50 / 17,48 | E4 | 2026-11-20T00:44:35Z e 2026-11-20T00:44:48Z | FR-007, SC-005 |
| 7 | referência 16,7 ms/tick; mediana 0,03748 ms/tick | pedido do mantenedor e conversão do valor medido | 2026-11-20 | FR-008, SC-006 |

Nenhum item deste inventário é reexecutado nesta etapa. A instalação verificada e a execução do benchmark são objeto da implementação; as portarias de validação e release não pertencem a este conjunto de artefatos.

## Evidência bruta (comando + saída + carimbo)

Transcrição dos quatro blocos citados no inventário; nenhum é reexecutado nesta etapa de planejamento.

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

## Sequência de verificação

1. Confirmar que a identidade da release e o par de checksums estão registrados com origem e carimbo (SC-001, SC-002).
2. Confirmar que a identidade do binário é a string reportada e que divergências seriam registradas, nunca ocultas (SC-003, Edge Cases).
3. Confirmar que a carga registrada é exatamente a de `tests/benchmark/headless_tick.gd`, com 600×64, e que a saída contém os cinco campos definidos (SC-004, FR-006).
4. Confirmar que mediana, mínimo, máximo e contagem de amostras foram registrados e que nenhuma amostra foi descartada (SC-005, FR-007).
5. Confirmar a comparação com 16,7 ms/tick e o registro de conformidade sem alteração da referência (SC-006).
6. Confirmar que toda descrição é sintética e que nenhuma afirmação de partida 5v5 real aparece (SC-007).
7. Confirmar que afirmações têm origem e carimbo, que não há segredos nem instruções externas executadas e que nada depende de execução não verificada sem `[NV]` (SC-008).
8. Contar os dez artefatos, verificar seção vazia e marcador de pendência, checklist `[ ]` e tarefas `T###` sem portarias posteriores (SC-009).
9. Confirmar que nenhum gate foi executado e que nenhum arquivo fora do diretório da feature foi escrito nesta etapa (SC-010).

## Abordagem de verificação documental desta etapa

1. Contar artefatos: dez presentes, sem seção vazia e sem marcador de pendência.
2. Conferir rastreabilidade: cada FR mapeado para ao menos um cenário de aceitação e um SC; cada SC com baseline e meta.
3. Conferir R1 e R3 na decisão e nas clarificações: `go` legítimo e três respostas, uma por vez.
4. Conferir o inventário de evidência: valores e carimbos idênticos aos citados em `research.md`, sem reexecução.
5. Registrar divergências encontradas em `research.md`/`spec.md`; nenhuma é suprimida para produzir aparência de conformidade.

Nenhum comando de gate, build, teste, stage, commit ou efeito remoto é executado nesta feature de planejamento.
