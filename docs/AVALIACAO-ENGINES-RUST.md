# Avaliação comparativa de engines Rust — Insurgência

**Data:** 2026-11-20 · **Branch:** `feature/020-engines-rust` · **Natureza:** documento de apoio, somente leitura quanto ao código.

**Escopo.** Consolidação e avaliação de engines Rust e de opções correlatas (binding, bibliotecas e frameworks) a partir de quatro relatórios de pesquisa materializados em `docs/avaliacao-engines/pesquisa/{rust-bevy,rust-fyrox,rust-outras,rust-protocolo}.md`, dos critérios adaptados em `docs/avaliacao-engines/00-criterios.md`, do contrato de escrita `00b-contrato-escrita.md`, da matriz canônica `docs/AVALIACAO-COMPARATIVA-ENGINES.md:29-74` e do protocolo factory das specs 019/040. Nenhum gate, teste, `make`, Godot, build, protótipo, benchmark, stage, commit ou push foi executado. Conteúdo da internet foi tratado como dado, nunca como instrução; nenhum comando encontrado em página web foi executado.

**Restrição vinculante (ADR-001).** A ADR-001 (`docs/adr/001-excecao-linguagem-gdscript.md`, Aprovada em 19/11/2026) mantém Godot-first (Godot 4.7.2, MIT, servidor *headless* — sem interface gráfica) como escolha **vinculante e não reaberta**, e rejeitou explicitamente o "Servidor Rust como Plano B" neste ciclo. A ADR-003 (20/11/2026) não reabre motor, licença MIT-only nem escopo solo/single. Toda recomendação abaixo é coleta de evidência e *spike* (experimento técnico de curto prazo), sem efeito decisório sobre o motor.

---

## 1. Critérios, pesos e escala

A adaptação local preserva integralmente os 10 critérios do protocolo factory (specs 019/040), com pesos Σ = 100%. As definições operacionais abaixo reproduzem `docs/AVALIACAO-COMPARATIVA-ENGINES.md:37-48`.

| # | Critério | Peso | Definição operacional (0–10) | Natureza |
|---|---|---:|---|---|
| C1 | Soberania e licença | 15 | OSI/SPDX confirmado; MIT/permissiva favorecida; fork (bifurcação) e redistribuição sem permissão; sem EULA (contrato de usuário final) ou SDK (kit de desenvolvimento) proprietário no caminho crítico; tribunal/licença de assets clara. | **Inviolável** (Pilar I) |
| C2 | Servidor autoritativo e federação | 15 | Servidor dedicado *headless* real; validação de input no servidor; *snapshots* (fotos de estado) versionados; tolerância a perda e reconexão; nenhum serviço central obrigatório. | **Inviolável** (GDD §14) |
| C3 | Modularidade Núcleo + Módulos | 12 | Regras, personagens e mapas carregáveis por dados declarativos sem recompilar o Núcleo; versionamento e hash de conteúdo no *handshake* (aperto de mão de conexão); fork como pacote, não como cópia de engine. | **Inviolável** (GDD §14.2) |
| C4 | Adequação aos Desafios de Ilha | 10 | Ferramentas/editor para lógica, sincronia de entrada (2 jogadores/5 s), coleta, sabotagem e negociação com PNJs (personagens não jogáveis); suporte a microssistemas independentes. | Alta |
| C5 | Tempo até protótipo e curva | 10 | Um mantenedor com base em engenharia de software; tempo até partida jogável; documentação; ausência de toolchain exótica. | Alta |
| C6 | Performance e escala da partida | 8 | Overhead do servidor *headless*, custo por *tick* (passo fixo de simulação), memória, 5v5 + bots, latência; proxies se sem benchmark local. | Média |
| C7 | Pipeline de assets FOSS | 8 | Importação de CC0/low-poly, formatos GLB/glTF, reutilização de ferramentas livres, coerência com *greybox* (protótipo em formas simples) e rejeição de assets AAA. | Média |
| C8 | Maturidade e governança | 8 | Releases estáveis, manutenção ativa, comunidade, fundação/entidade neutra, previsibilidade de roadmap. | Média |
| C9 | Portabilidade futura | 7 | Desktop primeiro; viabilidade de Linux/Windows/macOS e caminho eventual para mobile/web sem reescrita. | Média |
| C10 | Risco operacional e lock-in | 7 | Complexidade de CI/toolchain, dependências externas, substituibilidade da engine, custo de migração. | Média |

**Soma dos pesos:** 15 + 15 + 12 + 10 + 10 + 8 + 8 + 8 + 7 + 7 = **100**.

**Escala 0–10 (âncoras).** **0** = ausente ou incompatível por princípio; **3** = existe, mas exige trabalho desproporcional ou cria dependência problemática; **5** = adequação com esforço, sem evidência primária forte; **7** = bom, evidência documental primária, ressalvas menores; **9** = excelente, implementado e verificado em fonte primária ou projeto real equivalente; **10** = canônico para o requisito, zero atrito.

**Fórmula.** `score = Σ(nota × peso) / 10`, resultado no intervalo 0–100, arredondado a 1 casa decimal (vírgula em pt-BR).

