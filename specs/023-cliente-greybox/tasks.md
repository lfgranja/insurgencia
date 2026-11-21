# Tasks — 023-cliente-greybox

**Data:** 2026-11-21T00:24Z

Marcações `[X]` registram a frente de trabalho documental protocolada nesta etapa de escrita; execução de `cargo`, gates, testes, build, `cargo run`, formatadores e portarias R4/R7 permanece **[NV]** e não é afirmada aqui. Nenhum item escreve código Rust, altera os cinco gates determinísticos, introduz dependência de rede (Q2), apaga o GDScript histórico, modifica os crates `contratos`/`sim` ou toca baseline, `evidence.csv` e `Makefile`.

## Phase 1: Implementação

- [X] **T001** Criar `crates/cliente/Cargo.toml` com o pacote `insurgencia-cliente`, `edition = "2024"` e alvo binário em `src/main.rs`, declarando exatamente `bevy = "0.20.1"` em **default features** — `default-features = false` rejeitado porque o cliente precisa de janela e renderização —, versão fixada sem `*` ou intervalo aberto, sem substituição automática de pacote similar e sem `lightyear`/UDP/HTTP (FR-001, Q2, Q4)
- [X] **T002** Declarar os lints do crate — `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, avisos fatais e `unwrap_used`/`expect_used` em `deny` — com erros via `Result<T, E>`, `?` e `thiserror` onde aplicável, sem alterar `Cargo.toml`/`clippy.toml` para desativar lint; única exceção `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado (FR-008)
- [X] **T003** Materializar em `crates/cliente/src/main.rs` o `App` Bevy com `DefaultPlugins`, `.run()` e um único sistema de `Startup` de cena principal; nenhuma cena Godot, `.tscn` ou `project.godot` (FR-002, US1)
- [X] **T004** Configurar no `setup` exatamente uma câmera 3D perspectiva (componente `Camera3d`) acima e inclinada sobre a origem, enquadrando as quatro ilhas e o avatar, com fundo cinza-neutro e 0 câmeras duplicadas (FR-003, SC-004)
- [X] **T005** Criar no `setup` as quatro ilhas `Topo` (verde 0,10/0,80/0,20), `Meio` (amarelo 0,95/0,80/0,10), `Inferior` (azul 0,10/0,45/0,95) e `Central` (vermelho 0,90/0,12/0,12) com geometria primitiva, posições distintas no plano XZ, Central na origem, 4 de 4 cores distintas e nenhuma textura/sprite/modelo externo (FR-004, SC-003, Q3)
- [X] **T006** Criar no `setup` o avatar como entidade geométrica em cinza-claro (0,90/0,90/0,90), visível e fora do centro da Ilha Central, e o texto mínimo de HUD de feedback — único elemento de interface da greybox (FR-005, Q3)
- [X] **T007** Implementar o sistema de `Update` de movimento: ler `ButtonInput<KeyCode>` para WASD/setas, normalizar a direção no plano XZ, aplicar 4,0 unidades/s com `Time` delta, limitar a posição à área da câmera e manter o avatar parado sem foco da janela, sem relógio de parede e sem aleatoriedade (FR-006, US3)
- [X] **T008** Implementar o sistema de `Update` de interação: dentro do raio alvo de 1,5 unidade, `E`/`Espaço` atualiza o HUD com o nome/cor da ilha e troca a cor do avatar; fora do raio é no-op determinístico, em empate vence a ilha mais próxima, e nenhum evento de rede, snapshot, persistência ou autoridade de servidor é produzido (FR-007, Q1, Q2)

## Phase 2: Verificação

- [X] **T009** Verificar por leitura (sem executar comando) que `crates/cliente/Cargo.toml` fixa `bevy = "0.20.1"` em default features e não tem rede (FR-001), que `src/main.rs` tem `App`/`DefaultPlugins`/`Startup` (FR-002), 1 de 1 câmera (SC-004), 4 de 4 ilhas com cores distintas (SC-003), avatar, movimento WASD/setas e interação local (FR-005–FR-007), que não há `unsafe`/`unwrap`/`expect` com lints em `deny` (FR-008) e que `verify.sh` mantém os quatro estágios e o `verify.sh: ALL STAGES OK` (SC-001); confirmar ainda que os cinco gates determinísticos alvo — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — permanecem presentes e inalterados, com `Makefile`, baselines, `evidence.csv`, crates `contratos`/`sim` e arquivos `.gd` intactos (SC-005)

## Phase 3: Convergência READ-ONLY

- [X] **T010** Convergência READ-ONLY: revisar `crates/cliente`, `spec.md`, `plan.md`, este `tasks.md` e o checklist contra STATE.md (prioridade da SE), ADR-004, specs 021/022, `projetos/AGENTS.md` e a skill `mdmv-rust-thin`, corrigindo divergências sempre na fonte (código/documento de origem), nunca no artefato derivado; manter zero comando executado, zero código Rust escrito nesta etapa, zero gate/baseline/`evidence.csv`/`Makefile` alterado, zero dependência de rede, zero modificação dos crates mesclados, zero `.gd` apagado, zero artefato R4/R7 e zero alegação de execução observada — o não observado permanece **[NV]** (FR-008, SC-001–SC-005)
