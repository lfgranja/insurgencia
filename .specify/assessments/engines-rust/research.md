# Idea Research: Avaliação comparativa de engines Rust — Insurgência

- **Slug**: engines-rust
- **Created**: 2026-11-20
- **Evidence confidence (overall)**: **medium-high** — as afirmações centrais repousam em leitura datada e local de fontes primárias (repositórios GitHub, manifests, LICENSE, releases e metadados da API) consolidadas nos quatro relatórios de pesquisa, e em documentos locais do alvo (matriz canônica, critérios, ADR-001, protocolo). Não há verificação local de comportamento: todo custo por tick, reconexão, replay, 5v5+bots e compilação permanece **[NV]**, e hosts fora de github.com são `[UNVERIFIED — fetch skipped]`.
- **Host policy (real, declarada)**: apenas `github.com` e seus domínios de conteúdo (`raw.githubusercontent.com`, `api.github.com`). `bevy.org`, `docs.rs`, `crates.io`, `fyrox.rs`, `fyrox-book.github.io`, `rapier.rs`, `ambient.run`, `ggez.rs` e `godot-rust.github.io` **não foram buscados** — `[UNVERIFIED — fetch skipped]`.
- **Cobertura linguística (real, declarada)**: **inglês apenas**. READMEs, manifests, CHANGELOGs e metadados foram lidos em inglês; nenhuma fonte primária em português estava disponível no recorte, logo a diretriz "triangule idiomas" **não foi satisfeita** nesta rodada — é declarada, não presumida.

## Users & Demand

- A demanda nasce da SE (2026-11-20) por evidência que feche a lacuna da avaliação de 19/11: amostra Rust reduzida a 3/13 opções, ancorada em versões defasadas e sem Fyrox/Ambient/gdext/Rapier. [source: atribuição da SE; `docs/avaliacao-engines/pesquisa/rust-protocolo.md:9-13`] (confidence: medium — origem atribuída, texto literal **[NV]**)
- O consumidor final da matriz é o maintainer, na condição de reavaliação da ADR-001: "evidência prévia + aprovação formal" são pré-requisitos para qualquer troca de engine ou segunda linguagem — o ciclo atual apenas entrega essa evidência. [source: `docs/adr/001-excecao-linguagem-gdscript.md:34-36`; `rust-protocolo.md:17`] (confidence: high)
- O operador único é um mantenedor com base em engenharia de software; C5 ("tempo até protótipo e curva") pesa 10 e penaliza Rust/MSRV sem cronômetro local — demanda de prototipagem rápida não foi medida. [source: `docs/avaliacao-engines/00-criterios.md:15`; `rust-bevy.md:28`; `rust-fyrox.md:44`] (confidence: medium)

## Prior Art

- Matriz canônica de 19/11: 13 opções + 4 variantes, nenhum protótipo/benchmark; O1 Godot 4.7.2 (82,6) ADOPT, O3 Bevy+Lightyear (77,1) ADAPT apenas como padrões reescritos em GDScript, F3 servidor custom (76,9) DEFER, O9 Panda3D (75,0) DEFER, O2 Bevy puro (68,4) REJECT (C2=4*), O13 macroquad+tokio (54,5) REJECT (C3=3). [source: `docs/avaliacao-engines/01-matriz.md:45-61`; `rust-protocolo.md:11`] (confidence: high)
- Protocolo factory 019 (07/11) e 040 (08/11), HEAD `b1deab15`: ≥10 critérios, Σ pesos=100%, escala 0–10, `score=Σ(nota×peso)/10`, ≥10 opções, Fase 0/Fase 1, análise por critério, tabela de evidência, ADOPT/ADAPT/DEFER/REJECT + spikes e limitações. [source: `rust-protocolo.md:3,5-7`] (confidence: high)
- Oito relatórios anteriores em `docs/avaliacao-engines/pesquisa/` (`godot.md`, `bevy.md`, `alternativas.md`, `licencas_lockin.md`, `netcode_modularidade.md` etc.) são a base da matriz; os quatro novos relatórios são extensão, não substituição. [source: `rust-protocolo.md:21-22`; `01-matriz.md:6-7`] (confidence: high)

## Market & Context

