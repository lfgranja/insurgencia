# Outras opções Rust open source para jogo/engine — pesquisa primária

Data: 20/11/2026. Escopo: opções Rust complementares/alternativas às já cobertas em `bevy.md` e
`alternativas.md` (O13). Critérios e pesos de `../00-criterios.md:37-49` (C1–C3 invioláveis;
`score = Σ(nota × peso)/10`). Sem protótipo, build ou benchmark nesta rodada: toda nota com `*` é
proxy ou `[NV]`. Fontes **apenas** em hosts github.com (README, LICENSE/Cargo.toml, releases e
metadados da API do GitHub). Termos estrangeiros explicados na 1ª ocorrência.

**Cobertura real:** 6 opções, 100% das afirmações abaixo apoiadas em repositórios GitHub; os sítios
buscados** — `[UNVERIFIED — fetch skipped]` por política de URL. Não havia fontes primárias em
português; triangulação entre idiomas **não foi possível** (READMEs e manifests lidos em inglês).

## Resumo

| Opção | Natureza (classificação correta) | Licença SPDX | Manutenção (metadados GitHub) | Servidor headless (sem interface gráfica) / rede | Editor | Curva | Adequação ao Insurgência | Score |
|---|---|---|---|---|---|---|---|---:|
| **R1 macroquad** | **Biblioteca/framework** de renderização 2D (não engine completa); equivalente a O13 | `MIT OR Apache-2.0` (`Cargo.toml`; API detecta só `Apache-2.0`) | `pushed_at` 18/08/2026; 4.736 estrelas; 349 issues abertas; sem release marcado | Headless viável via Rust/`tokio`, mas **sem rede**; snapshots do zero | Nenhum | Baixa para desenhar; alta para jogo completo | EXCLUÍDA como engine-base (C3=3) | 54,5 |
| **R2 ggez** | **Framework** leve 2D (3D opcional), inspirado em LÖVE; não engine | `MIT` (README + `Cargo.toml` + API) | `pushed_at` 26/10/2026; 4.723 estrelas; release **0.9.3** (10/07/2023) vs. crate 0.10.0 | Rede é **Non-Feature** explícita; headless não documentado | Nenhum | Moderada (`EventHandler`, Rust) | REJECT: sem autoridade, sem módulos, foco 2D | 54,8 |
| **R3 Ambient** | **Engine/runtime multiplayer** escrita em Rust (WASM/WebGPU); **pausada indefinitely** | `MIT OR Apache-2.0` (README; API detecta `Apache-2.0`) | Aviso de pausa no README; `pushed_at` 07/01/2025; último release v0.2.1 (06/05/2023) | Banco de dados em tempo real + sincronização automática; servidor dedicado não documentado | Sem editor de cena próprio documentado | Alta (toolchain WASM/WebGPU, 0.3.2-dev) | REJECT: risco de abandono (C8=2) | 57,0 |
| **R4 godot-rust/gdext** | **Binding (ligação/ponte) GDExtension** para Godot 4 — **não é engine escrita em Rust**; a engine é Godot (C++) | `MPL-2.0` (README + `Cargo.toml` + API): copyleft fraco por arquivo | `pushed_at` 11/11/2026; 5.414 estrelas; release v0.5.5 (09/08/2026) | Herda `--headless`/`dedicated_server` do Godot; não implementa autoridade/snapshots | Editor do **Godot** (não do binding) | Alta (Rust + FFI + Godot; mudanças quebráveis eventuais) | Útil como camada Rust sobre O1, não como engine-base | 68,6* |
| **R5 Rapier** | **Biblioteca de física** 2D/3D (crates `rapier2d/3d`), não engine | `Apache-2.0` (README + `Cargo.toml` + API) | `pushed_at` 01/11/2026; 6.105 estrelas; workspace v0.36.1 | Roda sem janela/GPU no servidor; **nenhuma rede/autoridade** | Nenhum | Moderada (MSRV 1.86, edition 2024) | Complemento de física; travada como engine (C2/C3=3) | 60,4 |
| **R6 Fyrox** | **Engine escrita em Rust** com editor de cena (a única da amostra nessa classe) | `MIT` (README + `Cargo.toml` + API) | `pushed_at` 20/11/2026; 9.689 estrelas; release 1.0.0 (29/03/2026), master em 2.0.0-rc.1 | Sem crate de rede no workspace; headless/servidor não documentados | **Sim** (`editor`, `editor-standalone`, `project-manager`) | Alta (MSRV 1.94; divergência 1.0.0 vs. 2.0.0-rc.1) | REJECT na Fase 0 por C2=3, apesar do editor | 62,6 |

