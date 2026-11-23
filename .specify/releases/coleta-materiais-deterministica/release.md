# Release (R7) — 039-coleta-materiais-deterministica

**Data**: 2026-11-23T01:56Z · **Versão**: `1.0.0` (SemVer) · **R4**: aprovado em `.specify/validations/coleta-materiais-deterministica/validate.md` (antes desta release)

## Changelog
### [1.0.0] — 2026-11-23
### Added
- FR-001 / T002, T003, T007 — regra canônica do GDD §7.1 registrada: 6 s = 180 ticks a 1/30 s em ponto neutro rendem exatamente +3 Suprimentos e +2 Materiais; sair antes zera o acúmulo parcial.
- FR-002 / T003, T009, T015 — contrato técnico auditado registrado: `TICKS_COLETA=180`; campos `materiais`/`suprimentos`/`coleta_ticks`/`coleta_ponto` em `Jogador`; `DestinoInvalido` para `coletar_materiais` sem destino ou destino ≠ posição; crédito atômico em `fase_skills`; `hash_estado` inclui os novos campos; `Comando.destino` é o ID do ponto, sem nova mensagem.
- FR-003 / T009, T013 — fronteira de implementação futura restrita a `crates/sim/src/lib.rs`; nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, GDScript, GUI, rede, dano, barricada ou R6.
- FR-004 / T003, T011, T017 — constituição **v1.2.2** do alvo aplicada: `... 09 checklist → revisão → 10 tasks → 11 analyze (READ-ONLY) → 12 implement → 13 converge → 27 validate → 26 release`, R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only; **R6 suspensa por decisão ratificada**, PARE se invocada, nenhuma issue simulada.
- FR-005 / T002, T005 — 80/20 documentado: 5×4×3×3 = 180/625 escolhido por fila documental esgotada e menor incremento com valor canônico; documentação pós-v1.2.2 (375/625) executada no ciclo 038.
- FR-006 / T007, T016 — 14 artefatos na fronteira exata, stub de `spec.md` substituído; `.specify/memory/STATE.md`, `converge.md` alheio e alteração preexistente em `.specify/feature.json` preservados.
- FR-007 / T015 — `test-roadmap.md` escrito antes do PR com 10 passos, superfície documental/CLI, capturas, `automatizável?`, rollback e os seis casos: 180 ticks, saída antes, recusa sem destino, hash divergente, ausência de consumo parcial e determinismo.
- FR-008 / T017, T018 — não-execução: nenhum build/gate/formatador/teste/commit/push/PR/merge/deploy; nenhum verde afirmado sem comando; pendências **[NV]**; código e verificação ao SWE/orquestrador.

## Rastreabilidade
FR-001..FR-008 · SC-001..SC-006 · T001..T019 · R1 `go` (`.specify/assessments/coleta-materiais-deterministica/decision.md`, scorecard 6/6 strong, 80/20 = 180/625) · R4 (`.specify/validations/coleta-materiais-deterministica/validate.md`) · roteiro (`.specify/validations/coleta-materiais-deterministica/test-roadmap.md`, passos 2–7 para os seis casos e 8–10 para fronteira/governança/rollback). Âncoras: GDD §7.1, `crates/sim/src/lib.rs`, contrato técnico auditado pelo orquestrador, constituição v1.2.2, branch `039-coleta-materiais-deterministica` (base `990ab06`), sessão `20261123T015356Z-coleta-materiais`.

## Rollback
Para reverter o ciclo 039: excluir os 5 artefatos de `.specify/assessments/coleta-materiais-deterministica/`, os 6 artefatos de `specs/039-coleta-materiais-deterministica/` (restaurando o stub de `spec.md`), `validate.md`, `test-roadmap.md` e `release.md`. Se o baseline append-only tiver sido atualizado, remover a entrada 039. **Nunca executar neste ciclo — apenas documentar.** Não reverter o histórico de ciclos anteriores, `docs/GDD*`, baselines, ledger, CI, `.specify/memory/STATE.md`, `.specify/feature.json`, `converge.md` alheio, paths untracked alheios, e não executar merge/PR/deploy nem reexecução de gates — decisão do SWE/orquestrador. A implementação em `lib.rs`, se já aplicada pelo SWE, é revertida pelo próprio SWE conforme seu controle de versão, não por este agente.

## Não incluído
Implementação em `crates/sim/src/lib.rs` por este agente; alterações em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`; barricada, dano/emboscada, GUI, rede, GDScript; entidade "ponto neutro" no protocolo; execução de `./verify.sh`, gates, roteiro ou capturas; adoção de R6 como regra ativa, `taskstoissues`/`resolve-tasks.sh` ou simulação de issues; edição de `converge.md` existente, `.specify/memory/STATE.md` ou da alteração preexistente em `.specify/feature.json`; janelas, credenciais, deploy, assinatura GPG, commits/push/PRs deste agente — todos **[NV]** até o orquestrador/SWE.
