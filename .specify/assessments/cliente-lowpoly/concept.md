# Concept — Como entregar o cliente low-poly jogável do item 1 redefinido

**Data:** 2026-11-21 · **Opção recomendada:** Opção 1 — malhas procedurais em Bevy 3D, no crate `insurgencia-cliente`

## Contexto

A SE redefiniu em 2026-11-21T02:04Z o item 1 do entregável: de "cliente greybox jogável" para "**cliente low-poly jogável (main scene, câmera, ilhas)**", com malhas procedurais e sem download de assets. O cliente entregue no ciclo 023 e validado pela SE (STATE:6,12) é 2D — `Camera2d` e `Sprite` em `crates/cliente/src/main.rs:71-99`, geometria plana em `lib.rs:23-121` —, enquanto o GDD exige direção visual low-poly (`:1035-1046`), 100% de personagens sem textura (`:1042`), ilhas por silhueta+cor (`:1044`) e ≤ 10 mil triângulos por personagem (`:1126-1127`). O impacto é 4 e a confiança 4: a base existe e os itens 2–4 já estão entregues (STATE:5,7-9), mas o pedido atual pede câmera 3D e malhas 3D procedurais. A ADR-004 mantém Rust 1.99.1 + Bevy 0.20.1 + Lightyear 0.31.0 e veta novas trocas de engine (`:17,53`); a regra da janela da SE exige `--minimizado` em validação de agente e janela normal só a pedido (STATE:13). Nenhuma engine nova, nenhum asset baixado, nenhuma textura.

## Opções consideradas

1. **Malhas procedurais em Bevy 3D, evoluindo o crate `insurgencia-cliente` — escolhida.**
   - **Sketch**: manter o crate já mesclado e trocar a representação 2D pela 3D na mesma `App` Bevy: `Camera3d` no lugar de `Camera2d` (`main.rs:71`), avatar e ilhas como entidades com `Mesh3d`/malha construída por código (vértices/índices ou primitivas do Bevy) e material sem textura (cor por ilha/personagem), substituindo os `Sprite` de `main.rs:76-99`. A geometria nasce de geradores puros em `lib.rs` (onde já vivem `Forma`, `Ilha`, `ILHAS`), agora com coordenadas 3D; o espelho `insurgencia-sim` e o flag `--minimizado` são preservados. Cena inteiramente local, single-player, sem plugin de rede, sem lobby e sem servidor. Contagem de triângulos é uma métrica de construção da malha, não um asset medido. Erros tipados com `Result`/`thiserror`/`anyhow` e governança `mdmv-rust-thin` desde o primeiro arquivo.
   - **Appetite**: medium (dias a uma semana) — reaproveita workspace, toolchain, dependência `bevy = "=0.20.1"` já fixada e o gate `./verify.sh`; o custo extra está em gerar/conferir malhas 3D, enquadrar a câmera e adaptar os quatro `ILHAS` para 3D. Registro de orçamento, não medição.
   - **Trade-offs**: ganha alinhamento direto com a redefinição da SE e com o GDD §13.2, entrega uma cena low-poly observável e mantém o mesmo gate e a mesma engine do Núcleo; não introduz dependência de rede nem passivo de licença. Sacrifica a simplicidade do sprite 2D e exige disciplina de lints/versões com as features 3D do Bevy. Risco: churn de API do Bevy 0.x (ADR-004:33), já visível nas correivas de `SystemParam` do converge 023.
   - **Rabbit holes**: desenhar malhas com muitos vértices e estourar o orçamento de 10 mil triângulos; confundir "3D" com apenas trocar a projeção da câmera e manter sprites; converter `Forma`/`ILHAS` e mexer em `insurgencia-sim` antes da spec, inflando o item 1; esquecer o `--minimizado` e abrir a janela na frente sem pedido da SE.
   - **Por que atende**: é a única opção que satisfaz simultaneamente câmera 3D, malhas procedurais, sem textura, silhueta+cor, orçamento de triângulos, regra da janela e a stack autorizada — sem asset externo e sem host fora da allowlist.

2. **Baixar assets low-poly Kenney CC0 — rejeitada neste ciclo por dependência de rede e licença a auditar.**
   - **Sketch**: obter modelos FOSS de `kenney.nl/assets` (GDD:1237, `:1854`), importá-los como malhas no Bevy e aplicar materiais CC0. A GDD chama Kenney de "fonte principal de *low-poly* após o *greybox*" (`:1237`) e o CC0 é registrado como "domínio público, uso comercial livre, atribuição opcional", com a ressalva expressa: "conferir o arquivo de licença incluído" (`:1237`).
   - **Appetite**: small a medium para importar, aparentemente o caminho mais rápido para uma estética low-poly "pronta".
   - **Trade-offs**: entregaria variedade visual e coerência com a fonte preferencial do GDD; em troca exige download de assets, rastreabilidade de proveniência e auditoria de licença por pacote — exatamente o que a SE vetou neste ciclo ("sem download de assets") e o que as diretrizes de governança tratam com rigor (`.specify/assessments/ast-20-licenciamento-proveniencia-arte/`).
   - **Rabbit holes**: `kenney.nl` está **fora da allowlist** (github.com + domínios de conteúdo) — `[UNVERIFIED — fetch skipped]`; nenhum host externo foi consultado e nenhum arquivo de licença foi lido. A dependência de rede torna o resultado não reproduzível offline e cria passivo de auditoria antes mesmo da primeira cena.
   - **Rejeição**: contraria a restrição explícita da SE (sem download; malhas procedurais) e introduz dependência de rede/licença a auditar sem necessidade — o GDD §13.2 diz que a baixa poligonalidade **complementa** o greybox, não que exige asset externo (`:1036-1038`). Fica como opção futura, condicionada a autorização de host e auditoria de licença; não é adotada agora.