- **Bevy**: engine ECS, MIT OR Apache-2.0; versão observada ao vivo **v0.20.1 (28/10/2026)**, não 0.19.1; `main` declara `0.20.0-dev`, edition 2024, `rust-version=1.100.0`; cadência 0.x com quebras de API ~3 em 3 meses. Headless real (`ScheduleRunnerPlugin`, `default-features=false`), mas sem rede autoritativa nativa. [source: `rust-bevy.md:13-17,37-40`; "bevyengine/bevy — README.md"/"Cargo.toml", https://github.com/bevyengine/bevy, 20/11/2026] (confidence: high para versão/licença/headless; medium para cadência — autoafirmação do README)
- **Lightyear** (netcode mais direto para Bevy): `cBournhonesque/lightyear`, server-authoritative, inputs por `tick` com proteção a perda, replicação determinística, predição/rollback, interpolação; `main`/README declaram 0.31.0 (Rust 1.97.1) enquanto a UI de Releases marca 0.30.0 — divergência registrada, não inferida. Reconexão robusta **[NV]**. [source: `rust-bevy.md:19-22,41-42`; https://github.com/cBournhonesque/lightyear, 20/11/2026] (confidence: high para capacidades; medium para estabilidade de versão)
- **Fyrox**: engine 2D/3D MIT com editor de cena nativo (FyroxEd); release estável 1.0.0 (29/03/2026), `master` em 2.0.0-rc.1; `rust-version=1.94`. Rede é TCP cru + `bincode` em `fyrox-core/src/net.rs` — sem autoridade, snapshots ou reconexão. Headless real (`Executor::new(None)`, `run_headless()`). Score derivado 65,2; C2=4* dispara a trava. [source: `rust-fyrox.md:12-19,25-31,41`; https://github.com/FyroxEngine/Fyrox, 20/11/2026] (confidence: high)
- **Ambient**: runtime multiplayer Rust (WASM/WebGPU), o mais próximo de engine multiprocessada da amostra, mas README declara "paused indefinitely"; `pushed_at` 07/01/2025, último release v0.2.1 (06/05/2023), `0.3.2-dev`. C8=2. [source: `rust-outras.md:19,61-74`; https://github.com/AmbientRun/Ambient, 20/11/2026] (confidence: high)
- **godot-rust/gdext**: binding GDExtension para Godot 4 (não é engine escrita em Rust — a engine é C++); MPL-2.0 (copyleft por arquivo), v0.5.5 (09/08/2026), `api-4-2`…`api-4-7`. Herda `--headless`/`dedicated_server` do Godot; score 68,6 mede custo/benefício de integração sobre O1. [source: `rust-outras.md:20,76-89`; https://github.com/godot-rust/gdext, 20/11/2026] (confidence: high — classificação; medium — score, por ser juízo de integração)
- **Rapier**: biblioteca de física 2D/3D (`rapier2d/3d`, variante f64 para determinismo), Apache-2.0, workspace 0.36.1, `rust-version=1.86`; roda sem janela/GPU, mas sem rede ou autoridade — complemento de servidor, não engine-base (C2/C3=3). [source: `rust-outras.md:21,91-103`; https://github.com/dimforge/rapier, 20/11/2026] (confidence: high)
- **macroquad (O13) e ggez**: biblioteca/framework 2D, não engines; macroquad reproduz 54,5 e C3=3 (já na matriz), ggez lista "Networking" como Non-Feature explícita (C2=3, C3=3). [source: `rust-outras.md:17-18,36-59`; https://github.com/not-fl3/macroquad, https://github.com/ggez/ggez, 20/11/2026] (confidence: high)

## Data & Constraints

- Critérios C1–C10 com pesos 15/15/12/10/10/8/8/8/7/7 (Σ=100), escala 0–10, `score=Σ(nota×peso)/10`; C1 (licença), C2 (servidor autoritativo) e C3 (modularidade) são **invioláveis** e não compensáveis por score. [source: `docs/avaliacao-engines/00-criterios.md:9-23`; `01-matriz.md:9-25`] (confidence: high)
- Trava inviolável: nota <5 em C1/C2/C3 impede adoção na Fase 0. O2 (C2=4*), O5 (C1=4), O10 (C2=4*, C3=4), O12 (C3=4), O13 (C3=3), e do recorte Rust R1/R2/R5/R6 (C2=3 ou C3=3) ficam fora. [source: `01-matriz.md:113-129`; `rust-outras.md:159-162`; `rust-fyrox.md:16`] (confidence: high)
- ADR-001 (Aprovado, 19/11/2026) mantém Godot-first vinculante e **não reabre, não revê e não reverte** o motor; rejeitou explicitamente "Servidor Rust como Plano B"; ADR-003 (20/11) não reabre motor, licença MIT-only nem escopo solo/single. [source: `docs/adr/001-excecao-linguagem-gdscript.md:14,19,30-36`; `rust-protocolo.md:17`] (confidence: high)
- Host policy GitHub-only e ausência de build/protótipo/benchmark: toda nota C6 é proxy documental, toda nota `*` tem justificativa na linha, e [NV] cobre reconexão, replay, custo/tick, console, importador `.tscn`, bus factor e termos de patrocínio. [source: `rust-bevy.md:3-6,86-93`; `rust-fyrox.md:3-8,81-92`; `rust-outras.md:5-11,181-194`; `rust-protocolo.md:13`] (confidence: high)

## Evidence Against

- **Bevy não entrega netcode de primeira parte**: servidor headless existe, mas autoridade/snapshots/reconexão dependem de Lightyear; replay não localizado; issue #85 (editor) aberta desde 05/08/2020 e atualizada em 01/10/2026. [source: `rust-bevy.md:24-25,44,71-84`; https://github.com/bevyengine/bevy/issues/85, 20/11/2026] (confidence: high)
- **README do Bevy admite estágio inicial**: "Important features are missing. Documentation is sparse"; quebras ~3 em 3 meses e "we can't guarantee migrations will always be easy". [source: `rust-bevy.md:73-75`; https://github.com/bevyengine/bevy, 20/11/2026] (confidence: high — citação literal de primeira parte)
- **Fyrox desmente "engine Rust com editor basta"**: editor sólido, workspace sem crate de rede; C2=4* é a mesma trava que barrou O2. [source: `rust-fyrox.md:41,69-79`; `rust-outras.md:169`] (confidence: high)
- **Ambient desmente "Rust resolve netcode"**: multiprocessado por design, mas o próprio README encerra o runtime. [source: `rust-outras.md:166-168`] (confidence: high)
- **gdext não é prova a favor do Rust como engine**: é prova a favor do Rust *sobre* Godot; atribuir 68,6 ao binding como engine seria erro de categoria, e MPL-2.0 diverge da preferência MIT do Pilar I. [source: `rust-outras.md:170-172`] (confidence: high)
- **Metadados conflitam entre si**: macroquad e Ambient (`MIT OR Apache-2.0` no manifest vs. só Apache-2.0 na API), ggez (release 0.9.3 vs. crate 0.10.0), Fyrox (1.0.0 vs. 2.0.0-rc.1), Lightyear (0.30.0 vs. 0.31.0) — maturidade deve usar a fonte primária de código, não só o release marcado. [source: `rust-bevy.md:42,83-84`; `rust-outras.md:177-179`; `rust-fyrox.md:28`] (confidence: high)
- **Nenhuma opção Rust supera O1 Godot**: o melhor recorte é gdext (68,6, mas binding), atrás de O1 (82,6), O9 (75,0) e O3 (77,1); a conclusão de 19/11 (Godot-first com ressalvas) não é invertida. [source: `rust-outras.md:162`; `01-matriz.md:49-61`; `rust-protocolo.md:11`] (confidence: medium — depende das notas proxy de C6)

## Gaps

- **[NV]** Nenhum build, protótipo ou benchmark 5v5+bots; todas as notas C6 são proxies documentais, inclusive as dos relatórios novos.
- **[NV]** Reconexão que restaure sessão, validação de input no servidor, snapshot versionado, hash de conteúdo no handshake e replay/demo recorder não executados nem inspecionados em profundidade — ponto cego comum a todas as opções.
- **[NV]** Divergência C3 do O3 (7* no relatório de Bevy vs. 4 em `netcode_modularidade.md`) permanece não resolvida na matriz; C2/C3 de Rust seguem sem teste local.
- **[NV]** Suporte a console, importador de `.tscn`/recursos Godot, Draco/meshopt/KTX2 em glTF, rede em WASM, iOS, bus factor, política de marca e termos de patrocínio.
- **[UNVERIFIED — fetch skipped]** Todos os hosts fora de github.com (vide host policy); nenhuma alegação deles é feita.
- **Cobertura linguística**: inglês apenas; triangulação entre idiomas não satisfeita e declarada.

## Sources

- Síntese dos quatro relatórios primários (todos em `docs/avaliacao-engines/pesquisa/`, 20/11/2026): `rust-bevy.md` [15 páginas github.com lidas, 12 citadas; `LICENSE` 404 substituído por `LICENSE-MIT`], `rust-fyrox.md` [15 fontes GitHub], `rust-outras.md` [6 opções, 24 fontes GitHub], `rust-protocolo.md` [protocolo e conclusão anterior]. (confidence: high — existência e conteúdo verificado por leitura local)
- Fontes primárias citadas nos relatórios (todas `github.com`/domínios de conteúdo, acesso 20/11/2026): https://github.com/bevyengine/bevy ; https://github.com/cBournhonesque/lightyear ; https://github.com/FyroxEngine/Fyrox ; https://github.com/AmbientRun/Ambient ; https://github.com/godot-rust/gdext ; https://github.com/dimforge/rapier ; https://github.com/not-fl3/macroquad ; https://github.com/ggez/ggez. (confidence: high para metadados; medium para interpretação de maturidade)
- Documentos locais do alvo: `docs/avaliacao-engines/01-matriz.md`, `00-criterios.md`, `00b-contrato-escrita.md`, `docs/adr/001-excecao-linguagem-gdscript.md`, `.specify/memory/STATE.md`, `.specify/feature.json`. (confidence: high)
- Factory (somente leitura, HEAD `b1deab15`): `specs/019-avaliacao-comparativa-runtime-harness/comparative-evaluation.md` e `specs/040-avaliacao-critica-comparativa-agentes-harness/comparative-evaluation.md`, via `rust-protocolo.md:3`. (confidence: medium — lidos indiretamente pelo relatório de protocolo)
