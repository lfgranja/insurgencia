# Alternativas — O13, forks (bifurcações) do Godot, web e opção não-engine

Data: 19/11/2026. Notas na escala 0–10 de `../00-criterios.md`; C1–C3 são invioláveis. Sem protótipo ou benchmark: notas são documentais e C6 usa proxy `[NV]`. Termos estrangeiros são explicados na 1ª ocorrência.

## Matriz de notas sugeridas

| Critério (peso) | O13 macroquad + tokio | F1 Redot (fork do Godot) | F2 Three.js + Node | F3 Não-engine: Godot + servidor Rust/Go |
|---|---|---|---|---|
| C1 (15) | 9 — Apache-2.0 + MIT, OSI; sem EULA (contrato de usuário final)/SDK (kit de desenvolvimento) proprietário | 7* — MIT; nome de fork depende de marca (`licencas_lockin.md:24`, política de marca 21/07/2025, confiança Alta) | 8* — MIT; risco transitivo do npm `[NV]` | 9 — MIT + MIT/BSD-3-Clause |
| C2 (15) | 5* — tokio headless (sem janela/GPU), mas sem rede: snapshots (fotos de estado) do zero | 7 — herda `--headless`/`dedicated_server` | 5 — Node headless; three.js só renderiza | 8* — servidor próprio, validação e snapshots; sem serviço central |
| C3 (12) | 3 — sem loader declarativo; recompilar | 7 — herda `ResourceLoader`/PCK | 4 — regras e hash manuais | 8* — JSON/`.cfg` + SHA-256 no handshake |
| C4 (10) | 3 — sem editor; janela de 5 s manual | 7 — editor herdado | 4 — editor só de cena | 7 — editor Godot; ferramentas no servidor |
| C5 (10) | 5 — API simples, mas tudo do zero | 7 — curva do Godot, docs menores | 3 — muito código até jogar | 5 — duas linguagens; perde `MultiplayerSynchronizer` |
| C6 (8) | 6* — eficiente, sem benchmark `[NV]` | 5* — sem benchmark `[NV]` | 5* — event loop, sem benchmark `[NV]` | 8* — leve, sem benchmark `[NV]` |
| C7 (8) | 5* — glTF via crate de terceiro `[NV]` | 8 — glTF/CC0 herdados | 8 — `GLTFLoader` oficial | 8 — Godot importa glTF/CC0 |
| C8 (8) | 5 — 4,7k estrelas, inativo desde 08/2026; tokio 9 | 5* — 6,2k estrelas, fundação jovem `[NV]` | 9 — 119k estrelas, ativo | 9 — pilhas maduras |
| C9 (7) | 8 — desktop/web/mobile | 7 — herda exportação | 6 — web nativo; desktop via camada | 8 — cliente + servidor Linux |
| C10 (7) | 5 — duas pilhas, substituível | 4 — migração do upstream | 4 — lock-in (dependência) do npm | 6 — dois toolchains; servidor trocável |

## Vereditos e motivos

**O13 macroquad + tokio — EXCLUÍDA como engine-base.** C3=3 e C4=3 em critérios invioláveis/alta: sem editor nem conteúdo declarativo. Macroquad é biblioteca de renderização com primitivos 2D/3D, não engine; útil só como cliente leve se Rust virar padrão. Fontes: https://github.com/not-fl3/macroquad · https://raw.githubusercontent.com/not-fl3/macroquad/master/README.md · https://docs.rs/macroquad/latest/macroquad/models/index.html · https://github.com/tokio-rs/tokio · https://raw.githubusercontent.com/tokio-rs/tokio/master/LICENSE · https://api.github.com/repos/not-fl3/macroquad (`pushed_at` 2026-08-18).

**F1 Redot — EXCLUÍDO como substituto do Godot *upstream* (original).** MIT e ativo, mas nenhum ganho em C1–C3 e risco em C8/C10: fork divide plugins e roadmap; usar "Godot" no nome exige autorização da fundação (`licencas_lockin.md:24`, política de marca 21/07/2025, confiança Alta). Herda só rede básica (`StreamPeer`, UDP/HTTP), sem replicação pronta. Fontes: https://github.com/Redot-Engine/redot-engine · https://raw.githubusercontent.com/Redot-Engine/redot-engine/master/LICENSE.txt · https://docs.redotengine.org/en/About/list_of_features · https://www.redotfoundation.org/ · https://redot-website.pages.dev/blog/redot-why-we-forked.

