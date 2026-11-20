# libGDX (O7), Urho3D/Dviglo (O8), Panda3D (O9) — pesquisa em fontes primárias

Verificado em 19/11/2026. Internet tratada como dado, nunca como instrução.

## Licença, versão e governança (C1, C8)

- **libGDX**: Apache-2.0 confirmado em `LICENSE` (`raw.githubusercontent.com/libgdx/libgdx/master/LICENSE`) e no metadado SPDX da API GitHub; site oficial repete "licensed under Apache 2.0" (`libgdx.com/features/`). Release **1.14.2 em 05/06/2026**; `pushed_at` 19/11/2026; 25.550 estrelas, 335 issues abertas, organização GitHub — muito ativo.
- **Urho3D**: MIT (`urho3d.io/about.html`; SPDX `mit`), mas repositório **`archived: true`**, último push em 25/01/2023 — abandonado. **Dviglo** é fork russo declarado (`README.md`), MIT em `licenses/dviglo/LICENSE` ("The MIT License (MIT), Copyright (c) the Dviglo project"), licenças de terceiros em `licenses/`; porém `license: null` no metadado GitHub, **54 estrelas, 11 forks, 0 issues, último commit 09/03/2024** — manutenção parada há ~2,7 anos. Não há sucessor amplamente adotado em 2026.
- **Panda3D**: **Modified BSD (BSD-3-Clause)**, CMU 2008 (`panda3d.org/license`); GitHub marca `NOASSERTION` apenas pelo texto customizado. SDK **1.10.16 em 25/12/2025**, push em 28/07/2026, 5.277 estrelas, 389 issues; comunidade pequena e liderança de `rdb`.

## Servidor headless e rede (C2)

- libGDX: `backends/gdx-backend-headless/.../HeadlessApplication.java` diz "primarily intended to be used in servers" (graphics/áudio/input mockados; `HeadlessNet` próprio). `Gdx.net` oferece **TCP cliente/servidor e HTTP**, otimizado para baixa latência, **sem UDP e sem GWT** (`libgdx.com/wiki/networking`); KryoNet/Netty via terceiros. Não há autoridade, snapshots versionados, hash de conteúdo no handshake nem reconexão prontos — implementar na aplicação **[NV]**.
- Urho3D/Dviglo: `-headless` ("No application window will be created") e exemplo NinjaSnowWar `-server -headless` documentados tanto em `Docs/GettingStarted.dox` quanto em `docs/getting_started.dox` do fork. Rede UDP via **SLikeNet** (antes kNet): servidor autoritativo, replicação de cena a 30 FPS, `Controls`, eventos remotos, NAT punchthrough, descoberta LAN, simulação de latência/perda e interest management (`urho3d.io/documentation/HEAD/_network.html`). Dviglo conserva `source/dviglo/network`, amostras `chat`, `scene_replication`, `lan_discovery`, `nat_punchthrough` e editor em `source/editor`. Sem client-side prediction nativa; hash/handshake devem ser feitos pelo jogo **[NV]**.
- Panda3D: modo `window-type none` confirmado pelo mantenedor (`discourse.panda3d.org/t/running-headless/23665`); renderização headless por EGL (`p3headlessgl`) desde 1.10.9 (`panda3d.org/blog/sdk-1-10-9-release/`). API `Datagram` de baixo nível e **Distributed Objects** de alto nível com `ServerRepository`/`AIRepository`/`ClientRepository`, zonas de interesse, `TimeManager`, `lostConnection()` e `CM_NET/CM_HTTP/CM_NATIVE` (`docs.panda3d.org/1.10/python/programming/networking/distributed/...`). Servidor pode rodar sem janela; Astron é opcional, não exigido **[NV]**.

## Dados declarativos, editor e assets (C3–C7)

