# Problem — Lacuna: nenhum cliente jogável no workspace da stack autorizada

**Data:** 2026-11-21

## Problema

A SE reorientou a prioridade em 2026-11-21T00:22Z para um MVP **single-player** que prove a tese, cujo item 1 é "cliente greybox jogável (main scene, câmera, ilhas geométricas)" (`.specify/memory/STATE.md:5`). O workspace já tem `insurgencia-contratos` e `insurgencia-sim` mesclados (PR #27 e #28, HEAD `a2c8b22`) e `./verify.sh` → ALL STAGES OK com 15 testes (`specs/022-simulacao-rust/converge.md:13`), mas **não existe nenhum cliente jogável**: `crates/` contém apenas `contratos` e `sim`, o `specs/023-cliente-greybox/spec.md` ainda é template Draft e nada abre uma main scene com câmera e o Arquipélago em formas geométricas. Há, portanto, uma lacuna entre a fatia visual exigida pela SE — primeira do entregável, pré-requisito dos itens 2, 3 e 4 — e a ausência total de cliente na stack autorizada pela ADR-004 (Rust 1.99.1 + Bevy 0.20.1 + Lightyear 0.31; `docs/adr/004-migracao-bevy-lightyear.md:17`).

## Por que é válido

- **Impacto 5** (escala 1–5) — máximo: sem cliente, nenhuma das três metas seguintes da SE pode ser executada — duas partidas com cliente modificado (GDD:1567), instrumentação + 20 partidas da §17 e exportação de conta/personagem — e a tese do MVP ("jogabilidade baseada em desafios de ilha … é viável e divertida", GDD:1509) permanece sem prova observável. Não é dívida documental nem ajuste incremental: exige main scene, câmera e o mapa em greybox com três ilhas de rota + Ilha Central (GDD:1486,1492), além da governança `mdmv-rust-thin` aplicada desde o primeiro arquivo. O próprio cancelamento do ciclo 023 de servidor isolado (STATE:7) mostra que o cliente é o gargalo atual. [source: STATE:5-8; GDD:1486,1492,1509,1567; ADR-004:17; inspeção de `crates/`]
- **Confiança 4** (escala 1–5) — a lacuna é inequívoca e verificada por inspeção da estrutura do alvo nesta etapa (`crates/` sem cliente; spec 023 em template), e duplamente ancorada: a interjeição da SE é datada e literal (STATE:3-8) e o GDD fixa o greybox e o mapa (`:1486,1492,1500-1507`). O desconto a 4 (e não 5) deve-se a três incertezas declaradas: (1) "jogável" no item 1 não tem definição operacional — controles, enquadramento e nível de implementação dos desafios não foram fixados; (2) o GDD §16.3/§16.4 ainda descrevem cliente-servidor e dois clientes (`:1490,1513-1516`), em tensão com o item single-player, sem reconciliação escrita; (3) nenhuma janela, câmera ou malha foi executada, e não há captura/frame do greybox. [source: STATE:5,8; GDD:1490,1513-1516; inspeção de `specs/023-cliente-greybox/spec.md:1-9`]
- **Custo de não agir** — o entregável inteiro fica travado no item 1: os itens 2–4 não têm onde rodar, os critérios §16.4 e as métricas §17 seguem sem coleta e a instrução da SE ("cliente greybox entra como próximo ciclo, não servidor isolado", STATE:8) deixa de ser cumprida. Adiar para depois de novos ciclos de servidor repete o churn que motivou o cancelamento do 023. Corrigir agora é barato e rastreável; corrigir depois de mais código exige retrabalho da integração com `insurgencia-sim`. [source: STATE:5-8; GDD:1567,1607-1622]

## Afetados & stakeholders

- **Usuários**: mantenedor único — precisa de uma fatia executável, com main scene/câmera/ilhas, no mesmo workspace Rust já verde; jogadores — servidos indiretamente, pois o objetivo do MVP (GDD:1509) e as métricas da §17 dependem de uma partida jogável para serem validados.
- **Stakeholders**: SE — autoridade que reorientou a prioridade em 2026-11-21T00:22Z e cobra a sequência de itens (STATE:5-8); Núcleo/`insurgencia-sim` — consumidor das intenções do cliente, cuja integração deverá ser definida na spec; ADR-004 — norma de stack e governança que o cliente precisa respeitar sem nova troca de engine (`:17,53`).

## Metas

1. **Materializar o cliente jogável** — criar o crate de cliente no workspace `crates/` (layout Rust-only), com uma `App` Bevy que abra uma **main scene real**, com **câmera** enquadrando o Arquipélago da Insurgência em **ilhas geométricas** sem textura (greyboxing, GDD:1486): três ilhas de rota e uma Ilha Central (`:1492`). Nada de tela de terminal, log de texto ou stub headless.
2. **Manter o item 1 single-player e local** — nenhuma dependência de lobby, matchmaking, 5v5, NAT/UDP externo ou servidor remoto; servidores com Módulos diferentes, queda do servidor de referência, servidores comunitários e forks seguem diferidos (STATE:6-8).
3. **Preservar a norma do GDD e da ADR-004** — GDD §16.3/§16.4, pilares, regras de ilha, condição de vitória e critérios §16.4 permanecem vigentes para os itens seguintes; nenhuma revogação silenciosa, nenhuma reabertura da ADR-001 e nenhuma troca adicional de engine sem novo ADR (ADR-004:53).
4. **Aplicar a governança desde o primeiro arquivo** — `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, zero `.unwrap()`/`.expect()`, erros via `Result`/`?`/`thiserror`/`anyhow`, versões fixadas com justificativa no commit e `./verify.sh` como gate determinístico, sem relaxar lints (skill `mdmv-rust-thin`).
5. **Entregar o fluxo SDD ordenado** — converter o assessment em spec preenchida, clarificações, plano, checklist e tasks antes de qualquer implementação, com R1 `go` e R4 precedendo R7 (`.specify/memory/constitution.md:11`).

## Limites

Fora de escopo nesta etapa: escrita de qualquer crate, `Cargo.toml`, spec, plano ou task; execução de `cargo`, `./verify.sh`, clippy, nextest, testes, `make`, Godot, build, formatação, janela, câmera, render, gates ou benchmark; consulta a hosts externos; stage, commit, push, merge, deploy. Fora do item 1 (mas não revogado): duas partidas consecutivas com cliente modificado (item 2 / GDD:1567), instrumentação + 20 partidas da §17 (item 3), exportação/`COPYRIGHT.txt`/commits assinados (item 4), dois servidores com Módulos diferentes e teste de queda (item 5), servidores comunitários e forks (item 6). Também fora: remoção ou reescrita do histórico GDScript, FFI para manter GDScript vivo, lobby, 5v5, NAT/UDP externo, arte AAA e troca de engine. Nenhuma alegação de compilação, renderização, jogabilidade, desempenho ou validação ao vivo é feita — todo esse estado permanece **[NV]** até verificação datada na portaria própria. Os 5/5 gates são **citados** de `.specify/memory/STATE.md:7` e o ALL STAGES OK de `specs/022-simulacao-rust/converge.md:13`, não reexecutados. Convenções: pt-BR, `[NV]`, termos estrangeiros explicados na 1ª ocorrência.

## Critérios de sucesso

- Estrutura Rust com o crate de cliente materializado no workspace e dependências fixadas (baseline: 0 crates de cliente; meta: 1, rastreável no `Cargo.toml` do workspace).
- Main scene executável com câmera e as quatro ilhas em formas geométricas visíveis (baseline: nenhuma janela/cena observada; meta: captura/frame documentado no ciclo de implementação, qualitativamente declarado).
- Cliente sem rede: nenhum lobby, matchmaking, 5v5, NAT ou servidor remoto no caminho de execução do item 1 (baseline: **[NV]**; meta: execução local verificada sem serviços externos).
- Critérios §16.4 e reduções de escala preservados como norma e carregados para os itens 2/3, sem revogação (baseline: texto normativo em GDD:1494,1496,1513-1516; meta: rastreabilidade explícita na spec).
- `./verify.sh` com ALL STAGES OK no ciclo de implementação (baseline: ALL STAGES OK registrado para contratos/sim nos ciclos 021/022; meta: mesmo resultado com o novo crate incluído), sem stage/commit/push nesta etapa.

## Perguntas abertas

- **[NV]** O que exatamente significa "jogável" no item 1: controles, enquadramento da câmera e os desafios das ilhas precisam estar implementados ou basta o mapa navegável?
- **[NV]** Câmera 2D ou 3D e disposição das ilhas na cena — qual representação do Arquipélago atende melhor ao greybox da GDD?
- **[NV]** Como reconciliar o GDD §16.3/§16.4 (dois clientes, servidor autoritativo) com o item 1 single-player, sem revogar critérios que o item 2 ainda precisa?
- **[NV]** Nome do crate, API pública e fronteira com `insurgencia-sim`/`insurgencia-contratos`, além das versões fixadas das dependências visuais.
