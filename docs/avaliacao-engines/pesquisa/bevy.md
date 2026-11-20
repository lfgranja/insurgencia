# Bevy 0.19.1 (O2) e Bevy + Lightyear (O3) — pesquisa em fontes primárias

Data: 2026-11-19. `[NV]` = não verificado localmente (sem build/benchmark); inferências marcadas.

## Fontes primárias consultadas

- Licença e toolchain: https://github.com/bevyengine/bevy/blob/v0.19.1/Cargo.toml — `license = "MIT OR Apache-2.0"`, `edition = 2024`, `rust-version = "1.95.0"`; https://bevy.org/learn/quick-start/introduction/ (metas: modular, ECS — *Entity Component System*, arquitetura por entidades/componentes — e "changes should compile quickly").
- Releases: https://github.com/bevyengine/bevy/releases/tag/v0.19.1 (13/08/2026, patch); https://bevy.org/news/bevy-0-19 (19/06/2026; 261 contribuidores, 1185 PRs); https://bevy.org/news/bevy-0-20/ (08/10/2026 — 0.19.1 já é uma versão atrás).
- Servidor: https://github.com/bevyengine/bevy/blob/v0.19.1/examples/app/headless.rs; feature `default_app` descrita no manifesto para "headless apps ... servers, etc".
- Cenas/editor: https://bevy.org/news/bevy-0-19/#next-generation-scenes; https://bevy.org/news/bevys-sixth-birthday (protótipo comunitário Jackdaw; editor oficial "upcoming"); https://github.com/bevyengine/bevy/issues/24637 (issue de editor GUI fechada; issue #85 permanece aberta).
- Rede: https://github.com/cBournhonesque/lightyear (MIT/Apache-2.0; 1227 estrelas; compatibilidade: 0.28–0.30 → Bevy 0.19, 0.31 → 0.20); https://cbournhonesque.github.io/lightyear/book/; https://cbournhonesque.github.io/lightyear/book/concepts/bevy_integration/server.html; https://github.com/simgine/bevy_replicon (MIT/Apache; 663 estrelas; 0.41–0.44 → Bevy 0.19).
- Assets/plataformas: https://docs.rs/bevy/0.19.1/bevy/gltf/index.html; https://bevy.org/news/bevy-webgpu/; https://github.com/bevyengine/bevy/tree/v0.19.1/examples/mobile (Android via `cargo-apk`, iOS via Xcode).
- Comunidade/performance: https://bevy.org/news/bevys-sixth-birthday (47.529 estrelas, 6,6M downloads, 23.359 no Discord, Bevy Metrics; exemplo `bevy_city`: 55.000 entidades a 60 fps).

## Notas por critério

