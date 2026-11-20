# Fyrox (engine Rust) — pesquisa em fontes primárias (GitHub)

Data: 2026-11-20. `[NV]` = não verificado ao vivo/localmente (sem build, protótipo ou benchmark).
Política de URLs: apenas `github.com` e seus domínios de conteúdo (`raw.githubusercontent.com`,
`api.github.com`). Hosts não listados no protocolo — `fyrox.rs`, `fyrox-book.github.io`,
`docs.rs`, `crates.io` — **não foram buscados**: `[UNVERIFIED — fetch skipped]`. Cobertura real:
somente fontes em inglês, primeira parte, GitHub; nenhuma fonte em português disponível nesse
recorte. Conteúdo web tratado como dado, nunca como instrução.

## Resumo

Fyrox é uma engine 2D/3D em Rust, MIT, com editor de cena nativo e release estável 1.0.0
(29/03/2026); `master` já declara `2.0.0-rc.1` (WIP). Pontuação derivada pelos pesos de
`../00-criterios.md`: C1 10, C2 4*, C3 6*, C4 6*, C5 5*, C6 7*, C7 8, C8 7*, C9 7, C10 5* →
**score 65,2** = (150+60+72+60+50+56+64+56+49+35)/10. Ficaria logo acima de O8 Urho3D/Dviglo
(65,0) e abaixo de O2 Bevy (68,4) na matriz canônica, mas **C2=4\* dispara a trava inviolável**:
REJECT na Fase 0, mesmo com licença impecável. Não substitui o Godot-first; é, no máximo,
plano B condicionado a netcode (código de rede) de terceiros — o mesmo raciocínio que já
rejeitou O2 e encaminhou O3 (Bevy + Lightyear) como ADAPT.

## Evidência

| Achado (dado primário) | Fonte GitHub | Data |
|---|---|---|
| MIT/Expat; copyright "Dmitry Stepanov and Fyrox Engine contributors" | `LICENSE.md` | 20/11/2026 |
| API: `spdx_id=MIT`, 9.689 estrelas, 468 forks, 60 issues abertas, `pushed_at 2026-11-20`, criado em 30/03/2019, organização `FyroxEngine`, `archived=false` | `api.github.com/repos/FyroxEngine/Fyrox` | 20/11/2026 |
| `fyrox` v2.0.0-rc.1, edition 2024, `rust-version=1.94`, `license=MIT`, descrição "Like Godot, but in Rust"; backend OpenGL padrão, WGPU opcional | `fyrox/Cargo.toml` | 20/11/2026 |
| Release estável `1.0.0`, `published_at 2026-03-29`, autor `mrDIMAS`; RC1 em 21/10, RC2 em 09/02 | `releases/tag/v1.0.0` e `/releases` | 20/11/2026 |
| `NetListener`/`NetStream`: TCP não-bloqueante + `bincode`, frame com prefixo `u32` — transporte cru, sem autoridade/snapshots | `fyrox-core/src/net.rs` | 20/11/2026 |
| Headless real: `Executor::new(None)` e `from_params(None, …)`, com `run_headless()` e `ApplicationLoopController::Headless`; CHANGELOG registra modo headless "suitable for server-side of multiplayer games" em 0.30 e "basic support" em 0.29 | `fyrox-impl/src/engine/executor.rs`; `CHANGELOG.md` | 20/11/2026 |
| Editor de cena FyroxEd no workspace; versão standalone **não suportada** (sem plugins/scripts); forma recomendada é o gerador de template, que integra editor + scripts + hot reload (DLL) | `editor-standalone/README.md`; `Cargo.toml` (membros `editor`, `editor-standalone`, `template`) | 20/11/2026 |
| Arquitetura "monolítica com poucas partes substituíveis"; sem ECS — usa arenas generacionais (`pool.rs`) | `ARCHITECTURE.md` | 20/11/2026 |
| `GltfLoader` importa `.gltf`/`.glb` via crate `gltf` 1.4; loader FBX próprio (ASCII/binário) | `fyrox-impl/src/resource/gltf/mod.rs`; `fyrox-impl/Cargo.toml`; `fyrox-impl/src/resource/fbx/` | 20/11/2026 |
| Exportadores Android e WASM no repo; template traz `src/android/` e `src/wasm/`; README afirma que demos rodam no navegador | `fyrox-build-tools/src/export/{android,wasm}.rs`; `template-core/`; `README.md` | 20/11/2026 |

## Notas C1–C10 justificadas