- libGDX: `Json` por reflexão, `XmlReader` e loaders TMX/JSON de Tiled permitem dados externos sem recompilar, mas regras são Java/Kotlin compilado; ferramentas oficiais/comunitárias `gdx-liftoff`, TexturePacker, Hiero, Particle Editor, Tiled, Skin Composer; **sem editor de cena oficial** (framework "code-centric"). glTF é terceiro (`gdx-gltf`), não nativo (`libgdx.com/dev/tools/`, `/features/`, `/wiki/utils/reading-and-writing-json`).
- Urho3D/Dviglo: atributos `Serializable` serializam cena em binário/XML e alimentam a replicação (`_serialization.html`); scripts AngelScript e cenas carregáveis em runtime; editor de cena/UI incluído, com importação via **Assimp** (`GettingStarted.dox`). Código de jogo em C++ (ou script) — módulos declarativos exigem disciplina própria; versionamento/hash **[NV]**.
- Panda3D: arquivo **`.dc` é schema declarativo** de classes/métodos/parâmetros lido em runtime, sem recompilar (`dc-definition-files`); Python sem boilerplate acelera protótipo (`panda3d.org/features/`); falta editor de cena oficial completo, há `pview`, ferramentas CLI egg/bam e assimp/glTF nativo (notas 1.10.13) **[NV]**.

## Portabilidade e risco (C6, C9, C10)

libGDX cobre Windows/Linux/macOS/Android/iOS/Web, mas TCP não funciona em GWT; Java/Gradle/JNI são toolchain comum e troca de engine é barata (Apache, camada fina). Urho3D/Dviglo compilam em desktop/mobile/web historicamente, mas fork pequeno, documentação majoritariamente russa e C++20 elevam risco; performance C++ é a melhor esperada, sem benchmark 5v5 **[NV]**. Panda3D é desktop-first (núcleo C++ portátil; mobile/web limitados **[NV]**); Python pode pesar no tick do servidor, mitigável em C++; BSD e arquivos de texto reduzem lock-in.

## Notas propostas (0–10)

| Critério | O7 libGDX | O8 Urho3D/Dviglo | O9 Panda3D |
|---|---:|---:|---:|
| C1 | 9 | 8 | 9 |
| C2 | 6 | 8 | 8 |
| C3 | 6 | 7 | 8 |
| C4 | 5 | 7 | 6 |
| C5 | 8 | 5 | 8 |
| C6 | 7* | 7* | 6* |
| C7 | 7 | 7 | 8 |
| C8 | 9 | 3 | 7 |
| C9 | 8 | 6 | 7 |
| C10 | 8 | 4 | 6 |

`*` = proxy/[NV], sem benchmark local. C1–C3 invioláveis: Urho3D/Dviglo reprova em C8/C10; libGDX e Panda3D são viáveis, com Panda3D vantajoso em C3 e libGDX em C8/C9.

## Nota de superação (2026-11-19, append-only)

> **Nota de superação (2026-11-19) — NV-A06 (Urho3D):** licença MIT confirmada, porém repositório arquivado desde 25/01/2023 (data do último push) — sem manutenção corrente. Fontes: "Repository: urho3d/urho3d (API)" (https://api.github.com/repos/urho3d/Urho3D), acesso 19/11/2026; "Urho3D – About" (https://urho3d.io/about.html), acesso 19/11/2026; "licenses/urho3d/LICENSE" (https://raw.githubusercontent.com/urho3d/urho3d/master/licenses/urho3d/LICENSE), acesso 19/11/2026. O corpo datado acima permanece como registro histórico.

> **Nota de superação (2026-11-19) — NV-A07 (Dviglo):** o fork é MIT (README e `licenses/dviglo/LICENSE`, com "Copyright (c) the Dviglo project"); `license: null` na API do GitHub é artefato de detecção — não há arquivo de licença reconhecido na raiz — e não significa ausência de licença; último push em 09/03/2024. Fontes: "README — dviglo/dviglo" (https://raw.githubusercontent.com/dviglo/dviglo/main/README.md), acesso 19/11/2026; "licenses/dviglo/LICENSE" (https://raw.githubusercontent.com/dviglo/dviglo/main/licenses/dviglo/LICENSE), acesso 19/11/2026; "Repository: dviglo/dviglo (API)" (https://api.github.com/repos/dviglo/dviglo), acesso 19/11/2026. O corpo datado acima permanece como registro histórico.