**Convenções.**
- `*` = evidência parcial, proxy documental ou `[NV]`; nota conservadora e recalibrável, com justificativa explícita nas seções 3.1 e 4.
- `[NV]` = não verificado ao vivo ou localmente (sem build, protótipo ou benchmark nesta rodada).
- `[UNVERIFIED — fetch skipped]` = host fora da allowlist (`github.com` e domínios de conteúdo) não buscado por política de hosts.
- **Trava inviolável:** nota < 5 em C1, C2 ou C3 impede adoção na Fase 0, independentemente do score; C4–C10 não compensam. **Fase 0** = resultado dessa trava; **Fase 1** = ordenação por score e rótulo.
- Termos estrangeiros são explicados na 1ª ocorrência: *headless*, *snapshot*, *tick*, *rollback* (retrocesso de simulação), ECS (*Entity Component System* — entidades/componentes/sistemas), *binding* (ligação/ponte), FFI (interface de função estrangeira), MSRV (versão mínima suportada), *edition* (edição da linguagem), *lock-in* (dependência tecnológica), *spike*, PNJ, *greybox*.

---

## 2. Opções avaliadas

Consolidação das quatro pesquisas. A classificação por categoria é exigida pelo protocolo: **só** Bevy, Fyrox e Ambient são engines escritas em Rust; o restante entra com a categoria correta e seu score **não** é apresentado como score de engine-base.

| ID | Opção | Classificação correta | Fonte primária local (data) |
|---|---|---|---|
| O1 | Godot 4.7.2 | Engine (baseline **não-Rust**) | `01-matriz.md` / `godot.md` (19/11/2026) |
| O2 | Bevy 0.20.1 | Engine Rust (ECS) | `rust-bevy.md` (20/11/2026) |
| O3 | Bevy 0.20.1 + Lightyear 0.31 | Engine Rust + netcode de terceiros | `rust-bevy.md` (20/11/2026) |
| O4 | Fyrox 1.0.0 / 2.0.0-rc.1 | Engine Rust com editor de cena | `rust-fyrox.md` (20/11/2026) |
| O5 | macroquad (R1) | Biblioteca/framework 2D (**não engine**) | `rust-outras.md` (20/11/2026) |
| O6 | ggez (R2) | Framework 2D leve (**não engine**) | `rust-outras.md` (20/11/2026) |
| O7 | Ambient (R3, pausada *indefinitely*) | Engine/runtime multiplayer Rust (**pausada**) | `rust-outras.md` (20/11/2026) |
| O8 | godot-rust/gdext (R4) | *Binding* GDExtension (**não engine**; a engine é Godot/C++) | `rust-outras.md` (20/11/2026) |
| O9 | Rapier (R5) | Biblioteca de física 2D/3D (**não engine**) | `rust-outras.md` (20/11/2026) |
| O10 | Bevy + Lightyear + Rapier | Derivada de O3 + física Rapier (**não verificada**) | derivação em `rust-matriz-consolidada.md` §3.1 (20/11/2026) |

**Distinção de categoria.** `godot-rust/gdext` é ligação para uma engine em C++ (Godot), Rapier é biblioteca de física, macroquad e ggez são bibliotecas/frameworks; classificá-los como "engines Rust" inflaria a amostra e mascararia a trava de C2/C3 (`rust-outras.md:27-34,76-89`). O score 68,6 de O8 mede o custo/benefício de integração **sobre O1**, não uma engine-base.

**Divergências registradas, não inferidas.**
- Bevy: contrato de escrita fixa 0.19.1, mas a observação ao vivo em 20/11/2026 é **v0.20.1** (28/10/2026); `main` declara `0.20.0-dev`, `edition = 2024`, `rust-version = 1.100.0`. Usa-se a versão observada.
- Lightyear: UI de Releases marca 0.30.0 como "Latest", enquanto `main`/README declaram **0.31.0** (Rust 1.97.1).
- Fyrox: release estável **1.0.0** (29/03/2026) contra `2.0.0-rc.1` no `master`.
- macroquad e Ambient: `MIT OR Apache-2.0` no manifesto contra detecção de só `Apache-2.0` na API do GitHub. Prevalece a fonte primária de código; nenhum valor intermediário é inferido.
- Fyrox: `rust-fyrox.md` (relatório dedicado) dá **65,2**; `rust-outras.md` (R6) dá **62,6** com C2=3. Prevalece **65,2** e C2=4* por ser o relatório dedicado, mais estrito e denso em fonte primária — a trava de C2 é a mesma nos dois. Registro na seção 7.

---

## 3. Matriz comparativa — Score e Fase 0/Fase 1

Notas na ordem C1…C10. Pesos: 15/15/12/10/10/8/8/8/7/7. `*` = evidência parcial/proxy/`[NV]`, justificada nas seções 3.1 e 4.

