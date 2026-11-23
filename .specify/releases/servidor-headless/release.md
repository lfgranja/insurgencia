# Release (R7) — 042-servidor-headless

**Data:** 2026-11-23T04:30Z · **Versão:** `1.0.0-local` (SemVer, pré-release local) · **R4:** aprovado em `.specify/validations/servidor-headless/validate.md` **(condicionado à execução do roteiro)**, antes desta release

## Changelog

### [1.0.0-local] — 2026-11-23

### Added
- FR-001 / T001 — fronteira registrada: novo crate `crates/servidor` com binário `insurgencia-servidor`, dependendo **apenas** de `insurgencia-sim`; `Cargo.toml` da workspace ganha o membro e `Cargo.lock` é gerado pelo Cargo; nenhuma dependência externa nova, nenhum async, nenhum socket e nenhum Lightyear.
- FR-002 / T002 — CLI registrada: `--ticks N` com inteiro positivo; ausência, zero, negativo e não-numérico são rejeitados com erro determinístico e código de saída **2**, sem pânico.
- FR-003 / T003, T004 — parser registrado: uma linha por comando, `peer_id acao seq request_id [x y]`, com `peer_id`/`seq`/`request_id` inteiros; coordenadas `x y` **obrigatórias** para `mover`, `coletar_materiais` e `construir_barreira` e **proibidas** para `sinalizar_sincronia`, `sinalizar_alavanca` e `atacar_forte`; ações conhecidas limitadas a `ACOES_PERMITIDAS` de `crates/sim/src/lib.rs`.
- FR-003 / T005 — agendamento registrado: toda a stdin é lida e os comandos são agendados, em ordem, antes do tick 0 no `Mundo` autoritativo (`agendar_comando`), sem reordenação por mapas, tempo ou aleatoriedade.
- FR-004 / T006, T007 — saída registrada: exatamente uma linha por tick, `<tick>\t<hash sha256>`, iniciando em `0`, com hash canônico de `Mundo::hash_estado()` (64 dígitos hexadecimais) e sequência contínua de ticks.
- FR-005 / T008 — rejeição registrada: entrada malformada e ação desconhecida produzem erro determinístico e saída **2**, propagados por `Result`/`ExitCode`, sem `unwrap`/`expect` e sem pânico.
- FR-006 / T009, T010 — testes registrados: testes unitários do parser (tokenização, obrigatoriedade/proibição de coordenadas, ações conhecidas) e teste de integração CLI (duas execuções idênticas byte a byte, número de linhas igual a `N` e rejeição de entrada inválida).
- FR-006 / T011 — convenções Rust registradas: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, `max_width = 100`, documentação `///` em itens públicos e nenhum `#[allow]`.
- FR-006 / T012 — governança registrada: constituição **v1.2.2**, ordem `... 09 checklist → revisão → 10 tasks → 11 analyze (READ-ONLY) → 12 implement → 13 converge → 27 validate → 26 release`, R1 `go`, R4 antes de R7 `1.0.0-local`, `converge` append-only; **R6 N/A** — fora da sequência executável até existir `resolve-tasks.sh`/adaptador, PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado, dívida de automação registrada; decisão 80/20 (256/625) registrada em `.specify/memory/STATE.md`.
- FR-006 / T012 — 5 artefatos na fronteira exata, com `tasks` antes de `analyze` READ-ONLY; `STATE.md`, `AGENTS.md`, `concurrency.jsonl`, paths alheios e `converge.md` existentes preservados.
- FR-006 / T012 — `test-roadmap.md` escrito antes do PR com **exatamente 10 passos**, superfície documental/CLI, capturas planejadas, `automatizável?`, rollback e os casos: CLI/`--ticks`, parser/coordenadas, agendamento antes do tick 0, ticks/contagem de linhas, hash por tick, determinismo entre duas execuções, rejeição com saída 2, fronteira de dependências/regras Rust e governança/R6 N/A.
- FR-006 / T012 — não-execução: nenhum build/gate/formatador/teste/commit/push/PR/merge/deploy; nenhum verde afirmado sem comando; pendências **[NV]**; código, gates e verificação ao SWE/orquestrador.