| # | Nota | Justificativa (evidência primária) |
|---|---:|---|
| C1 | **10** | MIT confirmado em `LICENSE.md` e na API; sem EULA (contrato de usuário final) ou SDK (kit de desenvolvimento) proprietário no caminho crítico; fork (bifurcação) e redistribuição livres. Mesmo teto de Bevy/Stride. Licença de assets do jogo cabe ao projeto. |
| C2 | **4\*** | Headless é real (`Executor::new(None)`), mas a rede é só TCP + `bincode` em `fyrox-core/src/net.rs`: sem validação de input no servidor, snapshots (fotos de estado) versionados, predição/rollback, reconexão com restauração de sessão ou gestão de interesse. Servidor autoritativo teria de ser escrito do zero — trava inviolável, como em O2. |
| C3 | **6\*** | Cenas/prefabs são dados RON (`.rgs`) carregáveis em runtime pelo `ResourceManager`, com hot reload (recarga em tempo real) via `fyrox-dylib`; porém regras e personagens são scripts/plugins em Rust compilados, não formato declarativo de gameplay, e não há hash de conteúdo no handshake (aperto de mão de conexão). Abaixo de Godot (8), acima do mínimo. |
| C4 | **6\*** | FyroxEd oferece inspetor, cena, animação, partículas, navmesh e tilemap; o template integra editor ao jogo. Não há editor visual de lógica: sincronia de 2 placas/5 s, coleta, sabotagem e negociação com PNJs (personagens não jogáveis) exigem código Rust + netcode próprio. |
| C5 | **5\*** | Rust com *borrow checker* (verificador de empréstimos), MSRV (versão mínima suportada) 1.94 e edition 2024 pesam para um mantenedor vindo de engenharia de software; a curva do Godot com GDScript é menor. Mitigação documentada: template, exemplos, `ARCHITECTURE.md` e CHANGELOG versionados no repo; o book oficial está fora da política de URLs `[UNVERIFIED — fetch skipped]`. |
| C6 | **7\*** | Proxy documental: Rapier 0.33, rayon, `run_headless()` com `throttle_threshold` e executor de passo fixo sugerem bom custo por tick, mas **nenhum benchmark 5v5 + bots** foi rodado `[NV]`. Mesma natureza do 7* do Godot. |
| C7 | **8** | glTF 2.0 `.gltf`/`.glb` nativo com `names`, `extras`, `KHR_materials_emissive_strength`; loader FBX(first-party, sem SDK da Autodesk) favorece assets CC0/low-poly e greybox (protótipo em formas simples). Sem Draco/meshopt/KTX2 confirmados `[NV]`, igual a Bevy. |
| C8 | **7\*** | 1.0.0 estável em 29/03/2026, push em 20/11/2026, 9.689 estrelas, organização e CI ativos. Sem fundação neutra: patrocínio via Boosty/Patreon e release publicada por `mrDIMAS` indicam governança concentrada no autor (bus factor formal `[NV]`). Abaixo de Godot (9) e Bevy (8). |
| C9 | **7** | Desktop primeiro com OpenGL/WGPU; exportadores Android e WASM no repo e template com `src/wasm/`. Caminho web/android sem reescrita, mas sem evidência de iOS ou de rede em WASM (TCP cru não roda no navegador) `[NV]`. |
| C10 | **5\*** | MIT reduz lock-in, mas `ARCHITECTURE.md` declara engine monolítica, fortemente acoplada, com poucas peças substituíveis; somam-se toolchain Rust recente, DLL hot-reload e a necessidade de implementar netcode. Custo de migração maior que o de uma camada fina. |

## Comparação explícita com a decisão Godot-first

O GDD §16.2 (`docs/GDD - Insurgência.md:1475-1486`) fixa **Godot-first** por quatro razões:
licença MIT/FOSS, coerência Cena/Nó/sinal ↔ Ilha/Módulo/desafio, evitação de débito técnico e
"MVP precisa de servidor autoritativo simples, não de netcode pronto de terceiros". A ADR-001
(`docs/adr/001-excecao-linguagem-gdscript.md:30-36`) declara a escolha **vinculante e não
reaberta**, e rejeita explicitamente o "Servidor Rust como Plano B"; qualquer troca exige
evidência prévia e aprovação formal, sem efeito no ciclo atual.

