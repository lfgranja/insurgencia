# Recomendação final — avaliação comparativa de engines (Insurgência)

Data: 19/11/2026. Protocolo 019/040 — pesos C1 15, C2 15, C3 12, C4 10, C5 10, C6 8, C7 8, C8 8, C9 7, C10 7 (Σ=100); score = Σ(nota × peso)/10. Notas e scores provenientes dos oito relatórios em `docs/avaliacao-engines/pesquisa/`; nenhum protótipo ou benchmark foi executado.

**Trava inviolável (declarada conforme `00b-contrato-escrita.md`).** Nota < 5 em C1 (soberania/licença), C2 (servidor autoritativo) ou C3 (modularidade) **impede adoção na Fase 0**, mesmo com score alto. C4–C10 não compensam. Nenhuma recomendação abaixo viola esta trava.

## Revisão da decisão Godot-first do GDD §14.8

**Veredito: CONFIRMA COM RESSALVAS.**

A pesquisa confirma o Godot 4.7.2 como base: é o único candidato que satisfaz simultaneamente os três invioláveis com folga (C1=9, C2=8, C3=8), tem o maior score (82,6) e o menor custo de migração — cenas já são ilhas, GDScript não exige build e o editor existe. Nenhuma alternativa supera essa combinação: O3 (Bevy + Lightyear, 77,1) é melhor em C2 (9) mas não entrega editor nem conteúdo declarativo sem código, e exige recriar todo o ativo de autoria; O9 Panda3D (75,0) tem C3 sólido mas pilha opinativa e sem editor moderno; O5 Flax reprova em C1 (4); O2 Bevy puro reprova em C2 (4). Não há evidência que **refute** o §14.8.

As ressalvas são exatamente os riscos documentados, e impedem um "CONFIRMA" pleno:

| Risco | Evidência | Consequência prática |
|---|---|---|
| Protocolo de rede instável | API de alto nível é detalhe de implementação que "may change without notice" e **não interopera** com servidor não-Godot; RPC exige mesmo `NodePath` e checksum de todos os RPCs (`class_scenemultiplayer`) | Congelar versão 4.7.2 e o contrato de RPC no manifesto; nunca depender da API para falar com outro servidor |
| `allow_object_decoding` | Default `false`, mas `true` permite executar código recebido (RCE) | Manter `false` e cobrir com teste de regressão; decoding de objetos é vetado no cliente |
| NAT/UDP | API de alto nível usa **somente UDP** (ENet modificado com IPv6) → exige encaminhamento de porta UDP; web só WebSocket/WebRTC, sem TCP/UDP cru | Plano de port forwarding + fallback WebRTC; teste inicial na Fase 0, aprofundado no spike C (Fase 1) |
| Ausência de snapshot versionado | Sem primitiva documentada de snapshot versionado nem hash de conteúdo no handshake `[NV]`; GDD §15.9 exige versão + hash no handshake **já no MVP** | Escrever à mão no payload de `auth_callback`/`send_auth`; ver Fase 0 |

Ou seja: o engine é confirmado, mas a **camada C2/C3 que o GDD exige não vem pronta** — vem como primitivas. A decisão anterior não é mantida por inércia: ela sobrevive porque nenhuma alternativa oferece ganho líquido nos invioláveis sem destruir o fluxo de autoria. Fontes: `https://docs.godotengine.org/en/stable/tutorials/networking/high_level_multiplayer.html`, `https://godotengine.org/license/`, `https://godotengine.org/download/archive/`, relatórios `godot.md`, `netcode_modularidade.md`, `bevy.md`, `alternativas.md`.

## Fase 0 — protótipo/MVP (próximos 3 meses)

Trava aplicada a todas as linhas: nota < 5 em C1/C2/C3 → sem adoção. O1–O13 são as 13 opções canônicas da matriz; F1/F2/F3 vêm de `alternativas.md` e constam aqui como opções suplementares (F3 é o Plano B), usando os mesmos rótulos.

