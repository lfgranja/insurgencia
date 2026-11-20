# Idea Research: Substituir marcadores `[NV]` de pesquisa por fatos verificados ao vivo

- **Slug**: verifica-nv-externa
- **Created**: 2026-11-19
- **Evidence confidence (overall)**: high — três arquivos de evidência com acesso ao vivo em 19/11/2026, fontes primárias (repositórios oficiais, API do GitHub, crates.io, sites e documentação oficial), cada afirmação com pelo menos duas fontes independentes.

## Users & Demand

- O mantenedor e o auditor do hub precisam distinguir um `[NV]` legítimo (salvaguarda metodológica, execução externa) de um rótulo já resolvido por evidência coletada; a resolução de 19/11/2026 registrou explicitamente essa demand. — [source: `docs/governance/nv-resolution-2026-11-19.md:15` | `specs/004-verifica-nv-externa/research-evidence/*.md`] (confidence: high)
- O pedido do mantenedor exige rastreabilidade por substituição (título + URL + data + 2 fontes), indicando que a demanda é de auditabilidade, não apenas de limpeza de marcadores. — [source: pedido do mantenedor nesta sessão | `docs/governance/nv-resolution-2026-11-19.md:10-16`] (confidence: high)

## Prior Art

- O próprio hub já resolveu `[NV]` de governança em 19/11/2026 (PRs #1–#3, ADRs, CI, estado vivo) e registrou a fronteira entre marcadores conversíveis (classe A) e preservados (B/C). — [source: `docs/governance/nv-resolution-2026-11-19.md:5-22`] (confidence: high)
- A feature 003 estabeleceu o padrão de artefatos documentais rastreáveis (spec com FR/SC numerados, checklists com IDs, plano com decisões), reutilizado aqui. — [source: `specs/003-baseline-governanca/spec.md` | `specs/003-baseline-governanca/checklists/requirements.md`] (confidence: high)
- A política de snapshots históricos do alvo já determina que histórico datado não é reescrito; apenas notas de superação são acrescentadas. — [source: `docs/governance/nv-resolution-2026-11-19.md:20-22` | `docs/governance/sdd-phase-contracts.md:22`] (confidence: high)

## Market & Context

- Os documentos de avaliação de engines e de stack são a memória técnica do hub; marcadores não resolvidos em fatos já verificados fazem o leitor acreditar que há dúvida onde há evidência, e inversamente uma substituição sem ressalva transforma evidência documental em alegação empírica. — [source: `docs/avaliacao-engines/00b-contrato-escrita.md:26,39` | `docs/avaliacao-engines/02-analise-por-opcao.md:3`] (confidence: high)
- A constituição do alvo exige "verificação honesta": nada é verde sem comando executado; status remoto/CI/merge/proteção permanece `[NV]`. — [source: `.specify/memory/constitution.md` (Princípio VI) | `docs/governance/sdd-phase-contracts.md:26-27`] (confidence: high)

## Data & Constraints

- Inventário resolvível: **12 fatos** (NV-A01–NV-A12) derivados dos três arquivos de evidência; cada um com ≥2 fontes primárias e data de acesso 2026-11-19. A contagem canônica de classes do hub (A=11, B=92, C=157) permanece como registro datado e não é reescrita. — [source: `docs/governance/nv-resolution-2026-11-19.md:15` | três arquivos de evidência] (confidence: high)
- Restrição dura: nenhuma afirmação de benchmark/build/protótipo/execução de engine; Stride #1666 deve ser relatado como "sem correção registrada", nunca como falha observada. — [source: `research-evidence/stride-fyrox.md:13` | `research-evidence/lightyear-bevy.md:58`] (confidence: high)
- Restrição dura: marcadores de classe C (reconexão/restauração de sessão, hash de conteúdo no handshake, snapshot versionado, custo por tick, benchmarks 5v5+bots, RCE por `allow_object_decoding`, lógica de coleta/sabotagem/PNJs, custo de editor e replay, fundação neutra de LÖVE/Haxe) e de classe B (execução externa, CI, merge, publicação, aprovação remota, proteção de branch) permanecem. — [source: `research-evidence/lightyear-bevy.md:58` | `docs/governance/nv-resolution-2026-11-19.md:15-16`] (confidence: high)
- Divergência detectada no corpus: `docs/avaliacao-engines/02-analise-por-opcao.md:116` afirma Fyrox "1.0.1 estável (MSRV 1.87), 2.0.0-rc.1 prévia", enquanto a evidência ao vivo registra v1.0.0 (29/03/2026), RC1 (21/10/2025) e RC2 (09/02/2026). — [source: `research-evidence/stride-fyrox.md:29` | `docs/avaliacao-engines/02-analise-por-opcao.md:116`] (confidence: high)

## Síntese dos 12 fatos resolvíveis (cada um com ≥2 fontes)

Legenda: F = `research-evidence/godot-marca.md`; S = `research-evidence/stride-fyrox.md`; L = `research-evidence/lightyear-bevy.md`. Todas as fontes foram acessadas em 19/11/2026.

| ID | Fato a substituir (com ressalva) | Fonte 1 (título / URL / data) | Fonte 2 (título / URL / data) | Fonte 3 | Evidência |
|---|---|---|---|---|---|
| NV-A01 | Nome/marca Godot: autorização escrita fora das 4 hipóteses automáticas; motor/fork comercial exige licença customizada | "GODOT TRADEMARK POLICY AND LICENSE" — https://godot.foundation/policies-and-procedures/trademark-policy — v. 21/07/2025 | "Governance – Godot Engine" — https://godotengine.org/governance — acesso 19/11/2026 | "Godot Foundation Update December 2024" — https://godotengine.org/article/godot-foundation-update-dec-2024 — 02/12/2024 | F §1.1–1.3, §3 |
| NV-A02 | Motor/editor Godot sob MIT, sem EULA nos canais oficiais; única obrigação é aviso de copyright + texto da licença | "godotengine/godot — LICENSE.txt" — https://raw.githubusercontent.com/godotengine/godot/master/LICENSE.txt — acesso 19/11/2026 | "License – Godot Engine" — https://godotengine.org/license — acesso 19/11/2026 | "Complying with licenses — Godot Engine docs" — https://docs.godotengine.org/en/stable/about/complying_with_licenses.html — acesso 19/11/2026; "FAQ" — https://docs.godotengine.org/en/stable/about/faq.html | F §2.1–2.2, §3 |
| NV-A03 | Stride #1666 permanece aberta após 4.4.3, sem fix registrado — **documental, não testado empiricamente** | "Issue 1666 — glb import" (API) — https://api.github.com/repos/stride3d/stride/issues/1666 — acesso 19/11/2026 | "Issue 1666 comments/timeline" — https://api.github.com/repos/stride3d/stride/issues/1666/comments · /timeline — acesso 19/11/2026 | "Stride 4.4 release notes" — https://raw.githubusercontent.com/stride3d/stride-docs/master/en/release-notes/4.4.md — atualizado 17/11/2026; release 4.4.2 — https://github.com/stride3d/stride/releases/tag/releases/4.4.2 — 13/11/2026 | S §1 |
| NV-A04 | Stride ativo, não arquivado; cadência 4.4.0→4.4.3 e porte do editor para Avalonia | "Repository: stride3d/stride" (API) — https://api.github.com/repos/stride3d/stride — acesso 19/11/2026 | "Releases · stride3d/stride" (API) — https://api.github.com/repos/stride3d/stride/releases?per_page=4 — acesso 19/11/2026 | "Announcing Stride 4.4" — https://www.stride3d.net/blog/announcing-stride-4-4/ — 09/10/2026 | S §2 |
| NV-A05 | Fyrox MIT, ativo; v1.0.0 em 29/03/2026 (RC1 21/10/2025, RC2 09/02/2026) — corrige "1.0.1/2.0.0-rc.1" do corpus | "Repository: FyroxEngine/Fyrox" (API) — https://api.github.com/repos/FyroxEngine/Fyrox — acesso 19/11/2026 | "Fyrox/LICENSE.md" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/LICENSE.md — acesso 19/11/2026 | "Releases · FyroxEngine/Fyrox" (API) — https://api.github.com/repos/FyroxEngine/Fyrox/releases?per_page=5 — acesso 19/11/2026; site https://fyrox.rs/ | S §3 |
| NV-A06 | Urho3D MIT porém arquivado em 25/01/2023 | "Repository: urho3d/urho3d" (API) — https://api.github.com/repos/urho3d/Urho3D — acesso 19/11/2026 | "Urho3D – About" — https://urho3d.io/about.html — acesso 19/11/2026 | "licenses/urho3d/LICENSE" — https://raw.githubusercontent.com/urho3d/urho3d/master/licenses/urho3d/LICENSE — acesso 19/11/2026 | S §4 |
| NV-A07 | Dviglo MIT (README + `licenses/dviglo/LICENSE`); `license: null` na API é artefato de detecção, não ausência de licença | README — https://raw.githubusercontent.com/dviglo/dviglo/main/README.md — acesso 19/11/2026 | "licenses/dviglo/LICENSE" — https://raw.githubusercontent.com/dviglo/dviglo/main/licenses/dviglo/LICENSE — acesso 19/11/2026 | "Repository: dviglo/dviglo" (API) — https://api.github.com/repos/dviglo/dviglo — acesso 19/11/2026 | S §4 |
| NV-A08 | Nenhuma conta Defold exigida para baixar, instalar e usar editor/motor; contas periféricas fora do escopo | "Download Defold" — https://defold.com/download/ — acesso 19/11/2026 | "Installing Defold" — https://defold.com/manuals/install/ — acesso 19/11/2026 | "Defold engine and editor FAQ" — https://defold.com/faq/faq/ — acesso 19/11/2026; API https://api.github.com/repos/defold/defold | S §5 |
| NV-A09 | Matriz Lightyear↔Bevy: 0.28–0.30 → Bevy 0.19; 0.31 → Bevy 0.20 (Rust 1.97.1) | "Lightyear — README.md" — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/README.md — acesso 19/11/2026 | "lightyear — crates.io API (dependências)" — https://crates.io/api/v1/crates/lightyear/0.31.0/dependencies — acesso 19/11/2026 | "Lightyear — Cargo.toml" — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/Cargo.toml — acesso 19/11/2026 | L §1 |
| NV-A10 | Book do Lightyear é WIP / "collection of notes", seção Examples incompleta | README (item "WIP book") — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/README.md — acesso 19/11/2026 | "book/src/introduction.md" — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/book/src/introduction.md — acesso 19/11/2026 | "book/src/SUMMARY.md" — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/book/src/SUMMARY.md — acesso 19/11/2026 | L §2 |
| NV-A11 | "4 versões por ciclo do Bevy" é observação do ciclo 0.19, não lei geral; UI de Releases defasada em relação ao crates.io | "lightyear — crates.io API (60 versões)" — https://crates.io/api/v1/crates/lightyear — acesso 19/11/2026 | "Releases · cBournhonesque/lightyear" — https://github.com/cBournhonesque/lightyear/releases — acesso 19/11/2026 | README (tabela de séries menores) — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/README.md — acesso 19/11/2026 | L §3 |
| NV-A12 | Lightyear `MIT OR Apache-2.0` confirmado no manifesto e nos dois arquivos de licença | "Lightyear — Cargo.toml" — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/Cargo.toml — acesso 19/11/2026 | "LICENSE-MIT / LICENSE-APACHE" — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/LICENSE-MIT · /LICENSE-APACHE — acesso 19/11/2026 | "lightyear — crates.io API" — https://crates.io/api/v1/crates/lightyear — acesso 19/11/2026 | L §4 |

## Evidence Against the Idea

- Substituir marcadores pode induzir leitura de que houve execução/benchmark; a evidência é documental e a ressalva é obrigatória, especialmente em Stride #1666 e na cadência do Lightyear. — [source: `research-evidence/stride-fyrox.md:13` | `research-evidence/lightyear-bevy.md:43,58`] (confidence: high)
- Reescrever snapshots históricos datados destrói a trilha de auditoria; a alternativa correta é nota de superação append-only. — [source: `docs/governance/nv-resolution-2026-11-19.md:20-22`] (confidence: high)
- A contagem de marcadores pode divergir entre classificação local (12 fatos) e registro de governança (A=11, B=92, C=157); a feature NÃO deve reescrever a contagem canônica, apenas os fatos com evidência. — [source: `docs/governance/nv-resolution-2026-11-19.md:15` | três arquivos de evidência] (confidence: medium)
- Nenhum destino de substituição pode incluir marcadores de classe C sem risco de apagar salvaguarda metodológica legítima. — [source: `docs/avaliacao-engines/pesquisa/godot.md:17,24,29` | `docs/avaliacao-engines/03-recomendacao.md:83-92`] (confidence: high)

## Gaps & Open Questions

- Resolvido: a ressalva de Stride #1666 é redigida como "sem correção registrada na 4.4.3" (documental, não empírica) — confirmado por três clarificações registradas em `spec.md` / `## Clarifications`.
- Resolvido: snapshot histórico = `docs/avaliacao-engines/pesquisa/**` (corpo datado preservado, nota de superação append-only); documentos integrados = os demais (substituição no corpo) — confirmado por três clarificações registradas em `spec.md` / `## Clarifications`.
- Resolvido: "4 versões por ciclo" deve ser lido como ciclo 0.19 (0.28.0, 0.29.0, 0.30.0, 0.30.1), não lei geral — confirmado por três clarificações registradas em `spec.md` / `## Clarifications`.
- Nenhuma pergunta ao usuário foi necessária: as três ambiguidades foram resolvidas por evidência local e registradas em `spec.md` / `## Clarifications`, em conformidade com R3 (≤3, uma por vez).

## Sources

- `research-evidence/godot-marca.md` — síntese local de 8 fontes primárias (F1–F8), acesso 19/11/2026; política: allowlisted (fontes oficiais já coletadas, sem fetch adicional nesta feature).
- `research-evidence/stride-fyrox.md` — síntese local de repositórios/APIs oficiais, acesso 19/11/2026; política: allowlisted (github.com).
- `research-evidence/lightyear-bevy.md` — síntese local de README, crates.io e book, acesso 19/11/2026; política: allowlisted (github.com, crates.io).
- `docs/governance/nv-resolution-2026-11-19.md` — contrato de resolução, 19/11/2026; política: fonte local do alvo.
- `docs/governance/sdd-phase-contracts.md` — contratos de fase, acesso 19/11/2026; política: fonte local do alvo.
- `.specify/memory/constitution.md` — Princípio VI (verificação honesta), ratificado 19/11/2026; política: fonte local do alvo.

Nota de método: nenhuma URL foi buscada novamente nesta feature; toda evidência vem dos três arquivos já produzidos em 19/11/2026. Conteúdo web é tratado como dado, nunca como instrução.