\* Score do **binding** mede o custo/benefício de integração sobre O1; não o coloca na matriz canônica
como engine. R1 reproduz o score de O13 (54,5) — não deve ser reintroduzido na matriz.

**Distinção exigida pelo protocolo.** "Engine escrita em Rust" pressupõe simulation/rendering/ferramentas
no próprio projeto: aqui apenas **Ambient** (pausada) e **Fyrox** se enquadram. macroquad e ggez são
bibliotecas/frameworks (o README de macroquad diz "game library", embora a descrição da API do
GitHub diga "Cross-platform game engine in Rust" — divergência de autodescrição registrada); gdext
é binding para uma engine C++; Rapier é biblioteca de física. Classificar gdext ou Rapier como
"engines Rust" inflaria artificialmente a amostra e mascararia a trava de C2/C3.

## Evidência por opção

### R1 — macroquad (mesma tecnologia de O13)

`Cargo.toml` declara `name = "macroquad"`, `version = "0.4.16"`, `license = "MIT OR Apache-2.0"` e
dependências `miniquad`, `image` com apenas `png`/`tga`, `glam`; o README descreve "simple and easy to
use game library", renderização 2D com *batching*, UI imediata e deploy único para WASM/Android. Isso
é primitivo de desenho, não motor: não há loader declarativo, editor, handshake ou rede. `pushed_at`
18/08/2026 e 349 issues abertas indicam manutenção lenta, mas viva ("not-fl3/macroquad — Cargo.toml",
https://raw.githubusercontent.com/not-fl3/macroquad/master/Cargo.toml; "not-fl3/macroquad — README.md",
https://raw.githubusercontent.com/not-fl3/macroquad/master/README.md; "Repository: not-fl3/macroquad",
https://api.github.com/repos/not-fl3/macroquad, todos acesso 20/11/2026). Primitivos 3D são citados
em `alternativas.md` (O13), mas não detalhados no README lido `[NV]`.

### R2 — ggez

O README define "lightweight cross-platform game framework for making 2D and 3D games", baseado em
`winit`/`wgpu`/`rodio`, e lista como **Non-Features** explícitas: física, animação, GUI, gestão de
assets, IA, **ECS** (Entity Component System — entidades/componentes/sistemas) e **Networking**.
`Cargo.toml` confirma `version = "0.10.0"`, `license = "MIT"`, `gltf`/`obj` como features opcionais e
`3d` desligado por padrão; a API do GitHub confirma MIT e `pushed_at` 26/10/2026, enquanto o último
release é 0.9.3 de 10/07/2023 — defasagem entre release e manifest. Sem servidor, sem autoridade e sem
edição de cenas ("ggez — README.md", https://raw.githubusercontent.com/ggez/ggez/master/README.md;
"ggez — Cargo.toml", https://raw.githubusercontent.com/ggez/ggez/master/Cargo.toml; "Repository:
ggez/ggez", https://api.github.com/repos/ggez/ggez; "Release 0.9.3",
https://api.github.com/repos/ggez/ggez/releases/latest, acesso 20/11/2026).

### R3 — Ambient

O README abre com aviso literal: "Work on the Ambient runtime is paused indefinitely". O mesmo README
descreve runtime extensível com "in-game real-time database", "automatic synchronization", pacotes
interoperáveis e pipeline de assets configurável — o que o torna o mais próximo de uma engine
multiprocessada em Rust, porém parada. `Cargo.toml` tem `version = "0.3.2-dev"`, `rust-version =
1.70.0`, `gltf`, `fbxcel`, `russimp` (Assimp), `quinn`, `h3-quinn` e `h3-webtransport` (redes/QUIC e
WebTransport); a API mostra `pushed_at` 07/01/2025 e último release v0.2.1 (06/05/2023). A palavra
"platform for building, deploying and sharing" levanta risco de serviço central, não confirmado
`[NV]` ("AmbientRun/Ambient — README.md",
https://raw.githubusercontent.com/AmbientRun/Ambient/main/README.md; "AmbientRun/Ambient — Cargo.toml",
https://raw.githubusercontent.com/AmbientRun/Ambient/main/Cargo.toml; "Repository: AmbientRun/Ambient",
https://api.github.com/repos/AmbientRun/Ambient; "Release v0.2.1",
https://api.github.com/repos/AmbientRun/Ambient/releases/latest, acesso 20/11/2026).

### R4 — godot-rust/gdext (binding, não engine)

O `ReadMe.md` raiz diz textualmente: "godot-rust is a library to integrate the Rust language with
Godot 4" e que a GDExtension (API de extensão do Godot) integra linguagens de terceiros; o
`godot/crate-readme.md` repete "Rust bindings for Godot 4". `godot/Cargo.toml` confirma crate `godot`
v0.5.5, `description = "Rust bindings for Godot 4"`, `license = "MPL-2.0"` e features `api-4-2` a
`api-4-7` (acoplamento de versão). O README explica o MPL: código do jogo pode ficar fechado, mas
alterações no próprio binding devem ser publicadas — copyleft por arquivo, não MIT. A API registra
`pushed_at` 11/11/2026 e release v0.5.5 (09/08/2026). O README ainda cita o binding legado
`godot-rust/gdnative` (Godot 3), não verificado nesta rodada `[NV]` ("godot-rust/gdext — ReadMe.md",
https://raw.githubusercontent.com/godot-rust/gdext/master/ReadMe.md; "godot crate — crate-readme.md",
https://raw.githubusercontent.com/godot-rust/gdext/master/godot/crate-readme.md; "godot — Cargo.toml",
https://raw.githubusercontent.com/godot-rust/gdext/master/godot/Cargo.toml; "Repository:
godot-rust/gdext", https://api.github.com/repos/godot-rust/gdext, acesso 20/11/2026).

### R5 — Rapier (física, não engine)

README: "2D and 3D physics engines for games, animation, and robotics", crates `rapier2d`,
`rapier3d`, `rapier2d-f64`, `rapier3d-f64`, "forever free and open-source"; solver SIMD em lote e
feature `parallel` via rayon. O workspace (`Cargo.toml`) fixa `version = "0.36.1"`, `license =
"Apache-2.0"`, `edition = "2024"`, `rust-version = "1.86"`, além de `rapier3d-meshloader`,
`rapier3d-urdf` e plugin `bevy_rapier` em `bindings/`. API: `pushed_at` 01/11/2026, 6.105 estrelas.
Não há renderização, janela, editor, rede ou carregamento de regras — é motor de física puro, útil
no servidor (inclusive variante f64 para determinismo) mas dependente de uma engine/application para
tornar-se jogo ("dimforge/rapier — README.md",
https://raw.githubusercontent.com/dimforge/rapier/master/README.md; "dimforge/rapier — Cargo.toml",
https://raw.githubusercontent.com/dimforge/rapier/master/Cargo.toml; "Repository: dimforge/rapier",
https://api.github.com/repos/dimforge/rapier, acesso 20/11/2026).

### R6 — Fyrox (engine escrita em Rust, com editor)

README: "feature-rich, production-ready, general purpose 2D/3D game engine written in Rust with a
scene editor" e "no AI/LLM was (and never will be) used in development". O `Cargo.toml` da raiz lista
membros `editor`, `editor-standalone`, `project-manager`, `fyrox-resource`, `fyrox-scripts`,
`fyrox-graphics-wgpu`; não há nenhum membro de rede. O `fyrox/Cargo.toml` declara `version =
"2.0.0-rc.1"`, `license = "MIT"`, `rust-version = "1.94"`, backends OpenGL e wgpu. A API mostra
`pushed_at` 20/11/2026, 9.689 estrelas e release 1.0.0 em 29/03/2026 — estável e master divergentes.
É a opção com melhor evidência primária de editor, mas sem netcode documentado
`[NV]` ("FyroxEngine/Fyrox — README.md",
https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/README.md; "Fyrox — Cargo.toml (workspace)",
https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/Cargo.toml; "fyrox — Cargo.toml",
https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/fyrox/Cargo.toml; "Repository:
FyroxEngine/Fyrox", https://api.github.com/repos/FyroxEngine/Fyrox, acesso 20/11/2026).

## Notas C1–C10

Pesos na ordem C1…C10 = 15, 15, 12, 10, 10, 8, 8, 8, 7, 7. Célula com `*` tem justificativa na
subseção seguinte.

| Critério (peso) | R1 macroquad | R2 ggez | R3 Ambient | R4 gdext | R5 Rapier | R6 Fyrox |
|---|---|---|---|---|---|---|
| C1 (15) | 9 — dupla licença OSI; API detecta só Apache-2.0 | 10 — MIT em três fontes | 9* — README diz dupla; API detecta Apache-2.0 | 8* — MPL-2.0 é OSI, mas copyleft por arquivo | 10 — Apache-2.0 em três fontes | 10 — MIT em três fontes |
| C2 (15) | 5* — headless possível; sem rede/snapshots | 3* — Networking é Non-Feature | 7* — sincronia automática; servidor dedicado `[NV]` | 7* — herda Godot; não implementa autoridade | 3* — física headless, sem rede | 3* — sem crate de rede no workspace |
| C3 (12) | **3** — sem loader declarativo; recompilar | 3 — só filesystem/zip e config | 5* — pacotes e pipeline, sem hash | 7* — herda `Resource`/PCK; sem hash | 3* — meshloader ≠ Módulos | 5* — recursos em arquivo, sem formato de mod |
| C4 (10) | 3 — sem editor; janela de 5 s manual | 3 — sem editor; foco 2D | 4* — sincronia ajuda; runtime parado | 7 — editor Godot + classes Rust | 4* — física apoia sabotagem; sem editor | 7 — editor de cena e scripts |
| C5 (10) | 5 — API simples, jogo do zero | 6 — `EventHandler` simples; Rust | 4* — parado, 0.3.2-dev | 4* — FFI, breaking changes, docs WIP | 6 — guia e exemplos; MSRV 1.86 | 5* — MSRV 1.94; 1.0.0 vs. 2.0.0-rc.1 |
| C6 (8) | 6* — leve, sem benchmark `[NV]` | 5* — wgpu, sem benchmark `[NV]` | 6* — "high-performance", sem benchmark `[NV]` | 6* — FFI + Godot, sem benchmark `[NV]` | 8* — SIMD/rayon/f64, sem 5v5 `[NV]` | 7* — backends modernos, sem benchmark `[NV]` |
| C7 (8) | 5* — `image` só png/tga; glTF de terceiro `[NV]` | 6 — feature `gltf` opcional + zip | 7 — `gltf`/`fbxcel`/`russimp` + pipeline | 8 — herda importação Godot | 5* — meshloader/URDF, não pipeline de engine | 6* — `fyrox-resource`; glTF não confirmado `[NV]` |
| C8 (8) | 5 — individual, lento, sem release marcado | 6 — ativo, mas release de 2023 | **2** — pausa indefinida no README | 8 — v0.5.5, CI com sanitizadores | 8 — 0.36.1 ativo, org Dimforge | 7 — ativo, mas dependência de mantenedor |
| C9 (7) | 8 — desktop/HTML5/Android/iOS | 7 — mobile/web "não oficialmente" | 7 — web/desktop; mobile `[NV]` | 7* — Wasm/Android/iOS experimentais | 8 — wasm, bindings C/JS/Python | 7 — desktop + wasm; mobile `[NV]` |
| C10 (7) | 5 — duas pilhas, substituível | 6 — camada fina, exige várias crates | 3 — pausada + risco de plataforma central | 6* — MPL + marca Godot + acoplamento de versão | 7 — substituível; acoplamento nalgebra/parry | 6 — monopólio de engine + divergência de versão |
| **Score** | **54,5** | **54,8** | **57,0** | **68,6** | **60,4** | **62,6** |

### Justificativas das notas com `*`

- **R1 C1/C2/C6/C7:** `license = "MIT OR Apache-2.0"` no manifest, mas metadado da API reporta
  apenas `Apache-2.0` — dupla fonte divergente; C2 usa `tokio` headless apenas como possibilidade
  (fora do crate); C6 e C7 sem benchmark e sem glTF nativo no manifest lido.
- **R2 C2/C6:** "Networking" aparece na lista de **Non-Features** do README, logo exige pilha
  externa desproporcional; desempenho inferido do backend `wgpu`, sem medição.
- **R3 C1/C4/C6:** licença dupla no README contra detecção da API; C4 penalizada porque o aviso de
  pausa invalida qualquer promessa de suporte; C6 é afirmação de marketing ("high-performance"),
  não benchmarks.
- **R4 C1/C2/C3/C6/C9/C10:** MPL-2.0 não é a licença permissiva preferida (C1); C2/C3 são
  capacidades **herdadas do Godot (O1)**, não implementadas pelo binding; C6 sem benchmark; C9 o
  próprio README marca Wasm/Android/iOS como experimentais; C10 soma copyleft de arquivo, política
  de marca da Godot Foundation e features `api-4-x`.
- **R5 C2/C3/C7:** Rapier é headless por natureza, mas ausência de rede e de sistema de Módulos é
  estrutural (não "falta de evidência"); `meshloader`/URDF carregam geometria, não regras de mod.
- **R6 C3/C5/C6/C7:** serialização de recursos existe (`fyrox-resource`), mas nenhum formato
  declarativo de mod/hash no README; C5 reflete MSRV 1.94 e release 1.0.0 defasado em relação a
  2.0.0-rc.1; C7 não confirma glTF nas fontes lidas; C6 sem benchmark 5v5.

**Trava inviolável (C1–C3 < 5):** R1 (C3=3), R2 (C2=3, C3=3), R5 (C2=3, C3=3) e R6 (C2=3) não
podem ser adotadas como engine-base na Fase 0. R3 (C8=2) e R4 (binding, não engine) ficam fora por
risco operacional e por categoria, respectivamente. Nenhuma das seis supera O1 Godot (82,6) ou O3
Bevy+Lightyear (77,1) na matriz canônica.

## Evidência Contra

- **Ambient desmente a tese "Rust resolve netcode":** multiprocessado por design, mas o próprio
  README encerra o runtime. Um motor parado não pode ser base de Núcleo (README + API, 20/11/2026).
- **Fyrox desmente "engine Rust com editor basta":** editor sólido, workspace sem rede; sem C2 não há
  servidor autoritativo, e C2 é inviolável (Cargo.toml raiz + README, 20/11/2026).
- **gdext não é prova a favor do Rust como engine:** é prova a favor do Rust *sobre* Godot; atribuir
  os 68,6 ao binding como se fosse engine seria erro de categoria. Ainda assim, MPL-2.0 diverge da
  preferência MIT do Pilar I (ReadMe.md + Cargo.toml, 20/11/2026).
- **macroquad já foi avaliada:** O13 tem 54,5 e C3=3; reintroduzi-la aqui como "nova" opção seria
  duplicar a matriz (`../01-matriz.md:134`, `alternativas.md:22`).
- **Rapier é complemento, não substituto:** ausência de renderização/edição/rede é uma virtude para
  física de servidor e uma limitação para engine-base (README + Cargo.toml, 20/11/2026).
- **Metadados conflitam entre si:** macroquad (`MIT OR Apache-2.0` vs. `Apache-2.0` na API), Ambient
  (idem), ggez (release 0.9.3 vs. crate 0.10.0) e Fyrox (release 1.0.0 vs. master 2.0.0-rc.1).
  Scores de maturidade devem usar a fonte primária de código, não só o release marcado.

## Lacunas / [NV]

- Nenhum servidor headless, validação de input, snapshot versionado, reconexão ou hash de conteúdo no
  handshake foi executado ou inspecionado em profundidade para as seis opções `[NV]`.
- Nenhum benchmark 5v5 + bots; todas as notas C6 são proxies documentais `[NV]`.
- Suporte a glTF/GLB em Fyrox e primitivos 3D em macroquad não confirmados nos arquivos lidos `[NV]`.
- Servidor dedicado e ausência de serviço central no Ambient não confirmados `[NV]` (README fala em
  "plataforma" de deploy).
- Extensibilidade do editor do Fyrox para lógica de Desafios de Ilha não verificada `[NV]`.
- `godot-rust/gdnative` (binding legado) e commits/READMEs em outros idiomas não examinados `[NV]`.
- Sítios fora de github.com — `fyrox.rs`, `rapier.rs`, `ambient.run`, `ggez.rs`, `crates.io`,
  `docs.rs`, `godot-rust.github.io`, `fyrox-book.github.io` — `[UNVERIFIED — fetch skipped]`.
- Cobertura linguística: inglês apenas; nenhuma fonte primária em português disponível nos
  repositórios, logo "triangule idiomas" não foi satisfeito nesta rodada.

## Fontes

1. "not-fl3/macroquad — README.md", https://raw.githubusercontent.com/not-fl3/macroquad/master/README.md, acesso 20/11/2026.
2. "not-fl3/macroquad — Cargo.toml", https://raw.githubusercontent.com/not-fl3/macroquad/master/Cargo.toml, acesso 20/11/2026.
3. "Repository: not-fl3/macroquad", https://api.github.com/repos/not-fl3/macroquad, acesso 20/11/2026.
4. "ggez — README.md", https://raw.githubusercontent.com/ggez/ggez/master/README.md, acesso 20/11/2026.
5. "ggez — Cargo.toml", https://raw.githubusercontent.com/ggez/ggez/master/Cargo.toml, acesso 20/11/2026.
6. "Repository: ggez/ggez", https://api.github.com/repos/ggez/ggez, acesso 20/11/2026.
7. "ggez — Release 0.9.3", https://api.github.com/repos/ggez/ggez/releases/latest, acesso 20/11/2026.
8. "AmbientRun/Ambient — README.md", https://raw.githubusercontent.com/AmbientRun/Ambient/main/README.md, acesso 20/11/2026.
9. "AmbientRun/Ambient — Cargo.toml", https://raw.githubusercontent.com/AmbientRun/Ambient/main/Cargo.toml, acesso 20/11/2026.
10. "Repository: AmbientRun/Ambient", https://api.github.com/repos/AmbientRun/Ambient, acesso 20/11/2026.
11. "Ambient — Release v0.2.1", https://api.github.com/repos/AmbientRun/Ambient/releases/latest, acesso 20/11/2026.
12. "godot-rust/gdext — ReadMe.md", https://raw.githubusercontent.com/godot-rust/gdext/master/ReadMe.md, acesso 20/11/2026.
13. "godot crate — crate-readme.md", https://raw.githubusercontent.com/godot-rust/gdext/master/godot/crate-readme.md, acesso 20/11/2026.
14. "godot — Cargo.toml", https://raw.githubusercontent.com/godot-rust/gdext/master/godot/Cargo.toml, acesso 20/11/2026.
15. "Repository: godot-rust/gdext", https://api.github.com/repos/godot-rust/gdext, acesso 20/11/2026.
16. "godot-rust/gdext — Release v0.5.5", https://api.github.com/repos/godot-rust/gdext/releases/latest, acesso 20/11/2026.
17. "dimforge/rapier — README.md", https://raw.githubusercontent.com/dimforge/rapier/master/README.md, acesso 20/11/2026.
18. "dimforge/rapier — Cargo.toml", https://raw.githubusercontent.com/dimforge/rapier/master/Cargo.toml, acesso 20/11/2026.
19. "Repository: dimforge/rapier", https://api.github.com/repos/dimforge/rapier, acesso 20/11/2026.
20. "FyroxEngine/Fyrox — README.md", https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/README.md, acesso 20/11/2026.
21. "Fyrox — Cargo.toml (workspace)", https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/Cargo.toml, acesso 20/11/2026.
22. "fyrox — Cargo.toml", https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/fyrox/Cargo.toml, acesso 20/11/2026.
23. "Repository: FyroxEngine/Fyrox", https://api.github.com/repos/FyroxEngine/Fyrox, acesso 20/11/2026.
24. "Fyrox — Release 1.0.0", https://api.github.com/repos/FyroxEngine/Fyrox/releases/latest, acesso 20/11/2026.