## Rastreabilidade

FR-001..FR-006 · SC-001..SC-006 · T001..T012 (SDD, com `tasks` antes de `analyze` READ-ONLY e artefatos de validação/release no mesmo intervalo) · R1 `go` (`.specify/memory/STATE.md`, decisão 80/20 do ciclo 042, 256/625) · R4 (`.specify/validations/servidor-headless/validate.md`, aprovado condicionado à execução do roteiro) · roteiro (`.specify/validations/servidor-headless/test-roadmap.md`, passos 2–8 para CLI/parser/agendamento/ticks/hash/determinismo/rejeição, passo 9 para fronteira de dependências e regras Rust, passo 10 para governança/R6 N/A e rollback). Âncoras: `crates/sim/src/lib.rs` (`Mundo`, `Comando`, `hash_estado`, `ACOES_PERMITIDAS`), `Cargo.toml` da workspace, constituição v1.2.2, branch `042-servidor-headless` (base `b099093`), sessão `20261123T043000Z-servidor-headless`.

## Rollback

Para reverter o ciclo 042: excluir os 5 artefatos escritos neste ciclo — `specs/042-servidor-headless/analyze.md`, `specs/042-servidor-headless/converge.md`, `.specify/validations/servidor-headless/test-roadmap.md`, `.specify/validations/servidor-headless/validate.md` e `.specify/releases/servidor-headless/release.md` — preservando ou restaurando `specs/042-servidor-headless/spec.md`, `plan.md`, `tasks.md` e `checklists/` conforme o estado de chegada definido pelo SDD. Se a implementação tiver sido entregue pelo SWE, remover `crates/servidor/`, reverter a entrada do membro no `Cargo.toml` da workspace e restaurar o `Cargo.lock` gerado pelo Cargo ao estado anterior. Se o baseline append-only tiver sido atualizado, remover a entrada 042. **Nunca executar neste ciclo — apenas documentar.** Não reverter o histórico de ciclos anteriores (inclusive PRs #52–#54), `docs/GDD*`, baselines, ledger, CI, `.specify/memory/STATE.md`, `AGENTS.md`, `concurrency.jsonl`, `.specify/feature.json` ou a alteração preexistente em qualquer caminho alheio. Não usar `git clean`, `git reset`, force-push, deletar worktree alheio nem desfazer merge/PR/deploy executados por outros agentes.

## Não incluído

Implementação em `crates/servidor/`, alterações no `Cargo.toml` da workspace ou no `Cargo.lock` por este agente; mudanças em `crates/sim`, `crates/contratos`, protocolo ou `PROTOCOL_VERSION`; async, sockets, Lightyear, novas dependências externas, janelas, credenciais ou rede; execução de `./verify.sh`, gates, roteiro, testes, capturas ou formatadores; adoção de R6 como regra ativa, `taskstoissues`/`resolve-tasks.sh` ou simulação de issues e `gh` improvisado; edição de `converge.md` existente, `.specify/memory/STATE.md`, `AGENTS.md`, `concurrency.jsonl` ou de paths alheios; commits, push, PRs, merge, deploy, assinatura GPG deste agente — todos **[NV]** até o orquestrador/SWE.

## Execução pelo orquestrador — 2026-11-23T04:38Z

R4 foi cumprido: roteiro **10/10 PASS**, `./verify.sh` **ALL STAGES OK** com **71/71 testes**, evidência em `.specify/validations/servidor-headless/evidence.md`. O crate `crates/servidor`, o membro da workspace e o `Cargo.lock` gerado pelo Cargo passam a fazer parte do escopo entregue. Rollback segue documentado e não executado; promoção e merge em `main` dependem do SWE.