3. **Manter o cliente greybox 2D atual — rejeitada por não atender ao pedido redefinido.**
   - **Sketch**: continuar com `Camera2d` + `Sprite` e a geometria plana entregue no ciclo 023 (`main.rs:71-99`, `lib.rs:23-121`), argumentando que a validação da SE em 01:57Z (STATE:12) já cobre "main scene, câmera e ilhas".
   - **Appetite**: zero — é o status quo.
   - **Trade-offs**: evita qualquer trabalho novo e preserva a fatia validada; entrega exatamente o que a SE pediu para **substituir** — não há câmera 3D, não há malhas 3D procedurais e a estética segue presa ao greybox 2D.
   - **Rabbit holes**: confundir "o greybox foi validado" com "o pedido redefinido foi atendido"; a validação de STATE:12 é anterior à redefinição (02:04Z) e não cobre low-poly. Também deixaria o projeto em desacordo com a direção visual normativa do GDD (`:1037,1042,1044,1237`).
   - **Rejeição**: não atende literalmente o item 1 redefinido pela SE; é a opção "não fazer" disfarçada de decisão, sem valor de prova para a mudança pedida.

## Recomendação

**Opção 1.** É a única que satisfaz simultaneamente a redefinição da SE (câmera 3D, main scene, ilhas), as restrições do ciclo (sem download de assets, malhas procedurais, sem textura, janela normal só a pedido / minimizada em validação) e a norma (GDD §13.2 e orçamento de triângulos; ADR-004). A Opção 2 é rejeitada por introduzir dependência de rede e licença a auditar, com host fora da allowlist — fica como alternativa futura condicionada; a Opção 3 é rejeitada por ser o status quo 2D que a redefinição veio corrigir. A recomendação é de conceito: representação exata das malhas, enquadramento da câmera, controles, fronteira com `insurgencia-sim`, contagem de triângulos e versões pertencem ao handoff `/speckit.specify`. Nenhuma alegação de compilação, renderização, jogabilidade ou número de triângulos medido é feita aqui.

## Fora de escopo (opção recomendada)

- Nesta etapa: qualquer crate, `Cargo.toml`, spec, plano, task, código, gate, teste, build, formatação, janela, câmera, render, captura, stage, commit ou push; escrita fora dos cinco artefatos de assessment; consulta a hosts externos ou download de assets.
- No item 1/ciclo de implementação: texturas, arquivos de modelo externos, hosts fora da allowlist, auditoria de licença de assets (adiada com a Opção 2), lobby, pareamento, 5v5, NAT/UDP externo, servidor remoto, itens 5/6 (STATE:10), reabertura dos itens 2–4, remoção do histórico GDScript, FFI, arte AAA, troca de engine (ADR-004:53), reabertura da ADR-001 e alteração de GDD, critérios §16.4, ADR-002/003, baseline de converge ou `Makefile`.
- Revogação silenciosa da validação da SE (STATE:12) ou dos critérios §16.4 — a redefinição evolui o item 1, não apaga o que foi entregue.
- Não se afirma nesta etapa compilação, renderização, captura/frame, desempenho, integração com `insurgencia-sim`, contagem real de triângulos ou validação ao vivo — tudo permanece **[NV]**.

## Suposições a validar

- Uma `App` Bevy 0.20.1 com `Camera3d` e malhas procedurais simples é suficiente para uma main scene low-poly que atenda GDD:1035-1046,1042,1044 e o orçamento de `:1126-1127`, mantendo o gate verde — sem benchmark ou protótipo executado nesta etapa.
- A evolução reaproveita `insurgencia-cliente` e o espelho `insurgencia-sim` sem retrabalho na fronteira; nome/API e a eventual necessidade de `Lightyear` no cliente single-player serão decididos na spec.
- A conversão de `ILHAS`/`Forma` (2D) para geometria 3D e a definição de "jogável" no item 1 redefinido pertencem ao handoff; o item 1 não precisa, por ora, implementar os desafios das ilhas.
- A regra da janela (`--minimizado` para agente, normal só a pedido da SE; STATE:13) sobrevive à troca de câmera sem alteração no comportamento observado no greybox (`main.rs:37-38,60-69`).
- Ferramenta de medição de triângulos, toolchain 1.99.1 e MSRV registrado seguem como fatos a verificar no ciclo de implementação, não como compilação observada nesta etapa.
