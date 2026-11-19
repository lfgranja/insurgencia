# Implementation Plan: Baselines locais de governança

**Branch**: `003-baseline-governanca` | **Date**: 2026-11-19 | **Spec**: [spec.md](./spec.md)

**Input**: spec esclarecida em `specs/003-baseline-governanca/spec.md` — decisão R1 de 2026-11-19 (veredito `go`, opção A), três respostas em `## Clarifications / ### Session 2026-11-19`, zero marcadores pendentes e `checklists/requirements.md` revalidado em 18/18 (17/18 → 18/18).

**Fase**: 8 (plan) do contrato SDD (`docs/governance/sdd-phase-contracts.md:17`). Antecede checklist (9) e tasks (11). Nenhum gate é executado nesta fase: os comandos aparecem como procedimento de bootstrap e de verificação, e a colagem da saída executada com data/hora pertence à portaria de evidência (R4), não a este plano.

## Summary

Materializar três arquivos JSON em `docs/governance/` — `sdd-completeness-baseline.json`, `converge-append-only-baseline.json` e `task-id-uniqueness-baseline.json` — derivados do estado corrente do alvo pelos próprios scripts locais. O gate de convergência expõe `--bootstrap` e gera diretamente o JSON; os outros dois não expõem gerador nativo, então o baseline é derivado do mapa `regressions` que cada script emite com `--json` quando o arquivo de baseline ainda não existe. Cada arquivo recebe um campo `_provenance` declarando origem local; os carregadores dos gates leem apenas chaves nomeadas e ignoram campos extras, de modo que nenhum script precisa ser alterado. Formatos e schemas extraídos diretamente de `scripts/ci/check_*.py`. A origem autoritativa é a spec; as decisões de desenho estão em [research.md](./research.md), as entidades em [data-model.md](./data-model.md) e o formato executável em [contracts/baseline-bootstrap-contract.md](./contracts/baseline-bootstrap-contract.md).

## Technical Context

| Campo | Valor | Fundamento |
|---|---|---|
| Language/Version | **Python 3 (stdlib)** — os três gates usam `from __future__ import annotations`, anotações `dict[str, ...]`, `argparse`, `json`, `hashlib`, `re`, `collections.Counter` e `pathlib`. Nenhuma linguagem é introduzida: a feature consome os scripts existentes. | scripts `check_sdd_completeness.py:16-21`, `check_converge_append_only.py:10-15`, `check_task_id_uniqueness.py:7-13`; spec FR-011 |
| Primary Dependencies | **Nenhuma instalação** — apenas a biblioteca padrão dos três scripts; nenhum engine, SDK, pacote, serviço ou credencial. | spec Assumptions; constituição IV |
| Storage | Três arquivos JSON versionados em `docs/governance/`, únicos caminhos de persistência; o repositório Git local é o suporte. Sem banco, fila, índice ou cache. | spec FR-001–FR-003; `DEFAULT_BASELINE` nos três scripts |
| Testing | **Execução determinística dos três gates com `--json` e em modo texto**, comparando contagens, `regressions`/`rewritten_or_truncated` e `stale_baseline`; saída executada com data/hora colada na portaria de evidência (R4). | spec SC-003, SC-008, FR-012; quickstart.md |
| Target Platform | Repositório local do alvo, offline; sem binário, runtime, device ou deploy. | spec Assumptions; constituição VI |
| Project Type | Documental-determinístico: materialização de três JSON de baseline a partir de inventário local; nenhuma alteração de código. | spec FR-001–FR-012, SC-006 |
| Performance Goals | **Não aplicável** — nenhum sistema em execução produz latência ou vazão; o sucesso é medido por contagens (3/3 arquivos, 3/3 PASS, 0 `stale`, 0 chaves órfãs) e por exit code. | spec SC-001–SC-008 |
| Constraints | Schemas e caminhos imutáveis; factory é fonte, não autoridade; campo `_provenance` ignorado pelos loaders; estado corrente é a única autoridade no ato da bootstrap; mapas vazios honesty; remoto/CI/merge/proteção `[NV]`. | spec FR-004, FR-005, FR-009–FR-012; Clarifications |
| Scale/Scope | 3 baselines; 3 scripts lidos; 12 FR; 3 US (P1, P1, P2); 8 SC; 1 entrada legítima em completude, 0 em convergência e 0 em unicidade no estado documentado. | spec FR-001–FR-012, US1–US3, SC-001–SC-008 |
| Formatos extraídos | convergência: `{"schema_version":"087.1.0","enforce_unbaselined":true,"files":{"<rel>":{"sha256":"…","bytes":N}}}`; completude: `{"allow_missing":{"<NNN-slug>":["<artefato>",…]}}` (saída do gate reporta `schema:"047.2.0"`, `strict_from:88`); unicidade: `{"allow_duplicates":{"<NNN-slug>":["duplicate_task:T###",…]}}` (saída reporta `schema:"087.1.0"`). | `check_converge_append_only.py:18,25-32,35-46,59-62`; `check_sdd_completeness.py:24-31,50-77,132-141`; `check_task_id_uniqueness.py:16-17,20-39,43-53,86-94` |
| Versionamento do conjunto | `1.0.0` documental na primeira materialização; schemas dos arquivos são fixados pelos scripts (`schema_version` de converge `087.1.0`; schemas de saída `047.2.0` e `087.1.0`). Evolução e rollback documentais em `contracts/baseline-bootstrap-contract.md §5`. | contrato §5; sdd-phase-contracts.md:27 |
| Meio de verificação | Comandos locais `python3 scripts/ci/check_*.py` com `--bootstrap` (convergência) ou `--json` antes/depois da gravação (completude e unicidade); nenhum serviço externo, nenhuma rede. | quickstart.md; spec FR-001–FR-003 |