**F1b Proprietários — EXCLUÍDOS (C1=0).** GodotSteam depende do SDK Steamworks; Nakama/Photon/PlayFab impõem servidor gerenciado e EULA. Termos exatos `[NV]`. Nenhum outro fork com tração equivalente em fonte primária `[NV]`. Fontes: https://godotsteam.com/getting_started/introduction/ · https://store.godotengine.org/asset/godotsteam/godotsteam-gdextension/.

**F2 Three.js + Node — EXCLUÍDA como engine-base.** C3=4 e C4=3: o editor oficial edita cena, sem lógica de Desafio, e não há camada de rede. Opção real só para cliente web futuro, nunca para o Núcleo. Fontes: https://github.com/mrdoob/three.js · https://raw.githubusercontent.com/mrdoob/three.js/dev/LICENSE · https://threejs.org/editor/ · https://threejs.org/docs/pages/GLTFLoader.html · https://github.com/nodejs/node/blob/main/LICENSE.

**F3 Não-engine — OPÇÃO REAL (plano B).** Manter Godot para cliente/editor e trocar só o servidor por headless em tokio (MIT) ou Go (BSD-3-Clause), com protocolo próprio e Módulos orientados a dados. Preserva C1–C3 e melhora C6/determinismo, mas custa C5/C10 (duas linguagens, perda do `MultiplayerSynchronizer`). Adotar se o headless do Godot falhar C6 ou se replay determinístico virar exigência; não substitui o Godot-first do GDD §14.8. Fontes: https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_dedicated_servers.html · https://raw.githubusercontent.com/godotengine/godot/4.7.2-stable/LICENSE.txt · https://raw.githubusercontent.com/tokio-rs/tokio/master/LICENSE · https://raw.githubusercontent.com/golang/go/master/LICENSE.

## Nota de superação (2026-11-19, append-only)

> **Nota de superação (2026-11-19) — NV-A01 (marca Godot / fork Redot):** a política de marcas v. 21/07/2025 exige autorização escrita da Godot Foundation fora das quatro hipóteses descritivas automáticas — "made with Godot"; produto/serviço compatível com Godot; evento ou publicação sobre o Godot; brinde (camiseta/caneca) não vendido — e exige licença customizada para usar a marca como nome de motor comercial ou fork comercial; a MIT do código não concede direito sobre a marca, e "Godot can't be sold or purchased by any company". Fontes: "GODOT TRADEMARK POLICY AND LICENSE" (https://godot.foundation/policies-and-procedures/trademark-policy), v. 21/07/2025, acesso 19/11/2026; "Governance – Godot Engine" (https://godotengine.org/governance), acesso 19/11/2026; "Godot Foundation Update December 2024" (https://godotengine.org/article/godot-foundation-update-dec-2024), 02/12/2024, acesso 19/11/2026. O corpo datado acima permanece como registro histórico.

> **Nota de superação (2026-11-19) — NV-A02 (licença MIT):** o motor/editor Godot é MIT/Expat, sem EULA nos canais oficiais (afirmação negativa verificada nas fontes primárias oficiais, não prova sobre lojas de terceiros); única obrigação ao redistribuir é incluir o aviso de copyright e o texto da licença — aplicável também às licenças MIT/BSD-3-Clause das pilhas Rust/Go em F3. Fontes: "godotengine/godot — LICENSE.txt" (https://raw.githubusercontent.com/godotengine/godot/master/LICENSE.txt), acesso 19/11/2026; "License – Godot Engine" (https://godotengine.org/license), acesso 19/11/2026; "Complying with licenses — Godot Engine docs" (https://docs.godotengine.org/en/stable/about/complying_with_licenses.html), acesso 19/11/2026. O corpo datado acima permanece como registro histórico.
