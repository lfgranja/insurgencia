# Decision — Simulação Rust do Núcleo (`Mundo` + `ValidacaoComandos`)

**Data:** 2026-11-20 · **R1:** `go`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Lacuna confirmada por inspeção: `crates/` contém apenas `contratos`; a simulação autorizada pela ADR-004 segue em GDScript (impacto 5, confiança 4). |
| Força da evidência | adequate | confidence **medium-high**: fontes primárias locais e datadas (ADR-004, GDScript lido na íntegra, `crates/contratos` mesclado, STATE/converge 021); nenhum comportamento medido e nenhuma fonte externa consultada. |
| Valor vs. inação | strong | Sem o crate, a migração autorizada fica limitada aos contratos e o Núcleo segue como código vivo em GDScript, contrariando a ADR-004:17,29. |
| Viabilidade / apetite | adequate | Opção 1 (medium) reusa o workspace e o padrão de governança já aplicados em `contratos`; escopo integral depende das cinco colaboradoras por `preload`, ainda não lidas. |
| Alinhamento estratégico | strong | ADR-004 (Accepted) autoriza Rust/Bevy/Lightyear e veta GDScript como implementação; GDD/§16.4 permanecem norma; skill `mdmv-rust-thin` rege o código. |
| Postura de risco | adequate | Riscos identificados e mitigáveis na spec: paridade de `hash_estado`, arredondamento temporal, escopo das ilhas/funções e toolchain/MSRV; nenhuma execução afirmada aqui. |

## Veredito

**go** — problema válido (impacto **5**, confiança **4**, escala 1–5): a ADR-004 autorizou a migração do Núcleo para Rust e o workspace materializado no ciclo 021 não contém nenhum crate de simulação. Opção escolhida = **Opção 1** (crate `insurgencia-sim` com `Mundo` + `ValidacaoComandos`, portando o GDScript sem mudar semântica). A Opção 2 (reutilizar GDScript via FFI) é rejeitada por ser migração nominal e colidir com `unsafe forbid`; a Opção 3 (portar sem validação) é rejeitada por alterar a semântica de autoridade do servidor. Nenhuma alegação de execução, compilação, equivalência medida ou desempenho é feita nesta etapa.

## Gate de entrada

