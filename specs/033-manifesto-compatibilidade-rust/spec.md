# Feature Specification: Porte para Rust do manifesto de compatibilidade

**Branch**: `033-manifesto-compatibilidade-rust` (base `de3cfb3` = merge do PR #37) · **Created**: 2026-11-21 · **Status**: Draft · **R1**: go

**Âncora obrigatória**: `manifesto.json`, `specs/011-compatibilidade-manifesto/spec.md` FR-001..FR-006, `nucleo/compatibilidade/manifesto.gd`, `tests/test_manifesto.gd`. Este ciclo **porta** o contrato existente; não cria segundo esquema (`id/versao/regras`).

## User Scenarios
### US1 — Núcleo Rust aplica o mesmo contrato do GDScript (P1)
O validador em `crates/contratos` aceita o `manifesto.json` original e recusa exatamente as mesmas condições que a implementação GDScript, com os mesmos 6 campos, os mesmos 5 códigos literais e a mesma ordem de checagem.

**Acceptance Scenarios**: 1) `validar_formato` com os 6 campos presentes e tipados devolve `ManifestoCompatibilidade`; 2) campo ausente ou tipo errado (ex.: `checked_at` removido, `protocol_version` como string) devolve `ErroCompatibilidade::ManifestoInvalido` → `"MANIFEST_INVALID"`, sem checar tamanho do hash nessa etapa; 3) `conferir` com `hash_regras=None` devolve `"REGRAS_AUSENTES"`; 4) a ordem observada é formato → versão → módulo → comprimento/hex do hash → arquivo de regras → hash.

### US2 — Porta sem I/O e sem pânico (P1)
A validação é pura em relação ao sistema de arquivos: quem chama fornece o hash das regras (`Option<&str>`), conforme Constituição II (adaptadores/portas). Nenhum `.unwrap()`/`.expect()`, nenhum `unsafe`, nenhuma dependência nova.

**Acceptance Scenarios**: 1) `conferir` não abre arquivos nem calcula SHA-256 internamente; 2) JSON inválido devolve `ManifestoInvalido` em vez de pânico; 3) hash de 63/65 caracteres é recusado na etapa de comprimento, antes da comparação com as regras; 4) `eq_ignore_ascii_case` compara `content_hash` e `hash_regras` sem alterar o literal do erro.

### US3 — SDD ordenado, rastreável e com rollback documentado (P2)
O ciclo segue R1 `go` → R4 → R7, com `converge.md` append-only (primeira entrada `2026-11-21`), `analyze.md` READ-ONLY, checklist `[x]` = revisão de requisitos e release `1.0.0` com changelog FR/T e rollback documentado — nunca executado.

**Acceptance Scenarios**: 1) 13 artefatos escritos na Fronteira A e 3 arquivos na Fronteira B, sem sobrescrita; 2) factory `b1deab1` apenas como fonte; 3) 5 gates PASS reportados em `2026-11-21T21:24:08Z`; saída de `./verify.sh` **[NV]** por contenção de `cargo`; 4) nenhum build/gate/commit/PR, nenhum toque em `docs/GDD*`, baselines, CI ou 194 paths untracked alheios.

