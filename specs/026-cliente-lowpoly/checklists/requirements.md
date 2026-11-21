# Checklist de requisitos — 026-cliente-lowpoly

**Data:** 2026-11-21T02:04Z

Revisão de qualidade de requisitos — `[x]` **não** significa migração executada, escrita de código Rust, abertura de janela, execução de `cargo`/`cargo run`, gates, testes, build, `cargo fmt`, stage, commit ou push.

- [x] Revisão de requisitos concluída sobre `.specify/feature.json`, `.specify/memory/STATE.md` (redefinição da SE: item 1 = cliente low-poly jogável; itens 2–4 entregues; 5–6 DEFER; regra da janela), ADR-004 (Accepted, 20/11/2026), `docs/GDD - Insurgência.md` §13.2 (~1045) e §14 (~1127), spec 023, `crates/cliente/` atual (2D: `Camera2d` + `Sprite`, `lib.rs` puro, dependência de `insurgencia-sim`), `Cargo.toml` do workspace, `verify.sh`, `clippy.toml`, `scripts/ci/`, `projetos/AGENTS.md` e skill `mdmv-rust-thin`
- [x] Saída desta etapa é exclusivamente documental: `spec.md`, `plan.md`, `checklists/requirements.md` e `tasks.md`; nenhum `.rs`, `Cargo.toml`, gate, baseline, `evidence.csv`, `Makefile`, `.gd` ou artefato R4/R7 é escrito ou alterado aqui (FR-008)
- [x] US1 (P1) fixa a cena principal 3D: migração in-place de `Camera2d`/`Sprite` para exatamente uma `Camera3d` perspectiva, com iluminação de ambiente e direcional e único sistema de `Startup` (FR-002, FR-003, SC-002)
- [x] US2 (P1) fixa as quatro ilhas low-poly — `Topo` verde, `Meio` amarelo, `Inferior` azul, `Central` vermelho — por silhueta e cor, 4 de 4 silhuetas e 4 de 4 cores distintas, reconhecíveis sem letreiro conforme GDD:1045 (FR-005, SC-003, Q2)
- [x] US3 (P2) fixa o avatar low-poly e o input: WASD/setas movem no plano XZ com direção normalizada e `Time` delta usando a função pura `deslocar`; a região é detectada pela função pura `ilha_em` de `lib.rs`, sem rede (FR-006, FR-007, SC-004)
- [x] US4 (P2) fixa o HUD: um único painel de texto exibe a posição do avatar e a ilha atual, fora do raio a operação é no-op determinístico, sem UI decorativa e sem servidor (FR-007, SC-002)
- [x] **Q1 — procedural sem assets externos:** todas as malhas são construídas em código com `Mesh3d`; nenhum `.gltf`, `.obj`, imagem, download ou asset externo neste ciclo; Kenney Nature Kit 3D (CC0) registrada apenas como fonte primária futura
- [x] **Q2 — sem textura nem letreiro:** 100% dos personagens do MVP usam formas geométricas sem textura aplicada; ilhas reconhecíveis apenas por silhueta+cor; nenhum modelo do pacote Paragon em captura do MVP
- [x] **Q3 — orçamento low-poly:** ≤ 10 mil triângulos por personagem (GDD:1127), aferível por leitura das malhas primitivas
- [x] **Q4 — regra da janela:** execução normal somente quando a SE solicitar; inspeção, debug, testes e validação de agente iniciam com `--minimizado`; flag preservada na migração
- [x] FR-001 mantém o mesmo pacote `insurgencia-cliente` em `crates/cliente`, `bevy = "=0.20.1"` em default features, dependência de `insurgencia-sim` preservada, sem `lightyear`/UDP/HTTP e sem novo crate
- [x] FR-002 mantém `App`/`DefaultPlugins` e único `Startup`, remove `Camera2d`/`Sprite` e qualquer cena Godot (`.tscn`/`project.godot`)
- [x] FR-003 exige exatamente 1 `Camera3d` perspectiva, iluminação de ambiente + direcional, fundo neutro e 0 câmeras duplicadas
- [x] FR-004 exige malhas 100% procedurais em código, sem carregamento de arquivo de asset nem download
- [x] FR-005 exige 4 de 4 ilhas `Mesh3d` com silhuetas e cores literalmente distintas, Central na origem e 4 de 4 reconhecíveis sem letreiro
- [x] FR-006 exige avatar low-poly distinto, 100% sem textura, ≤ 10 mil triângulos e nenhum modelo Paragon
- [x] FR-007 exige movimento normalizado com `Time` delta via `deslocar`, detecção por `ilha_em` (raio 1,5; empate = ilha mais próxima) e HUD de posição/ilha, tudo sem rede e com `lib.rs` intacto
- [x] FR-008 fixa `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, `unwrap_used`/`expect_used` em `deny`, `Result`/`?`/`thiserror`, sem desativação de lint; única exceção `#[expect(...)]` com razão auditável para proc-macro isolado
- [x] FR-008 mantém os quatro estágios de `verify.sh` na ordem e o `verify.sh: ALL STAGES OK`, os modos `--fast`, `--install-hook` e `--uninstall-hook`, a falha no primeiro estágio vermelho e o fallback documentado para `cargo test`
- [x] FR-008 preserva os cinco gates determinísticos alvo — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — os 20 testes do núcleo e `Makefile`, baselines e `evidence.csv` inalterados
- [x] Critérios de sucesso mensuráveis e binários: 4 de 4 estágios OK e `ALL STAGES OK` (SC-001); 1 de 1 `Camera3d` + `Mesh3d` com luz e 0 `Camera2d`/sprite (SC-002); 4 de 4 ilhas reconhecíveis por silhueta+cor (SC-003); 20 de 20 testes da lógica pura preservados (SC-004); 5 de 5 gates alvo PASS (SC-005)
- [x] Plano descreve a migração in-place em `crates/cliente` — `lib.rs` preservado sem Bevy e com seus 5 testes, `main.rs` convertido para 3D com iluminação/malhas/HUD, `Cargo.toml` intacto salvo necessidade, `verify.sh` inalterado — comandos de verificação e convergência READ-ONLY com correção na fonte
- [x] Tasks T001..T010 com marcas `[X]` nas fases Implementação, Verificação e Convergência, mapeando FRs/SCs e cobrindo preservação de `lib.rs`, câmera/luz, ilhas, avatar, input/detecção, HUD, gate e preservação dos 20 testes e dos cinco gates
- [x] Escopo explicitamente fora: código Rust, toolchain/dependências baixadas, janela aberta, `cargo run`, gates, teste, build, formatador, Godot, `make`, baseline, `evidence.csv`, `Makefile`, arquivos `.gd`, alteração nos crates mesclados, dependência de rede, artefatos R4/R7, stage, commit e push
- [x] Nenhum marcador `[NEEDS CLARIFICATION]` em aberto: as quatro ambiguidades (procedural, textura/letreiro, orçamento, janela) foram resolvidas em `## Clarifications` com fontes locais
- [x] Saídas não observadas permanecem **[NV]**; registro com data/hora pertence a R4, e versão, changelog com FR/T e rollback a R7; R4 precede R7
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado e nenhum comando executado nesta etapa de especificação
