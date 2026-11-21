# Concept — Como materializar a simulação Rust do Núcleo

**Data:** 2026-11-20 · **Opção recomendada:** Opção 1 — crate `insurgencia-sim` com `Mundo` + `ValidacaoComandos`

## Contexto

A ADR-004 autorizou a reescrita do Núcleo para Rust 1.99.1 + Bevy 0.20.1 + Lightyear 0.31.0 e abandonou GDScript como linguagem de implementação, preservando GDD, regras de ilha, condição de vitória e critérios §16.4 como norma (`docs/adr/004-migracao-bevy-lightyear.md:11-24`). O ciclo 021 mesclou o workspace e `crates/contratos`, com `./verify.sh` → ALL STAGES OK e 7 testes, mas `crates/` não tem crate de simulação (problema: impacto 5, confiança 4). A referência semântica é `nucleo/mundo.gd` (30 Hz, três fases, ilhas/funções, `estado_persistido`, `hash_estado` SHA-256) e `nucleo/sim/validacao.gd` (6 ações, limites, 8 códigos de recusa). A governança `mdmv-rust-thin` impõe `unsafe forbid`, `clippy pedantic deny`, sem `unwrap`/`expect`, `Result` + `thiserror`/`anyhow`, `./verify.sh` e versões fixadas. Nenhuma opção abaixo autoriza, nesta etapa, spec, plano, tasks, código, gate, teste, build, formatação, stage/commit/push ou escrita fora dos cinco artefatos de assessment.

## Opções consideradas

1. **Crate `insurgencia-sim` com `Mundo` + `ValidacaoComandos` — escolhida.**
   - **Sketch**: criar, no workspace `crates/` já existente, um crate Rust que reexprima o `Mundo` e a `ValidacaoComandos` do GDScript **sem mudar semântica**: mesmo `PASSO_SEGUNDOS = 1/30`, mesma ordem de fases (`prepare` → `skills` → `finish`), mesmas 6 ações, contagem de ilhas, funções, `estado_persistido`, vitória da Ilha Central e `hash_estado`; a validação mantém os mesmos limites e os 8 códigos de recusa. Erros tipados com `Result`/`thiserror`, nenhum `unsafe`, integração com `insurgencia-contratos` e vetores de paridade com o GDScript na spec.
   - **Appetite**: medium (semanas) — o port cobre dois módulos nomeados mais as cinco colaboradoras de `nucleo/mundo.gd:5-9`, cujas fontes não foram lidas nesta etapa; a estimativa é orçamento, não medição.
   - **Trade-offs**: ganha implementação Rust-only alinhada à ADR-004, limites de módulo auditáveis e gate aplicável desde o primeiro arquivo; sacrifica a simplicidade de manter a lógica em GDScript e exige prova de equivalência (especialmente do hash e do tempo `float(tick) * PASSO_SEGUNDOS`).
   - **Rabbit holes**: paridade de `str()`/ordem de `Dictionary` no `hash_estado`; arredondamento `f32`/`f64` em capturas e canalização; descoberta tardia do escopo das ilhas/funções por `preload`; API pública e fronteira `dominio/sim` vs. `crates/`.
2. **Reutilizar GDScript via FFI — rejeitada.**
   - **Sketch**: manter `nucleo/mundo.gd` e `nucleo/sim/validacao.gd` como código vivo e expor uma camada Rust por FFI (*interface de função estrangeira*) para "cumprir" a migração.
   - **Appetite**: small (dias) para o wrapper, mas com dívida operacional permanente.
   - **Trade-offs**: evita reescrever as regras; mantém duas linguagens, dois ciclos de teste e o GDScript como implementação viva — exatamente o risco que a ADR-004 descarta para o "serviço Rust separado mantendo Godot no cliente" (`:38`).
   - **Rabbit holes**: FFI em código de produção colide com `unsafe_code = "forbid"` da governança e exigiria exceção isolada; equivalência com a ADR-004 é apenas nominal; o histórico GDScript deixaria de ser registro para voltar a ser código vivo.
   - **Rejeição**: contradiz a ADR-004, que determina abandonar GDScript como linguagem de implementação e adotar Rust como stack do Núcleo e do servidor (`:17`), além de violar `unsafe forbid`.
3. **Portar apenas `Mundo`, sem validação — rejeitada.**
   - **Sketch**: portar o ciclo de simulação e as ilhas/funções, mas confiar no cliente para `seq`, limites de movimento, alvos e recursos, omitindo `ValidacaoComandos`.
   - **Appetite**: small (dias), aparentemente mais barato.
   - **Trade-offs**: reduz o escopo imediato; elimina justamente as verificações anti-replay e anti-fraude que sustentam a autoridade do servidor.
   - **Rabbit holes**: ausência de `SEQ_OU_REQUEST_REPLAY`, `MOVIMENTO_FORA_DO_LIMITE`, `PLACA_INVALIDA`, `ALAVANCA_INVALIDA`, `MATERIAIS_FORJADOS` e `RECURSO_FORJADO` altera semântica observável e exigiria reintrodução depois — com risco de divergência dos critérios §16.4.
   - **Rejeição**: muda a semântica que se propõe preservar; a validação é parte do contrato de autoridade registrado desde o ciclo 009 (`.specify/memory/STATE.md:55-57`), não acessório opcional.

## Recomendação

**Opção 1.** É a única que satisfaz simultaneamente as metas do problema: materializa o crate de simulação no workspace Rust, preserva o GDD como contrato de comportamento e mantém a validação e o hash com semântica equivalente ao GDScript de referência, sob a governança já aplicada em `crates/contratos`. A Opção 2 é rejeitada por ser migração apenas nominal (GDScript continuaria vivo, com FFI e risco `unsafe`); a Opção 3, por alterar o comportamento observável e remover a autoridade de validação do servidor. A recomendação é de conceito: a implementação, os testes de paridade e o preenchimento da spec pertencem ao handoff `/speckit.specify`, não a esta etapa.

## Fora de escopo (opção recomendada)

- Nesta etapa: qualquer crate, `Cargo.toml`, spec, plano, task, código, gate, teste, build, formatação, stage, commit ou push; escrita fora dos cinco artefatos de assessment.
- No ciclo de implementação: remoção ou reescrita do histórico GDScript; troca adicional de engine (ADR-004:53); reabertura da ADR-001; alteração de GDD, critérios §16.4, ADR-002/003, baseline de converge, `Makefile` ou `evidence.csv`.
- Não se afirma nesta etapa compilação, equivalência de hash, desempenho, reconexão ou validação ao vivo — tudo permanece **[NV]**.

## Suposições a validar

- A equivalência comportamental do GDScript pode ser expressa em Rust, inclusive `hash_estado()` com ordem de serialização controlada (vetores de paridade a definir na spec).
- As cinco colaboradoras por `preload` em `nucleo/mundo.gd:5-9` entram no escopo do port; suas fontes precisam ser lidas e seu comportamento fixado antes da implementação.
- O nome `insurgencia-sim` e a fronteira entre `dominio/sim` e `crates/` são propostas de conceito, sujeitas ao fluxo SDD; nenhuma estrutura é materializada aqui.
- Toolchain `1.99.1` e MSRV `1.97.1` do crate publicado seguem como registros datados da ADR-004 e do converge 021, não como compilação observada nesta etapa.
