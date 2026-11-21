# Problem — Lacuna: nenhum crate de simulação no workspace Rust

**Data:** 2026-11-20

## Problema

A ADR-004 (Accepted, 20/11/2026) autorizou a reescrita do Núcleo e do servidor para Rust 1.99.1 + Bevy 0.20.1 + Lightyear 0.31.0, preservando GDD, regras de ilha, condição de vitória e critérios §16.4 como norma (`docs/adr/004-migracao-bevy-lightyear.md:11-24`). O ciclo 021 materializou o workspace e `crates/contratos` (PR #27, HEAD `4a65223`), mas **não existe nenhum crate de simulação no alvo**: `crates/` contém apenas `contratos`, e a simulação vigente permanece integralmente em GDScript — `nucleo/mundo.gd` (30 Hz, três fases por tick, ilhas/funções, estado persistido, `hash_estado`) e `nucleo/sim/validacao.gd` (ações, limites e códigos de recusa). Há, portanto, uma lacuna entre uma migração autorizada e já parcialmente executada e a ausência da camada de simulação em Rust; o `specs/022-simulacao-rust/spec.md` ainda é template Draft (`specs/022-simulacao-rust/spec.md:1-9`).

## Por que é válido

- **Impacto 5** (escala 1–5) — máximo: sem o crate de simulação, nenhuma regra de jogo migrada pode ser compilada, testada ou integrada ao servidor Rust autoritativo; a ADR-004 fica materializada apenas na camada de contratos. O port envolve o comportamento integral de `Mundo` e `ValidacaoComandos` (incluindo cinco colaboradores por `preload` e hash SHA-256 do estado), não é dívida documental nem ajuste incremental. Adiar a materialização prolonga a convivência entre uma decisão vigente e uma implementação de Núcleo que ela já não descreve. [source: ADR-004:17; `nucleo/mundo.gd:1-23,83-151`; `nucleo/sim/validacao.gd:1-44`; inspeção de `crates/`]
- **Confiança 4** (escala 1–5) — a lacuna é inequívoca e verificada por inspeção da estrutura do alvo nesta etapa, e duplamente ancorada: a ADR-004 registra a autorização e a governança (`:11-24`), o ciclo 021 registra `./verify.sh` → ALL STAGES OK com 7 testes apenas em `contratos` (`specs/021-migracao-bevy/converge.md:8-13`) e o STATE registra os `[NV]` remanescentes, incluindo "migração de toolchain" (`.specify/memory/STATE.md:5-9`). O desconto a 4 (e não 5) deve-se a três incertezas declaradas: (1) as fontes dos cinco colaboradores de `nucleo/mundo.gd:5-9` não foram lidas, então o escopo integral do port ainda não está fixado; (2) a equivalência de `hash_estado()` depende de ordem de serialização não medida; (3) nenhum teste de paridade, build ou compilação foi executado nesta etapa — MSRV/toolchain são registros citados, não fatos medidos.
- **Custo de não agir** — o workspace Rust segue com contratos, mas sem Núcleo: o histórico GDScript permanece como código vivo quando a ADR-004 já o declara registro histórico; o próximo ciclo começaria com spec em template, sem fronteira de crate definida e sem vetores de equivalência. Corrigir cedo é caro mas rastreável; corrigir depois de novas alterações em GDScript aumentaria o churn e o risco de perder equivalência com os critérios §16.4.

## Afetados & stakeholders

- **Usuários**: mantenedor único — opera o workspace Rust e precisa de uma simulação compilável sob `mdmv-rust-thin`; jogadores — servidos indiretamente, pois as regras de ilha, vitória e validação não podem mudar com o port.
- **Stakeholders**: SE — autoridade que emitiu a autorização na ADR-004 e cobra a continuidade dos ciclos; servidor autoritativo — consumidor final da validação e do hash de estado (registro do ciclo 009 em `.specify/memory/STATE.md:55-57`).

## Metas

1. **Materializar o crate de simulação** — criar o crate Rust no workspace `crates/` (layout Rust-only), reexpressando `Mundo` e `ValidacaoComandos` com nomes, valores e comportamento observável equivalentes ao GDScript de referência: passo `1/30 s`, fases `prepare` → `skills` → `finish`, ações e ilhas, `estado_persistido`, vitória da Ilha Central e `hash_estado` SHA-256.
2. **Aplicar a governança desde o primeiro arquivo** — `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, zero `.unwrap()`/`.expect()`, erros via `Result`/`?`/`thiserror`, dependências fixadas com justificativa no commit e `./verify.sh` como gate determinístico, sem relaxar lints.
3. **Preservar norma e histórico** — manter GDD (pilares, regras de ilha, vitória, critérios §16.4) como contrato de comportamento e `nucleo/*.gd` no Git como registro histórico, sem apagamento, reescrita ou force-push (ADR-004:29,42).
4. **Entregar o fluxo SDD ordenado** — converter o assessment em spec preenchida, clarificações, plano, checklist e tasks antes de qualquer implementação, precedendo R4 e R7 na portaria própria.

## Limites

Fora de escopo nesta etapa: escrita de qualquer crate, `Cargo.toml`, spec, plano ou task; execução de `cargo`, `./verify.sh`, clippy, nextest, testes, `make`, Godot, build, formatação, gates ou benchmark; consulta a hosts externos; stage, commit, push, merge, deploy; remoção ou reescrita do histórico GDScript; reabertura da ADR-001 ou troca adicional de engine (ADR-004:53). Nenhuma alegação de compilação, equivalência funcional, paridade de hash, desempenho ou validação ao vivo é feita — todo esse estado permanece **[NV]** até verificação datada na portaria própria. Os 5/5 gates são **citados** de `.specify/memory/STATE.md:8` e o ALL STAGES OK de `specs/021-migracao-bevy/converge.md:13`, não reexecutados. Convenções: pt-BR, `[NV]`, termos estrangeiros explicados na 1ª ocorrência.

## Critérios de sucesso

- Estrutura Rust com o crate de simulação materializado no workspace e dependências fixadas (baseline: 0 crates de simulação; meta: 1, rastreável no `Cargo.toml` do workspace).
- Equivalência semântica verificada por vetores GDScript ↔ Rust para tick/fases, 6 ações, contagem de ilhas, vitória persistida e `hash_estado` (baseline: **[NV]**; meta: paridade documentada com casos de borda).
- Os 8 códigos de recusa e os limites (`1,0 m`, alvo `1–2`, alavanca `1–3`, campos de recurso) preservados com os mesmos literais (baseline: 0 no Rust; meta: 8 de 8).
- `./verify.sh` com ALL STAGES OK no ciclo de implementação (baseline: ALL STAGES OK registrado apenas para `contratos` no ciclo 021; meta: mesmo resultado com o novo crate incluído), sem stage/commit/push nesta etapa.

## Perguntas abertas

- **[NV]** Quais das cinco colaboradoras de `nucleo/mundo.gd:5-9` entram no escopo do ciclo 022 e como serão mapeadas no crate?
- **[NV]** Como garantir paridade de `hash_estado()` entre `str()` do GDScript e a serialização Rust (ordem de iteração e formatação)?
- **[NV]** Qual a API pública do crate e a fronteira entre `dominio/sim` e `crates/` previstas no plano 021?
