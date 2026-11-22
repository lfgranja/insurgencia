# Implementation Plan — Migração do Núcleo para Bevy 0.20.1 + Lightyear 0.31 (workspace Rust)

**Data:** 2026-11-20T23:57Z · **Fonte:** ADR-004 (`docs/adr/004-migracao-bevy-lightyear.md`, Accepted, 20/11/2026) autoriza Rust 1.99.1 (stable), Bevy 0.20.1 e Lightyear 0.31.0 e supersede a ADR-001 na parte de linguagem/engine. `cargo info bevy@0.20.1` (20/11/2026) declara `rust-version: 1.97.1` no crate publicado; a toolchain local `stable 1.99.1` registrada na ADR-004 atende esse MSRV. Contratos de origem: `nucleo/contratos/protocolo.gd` (57 linhas) e `nucleo/contratos/carregador_modulo.gd` (50 linhas). Gate Rust: `verify.sh` (quatro estágios). Layout: Rust-only com `crates/` conforme `projetos/AGENTS.md`. Regras: skill `mdmv-rust-thin`. Este plano descreve a implementação futura; nenhum `cargo`, gate, teste, build ou formatador é executado nesta etapa de escrita.

## Arquivos (exatamente os do plano)

Materializar na implementação: `Cargo.toml` (workspace na raiz), `crates/contratos/{Cargo.toml,src/}`, `crates/dominio/sim/{Cargo.toml,src/}` (ou `crates/dominio-sim/`, conforme o nome de crate escolhido e registrado), `crates/servidor/{Cargo.toml,src/}` e `crates/aplicacao/{Cargo.toml,src/}`; ajustes em `verify.sh` apenas se necessários para refletir o workspace sem alterar a ordem dos quatro estágios. Leitura sem alteração: `docs/adr/004-migracao-bevy-lightyear.md`, `docs/adr/001-excecao-linguagem-gdscript.md`, `nucleo/contratos/protocolo.gd`, `nucleo/contratos/carregador_modulo.gd`, `clippy.toml`, `scripts/ci/{check_sdd_completeness,check_work_order_dag,check_task_id_uniqueness,check_converge_append_only,hub_docs_gate}.py`, `Makefile`, `.specify/memory/STATE.md` e `docs/governance/sdd-phase-contracts.md`. Nenhum outro path é tocado nesta etapa: nenhum gate, baseline, `evidence.csv`, `Makefile`, arquivo `.gd`, artefato R4/R7, build, CI, stage, commit ou push. Nenhum código Rust é escrito nesta etapa de especificação (FR-008, SC-005).

## Steps