## Requirements
- **FR-001**: manter como âncora os 6 campos e valores de `manifesto.json` — `manifest_version="1.0.0"`, `protocol_version=1`, `module_id="insurgencia.exemplo"`, `content_hash="b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746"`, `checked_at="2026-11-19T00:00:00Z"`, `tool_version="4.7.2.stable.official"` — com nomes idênticos ao JSON. Nenhum esquema novo; a especificação de origem é `specs/011` FR-001..FR-006 (GDD §1/§14.2).
- **FR-002**: criar `crates/contratos/src/manifesto.rs` expondo `pub const REGRAS_AUSENTES: &str = "REGRAS_AUSENTES";`, `pub struct ManifestoCompatibilidade` com os 6 campos públicos (`protocol_version: u8`) e `#[derive(...thiserror::Error)] pub enum ErroCompatibilidade` com as variantes `ManifestoInvalido`, `VersaoProtocoloDivergente`, `ModuloAusente`, `RegrasAusentes`, `HashModuloDivergente` e `pub fn codigo(&self) -> &'static str` devolvendo `MANIFEST_INVALID`, `VERSAO_PROTOCOLO_DIVERGENTE`, `MODULO_AUSENTE`, `REGRAS_AUSENTES`, `HASH_MODULO_DIVERGENTE`. Em `lib.rs`, adicionar **apenas** `pub mod manifesto;` e ajustar o doc do módulo se necessário; constantes e funções existentes intactas.
- **FR-003**: `pub fn validar_formato(json: &str) -> Result<ManifestoCompatibilidade, ErroCompatibilidade>` exige os 6 campos com os tipos corretos; ausência ou tipo errado → `ManifestoInvalido`. Esta etapa **não** checa o tamanho do hash (011 FR-003 mantém essa checagem em `conferir`).
- **FR-004**: `pub fn conferir(json: &str, modulo_id: &str, versao_local: u8, hash_regras: Option<&str>) -> Result<ManifestoCompatibilidade, ErroCompatibilidade>` aplica, nesta ordem: `validar_formato` → `protocol_version != versao_local` → `module_id != modulo_id` → `content_hash` com 64 caracteres hex → `hash_regras == None` → `hash_regras` diferente do `content_hash` (`eq_ignore_ascii_case`). Sem pânico e sem I/O; o hash das regras é passado pelo chamador.
- **FR-005**: higiene Rust em todo `.rs`: `#![forbid(unsafe_code)]` e `#![deny(clippy::pedantic)]`, sem `.unwrap()`/`.expect()` em produção, `Result<T,E>` + `thiserror`, `BTreeMap` para saídas ordenadas, nomes de campos idênticos ao JSON e nenhuma dependência nova (`serde_json` e `thiserror` já existem em `crates/contratos`). `crates/contratos/Cargo.toml` não é alterado.
- **FR-006**: escrever `crates/contratos/tests/manifesto.rs` com 7 testes espelhando `tests/test_manifesto.gd` — original aceito, JSON ausente/inválido recusado, `checked_at` removido recusado, divergência de versão, divergência de hash, `module_id` divergente e `hash_regras=None` — usando `serde_json::json!` e os valores de `manifesto.json`. Preservar SDD ordenado R1 → R4 → R7, `converge.md` append-only, `analyze.md` READ-ONLY, fronteiras disjuntas, rollback documentado e nenhum build/gate/commit.

## Success Criteria
SC-001: os 6 campos de `manifesto.json` e os 5 códigos literais são idênticos à âncora 011/GDScript, com nenhum campo renomeado e nenhum esquema paralelo. SC-002: `conferir` produz o código correto para cada uma das 7 verificações de referência, na ordem formato → versão → módulo → 64 hex → regras → hash. SC-003: nenhuma chamada de I/O, SHA-256, `.unwrap()`, `.expect()` ou `unsafe` aparece na validação; `hash_regras` é `Option<&str>`. SC-004: `lib.rs` ganha somente `pub mod manifesto;` (e eventual ajuste de doc), com constantes/funções existentes e `Cargo.toml` intactos. SC-005: SDD ordenado R1 go → R4 aprovado → R7 `1.0.0`, `converge.md` append-only (primeira entrada 2026-11-21), `analyze.md` READ-ONLY, checklist = revisão, 13 artefatos na Fronteira A e 3 arquivos na Fronteira B. SC-006: release `1.0.0` com changelog FR/T e rollback documentado (remover `manifesto.rs`, `tests/manifesto.rs`, a linha `pub mod manifesto;` e os 13 artefatos — nunca executar); nenhum build/gate/commit/PR; `[NV]` mantido para `./verify.sh`; factory `b1deab1`, `docs/GDD*`, baselines, gates, CI e 194 paths untracked alheios intactos.

## Out of scope
Segundo esquema de manifesto, I/O ou cálculo de hash dentro da validação, dependências novas, alteração de `Cargo.toml`, factory `b1deab1` (somente fonte), `docs/GDD*`, baselines/gates/CI, 194 paths untracked alheios, janelas do cliente, credenciais, deploy/remote, commits/builds/gates/PRs e reexecução de `./verify.sh`. O que não foi observado permanece **[NV]**.
