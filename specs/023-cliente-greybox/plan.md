# Implementation Plan — Cliente Greybox Jogável (`crates/cliente`)

**Data:** 2026-11-21T00:24Z · **Fonte:** `.specify/feature.json` → `specs/023-cliente-greybox`; `.specify/memory/STATE.md` (estado vivo 2026-11-21T00:22Z) reorienta a SE para o MVP jogável single-player, cujo item 1 é o cliente greybox (cena principal, câmera, ilhas geométricas). ADR-004 (Accepted, 20/11/2026) fixa Bevy 0.20.1 e toolchain 1.99.1; os crates `insurgencia-contratos` (`crates/contratos`) e `insurgencia-sim` (`crates/sim`) já estão mesclados (specs 021/022). Gate Rust: `verify.sh` (`verify.sh: ALL STAGES OK`). Layout: Rust-only com `crates/` conforme `projetos/AGENTS.md`; o workspace raiz já declara `members = ["crates/*"]` e `resolver = "2"`. Regras: skill `mdmv-rust-thin` (`unsafe` proibido, `clippy::pedantic = deny`, sem `unwrap`/`expect`, `Result` + `thiserror`). Este plano descreve a implementação futura; nenhum `cargo`, gate, teste, build ou formatador é executado nesta etapa de escrita, e nenhum código Rust é escrito aqui.

## Arquivos (exatamente os do plano)

Materializar na implementação: `crates/cliente/Cargo.toml` (pacote `insurgencia-cliente`, `edition = "2024"`, `bevy = "0.20.1"` em **default features** — justificativa: o cliente precisa de janela e renderização) e `crates/cliente/src/main.rs` com o `App`, a cena principal, a câmera, as quatro ilhas, o avatar, o input e a interação local. Leitura sem alteração: `specs/023-cliente-greybox/spec.md`, `.specify/feature.json`, `.specify/memory/STATE.md`, `docs/adr/004-migracao-bevy-lightyear.md`, `specs/021-migracao-bevy/spec.md`, `specs/022-simulacao-rust/spec.md`, `Cargo.toml` (workspace), `verify.sh`, `clippy.toml`, `scripts/ci/{check_sdd_completeness,check_work_order_dag,check_task_id_uniqueness,check_converge_append_only,hub_docs_gate}.py`, `Makefile`, `docs/governance/`, `docs/progress/evidence.csv`, skill `mdmv-rust-thin`. Nenhum desses arquivos de leitura é modificado nesta etapa; nenhum arquivo `.gd` é apagado.

## Steps

