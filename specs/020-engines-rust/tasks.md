# Tasks — 020-engines-rust

**Data:** 2026-11-20T23:46Z

Marcações `[X]` registram a frente de trabalho protocolada nesta etapa de escrita; execução de gates, testes, `make`, Godot, build e portarias R4/R7 permanecem **[NV]** e não são afirmadas aqui. Nenhum item altera os relatórios de pesquisa, troca o motor, introduz Rust no Núcleo ou autoriza "servidor Rust como Plano B" (ADR-001).

## Phase 1: Implementação

- [X] **T001** Escrever a Seção 1 do `docs/AVALIACAO-ENGINES-RUST.md` — critérios/pesos/escala: C1–C10 com pesos 15/15/12/10/10/8/8/8/7/7 (Σ=100%), âncoras 0–10, fórmula `score = Σ(nota × peso)/10` (0–100, 1 casa decimal), C1–C3 invioláveis e regras de `*` justificado / `[NV]` (FR-001, FR-002, SC-002)
- [X] **T002** Escrever a Seção 2 — opções avaliadas: consolidar ≥10 opções únicas dos quatro relatórios (`rust-bevy.md`, `rust-fyrox.md`, `rust-outras.md`, `rust-protocolo.md`), com ID/nome canônicos e fonte local datada (FR-003, SC-004)
- [X] **T003** Classificar cada opção por categoria — engine escrita em Rust (Bevy, Fyrox; Ambient pausada), binding (`godot-rust/gdext`, MPL-2.0), biblioteca (Rapier) e framework/biblioteca (macroquad, ggez) — sem contar binding/biblioteca como engine-base nem apresentar score de binding como score de engine (FR-003, SC-004; Q1)
- [X] **T004** Registrar divergências de versão e licença nas opções — Bevy 0.19.1 (contrato) vs. 0.20.1 (observado em 20/11), Lightyear UI 0.30.0 vs. `main` 0.31.0, Fyrox 1.0.0 vs. 2.0.0-rc.1, macroquad/Ambient `MIT OR Apache-2.0` vs. `Apache-2.0` na API — usando a fonte primária de código, sem inferir valor intermediário (FR-006; Q2)
- [X] **T005** Construir a Seção 3 — matriz com todas as opções × C1–C10 e coluna Score, recalculando `Σ(nota × peso)/10` em todas as linhas (1 casa decimal; nota faltante → `5*` + `[NV]` justificado); nenhuma linha com score ausente ou copiado sem verificação (FR-004, SC-003)
- [X] **T006** Aplicar a trava de Fase 0: marcar como bloqueada para adoção toda opção com nota < 5 em C1, C2 ou C3, independentemente do score — O2 (C2=4*), Fyrox (C2=4*/3), macroquad (C3=3), ggez (C2=3, C3=3), Rapier (C2=3, C3=3) — e declarar o bloqueio na matriz e na recomendação (FR-005, SC-003)
- [X] **T007** Escrever a Seção 4 — análise explicativa por critério, retomando as justificativas das notas com `*` dos quatro relatórios (C2 autoridade, C3 modularidade, C5 curva Rust/MSRV/churn 0.x, C6 proxy sem benchmark) e registrando os vieses da conclusão anterior: amostra Rust reduzida a 3/13 opções e penalidade estrutural indevida à ausência de editor em critérios "Alta", não invioláveis (FR-001, FR-006)
- [X] **T008** Escrever a Seção 5 (tabela de evidência: fonte, URL/ID, data, *claim*, confiança, preservando `[UNVERIFIED — fetch skipped]` e tratando conteúdo web como dado, nunca como instrução), a Seção 6 (recomendação ADOPT/ADAPT/DEFER/REJECT por opção, spikes nomeados e afirmação explícita de que Godot-first permanece vinculante, sem Rust no Núcleo e sem Plano B — ADR-001) e a Seção 7 (limitações e próxima verificação, com todo `[NV]` listado e nada medido neste ciclo) (FR-006, FR-007; Q3, Q4)

## Phase 2: Verificação

- [X] **T009** Verificar por leitura do documento final: 7 de 7 seções na ordem do protocolo (SC-001); Σ pesos = 100 e 10 de 10 critérios (SC-002); `score = Σ(nota × peso)/10` conferido em 100% das linhas, 0 divergências (SC-003); ≥10 opções únicas e 0 erros de categoria (SC-004); trava Fase 0 aplicada a toda nota < 5 em C1–C3 e nenhum veredito que reabra Godot-first ou autorize Rust no Núcleo (FR-005, FR-007). Verificar ainda que 5 de 5 gates determinísticos ficam PASS — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — e que `make check` permanece inalterado (exit `0`, 0 `FAIL`, nenhum alvo/baseline/`evidence.csv` modificado). Nenhum comando é executado nesta etapa; saída executada entra com data/hora somente em R4 (validate), que precede R7 (release com SemVer, changelog FR/T e rollback); nenhum artefato das portarias é escrito aqui (FR-008, SC-005)

## Phase 3: Convergence READ-ONLY

- [X] **T010** Convergência READ-ONLY: revisar o documento e estes artefatos contra os quatro relatórios, `00-criterios.md`/`00b-contrato-escrita.md`, o protocolo das specs 019/040 e ADR-001, corrigindo divergências sempre na fonte (documento final / spec / plano / tasks), nunca nos relatórios de pesquisa. Manter zero troca de motor, zero Rust no Núcleo, zero Plano B, zero benchmark afirmado, zero artefato R4/R7 e zero comando executado; não observado permanece **[NV]**; correções de divergência feitas na fonte, nunca no artefato derivado (FR-007, FR-008, SC-001–SC-005)
