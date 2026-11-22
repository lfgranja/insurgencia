# Implementation Plan — Simulação Rust do Núcleo (`crates/sim`)

**Data:** 2026-11-20T00:08Z · **Fonte:** ADR-004 (Accepted, 20/11/2026) autoriza a reescrita do Núcleo em Rust; `crates/contratos` (`insurgencia-contratos`) já está mesclado pela spec 021. Esta feature cria o crate de simulação pura `crates/sim`, reexpressando `nucleo/mundo.gd` e `nucleo/sim/validacao.gd`. Gate Rust: `verify.sh` (`verify.sh: ALL STAGES OK`). Layout: Rust-only com `crates/` conforme `projetos/AGENTS.md`. Regras: skill `mdmv-rust-thin` (`unsafe` proibido, `clippy::pedantic = deny`, sem `unwrap`/`expect`, `Result` + `thiserror`). Este plano descreve a implementação futura; nenhum `cargo`, gate, teste, build ou formatador é executado nesta etapa de escrita.

## Arquivos (exatamente os do plano)

Materializar na implementação: `crates/sim/Cargo.toml` com dependência fixada de `insurgencia-contratos` e `crates/sim/src/` com os módulos do `Mundo`, da validação e do hash. Leitura sem alteração: `specs/022-simulacao-rust/spec.md`, `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `specs/009-servidor-validacao/spec.md:FR-001,FR-005,FR-007`, `specs/021-migracao-bevy/spec.md`, `crates/contratos/`, `verify.sh`, `clippy.toml`, `scripts/ci/{check_sdd_completeness,check_work_order_dag,check_task_id_uniqueness,check_converge_append_only,hub_docs_gate}.py`, `Makefile`, `.specify/feature.json`, `.specify/memory/STATE.md`, skill `mdmv-rust-thin`. Nenhum desses arquivos de leitura é modificado nesta etapa.

## Steps

1. Criar `crates/sim/Cargo.toml` com o pacote `insurgencia-sim`, `edition = "2024"` e a dependência explícita e fixada `insurgencia-contratos = { path = "../contratos" }` (ou a versão fixada registrada), SEM `bevy`, `lightyear` ou qualquer runtime de engine. Toda dependência é fixada (sem `*` ou intervalo aberto), com justificativa no commit e sem substituição automática de pacote similar. (FR-001, Q1)
2. Declarar os lints do crate: `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, avisos fatais via `-D warnings` e `unwrap_used`/`expect_used` em `deny`. Não alterar `Cargo.toml`/`clippy.toml` para desativar lint; única exceção admitida é `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado. (FR-007, Q3)
3. Definir os `Tipos Irish` do domínio em `crates/sim/src/`: newtypes `Tick`, `Epoch`, `MatchId`, `PeerId`, e enums `Fase` (`Prepare`/`Skills`/`Finish`), `CodigoValidacao` (5 variantes) e `Acao` (as seis de `ACOES_PERMITIDAS`). Valores literais do GDScript preservados; nenhum tipo de engine e nenhum primitivo solto na interface pública. (FR-004, FR-007, Q4)
4. Materializar `Mundo` com `tick`, `epoch`, `match_id`, `jogadores`, `comandos_pendentes` e `comandos_do_tick`, `PASSO_SEGUNDOS = 1.0 / 30.0` preservado como literais de referência (`1`/`30`), e as operações `adicionar_jogador`, `remover_jogador` e `agendar_comando`. `epoch` e `match_id` iniciam em `1`; `tick` em `0`. Tempo do servidor é inteiro. (FR-002, Q2)
5. Implementar `avancar_tick` com as três fases na ordem exata `prepare → skills → finish`: `prepare` copia `comandos_pendentes` para `comandos_do_tick` e esvazia os pendentes; `skills` aplica as ações permitidas ao estado dos jogadores; `finish` aplica os efeitos de fim de tick e limpa `comandos_do_tick`; `tick` é incrementado ao final. (FR-003, SC-003)
6. Implementar `ValidacaoComandos` com `LIMITE_MOVIMENTO_M = 1.0` e exatamente cinco códigos — `PEER_NAO_AUTORIZADO`, `SEQ_OU_REQUEST_REPLAY`, `ACAO_DESCONHECIDA`, `MOVIMENTO_FORA_DO_LIMITE`, `OK`. A ordem de verificação é peer → replay (`seq`/`request_id` regressivos) → ação fora de `ACOES_PERMITIDAS` → deslocamento acima de `1.0`; `OK` somente se nenhuma recusa se aplicar. (FR-004, FR-005, SC-002)
7. Implementar `hash_estado` com SHA-256 determinístico de 64 hex sobre a serialização em ordem fixa de `jogadores`, `tick` e componentes da simulação, sem relógio de parede, sem aleatoriedade e sem `f64` como relógio; estados iguais devem produzir hash igual e qualquer mudança de estado deve alterá-lo. (FR-006, SC-004, Q2)
8. Tratar toda falha com `Result<T, E>`, `?` e `thiserror` em libs, sem pânico e sem `.unwrap()`/`.expect()`; nenhuma recusa é representada por exceção ou dictionary mutável. (FR-007, Q3)
9. Manter `verify.sh` como gate Rust: quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — preservando `--fast`, `--install-hook`, `--uninstall-hook`, a falha no primeiro estágio vermelho e o `verify.sh: ALL STAGES OK`. Nenhuma alteração que desative lints ou pule estágio no modo padrão. (FR-008, SC-001)
10. Verificar por leitura que os cinco gates determinísticos alvo — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e `Makefile`/baselines/`evidence.csv` permanecem inalterados, e que nenhum comando foi executado nesta etapa. (FR-008, SC-005)
11. Na implementação (fora desta etapa), executar os comandos da seção abaixo e colar a saída datada somente em R4; R4 (`validate`) precede R7 (`release`: SemVer, changelog com IDs FR/T e rollback), e nenhum artefato dessas portarias é escrito aqui. (FR-008, SC-001, SC-005)
12. Convergência READ-ONLY: revisar `crates/sim`, `spec.md` e este plano contra ADR-004, `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, spec 009 FR-005 e skill `mdmv-rust-thin`, corrigindo divergências sempre na fonte (código/documento de origem), nunca no artefato derivado; manter zero alegação de execução observada — o não observado permanece **[NV]**. (SC-001–SC-005)

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

Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora em R4. Os estágios devem terminar com `verify.sh: ALL STAGES OK` e 0 estágios vermelhos (SC-001); os cinco gates alvo devem permanecer PASS sem modificar gate, baseline ou `evidence.csv` (SC-005). A checagem estrutural (5 códigos exatos, `LIMITE_MOVIMENTO_M = 1.0`, 3 fases na ordem, `PASSO_SEGUNDOS = 1.0 / 30.0`, dependência apenas de `insurgencia-contratos`, ausência de `unsafe`/`unwrap`/`expect`) é feita por leitura na fase de verificação, sem ferramenta executada nesta etapa.
