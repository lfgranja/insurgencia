# Problem — Lacuna: o cliente atual é 2D (greybox), não low-poly 3D

**Data:** 2026-11-21

## Problema

A SE redefiniu em 2026-11-21T02:04Z o item 1 do entregável: de "cliente greybox jogável (main scene, câmera, ilhas geométricas)" para "**cliente low-poly jogável (main scene, câmera, ilhas)**", com malhas procedurais e sem download de assets. O cliente entregue no ciclo 023 (PR #29) e validado pela SE em 01:57Z (STATE:6,12) **existe e funciona**, mas é bidimensional: `commands.spawn(Camera2d)` e ilhas/avatar desenhados como `Sprite` (`crates/cliente/src/main.rs:71-99`), com geometria plana em `Forma`/`Ilha`/`ILHAS` (`crates/cliente/src/lib.rs:23-121`). Há, portanto, uma lacuna entre o pedido redefinido — câmera 3D, malhas 3D procedurais, sem textura, silhueta+cor, ≤ 10 mil triângulos por personagem (GDD:1035-1046,1126-1127) — e a implementação 2D vigente. Não é a ausência de cliente do ciclo 023: é um **desalinhamento de dimensão e estética** entre o que existe e o que a SE agora pede.

## Por que é válido

- **Impacto 4** (escala 1–5) — alto, mas não máximo. Diferente do ciclo 023 (impacto 5: nenhum cliente existia), aqui há uma base entregue, mesclada e validada (STATE:6,12; cinco crates no workspace, STATE:5), e os itens 2–4 já estão concluídos (STATE:7-9) — o entregável não está travado. O impacto permanece em 4 porque o pedido da SE é uma mudança **visual e estrutural** do item 1 (câmera 3D + malhas procedurais em lugar de `Camera2d` + `Sprite`), afeta a face mais visível do produto e é a direção de estética normativa do GDD (`:1037,1042,1044,1237`). O desconto a 4 (e não 5) é registrado: não há risco de os itens 2–4 ficarem sem suporte, e a transição pode reaproveitar `insurgencia-cliente`, o workspace e a governança já verdes. [source: handoff da SE; STATE:5-14; GDD:1037,1042-1044,1237; inspeção de `crates/cliente/`]
- **Confiança 4** (escala 1–5) — a lacuna é inequívoca e verificada por inspeção do código nesta etapa (`Camera2d` em `main.rs:71`, `Sprite` em `:76-99`, `Forma`/`Ilha` 2D em `lib.rs:23-121`), e duplamente ancorada: a redefinição é da SE (2026-11-21T02:04Z, via handoff) e o GDD fixa a direção visual low-poly e o orçamento de triângulos (`:1035-1046,1126-1127`). O desconto a 4 deve-se a quatro incertezas declaradas: (1) a redefinição ainda não foi reescrita no STATE vivo (01:26Z, que descreve o item 1 como greybox); (2) "jogável" não tem definição operacional no pedido redefinido — controles, enquadramento 3D e nível dos desafios não fixados; (3) nenhuma malha, câmera 3D, render ou captura foi executada, e a validação da SE cobre apenas o greybox 2D (STATE:12); (4) a conversão de `ILHAS`/`Forma` para 3D e a aplicação exata do limite de 10 mil triângulos não foram decididas. [source: handoff da SE; STATE:6,12; GDD:1035-1046,1126-1127; inspeção de `crates/cliente/`]
- **Custo de não agir** — a SE fica sem a fatia low-poly pedida e o projeto permanece preso à estética greybox 2D, enquanto o GDD já aponta low-poly/FOSS como direção visual (`:1037,1237`). Diferente do ciclo 023, não há bloqueio dos itens seguintes; o risco é de **desalinhamento com a autoridade e com a norma de produto**, agravado pelo churn de API do Bevy 0.x registrado na ADR-004:33 — quanto mais tarde a mudança visual, mais caro retrabalhar o cliente. Corrigir agora reusa o crate e o gate já verdes. [source: handoff da SE; GDD:1037,1237; ADR-004:33; STATE:6,12-14]

## Afetados & stakeholders

- **Usuários**: mantenedor único — precisa de uma fatia low-poly 3D observável no mesmo workspace Rust já verde; jogadores — servidos indiretamente, pois o objetivo do MVP (GDD:1509) e as métricas da §17 dependem de uma partida jogável, agora com a estética normativa do GDD §13.2.
- **Stakeholders**: SE — autoridade que redefiniu o item 1 em 2026-11-21T02:04Z e cuja validação visual anterior cobre apenas o greybox (STATE:12); Núcleo/`insurgencia-sim` — espelho autoritativo já consumido pelo cliente (`main.rs:12,117-120`), cuja fronteira com a cena 3D será definida na spec; ADR-004 — norma de stack e governança que o cliente deve respeitar sem nova troca de engine (`:17,53`); GDD §13.2 — norma de estética (sem textura, silhueta+cor, orçamento de triângulos).

## Metas

1. **Elevar o cliente existente a low-poly 3D** — evoluir o crate `insurgencia-cliente` para abrir uma **main scene real** com **câmera 3D** e as ilhas do Arquipélago representadas por **malhas procedurais** geradas em código, sem download de assets e sem arquivos de modelo externos. Nada de recolorir `Sprite` 2D e chamá-lo de low-poly.
2. **Cumprir a direção visual normativa** — 100% dos personagens jogáveis sem textura aplicada (GDD:1042), ilhas reconhecíveis por silhueta e cor sem letreiro (`:1044`), ausência de textura como leitura de estado (`:1038`) e orçamento de ≤ 10 mil triângulos por personagem (`:1126-1127`). Manter a complementaridade greybox/low-poly do GDD (`:1036-1038`).
3. **Preservar o que já foi entregue** — não reabrir os itens 2–4 (STATE:7-9), não revogar critérios §16.4 nem métricas §17, manter itens 5/6 DEFER (STATE:10), nenhuma revogação silenciosa e nenhuma troca de engine sem novo ADR (ADR-004:53).
4. **Manter a regra da janela e o modo single-player** — rodar **normal** somente se a SE pedir; em inspeção/debug/testes/validação de agente, **minimizado** com `--minimizado` (STATE:13; `AGENTS.md:42-47`). Sem lobby, pareamento, 5v5, NAT/UDP externo ou servidor remoto.
5. **Aplicar a governança e o fluxo SDD** — `unsafe forbid`, `clippy pedantic deny`, zero `.unwrap()`/`.expect()`, `Result`/`?`/`thiserror`/`anyhow`, versões fixadas com justificativa no commit, `./verify.sh` como gate, e conversão do assessment em spec/clarificações/plano/tasks antes de qualquer implementação, com R1 `go` e R4 precedendo R7 (`.specify/memory/constitution.md:11`).

## Limites

Fora de escopo nesta etapa: escrita de qualquer crate, `Cargo.toml`, spec, plano ou task; execução de `cargo`, `./verify.sh`, clippy, nextest, testes, build, formatação, janela, câmera, render, captura ou gates; download de assets, fetch a `kenney.nl` ou consulta a hosts fora da allowlist; stage, commit, push, merge, deploy. Fora do item 1 redefinido (mas não revogados): itens 2–4 já entregues (STATE:7-9), itens 5/6 diferidos (STATE:10), critérios §16.4 e métricas §17. Também fora: remoção/reescrita do histórico GDScript, FFI para manter GDScript vivo, arte AAA, texturas, troca de engine, reabertura da ADR-001 e alteração de GDD/ADRs/baseline de converge. Nenhuma alegação de compilação, renderização, jogabilidade, desempenho, número de triângulos medido ou validação ao vivo do cliente low-poly é feita — tudo permanece **[NV]**.

## Critérios de sucesso

- Cliente `insurgencia-cliente` evoluído no workspace com **câmera 3D** e ilhas/avatar em **malhas procedurais** (baseline: `Camera2d` + `Sprite` em `main.rs:71-99`; meta: `Camera3d` + malhas 3D geradas em código, rastreadas no ciclo de implementação).
- **Nenhuma textura e nenhum download**: ausência de assets externos, de arquivos de modelo e de fetch a hosts externos no caminho do cliente (baseline: `bevy` com features padrão e nenhum asset visual; meta: geometria 3D inteiramente procedural, verificável por inspeção de código e dependências).
- **Direção visual do GDD rastreada**: 100% sem textura, ilha por silhueta+cor, orçamento ≤ 10 mil triângulos/personagem como contrato observável (baseline: norma em GDD:1042,1044,1126-1127; meta: rastreabilidade explícita na spec e medição no ciclo de implementação).
- **Regra da janela preservada**: `--minimizado` em validação de agente, normal só a pedido da SE (baseline: implementado no greybox, `main.rs:37-38,60-69`; meta: mesmo comportamento no cliente low-poly).
- **Estado verde mantido e citado**: `./verify.sh` ALL STAGES OK e gates 5/5 no ciclo de implementação (baseline: 5/5, ALL STAGES OK, 34 testes e 31 CI em STATE:14; meta: mesmo resultado com o cliente 3D incluído), sem stage/commit/push nesta etapa.

## Perguntas abertas

- **[NV]** O que exatamente significa "low-poly jogável" no item 1 redefinido: basta câmera 3D + malhas procedurais navegáveis, ou os desafios das ilhas precisam estar implementados?
- **[NV]** Como converter as quatro ilhas atuais (`ILHAS`/`Forma` 2D em `lib.rs:23-121`) para malhas 3D — geometria, coordenadas, silhueta e cor por ilha?
- **[NV]** O orçamento de 10 mil triângulos (GDD:1126-1127) aplica-se a personagens, ilhas ou ambos, e como medi-lo sem ferramenta executada?
- **[NV]** Fronteira da cena 3D com `insurgencia-sim` e `insurgencia-contratos`, necessidade de `Lightyear` no cliente single-player e versões fixadas das dependências visuais.
- **[NV]** Reconciliar a redefinição (2026-11-21T02:04Z) com o STATE vivo (01:26Z, item 1 ainda descrito como greybox) sem reescrever o histórico nem revogar a validação da SE (STATE:12).
