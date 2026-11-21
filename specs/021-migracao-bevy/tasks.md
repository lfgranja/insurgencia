# Tasks — 021-migracao-bevy

**Data:** 2026-11-20T23:57Z

Marcações `[X]` registram a frente de trabalho documental protocolada nesta etapa de escrita; execução de `cargo`, gates, testes, build, formatadores e portarias R4/R7 permanece **[NV]** e não é afirmada aqui. Nenhum item escreve código Rust, altera os cinco gates determinísticos, troca o motor além da ADR-004, apaga o GDScript histórico ou modifica baseline, `evidence.csv` e `Makefile`.

## Phase 1: Implementação

- [X] **T001** Materializar `Cargo.toml` na raiz com `[workspace]`, `members = ["crates/*"]`, `resolver = "2"` e `edition = "2024"`, adotando `stable 1.99.1` e o MSRV publicado `1.97.1` do `bevy@0.20.1` (FR-001, Q1)
- [X] **T002** Criar os quatro crates em `crates/` — `contratos`, `dominio/sim`, `servidor`, `aplicacao` — com `Cargo.toml` e `src/` próprios, em layout Rust-only (`projetos/AGENTS.md`), sem `libs/domain`/`libs/infra`; `servidor` usa `bevy` com `default-features = false` + `ScheduleRunnerPlugin` e `lightyear` para rede autoritativa (FR-002)
- [X] **T003** Fixar dependências com versão explícita e justificativa no commit — Bevy `0.20.1`, Lightyear `0.31.0`, `thiserror` em libs, `anyhow` em apps — sem `*` ou intervalo aberto e sem substituição automática de pacote similar; registrar a divergência Lightyear 0.30.0 (UI) vs. 0.31.0 (ADR-004) (FR-002, Q2)
- [X] **T004** Declarar lints no workspace — `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, avisos fatais, `unwrap_used`/`expect_used` em `deny` — sem alterar `Cargo.toml`/`clippy.toml` para desativar lint; única exceção `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado (FR-006, SC-004, Q3)
- [X] **T005** Portar `nucleo/contratos/protocolo.gd:4-26` para `crates/contratos`: `PROTOCOL_VERSION = 1`, `MODULO_ID = "insurgencia.exemplo"`, `TipoPacote` (JOIN=1 a EVENTO_DESAFIO=6) e os quatro conjuntos de campos de `PADROES`, com nomes e valores literais preservados (FR-003, SC-002)
- [X] **T006** Portar `SnapshotMeta` (cinco campos) com `to_dict` e `normalizar_pacote` de `protocolo.gd:28-57`, preservando somente chaves conhecidas e injetando `tipo`; preservar os quatro códigos de recusa de `protocolo.gd:16-19` — `VERSAO_PROTOCOLO_DIVERGENTE`, `HASH_MODULO_DIVERGENTE`, `MODULO_AUSENTE`, `MANIFEST_INVALID` — como constantes ou variantes tipadas (FR-003, FR-004, SC-002)
- [X] **T007** Portar a validação de `carregador_modulo.gd:26-50`: `content_hash` com 64 caracteres hex SHA-256 e `content_bytes` em 1–25.000.000, recusando 63/65 hex e bytes fora do intervalo, com `Result<T, E>` e `thiserror` em libs, sem pânico (FR-005, SC-003)
- [X] **T008** Confirmar `verify.sh` como gate Rust com quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — preservando `--fast`, `--install-hook`, `--uninstall-hook` e a falha no primeiro estágio vermelho; documentar os comandos `cargo` no plano (FR-007, SC-001)

## Phase 2: Verificação

- [X] **T009** Verificar por leitura (sem executar comando) que `Cargo.toml` e `crates/` materializam os quatro módulos, que `crates/contratos` expõe 4 de 4 códigos de recusa literais (SC-002), que a validação exige 64 hex e 1–25.000.000 bytes (SC-003), que não há `unsafe`, `.unwrap()` ou `.expect()` no código de produção com lints em `deny` (SC-004) e que `verify.sh` mantém os quatro estágios; verificar ainda que os cinco gates determinísticos alvo — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — permanecem presentes e inalterados, com `Makefile`, baselines e `evidence.csv` intactos e os 31 arquivos `.gd` no histórico do Git. Os comandos do plano são metas a aferir na implementação/R4 (4 de 4 estágios OK, 5 de 5 gates PASS), nunca resultados desta etapa; toda saída não observada permanece **[NV]** (FR-001–FR-008, SC-001–SC-005)

## Phase 3: Convergência READ-ONLY

- [X] **T010** Convergência READ-ONLY: revisar `Cargo.toml`, `crates/`, `verify.sh` e estes artefatos contra ADR-004, `protocolo.gd`, `carregador_modulo.gd`, `projetos/AGENTS.md` e a skill `mdmv-rust-thin`, corrigindo divergências sempre na fonte (código/documento de origem), nunca no artefato derivado; manter zero comando executado, zero código Rust escrito nesta etapa, zero gate/baseline/`evidence.csv`/`Makefile` alterado, zero `.gd` apagado, zero artefato R4/R7 e zero alegação de execução observada (FR-008, SC-001–SC-005)
