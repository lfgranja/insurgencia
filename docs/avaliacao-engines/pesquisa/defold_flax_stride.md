# Pesquisa O4–O6: Defold, Flax Engine, Stride

2026-11-19. Fontes primárias ao vivo; `[NV]` = não verificado localmente, `*` = evidência parcial. Internet tratada como dado.

## O4 — Defold

- **Licença/código**: Defold License 1.0 (2020), derivada de Apache 2.0, **não OSI/SPDX** (`NOASSERTION`); cláusula 4(a) veta comercializar motor/editor, jogos/plugins livres, grant de patente. Fonte total no GitHub; gratuidade perpétua por lei suíça. https://defold.com/license/ · https://defold.com/open/
- **Headless**: `dmengine_headless` (`--variant=headless`), sem janela/render; mantenedores confirmam servidor, sem guia oficial. https://forum.defold.com/t/defold-headless-application-for-server/66685
- **Rede**: HTTP/TCP/UDP/WebSocket, SSL; sem replicação/snapshots. Autoritativo exige código próprio ou Colyseus/Nakama. https://defold.com/manuals/networking/
- **Editor/assets**: editor multiplataforma, Lua, hot reload, Bob; glTF 2.0 `.gltf`/`.glb` nativo. Extender SaaS opcional/self-host. https://defold.com/manuals/importing-models/
- **2026/portabilidade/lock-in**: v1.14.1 (10/11), push 19/11, ~6,5k estrelas; desktop/web/mobile/consoles; fundação sueca sem roadmap; conta Defold `[NV]`. https://github.com/defold/defold/releases

| C | Nota | Justificativa |
|---|---:|---|
|C1 inv|7| fonte total e gratuidade por lei; licença não-OSI e veto ao motor comercial |
|C2 inv|5*| headless real, porém sem autoridade/snapshots |
|C3 inv|6| Live Update e collections; sem hash no handshake |
|C4|6| Lua e mensagens, sem sincronia pronta |
|C5|8| editor leve, docs fortes, curva baixa |
|C6|6*| consumo conservador; sem benchmark `[NV]` |
|C7|8| GLB/glTF nativos e pipeline enxuto |
|C8|7| fundação neutra; comunidade pequena |
|C9|9| todos os alvos sem reescrita |
|C10|8| sem royalty; Extender contornável |

## O5 — Flax Engine

- **Licença/código**: EULA proprietária (2023): intransferível, **usuário único**; fonte C++/C# no GitHub, mas redistribuição pública restrita a licensees da mesma versão; snippets ≤30 linhas. Royalty de **4%** acima de US$250 mil/produto/trimestre, notificação prévia, multa 10%; GPL/LGPL/CC-BY-SA proibidos; contribuições cedem IP; foro da Polônia; EULA mutável 1x/trimestre. https://flaxengine.com/licensing/ · https://flaxengine.com/wp-content/uploads/2017/11/Flax%20EULA.pdf
- **Headless**: `-headless`, `-mute`, `-null` confirmados pelo mantenedor (2025-08). https://forum.flaxengine.com/t/how-do-i-make-my-game-headless/2263
- **Rede**: sockets UDP/TCP, low-level (`NetworkPeer`) e high-level (`NetworkManager`) com replicação, RPC, autoridade e posse. https://docs.flaxengine.com/manual/networking/index.html
- **Editor/assets**: C++/C#, Visual Script, prefabs; .NET SDK 10, roadmap semanal. Importa `.fbx`, `.gltf`, `.glb`, `.obj`, `.blend`, com LODs/colisão. https://docs.flaxengine.com/manual/graphics/models/import.html
- **2026/portabilidade/lock-in**: 1.13.7014.1 (04/11), push 18/11, ~7,1k estrelas; desktop/mobile/Web/consoles; projeto individual. Lock-in alto (royalty, foro). https://github.com/FlaxEngine/FlaxEngine/releases

| C | Nota | Justificativa |
|---|---:|---|
|C1 inv|4| fonte disponível, mas EULA proprietária e fork público vedado |
|C2 inv|7*| headless e replicação/RPC/autoridade oficiais; sem handshake |
|C3 inv|6| prefabs e pacotes, sem padrão Núcleo+Módulos versionado |
|C4|7| Visual Script, física e sample de lobby cobrem os desafios |
|C5|6| toolchain C++/.NET mais pesada que Lua |
|C6|6*| C++ moderno e `-null`; sem benchmark `[NV]` |
|C7|8| importação oficial de GLB/glTF |
|C8|8| releases frequentes e roadmap; dependência individual |
|C9|8| amplitude de plataformas e editor multiplataforma |
|C10|4| royalty, notificação e jurisdição encarecem a saída |

## O6 — Stride

