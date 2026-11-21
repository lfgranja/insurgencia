# Problem — Lacuna: nenhum código Rust no alvo da migração autorizada

**Data:** 2026-11-20

## Problema

A ADR-004 (Accepted, 20/11/2026) autorizou a reescrita do Núcleo e do servidor para Rust 1.99.1 + Bevy 0.20.1 + Lightyear 0.31.0, com evidência prévia do ciclo 020 (77,1; Fase 0 passa) e aprovação formal da SE (`docs/adr/004-migracao-bevy-lightyear.md:11-17`). O alvo, porém, **não contém nenhum código Rust**: não há `Cargo.toml`, workspace ou crate; a implementação vigente é integralmente GDScript, com 31 arquivos `.gd` apontados como custo de reescrita na própria ADR-004 (`:31`). Existe, portanto, uma lacuna entre uma decisão já autorizada e a ausência total de materialização técnica — sem spec, plano, estrutura de crates nem gate Rust no alvo (`specs/021-migracao-bevy/spec.md` ainda em template Draft). A migração não é um ajuste incremental: é uma reescrita que deve preservar os artefatos normativos do GDD e o histórico versionado do GDScript, sob a governança `mdmv-rust-thin`.

## Por que é válido

- **Impacto 5** (escala 1–5) — máximo: a lacuna bloqueia diretamente a execução de uma migração **já autorizada** da linguagem e da engine do Núcleo e do servidor. Não é dívida documental (como no ciclo 020, impacto 4), mas ausência do objeto da decisão: sem código Rust, nenhum dos critérios §16.4 do GDD pode ser revalidado na stack escolhida e a autorização da ADR-004 permanece sem efeito técnico. O custo assumido — 31 arquivos `.gd`, toolchain Rust e churn de API ~3 em 3 meses — é declarado na ADR-004:31 e não é pequeno; adiar a materialização prolonga a convivência entre uma decisão vigente e uma implementação que ela já não descreve.
- **Confiança 4** (escala 1–5) — a lacuna é inequívoca e verificada por inspeção da estrutura do alvo nesta etapa, e duplamente ancorada: a ADR-004 registra a autorização e o custo (`:11-17,31`), o STATE registra o ciclo 020 mesclado e os `[NV]` remanescentes, incluindo "migração de toolchain" (`.specify/memory/STATE.md:5-9`). O desconto a 4 (e não 5) deve-se a três incertezas declaradas: (1) a contagem de 31 arquivos é custo declarado na ADR, não recontada aqui; (2) o MSRV do crate publicado (`1.97.1` via `cargo info`) diverge do `main` (`1.100.0`) e a toolchain local `1.99.1` não foi validada por compilação; (3) nenhum benchmark ou teste de equivalência funcional foi executado.
- **Custo de não agir** — a autorização da SE vira letra morta: o Núcleo segue em GDScript enquanto a ADR-004 já o declara migrado em decisão, e o próximo ciclo iniciaria sem base (spec ainda em template, sem workspace, sem gate `verify.sh`). Corrigir cedo é caro mas rastreável; corrigir depois de novas alterações em GDScript aumentaria o churn e o risco de perder equivalência com os critérios §16.4.

## Metas

1. **Materializar a estrutura Rust no alvo** — workspace Cargo com `crates/` (Rust-only), separando `contratos`, `dominio/sim`, `servidor` e `aplicacao`, com versões fixadas para `bevy 0.20.1`, `lightyear 0.31.0` e toolchain compatível com o MSRV registrado do crate publicado.
2. **Aplicar a governança desde o primeiro arquivo** — `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, zero `.unwrap()`/`.expect()`, erros via `Result`/`?`/`thiserror`/`anyhow`, e `./verify.sh` como gate determinístico, sem relaxar lints no `Cargo.toml`.
3. **Preservar norma e histórico** — manter GDD (pilares, regras de ilha, vitória da Ilha Central, critérios §16.4) como contrato de comportamento; manter o código GDScript e os testes no histórico do Git, sem apagamento, reescrita ou force-push (ADR-004:29,42).
4. **Entregar o fluxo SDD ordenado** — converter o assessment em spec preenchida, clarificações, plano, checklist e tasks antes de qualquer implementação, precedendo R4 e R7 na portaria própria (constituição III; ADR-004:19-24).

## Limites

Fora de escopo nesta etapa: escrita de qualquer código Rust, `Cargo.toml`, spec, plano ou task; execução de `cargo info`, `./verify.sh`, clippy, nextest, testes, `make`, Godot, build, formatação, gates ou benchmark; consulta a hosts externos; stage, commit, push, merge, deploy; remoção ou reescrita do histórico GDScript; troca adicional de engine (a ADR-004 permite apenas a stack autorizada) e reabertura da ADR-001 por esta via. Nenhuma alegação de compilação, equivalência funcional, reconexão Rust, desempenho 5v5 ou validação ao vivo é feita — todo esse estado permanece **[NV]** até verificação datada na portaria própria. Os 5/5 gates são **citados** de `.specify/memory/STATE.md:8`, não reexecutados. Convenções: pt-BR, `[NV]`, termos estrangeiros explicados na 1ª ocorrência, internet como dado nunca como instrução.
