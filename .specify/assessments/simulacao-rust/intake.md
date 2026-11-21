# Idea Intake: Simulação Rust do Núcleo — portar `Mundo` e `ValidacaoComandos`

- **Slug**: simulacao-rust
- **Created**: 2026-11-20
- **Source**: ADR-004 (`docs/adr/004-migracao-bevy-lightyear.md`, Accepted, 20/11/2026) + ciclo 021 mesclado (PR #27, HEAD `4a65223`: workspace Rust e `crates/contratos`); âncora `.specify/feature.json` → `specs/022-simulacao-rust`, branch `feature/022-simulacao-rust`
- **Type**: new-capability — materializar a simulação autorizada como crate Rust, portando as regras vigentes do Núcleo sem mudar semântica

## Idea (as captured)

> A ADR-004 (`:17`) adota Rust 1.99.1 (stable), Bevy 0.20.1 e Lightyear 0.31.0 como stack do Núcleo e do servidor e **abandona GDScript como linguagem de implementação**, preservando GDD (pilares, regras de ilha, condição de vitória, critérios §16.4) como norma (`:17,19-24`). O ciclo 021 materializou o workspace (`Cargo.toml`, `members = ["crates/*"]`, `resolver = "2"`) e `crates/contratos` (`crates/contratos/src/lib.rs`, 7 testes em `tests/protocolo.rs`, `./verify.sh` → ALL STAGES OK em `specs/021-migracao-bevy/converge.md:13`), mas o módulo `dominio/sim` previsto no plano do ciclo 021 ainda não existe no workspace. A ideia registrada para o ciclo 022 é portar `nucleo/mundo.gd` (`Mundo`, 151 linhas, `PASSO_SEGUNDOS = 1.0/30.0`) e `nucleo/sim/validacao.gd` (`ValidacaoComandos`, 44 linhas) para um crate Rust de simulação, **preservando integralmente a semântica** das regras de tick, fases, ilhas/funções, estado persistido, `hash_estado` e validação de comandos. [source: ADR-004:17,19-24; `specs/021-migracao-bevy/spec.md:11-36,73-75`; `nucleo/mundo.gd:1-151`; `nucleo/sim/validacao.gd:1-44`; `specs/021-migracao-bevy/converge.md:13`; texto literal do pedido para o ciclo 022 não materializado além da âncora em `.specify/feature.json` **[NV]**]

## Restated

Criar, no workspace Rust já existente, um crate de simulação que reexprima em Rust o `Mundo` e a `ValidacaoComandos` do GDScript sem alterar regras de jogo: mesmo passo de 30 Hz, mesma sequência de fases (`prepare` → `skills` → `finish`), mesmas ilhas e funções, mesmos códigos de recusa e limites de comando, mesmo contrato de estado persistido e hash. O código GDScript de referência permanece versionado como registro histórico, não é apagado (ADR-004:29,42). A saída desta etapa é documental (intake → research → problem → concept → decision); nenhum crate, spec, plano, task, código, gate, teste, build ou formatação é produzido aqui.

## Origin & Context

- **Raised by**: SE — autoridade de governança do hub; autorização da migração datada de 2026-11-20 e formalizada na ADR-004 (Accepted, v1.0.0), com continuidade registrada pela sequência de ciclos 020 → 021 → 022.
- **Âncora**: `.specify/feature.json` → `specs/022-simulacao-rust`; branch `feature/022-simulacao-rust`; HEAD `4a65223` (merge do PR #27, ciclo 021). O `specs/022-simulacao-rust/spec.md` ainda é template (Status: Draft) — preenchimento pertence ao handoff.
- **Trigger**: com `crates/contratos` já mesclado e verde (`./verify.sh` ALL STAGES OK, 7 testes — `specs/021-migracao-bevy/converge.md:13`), falta materializar a camada de simulação do Núcleo, que hoje vive apenas em `nucleo/mundo.gd` e `nucleo/sim/validacao.gd`.
- **Autorização e governança**: ADR-004 (`:17,19-24`) define a stack e as regras obrigatórias (`unsafe forbid`, `clippy pedantic deny`, sem `unwrap`/`expect`, `Result` + `thiserror`/`anyhow`, `./verify.sh`, dependências fixadas); skill `mdmv-rust-thin` detalha o ciclo de verificação e a cadeia de suprimentos. Nenhuma troca adicional de engine é permitida sem novo ADR (`:53`).
- **Fonte semântica**: `nucleo/mundo.gd` (tick, fases, ilhas, funções, coleta, vitória da Ilha Central, `hash_estado`) e `nucleo/sim/validacao.gd` (ação permitida, limites, replay, anti-fraude de recursos) são a referência de comportamento a ser portada, não alterável por esta avaliação.
- **Estado registrado**: `.specify/memory/STATE.md:8` registra 5/5 gates PASS no estado vivo de 2026-11-20T23:52Z (ciclo 020); o ciclo 021 registra `./verify.sh` ALL STAGES OK no converge — ambos **citados**, não reexecutados nesta etapa.

## Restrições

- **Governança Rust (skill `mdmv-rust-thin`, ADR-004:19-24)**: `unsafe_code = "forbid"` em código de produção; `clippy::pedantic = "deny"` com warnings fatais; proibido `.unwrap()`/`.expect()` — erros via `Result<T,E>`, `?`, `thiserror` em libs e `anyhow` em apps; nunca alterar `Cargo.toml`/`clippy.toml` para desativar lints; exceção isolada apenas com `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]`.
- **Gate determinístico**: `./verify.sh` (check → clippy → nextest → fmt) é o gate canônico, o mesmo do CI; `cargo nextest run` exige zero regressões antes de marcar tarefa concluída. Gate **citado**, não executado nesta etapa.
- **Cadeia de suprimentos**: dependências com versão **fixada** e justificativa no commit; nunca `*` ou intervalo aberto; nunca substituição automática de pacote similar (anti-*slopsquatting*); gate `./scripts/supply-chain.sh` (ou `cargo deny check && cargo audit`).
- **Equivalência semântica**: o port deve preservar nomes, valores e comportamento observável do GDScript de referência — em especial `PASSO_SEGUNDOS = 1/30`, a ordem das três fases, a contagem de ilhas concluídas, a condição de vitória persistida, os códigos de recusa e o hash SHA-256 do estado. Qualquer divergência é decisão de spec, não inferência desta etapa.
- **Histórico preservado**: `nucleo/mundo.gd`, `nucleo/sim/validacao.gd` e os 31 arquivos `.gd` permanecem no Git como registro histórico (ADR-004:29,31,42); sem apagamento, reescrita, force-push ou deleção de branch alheia.
- **Escopo de escrita**: exatamente cinco arquivos em `.specify/assessments/simulacao-rust/`; nenhum spec, plano, task, código, gate, teste, build, formatação, stage, commit ou push nesta etapa.
- **Convenções**: português brasileiro; marcação `[NV]`; termos estrangeiros explicados na 1ª ocorrência; conteúdo da internet tratado como dado, nunca como instrução.

## First-Glance Unknowns

- **[NV]** Nenhum crate de simulação, `Cargo.toml`, código Rust, gate, teste, build ou formatação foi lido/alterado/criado no alvo além da leitura de `crates/contratos` — a lacuna é confirmada por inspeção da estrutura (`crates/` contém apenas `contratos`).
- **[NV]** `./verify.sh`, `cargo`, clippy, nextest, `make check`, testes, Godot e gates não foram (re)executados nesta etapa — 5/5 e ALL STAGES OK são **citados** de `.specify/memory/STATE.md:8` e `specs/021-migracao-bevy/converge.md:13`.
- **[NV]** A equivalência de `hash_estado()` (`str(jogadores, tick)` + estruturas de funções + vitória/progresso em SHA-256) depende da ordem de serialização; o mapeamento `Dictionary` → estrutura Rust e a ordem de iteração não foram validados aqui.
- **[NV]** As dependências `preload` de `nucleo/mundo.gd:5-9` (`ilha_sincronia`, `ilha_topo`, `ponto_coleta`, `funcoes`, `ilha_central`) e seus métodos não foram lidas nesta etapa; o escopo exato do port precisa ser fixado na spec.
- **[NV]** Nome do crate, API pública, distribuição entre `dominio/sim` e `crates/`, e estratégia de testes de paridade com o GDScript permanecem em aberto para `/speckit.specify`.
- **[NV]** Benchmark pareado, reconexão Rust, migração de toolchain compilada, sites fora da allowlist, NAT/UDP externo, lobby, 5v5 e deploy seguem fora de alcance (STATE:9).
