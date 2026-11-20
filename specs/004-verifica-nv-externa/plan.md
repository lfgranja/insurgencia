# Implementation Plan: Verifica NV externa

**Feature**: `004-verifica-nv-externa` — substituir marcadores `[NV]` de pesquisa por fatos verificados ao vivo, com ressalvas, nos documentos integrados do hub.
**Input**: [spec.md](spec.md), [research.md](research.md), [decision.md](decision.md) (R1 `go`, opção A), `docs/governance/nv-resolution-2026-11-19.md`, `.specify/memory/constitution.md`.

## Technical Context

| Campo | Valor |
|---|---|
| Tipo de feature | Documental — edição de precisão e rastreabilidade em arquivos Markdown já existentes |
| Stack / linguagem / runtime | Não aplicável; nenhum código, build, protótipo ou benchmark é produzido |
| Fonte de evidência | Três arquivos em `specs/004-verifica-nv-externa/research-evidence/`, verificados ao vivo em 2026-11-19 |
| Unidade de trabalho | 12 fatos NV-A01–NV-A12, cada um com ≥2 fontes primárias (título/URL/data) e uma ressalva de escopo |
| Destinos | `docs/AVALIACAO-COMPARATIVA-ENGINES.md`, `docs/avaliacao-engines/**`, `docs/AVALIACAO-STACK-MDMV.md`, `docs/avaliacao-stack/**` |
| Modalidades de escrita | (1) substituição no corpo em documentos integrados; (2) nota de superação append-only em `docs/avaliacao-engines/pesquisa/**` |
| Fora de escopo | Marcadores de classe B (execução externa) e C (salvaguarda metodológica); governança/ADRs/histórico datado; notas, índices e recomendações |
| Verificação | Contagens documentais (spec §SC-001–SC-010), sem gates, stage, commit ou efeito remoto |
| Artefatos de design | Consolidados neste `plan.md`; `data-model.md`, `contracts/` e `quickstart.md` não se aplicam a uma feature sem código, API, dados persistidos ou comandos executáveis |

**NEEDS CLARIFICATION na context**: nenhum. As três ambiguidades foram resolvidas em `spec.md` / `## Clarifications`, antes deste plano (R3 respeitado).

## Constitution Check

| Princípio / norma | Reavaliação | Status |
|---|---|---|
| III — Fluxo SDD ordenado | intake→research→define→shape→decide(`go`)→specify→clarify→plan→checklist→tasks; nenhuma portaria pulada neste documento | PASS |
| VI — Verificação honesta | Toda substituição cita fonte primária com data; nenhum verde, CI, merge, publicação, aprovação ou proteção afirmado; Stride #1666 permanece documental | PASS |
| I — Determinismo primeiro | Verificação por contagens explícitas e inventário rastreável (FR-013, SC-001–SC-010), não por impressão | PASS |
| IV — Soberania do jogador | Nenhuma dependência proprietária introduzida; defold/Fyrox/Godot tratados por licença real, sem lock-in novo | PASS |
| Norma — commits/branch/PR | Nenhum commit, stage, PR ou efeito remoto nesta feature de planejamento (FR-014) | PASS |
| Contrato de resolução `[NV]` | Classe A aplicada com ressalvas; classes B/C preservadas; histórico datado não reescrito (FR-005–FR-007, FR-010) | PASS |
| Contratos de fase (§checklist/tasks) | Checklists com `- [ ]` e IDs CHK###, ≥80% rastreáveis; tasks com `- [ ] T### [P?] [USn?]` e caminho; sem portarias posteriores a tasks | PASS |

Nenhuma violação injustificada; gates não são rodados nesta feature.

## Decisões de design

- **D1 — Unidade de trabalho**: 12 fatos NV-A01–NV-A12, não "todos os `[NV]`". A contagem local é o inventário operacional; a contagem canônica A=11/B=92/C=157 permanece como registro datado de governança e não é recalculada. Justificativa: `research.md` e `nv-resolution-2026-11-19.md:15`.
- **D2 — Citation-first**: cada substituição carrega título, URL, data de acesso (2026-11-19) e ≥2 fontes no próprio destino; remissões vagas são proibidas. Justificativa: FR-002, SC-002.
- **D3 — Ressalva obrigatória**: nenhuma fonte é citada sem a limitação de escopo do fato; a ressalva é parte do requisito, não adorno. Justificativa: FR-003, SC-008, Princípio VI.
- **D4 — Sem empirismo**: nenhuma frase de benchmark/build/protótipo/falha observada; Stride #1666 é "sem fix registrado, não testado empiricamente". Justificativa: FR-004, SC-003.
- **D5 — Append-only em snapshots**: em `docs/avaliacao-engines/pesquisa/**`, o corpo datado permanece byte-for-byte e a correção vai em nota de superação; em documentos integrados, a substituição é no corpo. Justificativa: FR-007/FR-008, `nv-resolution-2026-11-19.md:20-22`.
- **D6 — Estabilidade de scores**: nenhuma nota, vetor, índice ou recomendação é recalculado; a divergência de versão do Fyrox é correção de fato em `02-analise-por-opcao.md:116`, com nota de que o vetor permanece. Justificativa: FR-009, SC-007.
- **D7 — Preservação A/B/C**: marcadores de classe B e C são imutáveis; a feature não decide sobre CI, merge, publicação, proteção de branch, reconexão, hash ou benchmarks. Justificativa: FR-005/FR-006, SC-004/SC-005.
- **D8 — Escopo de escrita e verificação**: nenhuma escrita fora de `specs/004-verifica-nv-externa/` nesta feature de planejamento; nenhum gate, stage ou commit; a verificação é contagem documental. Justificativa: FR-014, SC-009, pedido do mantenedor.