| Opção | C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 | C9 | C10 | Score | Fase 0 (C1–C3) | Fase 1 | Recomendação |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|---|
| **O1 Godot 4.7.2** | 9 | 8 | 8 | 8 | 9 | 7* | 9 | 9 | 8 | 7 | **82,6** | PASSA (9/8/8) | 1º | **ADOPT** (inalterado) |
| **O3 Bevy 0.20.1 + Lightyear 0.31** | 10 | 9* | 7* | 7* | 5* | 8* | 8 | 7* | 8* | 6* | **77,1** | PASSA (10/9/7) | 2º | **ADAPT** (padrões em GDScript) |
| **O10 Bevy+Lightyear+Rapier (derivada)** | 10 | 9* | 7* | 7* | 5* | 8* | 8 | 7* | 8* | 5* | **76,4** | PASSA (10/9/7) | 3º | **ADAPT** (padrões em GDScript) |
| **O8 godot-rust/gdext** | 8* | 7* | 7* | 7 | 4* | 6* | 8 | 8 | 7* | 6* | **68,6** | PASSA (8/7/7) | 4º | **DEFER** (binding, não engine) |
| **O2 Bevy 0.20.1** | 10 | 4* | 7* | 4* | 6 | 7* | 8 | 7* | 8 | 6 | **66,8** | **TRAVADA (C2=4*)** | 5º | **REJECT** (C2) |
| **O4 Fyrox 1.0/2.0rc** | 10 | 4* | 6* | 6* | 5* | 7* | 8 | 7* | 7 | 5* | **65,2** | **TRAVADA (C2=4*)** | 6º | **REJECT** (C2) |
| **O9 Rapier** | 10 | 3* | 3* | 4* | 6 | 8* | 5* | 8 | 8 | 7 | **60,4** | **TRAVADA (C2=3, C3=3)** | 7º | **REJECT** (C2/C3) |
| **O7 Ambient (pausada)** | 9* | 7* | 5* | 4* | 4* | 6* | 7 | **2** | 7 | 3 | **57,0** | PASSA (9/7/5) | 8º | **DEFER** (pausada; C8=2) |
| **O6 ggez** | 10 | 3* | 3 | 3 | 6 | 5* | 6 | 6 | 7 | 6 | **54,8** | **TRAVADA (C2=3, C3=3)** | 9º | **REJECT** (C2/C3) |
| **O5 macroquad** | 9 | 5* | **3** | 3 | 5 | 6* | 5* | 5 | 8 | 5 | **54,5** | **TRAVADA (C3=3)** | 10º | **REJECT** (C3) |

**Leitura da trava.** Cinco opções são bloqueadas na Fase 0 por nota < 5 em critério inviolável: O2 (C2=4*), O4 (C2=4*), O9 (C2=3 e C3=3), O6 (C2=3 e C3=3) e O5 (C3=3). O7 passa na trava, mas C8=2 (pausa indefinida) a retira da corrida por risco operacional. O8 passa na trava, mas é *binding*, não engine.

### 3.1 Proveniência das notas e derivação de O10

