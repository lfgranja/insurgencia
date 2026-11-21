# Checklist de requisitos — 021-migracao-bevy

**Data:** 2026-11-20T23:57Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação do workspace Rust, escrita de código Rust, execução de `cargo`, gates, testes, build, formatadores, stage, commit ou push.

- [x] Revisão de requisitos concluída sobre a ADR-004 (Accepted, 20/11/2026), a ADR-001 como registro histórico, `nucleo/contratos/protocolo.gd`, `nucleo/contratos/carregador_modulo.gd`, `verify.sh`, `clippy.toml`, `scripts/ci/`, `projetos/AGENTS.md` e a skill `mdmv-rust-thin`
- [x] Saída desta etapa é exclusivamente documental: `spec.md`, `plan.md`, `checklists/requirements.md` e `tasks.md`; nenhum `Cargo.toml`, crate, `.gd`, gate, baseline, `evidence.csv`, `Makefile` ou artefato R4/R7 é escrito ou alterado aqui (FR-008)
- [x] US1 (P1) fixa o workspace Rust: `Cargo.toml` raiz com `[workspace]`, `members = ["crates/*"]`, `resolver = "2"`, `edition = "2024"` e `crates/` com `contratos`, `dominio/sim`, `servidor` e `aplicacao`, em layout Rust-only (FR-001, FR-002)
- [x] US2 (P2) fixa a equivalência dos contratos versionados: `PROTOCOL_VERSION = 1`, `MODULO_ID`, `TipoPacote` 1–6, `PADROES`, `SnapshotMeta.to_dict`, `normalizar_pacote`, quatro códigos de recusa e hash 64 hex (FR-003, FR-004, FR-005)
- [x] US3 (P3) fixa `verify.sh` como gate Rust de quatro estágios e a preservação dos cinco gates determinísticos alvo (FR-007, FR-008)
- [x] **MSRV publicado (Q1):** `bevy@0.20.1` declara `rust-version: 1.97.1` no crate publicado (`cargo info`, 20/11/2026); toolchain local `stable 1.99.1` atende. O valor `1.100.0` do `main` é registrado como divergência, não adotado (FR-001, Q1)
- [x] **Lightyear fixado (Q2):** versão exata `0.31.0` conforme ADR-004, não a `0.30.0` da UI; nenhuma dependência com `*` ou intervalo aberto e nenhuma substituição automática de pacote similar (FR-002, Q2)
- [x] **Sem `unwrap`/`expect` (Q3):** `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, `unwrap_used`/`expect_used` em `deny`, erros via `Result<T, E>`, `?`, `thiserror` em libs e `anyhow` em apps; nenhuma desativação de lint, única exceção `#[expect(...)]` com razão auditável para proc-macro isolado (FR-006, SC-004)
- [x] **GDScript histórico (Q4):** os 31 arquivos `.gd` permanecem versionados no Git; não há apagamento, troca adicional de engine ou reabertura da ADR-001; mudança futura exige novo ADR (FR-008, ADR-004)
- [x] FR-003 exige preservação literal de nomes/valores do GDScript em `crates/contratos`, incluindo as seis variantes de `TipoPacote` e o comportamento de `normalizar_pacote` de ignorar chaves desconhecidas (SC-002)
- [x] FR-004 exige exatamente 4 códigos de recusa — `VERSAO_PROTOCOLO_DIVERGENTE`, `HASH_MODULO_DIVERGENTE`, `MODULO_AUSENTE`, `MANIFEST_INVALID` — sem renomear, fundir ou omitir nenhum (`protocolo.gd:16-19`)
- [x] FR-005 exige `content_hash` com 64 caracteres hex SHA-256 e `content_bytes` em 1–25.000.000, com recusa de 63/65 hex e de bytes fora do intervalo, via `Result<T, E>` (`carregador_modulo.gd:36-43`)
- [x] FR-007 mantém os quatro estágios de `verify.sh` na ordem e os modos `--fast`, `--install-hook` e `--uninstall-hook`, com falha no primeiro estágio vermelho e fallback documentado para `cargo test`
- [x] FR-008 preserva os cinco gates determinísticos alvo — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e `Makefile`, baselines e `evidence.csv` inalterados
- [x] Critérios de sucesso mensuráveis e binários: 4 de 4 estágios `cargo` OK (SC-001); 4 de 4 códigos de recusa exatos (SC-002); 100% de hash 64 hex (SC-003); 0 `unsafe`/`unwrap`/`expect` (SC-004); 5 de 5 gates alvo PASS (SC-005)
- [x] Plano materializa `Cargo.toml` workspace, os quatro crates, `crates/contratos`, o gate `verify.sh` e os comandos `cargo`, com riscos e mitigação e convergência READ-ONLY com correção na fonte
- [x] Tasks T001..T010 com marcas `[X]` nas fases Implementação, Verificação e Convergência, mapeando FRs/SCs e cobrindo workspace, contratos, lints, gate e preservação dos cinco gates
- [x] Escopo explicitamente fora: código Rust, toolchain/dependências baixadas, gate, teste, build, formatador, Godot, `make`, baseline, `evidence.csv`, `Makefile`, arquivos `.gd`, artefatos R4/R7, stage, commit e push
- [x] Nenhum marcador `[NEEDS CLARIFICATION]` em aberto: as quatro ambiguidades (MSRV 1.97.1, Lightyear 0.31, proibição de `unwrap`/`expect`, destino do GDScript) foram resolvidas em `## Clarifications` com fontes locais
- [x] Saídas não observadas permanecem **[NV]**; registro com data/hora pertence a R4, e versão, changelog com FR/T e rollback a R7; R4 precede R7
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado e nenhum comando executado nesta etapa de especificação