Nenhum campo permanece em branco: os campos sem aplicação a esta feature trazem "**Não aplicável**" com o respectivo motivo.

## Constitution Check

*Gate reavaliado após a Phase 1 (modelo de dados, contrato e quickstart), conforme `.specify/memory/constitution.md` e o contrato de fase. A reposição abaixo cobre integralmente os princípios I–VI.*

| Princípio | Exigência | Avaliação pré-projeto | Reavaliação pós-projeto | Resultado |
|---|---|---|---|---|
| I — Determinismo primeiro | Trabalho computável por script/gate/template deve sê-lo; saída de LLM nunca roteia estado nem finaliza validação. | Os três baselines são derivados por comandos determinísticos dos próprios scripts; nenhuma contagem é inventada (SC-002, FR-001–FR-003). | O contrato fixa as transformações `regressions → allow_missing/allow_duplicates` e a saída literal de `--bootstrap`; o quickstart lista comandos e saídas esperadas; nenhum verde é afirmado sem execução (FR-012). | **Atendido** |
| II — Adaptadores e portas | Nenhum código de domínio chama diretamente serviço concreto; integrações por trait/interface e adaptador. | Não há código de domínio, SDK ou serviço; os gates são adaptadores determinísticos locais já existentes. | Nenhum módulo ou chamada é introduzido; a restrição permanece registrada como limite, não como artefato novo. | **Atendido por não aplicabilidade declarada** — motivo: a feature materializa JSON e não produz código de integração. |
| III — Fluxo SDD ordenado | Ordem integral das portarias; R1=`go`; R4 precede R7; `converge.md` append-only. | intake→research→define→shape→decide→specify→clarify concluídos; R1=`go` (decision.md:5). | Plan é a fase 8; checklist (9), tasks (11) e as portarias posteriores permanecem seguintes. Nenhum `converge.md` é criado, reescrito ou re-baselinado nesta feature — o baseline de convergência registra os existentes, sem alterá-los. | **Atendido** |
| IV — Soberania do jogador | Regras e conteúdo inspecionáveis, modificáveis e redistribuííveis; sem dependência proprietária ou serviço externo no caminho crítico. | Baselines locais, legíveis e versionados; nenhuma dependência externa; factory é fonte, não autoridade. | Os três JSON ficam no repositório do alvo, com proveniência local; nenhuma chave externa é incorporada (FR-005, SC-005). | **Atendido** |
| V — Materialismo e ação coletiva | Sem magia ou campeão solitário; habilidades são ações materiais e vitória exige coordenação. | A feature não define mecânicas, habilidades ou regras de jogo. | Nenhum conteúdo de jogo é introduzido; a materialização é ato de governança registrado, não automação de decisão de produto. | **Atendido por não aplicabilidade declarada** — motivo: o objeto é integridade documental, não design de jogo. |
| VI — Verificação honesta | Nada é verde sem comando executado e saída colada; remoto/CI/merge/proteções `[NV]`; divergências registradas. | Toda evidência é local; contagens históricas divergentes são registradas e subordinadas ao estado corrente; remoto `[NV]` (FR-012). | O quickstart descreve comandos e saídas esperadas sem afirmar execução nesta fase; o contrato exige saída colada com data/hora na portaria de evidência; nenhum CI, merge ou aprovação é afirmado. | **Atendido** |