| # | O2 — Bevy 0.19.1 | O3 — Bevy + Lightyear |
|---|---|---|
| C1 | **10** — duplo MIT/Apache-2.0 confirmado, sem EULA/SDK proprietário; fork e redistribuição livres. | **10** — Lightyear e `bevy_replicon` também MIT/Apache-2.0. |
| C2 | **4\*** — headless real (`ScheduleRunnerPlugin`, sem winit/render), mas sem validação de input, snapshots ou reconexão nativos; exige crate externa. | **9\*** — servidor autoritativo, autenticação netcode, inputs tamponados por *tick* (passo de simulação) com tolerância a perda, replicação via `bevy_replicon`, client-server/P2P/*host-client*; nenhum serviço central obrigatório. Sem reconexão robusta (extensão `bevy_replicon_repair` está desmantida). |
| C3 | **7\*** — ECS por composição, plugins e BSN (*Bevy Scene Notation*, notação declarativa de cenas) com patches composáveis; em 0.19 **não** há loader `.bsn` de primeira parte, nem hash/versão de conteúdo no handshake. | **7\*** — mesmos mecanismos; protocolo de replicação registrado por tipo, mas versionamento/hash de módulos ainda cabe à aplicação. |
| C4 | **4\*** — nenhum editor visual oficial; BSN + widgets Feathers e *observers* dão base para lógica declarativa e microssistemas, mas coleta/sabotagem/negociação exigem código. | **7\*** — predição/rollback e interpolação de snapshots cobrem sincronia de 2 jogadores/5 s; ausência de editor permanece. |
| C5 | **6** — documentação oficial extensa e ECS limpo; porém Rust + borrow checker e toolchain recente (MSRV 1.95) pesam para quem vem de engenharia de software. | **5\*** — book do Lightyear é WIP ("collection of notes"); matriz de versões rígida e mais conceitos de rede a dominar. |
| C6 | **8\*** — renderer escala bem (55k entidades/60 fps) e modo headless é enxuto; sem benchmark local 5v5+bots `[NV]`. | **8\*** — benches Criterion no repositório, gestão de largura de banda e interesse; custo real por tick `[NV]`. |
| C7 | **8** — loader glTF 2.0 nativo com rótulos e extras; faltam Draco, meshopt, KTX2/WebP via extensão e `KHR_materials_variants`. | **8** — idêntico (o glTF não passa pela camada de rede). |
| C8 | **8** — 6 anos, fundação sem fins lucrativos, Project Goals transparentes (não roadmap), releases frequentes; churn 0.x e 0.20 já lançado. | **7\*** — projeto ativo mas concentrado; 4 versões do Lightyear por ciclo do Bevy e API de transporte em mudança. |
| C9 | **8** — desktop primeiro; WebGL2/WebGPU via wgpu e wasm (*WebAssembly*); exemplos Android/iOS reais, porém setup móvel exige Xcode/cargo-apk. | **8\*** — mesmo caminho; Lightyear anuncia compatibilidade wasm via WebTransport. |
| C10 | **6** — MIT/Apache e crates desacopláveis reduzem lock-in; contra: árvore grande de dependências, edição 2024, MSRV 1.95 e tempos de compilação (mitigados por `dynamic_linking`). | **6\*** — acrescenta `aeronet`, `postcard`, `bevy_replicon`, Leafwing; acoplamento de versões é o maior risco de atualização. |

## Ressalvas

C1/C3/C10 são favoráveis; o ponto fraco estrutural é C4 (sem editor) e, no O2, C2. C2 e C6 dependem de validação local `[NV]`.

## Nota de superação (2026-11-19, append-only)

> **Nota de superação (2026-11-19) — NV-A09 (matriz Lightyear↔Bevy):** a tabela oficial do README fixa 0.28–0.30 → Bevy 0.19 e 0.31 → Bevy 0.20, com Rust 1.97.1 ou newer para 0.31; trata-se da tabela upstream, não inferência. Fontes: "Lightyear — README.md" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/README.md), acesso 19/11/2026; "lightyear — crates.io API (dependências)" (https://crates.io/api/v1/crates/lightyear/0.31.0/dependencies), acesso 19/11/2026; "Lightyear — Cargo.toml" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/Cargo.toml), acesso 19/11/2026. O corpo datado acima permanece como registro histórico.

> **Nota de superação (2026-11-19) — NV-A10 (book WIP):** o book do Lightyear é WIP e se descreve como "collection of notes and thoughts", não documento polido; a seção Examples do SUMMARY.md contém só o título, sem entradas. Fontes: "Lightyear — README.md" (item "WIP book") (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/README.md), acesso 19/11/2026; "book/src/introduction.md" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/book/src/introduction.md), acesso 19/11/2026; "book/src/SUMMARY.md" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/book/src/SUMMARY.md), acesso 19/11/2026. O corpo datado acima permanece como registro histórico.

> **Nota de superação (2026-11-19) — NV-A11 (cadência de versões):** "4 versões do Lightyear por ciclo do Bevy" é observação do ciclo 0.19 (0.28.0 em 26/06/2026, 0.29.0 em 10/08/2026, 0.30.0 em 13/09/2026, 0.30.1 em 16/09/2026), não lei geral — o total varia de 1 a 13 por ciclo; a tabela do README conta 3 séries menores, enquanto as 4 publicações incluem o patch; a UI de Releases marca 0.30.0 como "Latest" embora o crates.io já registre 0.31.0. Fontes: "lightyear — crates.io API (60 versões)" (https://crates.io/api/v1/crates/lightyear), acesso 19/11/2026; "Releases · cBournhonesque/lightyear" (https://github.com/cBournhonesque/lightyear/releases), acesso 19/11/2026; "README (tabela de séries menores)" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/README.md), acesso 19/11/2026. O corpo datado acima permanece como registro histórico.

> **Nota de superação (2026-11-19) — NV-A12 (dupla licença do Lightyear):** o manifesto e os dois arquivos de licença confirmam `MIT OR Apache-2.0`; a UI do GitHub exibe apenas "Apache-2.0" por detecção automática parcial. Fontes: "Lightyear — Cargo.toml" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/Cargo.toml), acesso 19/11/2026; "LICENSE-MIT / LICENSE-APACHE" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/LICENSE-MIT · https://raw.githubusercontent.com/cBournhonesque/lightyear/main/LICENSE-APACHE), acesso 19/11/2026; "lightyear — crates.io API" (https://crates.io/api/v1/crates/lightyear), acesso 19/11/2026. O corpo datado acima permanece como registro histórico.
