# Checklist de requisitos — 022-simulacao-rust

**Data:** 2026-11-20T00:08Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação do crate `crates/sim`, escrita de código Rust, execução de `cargo`, gates, testes, build, formatadores, stage, commit ou push.

- [x] Revisão de requisitos concluída sobre a ADR-004 (Accepted, 20/11/2026), a spec 021 (`crates/contratos` já mesclado), `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `specs/009-servidor-validacao/spec.md:FR-001,FR-005,FR-007`, `verify.sh`, `clippy.toml`, `scripts/ci/`, `projetos/AGENTS.md`, `.specify/feature.json` e a skill `mdmv-rust-thin`
- [x] Saída desta etapa é exclusivamente documental: `spec.md`, `plan.md`, `checklists/requirements.md` e `tasks.md`; nenhum `crates/sim/`, `Cargo.toml`, `.gd`, gate, baseline, `evidence.csv`, `Makefile` ou artefato R4/R7 é escrito ou alterado aqui (FR-008)
- [x] US1 (P1) fixa o `Mundo`: `tick`, `epoch`, `match_id`, `jogadores`, `comandos_pendentes`, `comandos_do_tick`, `PASSO_SEGUNDOS = 1.0 / 30.0` e as três fases `prepare → skills → finish` na ordem exata, sem dependência de engine (FR-002, FR-003, SC-003)
- [x] US2 (P2) fixa exatamente cinco códigos — `PEER_NAO_AUTORIZADO`, `SEQ_OU_REQUEST_REPLAY`, `ACAO_DESCONHECIDA`, `MOVIMENTO_FORA_DO_LIMITE`, `OK` — e `LIMITE_MOVIMENTO_M = 1.0`, com a ordem peer → replay → ação → movimento conforme `validacao.gd` (FR-004, FR-005, SC-002)
- [x] US3 (P3) fixa `hash_estado` determinístico: SHA-256 de 64 hex sobre serialização em ordem fixa, estados iguais ⇒ hash igual e qualquer mudança altera o hash (FR-006, SC-004)
- [x] **Sem dependência de engine (Q1):** `crates/sim` depende somente de `insurgencia-contratos` e da biblioteca padrão; `bevy`/`lightyear` ficam nos crates de servidor/aplicação da spec 021; nenhuma versão flutuante ou substituição automática de pacote similar (FR-001)
- [x] **Tempo inteiro do servidor (Q2):** `tick`, `epoch`, `match_id`, `seq` e `request_id` são inteiros; `PASSO_SEGUNDOS = 1.0 / 30.0` preservado como literais de referência (`1`/`30`), não como relógio `f64`, eliminando não determinismo por arredondamento no `hash_estado` (FR-002, FR-006)
- [x] **Sem `unwrap`/`expect` (Q3):** `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, `unwrap_used`/`expect_used` em `deny`, erros via `Result<T, E>`, `?` e `thiserror`; nenhuma desativação de lint, única exceção `#[expect(...)]` com razão auditável para proc-macro isolado (FR-007, SC-001)
- [x] **`Tipos Irish` (Q4):** modelagem com tipos ricos (newtypes `Tick`, `Epoch`, `MatchId`, `PeerId` e enums `Fase`, `CodigoValidacao`, `Acao`) em vez de primitivos soltos, preservando os literais do GDScript e mantendo o crate sem engine (FR-004, FR-007)
- [x] FR-001 exige `Cargo.toml` em `crates/sim` com dependência fixada e explícita de `insurgencia-contratos` e ausência de `bevy`/`lightyear` neste crate
- [x] FR-003 exige as três fases na ordem exata `prepare → skills → finish`, com cópia de pendentes em `prepare`, aplicação em `skills`, limpeza em `finish` e incremento de `tick` ao final
- [x] FR-004/FR-005 exigem 5 de 5 códigos literais sem renomear, fundir ou omitir nenhum, com `OK` somente quando nenhuma recusa se aplica e movimento acima de `1.0` recusado (igual a `1.0` aceito)
- [x] FR-006 exige `hash_estado` com 100% de determinismo para estados iguais e 0 colisões por omissão de campo, sem relógio de parede ou aleatoriedade
- [x] FR-008 mantém os quatro estágios de `verify.sh` na ordem e o `verify.sh: ALL STAGES OK`, os modos `--fast`, `--install-hook` e `--uninstall-hook`, a falha no primeiro estágio vermelho e o fallback documentado para `cargo test`
- [x] FR-008 preserva os cinco gates determinísticos alvo — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e `Makefile`, baselines e `evidence.csv` inalterados
- [x] Critérios de sucesso mensuráveis e binários: 4 de 4 estágios `cargo` OK e `ALL STAGES OK` (SC-001); 5 de 5 códigos exatos (SC-002); 3 de 3 fases na ordem (SC-003); 100% de determinismo do hash (SC-004); 5 de 5 gates alvo PASS (SC-005)
- [x] Plano materializa `crates/sim/Cargo.toml` dependente de `insurgencia-contratos`, os `Tipos Irish`, o `Mundo`, a validação, o `hash_estado`, o gate `verify.sh` e os comandos `cargo`, com convergência READ-ONLY e correção na fonte
- [x] Tasks T001..T010 com marcas `[X]` nas fases Implementação, Verificação e Convergência, mapeando FRs/SCs e cobrindo crate, tipos, mundo/fases, validação, hash, lints, gate e preservação dos cinco gates
- [x] Escopo explicitamente fora: código Rust, toolchain/dependências baixadas, gate, teste, build, formatador, Godot, `make`, baseline, `evidence.csv`, `Makefile`, arquivos `.gd`, artefatos R4/R7, stage, commit e push
- [x] Nenhum marcador `[NEEDS CLARIFICATION]` em aberto: as quatro ambiguidades (engine, tempo inteiro, `unwrap`/`expect`, `Tipos Irish`) foram resolvidas em `## Clarifications` com fontes locais
- [x] Saídas não observadas permanecem **[NV]**; registro com data/hora pertence a R4, e versão, changelog com FR/T e rollback a R7; R4 precede R7
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado e nenhum comando executado nesta etapa de especificação
