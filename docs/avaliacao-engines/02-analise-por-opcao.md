# Análise explicativa por critério e por opção — engines para o Insurgência

Data: 19/11/2026. Escopo: 13 opções canônicas do contrato (`00b-contrato-escrita.md`), critérios e pesos de `00-criterios.md` (Σ = 100; C1–C3 invioláveis). As notas abaixo são as dos oito relatórios em `pesquisa/`; não houve protótipo, build ou benchmark, de modo que toda nota com `*` é conservadora e toda inferência está marcada. Os índices ponderados exibidos nas fichas são **cálculo derivado** de Σ(nota × peso)/10 a partir dessas notas — servem apenas de coerência com a matriz (`01-matriz.md`, em elaboração quando deste texto) e não substituem a decisão final da Fase 0, que depende da regra dos invioláveis e da decisão **Godot-first** do GDD §14.8. Termos estrangeiros são explicados na primeira ocorrência.

## Análise por critério

**Correções factuais obrigatórias aplicadas neste documento** (contrato de escrita; confirmadas em fontes primárias em `licencas_lockin.md`): (1) Panda3D é **BSD-3-Clause (Modified BSD)**, não MIT; (2) MonoGame e FNA são **Ms-PL**, não MIT puro; (3) o **compilador Haxe é GPL-2.0-or-later** — stdlib e OpenFL são MIT, e a saída gerada não é contaminada; (4) Flax é **source-available sob EULA** (royalty de 4%, usuário único), não software livre; (5) Defold tem licença **não-OSI** (cláusula 4(a) veta comercializar o motor); (6) LÖVE é **zlib**, com OpenAL Soft **LGPL-2.0+** embarcado; (7) Urho3D está **arquivado** e o fork Dviglo parou em **09/03/2024**; (8) Bevy 0.19 **não tem editor visual oficial**; (9) Godot **4.7.2 é a estável** e 4.8-beta4 é prévia.

### C1 — Soberania e licença (peso 15, inviolável)

