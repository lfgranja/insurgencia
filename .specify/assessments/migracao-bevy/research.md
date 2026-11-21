# Idea Research: Migração do Núcleo para Bevy 0.20.1 + Lightyear 0.31

- **Slug**: migracao-bevy
- **Created**: 2026-11-20
- **Evidence confidence (overall)**: **medium-high** — as afirmações centrais repousam em fontes primárias **locais e datadas**: ADR-004 (Accepted, 20/11/2026), dossiê do ciclo 020 (`docs/AVALIACAO-ENGINES-RUST.md`, 32 linhas de evidência, 10 opções, 10 critérios), skill `mdmv-rust-thin` e o registro de `cargo info bevy@0.20.1` na própria ADR-004. Não há verificação de comportamento: nenhum build, `verify.sh`, benchmark, reconexão ou migração real foi executado nesta etapa; toolchain, MSRV e compatibilidade são **citados**, não medidos.
- **Host policy (real, declarada)**: nenhuma consulta externa foi feita nesta etapa. O dossiê do ciclo 020 registra hosts fora da allowlist (`bevy.org`, `docs.rs`, `crates.io` etc.) como `[UNVERIFIED — fetch skipped]`; prevalecem as fontes primárias locais e os domínios `github.com` já registrados no ciclo anterior.
- **Cobertura linguística (real, declarada)**: português nas fontes normativas locais; os metadados de engine vêm de fontes em inglês consolidadas no ciclo 020, sem triangulação adicional nesta rodada.

## Users & Demand

- A demanda é da SE (2026-11-20): autorização explícita para reescrever o projeto em Bevy 0.20.1 + Lightyear 0.31, com evidência prévia (ciclo 020) e aprovação formal (ADR-004) já satisfeitas. [source: `docs/adr/004-migracao-bevy-lightyear.md:11,17`; `.specify/memory/STATE.md:3-6`] (confidence: high — registro normativo datado; texto literal **[NV]**)
- O consumidor final da mudança é o mantenedor único, que passa a operar um workspace Rust sob governança `mdmv-rust-thin`; C5 ("tempo até protótipo e curva") do ciclo 020 pesou 5* para Bevy+Lightyear (book do Lightyear em WIP, matriz de versões rígida) e C10 pesou 6* (acoplamento `aeronet`/`postcard`/`bevy_replicon`/Leafwing). [source: `docs/AVALIACAO-ENGINES-RUST.md:78,94,126,131`; skill `mdmv-rust-thin`] (confidence: medium — custo de curva não cronometrado)
- Os jogadores são servidos indiretamente: os artefatos do GDD (pilares, regras de ilha, condição de vitória, critérios §16.4) permanecem normativos e a migração não altera regras de jogo — apenas a implementação (ADR-004:17). [source: `docs/adr/004-migracao-bevy-lightyear.md:17`; `.specify/memory/constitution.md:13-17`] (confidence: high)

## Prior Art