1. Materializar `Cargo.toml` na raiz com `[workspace]`, `members = ["crates/*"]`, `resolver = "2"` e `edition = "2024"`, adotando `stable 1.99.1` como toolchain registrada e `1.97.1` como MSRV do crate publicado `bevy@0.20.1`. Declarar o workspace sem `package` virtual que conflite com os crates membros. (FR-001, Q1)
2. Criar os quatro crates em `crates/` — `contratos`, `dominio/sim`, `servidor`, `aplicacao` — cada um com `Cargo.toml` próprio e diretório `src/`, seguindo o layout Rust-only de `projetos/AGENTS.md` (sem `libs/domain`/`libs/infra`). `servidor` usa `bevy` com `default-features = false` + `ScheduleRunnerPlugin` e `lightyear` para rede autoritativa, conforme ADR-004. (FR-002)
3. Fixar dependências com versão explícita — Bevy `0.20.1`, Lightyear `0.31.0`, `thiserror` em libs e `anyhow` em apps — sem `*` ou intervalo aberto, com justificativa no commit e sem substituição automática de pacote similar. Registrar a divergência Lightyear 0.30.0 (UI) vs. 0.31.0 (ADR-004) sem inferir valor intermediário. (FR-002, Q2)
4. Declarar os lints no workspace: `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, avisos fatais via `-D warnings`, `unwrap_used`/`expect_used` em `deny`. Não alterar `Cargo.toml`/`clippy.toml` para desativar lint; única exceção admitida é `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado. (FR-006, SC-004, Q3)
5. Materializar `crates/contratos` com o equivalente Rust de `nucleo/contratos/protocolo.gd:4-26`: `PROTOCOL_VERSION = 1`, `MODULO_ID = "insurgencia.exemplo"`, `TipoPacote` com JOIN=1, JOIN_ACEITO=2, JOIN_RECUSADO=3, COMANDO=4, SNAPSHOT=5, EVENTO_DESAFIO=6 e os quatro conjuntos de campos de `PADROES`. (FR-003, SC-002)
6. Portar `SnapshotMeta` (cinco campos: `protocol_version`, `epoch`, `match_id`, `tick`, `content_hash`) com `to_dict` equivalente e `normalizar_pacote`, que copia somente chaves conhecidas e injeta `tipo`, conforme `protocolo.gd:28-57`. (FR-003)
7. Preservar os quatro códigos de recusa de `protocolo.gd:16-19` com literais exatos — `VERSAO_PROTOCOLO_DIVERGENTE`, `HASH_MODULO_DIVERGENTE`, `MODULO_AUSENTE`, `MANIFEST_INVALID` — como constantes ou variantes de erro tipadas, sem renomeá-los nem fundi-los. (FR-004, SC-002)
8. Portar a validação de `carregador_modulo.gd:26-50`: `content_hash` somente com 64 caracteres hex (SHA-256) e `content_bytes` estritamente entre 1 e 25.000.000; devolver `Result<T, E>` com `thiserror` em libs, sem pânico e sem `.unwrap()`/`.expect()`. Rejeitar explicitamente hash com 63/65 caracteres. (FR-005, SC-003)
9. Confirmar `verify.sh` como gate Rust: quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — preservando `--fast`, `--install-hook`, `--uninstall-hook` e a falha no primeiro estágio vermelho. Nenhuma alteração que desative lints ou pule estágio no modo padrão. (FR-007, SC-001)
10. Verificar por leitura que os cinco gates determinísticos alvo e `Makefile`/baselines/`evidence.csv` permanecem inalterados e que os 31 arquivos `.gd` seguem no histórico do Git; nenhum gate é executado nesta etapa. (FR-008, SC-005, Q4)
11. Na implementação (fora desta etapa), executar os comandos da seção abaixo e colar a saída datada somente em R4; R4 (`validate`) precede R7 (`release`: SemVer, changelog com IDs FR/T e rollback), e nenhum artefato dessas portarias é escrito aqui. (FR-008, SC-001, SC-005)
12. Convergência READ-ONLY: revisar `Cargo.toml`, `crates/`, `verify.sh` e estes artefatos contra ADR-004, os dois contratos GDScript e o skill `mdmv-rust-thin`, corrigindo divergências sempre na fonte (código/documento de origem), nunca no artefato derivado; manter zero alegação de execução observada — o não observado permanece **[NV]**. (SC-001–SC-005)

## Comandos de verificação

```bash
# Gate Rust (única fonte de verdade, quatro estágios na ordem)
./verify.sh
# Equivalente explícito dos estágios (execução pertence à implementação/R4, não a esta etapa):
cargo check --locked --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace          # fallback: cargo test --locked --workspace
cargo fmt --all -- --check

# Gates determinísticos alvo — preservados, não alterados
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```

Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora em R4. Os estágios devem terminar com exit `0` e 0 estágios vermelhos (SC-001); os cinco gates alvo devem permanecer PASS sem modificar gate, baseline ou `evidence.csv` (SC-005). A checagem estrutural dos contratos (4 códigos de recusa, hash 64 hex, MSRV 1.97.1, Lightyear 0.31.0, ausência de `unsafe`/`unwrap`/`expect`) é feita por leitura na fase de verificação, sem ferramenta executada nesta etapa.

## Riscos e mitigação

- MSRV errado (1.100.0 do `main`) → usar `1.97.1` do crate publicado e registrar a divergência; toolchain local 1.99.1 atende (Q1, FR-001).
- Versão flutuante ou pacote similar substituído → fixar Bevy 0.20.1 e Lightyear 0.31.0 explicitamente e justificar no commit; anti-*slopsquatting* (FR-002).
- Layout herdado do Godot (`libs/domain`/`libs/infra`) → usar `crates/` conforme `projetos/AGENTS.md` (FR-002).
- Código de recusa renomeado ou fundido → preservar os quatro literais de `protocolo.gd:16-19` um a um (FR-004, SC-002).
- Hash aceito por correspondência parcial → exigir 64 hex exatos e `content_bytes` em 1–25.000.000 (FR-005, SC-003).
- Pânico em validação → `Result<T, E>` + `?` + `thiserror`; `unwrap_used`/`expect_used` em `deny` (FR-006, SC-004).
- Lint desativado para passar no gate → corrigir o código; `Cargo.toml`/`clippy.toml` intactos (FR-006).
- Regressão dos gates → cinco gates alvo PASS e `Makefile`/baseline/`evidence.csv` inalterados (FR-008, SC-005).
- Apagamento indevido do GDScript → histórico versionado preservado; contratos reexpressos em Rust, não removidos (Q4).
- Árvore suja com trabalho alheio → nenhum stage, commit ou push nesta etapa; somente os quatro documentos de especificação são escritos.
