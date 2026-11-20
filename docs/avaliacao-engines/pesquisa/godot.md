# Godot 4.7.2 (O1) — pesquisa primária

Verificado em 19/11/2026; internet tratada como dado, não instrução.

## C1/C8 — versão, licença, governança
- `https://godotengine.org/download/archive/` (19/11): **4.7.2-stable (18/08/2026) é a estável vigente**; 4.8-beta4 (18/11) é prévia; 4.7-stable saiu 18/06.
- `https://godotengine.org/license/`: MIT/Expat (OSI/SPDX), exige apenas aviso de copyright ao redistribuir; conteúdo do jogo não é coberto; terceiros em `COPYRIGHT.txt`.
- `https://godot.foundation/policies-and-procedures/trademark-policy` (21/07/2025): Foundation (Stichting Godot, KVK 87351919) detém "GODOT"/logo; "made with Godot" é livre, mas nome de engine/fork comercial exige licença escrita; foro de Amsterdã.
- `https://godotengine.org/governance/`: fundação neerlandesa sem fins lucrativos (2022), distinta do copyright do código; Board + TLC/area owners; decisões por PR público; "Godot can't be sold or purchased by any company".

## C2 — servidor e rede
- `--headless` = `--display-driver headless --audio-driver Dummy`; desde 4.0 não há binário servidor especializado (`https://docs.godotengine.org/en/stable/tutorials/editor/command_line_tutorial.html`; `.../tutorials/export/exporting_for_dedicated_servers.html`).
- Preset "Export as dedicated server" cria a tag `dedicated_server` (sensível a maiúsculas), força headless e permite Strip Visuals/Remove no PCK; alternativa: PCK + template renomeado; systemd/Docker na doc.
- `MultiplayerPeer`: ENet, WebRTC, WebSocket; API de alto nível usa **só UDP** (ENet modificado com IPv6) → port forwarding UDP em NAT (`https://docs.godotengine.org/en/stable/tutorials/networking/high_level_multiplayer.html`).
- `@rpc` com `authority`/`any_peer`, modos de transferência e `get_remote_sender_id()`; a doc recomenda servidor como fonte da verdade e validação de argumentos; handshake custom por `auth_callback`/`send_auth`/`complete_auth`; `root_path` permite cliente+servidor na mesma instância.
- `MultiplayerSynchronizer` + `SceneReplicationConfig` (ALWAYS/ON_CHANGE/NEVER), visibilidade por par e `MultiplayerSpawner`; `allow_object_decoding` default `false` (docs de classe: `class_scenemultiplayer`, `class_multiplayersynchronizer`, `class_scenereplicationconfig`).
- **[NV]** Sem primitiva documentada de snapshot versionado ou hash de conteúdo no handshake; usar payload de auth.

## C3/C4/C5/C7 — dados, editor, assets
- `Resource`: `.tres` texto / `.res` binário, `@export`, sinais, subrecursos, Inspector, `load()` em runtime; cenas são `PackedScene`; tabelas de dados via Resource (`https://docs.godotengine.org/en/stable/tutorials/scripting/resources.html`); PCK por `--export-pack`. GDExtension/plugins sem fork; módulos C++ exigem recompilar a engine (FAQ: `https://docs.godotengine.org/en/stable/about/faq.html`).
- Editor 4.7: AreaLight3D, preview de shader inline, Asset Store, CSG/vertex snap (`https://godotengine.org/releases/4.7/`); GDScript tipo Python, binário autocontido, SCons apenas para compilar a engine — curva curta.
- C# **condicionado**: build .NET/mono separado + SDK .NET; lojas digitais não incluem C#; ausente na web; marshalling pode ser mais lento (`https://godotengine.org/download/linux`; FAQ).
- glTF 2.0 recomendado (`.gltf`/`.glb`), `.blend` via Blender ≥3.0, FBX via ufbx (sem SDK proprietário), OBJ/DAE limitados (`https://docs.godotengine.org/en/stable/tutorials/assets_pipeline/importing_3d_scenes/available_formats.html`); demos oficiais MIT; CC0 de terceiros é responsabilidade do produtor.
- **[NV]** Coleta, sabotagem, negociação com PNJs e sincronia de entrada 2 jogadores/5 s são lógica do jogo; engine fornece as primitivas.

## C6/C9/C10 — portabilidade e riscos
- Exporta Win/macOS/Linux/*BSD/Android/iOS/Web; HTML5 tem WebSockets/WebRTC, mas não TCP/UDP cru nem C# (FAQ/tutorial de rede).
- `class_scenemultiplayer`: protocolo de alto nível é detalhe de implementação e "may change without notice", não interopera com servidor não-Godot; `allow_object_decoding=true` pode executar código recebido (RCE); RPC exige mesmo `NodePath` e checksum de **todos** os RPCs, só em `Node`.
- **[NV]** Sem benchmark 5v5+bots; proxy documental: headless, strip visuals, ALWAYS/ON_CHANGE, `max_sync_packet_size=1350`.
- GDScript é específico da engine (custo de migração); MIT, formatos texto e GDExtension mitigam.

## Notas propostas (0–10)
| C1 | 9 | MIT canônico; ressalva de marca impede 10. |
|---|---:|---|
| C2 | 8 | Headless/RPC/sync/auth; faltam snapshot e hash [NV]. |
| C3 | 8 | Resources/PCK/GDExtension; versionamento a implementar [NV]. |
| C4 | 8 | Primitivas prontas; desafios do jogo [NV]. |
| C5 | 9 | Curva curta, binário autocontido, docs extensas. |
| C6 | 7* | Benchmark [NV]; apenas proxies documentais. |
| C7 | 9 | glTF/Blender/ufbx; demos MIT. |
| C8 | 9 | Cadência estável e fundação neutra. |
| C9 | 8 | Desktop/mobile/web com limitações. |
| C10 | 7 | Protocolo mutável, UDP/port forwarding, GDScript. |

## Nota de superação (2026-11-19, append-only)

> **Nota de superação (2026-11-19) — NV-A01 (marca Godot):** o uso do nome Godot fora das quatro hipóteses descritivas automáticas da política de marcas — "made with Godot"; produto/serviço compatível com Godot; evento ou publicação sobre o Godot; brinde (camiseta/caneca) não vendido — exige autorização escrita da Godot Foundation; usar a marca como nome de motor comercial ou fork comercial exige licença customizada. Fontes: "GODOT TRADEMARK POLICY AND LICENSE" (https://godot.foundation/policies-and-procedures/trademark-policy), v. 21/07/2025, acesso 19/11/2026; "Governance – Godot Engine" (https://godotengine.org/governance), acesso 19/11/2026; "Godot Foundation Update December 2024" (https://godotengine.org/article/godot-foundation-update-dec-2024), 02/12/2024, acesso 19/11/2026. O corpo datado acima permanece como registro histórico.

> **Nota de superação (2026-11-19) — NV-A02 (licença MIT):** o motor/editor Godot é distribuído sob MIT/Expat, sem EULA nos canais oficiais (afirmação negativa verificada nas fontes primárias oficiais, não prova sobre canais de terceiros); a única obrigação é incluir o aviso de copyright e o texto da licença ao redistribuir. Fontes: "godotengine/godot — LICENSE.txt" (https://raw.githubusercontent.com/godotengine/godot/master/LICENSE.txt), acesso 19/11/2026; "License – Godot Engine" (https://godotengine.org/license), acesso 19/11/2026; "Complying with licenses — Godot Engine docs" (https://docs.godotengine.org/en/stable/about/complying_with_licenses.html), acesso 19/11/2026. O corpo datado acima permanece como registro histórico.