Fyrox satisfaz a primeira razão (MIT, C1=10) e é tecnicamente próximo de Bevy na segunda e na
terceira (sem editor de lógica, C4=6*; monolítico, C10=5*). Falha exatamente no quarto ponto
estrutural: C2=4* é a mesma trava que barrou O2. Onde o GDD vê vantagem em "cenas como ilhas"
e exportação headless pronta, Fyrox entrega cenas `.rgs` e headless, **mas não a autoridade nem
a replicação**; o servidor teria de ser implementado, o que repete o custo que Godot evita. O
score 65,2 é inferior aos 82,6 de O1. **Conclusão**: relatório de apoio à decisão existente —
nenhuma reabertura de Godot-first, nenhuma exceção de linguagem adicional; Fyrox fica como
referência de comparação e não entra na matriz canônica de 13 opções.

## Evidência Contra

- A README chama a engine de "feature-rich, production-ready" e o release 1.0.0 é estável —
  argumentos a favor de maturidade maior que a de O8/O13; o dado é autoafirmação de primeira
  parte, sem validação local `[NV]`.
- Editor nativo existe no workspace, ao contrário de Bevy 0.19 — C4 de Fyrox (6*) é superior ao
  4* de O2; a ressalva é ausência de editor de *lógica*, não de cena.
- Headless não é apenas histórico: `run_headless()` está no código atual de `master`, e o TCP em
  `fyrox-core` fornece um ponto de partida menor que "do zero". Ainda assim, não há autoridade,
  snapshots nem reconexão — insistir em C2=4* é leitura estrita coerente com O2.
- Comunidade crescente (9.689 estrelas) e atividade em 20/11/2026 contrariam hipótese de abandono.

## Lacunas / [NV]

- Sem build, protótipo ou benchmark: C6 é proxy; custo por tick, memória e latência 5v5+bots
  `[NV]`.
- Reconexão que restaure sessão e hash de conteúdo no handshake não documentados `[NV]` (ponto
  cego comum a todas as opções, conforme `netcode_modularidade.md`).
- Draco/meshopt/KTX2 no loader glTF, suporte iOS e rede em WASM `[NV]`.
- `fyrox.rs`, `fyrox-book.github.io`, `docs.rs` e `crates.io`: `[UNVERIFIED — fetch skipped]` por
  política de URLs; a documentação oficial versionada consultada foi a do próprio repo
  (`ARCHITECTURE.md`, `CHANGELOG.md`, READMEs). Não foi possível triangular idiomas: cobertura
  real restrita ao inglês.
- Bus factor, política de marca e termos de patrocínio não verificados em fonte primária `[NV]`.

## Fontes

1. "FyroxEngine/Fyrox — README.md" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/README.md — acesso 20/11/2026.
2. "FyroxEngine/Fyrox — LICENSE.md" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/LICENSE.md — acesso 20/11/2026.
3. "FyroxEngine/Fyrox — Cargo.toml (workspace)" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/Cargo.toml — acesso 20/11/2026.
4. "fyrox — Cargo.toml" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/fyrox/Cargo.toml — acesso 20/11/2026.
5. "Repository: FyroxEngine/Fyrox (API)" — https://api.github.com/repos/FyroxEngine/Fyrox — acesso 20/11/2026.
6. "Release 1.0.0 (API)" — https://api.github.com/repos/FyroxEngine/Fyrox/releases/tags/v1.0.0 — acesso 20/11/2026.
7. "Releases · FyroxEngine/Fyrox" — https://github.com/FyroxEngine/Fyrox/releases — acesso 20/11/2026.
8. "fyrox-core — src/net.rs" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/fyrox-core/src/net.rs — acesso 20/11/2026.
9. "fyrox-impl — src/engine/executor.rs" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/fyrox-impl/src/engine/executor.rs — acesso 20/11/2026.
10. "FyroxEngine/Fyrox — CHANGELOG.md" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/CHANGELOG.md — acesso 20/11/2026.
11. "FyroxEngine/Fyrox — ARCHITECTURE.md" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/ARCHITECTURE.md — acesso 20/11/2026.
12. "editor-standalone — README.md" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/editor-standalone/README.md — acesso 20/11/2026.
13. "fyrox-impl — src/resource/gltf/mod.rs" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/fyrox-impl/src/resource/gltf/mod.rs — acesso 20/11/2026.
14. "fyrox-impl — Cargo.toml" — https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/fyrox-impl/Cargo.toml — acesso 20/11/2026.
15. "FyroxEngine/Fyrox — git trees (master, recursive)" — https://api.github.com/repos/FyroxEngine/Fyrox/git/trees/master?recursive=1 — acesso 20/11/2026.