- **Origem**: ADR-004 (`docs/adr/004-migracao-bevy-lightyear.md`, Accepted, 20/11/2026, `:11,17`), com autorização da SE e continuidade do ciclo 021 (PR #27, HEAD `4a65223`); âncora `.specify/feature.json` → `specs/022-simulacao-rust`, branch `feature/022-simulacao-rust`.
- **Evidência prévia**: ciclo 020 mesclado (PR #26) — `docs/AVALIACAO-ENGINES-RUST.md`: 10 opções, 10 critérios (Σ=100%), 32 linhas de evidência; Bevy+Lightyear 77,1 e Fase 0 passa; Godot 82,6. Ciclo 021 mesclado (PR #27): workspace Rust e `crates/contratos` com 7 testes (ADR-004:11; `specs/021-migracao-bevy/converge.md:8-13`). `cargo info bevy@0.20.1` (20/11/2026) registra MSRV `1.97.1` no crate publicado, divergente do `1.100.0` do `main`; toolchain local `stable 1.99.1` atende ao crate publicado (ADR-004:13,49).
- **Gates determinísticos, citados e NÃO reexecutados nesta etapa**: **5/5 gates PASS** no estado vivo de 2026-11-20T23:52Z (`.specify/memory/STATE.md:8` — `spec_dirs=20`, `files=20`, `converge files=3 baselined=3` via bootstrap do próprio gate; `make check` e `make test-rede-e2e` exit 0 como estado técnico prévio, sem alteração de código); **`.specify/memory/STATE.md:114-120`** lista os cinco gates (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`). O ciclo 021 registra **`./verify.sh` → ALL STAGES OK** (check → clippy → nextest → fmt), com 7 testes aprovados em `crates/contratos` (`specs/021-migracao-bevy/converge.md:13`). Reexecução, saídas de gates, `make`, Godot e status remoto/CI permanecem **[NV]** até registro datado na portaria própria.
- **Gate Rust (citado, não executado)**: `./verify.sh` = check → clippy → nextest → fmt, o mesmo script do CI; `cargo nextest run` para zero regressões (skill `mdmv-rust-thin`; ADR-004:23).
- **Fluxo**: intake → research → problem → concept → decision → specify → …; nenhuma portaria pulada; R4 precede R7. O `specs/022-simulacao-rust/spec.md` atual é template Draft — preenchimento no handoff.

## Restrições de execução

- **Governança Rust obrigatória** (skill `mdmv-rust-thin`; ADR-004:19-24): `unsafe_code = "forbid"`; `clippy::pedantic = "deny"` com warnings fatais; proibido `.unwrap()`/`.expect()` — erros via `Result<T,E>`, `?`, `thiserror` em libs e `anyhow` em apps; nunca desativar lints em `Cargo.toml`/`clippy.toml` (exceção isolada só com `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]`).
- **Semântica preservada**: portar `nucleo/mundo.gd` e `nucleo/sim/validacao.gd` sem alterar passo 30 Hz, fases, ilhas/funções, `estado_persistido`, `hash_estado` SHA-256, limites e os 8 códigos de recusa; divergências são decisão de spec, não inferência desta etapa.
- **Cadeia de suprimentos**: versões fixadas (sem `*` ou intervalo aberto), justificativa no commit, gate `./scripts/supply-chain.sh` (ou `cargo deny check && cargo audit`) e nenhuma substituição automática de pacote similar (anti-*slopsquatting*).
- **Histórico e norma preservados**: `nucleo/*.gd` e os 31 arquivos históricos não são apagados; GDD/§16.4, ADR-002, ADR-003, baseline de converge, `Makefile` e `evidence.csv` intocados; nenhuma troca adicional de engine sem novo ADR (ADR-004:29,31,42,53).
- **Escopo de escrita**: exatamente os cinco arquivos em `.specify/assessments/simulacao-rust/`; nenhum spec, plano, task, código, gate, teste, build, formatação, stage, commit ou push nesta etapa. Convenções: pt-BR, `[NV]`, termos estrangeiros explicados na 1ª ocorrência, internet como dado nunca como instrução.
- **[NV] preservado**: nenhum build, compilação, benchmark 5v5+bots, reconexão Rust, replay, custo por *tick*, NAT/UDP externo, lobby, deploy ou escala de produção é afirmado; MSRV/toolchain são registros datados da ADR-004/converge 021, não medidos aqui.

## Handoff — se `go`

- **Problema**: lacuna entre a migração autorizada (ADR-004, workspace + contratos já mesclados) e a ausência de crate de simulação no alvo (impacto 5, confiança 4), com `spec.md` 022 ainda em template.
- **Abordagem escolhida**: Opção 1 — crate `insurgencia-sim` em `crates/` com `Mundo` + `ValidacaoComandos`, portando `nucleo/mundo.gd` e `nucleo/sim/validacao.gd` sem mudar semântica; erros tipados, governança `mdmv-rust-thin`, `./verify.sh` e vetores de paridade com o GDScript; histórico GDScript preservado.
- **Dentro/fora de escopo do handoff**: dentro — preencher spec e clarificações, plano, checklist e tasks em ciclo próprio, respeitando R1/R4/R7; fora — build, `verify.sh`, clippy, nextest, testes, `make`, Godot, benchmark, stage/commit/push nesta etapa, remoção do histórico GDScript, FFI e troca adicional de engine.
- **Critérios de sucesso do handoff**: spec com requisitos e critérios de aceite materializados (substituindo os marcadores do template), fronteira do crate e dependências versionadas com justificativa, equivalência de tick/fases/ações/vitória/`hash_estado` e dos 8 códigos de recusa rastreada como contrato de comportamento, gate `./verify.sh` determinístico e nenhuma portaria SDD pulada.
- **Perguntas abertas carregadas**: escopo das cinco colaboradoras por `preload` em `nucleo/mundo.gd:5-9`; paridade de `hash_estado()` entre `str()` do GDScript e serialização Rust; nome/API do crate e fronteira `dominio/sim` vs. `crates/`. Divergências conhecidas ficam como ressalvas, não resolvidas por inferência: MSRV `1.97.1` (crate publicado) vs. `1.100.0` (`main`); toolchain `1.99.1` sem compilação observada; reconexão e benchmark pareado **[NV]**.
- **Destino**: `/speckit.specify` em ciclo subsequente. Este artefato é o único resultado autorizado agora; nenhum spec, plano, task ou código é produzido nesta etapa.