| # | Opção (score) | C1/C2/C3 | Veredito | Condição operacional concreta |
|---:|---|---|---|---|
| 1 | **O1 Godot 4.7.2** (82,6) | 9/8/8 | **ADOPT** | Escrever à mão no servidor headless: (a) validação autoritativa de **todo** `@rpc("any_peer")` — checar `get_remote_sender_id()`, limites e estado antes de mutar; (b) handshake com `protocol_version` + SHA-256 do Módulo via `auth_callback`/`send_auth`, recusando divergência com erro legível (GDD §15.9); (c) camada de snapshot versionado e reconstrução de sessão; (d) `allow_object_decoding=false` fixo. Fixar 4.7.2 e os templates de exportação no manifesto. |
| 2 | **F3 Godot cliente + servidor custom (tokio/Go)** (76,9) | 9/8/8* | **DEFER** | Plano B; não acionar agora. Manter sob monitoramento até o benchmark de Fase 1. |
| 3 | **O3 Bevy + Lightyear** (77,1) | 10/9/7* | **ADAPT** | Não adotar Bevy como engine na Fase 0. ADAPTar os padrões do Lightyear — handshake com `protocol_id`, *input buffer* (fila de entrada) por *tick* (passo de simulação), snapshots versionados e *interest management* (filtro de visibilidade) — para a camada escrita à mão no Godot (linha 1), reescritos em GDScript, sem copiar código Rust. C3=7* é a nota oficial da matriz (`bevy.md`, relatório por engine); a leitura estrita de `netcode_modularidade.md` (C3=4, protocolo compilado trava conteúdo declarativo) não altera o veredito: mesmo assim exigiria camada de dados própria. Spike de engine na Fase 1. |
| 4 | **O9 Panda3D** (75,0) | 9/8/8 | **DEFER** | Monitorar apenas; sem editor visual moderno e pilha AI/DOG/DC opinativa. Reavaliar se ganhar editor mantido. |
| 5 | **O7 libGDX** (72,3) | 9/6/6 | **DEFER** | Framework de baixo nível: Núcleo/Módulos e netcode (TCP sem UDP) seriam construídos do zero. Sem ganho sobre O1. |
| 6 | **O6 Stride** (71,1) | 10/6/7 | **AVOID** | C1 canônico, mas netcode é de terceiros e `.glb` com texturas embarcadas quebra na issue aberta #1666; editor só Windows. |
| 7 | **O2 Bevy 0.19.1** (68,4) | 10/**4***/7* | **REJECT** | Trava: C2=4* (sem validação/snapshots/reconexão nativos). Só entra como O3 via spike. |
| 8 | **O4 Defold** (67,9) | 7/5*/6 | **REJECT** | Embora ≥5 na matriz, licença não-OSI (cláusula 4(a) veta comercializar o motor) fere o Pilar I; nenhum ganho em C1–C3. |
| 9 | **F1 Redot** (65,5) | 7*/7/7 | **AVOID** | Fork não agrega C1–C3, divide plugins/roadmap e o uso de "Godot" no nome de fork comercial exige autorização escrita (fora das quatro hipóteses automáticas; motor/fork comercial requer licença de marca customizada) — "GODOT TRADEMARK POLICY AND LICENSE" (https://godot.foundation/policies-and-procedures/trademark-policy), v. 21/07/2025, acesso 19/11/2026; "Governance – Godot Engine" (https://godotengine.org/governance), acesso 19/11/2026; "Godot Foundation Update December 2024" (https://godotengine.org/article/godot-foundation-update-dec-2024), 02/12/2024, acesso 19/11/2026. Ressalva: restrição de marca, não de licença de software. |
| 10 | **O8 Urho3D / Dviglo** (65,0) | 8/8/7 | **AVOID** | Replicação madura, mas Urho3D arquivado (2023) e Dviglo parado em 03/2024 → C8=3, C10=4. |
| 11 | **O5 Flax** (62,7) | **4**/7*/6 | **REJECT** | Trava: C1=4; EULA, usuário único, royalty 4% e foro na Polônia. |
| 12 | **O10 MonoGame** (59,6) | 8/**4***/**4** | **REJECT** | Trava dupla em C2/C3; runtime headless inexistente, sem rede embutida. |
| 13 | **F2 Three.js + Node** (55,9) | 8/5/**4** | **REJECT** | Trava em C3; útil só como cliente web futuro, nunca como Núcleo. |
| 14 | **O13 macroquad + tokio** (54,5) | 9/5*/**3** | **REJECT** | Trava em C3 (sem loader declarativo) e C4=3; biblioteca de render, não engine. |
| 15 | **O12 Haxe / OpenFL** (53,4) | 8/6/**4** | **REJECT** | Trava em C3; 2D e compilador Haxe GPL-2.0-or-later. |
| 16 | **O11 LÖVE** (52,4) | 8/5/5 | **REJECT** | C1–C3 passam, mas é 2D (C4=2) — incompatível com as ilhas 3D low-poly/glTF. |
| 17 | **O10 FNA** (46,7) | 8/**3***/**4** | **REJECT** | Trava dupla; toolchain legado e mantenedor único. |

`*` = evidência parcial/`[NV]`. Scores documentais, não medidos.

## Fase 1 — pós-protótipo

| Spike / marco | O que medir | Critério de manter / trocar | Prazo |
|---|---|---|---|
| **A — carga no headless Godot** | Partida sintética 10 jogadores + 40 Súditos a 30 Hz e 60 Hz: p95 do custo por tick, perda de pacote tolerada, memória (GDD §14.8 pede essa medição) | **Manter** se p95 de tick ≤ 16,7 ms a 30 Hz; **trocar/Plano B** se p95 > 33 ms ou se a reconstrução de sessão exceder 2 s | Meses 4–5; decisão no **M5** |
| **B — reconsiderar Bevy via Lightyear** | Subir servidor headless Bevy 0.19.1 + Lightyear 0.28–0.30 com sincronia de 2 jogadores/5 s: (a) tempo até tick autoritativo; (b) dias para handshake versionado + snapshot versionado; (c) loader declarativo de Módulos **sem recompilar** (serde/JSON + recarga) | Só reabrir o GDD §14.8 se o spike demonstrar C3 ≥ 5 efetivo (Módulos sem recompilar) **e** reduzir esforço de rede o suficiente para compensar a recriação de cenas/UI; caso contrário, manter O1 | Meses 4–6; veredito no **M6** |
| **C — resiliência NAT/UDP** | Reconexão após 5 s de perda; comportamento do ENet atrás de NAT simétrico e fallback WebRTC no web | Manter se < 2 s para reconstruir estado e se o fallback web funcionar; acionar Plano B se o protocolo de alto nível mudar de forma incompatível | Meses 4–5; anexo ao **M5** |

**Gatilhos de reavaliação (somente estes):** (i) Godot falhar o spike A; (ii) replay/lockstep determinístico tornar-se exigência comprovada em playtest para o anti-cheat do §14.8; (iii) mudança de licença/headless/carga declarativa do Godot; (iv) uma alternativa atingir C1–C3 ≥ 5 com editor e custo de migração favoráveis **e** o GDD §14.8 ser formalmente reaberto. Qualquer troca exige protótipo que demonstre a vantagem que a motivou (critérios de aceitação do protocolo, seção 8.3 do documento de avaliação anterior).

## Plano B

**Opção:** F3 — manter Godot como cliente/editor e substituir apenas o servidor por headless em **tokio** (MIT) ou **Go** (BSD-3-Clause), com protocolo próprio e Módulos orientados a dados (JSON/`.cfg` + SHA-256 no handshake). Scores: C1=9, C2=8*, C3=8*, C6=8*, mas C5=5 (perda do `MultiplayerSynchronizer`) e C10=6 (duas linguagens/toolchains). Preserva C1–C3 e melhora determinismo e custo por tick; **não substitui o Godot-first** — a engine do cliente continua Godot.

**Quando acionar:**

1. Spike A falhar: p95 de tick > 33 ms a 30 Hz com 10 jogadores + 40 Súditos, ou perda de snapshots acima do limiar medido.
2. Replay determinístico virar requisito do anti-cheat (§14.8) e o servidor Godot não entregar a reprodutibilidade exigida.
3. A API de alto nível do Godot mudar ("may change without notice") a ponto de quebrar forks ou a interoperabilidade entre versões.
4. Federação com servidores não-Godot for exigida (a API atual não interopera).

**Custo a aceitar explicitamente:** escrever validação, snapshots versionados, filtro de visibilidade e reconexão do zero em Rust/Go; perder `MultiplayerSynchronizer`; duas linguagens no time. Manter o Núcleo MIT e o protocolo versionado; auditar licenças antes de incorporar código AGPL ao servidor.

## 80/20 — próximo passo

| # | Ação | Prazo | Critério de sucesso mensurável |
|---:|---|---|---|
| 1 | Camada de validação autoritativa no servidor Godot, ADAPTando os padrões do Lightyear (*input buffer* por *tick*, *interest management*) + `allow_object_decoding=false` fixo | Semanas 1–2 | 100% dos RPCs de input validados via `get_remote_sender_id()` antes de mutar estado; teste de pacote adversarial no `--headless` resulta em rejeição, nunca em mutação |
| 2 | Handshake versionado (protocolo + SHA-256 do Módulo) no payload de autenticação | Semanas 2–4 | Cliente com hash divergente é recusado em 100% dos casos; teste automatizado rodando em `godot --headless` no CI |
| 3 | Spike de carga 10 jogadores + 40 Súditos e teste NAT/UDP | Semanas 4–12 | Relatório com p95 de tick a 30 e 60 Hz + decisão registrada; se p95 > 33 ms, acionar Plano B |

## Riscos e NV

- `[NV]` Nenhum protótipo, benchmark ou build foi executado: todos os scores são documentais e todas as notas C6 são proxies.
- `[NV]` Reconexão/restauração de sessão: nenhuma engine da amostra restaura estado completo sozinha; responsabilidade do jogo em todas.
- `[NV]` Snapshot versionado e hash de conteúdo no handshake do Godot: sem primitiva documentada; a solução via payload de auth é inferência a validar.
- `[NV]` Exploração prática de `allow_object_decoding=true` (RCE) não reproduzida localmente — apenas documentada.
- `[NV]` Estabilidade real da API de alto nível entre 4.7.2 e 4.8-beta; aviso "may change without notice" não foi medido.
- Matriz Lightyear↔Bevy confirmada: 0.28–0.30 → Bevy 0.19 e 0.31 → 0.20 (Rust 1.97.1) — "Lightyear — README.md" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/README.md), acesso 19/11/2026; "lightyear — crates.io API (dependências de 0.31.0)" (https://crates.io/api/v1/crates/lightyear/0.31.0/dependencies), acesso 19/11/2026; "Lightyear — Cargo.toml" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/Cargo.toml), acesso 19/11/2026. Book do Lightyear é WIP / "collection of notes", seção Examples incompleta — "book/src/introduction.md" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/book/src/introduction.md), acesso 19/11/2026; "book/src/SUMMARY.md" (https://raw.githubusercontent.com/cBournhonesque/lightyear/main/book/src/SUMMARY.md), acesso 19/11/2026. Ressalva: "4 versões por ciclo" é observação apenas do ciclo 0.19 (0.28.0, 0.29.0, 0.30.0, 0.30.1; 1–13 publicações por ciclo), não lei geral, e a UI de Releases está defasada em relação ao crates.io — "lightyear — crates.io API (60 versões)" (https://crates.io/api/v1/crates/lightyear), acesso 19/11/2026; "Releases · cBournhonesque/lightyear" (https://github.com/cBournhonesque/lightyear/releases), acesso 19/11/2026. Evidência documental; sem execução ou benchmark.
- `[NV]` Custo real de manutenção de um editor de Módulos declarativo; nenhuma engine o entrega pronto.
- `[NV]` Coleta, sabotagem, negociação com PNJs e sincronia 2 jogadores/5 s são lógica do jogo — nenhuma engine fornece prontas.
- Marca/licença do fork Redot: uso de "Godot" no nome de fork comercial exige autorização escrita (as quatro hipóteses automáticas não cobrem esse caso; licença de marca customizada necessária) — "GODOT TRADEMARK POLICY AND LICENSE" (https://godot.foundation/policies-and-procedures/trademark-policy), v. 21/07/2025, acesso 19/11/2026; "Governance – Godot Engine" (https://godotengine.org/governance), acesso 19/11/2026; "Godot Foundation Update December 2024" (https://godotengine.org/article/godot-foundation-update-dec-2024), 02/12/2024, acesso 19/11/2026. Ressalva: é política de marca, não licença de software; termos exatos dos serviços proprietários (GodotSteam, Nakama/Photon/PlayFab) continuam sem verificação nesta rodada (externos); C1 conforme `alternativas.md`.
- `[NV]` Custo de migração e viabilidade do replay determinístico não medidos; F3 é plano B documental, não protótipo.