## Inventory: fatos → fontes → destinos

Todas as fontes foram acessadas em 19/11/2026. Os números de linha são os identificados em `research.md` e devem ser confirmados no ato da edição.

| ID | Fato / ressalva | Fontes (≥2) | Destinos prioritários | Modalidade |
|---|---|---|---|---|
| NV-A01 | Marca Godot: escrita fora das 4 hipóteses automáticas; motor/fork comercial exige licença customizada | Política de marcas v. 21/07/2025 + Governance + artigo 02/12/2024 | `pesquisa/licencas_lockin.md:24`, `pesquisa/godot.md:8-9,33`, `pesquisa/alternativas.md:9,24`, `02-analise-por-opcao.md:11`, `03-recomendacao.md:38,91`, `AVALIACAO-COMPARATIVA-ENGINES.md:241` | nota em `pesquisa/`; substituição nos demais |
| NV-A02 | Godot MIT, sem EULA nos canais oficiais; só aviso de copyright + texto da licença | `LICENSE.txt` + `/license` + "Complying with licenses" + FAQ | `pesquisa/licencas_lockin.md:24`, `pesquisa/godot.md:7,33`, `pesquisa/alternativas.md:9` | nota em `pesquisa/` |
| NV-A03 | Stride #1666 aberta após 4.4.3, sem fix registrado — documental, não empírico | API issue + comments/timeline + release notes 4.4 + release 4.4.2 | `pesquisa/defold_flax_stride.md:52`, `02-analise-por-opcao.md:41`, `01-matriz.md:83` | nota em `pesquisa/`; substituição nos demais |
| NV-A04 | Stride ativo, não arquivado; 4.4.0→4.4.3; porte Avalonia | API repositório + API releases + blog 09/10/2026 | `pesquisa/defold_flax_stride.md:53`, `01-matriz.md:73`, `AVALIACAO-COMPARATIVA-ENGINES.md:148` | nota/substituição conforme diretório |
| NV-A05 | Fyrox MIT, ativo; v1.0.0 (29/03/2026), RC1/RC2 — corrige 1.0.1/2.0.0-rc.1 | API repositório + `LICENSE.md` + API releases + fyrox.rs | `02-analise-por-opcao.md:114-123` | substituição (sem alterar vetor/índice) |
| NV-A06 | Urho3D MIT, arquivado em 25/01/2023 | API repositório + site About + `licenses/urho3d/LICENSE` | `pesquisa/licencas_lockin.md:30`, `pesquisa/libgdx_urho_panda.md:8`, `02-analise-por-opcao.md:45` | nota/substituição conforme diretório |
| NV-A07 | Dviglo MIT; `license: null` é artefato de detecção; último push 09/03/2024 | README + `licenses/dviglo/LICENSE` + API repositório | `pesquisa/licencas_lockin.md:30`, `pesquisa/libgdx_urho_panda.md:8` | nota em `pesquisa/` |
| NV-A08 | Nenhuma conta Defold para download/instalação/uso; contas periféricas fora do escopo | Download + manual de instalação + FAQ + API repositório | `pesquisa/defold_flax_stride.md:11` | nota em `pesquisa/` |
| NV-A09 | Matriz Lightyear↔Bevy 0.28–0.30→0.19; 0.31→0.20; Rust 1.97.1 | README + API dependências crates.io + `Cargo.toml` | `pesquisa/bevy.md:11`, `01-matriz.md:80`, `AVALIACAO-COMPARATIVA-ENGINES.md:155` | nota/substituição conforme diretório |
| NV-A10 | Book Lightyear WIP / "collection of notes"; Examples incompleta | README + `introduction.md` + `SUMMARY.md` | `pesquisa/bevy.md:23`, `01-matriz.md:80`, `AVALIACAO-COMPARATIVA-ENGINES.md:155` | nota/substituição conforme diretório |
| NV-A11 | "4 versões por ciclo" apenas no ciclo 0.19; Releases defasada | API crates.io (60 versões) + Releases + README | `pesquisa/bevy.md:26`, `02-analise-por-opcao.md:45`, `03-recomendacao.md:88`, `01-matriz.md:80`, `AVALIACAO-COMPARATIVA-ENGINES.md:155` | nota/substituição conforme diretório |
| NV-A12 | Lightyear `MIT OR Apache-2.0`; UI do GitHub exibe só Apache por detecção parcial | `Cargo.toml` + `LICENSE-MIT`/`LICENSE-APACHE` + crates.io | `pesquisa/licencas_lockin.md:36`, `pesquisa/bevy.md:19` | nota em `pesquisa/` |

