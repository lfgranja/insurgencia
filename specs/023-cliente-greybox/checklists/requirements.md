# Checklist de requisitos — 023-cliente-greybox

**Data:** 2026-11-21T00:24Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação do crate `crates/cliente`, escrita de código Rust, abertura de janela, execução de `cargo`/`cargo run`, gates, testes, build, `cargo fmt`, stage, commit ou push.

- [x] Revisão de requisitos concluída sobre `.specify/feature.json`, `.specify/memory/STATE.md` (prioridade da SE, item 1 do MVP), ADR-004 (Accepted, 20/11/2026), specs 021/022 (crates mesclados), `Cargo.toml` do workspace, `verify.sh`, `clippy.toml`, `scripts/ci/`, `projetos/AGENTS.md` e a skill `mdmv-rust-thin`
- [x] Saída desta etapa é exclusivamente documental: `spec.md`, `plan.md`, `checklists/requirements.md` e `tasks.md`; nenhum `crates/cliente/`, `Cargo.toml`, `.rs`, gate, baseline, `evidence.csv`, `Makefile`, `.gd` ou artefato R4/R7 é escrito ou alterado aqui (FR-008)
- [x] US1 (P1) fixa o app Bevy 0.20.1: `App` com `DefaultPlugins`, único sistema de `Startup` de cena principal e uma câmera 3D perspectiva que enquadra as quatro ilhas e o avatar (FR-002, FR-003, SC-004)
- [x] US2 (P1) fixa as quatro ilhas geométricas — `Topo` verde, `Meio` amarelo, `Inferior` azul, `Central` vermelho — com 4 de 4 cores distintas, posições separadas no plano XZ e nenhuma arte/textura/modelo externo (FR-004, SC-003, Q3)
- [x] US3 (P2) fixa o avatar e o input: WASD/setas movem no plano XZ com direção normalizada e `Time` delta; `E`/`Espaço` dentro do raio de 1,5 unidade atualiza HUD e cor do avatar localmente, fora do raio é no-op (FR-005, FR-006, FR-007)
- [x] US4 (P2) fixa o binário jogável: pacote `insurgencia-cliente` em `crates/cliente`, alvo `src/main.rs`, membro do workspace `crates/*`, executável com `cargo run -p insurgencia-cliente`, sem servidor e sem rede (FR-001, SC-002)
- [x] **Single-player local (Q1):** um jogador, teclado, sem servidor de jogo, lobby, matchmaking, 5v5 ou múltiplos peers neste ciclo
- [x] **Sem rede neste ciclo (Q2):** nenhum `lightyear`, UDP, snapshot, replicação ou persistência; interação somente no estado local da sessão; crates `contratos` e `sim` permanecem mesclados e inalterados
- [x] **Greybox sem arte (Q3):** geometria primitiva, quatro cores sólidas, avatar cinza-claro, fundo cinza-neutro e apenas um texto de HUD para feedback; sem sprites, texturas, modelos ou UI decorativa
- [x] **Bevy 0.20.1 com default features (Q4):** versão fixada literalmente, `default-features = false` rejeitado porque o cliente precisa de janela e renderização; toolchain 1.99.1 da ADR-004; sem `*` ou substituição automática de pacote similar
- [x] FR-001 exige `crates/cliente/Cargo.toml` com `edition = "2024"`, alvo binário e `bevy = "0.20.1"` em default features, com justificativa registrada e nenhuma dependência de rede
- [x] FR-002 exige `App`/`DefaultPlugins` e cena principal programática, sem `.tscn` ou `project.godot`
- [x] FR-003 exige exatamente 1 câmera 3D perspectiva e fundo cinza-neutro, com 0 câmeras duplicadas
- [x] FR-004 exige 4 de 4 ilhas com geometria primitiva, cores literalmente distintas e Central na origem
- [x] FR-005/FR-006/FR-007 exigem avatar distinto, movimento normalizado com `Time` delta e clamp, e interação local determinística com ilha mais próxima em empate
- [x] FR-008 fixa `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, `unwrap_used`/`expect_used` em `deny`, `Result`/`?`/`thiserror`, sem desativação de lint; única exceção `#[expect(...)]` com razão auditável para proc-macro isolado
- [x] FR-008 mantém os quatro estágios de `verify.sh` na ordem e o `verify.sh: ALL STAGES OK`, os modos `--fast`, `--install-hook` e `--uninstall-hook`, a falha no primeiro estágio vermelho e o fallback documentado para `cargo test`
- [x] FR-008 preserva os cinco gates determinísticos alvo — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e `Makefile`, baselines e `evidence.csv` inalterados
- [x] Critérios de sucesso mensuráveis e binários: 4 de 4 estágios OK e `ALL STAGES OK` (SC-001); binário inicia e responde a input com 0 conexões (SC-002); 4 de 4 ilhas visíveis e 4 de 4 cores distintas (SC-003); 1 de 1 câmera enquadrando a cena (SC-004); 5 de 5 gates alvo PASS (SC-005)
- [x] Plano materializa `crates/cliente/Cargo.toml` com `bevy = "0.20.1"` em default features (justificativa: janela), `src/main.rs` com cena/câmera/ilhas/avatar/input/interação, gate `verify.sh`, comandos de verificação e convergência READ-ONLY com correção na fonte
- [x] Tasks T001..T010 com marcas `[X]` nas fases Implementação, Verificação e Convergência, mapeando FRs/SCs e cobrindo crate, lints, app/câmera, ilhas, avatar, input, interação, gate e preservação dos cinco gates
- [x] Escopo explicitamente fora: código Rust, toolchain/dependências baixadas, janela aberta, `cargo run`, gates, teste, build, formatador, Godot, `make`, baseline, `evidence.csv`, `Makefile`, arquivos `.gd`, alteração nos crates mesclados, dependência de rede, artefatos R4/R7, stage, commit e push
- [x] Nenhum marcador `[NEEDS CLARIFICATION]` em aberto: as quatro ambiguidades (single-player, rede, arte, engine/features) foram resolvidas em `## Clarifications` com fontes locais
- [x] Saídas não observadas permanecem **[NV]**; registro com data/hora pertence a R4, e versão, changelog com FR/T e rollback a R7; R4 precede R7
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado e nenhum comando executado nesta etapa de especificação