**Complexity Tracking**: nenhuma violação de constituição foi identificada; a tabela de exceções é **não aplicável** nesta feature, porque o desenho apenas deriva JSON dos scripts existentes, sem introduzir módulo, adaptador, dependência, gate novo ou exceção de arquitetura que exigisse justificativa.

## Project Structure

### Documentation (this feature)

```text
specs/003-baseline-governanca/
├── spec.md                                  # origem autoritativa (fases specify/clarify)
├── plan.md                                  # este arquivo (fase plan)
├── research.md                              # decisões de pesquisa — F0
├── data-model.md                            # entidades dos três baselines — F1
├── contracts/
│   └── baseline-bootstrap-contract.md       # formatos, comandos, validações e rollback — F1
├── quickstart.md                            # cenários executáveis de bootstrap e verificação — F1
├── checklists/
│   ├── requirements.md                      # revalidado na clarify (18/18); não alterado nesta fase
│   └── baseline-quality.md                  # checklist de qualidade dos requisitos (fase 9)
└── tasks.md                                 # saída da fase 11 — não criada por /speckit.plan
```

### Artefatos materializados pela feature (repository root)

```text
docs/governance/sdd-completeness-baseline.json
docs/governance/converge-append-only-baseline.json
docs/governance/task-id-uniqueness-baseline.json
```

**Não aplicável**: nenhum diretório de código, engine, teste, build, migração, telemetria, banco ou CI é criado. Os três scripts em `scripts/ci/` são somente lidos e executados; nenhum é alterado (spec FR-011). O diretório `.specify/assessments/baseline-governanca/` é lido como evidência de origem e não é modificado.

**Structure Decision**: adotar apenas a árvore documental acima mais os três JSON em `docs/governance/`. Contrato, modelo de dados e quickstart são artefatos de desenho; a materialização dos baselines pertence à fase de implementação e não é antecipada neste plano.

## Technical Decisions

