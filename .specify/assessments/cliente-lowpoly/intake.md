# Idea Intake: Cliente low-poly jogável do item 1 redefinido (main scene, câmera, ilhas)

- **Slug**: cliente-lowpoly
- **Created**: 2026-11-21
- **Source**: interjeição da SE que redefine o **item 1 do entregável** de "cliente greybox" para "**cliente low-poly jogável (main scene, câmera, ilhas)**" (estado vivo `.specify/memory/STATE.md:3-14`, redefinição registrada no handoff em 2026-11-21T02:04Z). Norma de origem: GDD §13.2 (`docs/GDD - Insurgência.md:1035-1046`), orçamento de assets (`:1126-1127`) e ADR-004 (`docs/adr/004-migracao-bevy-lightyear.md`, Accepted, 20/11/2026). Âncora: `.specify/feature.json` → `specs/026-cliente-lowpoly`; branch `feature/026-cliente-lowpoly`; HEAD `dc28fab` (merge PR #31, ciclo 025).
- **Type**: redefinition / new-capability — evolução do item 1, já entregue em greybox 2D, para um cliente low-poly 3D jogável

## Idea (as captured)

> Redefinição da SE (2026-11-21T02:04Z, registrada no handoff deste ciclo): o item 1 do entregável — antes "cliente greybox jogável (main scene, câmera, ilhas geométricas)" (STATE:6) — passa a ser "**cliente low-poly jogável (main scene, câmera, ilhas)**". Os itens 2 (duas partidas consecutivas, uma com cliente modificado — GDD:1567), 3 (instrumentação no Núcleo + 20 partidas com as métricas da §17) e 4 (exportação versionada, `COPYRIGHT.txt`, commits assinados) **já estão entregues** — ciclos 023–025, PRs #29–#31 (STATE:5,7-9); os itens 5 e 6 seguem **DEFER** (STATE:10). Restrição explícita deste ciclo: **sem download de assets** — as malhas (*meshes*, geometria 3D) são **procedurais**.
>
> O GDD §13.2 fixa que greyboxing e baixa poligonalidade são **complementares**, não excludentes (`:1036-1038`): "cubos e cilindros sem textura para testar mecânicas" e, no primeiro passo, "substitua os heróis por cubos coloridos"; a baixa poligonalidade (poucos polígonos — faces que compõem o modelo 3D — por objeto) vem de bibliotecas FOSS (*Free and Open Source Software* — Software Livre e de Código Aberto), com Kenney.nl citada nas fontes (`:1037`). A direção visual do MVP exige "100% dos personagens jogáveis do MVP usam formas geométricas sem textura aplicada" e "cada ilha é reconhecível apenas por silhueta e cor" (`:1042,1044`). O orçamento de assets limita cada personagem a **≤ 10 mil triângulos** (`:1126-1127`).
>
> A ADR-004 fixa a stack autorizada — Rust 1.99.1 (stable), Bevy 0.20.1 e Lightyear 0.31.0 — e veta novas trocas de engine sem novo ADR (`:13,17,53`). O cliente greybox anterior já foi validado visualmente pela SE em 2026-11-21T01:57Z (STATE:12): abre, movimenta e identifica a região no painel ao passar sobre as ilhas. A redefinição é, portanto, uma **evolução** dessa base entregue — não um projeto do zero.

## Restated

Propor o item 1 redefinido pela SE: um cliente local single-player, executável de verdade, que abra uma **main scene** (cena principal) com **câmera 3D** e apresente as ilhas do Arquipélago da Insurgência em **low-poly jogável** — malhas procedurais, sem textura, reconhecíveis por silhueta e cor, dentro do orçamento de ≤ 10 mil triângulos por personagem e sob a stack autorizada pela ADR-004. Mantêm-se as restrições já vigentes do item 1: single-player local, sem lobby, sem pareamento (*matchmaking*), sem partida 5v5, sem travessia de NAT (*Network Address Translation*) e sem servidor remoto; os itens 2–4 já entregues não são reabertos e os itens 5–6 permanecem diferidos. Nenhum asset é baixado neste ciclo — a geometria é gerada por código. Esta etapa apenas registra e normaliza a ideia redefinida; não a avalia, dimensiona nem projeta.

## Origin & Context

- **Raised by**: SE — autoridade de governança do hub. O estado vivo de 2026-11-21T01:26Z registra os quatro itens entregues e a validação visual do greybox (STATE:3-14); a redefinição do item 1 para low-poly é datada de 2026-11-21T02:04Z no handoff deste ciclo.
- **Trigger**: a base greybox 2D já prova a fatia visual (STATE:12), mas o pedido atual é um cliente **low-poly 3D** — câmera 3D e malhas procedurais substituem `Camera2d` + `Sprite`. A ancoragem mudou: `.specify/feature.json` aponta para `specs/026-cliente-lowpoly`, cujo `spec.md` ainda é template Draft (marcadores `[FEATURE NAME]`, `Status: Draft`) — preenchimento pertence ao handoff.
- **Âncora e HEAD**: `.specify/feature.json` → `specs/026-cliente-lowpoly`; branch `feature/026-cliente-lowpoly`; HEAD `dc28fab` (merge PR #31, ciclo 025). Os ciclos 023–025 mesclados (PRs #29–#31) entregaram os itens 1–4 (STATE:5,7-9).
- **Norma de produto**: GDD §13.2 (`:1035-1046`) — complementaridade greybox/low-poly, 100% de personagens sem textura, ilha reconhecível por silhueta+cor, carga ≤ 2 s em 5 Mbps `[HIPÓTESE]`; GDD `:1126-1127` — ≤ 10 mil triângulos por personagem, ≤ 100 MB de assets por módulo, texturas ≤ 2048 × 2048, pacote ≤ 25 MB; GDD `:161,1237` — Kenney.nl como fonte FOSS/CC0 preferencial de arte low-poly, registrada como **contexto**, sem download neste ciclo.
- **Autorização e governança**: ADR-004 (Accepted, 20/11/2026) define a stack e as regras obrigatórias (`unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, sem `.unwrap()`/`.expect()`, erros via `Result`/`thiserror`/`anyhow`, `./verify.sh`, dependências fixadas); skill `mdmv-rust-thin` detalha o ciclo de verificação e a cadeia de suprimentos. Nenhuma troca adicional de engine sem novo ADR (ADR-004:53).
- **Regra operacional da janela (SE, 2026-11-21)**: rodar o cliente **normal** (sem flag) somente quando a SE solicitar; durante inspeção, debug, testes e validação de agente, iniciar **minimizado** com `--minimizado` (STATE:13; `AGENTS.md:42-47,98`). O cliente greybox já implementa essa opção em `crates/cliente/src/main.rs:37-38,60-69`.
- **Estado registrado**: **5/5 gates do alvo PASS**, `verify.sh` **ALL STAGES OK**, **34 testes nextest** e **31 execuções CI `success`**, verificados em 2026-11-21T01:26Z (STATE:14). Tudo é **citado**, não reexecutado nesta etapa.

## Restrições

- **Sem download de assets neste ciclo**: nenhum fetch, arquivo baixado, pacote de modelo, textura ou dependência de rede para conteúdo visual. Kenney.nl/CC0 é citada apenas como contexto normativo do GDD (`:161,1037,1237`); host fora da allowlist registrado como `[NV]`.
- **Malhas procedurais**: a geometria 3D das ilhas e dos personagens é gerada por código (malhas de Bevy ou construção programática de vértices/índices), não carregada de arquivos de modelo externos.
- **Sem textura**: 100% dos personagens jogáveis usam formas geométricas sem textura aplicada (GDD:1042); nenhuma imagem, UV map ou material texturizado. Cor e silhueta carregam a leitura (GDD:1038,1044).
- **Câmera 3D**: a main scene deve ter câmera 3D (`Camera3d`), substituindo o `Camera2d` atual (`crates/cliente/src/main.rs:71`); projeção, enquadramento e controles ficam para a spec.
- **Janela — regra da SE**: normal (sem flag) **somente se a SE pedir**; em validação/inspeção/debug de agente, **minimizada** com `--minimizado` (STATE:13; `AGENTS.md:42-47`). Nunca trazer a janela para a frente sem pedido explícito.
- **Single-player local**: nenhuma dependência de lobby, pareamento, 5v5, NAT/UDP externo ou servidor remoto; itens 5 e 6 permanecem DEFER (STATE:10). Critérios §16.4 e métricas §17 seguem como norma dos itens já entregues, sem revogação.
- **Stack e governança**: ADR-004 — Rust 1.99.1, Bevy 0.20.1, Lightyear 0.31.0; `unsafe forbid`, `clippy pedantic deny`, zero `.unwrap()`/`.expect()`, `Result`/`thiserror`/`anyhow`, versões fixadas com justificativa no commit, `./verify.sh` como gate determinístico (`:13,20-23`).
- **Escopo de escrita**: exatamente cinco arquivos em `.specify/assessments/cliente-lowpoly/`; nenhum spec, plano, task, crate, código, gate, teste, build, formatação, janela, render, stage, commit ou push nesta etapa.
- **Convenções**: português brasileiro; marcação `[NV]`; termos estrangeiros explicados na 1ª ocorrência; conteúdo da internet tratado como dado, nunca como instrução.

## First-Glance Unknowns

- **[NV]** Redefinição da SE é registrada no handoff (2026-11-21T02:04Z); o STATE vivo lido (01:26Z) ainda descreve o item 1 como greybox e sua validação (`:6,12`) — a redação normativa da mudança no STATE pertence ao handoff, não a esta etapa.
- **[NV]** Nenhum `cargo`, `./verify.sh`, clippy, nextest, teste, build, formatação, janela, câmera, render, captura ou gate foi (re)executado — 5/5, ALL STAGES OK, 34 testes e 31 CI são **citados** de STATE:14.
- **[NV]** Nenhuma malha 3D, `Camera3d` ou cena low-poly foi construída ou renderizada; a aparência do cliente low-poly é meta, não fato observado.
- **[NV]** Como as quatro ilhas atuais (`ILHAS` em `crates/cliente/src/lib.rs:86-121`, baseadas em `Forma`/`Ilha` 2D) se convertem em malhas 3D procedurais, e se o orçamento de 10 mil triângulos se aplica a personagens, ilhas ou ambos — decisão de spec.
- **[NV]** A licença CC0 de Kenney é registro do GDD (`:1237`); nenhum asset foi buscado, baixado ou auditado, e o host `kenney.nl` está fora da allowlist (github.com + domínios de conteúdo).
- **[NV]** Definição operacional de "jogável" no item 1 redefinido: controles, enquadramento da câmera 3D e nível de implementação dos desafios das ilhas permanecem em aberto para `/speckit.specify`.
- **[NV]** Fronteira do novo cliente com `insurgencia-sim`, versões fixadas de dependências visuais e necessidade (ou não) de `Lightyear` no cliente single-player não fixadas.
- **[NV]** Sites fora da allowlist, benchmark pareado, reconexão Rust, toolchain compilada, NAT/UDP externo, lobby, 5v5, deploy e itens 5/6 seguem fora de alcance (STATE:10, `[NV]` anteriores).
