# Concept — Como entregar o cliente greybox jogável do item 1

**Data:** 2026-11-21 · **Opção recomendada:** Opção 1 — crate `insurgencia-cliente` com `App` Bevy, câmera e formas geométricas

## Contexto

A SE reorientou a prioridade em 2026-11-21T00:22Z: MVP single-player cujo item 1 é "cliente greybox jogável (main scene, câmera, ilhas geométricas)" (`.specify/memory/STATE.md:5`). O workspace já tem `contratos` e `sim` mesclados e verdes (PR #28, HEAD `a2c8b22`; `./verify.sh` → ALL STAGES OK com 15 testes — `specs/022-simulacao-rust/converge.md:13`), mas não há cliente algum (impacto 5, confiança 4). A ADR-004 autoriza Rust 1.99.1 + Bevy 0.20.1 + Lightyear 0.31.0 e veta GDScript como implementação e novas trocas de engine sem novo ADR (`:17,53`); o GDD define greyboxing como formas geométricas sem textura (`:1486`) e o mapa do MVP como três ilhas de rota + Ilha Central (`:1492`). As restrições do item 1 são duras: single-player, sem lobby/5v5/NAT, sem servidor remoto, main scene real, câmera e ilhas geométricas (STATE:5-8). Abaixo, três opções no nível de conceito — não é especificação, plano ou arquitetura.

## Opções consideradas

1. **Crate `insurgencia-cliente` com `App` Bevy + câmera + shapes — escolhida.**
   - **Sketch**: criar, no workspace `crates/` já existente, um crate Rust que inicializa uma `App` Bevy com plugins mínimos (janela, renderização, câmera 2D ou 3D — decisão de spec) e monta uma **main scene real**: o Arquipélago da Insurgência representado por formas geométricas sem textura (planos/cubos para as três ilhas de rota e a Ilha Central), com a câmera enquadrando o conjunto. Execução inteiramente local e single-player: nenhum plugin de rede, nenhum lobby, nenhum endereço de servidor, nenhuma sessão 5v5. Erros tipados com `Result`/`thiserror`/`anyhow`, governança `mdmv-rust-thin` desde o primeiro arquivo e dependências fixadas; integração futura com `insurgencia-sim` reservada, sem antecipá-la neste item.
   - **Appetite**: small (dias) para a fatia jogável mínima — reutiliza a toolchain, o workspace e o padrão de governança já aplicados em `contratos`/`sim`; dependências visuais e o nível de implementação dos desafios podem empurrar para medium, registro como orçamento, não medição.
   - **Trade-offs**: ganha alinhamento direto com a ADR-004 (mesma engine e mesmo gate do Núcleo), uma cena observável que prova a tese visual e um caminho natural para os itens 2/3 (cliente modificado e instrumentação); sacrifica a aparente simplicidade de um protótipo descartável e exige disciplina de dependências/lints desde o início. Risco: o Bevy 0.x tem churn de API registrado na ADR-004:33.
   - **Rabbit holes**: escolha 2D vs. 3D e enquadramento da câmera; definição de "jogável" (só navegação vs. desafios implementados); dependências de render/MSRV e fixação de versões; fronteira com `insurgencia-sim` — se o item 1 começar a consumir o Núcleo antes da spec, o escopo estoura.
2. **Protótipo em HTML/canvas (navegador) — rejeitada.**
   - **Sketch**: gerar uma página web com `<canvas>` e JavaScript desenhando as ilhas geométricas, aberta no navegador como "cliente" do MVP.
   - **Appetite**: small (dias), aparentemente o caminho mais rápido para algo visível.
   - **Trade-offs**: produz uma imagem rapidamente e facilita compartilhamento; introduz uma segunda stack (HTML/JS) fora da ADR-004, com toolchain, dependências e gate próprios, sem reutilizar o workspace Rust nem os crates mesclados.
   - **Rabbit holes**: cadeia de suprimentos e lints próprios (fora de `./verify.sh` e da governança `mdmv-rust-thin`); divergência entre o protótipo web e o cliente real, exigindo reescrita; nenhuma prova sobre a engine autorizada (Bevy) — o que é justamente a tese a validar; risco de virar "cliente de fachada" com catch-up depois.
   - **Rejeição**: quebra a stack autorizada e o gate determinístico (ADR-004:17,19-24), não exercita Bevy e cria dívida de reescrita sem reduzir o risco do item 1. Nenhuma troca de engine/stack sem novo ADR (ADR-004:53).
3. **Manter a tela de terminal / saída de texto como "cliente" — rejeitada.**
   - **Sketch**: usar o estado atual (logs, saída de terminal ou um headless que imprime o estado das ilhas) como prova do MVP, sem janela, câmera ou geometria.
   - **Appetite**: small (dias) — na prática, quase zero: é o status quo.
   - **Trade-offs**: evita qualquer trabalho visual e permite inspecionar o estado do Núcleo; entrega exatamente o que a SE pediu para **não** entregar — não há main scene, não há câmera e não há ilhas geométricas.
   - **Rabbit holes**: confundir "o Núcleo roda" com "o cliente é jogável"; os itens 2 (cliente modificado) e 3 (métricas de percepção/jogabilidade) perdem o suporte visual; nenhuma captura/frame para provar a tese do GDD:1509.
   - **Rejeição**: contraria literalmente o item 1 da SE (STATE:5) e o greybox normativo do GDD (`:1486,1492`); é a opção "não fazer" disfarçada de cliente, sem valor de prova.

## Recomendação

**Opção 1.** É a única que satisfaz simultaneamente as metas do problema e as restrições da SE: materializa o cliente no workspace Rust autorizado, entrega main scene real com câmera e ilhas geométricas, permanece single-player e sem rede, e mantém o `./verify.sh` e a governança `mdmv-rust-thin` já aplicados em `contratos`/`sim`. A Opção 2 é rejeitada por introduzir stack estranha à ADR-004, não exercitar Bevy e criar reescrita certa; a Opção 3, por não ser cliente jogável — é o status quo que a interjeição veio corrigir. A recomendação é de conceito: janela 2D/3D, controles, fronteira com o Núcleo, versões e implementação pertencem ao handoff `/speckit.specify`, não a esta etapa. Nenhuma alegação de compilação, renderização ou jogabilidade é feita aqui.

## Fora de escopo (opção recomendada)

- Nesta etapa: qualquer crate, `Cargo.toml`, spec, plano, task, código, gate, teste, build, formatação, janela, câmera, render, captura, stage, commit ou push; escrita fora dos cinco artefatos de assessment; consulta a hosts externos.
- No item 1/ciclo de implementação: lobby, matchmaking, 5v5, NAT/UDP externo, servidor remoto, dois servidores com Módulos diferentes, teste de queda do servidor de referência, servidores comunitários e forks (STATE:6-8); remoção/reescrita do histórico GDScript; FFI para manter GDScript vivo; arte AAA; troca adicional de engine (ADR-004:53); reabertura da ADR-001; alteração de GDD, critérios §16.4, ADR-002/003, baseline de converge, `Makefile` ou `evidence.csv`.
- Revogação silenciosa dos critérios §16.4: eles continuam norma para os itens 2/3 (GDD:1567; STATE:5).
- Não se afirma nesta etapa compilação, renderização, captura/frame, desempenho, integração com `insurgencia-sim` ou validação ao vivo — tudo permanece **[NV]**.

## Suposições a validar

- Uma `App` Bevy com plugins mínimos e formas geométricas simples é suficiente para uma main scene com câmera que prove o greybox do GDD:1486,1492 dentro de um appetite small — sem benchmark ou protótipo executado nesta etapa.
- O nome `insurgencia-cliente` e a existência do crate em `crates/` são propostas de conceito, sujeitas ao fluxo SDD; nenhuma estrutura é materializada aqui.
- Câmera 2D ou 3D, controles, disposição das ilhas e o significado exato de "jogável" no item 1 serão decididos na spec; o item 1 não precisa, por ora, implementar os desafios das ilhas.
- A fronteira com `insurgencia-sim` e `insurgencia-contratos` pode esperar o item 2 (cliente modificado) sem retrabalho — risco a confirmar, não fato.
- Toolchain `1.99.1` e MSRV `1.97.1` do crate publicado seguem como registros datados da ADR-004/converge 021, não como compilação observada nesta etapa.
