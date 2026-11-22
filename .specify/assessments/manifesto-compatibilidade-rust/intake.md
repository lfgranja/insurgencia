# Idea Intake: Porte para Rust do contrato de manifesto de compatibilidade

- **Slug**: manifesto-compatibilidade-rust
- **Created**: 2026-11-21T21:24:08Z
- **Source**: ciclo SDD 033 no branch `033-manifesto-compatibilidade-rust` (base `de3cfb3` = merge do PR #37); âncora = `manifesto.json`, `specs/011-compatibilidade-manifesto/spec.md` FR-001..FR-006 e `nucleo/compatibilidade/manifesto.gd` + `tests/test_manifesto.gd`
- **Type**: improvement (porte de contrato existente / documental + código em fronteira separada)

## Idea (as captured)

> "Portar para Rust o contrato de manifesto de compatibilidade **já existente**, sem criar segundo esquema. A implementação GDScript (`manifesto.gd`) e a spec 011 definem os 6 campos, a ordem formato → versão → módulo → comprimento do hash → arquivo de regras → hash e os códigos literais. A versão Rust deve ser equivalente em campos, códigos e ordem — não um novo `id/versao/regras`."

## Restated

Materializar em `crates/contratos` uma implementação Rust equivalente ao validador GDScript: os mesmos 6 campos de `manifesto.json`, os mesmos 5 códigos de recusa (`MANIFEST_INVALID`, `VERSAO_PROTOCOLO_DIVERGENTE`, `MODULO_AUSENTE`, `REGRAS_AUSENTES`, `HASH_MODULO_DIVERGENTE`) e a mesma ordem de validação, com 7 testes que espelham `tests/test_manifesto.gd`. O hash das regras é **passado pelo chamador** (`Option<&str>`): nenhum I/O de arquivo dentro da validação (Constituição II — adaptadores/portas).

## Origin & Context

- **Raised by**: SE (continuidade do ciclo 011 e do endurecimento do contrato Rust) · **Trigger**: `crates/contratos/src/lib.rs` já expõe `REJEICAO_MANIFESTO`, `REJEICAO_VERSAO`, `REJEICAO_MODULO`, `REJEICAO_HASH`, `MODULO_ID` e `PROTOCOL_VERSION`, mas **não** expõe validador de manifesto nem `REGRAS_AUSENTES`.
- **Estratégia**: GDD §1 (soberania das regras; protocolo implementado no Núcleo e estendido por Módulos) e GDD §14.2 (handshake troca versão do protocolo, identificador do módulo e hash SHA-256; constantes separadas evitam confundir mudança de regra com mudança de rede).

## First-Glance Unknowns

- [RESOLVIDO: esquema — é o de 011/`manifesto.json`, com 6 campos; nenhum esquema novo será inventado.]
- [RESOLVIDO: ordem e códigos — ÂNCORA em 011 FR-003 e no GDScript de referência; `REGRAS_AUSENTES` é o único literal ainda ausente em Rust.]
- [RESOLVIDO: fronteira — Fronteira A = 13 artefatos SDD; Fronteira B = `manifesto.rs`, `lib.rs` e `tests/manifesto.rs`. Nenhum build/gate/commit de nenhuma das partes.]
- [NV: saída final de `./verify.sh` — em execução por contenção de `cargo`; nenhuma saída observada até o registro.]
