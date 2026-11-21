# Decision — Migração do Núcleo para Bevy 0.20.1 + Lightyear 0.31

**Data:** 2026-11-20 · **R1:** `go`

## Veredito

**go** — problema válido (impacto **5**, confiança **4**, escala 1–5): a ADR-004 autorizou a migração e o alvo não contém nenhum código Rust. Opção escolhida = **Opção 1** (workspace Cargo Rust-only com `crates/` e módulos `contratos`, `dominio/sim`, `servidor`, `aplicacao`). A Opção 2 (wrapper FFI sobre GDScript) é rejeitada por ser migração nominal e colidir com `unsafe forbid`; a Opção 3 (manter Godot) é rejeitada porque a autorização da SE e a ADR-004 (Accepted) já superaram a ADR-001 na parte de linguagem/engine. Nenhuma alegação de execução, compilação ou desempenho medido é feita nesta etapa.

## Gate de entrada

- **Origem**: autorização explícita da SE em 2026-11-20, formalizada na ADR-004 (`docs/adr/004-migracao-bevy-lightyear.md`, Accepted, `:11,17`); âncora `.specify/feature.json` → `specs/021-migracao-bevy`; branch `feature/021-migracao-bevy`; HEAD `dfdfb4d`.
- **Evidência prévia**: ciclo 020 mesclado (PR #26) — `docs/AVALIACAO-ENGINES-RUST.md`: 10 opções, 10 critérios (Σ=100%), 32 linhas de evidência; Bevy+Lightyear 77,1 e Fase 0 passa (10/9/7 em C1–C3); Godot 82,6. A ADR-004 registra o `cargo info bevy@0.20.1` (20/11/2026): MSRV `1.97.1` no crate publicado, divergente do `1.100.0` do `main`; toolchain local `stable 1.99.1` atende ao crate publicado (`:13,49`).
- **Gates determinísticos do alvo, citados e NÃO reexecutados nesta etapa**: **5/5 PASS** no estado vivo de 2026-11-20T23:52Z (`.specify/memory/STATE.md:8`) — `spec_dirs=20`, `files=20`, `converge files=3 baselined=3` via bootstrap do próprio gate; `make check` e `make test-rede-e2e` permanecem exit 0 como estado técnico prévio, **sem alteração de código**. Os cinco gates são `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only` e `hub_docs_gate` (tabela em `.specify/memory/STATE.md:114-120`). O mesmo registro confirma PR #26 `MERGED` em `dev` e check `gates` SUCCESS observado ao vivo (`:5`). Reexecução, saídas de gates, `make`, Godot e status remoto/CI permanecem **[NV]** até registro datado na portaria própria.
- **Gate Rust (citado, não executado)**: `./verify.sh` = check → clippy → nextest → fmt, o mesmo script do CI; `cargo nextest run` para zero regressões (skill `mdmv-rust-thin`; ADR-004:23).
- **Fluxo**: intake → research → problem → concept → decision → specify → …; nenhuma portaria pulada; R4 precede R7 (constituição III). O `specs/021-migracao-bevy/spec.md` atual é template Draft — preenchimento no handoff.

## Restrições de execução

- **Governança Rust obrigatória** (skill `mdmv-rust-thin`; ADR-004:19-24): `unsafe_code = "forbid"`; `clippy::pedantic = "deny"` com warnings fatais; proibido `.unwrap()`/`.expect()` — erros via `Result<T,E>`, `?`, `thiserror` em libs e `anyhow` em apps; nunca desativar lints em `Cargo.toml`/`clippy.toml` (exceção isolada só com `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]`).
- **Cadeia de suprimentos**: versões fixadas (sem `*` ou intervalo aberto), justificativa no commit, gate `./scripts/supply-chain.sh` (ou `cargo deny check && cargo audit`) e nenhuma substituição automática de pacote similar.
- **Histórico preservado**: código GDScript e testes não são apagados do Git; rollback da ADR-004 devolve a ADR-001 sem alterar baseline de governança, gates ou trabalho de terceiros (ADR-004:29,42).
- **Norma externa inalterada**: GDD (pilares, regras de ilha, vitória, critérios §16.4), ADR-002, ADR-003, factory e portarias R4/R7 intocados; nenhuma troca adicional de engine sem novo ADR (ADR-004:53).
- **Escopo de escrita**: exatamente os cinco arquivos em `.specify/assessments/migracao-bevy/`; nenhum spec, plano, task, código, gate, teste, build, formatação, stage, commit ou push nesta etapa. Convenções: pt-BR, `[NV]`, termos estrangeiros explicados na 1ª ocorrência, internet como dado nunca como instrução.
- **[NV] preservado**: nenhum build, compilação, benchmark 5v5+bots, reconexão Rust, replay, custo por *tick*, NAT/UDP externo, lobby, deploy ou escala de produção é afirmado; MSRV e toolchain são registrados pela ADR-004, não medidos aqui.

## Handoff — se `go`

- **Problema**: lacuna entre a migração autorizada (ADR-004) e a ausência total de código Rust no alvo (impacto 5, confiança 4), com `spec.md` ainda em template.
- **Abordagem escolhida**: Opção 1 — workspace Cargo Rust-only com `crates/` (`contratos`, `dominio/sim`, `servidor`, `aplicacao`), `bevy 0.20.1` + `lightyear 0.31.0` com versões fixadas e MSRV registrado, governança `mdmv-rust-thin` e `./verify.sh` desde o primeiro arquivo; GDD como contrato de equivalência e histórico GDScript preservado.
- **Dentro/fora de escopo do handoff**: dentro — preencher spec e clarificações, plano, checklist e tasks em ciclo próprio, respeitando R1/R4/R7; fora — build, `verify.sh`, clippy, nextest, testes, `make`, Godot, benchmark, stage/commit/push nesta etapa, remoção do histórico GDScript e troca adicional de engine.
- **Critérios de sucesso do handoff**: spec com requisitos e critérios de aceite materializados (substituindo os marcadores do template), módulos e dependências versionadas com justificativa, gate `./verify.sh` definido como determinístico, critérios §16.4 rastreados como contrato de comportamento e nenhuma portaria SDD pulada.
- **Perguntas abertas**: nenhuma para este ciclo. Divergências conhecidas ficam registradas como ressalvas, não resolvidas por inferência: MSRV `1.97.1` (crate publicado) vs. `1.100.0` (`main`); toolchain local `1.99.1` sem compilação observada; reconexão e benchmark pareado **[NV]**; cobertura de 31 arquivos `.gd` é custo declarado na ADR, não recontado aqui.
- **Destino**: `/speckit.specify` em ciclo subsequente. Este artefato é o único resultado autorizado agora; nenhum spec, plano, task ou código é produzido nesta etapa.