1. Criar `crates/cliente/Cargo.toml` com o pacote `insurgencia-cliente`, `edition = "2024"` e alvo binário padrão (`src/main.rs`). Declarar **exatamente** `bevy = "0.20.1"` com **default features** — `default-features = false` é rejeitado porque o cliente precisa de janela e renderização (Q4). Versão fixada, sem `*` ou intervalo aberto, sem substituição automática de pacote similar e sem justificativa implícita: a razão fica registrada no commit. Não adicionar `lightyear`, UDP, HTTP, lobby ou qualquer dependência de rede (Q2). (FR-001)
2. Declarar os lints do crate: `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, avisos fatais via `-D warnings` e `unwrap_used`/`expect_used` em `deny`. Erros são `Result<T, E>`, `?` e `thiserror` onde aplicável; não alterar `Cargo.toml`/`clippy.toml` para desativar lint — única exceção admitida é `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado. (FR-008)
3. Materializar o ponto de entrada em `crates/cliente/src/main.rs`: construir o `App`, registrar `DefaultPlugins` e encerrar com `.run()`. Registrar um único sistema de `Startup` (`setup`) que monta a cena principal; nenhuma cena Godot, `.tscn` ou `project.godot` é introduzida. (FR-002, US1)
4. No `setup`, criar exatamente uma câmera 3D perspectiva (componente `Camera3d`) posicionada acima e inclinada sobre a origem, enquadrando simultaneamente as quatro ilhas e o avatar; definir fundo cinza-neutro para preservar o greybox. Nenhuma segunda câmera ativa. (FR-003, SC-004)
5. No `setup`, criar as quatro ilhas com geometria primitiva (caixa ou cilindro), posições distintas no plano XZ e cores sólidas literalmente distintas: `Topo` verde (0,10/0,80/0,20), `Meio` amarelo (0,95/0,80/0,10), `Inferior` azul (0,10/0,45/0,95) e `Central` vermelho (0,90/0,12/0,12), Central na origem e as três de rota ao redor. Sem textura, sprite, modelo externo ou arte (Q3). (FR-004, SC-003)
6. No `setup`, criar o avatar como entidade geométrica primitiva em cinza-claro (0,90/0,90/0,90), a uma posição visível e distinta do centro da Ilha Central, e criar o texto de HUD usado como feedback de interação — o único elemento de interface da greybox. (FR-005, Q3)
7. Implementar o sistema de movimento em `Update`: ler `ButtonInput<KeyCode>` para WASD e setas, montar a direção no plano XZ, normalizar o vetor (inclusive na diagonal), aplicar velocidade alvo de 4,0 unidades/s com `Time` delta e limitar (clamp) a posição à área enquadrada pela câmera. Sem relógio de parede e sem aleatoriedade. Sem foco da janela, nenhuma tecla é registrada e nada é movido. (FR-006, US3)
8. Implementar o sistema de interação em `Update`: calcular a distância do avatar a cada ilha; dentro do raio alvo de 1,5 unidade, `E` ou `Espaço` atualiza o texto do HUD com o nome/cor da ilha e troca a cor do avatar para a cor da ilha. Fora do raio, a operação é no-op determinístico; em empate, a ilha mais próxima vence. Nenhum evento de rede, snapshot, persistência ou autoridade de servidor — estado somente na sessão local (Q1, Q2). (FR-007)
9. Manter `verify.sh` como gate Rust único: quatro estágios na ordem — `cargo check --locked --workspace --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) e `cargo fmt --all -- --check` — preservando `--fast`, `--install-hook`, `--uninstall-hook`, a falha no primeiro estágio vermelho e o `verify.sh: ALL STAGES OK`. Nenhuma alteração que desative lint ou pule estágio no modo padrão. (FR-008, SC-001)
10. Verificar por leitura que os cinco gates determinísticos alvo — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e `Makefile`/baselines/`evidence.csv` permanecem inalterados, que os crates `contratos` e `sim` não foram modificados e que nenhum comando foi executado nesta etapa. (FR-008, SC-005)
11. Na implementação (fora desta etapa), executar os comandos da seção abaixo e colar a saída datada somente em R4; R4 (`validate`) precede R7 (`release`: SemVer, changelog com IDs FR/T e rollback), e nenhum artefato dessas portarias é escrito aqui. A janela e o input só são afirmados como observados após registro em R4; antes disso, permanecem **[NV]**. (SC-001, SC-002)
12. Convergência READ-ONLY: revisar `crates/cliente`, `spec.md`, este plano, `tasks.md` e o checklist contra STATE.md (prioridade da SE), ADR-004, specs 021/022, `projetos/AGENTS.md` e skill `mdmv-rust-thin`, corrigindo divergências sempre na fonte (código/documento de origem), nunca no artefato derivado; manter zero alegação de execução observada — o não observado permanece **[NV]**. (SC-001–SC-005)

## Comandos de verificação

```bash
# Gate Rust (única fonte de verdade, quatro estágios na ordem)
./verify.sh
# Equivalente explícito dos estágios (execução pertence à implementação/R4, não a esta etapa):
cargo check --locked --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace          # fallback: cargo test --locked --workspace
cargo fmt --all -- --check

# Alvo binário (execução pertence à implementação/R4, não a esta etapa):
cargo run -p insurgencia-cliente

# Gates determinísticos alvo — preservados, não alterados
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```

Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora em R4. Os estágios devem terminar com `verify.sh: ALL STAGES OK` e 0 estágios vermelhos (SC-001); o binário deve abrir a janela e responder a WASD/setas e `E`/`Espaço` (SC-002); os cinco gates alvo devem permanecer PASS sem modificar gate, baseline ou `evidence.csv` (SC-005). A checagem estrutural (crate `crates/cliente`, `bevy = "0.20.1"` em default features, 1 câmera, 4 ilhas com cores distintas, avatar, input/interação locais, ausência de `unsafe`/`unwrap`/`expect` e de dependência de rede) é feita por leitura na fase de verificação, sem ferramenta executada nesta etapa.