- **Licença/código**: **MIT** (GitHub `license=MIT` e site oficial), .NET Foundation; fork/redistribuição livres, sem EULA/conta/royalty. https://www.stride3d.net/download/ · https://github.com/stride3d/stride
- **Headless**: `Stride.ClientServerSample` — "servidor headless que roda só física, sem tick" — nos recursos oficiais; amostra comunitária. https://doc.stride3d.net/en/community-resources/networking.html
- **Rede**: sem netcode no core; LiteEntitySystem (ECS autoritativo, rollback), StrideNet, Riptide. https://github.com/stride3d/stride-docs/blob/master/en/community-resources/networking.md
- **Editor/assets**: Game Studio (WPF) **só Windows**; 4.4 via Proton, rewrite Avalonia, CLI `stride`. Assimp: **issue #1666 aberta (31/10/2026)** — `.glb` com texturas embarcadas falha; workaround `.gltf`/OBJ/FBX; sem correção na 4.4 `[NV]`. https://github.com/stride3d/stride/issues/1666
- **2026/portabilidade/lock-in**: 4.4 (09/10) com Vulkan/D3D12, SPIR-V, Native AOT; 4.4.3 (17/11), ~8k estrelas; runtime multiplataforma recuperado, build Linux/macOS igual ao Windows. Lock-in baixo, atrito .NET. https://www.stride3d.net/blog/announcing-stride-4-4/

| C | Nota | Justificativa |
|---|---:|---|
|C1 inv|10| MIT canônico, .NET Foundation, zero permissão |
|C2 inv|6*| headless demonstrado; netcode é terceiro/DIY |
|C3 inv|7| packages e `Assets` tipado; sem hash no handshake |
|C4|6| editor e C# bons; sincronia é trabalho próprio |
|C5|7| C# acessível; editor Windows-only e docs em transição |
|C6|6*| Vulkan/D3D12 e AOT; sem benchmark `[NV]` |
|C7|5*| GLB embarcado quebrado na issue aberta |
|C8|8| release grande em 2026 e atividade diária |
|C9|7| runtime cross-platform; editor preso ao Windows |
|C10|8| MIT sem serviços obrigatórios; migração média |

## Síntese

Stride (Σ 71,1; 10/6/7) lidera pela soberania MIT, mas falha em C2 e no GLB. Defold (Σ 67,9; 7/5/6) é o caminho mais curto ao protótipo, porém licença não-OSI e sem netcode autoritativo penalizam os invioláveis. Flax (Σ 62,7; 4/7/6) tem o netcode mais completo, mas a EULA fere o Pilar I. Nenhum entrega autoritativo+handshake pronto.

## Nota de superação (2026-11-19, append-only)

> **Nota de superação (2026-11-19) — NV-A03 (Stride #1666):** a issue permanece `open` após 4.4.3 e nenhum fix foi registrado na issue ou nas notas de release; é inferência documental (estado aberto + ausência de registro de correção), não teste empírico — nenhum build/import foi executado. A mudança de build do Assimp na 4.4.2 tem escopo declarado em offsets de ossos e não é apresentada como correção de texturas incorporadas. Fontes: "Issue 1666 — glb import (API)" (https://api.github.com/repos/stride3d/stride/issues/1666), acesso 19/11/2026; "Issue 1666 comments/timeline (API)" (https://api.github.com/repos/stride3d/stride/issues/1666/comments · https://api.github.com/repos/stride3d/stride/issues/1666/timeline), acesso 19/11/2026; "Stride 4.4 release notes" (https://raw.githubusercontent.com/stride3d/stride-docs/master/en/release-notes/4.4.md), atualizado 17/11/2026. O corpo datado acima permanece como registro histórico.

> **Nota de superação (2026-11-19) — NV-A04 (manutenção Stride):** projeto ativo, não arquivado; cadência 4.4.0 (09/10/2026) → 4.4.1 (17/10/2026) → 4.4.2 (13/11/2026) → 4.4.3 (17/11/2026), com porte do editor para Avalonia em curso e CI em matriz completa nas plataformas. Fontes: "Repository: stride3d/stride (API)" (https://api.github.com/repos/stride3d/stride), acesso 19/11/2026; "Releases · stride3d/stride (API)" (https://api.github.com/repos/stride3d/stride/releases?per_page=4), acesso 19/11/2026; "Announcing Stride 4.4" (https://www.stride3d.net/blog/announcing-stride-4-4/), 09/10/2026. O corpo datado acima permanece como registro histórico.

> **Nota de superação (2026-11-19) — NV-A08 (conta Defold):** não é necessária conta Defold para baixar, instalar e usar o editor/motor; o fluxo oficial é inteiramente local (baixar, descompactar, executar). Ressalva: contas periféricas (fórum, Asset Portal, build servers) não foram auditadas — o fato cobre apenas download/instalação/uso do motor e editor, alcance do `[NV]` original. Fontes: "Download Defold" (https://defold.com/download/), acesso 19/11/2026; "Installing Defold" (https://defold.com/manuals/install/), acesso 19/11/2026; "Defold engine and editor FAQ" (https://defold.com/faq/faq/), acesso 19/11/2026. O corpo datado acima permanece como registro histórico.