| ID | Decisão | Rationale | Origem |
|---|---|---|---|
| D1 | Convergência usa o gerador nativo `--bootstrap`; completude e unicidade derivam o baseline do mapa `regressions` do `--json` sem baseline. | Somente `check_converge_append_only.py` expõe `--bootstrap` (`rg` no código); os outros dois emitem `regressions` no resumo, que com baseline vazio equivale ao inventário completo. | research.md RQ-1–RQ-4; spec FR-001–FR-003 |
| D2 | Os caminhos padrão de `--baseline` são os destinos canônicos dos três JSON. | Evita divergência entre onde o gate lê e onde a feature grava. | `DEFAULT_BASELINE` nos três scripts; spec FR-001–FR-003 |
| D3 | Proveniência local é acrescentada como campo `_provenance`, ignorado pelos carregadores. | `load_baseline` lê apenas chaves nomeadas; nenhum script precisa mudar (FR-011). | research.md RQ-6; spec FR-004 |
| D4 | Estado produzido pelos scripts no ato da bootstrap prevalece sobre contagens históricas. | `spec_dirs=0/1/2` diverge em registros datados; a verdade é determinística e local. | Clarifications Q1; research.md RQ-5 |
| D5 | Gap legítimo = portaria não executada ou dívida datada; defeito = artefato de fase concluída, chave órfã ou da factory. | Define o critério que evita transformar dívida em anuência e evita baselinar erro. | Clarifications Q3; research.md RQ-8 |
| D6 | Mapas vazios são honestos e suficientes quando o estado corrente não tem entradas. | Inventar entradas inflaria `baselined` e reduziria a sensibilidade do gate. | Clarifications Q2; research.md RQ-7 |
| D7 | Schemas, limiares, regex e caminhos dos três scripts permanecem intactos. | SC-006 exige zero alterações; o formato já é suportado. | spec FR-011, SC-006; research.md RQ-9 |
| D8 | `enforce_unbaselined=true` é preservado no baseline de convergência. | É o mecanismo que recusa `converge.md` novo não baselinado. | `check_converge_append_only.py:45,59-62,100-103`; spec FR-001 |
| D9 | Ratchet ativo: entrada satisfeita ou feature removida vira `stale` e exige re-baselinação explícita. | Distingue não-regressão de baseline obsoleto e impede que dívida corrigida continue aceita. | lógica de `stale` nos três scripts; spec FR-005–FR-008 |
| D10 | Evidência com data/hora colada na portaria de evidência; remoto/CI/merge/proteção `[NV]`. | Constituição VI e contrato de fase exigem artefato, não comando. | spec FR-012, SC-007; sdd-phase-contracts.md:26 |

## Phase 0 and Phase 1 Outputs

| Artefato | Fase | Conteúdo |
|---|---|---|
| [research.md](./research.md) | F0 — Outline & Research | Dez decisões com decisão, justificativa, alternativas e fontes exclusivamente locais; formatos extraídos dos scripts; evidência contrária tratada. |
| [data-model.md](./data-model.md) | F1 — Design | Entidades Baseline de convergência, Baseline de completude, Baseline de unicidade, Entrada do estado corrente, Proveniência local e Evidência de gate, com campos, validações e transições. |
| [contracts/baseline-bootstrap-contract.md](./contracts/baseline-bootstrap-contract.md) | F1 — Contracts | Formatos exatos, procedimentos de bootstrap, regras de validação, ratchet/`stale`, cenários de regressão, SemVer documental e rollback. |
| [quickstart.md](./quickstart.md) | F1 — Validation guide | Pré-requisitos, comandos de bootstrap, saídas esperadas e cenários de verificação para os três gates. |

## Readiness Gate

- Spec esclarecida: `## Clarifications / ### Session 2026-11-19` com três respostas (uma por vez, R3 respeitado); zero marcadores pendentes; `checklists/requirements.md` em 18/18 após a clarify — sem alteração nesta fase.
- Constitution Check I–VI reavaliado após o desenho: sem violações e sem exceções a justificar.
- Este plano não cria, antecipa ou modifica artefatos de fases posteriores; a existência futura de `checklists/baseline-quality.md` e de `tasks.md` decorre das portarias 9 e 11, não deste documento.
- Próxima portaria: fase 9 (checklist), com `checklists/baseline-quality.md` append-only e itens `- [ ] CHK###`, sem marcação `[x]` pelo gerador.
- Nenhum gate foi executado nesta fase; nenhuma saída é afirmada como obtida. Estado remoto, CI, merge e proteção de branch permanecem `[NV]` até confirmação ao vivo.
