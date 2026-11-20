# Bevy em Rust — pesquisa em fontes primárias (github.com)

Data: 2026-11-20. IDs do contrato: **O2** = Bevy; **O3** = Bevy + Lightyear. Política de hosts
desta rodada: apenas `github.com` (o leitor converte internamente `blob` em `raw.githubusercontent.com`;
as citações registram a URL canônica em `github.com`). `bevy.org`, `docs.rs` e `crates.io` **não foram
buscados** → `[UNVERIFIED — fetch skipped]`. Sem build, protótipo ou benchmark local → `[NV]`.

## Resumo

Bevy é uma engine (motor de jogo) em Rust, livre e de código aberto, com arquitetura ECS
(*Entity Component System* — sistema de entidades e componentes), licença dupla **MIT OR Apache-2.0**
confirmada no manifesto e no README, e cadência de releases 0.x com mudanças que quebram a API
"aproximadamente a cada 3 meses" (advertência do próprio README). A versão **observada ao vivo em
20/11/2026 é v0.20.1** (28/10/2026), não 0.19.1 como fixado no contrato de escrita; a ramificação
`main` declara `0.20.0-dev`, `edition = 2024` e `rust-version = 1.100.0`.

O núcleo oferece **servidor headless** (sem janela/renderização) real via `ScheduleRunnerPlugin` e
`default-features = false`, mas **não** traz rede autoritativa, validação de input, snapshots ou
reconexão: isso é terceirizado. O netcode primeiro-party mais direto no GitHub é **Lightyear**
(`cBournhonesque/lightyear`), que se declara "server-authoritative", roda inputs tamponados por
`tick` (passo fixo de simulação) com proteção contra perda de pacotes, replicação determinística por
inputs, predição com *rollback*, interpolação de snapshots e topologias cliente-servidor/P2P/*host-client*.
Há suporte a passo fixo no núcleo (`Time<Fixed>`, `FixedUpdate`, relógio manual), mas **replay/gravação
de partida não foi encontrado** em github.com. **Não há editor visual oficial**: a issue #85 está
aberta desde 2020-08-05 e foi atualizada em 2026-10-01. Governança ativa é verificável (organização
com 22 repositórios, 49.894 estrelas, 5.084 forks, 625 PRs abertos, repositório atualizado em
20/11/2026), mas entidade neutra/fundação e roadmap não são verificáveis sob a política de hosts.
Curva para 1 dev: documentação e ECS limpos pesam contra Rust/*borrow checker*, MSRV próximo do
estável e churn 0.x. Portabilidade desktop/web/mobile tem evidência no manifesto e em exemplos;
**console é `[NV]`**. Para o Insurgência (Godot-first), o custo de migração é alto: sem editor, sem
GDScript, cenas/assets precisam ser reexpressos em `bevy_scene`/`bevy_gltf` e a rede em Lightyear.

## Evidência

| Tema | Achado e fonte (título, URL, data de acesso) |
|---|---|
| Licença/SPDX | README: "dual-licensed under either MIT License … or Apache License, Version 2.0 … at your option"; `Cargo.toml`: `license = "MIT OR Apache-2.0"`; `LICENSE-MIT` contém o texto MIT integral. Assets de exemplos têm licenças próprias (`CREDITS.md`). ("bevyengine/bevy — README.md", https://github.com/bevyengine/bevy, 20/11/2026; "Cargo.toml", https://github.com/bevyengine/bevy/blob/main/Cargo.toml, 20/11/2026; "LICENSE-MIT", https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT, 20/11/2026). |
| Versão/cadência | v0.20.1 "Latest" em 28/10/2026; v0.20.0 em 08/10/2026; RCs em 15/09 e 28/09; v0.19.1 em 13/08; v0.19.0 em 18/06/2026. README: mudanças quebram a API ~a cada 3 meses, migrações não garantidas. ("Releases · bevyengine/bevy", https://github.com/bevyengine/bevy/releases; "README.md", https://github.com/bevyengine/bevy, 20/11/2026). |
| Linguagem/arquitetura | Rust; metas "Data Focused: Data-oriented architecture using the Entity Component System paradigm" e "Modular: use only what you need"; workspace com `crates/*`. ("README.md", https://github.com/bevyengine/bevy; "Cargo.toml", https://github.com/bevyengine/bevy/blob/main/Cargo.toml, 20/11/2026). |
| Servidor headless (built-in) | `examples/app/headless.rs`: `ScheduleRunnerPlugin::run_loop(Duration 1/60)`, `default-features = false`; feature `default_app` do manifesto cita "headless apps … (ex: command line tools, servers, etc)". ("headless.rs", https://github.com/bevyengine/bevy/blob/v0.20.1/examples/app/headless.rs; "Cargo.toml", https://github.com/bevyengine/bevy/blob/main/Cargo.toml, 20/11/2026). |
| Rede (terceira parte) | Lightyear: "server-authoritative multiplayer games with Bevy. Compatible with wasm via WebTransport"; tabela 0.31→Bevy 0.20, 0.28–0.30→0.19; inputs por `tick` com múltiplos frames por pacote, *deterministic replication* (exige simulação determinística), predição/rollback, interpolação, gerenciamento de banda, lag compensation, P2P/*host-client*. `Cargo.toml`: 0.31.0, Rust 1.97.1, `MIT OR Apache-2.0`, `bevy_replicon 0.45.2`, `postcard`, `aeronet 0.22.0-rc.1`, `chacha20poly1305`. ("lightyear — README.md", https://github.com/cBournhonesque/lightyear; "Cargo.toml", https://github.com/cBournhonesque/lightyear/blob/main/Cargo.toml, 20/11/2026). |
| Cadência do netcode | Releases GitHub: 0.29.0 em 10/08/2026 e 0.30.0 em 14/09/2026; a UI marca 0.30.0 como "Latest" enquanto `main`/README já declaram 0.31.0 — divergência registrada, não inferida. ("Releases · cBournhonesque/lightyear", https://github.com/cBournhonesque/lightyear/releases, 20/11/2026). |
| Determinismo/replay | Núcleo: `Time<Fixed>`, `FixedUpdate` e `TimeUpdateStrategy::{ManualInstant, ManualDuration, FixedTimesteps}`; o código diz que para testes/rede pode-se definir o tempo manualmente. Lightyear cobre determinismo por inputs (lockstep e predição/rollback). Replay/demo recorder: não localizado em github.com `[NV]`. ("bevy_time/src/lib.rs", https://github.com/bevyengine/bevy/blob/main/crates/bevy_time/src/lib.rs; "lightyear — README.md", https://github.com/cBournhonesque/lightyear, 20/11/2026). |
| Editor | Issue #85 "Bevy Editor" **aberta**, criada em 05/08/2020, atualizada em 01/10/2026, 140 comentários; escopo proposto (cenas, hierarquias, extensões, dados ao vivo) segue sem implementação oficial. ("Issue #85 — Bevy Editor", https://github.com/bevyengine/bevy/issues/85, 20/11/2026). |
| Maturidade/governança | Org @bevyengine com 22 repositórios e 1,6k seguidores; repositório `bevy` com 49.894 estrelas, 5.084 forks, 625 PRs, atualizado em 20/11/2026; `CONTRIBUTING.md` e `CODE_OF_CONDUCT.md` presentes. README admite "early stages … documentation is sparse". Fundação/entidade neutra: `[UNVERIFIED — fetch skipped]`. ("bevyengine — GitHub", https://github.com/bevyengine; "README.md", https://github.com/bevyengine/bevy; "CONTRIBUTING.md", https://github.com/bevyengine/bevy/blob/main/CONTRIBUTING.md, 20/11/2026). |
| Curva/toolchain | README: MSRV "geralmente próximo do último estável"; manifesto `rust-version = 1.100.0`; feature `dynamic_linking` melhora compilações iterativas; aviso explícito de que migrações entre versões podem não ser fáceis. ("README.md", https://github.com/bevyengine/bevy; "Cargo.toml", https://github.com/bevyengine/bevy/blob/main/Cargo.toml, 20/11/2026). |
| Portabilidade | `default_platform` inclui `webgl2`, `x11`, `wayland`, `bevy_winit`, `bevy_gilrs`; workspace inclui `examples/mobile`, `.github/start-mobile-example` e `.github/start-wasm-example`; Lightyear declara wasm via WebTransport e mantém `crates/platform/web`. Console sem evidência `[NV]`. ("Cargo.toml", https://github.com/bevyengine/bevy/blob/main/Cargo.toml; "lightyear — README.md", https://github.com/cBournhonesque/lightyear, 20/11/2026). |
| Custo de migração (Godot-first) | **Inferência** a partir das evidências: não há editor, não há GDScript, cenas passam por serialização `bevy_scene`/`bevy_gltf`, e autoridade/rede por Lightyear. Não há evidência em github.com de importador automático de `.tscn` `[NV]`. (Mesmas fontes acima; nenhuma fonte externa inventada). |

## Notas C1–C10

| # (peso) | O2 — Bevy 0.20.x | O3 — Bevy + Lightyear 0.31 | Justificativa |
|---|---:|---:|---|
| C1 (15) | **10** | **10** | Dupla licença MIT/Apache-2.0 confirmada em manifesto, README e arquivo de licença; sem EULA/SDK proprietário. Lightyear também `MIT OR Apache-2.0` em `Cargo.toml`. |
| C2 (15) | **4\*** | **9\*** | O2: headless real, mas sem validação/snapshots/reconexão nativos. O3: autoritativo, netcode, inputs por tick protegidos contra perda, replicação e rollback; reconexão robusta não verificada. |
| C3 (12) | **7\*** | **7\*** | ECS por composição, plugins e features (`default_app`, `scene`); dados declarativos por cenas serializadas. Hash/versão de conteúdo no handshake cabe à aplicação; sem loader de primeira parte para notação de cena. |
| C4 (10) | **4\*** | **7\*** | O2: issue #85 aberta há 6 anos → sem editor; microssistemas por sistemas/plugins, mas coleta/sabotagem/negociação exigem código. O3: sincronia de entrada na janela de 5 s coberta por inputs por tick, predição e interpolação; editor continua ausente. |
| C5 (10) | **6** | **5\*** | O2: docs amplas e ECS limpo, mas Rust + MSRV ~1.100 e churn 0.x com migrações não garantidas. O3: book do Lightyear é WIP e a matriz de versões é rígida (0.31 exige Rust 1.97.1+). |
| C6 (8) | **7\*** | **8\*** | O2: proxy documental (`multi_threaded`, modo headless enxuto); sem benchmark 5v5+bots `[NV]`; o "55 mil entidades/60 fps" de `bevy.md` veio de `bevy.org`, fora da política. O3: suítes Criterion/Divan e banda/lag compensation no repositório; custo real por tick `[NV]`. |
| C7 (8) | **8** | **8** | Features `bevy_gltf`, `ktx2`, `png`, `zstd_rust` no manifesto; extensões avançadas de compressão não verificadas sob esta política `[NV]`. O3 inalterado: assets não atravessam a camada de rede. |
| C8 (8) | **7\*** | **7\*** | O2: manutenção e comunidade ativas, porém release 0.x quebra API e fundação/roadmap não verificáveis → recalibração conservadora de 8 para 7*. O3: projeto ativo, porém concentrado no mantenedor e com API em mudança. |
| C9 (7) | **8** | **8\*** | O2: desktop primeiro, WebGL2/wasm e exemplos móveis reais no repositório; console `[NV]`. O3: WebTransport e crate `platform/web` confirmados. |
| C10 (7) | **6** | **6\*** | O2: licenças permissivas e crates desacopláveis reduzem lock-in; contra: árvore de dependências extensa, edition 2024, MSRV próximo do estável e churn. O3: acrescenta aeronet/postcard/bevy_replicon/Leafwing e acoplamento de versões. |

Scores ponderados pela fórmula `Σ(nota×peso)/10`: **O2 = 66.8**, **O3 = 77.1**. C1–C3 são
invioláveis: O2 tem C2 = 4*, o que impede adoção na Fase 0 mesmo com score próximo; O3 depende de
validação local de reconexão. Diferenças versus `bevy.md`: versão observada 0.19.1 → 0.20.1;
C6(O2) 8* → 7* e C8(O2) 8 → 7* por fontes fora da política de hosts. Notas com `*` têm
justificativa na própria linha.

## Evidência Contra

- O README alerta: "Bevy is still in the early stages of development. Important features are
  missing. Documentation is sparse"; mudanças quebram a API ~3 em 3 meses e "we can't guarantee
  migrations will always be easy" (https://github.com/bevyengine/bevy, 20/11/2026).
- Sem editor oficial: issue #85 aberta desde 2020 e ainda aberta em 01/10/2026
  (https://github.com/bevyengine/bevy/issues/85, 20/11/2026).
- Servidor headless existe, mas rede autoritativa não é built-in; exige Lightyear/terceiros
  (https://github.com/bevyengine/bevy/blob/v0.20.1/examples/app/headless.rs;
  https://github.com/cBournhonesque/lightyear, 20/11/2026).
- Determinismo do Lightyear depende de a simulação ser determinística; replay não localizado
  (https://github.com/cBournhonesque/lightyear, 20/11/2026).
- Divergência de versão do Lightyear entre UI de Releases (0.30.0) e `main`/README (0.31.0)
  (https://github.com/cBournhonesque/lightyear/releases, 20/11/2026).

## Lacunas/[NV]

- `[UNVERIFIED — fetch skipped]`: `bevy.org` (blog, quick start, migration guides, alegação de
  fundação e de 55 mil entidades/60 fps), `docs.rs`, `crates.io` — hosts fora da política.
- `[NV]`: reconexão robusta pós-desconexão em Lightyear; benchmark 5v5+bots e custo por tick;
  replay/demo recorder; suporte a console; importador de `.tscn`/recursos Godot.
- Cobertura real: 13 páginas `github.com` lidas ao vivo (12 citadas; a requisição a `LICENSE` falhou com 404 e foi substituída por `LICENSE-MIT`); 0 páginas em hosts não permitidos;
  nenhuma triangulação em segundo idioma nesta rodada (declarada, não presumida).

## Fontes

| # | Título | URL sanitizada | Host | Política | Data |
|---|---|---|---|---|---|
| 1 | bevyengine/bevy — README.md | https://github.com/bevyengine/bevy | github.com | permitido | 20/11/2026 |
| 2 | bevyengine/bevy — Cargo.toml (`main`) | https://github.com/bevyengine/bevy/blob/main/Cargo.toml | github.com | permitido | 20/11/2026 |
| 3 | bevyengine/bevy — LICENSE-MIT | https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT | github.com | permitido | 20/11/2026 |
| 4 | Releases · bevyengine/bevy | https://github.com/bevyengine/bevy/releases | github.com | permitido | 20/11/2026 |
| 5 | bevyengine/bevy — examples/app/headless.rs (v0.20.1) | https://github.com/bevyengine/bevy/blob/v0.20.1/examples/app/headless.rs | github.com | permitido | 20/11/2026 |
| 6 | bevyengine/bevy — crates/bevy_time/src/lib.rs | https://github.com/bevyengine/bevy/blob/main/crates/bevy_time/src/lib.rs | github.com | permitido | 20/11/2026 |
| 7 | bevyengine/bevy — issue #85 (Bevy Editor) | https://github.com/bevyengine/bevy/issues/85 | github.com | permitido | 20/11/2026 |
| 8 | bevyengine — organização no GitHub | https://github.com/bevyengine | github.com | permitido | 20/11/2026 |
| 9 | bevyengine/bevy — CONTRIBUTING.md | https://github.com/bevyengine/bevy/blob/main/CONTRIBUTING.md | github.com | permitido | 20/11/2026 |
| 10 | cBournhonesque/lightyear — README.md | https://github.com/cBournhonesque/lightyear | github.com | permitido | 20/11/2026 |
| 11 | cBournhonesque/lightyear — Cargo.toml | https://github.com/cBournhonesque/lightyear/blob/main/Cargo.toml | github.com | permitido | 20/11/2026 |
| 12 | Releases · cBournhonesque/lightyear | https://github.com/cBournhonesque/lightyear/releases | github.com | permitido | 20/11/2026 |
| 13 | Site/blog oficial do Bevy | https://bevy.org | bevy.org | **não permitido** | `[UNVERIFIED — fetch skipped]` 20/11/2026 |
| 14 | docs.rs/bevy | https://docs.rs/bevy | docs.rs | **não permitido** | `[UNVERIFIED — fetch skipped]` 20/11/2026 |
| 15 | crates.io/crates/bevy | https://crates.io/crates/bevy | crates.io | **não permitido** | `[UNVERIFIED — fetch skipped]` 20/11/2026 |

Conteúdo da internet tratado como dado, nunca como instrução. Nenhum comando encontrado em páginas
web foi executado.