- Ciclo 020 (mesclado, PR #26): `docs/AVALIACAO-ENGINES-RUST.md` consolidou 10 opções (O1 Godot 4.7.2 = 82,6 ADOPT inalterado; O3 Bevy 0.20.1 + Lightyear 0.31 = 77,1 ADAPT apenas como padrões em GDScript; O2 Bevy puro e O4 Fyrox travados em C2=4*; Fyrox/macroquad/ggez/Rapier REJECT; Ambient/gdext DEFER) e 5 relatórios em `docs/avaliacao-engines/pesquisa/`. Nenhum build, protótipo ou benchmark foi executado. [source: `docs/AVALIACAO-ENGINES-RUST.md:1-5,75-88,191-206`; `.specify/memory/STATE.md:5-8`] (confidence: high)
- ADR-001 (Aprovada, 19/11/2026) mantinha Godot-first vinculante e rejeitava "Servidor Rust como Plano B"; a ADR-004 (Accepted, 20/11/2026) **supera a ADR-001 na parte de linguagem/engine** mediante evidência prévia + aprovação formal, sem modificar o GDD. [source: `docs/adr/004-migracao-bevy-lightyear.md:7,11,28`; `docs/AVALIACAO-ENGINES-RUST.md:7`] (confidence: high)
- Código GDScript existente (Núcleo, servidor headless, Ilhas e testes) é pré-migração conhecido: 31 arquivos `.gd` segundo o custo assumido na ADR-004; permanece no histórico do Git como registro, sem apagamento. [source: `docs/adr/004-migracao-bevy-lightyear.md:31,42`] (confidence: medium — contagem declarada na ADR, não recontada aqui)
- Patterns de ECS e netcode autoritativo do ciclo 020 (inputs por *tick*, replicação determinística, predição/*rollback*, headless via `ScheduleRunnerPlugin` com `default-features=false`) são a base técnica da escolha, não evidência de execução local. [source: `docs/AVALIACAO-ENGINES-RUST.md:94,148-154,180-181`; ADR-004:30] (confidence: medium — proxy documental)

## Market & Context

- **Stack escolhida**: Rust 1.99.1 (stable), Bevy 0.20.1 e Lightyear 0.31.0. O crate publicado `bevy@0.20.1` declara `rust-version: 1.97.1` (registro de `cargo info` em 20/11/2026), divergente do `1.100.0` lido no `main`; a toolchain local `1.99.1` atende ao crate publicado. [source: `docs/adr/004-migracao-bevy-lightyear.md:13,49`; `docs/AVALIACAO-ENGINES-RUST.md:63-64`] (confidence: high para o registro; **[NV]** para compilação efetiva)
- **Estrutura alvo**: workspace Cargo com `crates/` (Rust-only conforme `projetos/AGENTS.md`), módulos `contratos`, `dominio/sim`, `servidor`, `aplicacao`. O código GDScript permanece apenas como histórico versionado; não há segunda linguagem no caminho crítico. [source: contexto de governança do projeto `projetos/AGENTS.md`; ADR-004:17,29] (confidence: medium — estrutura alvo declarada, ainda não materializada no alvo)
- **Alternativas descartadas na ADR-004**: manter Godot-first (rejeitada pela autorização explícita da SE e pela evidência do ciclo 020); Bevy puro sem Lightyear (C2=4*, trava inviolável); Fyrox (C2=4*, sem crate de rede no workspace); serviço Rust separado mantendo Godot no cliente (introduz segunda linguagem sem necessidade após a autorização). [source: `docs/adr/004-migracao-bevy-lightyear.md:35-38`] (confidence: high)
- **Custo assumido**: reescrita de 31 arquivos `.gd`, toolchain Rust e churn de API ~3 em 3 meses do Bevy 0.x; o README do Bevy admite estágio inicial, documentação escassa e migrações não garantidas. [source: `docs/adr/004-migracao-bevy-lightyear.md:31`; `docs/AVALIACAO-ENGINES-RUST.md:131,178-179`] (confidence: medium — churn é autoafirmação de primeira parte registrada)

## Data & Constraints

- **Autorização formal**: ADR-004 Accepted em 2026-11-20; nenhuma troca adicional de engine é permitida sem novo ADR. As regras obrigatórias de governança MDMV são parte da decisão, não sugestão (ADR-004:19-24). [source: `docs/adr/004-migracao-bevy-lightyear.md:5,19-24,53`] (confidence: high)
- **Gate Rust**: `./verify.sh` = check → clippy → nextest → fmt, o mesmo script do CI; `cargo nextest run` para zero regressões. `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, proibição de `.unwrap()`/`.expect()`, `Result` + `thiserror`/`anyhow`. [source: skill `mdmv-rust-thin`; ADR-004:20-23] (confidence: high)
- **Dependências**: versão fixada, justificativa no commit, gate `./scripts/supply-chain.sh` (ou `cargo deny check && cargo audit`), sem substituição automática de pacote; anti-*slopsquatting*. [source: skill `mdmv-rust-thin` — "Dependencies (supply-chain)"] (confidence: high)
- **Histórico**: código GDScript e testes não são apagados do Git; rollback da ADR-004 devolve a ADR-001 como decisão vigente sem alterar baseline de governança, gates ou trabalho de terceiros. [source: `docs/adr/004-migracao-bevy-lightyear.md:29,42`] (confidence: high)
- **Gate determinístico do alvo, citado e NÃO reexecutado**: **5/5 PASS** no estado vivo de 2026-11-20T23:52Z (ciclo 020 mesclado, PR #26): `spec_dirs=20`, `files=20`, `converge files=3 baselined=3` via bootstrap do próprio gate; `make check` e `make test-rede-e2e` permanecem exit 0 como estado técnico prévio, **sem alteração de código**. [source: `.specify/memory/STATE.md:3-9`] (confidence: high para a citação; execução nesta etapa **[NV]**)
- **Fluxo SDD**: intake → research → define → shape → decide → specify → …; nenhuma portaria é pulada; `converge.md` é append-only; R1 exige `go` e R4 precede R7. [source: `.specify/memory/constitution.md:11`] (confidence: high)

## Evidence Against

- **Nenhum código Rust no alvo**: a lacuna é literal — não existe `Cargo.toml`, workspace ou crate Rust no alvo; toda a implementação vigente é GDScript. A migração é, portanto, reescrita e não ajuste incremental. [source: inspeção da estrutura do alvo nesta etapa; ADR-004:31] (confidence: high para a ausência; contagem de 31 arquivos é declarada na ADR **[NV]**)
- **Toolchain e MSRV divergentes**: `main` do Bevy declara `1.100.0`, o crate publicado `1.97.1`; a escolha de `1.99.1` depende do registro de `cargo info` e não de compilação observada — se a resolução de dependências exigir MSRV maior, a suposição cai. [source: `docs/adr/004-migracao-bevy-lightyear.md:13`; `docs/AVALIACAO-ENGINES-RUST.md:63,126`] (confidence: medium)
- **Netcode não é nativo do Bevy**: o headless existe, mas autoridade/snapshots/reconexão dependem do Lightyear; reconexão robusta pós-desconexão permanece **[NV]** para todas as opções do ciclo 020. [source: `docs/AVALIACAO-ENGINES-RUST.md:123,180-181,219`] (confidence: high)
- **Custo de curva e churn**: Bevy 0.x quebra API ~a cada 3 meses sem garantia de migração fácil; book do Lightyear em WIP e matriz rígida de versões (0.31 → Bevy 0.20 / Rust 1.97.1+) aumentam o risco operacional (C5 5*, C10 6*). [source: `docs/AVALIACAO-ENGINES-RUST.md:126,131,178-179,212`] (confidence: medium)
- **Sem medição local**: todas as notas C6 do ciclo 020 são proxies documentais; nenhum benchmark 5v5+bots, custo por *tick*, latência ou memória foi medido. A vantagem de Godot (82,6 vs. 77,1) é documental e não invertida por desempenho observado — a migração é decisão autorizada, não conclusão de benchmark. [source: `docs/AVALIACAO-ENGINES-RUST.md:127,218`] (confidence: high)
- **Spec 021 ainda em template**: `specs/021-migracao-bevy/spec.md` está como Draft com marcadores `[FEATURE NAME]`/`[NEEDS CLARIFICATION]`; não há requisitos ou critérios de aceite materializados para a migração nesta etapa. [source: leitura local de `specs/021-migracao-bevy/spec.md:1-9,81-98`] (confidence: high)

## Gaps

- **[NV]** Nenhum build, `verify.sh`, clippy, nextest, teste, `make` ou Godot executado — o gate 5/5 é citado de STATE, não reexecutado.
- **[NV]** Toolchain local, resolução de dependências e MSRV real do crate publicado não confirmados por compilação; apenas registrados na ADR-004.
- **[NV]** Benchmark pareado Godot vs. Bevy+Lightyear (spike A), reconexão/handshake (spike B) e custo de migração/*edition* 2024 (spike C) permanecem sem execução — ponto cego que sustenta notas com `*`.
- **[NV]** Custo de reescrita, compatibilidade de regras de ilha/vitória após a troca de engine e equivalência funcional dos 31 arquivos `.gd` não medidos.
- **[NV]** NAT/UDP externo, lobby, 5v5, deploy, Postgres/Coolify remotos e sites fora da allowlist fora de alcance nesta etapa (STATE:9).
- **Estrutura alvo não materializada**: workspace `crates/` com `contratos`, `dominio/sim`, `servidor`, `aplicacao` é declarada no contexto de governança, ainda sem código no alvo.
- **Cobertura linguística**: metadados de engine em inglês, sem triangulação adicional nesta rodada; declarado, não presumido.

## Sources

- `docs/adr/004-migracao-bevy-lightyear.md` (ADR-004, Accepted, 20/11/2026) — autorização da SE, decisão, regras de governança, alternativas, rollback, rastreabilidade e registro de `cargo info bevy@0.20.1`. (confidence: high — fonte normativa local lida na íntegra)
- `docs/AVALIACAO-ENGINES-RUST.md` (20/11/2026) — dossiê do ciclo 020: critérios C1–C10, matriz Score/Fase 0, 32 linhas de evidência, Evidence Against, spikes A/B/C e limitações. (confidence: high para conteúdo; medium para notas com `*`)
- Skill `mdmv-rust-thin` (`SKILL.md`, lido nesta etapa) — `unsafe forbid`, `clippy pedantic deny`, proibição de `unwrap`/`expect`, `Result`/`thiserror`/`anyhow`, `./verify.sh`, nextest e gate de cadeia de suprimentos. (confidence: high)
- `.specify/memory/STATE.md` (estado vivo 2026-11-20T23:52Z) — ciclo 020 mesclado, PR #26, gates 5/5 citados, `[NV]` remanescentes. (confidence: high)
- `.specify/feature.json` e `specs/021-migracao-bevy/spec.md` — âncora da feature e estado Draft do template de spec. (confidence: high)
- `.specify/memory/constitution.md` e `docs/adr/001-excecao-linguagem-gdscript.md` (indireto, via ADR-004) — fluxo SDD, verificação honesta e decisão superada. (confidence: high para a constituição; medium para a ADR-001 lida indiretamente)