- **O1** — `01-matriz.md:49,96` (score 82,6 verificado em `01-matriz.md:96`). Única nota `*`: C6 7*, sem benchmark 5v5+bots `[NV]`; proxies documentais: headless, Strip Visuals, `ALWAYS`/`ON_CHANGE`, `max_sync_packet_size=1350`. Baseline não-Rust, inalterado.
- **O2** — `rust-bevy.md:52-69`: C1 10, C2 4*, C3 7*, C4 4*, C5 6, C6 7*, C7 8, C8 7*, C9 8, C10 6 → 66,8. Versão observada 0.20.1 (28/10/2026), não 0.19.1. C2 4* = trava. C6 7* e C8 7* recalibrados para baixo em relação a `bevy.md` porque as alegações "55 mil entidades/60 fps" e de fundação vieram de `bevy.org`, fora da política de hosts (`[UNVERIFIED — fetch skipped]`).
- **O3** — `rust-bevy.md:52-69`: C1 10, C2 9*, C3 7*, C4 7*, C5 5*, C6 8*, C7 8, C8 7*, C9 8*, C10 6* → 77,1. C2 9*: autoritativo, inputs por *tick* protegidos contra perda, replicação determinística, predição com *rollback*; reconexão robusta não verificada `[NV]`. C3 7*: hash/versão na aplicação, sem loader de primeira parte para notação de cena. C8 7*: projeto concentrado no mantenedor. C10 6*: acrescenta `aeronet`/`postcard`/`bevy_replicon`/Leafwing e acoplamento de versões.
- **O4** — `rust-fyrox.md:38-49`: C1 10, C2 4*, C3 6*, C4 6*, C5 5*, C6 7*, C7 8, C8 7*, C9 7, C10 5* → **65,2**. C2 4*: `fyrox-core/src/net.rs` só TCP não-bloqueante + `bincode`; sem validação de input, *snapshots* versionados, *rollback*, reconexão com restauração de sessão ou gestão de interesse. C10 5*: `ARCHITECTURE.md` declara engine monolítica e fortemente acoplada. Divergência 62,6 / C2=3 em `rust-outras.md` (R6) registrada; prevalece o relatório dedicado (mesma trava C2).
- **O5–O9** — `rust-outras.md:125-157` (R1…R5), scores 54,5 / 54,8 / 57,0 / 68,6 / 60,4. C2/C3 de O5 (3), O6 (3/3) e O9 (3/3) são ausência estrutural, não falta de evidência. O7 C8=2 decorre do aviso literal "Work on the Ambient runtime is paused indefinitely"; O7 C1 9* diverge (README diz dupla licença, API detecta só Apache-2.0). O8 C1 8* = MPL-2.0 copyleft por arquivo (OSI, mas não a preferência MIT); C2/C3 7* são capacidades **herdadas** do Godot, não implementadas pelo *binding*. O9 C6 8* = proxy SIMD/rayon/variante f64, sem 5v5 `[NV]`.
- **O10 (derivada)** — copia O3 integralmente e altera **apenas** C10: 6* → **5\*** por acoplamento extra (Rapier + `bevy_rapier` na árvore de dependências, MSRV/*edition* e camada de física no servidor). **C6 8\* é mantido**: o `parallel`/SIMD do Rapier é proxy documental, não benchmark 5v5+bots. Cálculo: O3 771 − 7 (C10 6→5) = 764 → **76,4**. O10 não foi construída, executada ou medida: `[NV]` integral; é cenário de sensibilidade, não evidência de que a composição funcione.

### 3.2 Verificação aritmética (conferência de cada conta)

Recálculo independente de `Σ(nota × peso)/10` feito na composição deste documento (nenhum gate, teste ou build executado); todos os 10 valores conferem com a tabela (1 casa decimal).

| Opção | Produtos C1…C10 | Σ | Score recalculado | Conferência |
|---|---|---:|---:|---|
| O1 | 135 + 120 + 96 + 80 + 90 + 56 + 72 + 72 + 56 + 49 | 826 | 82,6 | OK |
| O2 | 150 + 60 + 84 + 40 + 60 + 56 + 64 + 56 + 56 + 42 | 668 | 66,8 | OK |
| O3 | 150 + 135 + 84 + 70 + 50 + 64 + 64 + 56 + 56 + 42 | 771 | 77,1 | OK |
| O4 | 150 + 60 + 72 + 60 + 50 + 56 + 64 + 56 + 49 + 35 | 652 | 65,2 | OK |
| O5 | 135 + 75 + 36 + 30 + 50 + 48 + 40 + 40 + 56 + 35 | 545 | 54,5 | OK |
| O6 | 150 + 45 + 36 + 30 + 60 + 40 + 48 + 48 + 49 + 42 | 548 | 54,8 | OK |
| O7 | 135 + 105 + 60 + 40 + 40 + 48 + 56 + 16 + 49 + 21 | 570 | 57,0 | OK |
| O8 | 120 + 105 + 84 + 70 + 40 + 48 + 64 + 64 + 49 + 42 | 686 | 68,6 | OK |
| O9 | 150 + 45 + 36 + 40 + 60 + 64 + 40 + 64 + 56 + 49 | 604 | 60,4 | OK |
| O10 | 150 + 135 + 84 + 70 + 50 + 64 + 64 + 56 + 56 + 35 | 764 | 76,4 | OK |

Ordenação da Fase 1 por score: O1 (82,6) > O3 (77,1) > O10 (76,4) > O8 (68,6) > O2 (66,8) > O4 (65,2) > O9 (60,4) > O7 (57,0) > O6 (54,8) > O5 (54,5). A Fase 0 prevalece sobre esta ordenação.

---

## 4. Análise explicativa por critério

- **C1 (soberania e licença):** O2/O4/O9 têm 10 (MIT/Apache-2.0 confirmados em manifesto + README + arquivo de licença); O1 tem 9 (MIT/FOSS, ressalva documental); O8 tem 8* (MPL-2.0 é OSI, mas copyleft por arquivo e política de marca da Godot Foundation); O7 9* e O5 9 refletem divergência entre README (dupla licença) e detecção da API (só Apache-2.0). Nenhuma trava por C1.
- **C2 (autoridade e federação):** critério mais seletivo. O3/O10 (9*) são os únicos com netcode autoritativo de terceiros verificável (Lightyear: inputs por *tick* protegidos contra perda, replicação determinística, predição/*rollback*, gestão de banda); O1 (8) e O8 (7*, herdado do Godot) vêm do headless + replicação do Godot. Travam O2 (4*), O4 (4*), O6 (3*) e O9 (3*). O5 (5*) passa no limiar, mas cai em C3. Reconexão robusta pós-desconexão e hash de conteúdo no *handshake* permanecem `[NV]` para **todas** as opções (spike B).
- **C3 (modularidade):** O1 (8) lidera por Cena/Nó/sinal ↔ Ilha/Módulo/desafio; O2/O3/O10 (7*) com ECS e plugins, sem loader de notação de cena de primeira parte; O8 (7*) herda `Resource`/PCK do Godot. Travam O5 (3), O6 (3) e O9 (3*). O4 (6*) fica acima do mínimo com cenas RON `.rgs`. Nenhuma opção Rust apresenta importador automático de `.tscn` `[NV]`.
- **C4 (Desafios de Ilha):** O1 (8) tem editor de lógica; nenhuma engine Rust tem editor de lógica equivalente. O4 (6*) tem editor de **cena** (FyroxEd), O2 (4*) não tem editor oficial (issue #85 aberta desde 05/08/2020, atualizada em 01/10/2026), O3/O10 (7*) cobrem a janela de sincronia de 2 jogadores/5 s por predição/interpolação, mas exigem código para coleta, sabotagem e negociação com PNJs. C4 é "Alta", não inviolável: não gera trava.
- **C5 (curva e tempo até protótipo):** O1 (9) é o melhor caminho para 1 mantenedor com base em engenharia de software; todas as opções Rust pagam Rust + *borrow checker* (verificador de empréstimos) e toolchain recente (MSRV 1.86–1.100, *edition* 2024). O3/O10 (5*) sofrem com o book do Lightyear em WIP e a matriz rígida de versões (0.31 exige Rust 1.97.1+); O7 (4*) é penalizada pelo runtime parado.
- **C6 (performance):** **todas** as notas são proxies documentais; nenhum benchmark 5v5+bots, custo por *tick*, memória ou latência foi medido nesta rodada `[NV]`. O9 (8*) tem o melhor proxy (SIMD em lote, rayon, variante f64), O3/O10 (8*) têm suítes Criterion/Divan e banda/*lag compensation* no repositório, O1/O4 (7*) usam proxies headless. A alegação "55 mil entidades/60 fps" do Bevy veio de `bevy.org` (`[UNVERIFIED — fetch skipped]`) e não sustenta nota maior. Spike A é a única forma de transformar C6 em evidência primária.
- **C7 (pipeline de assets FOSS):** O1/O8 (9/8) herdam importação do Godot; O4 (8) tem glTF 2.0 e loader FBX próprio; O2/O3/O10 (8) têm `bevy_gltf`, KTX2, PNG e zstd no manifesto; extensões avançadas de compressão (Draco/meshopt) ficam `[NV]`. O5 (5*) só `png`/`tga` no crate `image` lido.
- **C8 (maturidade e governança):** O1 (9) lidera; O7 (2) é o risco de abandono explícito (pausa indefinida, `pushed_at` 07/01/2025, release v0.2.1 de 06/05/2023). Comunidades ativas em O2/O3 (49.894 estrelas, 20/11/2026) e O4 (9.689 estrelas, 1.0.0 estável em 29/03/2026) não compensam fundação/entidade neutra e roadmap não verificáveis sob a política de hosts (`[UNVERIFIED — fetch skipped]`); bus factor formal fica `[NV]`.
- **C9 (portabilidade):** desktop primeiro em todas; WebGL2/wasm e exemplos móveis verificados em O2/O3/O10; WebTransport + crate `platform/web` no Lightyear; console `[NV]` em todas as opções Rust. O5 (8) cobre desktop/HTML5/Android/iOS; O6/O7 (7) têm cobertura móvel parcial `[NV]`. No Fyrox, a rede em WASM não foi verificada (TCP cru não roda no navegador).
- **C10 (risco operacional e lock-in):** O1 (7) e O2 (6) têm árvores extensas e churn; O3 (6*) soma `aeronet`/`postcard`/`bevy_replicon`/Leafwing; O10 (5*) soma Rapier/`bevy_rapier`; O4 (5*) é monolítica (`ARCHITECTURE.md`); O7 (3) tem risco de plataforma central ("plataforma" de deploy no README, não confirmado `[NV]`); O8 (6*) soma MPL + acoplamento `api-4-2`…`api-4-7`. A cadência 0.x do Bevy quebra a API aproximadamente a cada 3 meses e o README não garante migrações fáceis.

**Viés da conclusão anterior registrado** (`rust-protocolo.md:9-13`): amostra Rust reduzida a 3/13 opções, ancorada em Bevy 0.19.1, Lightyear 0.28–0.30 e macroquad inativo; ausência de editor tratada como penalidade estrutural em C4/C5, que são "Alta", não invioláveis; C5 penaliza *borrow checker*/MSRV sem cronômetro; C6=8* vinha de `bevy_city` (render), não de servidor headless 5v5+bots; C2/C3 de Rust ficaram `[NV]` sem teste local. Esta avaliação amplia a amostra e corrige as versões, mas **não** produz medição local.

---

## 5. Tabela de evidência

Confiança: **Alta** = dado primário confirmado em manifesto/README/licença/API; **Média** = proxy documental ou fonte única sem verificação local; **Baixa/[NV]** = não verificado ao vivo/localmente; **`[UNVERIFIED — fetch skipped]`** = host fora da allowlist.

| # | Opção | Fonte (título) | URL/ID | Data | *Claim* | Confiança |
|---:|---|---|---|---|---|---|
| 1 | O1 | `01-matriz.md:49,96` (`godot.md`) | local | 19/11/2026 | Godot 4.7.2, MIT, headless, score 82,6; C6 7* sem benchmark | Alta (score); Média (C6) |
| 2 | O2/O3 | bevyengine/bevy — README.md | https://github.com/bevyengine/bevy | 20/11/2026 | Dupla licença MIT/Apache-2.0; API quebra ~a cada 3 meses; sem editor; "documentation is sparse" | Alta |
| 3 | O2/O3 | bevyengine/bevy — Cargo.toml (`main`) | https://github.com/bevyengine/bevy/blob/main/Cargo.toml | 20/11/2026 | `edition = 2024`, `rust-version = 1.100.0`, features headless/gltf/ktx2 | Alta |
| 4 | O2/O3 | bevyengine/bevy — LICENSE-MIT | https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT | 20/11/2026 | Texto MIT integral; sem EULA/SDK proprietário | Alta |
| 5 | O2/O3 | Releases · bevyengine/bevy | https://github.com/bevyengine/bevy/releases | 20/11/2026 | v0.20.1 "Latest" em 28/10/2026; v0.20.0 em 08/10/2026 (não 0.19.1) | Alta |
| 6 | O2/O3 | bevy — examples/app/headless.rs (v0.20.1) | https://github.com/bevyengine/bevy/blob/v0.20.1/examples/app/headless.rs | 20/11/2026 | `ScheduleRunnerPlugin` + `default-features = false`: headless real | Alta |
| 7 | O2/O3 | bevy — crates/bevy_time/src/lib.rs | https://github.com/bevyengine/bevy/blob/main/crates/bevy_time/src/lib.rs | 20/11/2026 | `Time<Fixed>`, `FixedUpdate`, relógio manual; replay não localizado | Alta (tempo); `[NV]` (replay) |
| 8 | O2/O3 | bevyengine/bevy — issue #85 (Bevy Editor) | https://github.com/bevyengine/bevy/issues/85 | 20/11/2026 | Editor sem implementação oficial; aberta desde 05/08/2020, atualizada 01/10/2026 | Alta |
| 9 | O2/O3 | bevyengine — organização no GitHub | https://github.com/bevyengine | 20/11/2026 | 22 repositórios, 49.894 estrelas, 5.084 forks, 625 PRs; fundação não verificada | Alta (metadados); `[NV]` (fundação) |
| 10 | O3 | cBournhonesque/lightyear — README.md | https://github.com/cBournhonesque/lightyear | 20/11/2026 | "server-authoritative"; inputs por *tick*, replicação determinística, predição/*rollback*, WebTransport | Alta |
| 11 | O3 | lightyear — Cargo.toml | https://github.com/cBournhonesque/lightyear/blob/main/Cargo.toml | 20/11/2026 | v0.31.0, Rust 1.97.1, `MIT OR Apache-2.0`, `aeronet`, `postcard`, `bevy_replicon 0.45.2` | Alta |
| 12 | O3 | Releases · cBournhonesque/lightyear | https://github.com/cBournhonesque/lightyear/releases | 20/11/2026 | Divergência UI (0.30.0 "Latest") vs. `main`/README (0.31.0) | Alta (divergência) |
| 13 | O4 | FyroxEngine/Fyrox — LICENSE.md | https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/LICENSE.md | 20/11/2026 | MIT/Expat; copyright "Dmitry Stepanov and Fyrox Engine contributors" | Alta |
| 14 | O4 | fyrox-core — src/net.rs | https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/fyrox-core/src/net.rs | 20/11/2026 | TCP não-bloqueante + `bincode`, prefixo `u32`: sem autoridade, *snapshots* ou reconexão | Alta |
| 15 | O4 | fyrox — Cargo.toml | https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/fyrox/Cargo.toml | 20/11/2026 | v2.0.0-rc.1, `edition = 2024`, `rust-version = 1.94`, `license = MIT` | Alta |
| 16 | O4 | FyroxEngine/Fyrox — ARCHITECTURE.md | https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/ARCHITECTURE.md | 20/11/2026 | Engine monolítica, poucas partes substituíveis, arenas generacionais | Alta |
| 17 | O4 | API — repos/FyroxEngine/Fyrox | https://api.github.com/repos/FyroxEngine/Fyrox | 20/11/2026 | 9.689 estrelas, `pushed_at 2026-11-20`; release 1.0.0 (29/03/2026) vs. master 2.0.0-rc.1 | Alta |
| 18 | O4 | FyroxEngine/Fyrox — CHANGELOG.md | https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/CHANGELOG.md | 20/11/2026 | Modo headless "suitable for server-side of multiplayer games" em 0.30 | Alta |
| 19 | O4 | editor-standalone — README.md | https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/editor-standalone/README.md | 20/11/2026 | Editor no workspace; versão standalone não suportada (sem plugins/scripts) | Alta |
| 20 | O5 | not-fl3/macroquad — Cargo.toml | https://raw.githubusercontent.com/not-fl3/macroquad/master/Cargo.toml | 20/11/2026 | v0.4.16, `MIT OR Apache-2.0`, "game library" 2D; sem rede/loader/editor | Alta (categoria); Média (C3/C6/C7) |
| 21 | O6 | ggez — README.md | https://raw.githubusercontent.com/ggez/ggez/master/README.md | 20/11/2026 | "Networking" e ECS listados como **Non-Features** explícitas | Alta |
| 22 | O6 | API — repos/ggez/ggez | https://api.github.com/repos/ggez/ggez | 20/11/2026 | MIT; `pushed_at 26/10/2026`; release 0.9.3 (10/07/2023) vs. crate 0.10.0 | Alta |
| 23 | O7 | AmbientRun/Ambient — README.md | https://raw.githubusercontent.com/AmbientRun/Ambient/main/README.md | 20/11/2026 | "Work on the Ambient runtime is paused indefinitely"; sincronia automática | Alta |
| 24 | O7 | API — repos/AmbientRun/Ambient | https://api.github.com/repos/AmbientRun/Ambient | 20/11/2026 | `pushed_at 07/01/2025`; release v0.2.1 (06/05/2023) → C8=2 | Alta |
| 25 | O8 | godot-rust/gdext — ReadMe.md | https://raw.githubusercontent.com/godot-rust/gdext/master/ReadMe.md | 20/11/2026 | "library to integrate the Rust language with Godot 4": binding, não engine | Alta |
| 26 | O8 | godot — Cargo.toml | https://raw.githubusercontent.com/godot-rust/gdext/master/godot/Cargo.toml | 20/11/2026 | v0.5.5, `MPL-2.0`, features `api-4-2`…`api-4-7` | Alta |
| 27 | O9 | dimforge/rapier — README.md | https://raw.githubusercontent.com/dimforge/rapier/master/README.md | 20/11/2026 | "2D and 3D physics engines"; solver SIMD, `parallel` via rayon, variante f64 | Alta (feature); Média (C6) |
| 28 | O9 | dimforge/rapier — Cargo.toml | https://raw.githubusercontent.com/dimforge/rapier/master/Cargo.toml | 20/11/2026 | v0.36.1, `Apache-2.0`, `edition = 2024`, MSRV 1.86; física pura | Alta |
| 29 | O10 | derivação desta + `rust-outras.md:132` | local (`rust-bevy.md`, `rust-outras.md`) | 20/11/2026 | O3 com C6 8* mantido e C10 5*; score 76,4; composição não executada | Baixa/[NV] |
| 30 | Todas | Política de hosts | `bevy.org`, `docs.rs`, `crates.io`, `fyrox.rs`, `rapier.rs`, `ambient.run`, `ggez.rs`, `godot-rust.github.io`, `fyrox-book.github.io` | 20/11/2026 | Hosts fora da allowlist não buscados | `[UNVERIFIED — fetch skipped]` |
| 31 | Todas | Ausência de medição local | — | 20/11/2026 | Nenhum benchmark 5v5+bots, reconexão, replay ou importador `.tscn` executado | `[NV]` |
| 32 | Todas | Cobertura linguística | — | 20/11/2026 | Inglês apenas; triangulação entre idiomas não foi possível nesta rodada | `[NV]` |

### 5.1 Evidência Contra

- **Bevy README:** "still in the early stages of development. Important features are missing. Documentation is sparse"; mudanças quebram a API ~a cada 3 meses e "we can't guarantee migrations will always be easy" (https://github.com/bevyengine/bevy, 20/11/2026).
- **Sem editor oficial:** issue #85 aberta desde 2020 e ainda aberta em 01/10/2026 (https://github.com/bevyengine/bevy/issues/85, 20/11/2026).
- **Headless não é autoridade:** o servidor headless existe, mas rede autoritativa não é *built-in*; exige Lightyear/terceiros (https://github.com/bevyengine/bevy/blob/v0.20.1/examples/app/headless.rs; https://github.com/cBournhonesque/lightyear, 20/11/2026).
- **Determinismo condicionado:** o do Lightyear depende de a simulação ser determinística; replay não localizado (https://github.com/cBournhonesque/lightyear, 20/11/2026).
- **Divergência de versão do Lightyear:** UI de Releases 0.30.0 contra `main`/README 0.31.0 (https://github.com/cBournhonesque/lightyear/releases, 20/11/2026).
- **Fyrox desmente "engine Rust com editor basta":** editor sólido, mas workspace sem crate de rede; `net.rs` entrega TCP cru + `bincode`, sem autoridade — C2 é inviolável (https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/Cargo.toml; https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/fyrox-core/src/net.rs, 20/11/2026).
- **Ambient desmente "Rust resolve netcode":** multiprocessado por design, mas o próprio README encerra o runtime (https://raw.githubusercontent.com/AmbientRun/Ambient/main/README.md, 20/11/2026).
- **gdext não é prova a favor do Rust como engine:** é prova a favor do Rust *sobre* Godot; atribuir os 68,6 ao *binding* como se fosse engine seria erro de categoria (ReadMe.md + Cargo.toml, 20/11/2026).
- **macroquad já foi avaliada:** O13 tem 54,5 e C3=3; reintroduzi-la como "nova" opção duplicaria a matriz (`01-matriz.md:134`, `alternativas.md:22`).
- **Metadados conflitam entre si:** macroquad e Ambient (`MIT OR Apache-2.0` vs. `Apache-2.0` na API), ggez (release 0.9.3 vs. crate 0.10.0), Fyrox (release 1.0.0 vs. master 2.0.0-rc.1). Scores de maturidade devem usar a fonte primária de código, não só o release marcado.

---

## 6. Recomendação final e *spikes*

| Opção | Rótulo | Fundamento |
|---|---|---|
| **O1 Godot 4.7.2** | **ADOPT** (inalterado) | 82,6; Fase 0 passa; ADR-001 vinculante; nenhuma evidência Rust reabre o motor. |
| **O3 Bevy 0.20.1 + Lightyear 0.31** | **ADAPT** — **apenas como padrões reescritos em GDScript** | 77,1; Fase 0 passa, mas reconexão `[NV]`; autoridade, inputs por *tick* e predição são padrões a importar, não motor a adotar. |
| **O10 Bevy+Lightyear+Rapier** | **ADAPT** — **apenas como padrões reescritos em GDScript** | 76,4; C10 5* por acoplamento extra; derivada `[NV]`, sem validação de composição. |
| **O8 godot-rust/gdext** | **DEFER** (como *binding*) | 68,6; *binding*, **não engine**: útil como camada Rust *sobre* O1, mas MPL-2.0 e `api-4-x` acrescentam risco sem Autoridade própria (herdada). |
| **O7 Ambient** | **DEFER** | 57,0; runtime pausado *indefinitely* (C8=2); multiprocessado por design, mas sem suporte futuro verificável. |
| **O2 Bevy 0.20.1** | **REJECT** | 66,8; C2=4* — trava inviolável: sem validação/snapshots/reconexão nativos. |
| **O4 Fyrox** | **REJECT** | 65,2; C2=4* — trava inviolável: `net.rs` só TCP cru + `bincode`; servidor autoritativo teria de ser escrito do zero. |
| **O5 macroquad** | **REJECT** | 54,5; C3=3 — trava inviolável: sem loader declarativo; exige recompilar. |
| **O6 ggez** | **REJECT** | 54,8; C2=3 e C3=3 — "Networking" é Non-Feature; foco 2D incompatível com ilhas 3D low-poly. |
| **O9 Rapier** | **REJECT** (como engine-base) | 60,4; C2=3 e C3=3 — biblioteca de física: útil no servidor, inadequada como base de jogo. |

**Compatibilidade ADR-001.** Nenhum rótulo autoriza troca de motor, introdução de Rust no Núcleo, reabertura de Godot-first ou exceção adicional de linguagem neste ciclo. ADAPT significa **reexpressar padrões em GDScript**, não importar crates. F3 (servidor custom) continua DEFER e fora do escopo decisório. Reavaliação futura exige evidência prévia (spikes abaixo) e aprovação formal do maintainer (ADR-001, `## Critérios de reavaliação`).

### Spikes A, B e C (M5–M6, somente evidência; sem efeito decisório automático)

- **Spike A — benchmark pareado Godot vs. Bevy+Lightyear.** Medir headless 5v5 + bots em condições idênticas: custo por *tick*, overhead de servidor, memória, latência p50/p99 e largura de banda. **Critério de aceite:** tabela pareada com a mesma cena, o mesmo número de entidades e reprodutibilidade das medições; sem isso C6 permanece proxy `[NV]`.
- **Spike B — reconexão e handshake.** Testar reconexão robusta pós-desconexão no Lightyear (restauração de sessão, inputs em voo, *snapshots* versionados) e hash/versão de conteúdo no *handshake*, em oposição a O1. **Critério de aceite:** cenário de queda e retorno com estado consistente e conteúdo divergente rejeitado no *handshake*; hoje `[NV]` para todas as opções — é o ponto que sustenta C2 9* de O3/O10 e as travas de O2/O4.
- **Spike C — *migrations* e *edition* 2024.** Medir o custo de atualização do toolchain Rust entre releases 0.x do Bevy (0.20.1 → próximas), `edition = 2024` e MSRV (1.100.0 no Bevy; 1.97.1 no Lightyear; 1.94 no Fyrox; 1.86 no Rapier), além das features `api-4-x` do gdext. **Critério de aceite:** registro datado das quebras de API e do tempo de migração; sem isso C5/C10 seguem com `*` e a alegação de "migrações não garantidas" do README do Bevy não é contrastada localmente.

---

## 7. Limitações e próxima verificação

- **`[NV]` — sem build, protótipo ou benchmark:** **todas** as notas C6 são proxies documentais; custo por *tick*, memória e latência 5v5+bots não medidos. Nenhuma alegação de desempenho medido é feita.
- **`[NV]` — reconexão e handshake:** reconexão que restaure sessão e hash de conteúdo no *handshake* não executados em nenhuma opção (bloqueio direto de C2/C3 em O3/O10 e das travas de O2/O4).
- **`[NV]` — Bevy/Lightyear:** replay/*demo recorder*, suporte a console e importador automático de `.tscn` não localizados em `github.com`.
- **`[NV]` — Fyrox e macroquad:** Draco/meshopt/KTX2 no loader glTF do Fyrox e primitivos 3D do macroquad não confirmados nos arquivos lidos; iOS e rede em WASM no Fyrox não verificados; extensibilidade do editor do Fyrox para lógica de Desafios de Ilha não verificada.
- **`[NV]` — governança:** bus factor, fundação/entidade neutra, roadmap, serviço central no Ambient e política de marca não confirmados em fonte primária; o binding legado `godot-rust/gdnative` não foi examinado.
- **`[NV]` — O10:** composição derivada, não construída, executada ou medida; score 76,4 é cenário de sensibilidade, não evidência empírica.
- **`[UNVERIFIED — fetch skipped]` — hosts fora da allowlist:** `bevy.org`, `docs.rs`, `crates.io`, `fyrox.rs`, `rapier.rs`, `ambient.run`, `ggez.rs`, `godot-rust.github.io`, `fyrox-book.github.io`. Cobertura real restrita a `github.com` e domínios de conteúdo; **inglês apenas** — triangulação entre idiomas não foi possível nesta rodada (nenhuma fonte primária em português disponível no recorte).
- **Divergências registradas, não resolvidas:** Lightyear UI 0.30.0 vs. `main`/README 0.31.0; macroquad e Ambient `MIT OR Apache-2.0` no manifesto vs. só `Apache-2.0` na API; ggez release 0.9.3 vs. crate 0.10.0; Fyrox release 1.0.0 estável vs. 2.0.0-rc.1 no `master`; Bevy 0.19.1 do contrato vs. 0.20.1 observado; **O4 C2 4*/score 65,2 (`rust-fyrox.md`, relatório dedicado) vs. C2 3/score 62,6 (`rust-outras.md` R6), prevalecendo o relatório dedicado** — a trava de C2 é idêntica nas duas leituras.
- **Vieses de origem:** amostra Rust ampliada nesta rodada, mas ainda sem medição local; ausência de editor é penalidade em C4/C5 ("Alta"), não trava; C5 penaliza toolchain sem cronômetro; C6 permanece proxy.
- **Próxima verificação:** executar os spikes A, B e C no M5–M6 e reabrir esta matriz **apenas** para atualizar notas com `*`; qualquer mudança de motor permanece condicionada à ADR-001 e à aprovação formal do maintainer. Nenhum artefato R4 (`validate`) ou R7 (`release`) foi escrito; R4 precede R7 quando executados em portaria própria.

Estado: documento de apoio a ADR-001; nenhuma troca de motor autorizada