O topo do critério é ocupado por Bevy (O2/O3, `MIT OR Apache-2.0`, nota 10) e Stride (MIT sob a .NET Foundation, nota 10), seguidos por Godot (MIT, 9 — não chega a 10 apenas pela política de marca da Fundação: usar "Godot" no nome de um fork comercial exige autorização escrita, fora das quatro hipóteses automáticas de uso permitido, e motor/fork comercial exige licença de marca customizada — "GODOT TRADEMARK POLICY AND LICENSE" (https://godot.foundation/policies-and-procedures/trademark-policy), v. 21/07/2025, acesso 19/11/2026; "Governance – Godot Engine" (https://godotengine.org/governance), acesso 19/11/2026; "Godot Foundation Update December 2024" (https://godotengine.org/article/godot-foundation-update-dec-2024), 02/12/2024, acesso 19/11/2026. Ressalva: é restrição de marca, não de licença de software; o detalhamento da licença MIT do motor está em `licencas_lockin.md`), libGDX (Apache-2.0, 9), Panda3D (BSD-3-Clause corrigida, 9) e macroquad+tokio (Apache-2.0 + MIT, 9). A camada intermediária reúne Urho3D (MIT, 8), MonoGame/FNA (Ms-PL aprovada pela OSI, 8), LÖVE (zlib, 8) e Haxe/OpenFL (8, com o compilador GPL restrito à ferramenta). As duas reprovações estruturais são Defold (7: fonte total e gratuidade perpétua, mas licença não-OSI e veto estatutário à venda do motor) e Flax (4: EULA intransferível, usuário único, royalty de 4% acima de US$ 250 mil, foro na Polônia). Apenas Flax impõe EULA (contrato de usuário final) obrigatório; o alvo SWF do Haxe depende de SDK (kit de desenvolvimento de software) proprietário da Harman, mas é opcional e fora do caminho crítico. No mais, apenas as notas ≥5 em C1 sobrevivem à triagem inviolável — Flax é eliminada aqui. Fontes: `godot.md`, `bevy.md`, `defold_flax_stride.md`, `licencas_lockin.md` (tabelas C1 e C10; `https://godotengine.org/license/`, `https://github.com/bevyengine/bevy/blob/v0.19.1/Cargo.toml`, `https://raw.githubusercontent.com/libgdx/libgdx/master/LICENSE`, `https://www.panda3d.org/license/`, `https://flaxengine.com/licensing/`).

A distinção entre licença do motor e licença do conteúdo importa: em Godot, jogos não herdam o MIT, e terceiros ficam em `COPYRIGHT.txt`; em Flax, a EULA proíbe expressamente GPL/LGPL/CC-BY-SA e cede IP (propriedade intelectual) das contribuições ao fornecedor; em LÖVE, os componentes embarcados exigem inventário cuidadoso antes de redistribuir. Para o Pilar I do GDD (soberania sobre regras e infraestrutura), isso significa que "ter o código" não basta: é preciso poder **forkar (bifurcar)**, redistribuir e hospedar sem pedir permissão — condição que Defold e Flax não garantem plenamente (URLs em `licencas_lockin.md`, Tabela C1; confiança alta).

### C2 — Servidor autoritativo e federação (peso 15, inviolável)

Lightyear (O3) é o canônico (9*): servidor autoritativo por definição, handshake netcode.io com `protocol_id` e `ConnectToken`, input do *tick* (passo de simulação) N processado no tick N, predição/rollback e *interest management* (gestão de área de interesse) — sem serviço central obrigatório. Godot (8) vem logo atrás com `--headless`, preset "Export as dedicated server", `@rpc` com autoridade por nó, `get_remote_sender_id()`, `MultiplayerSynchronizer` e handshake customizável por `auth_callback`; o próprio documento oficial recomenda validar argumentos no servidor, mas **não valida automaticamente**. Panda3D (8) e Urho3D/Dviglo (8) entregam autoridade real — Distributed Objects com `ServerRepository`/zonas de interesse e replicação SLikeNet a 30 FPS com `Controls` inspecionados no servidor, respectivamente. Flax (7*) e Stride (6*) ficam abaixo por dependerem, na prática, de código próprio ou bibliotecas da comunidade; libGDX (6) e Haxe (6) oferecem transporte headless, não netcode; LÖVE (5) e macroquad (5*) só viabilizam servidor desligando módulos gráficos. Bevy puro (4*), MonoGame (4*), FNA (3*) e Defold (3*) reprovam: Defold tem `dmengine_headless`, mas nenhuma autoridade ou replicação pronta. Fontes: `netcode_modularidade.md` (matriz HD/VA/PV/SI/RS/AC/RD), `godot.md`, `bevy.md`, `defold_flax_stride.md`.

O ponto cego é comum a **todas** as 13 opções: reconexão que restaure o estado completo da sessão (RS) e hash de conteúdo no handshake (PV) não são primitivas documentadas — há `PeerId` persistente no Lightyear, `peer_disconnected` no Godot e `ID_INCOMPATIBLE_PROTOCOL_VERSION` no Urho3D, mas a reconstrução cabe ao jogo (`[NV]` para todas em `netcode_modularidade.md`). A tolerância a perda/reconexão do GDD §14 será, portanto, trabalho de aplicação, não diferencial de engine.

### C3 — Modularidade Núcleo + Módulos (peso 12, inviolável)

Panda3D (8) e Godot (8) lideram: o arquivo `.dc` do Panda3D é um *schema* declarativo lido em runtime, e o Godot combina `Resource`/`.tres`, `PackedScene` e `ProjectSettings.load_resource_pack()` para carregar PCK (pacote de conteúdo) em runtime sem recompilar o Núcleo. O padrão de referência citado na pesquisa é o MiniYaml do OpenRA — formato textual orientado a linhas com mods distribuídos como pacotes de dados — e nenhuma engine da amostra o traz embutido. Stride (7) oferece *packages* e ativos tipados; Urho3D/Dviglo (7) serializa cenas em XML/JSON/binário com `AddRequiredPackageFile()`; Bevy e Bevy+Lightyear (7* ambos) têm ECS (Entity Component System — entidades, componentes e sistemas), plugins e BSN (Bevy Scene Notation), mas **o loader `.bsn` de primeira parte e o hash/versão de conteúdo não existem na 0.19** — protocolo e componentes são registrados em Rust em tempo de compilação, o que trava a recarga declarativa. Defold (6) e Flax (6) dependem de collections/prefabs sem padrão Núcleo+Módulos versionado; libGDX (6) usa JSON/Tiled, mas regras são Java/Kotlin compilado; LÖVE (5) e Haxe/OpenFL (4) não têm loader declarativo; MonoGame/FNA (4) têm content pipeline em tempo de build; macroquad (3) exige recompilar. Fontes: `netcode_modularidade.md` (coluna RD), `godot.md`, `libgdx_urho_panda.md`, `bevy.md`.

Nenhuma opção atende integralmente ao GDD §14.2 (regras, personagens e mapas carregados por dados + hash no handshake); a melhor aproximação documental é Godot, coerente com a decisão de usar cenas como ilhas e resources como Módulos. Onde o índice é 5 ou menos — MonoGame, FNA, Haxe, macroquad — a regra inviolável barra a adoção independentemente do score agregado.

### C4 — Adequação aos Desafios de Ilha (peso 10)

Godot (8) oferece as primitivas mais próximas: `@export` + Inspetor para editar regras, sinais, `AreaLight3D` e prévia de shader no editor 4.7; coleta, sabotagem e negociação com PNJs (personagens não jogáveis) continuam sendo lógica do jogo (`[NV]` nos relatórios, que não prototiparam). Bevy+Lightyear (7*) cobre a janela de sincronia de 2 jogadores/5 s com predição, rollback e interpolação, e Flax (7) cobre com Visual Script, física e amostra de lobby; Urho3D/Dviglo (7) tem editor de cena e AngelScript. Defold (6), Stride (6) e Panda3D (6) entregam ferramentas, mas sem sincronia pronta. libGDX (5) e MonoGame (5) são code-centric, sem editor de cena oficial. Bevy (4*) sofre com a **ausência de editor visual oficial na 0.19** (correção obrigatória: a issue GUI #24637 foi fechada e a #85 segue aberta; só existe o protótipo comunitário Jackdaw). Haxe/OpenFL (3) e macroquad (3) não têm editor de lógica; LÖVE (2) é 2D e incompatível com as ilhas 3D low-poly/glTF do GDD. Fontes: `godot.md`, `bevy.md`, `defold_flax_stride.md`, `libgdx_urho_panda.md`, `monogame_love_haxe.md`.

### C5 — Tempo até protótipo e curva (peso 10)

Godot (9) é a referência para um mantenedor com base em engenharia de software: GDScript (linguagem de script do Godot, tipo Python) dispensa etapa de build, o binário é autocontido e a documentação é extensa; SCons só entra para compilar a própria engine. LÖVE (9) tem curva ainda menor (arquivo único), mas a natureza 2D anula a vantagem. Defold (8), libGDX (8) e Panda3D (8) combinam editor leve/hot reload (recarga em tempo real), ferramentas maduras ou Python sem boilerplate. Stride (7) é acessível em C#, mas o Game Studio roda só no Windows e a documentação está em transição. Bevy (6) e Flax (6) pagam o preço de Rust com *borrow checker* (verificador de empréstimos) e MSRV 1.95, e de C++/.NET 10, respectivamente; MonoGame (6) é framework sem editor. Bevy+Lightyear (5*) acrescenta a matriz rígida de versões e o book WIP; macroquad (5) e Haxe/OpenFL (5) partem de quase zero; FNA (3) desaconselha iniciantes, sem NuGet e com `fnalibs` locais. Fontes: `godot.md`, `bevy.md`, `defold_flax_stride.md`, `libgdx_urho_panda.md`, `monogame_love_haxe.md`.

### C6 — Performance e escala da partida (peso 8)

Todas as notas deste critério são proxies `[NV]`: nenhum relatório rodou benchmark de 5v5 + bots. O melhor sinal documental é Bevy/Lightyear (8*), com 55.000 entidades a 60 fps no exemplo `bevy_city`, benches Criterion e gestão de largura de banda/interesse; Godot (7*) oferece headless, strip visuals e `max_sync_packet_size=1350`; libGDX (7*) e Urho3D/Dviglo (7*) confiam em backend enxuto e replicação a 30 FPS; macroquad (6*) é leve por construção. Panda3D (6*) preocupa pelo custo do Python no tick, mitigável em C++; Stride (6*) aposta em Vulkan/D3D12 e Native AOT; Defold (6*) e Flax (6*) usam `-null`/`-headless`. MonoGame/FNA e Haxe (5*) carecem de evidência de carga; LÖVE (4*) é o pior proxy. A nota 7* do Godot não é medida — é inferência documental que a Fase 0 precisa confirmar com o build headless real (S6 do GDD §14.8). Fontes: `godot.md`, `bevy.md`, `defold_flax_stride.md`, `libgdx_urho_panda.md`, `monogame_love_haxe.md`.

### C7 — Pipeline de assets FOSS (peso 8)

Godot (9) é o melhor caminho para CC0/low-poly: glTF 2.0 (`.gltf`/`.glb`) recomendado, `.blend` via Blender ≥3.0, FBX via ufbx **sem SDK proprietário** e demos oficiais sob MIT. Defold (8), Flax (8), Panda3D (8) e Bevy/O3 (8) importam GLB/glTF de primeira parte — o loader glTF do Bevy expõe rótulos e extras, embora faltem Draco, meshopt, KTX2/WebP e `KHR_materials_variants`. Urho3D/Dviglo (7) e libGDX (7) dependem de Assimp e `gdx-gltf` de terceiros, respectively. Stride (5*) é penalizado pela **issue #1666 ainda aberta após 4.4.3**: `.glb` com texturas embarcadas falha na importação Assimp nessa versão; **sem fix registrado, não testado empiricamente** (afirmação documental, não falha observada nesta rodada) — "Issue 1666 — glb import (API)" (https://api.github.com/repos/stride3d/stride/issues/1666), acesso 19/11/2026; "Issue 1666 comments/timeline" (https://api.github.com/repos/stride3d/stride/issues/1666/comments · /timeline), acesso 19/11/2026; "Stride 4.4 release notes" (https://raw.githubusercontent.com/stride3d/stride-docs/master/en/release-notes/4.4.md), atualizado 17/11/2026, acesso 19/11/2026 — workaround `.gltf`/OBJ/FBX. MonoGame (6) não tem glTF nativo; macroquad (5*) depende de crate de terceiro; FNA (4) carrega PNG/WAV/OGG direto e desaconselha XNB; Haxe/OpenFL (3) e LÖVE (2) ficam atrás, o último por ser 2D. Fontes: `godot.md`, `bevy.md`, `defold_flax_stride.md`, `libgdx_urho_panda.md`, `monogame_love_haxe.md`.

### C8 — Maturidade e governança (peso 8)

libGDX (9, 25.550 estrelas, push em 19/11), Godot (9, fundação neerlandesa sem fins lucrativos desde 2022) e MonoGame (9, MonoGame Foundation) são os mais sólidos; Stride (8, release 4.4 em 09/10 e atividade diária), Bevy (8, seis anos, fundação sem fins lucrativos, metas transparentes) e Flax (8, releases frequentes e roadmap semanal) completam o grupo alto, com a ressalva de que Flax depende de projeto individual e Bevy tem churn de 0.x — 0.20 já lançado em 08/10/2026. Defold (7) e Panda3D (7) são ativos, mas com comunidades menores; Lightyear (7*) é concentrado e "4 versões por ciclo" é observação apenas do ciclo 0.19 do Bevy (0.28.0, 0.29.0, 0.30.0, 0.30.1; 1–13 publicações por ciclo), não lei geral; a UI de Releases está defasada em relação ao crates.io — "lightyear — crates.io API (60 versões)" (https://crates.io/api/v1/crates/lightyear), acesso 19/11/2026; "Releases · cBournhonesque/lightyear" (https://github.com/cBournhonesque/lightyear/releases), acesso 19/11/2026; README (tabela de séries menores) (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/README.md), acesso 19/11/2026. FNA (7) concentra manutenção em Ethan Lee; LÖVE (6) e Haxe/OpenFL (6) carecem de fundação neutra `[NV]` e têm ecossistema haxelib fragmentado. macroquad (5) está inativo desde 08/2026. O caso extremo é Urho3D/Dviglo (3): **Urho3D arquivado em 25/01/2023** — "Repository: urho3d/urho3d" (https://api.github.com/repos/urho3d/Urho3D), acesso 19/11/2026; "Urho3D – About" (https://urho3d.io/about.html), acesso 19/11/2026; "licenses/urho3d/LICENSE" (https://raw.githubusercontent.com/urho3d/urho3d/master/licenses/urho3d/LICENSE), acesso 19/11/2026 — e **Dviglo parado em 09/03/2024** (54 estrelas, 11 forks, 0 issues) — correção obrigatória do contrato. Fontes: `godot.md`, `bevy.md`, `defold_flax_stride.md`, `libgdx_urho_panda.md`, `monogame_love_haxe.md`, `alternativas.md`.

### C9 — Portabilidade futura (peso 7)

Defold (9) cobre desktop, web, mobile e consoles sem reescrita. Godot (8), Bevy (8), libGDX (8), Flax (8), macroquad (8) e Haxe/OpenFL (8 — maior amplitude: C++/HashLink/JS/Electron/Tauri/SWF) formam o segundo bloco. Lightyear (8*) anuncia wasm via WebTransport; Panda3D (7) e Stride (7) são desktop-first com runtime cross-platform, mas o editor do Stride fica no Windows; MonoGame (7) cobre desktop/mobile/consoles, sem alvo web oficial. Urho3D/Dviglo (6) compilava historicamente em desktop/mobile/web, mas o fork pequeno e a documentação majoritariamente russa pesam; FNA (5) exclui Android explicitamente e LÖVE (5) é limitado ao 2D. A ressalva do Godot é concreta: HTML5 tem WebSockets/WebRTC, **sem TCP/UDP cru e sem C#**; o cliente web do GDD dependerá de WebRTC/WebSocket e portas UDP exigem `port forwarding` (redirecionamento de porta) no headless. Fontes: `godot.md`, `bevy.md`, `monogame_love_haxe.md`, `libgdx_urho_panda.md`, `defold_flax_stride.md`.

### C10 — Risco operacional e lock-in (peso 7)

Defold (8), Stride (8) e libGDX (8) têm menor atrito: nenhum serviço obrigatório (o build server do Defold é substituível pelo `extender` self-host) e licenças permissivas; Godot (7) sofre com protocolo de alto nível que "may change without notice", port forwarding UDP e GDScript específico da engine, mitigados por formatos texto e GDExtension; MonoGame (7) é uma camada fina e barata de trocar. Bevy (6) e Lightyear (6*) sofrem com árvore grande de dependências, edition 2024, MSRV 1.95 e acoplamento de versões (`aeronet`, `postcard`, `bevy_replicon`, Leafwing). Panda3D (6) e macroquad (5) são substituíveis, mas o último exige duas pilhas. Flax (4) e Urho3D/Dviglo (4) combinam risco contratual e abandono, respectivamente; Haxe/OpenFL (4) fragmenta haxelib e atrai GPL se o compilador for forkado; LÖVE (4) carrega LGPL embarcado; FNA (3) é a mais arriscada (toolchain legado, mantenedor único). A cadeia de suprimentos é o risco transversal: SBOM (lista de componentes) no Godot, crates.io no Bevy/Lightyear, LGPL no LÖVE (`licencas_lockin.md`, Tabela C10).

## Análise por opção

As 13 fichas abaixo seguem a tabela canônica única de `00c-vetores-canonicos.md` (IDs, vetores C1–C10 e scores). O índice derivado é `Σ(nota × peso)/10`, uma casa decimal; a trava da Fase 0 usa **C2/C3/C4 < 6** (não C1–C3). Haxe/OpenFL não pertence ao cânone e não tem ficha; a referência histórica permanece apenas na tabela de evidência.

### O1 — Sem engine — Tauri v2 + Svelte 5 + DDD

- **Natureza:** baseline sem engine de jogo: Tauri v2 (janela), Svelte 5 (UI) e Núcleo DDD em Rust, sem runtime adicional.
- **Licença confirmada:** sem EULA/SDK proprietário no baseline; a lacuna de `LICENSE` na raiz do Aroeira penaliza C1, mas não zera a conformidade (C1 8*).
- **Pontos fortes:** dono de janela único (C2 10*); regras em `libs/domain` e snapshot aditivo v1 (C3 9*); Rust real com gates reusados (C4/C5 9*); feedback ≤60 s (C6 9*); Axum/SQLite (C7 6*); SDD/ADRs próprios (C8 8*); bundle <600 KB (C10 9*) — todos `[NV]`.
- **Pontos fracos:** nenhuma nota foi medida localmente; dívida *pedantic* em `playable.rs`; C7 e C8 são aspiracionais; Tauri mobile sem teste (C9 8*).
- **Risco principal:** ausência de piloto na Fase 0 — todas as conclusões são documentais/proxy `[NV]`.
- **Vetor (C1–C10):** 8* · 10* · 9* · 9* · 9* · 9* · 6* · 8* · 8* · 9*.
- **Trava (C2/C3/C4):** 10* · 9* · 9* — passa; é a única linha sem nota < 6.
- **Índice derivado / Fase 0:** 87,8 (Σ=878; 1º colocado) — **ADOPT**.

### O2 — Godot 4.7.2 (GDScript)

- **Natureza:** engine 3D/2D completa com editor, exportação headless e GDScript; 4.7.2 estável (18/08/2026), 4.8-beta4 prévia.
- **Licença confirmada:** MIT/Expat (C1 9*, reavaliado); marcas exigem autorização escrita para forks comerciais.
- **Pontos fortes:** `--headless` + preset `dedicated_server`; `@rpc` com autoridade por nó e `MultiplayerSynchronizer`; `Resource`/`.tres`/PCK em runtime; glTF/FBX sem SDK proprietário; fundação neutra (C8 9); portabilidade (C9 8).
- **Pontos fracos:** janela e render próprios (TAO vs `winit`, C35) → C2 2*; GDScript exigiria reescrever `libs/domain` → C3 4*; C4 3*; sem Módulo versionado nem hash no handshake `[NV]`; protocolo "may change without notice"; HTML5 sem UDP cru nem C#.
- **Risco principal:** segundo runtime de janela/render e linguagem específica — incompatível com o baseline Rust/DDD.
- **Vetor (C1–C10):** 9* · 2* · 4* · 3* · 6* · 5* · 6* · 9 · 8 · 6*.
- **Trava (C2/C3/C4):** 2* · 4* · 3* — reprovada.
- **Índice derivado / Fase 0:** 50,2 (Σ=502; 7º colocado) — **AVOID**.

### O3 — Godot 4.7.2 + godot-rust/gdext

- **Natureza:** Godot 4.7.2 com binding Rust `gdext` 0.5.5, comunitário.
- **Licença confirmada:** MIT/Expat no Godot; MPL-2.0 em `gdext` (copyleft de arquivo, sem contágio do Núcleo).
- **Pontos fortes:** `Resource`/`.tres`/PCK carregáveis em runtime preservam Módulos declarativos (C3 8*); integração em Rust (C4 8*, com ressalva — `pedagogia.md` sugere 6*); glTF sem SDK; fundação neutra (C8 8*).
- **Pontos fracos:** todas as notas `[NV]` (`gdext.md`); janela/render próprios → C2 3*; C5 5*, C6 4*, C7 5*, C9 4*, C10 4*; binding comunitário acoplado à versão do Godot.
- **Risco principal:** C2 — segundo runtime/schedule, agravado pela dependência de um binding de terceiros.
- **Vetor (C1–C10):** 7* · 3* · 8* · 8* · 5* · 4* · 5* · 8* · 4* · 4*.
- **Trava (C2/C3/C4):** 3* · 8* · 8* — reprovada em C2.
- **Índice derivado / Fase 0:** 57,6 (Σ=576; 3º colocado) — **AVOID**.

### O4 — Bevy 0.19.1 engine completa

- **Natureza:** engine ECS em Rust 0.19.1, sem editor visual oficial (issue #85 aberta; não é GPL).
- **Licença confirmada:** `MIT OR Apache-2.0` dual no `Cargo.toml`, sem EULA/SDK — C1 10 canônico.
- **Pontos fortes:** dupla licença; headless real via `ScheduleRunnerPlugin`; ECS, plugins e BSN; carregador glTF nativo; 55.000 entidades/60 fps em `bevy_city`; fundação sem fins lucrativos (C8 8).
- **Pontos fracos:** C2 2* (janela dupla somada à do Tauri, C54/C35); C3 5* (ECS desloca as regras de `libs/domain`); sem editor; MSRV 1.95/edition 2024; C6–C10 são proxies `[NV]`.
- **Risco principal:** C2/C3 — segundo dono de janela e regras fora do Núcleo DDD.
- **Vetor (C1–C10):** 10 · 2* · 5* · 8* · 5* · 4* · 5* · 8 · 7* · 4*.
- **Trava (C2/C3/C4):** 2* · 5* · 8* — reprovada.
- **Índice derivado / Fase 0:** 56,1 (Σ=561; 4º colocado) — **AVOID**.

### O5 — Bevy 0.19.1 headless (núcleo de simulação)

- **Natureza:** Bevy 0.19.1 reduzido a núcleo de simulação, sem janela/render, integrado ao baseline.
- **Licença confirmada:** `MIT OR Apache-2.0`, sem EULA/SDK.
- **Pontos fortes:** C1 10; `ScheduleRunnerPlugin` sem winit/render; C3 6* (regras permanecem em Rust); C4 8*; 55.000 entidades/60 fps em `bevy_city`; C8 8.
- **Pontos fracos:** C2 5* — segundo `Schedule`/runtime e IPC (`tauri_engine.md`); C9 6*, C10 4* (árvore de dependências 0.x); C6/C7 proxies `[NV]`.
- **Risco principal:** C2 — o segundo runtime/schedule bloqueia a Fase 0 mesmo sendo o 2º maior score.
- **Vetor (C1–C10):** 10 · 5* · 6* · 8* · 6* · 5* · 5* · 8 · 6* · 4*.
- **Trava (C2/C3/C4):** 5* · 6* · 8* — bloqueada em C2.
- **Índice derivado / Fase 0:** 64,2 (Σ=642; 2º colocado) — **DEFER**.

### O6 — Fyrox

- **Natureza (NV-A05):** engine Rust com editor de cena; correção de fato: **v1.0.0 estável (29/03/2026)**, com RC1 em 21/10/2025 e RC2 em 09/02/2026 — o registro anterior "1.0.1 estável (MSRV 1.87), 2.0.0-rc.1 prévia" estava desatualizado. Repositório ativo e licença MIT: "Repository: FyroxEngine/Fyrox" (https://api.github.com/repos/FyroxEngine/Fyrox), acesso 19/11/2026; "Fyrox/LICENSE.md" (https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/LICENSE.md), acesso 19/11/2026; "Releases · FyroxEngine/Fyrox" (https://api.github.com/repos/FyroxEngine/Fyrox/releases?per_page=5), acesso 19/11/2026; site https://fyrox.rs/. Ressalva: correção documental de versão/licença; vetor, índice e nota C1 9 permanecem inalterados, sem execução ou benchmark.
- **Licença confirmada:** MIT, sem EULA — C1 9.
- **Pontos fortes:** editor de cena e scripting; C4 7*; C8 7*; pipeline de assets própria.
- **Pontos fracos:** C2 2 (sem autoridade pronta); C3 3; C5 3* (compilação longa); C7 5* (`NetStream` TCP sem WebSocket); C10 3* (47 dependências). Notas derivadas de `fyrox.md`, `[NV]`.
- **Risco principal:** C2/C3 — nenhum caminho autoritativo nem formato de Módulo versionado.
- **Vetor (C1–C10):** 9 · 2 · 3 · 7* · 3* · 5* · 5* · 7* · 5* · 3*.
- **Trava (C2/C3/C4):** 2 · 3 · 7* — reprovada.
- **Índice derivado / Fase 0:** 47,0 (Σ=470; 9º colocado) — **AVOID**.

### O7 — macroquad + tokio

- **Natureza:** biblioteca de renderização 2D/3D (não engine) mais runtime assíncrono; macroquad inativo desde 08/2026.
- **Licença confirmada:** macroquad Apache-2.0 e tokio MIT, ambas OSI, sem EULA — C1 9.
- **Pontos fortes:** tokio roda headless sem janela/GPU (C2 5*); API simples e poucas dependências (C5 6*); C4 7*; C9 7*.
- **Pontos fracos:** C3 3 — sem loader declarativo, regras exigem recompilação (GDD §14.2); C7 3* (glTF via crate de terceiro); C8 5* (macroquad inativo); C2 5* — tokio não entrega rede pronta (`alternativas.md`).
- **Risco principal:** C2/C3 — rede e Módulos teriam de ser construídos na aplicação.
- **Vetor (C1–C10):** 9 · 5* · 3 · 7* · 6* · 4* · 3* · 5* · 7* · 5*.
- **Trava (C2/C3/C4):** 5* · 3 · 7* — reprovada em C2 e C3.
- **Índice derivado / Fase 0:** 54,4 (Σ=544; 5º colocado) — **AVOID**.

### O8 — wgpu + egui (renderer Rust custom)

- **Natureza:** renderer Rust custom — `wgpu` para GPU e `egui` para UI; sem engine completa e sem relatório dedicado.
- **Licença confirmada:** ambas permissivas, mas sem inventário fechado no Aroeira → C1 5* `[NV]` (contrato).
- **Pontos fortes:** C4 7* (Rust real, sem runtime alienígena); C6 5 documental; C3/C7/C10 5*.
- **Pontos fracos:** C2 4* — o exemplo `wgpu.rs` cria janela própria (segundo dono de janela, `tauri_engine.md`); C5 4*, C9 5*; todas as notas `[NV]`.
- **Risco principal:** C2/C3 — janela própria somada à do Tauri e nenhum formato de Módulo versionado.
- **Vetor (C1–C10):** 5* · 4* · 5* · 7* · 4* · 5 · 5* · 5* · 5* · 5*.
- **Trava (C2/C3/C4):** 4* · 5* · 7* — reprovada.
- **Índice derivado / Fase 0:** 50,4 (Σ=504; 6º colocado) — **AVOID**.

### O9 — Defold

- **Natureza:** engine 2D/3D leve com editor, Lua e hot reload; v1.14.1 (10/11).
- **Licença confirmada:** Defold License 1.0, **não-OSI** — cláusula 4(a) veta comercializar o motor; C1 4.
- **Pontos fortes:** `dmengine_headless`; exportação para todos os alvos; GLB/glTF nativos; fundação sueca; sem royalty.
- **Pontos fracos:** C2 3* (runtime Lua alienígena, sem caminho Tauri, sem autoridade/replicação); C3 5*; C4 2* (Lua ≠ Rust); C5/C6/C9/C10 são proxies `[NV]`.
- **Risco principal:** C4 e C1 — linguagem estranha ao Núcleo e licença não-OSI.
- **Vetor (C1–C10):** 4 · 3* · 5* · 2* · 4* · 5* · 5* · 7 · 6* · 6*.
- **Trava (C2/C3/C4):** 3* · 5* · 2* — reprovada em C4.
- **Índice derivado / Fase 0:** 41,9 (Σ=419; 12º colocado) — **AVOID**.

### O10 — libGDX

- **Natureza:** framework Java/Kotlin (não engine completa), code-centric, sem editor de cena oficial; 1.14.2 (05/06/2026).
- **Licença confirmada:** Apache-2.0 com grant de patente — C1 9.
- **Pontos fortes:** governança mais ativa da amostra (C8 9, 25.550 estrelas); backend `gdx-backend-headless`; TCP/HTTP; JSON/XmlReader/Tiled; camada fina (C10 7*).
- **Pontos fracos:** C2 3* (runtime JVM alienígena, sem Tauri); C3 5*; C4 2* (Java/Kotlin ≠ Rust); sem UDP, netcode externo (KryoNet/Netty); C5/C6/C9 proxies `[NV]`.
- **Risco principal:** C4 — linguagem e runtime alienígenas ao baseline Rust/DDD.
- **Vetor (C1–C10):** 9 · 3* · 5* · 2* · 4* · 5* · 5* · 9 · 6* · 7*.
- **Trava (C2/C3/C4):** 3* · 5* · 2* — reprovada em C4.
- **Índice derivado / Fase 0:** 48,6 (Σ=486; 8º colocado) — **AVOID**.

### O11 — Panda3D

- **Natureza:** engine Python com núcleo C++, voltada a aplicações distribuídas; SDK 1.10.16 (25/12/2025).
- **Licença confirmada:** **BSD-3-Clause (Modified BSD, CMU 2008)** — correção obrigatória, não MIT; C1 9.
- **Pontos fortes:** Distributed Objects (`ServerRepository`/`AIRepository`, zonas de interesse, `TimeManager`, `lostConnection()`); `.dc` como schema declarativo lido em runtime (C3 5*); glTF/assimp nativos (C7 5*); C8 7.
- **Pontos fracos:** C2 3* — `window-type none`/EGL headless existem, mas Python é segundo runtime e não há caminho Tauri; C4 2* (Python ≠ Rust: sem *ownership*/borrow checker nem telemetria Clippy); C5 4*, C6 4*, C9 5*, C10 5* — proxies `[NV]`.
- **Risco principal:** C2/C4 — segundo runtime e nenhum ganho de pedagogia Rust.
- **Vetor (C1–C10):** 9 · 3* · 5* · 2* · 4* · 4* · 5* · 7 · 5* · 5*.
- **Trava (C2/C3/C4):** 3* · 5* · 2* — reprovada em C2 e C4.
- **Índice derivado / Fase 0:** 45,0 (Σ=450; 10º colocado) — **AVOID**.

### O12 — LÖVE

- **Natureza:** framework **2D** em Lua; 11.5 (2023-12).
- **Licença confirmada:** **zlib** no motor, com OpenAL Soft **LGPL-2.0+** e componentes Apache/BSD/MIT embarcados que exigem inventário — C1 6.
- **Pontos fortes:** protótipo em arquivo único sem build; `graphics`/`window` desativáveis em `conf.lua`; `luasocket`/`lua-enet` embarcados (C2 3*).
- **Pontos fracos:** C4 2* (Lua ≠ Rust); C3 4* (sem Módulo versionado); C7 2* (sem snapshots/reconexão); incompatibilidade dimensional com as ilhas 3D; C5 4* (não reusa gates); C9 4* (sem caminho Tauri); C8 6 `[NV]`.
- **Risco principal:** C4 — inadequação ao produto, não apenas maturidade.
- **Vetor (C1–C10):** 6 · 3* · 4* · 2* · 4* · 4* · 2* · 6 · 4* · 5*.
- **Trava (C2/C3/C4):** 3* · 4* · 2* — reprovada em C4.
- **Índice derivado / Fase 0:** 37,5 (Σ=375; 13º colocado) — **AVOID**.

### O13 — Three.js + Node (cliente web)

- **Natureza:** stack web — Three.js para render 3D sobre Node, cliente web do GDD (`alternativas.md`, F2).
- **Licença confirmada:** MIT em ambas, sem EULA; C1 8* (`docs/avaliacao-engines/pesquisa/licencas_lockin.md:2-3,24-36`, lidas ao vivo em 19/11).
- **Pontos fortes:** ecossistema web maduro (C8 9); bom pipeline de assets (C7 6*); caminho natural para o cliente web.
- **Pontos fracos:** C2 3* (runtime alienígena, sem Tauri nem autoridade, `tauri_engine.md`); C3 4*; C4 2* (JavaScript ≠ Rust); C6 3*, C9 5*, C10 4* — proxies `[NV]`.
- **Risco principal:** C4 — o cliente web não substitui o Núcleo Rust/DDD.
- **Vetor (C1–C10):** 8* · 3* · 4* · 2* · 4* · 3* · 6* · 9 · 5* · 4*.
- **Trava (C2/C3/C4):** 3* · 4* · 2* — reprovada em C4.
- **Índice derivado / Fase 0:** 43,0 (Σ=430; 11º colocado) — **AVOID**.

## Evidência

Todas as linhas abaixo foram lidas em fonte primária em 19/11/2026, salvo quando indicado. "Alta" = repositório/docs oficiais; "Média" = fórum oficial ou metadado agregado; `[NV]` = não verificado ao vivo/benchmark ausente.

| Fonte | ID/URL | Data | Claim | Confiança |
|---|---|---|---|---|
| Godot — arquivo de downloads | https://godotengine.org/download/archive/ | 19/11/2026 | 4.7.2-stable (18/08/2026) é a estável; 4.8-beta4 (18/11) é prévia | Alta |
| Godot — licença | https://godotengine.org/license/ | 19/11/2026 | MIT/Expat; exige aviso de copyright; conteúdo do jogo não coberto | Alta |
| Godot Fundação — política de marcas | https://godot.foundation/policies-and-procedures/trademark-policy | 21/07/2025 | Nome de fork/engine comercial exige licença escrita; foro de Amsterdã | Alta |
| Godot — governança | https://godotengine.org/governance/ | 19/11/2026 | Fundação neerlandesa sem fins lucrativos (2022), distinta do copyright | Alta |
| Godot — servidores dedicados | https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_dedicated_servers.html | 19/11/2026 | Preset "Export as dedicated server", tag `dedicated_server`, strip visuals | Alta |
| Godot — multiplayer de alto nível | https://docs.godotengine.org/en/stable/tutorials/networking/high_level_multiplayer.html | 19/11/2026 | API usa só UDP (ENet); doc recomenda servidor como fonte da verdade; `auth_callback` | Alta |
| Godot — `SceneMultiplayer` | https://docs.godotengine.org/en/stable/classes/class_scenemultiplayer.html | 19/11/2026 | Protocolo "may change without notice"; `allow_object_decoding=true` permite RCE | Alta |
| Godot — Resources | https://docs.godotengine.org/en/stable/tutorials/scripting/resources.html | 19/11/2026 | `.tres`/`.res`, `@export`, `load()` em runtime | Alta |
| Godot — formatos 3D | https://docs.godotengine.org/en/stable/tutorials/assets_pipeline/importing_3d_scenes/available_formats.html | 19/11/2026 | glTF 2.0 recomendado; FBX via ufbx sem SDK proprietário | Alta |
| Godot — terceiros | https://raw.githubusercontent.com/godotengine/godot/master/thirdparty/README.md | 19/11/2026 | Bibliotecas vendorizadas sob várias licenças; exige SBOM | Alta |
| Bevy — `Cargo.toml` v0.19.1 | https://github.com/bevyengine/bevy/blob/v0.19.1/Cargo.toml | 19/11/2026 | `license = "MIT OR Apache-2.0"`, edition 2024, MSRV 1.95.0 | Alta |
| Bevy — release 0.19.1 | https://github.com/bevyengine/bevy/releases/tag/v0.19.1 | 13/08/2026 | Release patch de 0.19 | Alta |
| Bevy — anúncio 0.20 | https://bevy.org/news/bevy-0-20/ | 08/10/2026 | 0.19.1 já é uma versão atrás | Alta |
| Bevy — exemplo headless | https://github.com/bevyengine/bevy/blob/v0.19.1/examples/app/headless.rs | 19/11/2026 | `ScheduleRunnerPlugin` sem winit/render; servidor real | Alta |
| Bevy — cenas 0.19 | https://bevy.org/news/bevy-0-19/#next-generation-scenes | 19/06/2026 | BSN com patches composáveis; sem loader `.bsn` de primeira parte | Alta |
| Bevy — issue do editor | https://github.com/bevyengine/bevy/issues/24637 | 19/11/2026 | Issue de editor GUI fechada; #85 permanece aberta (sem editor oficial) | Alta |
| Bevy — aniversário/comunidade | https://bevy.org/news/bevys-sixth-birthday | 19/11/2026 | Fundação sem fins lucrativos; `bevy_city`: 55.000 entidades a 60 fps | Alta |
| Bevy — loader glTF | https://docs.rs/bevy/0.19.1/bevy/gltf/index.html | 19/11/2026 | Loader glTF 2.0 nativo com rótulos/extras | Alta |
| Lightyear — repositório | https://github.com/cBournhonesque/lightyear | 19/11/2026 | MIT/Apache; 0.28–0.30 → Bevy 0.19; 0.31 → 0.20 | Alta |
| Lightyear — conexão | https://cbournhonesque.github.io/lightyear/book/concepts/connection/title.html | 19/11/2026 | Handshake netcode.io com `protocol_id` + `ConnectToken` | Alta |
| Lightyear — servidor | https://cbournhonesque.github.io/lightyear/book/concepts/bevy_integration/server.html | 19/11/2026 | Servidor autoritativo em app Bevy; topologias cliente-servidor/P2P/host | Alta |
| Lightyear — interesse | https://cbournhonesque.github.io/lightyear/book/concepts/advanced_replication/interest_management.html | 19/11/2026 | Interesse por entidade/link ou "rooms" | Alta |
| Lightyear — `Cargo.toml` | https://raw.githubusercontent.com/cBournhonesque/lightyear/main/Cargo.toml | 19/11/2026 | Supply chain densa: aeronet rc, wtransport, bevy_replicon, tokio | Alta |
| Defold — licença | https://defold.com/license/ | 19/11/2026 | Defold License 1.0; não OSI; cláusula 4(a) veta comercializar o motor | Alta |
| Defold — rede | https://defold.com/manuals/networking/ | 19/11/2026 | HTTP/TCP/UDP/WebSocket/SSL; sem replicação ou snapshots | Alta |
| Defold — headless (fórum) | https://forum.defold.com/t/defold-headless-application-for-server/66685 | 19/11/2026 | `dmengine_headless` confirmado como servidor, sem guia oficial | Média |
| Defold — releases | https://github.com/defold/defold/releases | 19/11/2026 | v1.14.1 em 10/11 | Alta |
| Flax — licensing | https://flaxengine.com/licensing/ | 19/11/2026 | EULA, usuário único; royalty 4% acima de US$ 250 mil/trimestre | Alta |
| Flax — EULA (PDF) | https://flaxengine.com/wp-content/uploads/2017/11/Flax%20EULA.pdf | 31/10/2023 | Foro da Polônia; GPL/LGPL/CC-BY-SA proibidos; emenda até 1×/trimestre | Alta |
| Flax — headless (fórum) | https://forum.flaxengine.com/t/how-do-i-make-my-game-headless/2263 | 2025-08 | `-headless`, `-mute`, `-null` confirmados pelo mantenedor | Média |
| Flax — networking | https://docs.flaxengine.com/manual/networking/index.html | 19/11/2026 | `NetworkPeer`, `NetworkManager`, replicação, RPC, autoridade | Alta |
| Stride — download/licença | https://www.stride3d.net/download/ | 19/11/2026 | MIT sob .NET Foundation; sem EULA/conta/royalty | Alta |
| Stride — recursos de rede | https://doc.stride3d.net/en/community-resources/networking.html | 19/11/2026 | Servidor headless em amostra; netcode é terceiro (LiteEntitySystem) | Média |
| Stride — issue #1666 | https://github.com/stride3d/stride/issues/1666 | 31/10/2026 | `.glb` com texturas embarcadas falha na importação; issue aberta | Alta |
| Stride — anúncio 4.4 | https://www.stride3d.net/blog/announcing-stride-4-4/ | 09/10/2026 | Vulkan/D3D12, SPIR-V, Native AOT; runtime cross-platform | Alta |
| libGDX — LICENSE | https://raw.githubusercontent.com/libgdx/libgdx/master/LICENSE | 19/11/2026 | Apache-2.0 com grant de patente | Alta |
| libGDX — headless | https://github.com/libgdx/libgdx/blob/master/backends/gdx-backend-headless/src/com/badlogic/gdx/backends/headless/HeadlessApplication.java | 19/11/2026 | Backend "primarily intended to be used in servers" | Alta |
| libGDX — networking | https://libgdx.com/wiki/networking | 19/11/2026 | TCP cliente/servidor e HTTP; sem UDP e sem GWT | Alta |
| Urho3D — about | https://urho3d.io/about.html | 19/11/2026 | MIT; repositório arquivado, último push 25/01/2023 | Alta |
| Urho3D — networking | https://urho3d.io/documentation/HEAD/_network.html | 19/11/2026 | SLikeNet; autoridade, replicação a 30 FPS, interesse por distância | Alta |
| Dviglo — README | https://github.com/dviglo/dviglo/blob/master/README.md | 19/11/2026 | Fork russo; MIT; 54 estrelas; último commit 09/03/2024 | Alta |
| Panda3D — licença | https://www.panda3d.org/license/ | 19/11/2026 | **Modified BSD / BSD-3-Clause** (CMU 2008), não MIT | Alta |
| Panda3D — Distributed Objects | https://docs.panda3d.org/1.10/python/programming/networking/distributed/index.html | 19/11/2026 | `ServerRepository`/`AIRepository`, zonas de interesse, `lostConnection()` | Alta |
| Panda3D — arquivos `.dc` | https://docs.panda3d.org/1.10/python/programming/networking/distributed/dc-definition-files.html | 19/11/2026 | `.dc` é schema declarativo lido em runtime | Alta |
| MonoGame — LICENSE | https://github.com/MonoGame/MonoGame/blob/develop/LICENSE.txt | 19/11/2026 | **Ms-PL** + MIT em partes `Mono.Xna`; OSI aprovada | Alta |
| MonoGame — issue #7121 | https://github.com/MonoGame/MonoGame/issues/7121 | 19/11/2026 | Runtime headless pedido desde 2020, ainda aberta | Alta |
| FNA — LICENSE | https://github.com/FNA-XNA/FNA/blob/master/licenses/LICENSE | 19/11/2026 | **Ms-PL** (Ethan Lee e MonoGame Team) | Alta |
| LÖVE — license.txt | https://github.com/love2d/love/blob/main/license.txt | 19/11/2026 | **zlib**; OpenAL Soft **LGPL-2.0+** e componentes Apache/BSD/MIT embarcados | Alta |
| LÖVE — issue #2262 | https://github.com/love2d/love/issues/2262 | 2025-12 | Desativar `graphics`/`window` em `conf.lua` e sobrescrever `love.run` | Média |
| OpenFL — repositório (referência histórica; não opção canônica em `00c`) | https://github.com/openfl/openfl | 19/11/2026 | OpenFL sob MIT | Alta |
| Haxe — LICENSE (referência histórica; não opção canônica em `00c`) | https://github.com/HaxeFoundation/haxe/blob/HEAD/LICENSE | 19/11/2026 | **Compilador Haxe GPL-2.0-or-later** | Alta |
| Haxe — fundação (referência histórica; não opção canônica em `00c`) | https://haxe.org/foundation/open-source.html | 19/11/2026 | GPL da ferramenta não afeta código nem saída gerada | Alta |
| Haxe — Socket API (referência histórica; não opção canônica em `00c`) | https://api.haxe.org/sys/net/Socket.html | 19/11/2026 | `bind/listen/select` documentados; viabiliza headless | Alta |
| macroquad — repositório | https://github.com/not-fl3/macroquad | 19/11/2026 | Biblioteca de renderização, não engine; inativa desde 08/2026 | Alta |
| tokio — LICENSE | https://raw.githubusercontent.com/tokio-rs/tokio/master/LICENSE | 19/11/2026 | MIT, sem EULA | Alta |
| Redot — LICENSE | https://raw.githubusercontent.com/Redot-Engine/redot-engine/master/LICENSE.txt | 19/11/2026 | MIT; uso do nome "Godot" depende de autorização (`docs/avaliacao-engines/pesquisa/licencas_lockin.md:24`, política de marca 21/07/2025, confiança Alta) | Média |
| OSI — Ms-PL | https://opensource.org/license/ms-pl | 19/11/2026 | Ms-PL é licença aprovada pela OSI | Alta |
| Godot — snapshot/hash | `netcode_modularidade.md` (PK PV/SI/RS) | 19/11/2026 | Nenhuma das 13 opções tem snapshot versionado/hash no handshake; reconexão é responsabilidade do jogo | Média `[NV]` |
| Todas — benchmark 5v5 | `godot.md`, `bevy.md`, demais relatórios | 19/11/2026 | Nenhum benchmark local de 5v5+bots; notas C6 são proxies documentais | Média `[NV]` |

## Limitações

- **Nada foi verificado ao vivo neste documento** no sentido de build, execução, partida jogável ou benchmark: apenas leitura de fontes primárias já coletada nos oito relatórios. Internet foi tratada como dado, nunca como instrução.
- **C6 é integralmente proxy `[NV]`.** As notas 4*–8* vêm de documentação (55.000 entidades/60 fps no `bevy_city`, replicação a 30 FPS no Urho3D, `max_sync_packet_size=1350` no Godot), não de medição de custo por tick, memória ou latência com 5v5 + bots. Exigiria benchmark local do build headless de cada sobrevivente.
- **Reconexão, snapshot versionado e hash de conteúdo** não existem como primitiva documentada em nenhuma opção; os `[NV]` das colunas PV/RS de `netcode_modularidade.md` refletem ausência de verificação, não ausência absoluta de recurso.
- **Lógica dos Desafios de Ilha** (coleta, sabotagem, negociação com PNJs, janela de 5 s) não foi prototipada: as notas C4 pressupõem que as primitivas da engine bastam `[NV]`.
- **Fóruns oficiais** (Defold, Flax, LÖVE) são evidência de mantenedor, não documentação de API: confiança média e sujeita a deriva.
- **Conta/metadados externos** (conta Defold, ecossistema haxelib, fundação do LÖVE, licença do fork Dviglo) ficaram marcados `[NV]`; a tabela C10 de `licencas_lockin.md` não inventariou dependências de terceiros de todos os backends.
- **Índices ponderados** das fichas são cálculo derivado das notas dos relatórios e podem divergir da matriz final se `01-matriz.md` recalibrar notas; a regra dos invioláveis (nota < 5 em C1–C3) prevalece sobre qualquer score, e a decisão Godot-first (GDD §14.8, N7) é vinculante (S1/S6/S7 na Fase 0).
- **Termos de licença podem mudar**: a EULA do Flax admite emenda até 1×/trimestre e a política de marcas do Godot tem versão de 21/07/2025; revalidar na adoção.