## Templates de escrita

### Citação auditável (documento integrado)

```
<afirmação com ressalva> — Fontes: "<título da fonte 1>" (<URL>), <data do documento>, <data de acesso: 2026-11-19>; "<título da fonte 2>" (<URL>), <data do documento>, <data de acesso: 2026-11-19>. Verificado ao vivo em 2026-11-19; <ressalva de escopo>.
```

Regras do template: (i) título, URL e data para cada fonte; (ii) no mínimo duas fontes independentes; (iii) data de acesso 2026-11-19 distinguida da data de versão/publicação da fonte; (iv) ressalva na mesma frase ou imediatamente após; (v) nenhuma palavra de execução/benchmark.

### Nota de superação (snapshot histórico)

```
> **Nota de superação (2026-11-19):** <fato NV-Axx> — <ressalva>. Fontes: "<título>" (<URL>), <data>; "<título>" (<URL>), <data>. O corpo datado acima permanece como registro histórico.
```

Regras do template: (i) append-only, após o corpo existente; (ii) nenhuma linha anterior alterada, inclusive `Data: 19/11/2026`; (iii) uma nota por arquivo pode agrupar vários fatos, desde que cada fato tenha sua ressalva e suas ≥2 fontes; (iv) a nota não recalcula notas nem índices.

## Matriz arquivo → ação

| Arquivo | Ação | Fatos | Modo |
|---|---|---|---|
| `docs/avaliacao-engines/pesquisa/licencas_lockin.md` | nota de superação | A01, A02, A06, A07, A12 | append |
| `docs/avaliacao-engines/pesquisa/godot.md` | nota de superação | A01, A02 | append |
| `docs/avaliacao-engines/pesquisa/alternativas.md` | nota de superação | A01, A02 | append |
| `docs/avaliacao-engines/pesquisa/defold_flax_stride.md` | nota de superação | A03, A04, A08 | append |
| `docs/avaliacao-engines/pesquisa/libgdx_urho_panda.md` | nota de superação | A06, A07 | append |
| `docs/avaliacao-engines/pesquisa/bevy.md` | nota de superação | A09, A10, A11, A12 | append |
| `docs/avaliacao-engines/02-analise-por-opcao.md` | substituição no corpo | A01, A03, A05, A06, A11 | edição pontual |
| `docs/avaliacao-engines/01-matriz.md` | substituição no corpo | A03, A04, A09, A10, A11 | edição pontual |
| `docs/avaliacao-engines/03-recomendacao.md` | substituição no corpo | A01, A11 | edição pontual |
| `docs/AVALIACAO-COMPARATIVA-ENGINES.md` | substituição no corpo | A01, A04, A09, A10, A11 | edição pontual |
| `docs/AVALIACAO-STACK-MDMV.md` | nenhuma — apenas confirmar preservação de `[NV]` | — | read-only |
| `docs/avaliacao-stack/**` | nenhuma — apenas confirmar preservação de `[NV]` | — | read-only |

## Abordagem de verificação (documental)

1. **Contar fatos aplicados**: percorrer o inventário e confirmar 12 de 12 destinos com citação (SC-001).
2. **Contar fontes**: para cada fato, confirmar ≥2 fontes com título, URL e data (SC-002); 0 fatos abaixo do mínimo.
3. **Caçar empirismo**: buscar termos de benchmark/build/protótipo/"falha observada"/"custo por tick" nas frases novas; meta 0 (SC-003).
4. **Confirmar ressalvas**: 12 de 12 com a limitação de FR-003 no próprio destino (SC-008).
5. **Confirmar preservação**: listar marcadores de classe B/C antes e depois; nenhum removido (SC-004, SC-005).
6. **Confirmar append-only**: comparar o corpo dos snapshots com `git diff` somente para leitura do próprio trabalho, garantindo 0 linhas anteriores alteradas (SC-006). Nenhum stage/commit.
7. **Confirmar estabilidade**: 0 alterações em notas, vetores, índices e recomendações (SC-007).
8. **Confirmar artefatos**: 10 de 10 arquivos presentes, sem seção vazia e sem `TBD`/`[NEEDS CLARIFICATION]` (SC-010).

Nenhum comando de gate, build ou teste é executado nesta feature; a verificação é leitura e contagem. Qualquer divergência encontrada é registrada, não suprimida.
