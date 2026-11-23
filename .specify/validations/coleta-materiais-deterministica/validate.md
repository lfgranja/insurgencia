# Validação (R4) — 039-coleta-materiais-deterministica

**Data**: 2026-11-23T01:56Z · **Resultado**: **PASS (documental) — aprovado para R7** · R4 precede R7

| Verificação | Saída |
|---|---|
| Âncora | Worktree `.specify/queue/worktrees/coleta-materiais-039/`, branch `039-coleta-materiais-deterministica`, base `990ab06`; sessão `20261123T015356Z-coleta-materiais`. GDD §7.1, `crates/sim/src/lib.rs` e constituição v1.2.2 lidos; contrato técnico auditado pelo orquestrador registrado com fonte. |
| Escopo do código | Nenhuma alteração de código **por este agente**: os 14 artefatos são documentais. A implementação futura restrita a `crates/sim/src/lib.rs` está descrita no plano (T019, **[NV]**); este agente não tocou `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, GDScript, GUI, rede, dano nem barricada. Alterações concurrentes de código pertencem ao SWE/orquestrador e não são objeto desta validação documental. |
| Evidência local | Regra canônica 180 ticks a 1/30 s → +3 Suprimentos/+2 Materiais; saída antes zera parcial. Contrato: `TICKS_COLETA=180`, quatro campos em `Jogador`, `DestinoInvalido`, crédito atômico em `fase_skills`, `hash_estado` refletivo, `Comando.destino` como ID do ponto. Execução/gates **[NV]** (Princípio VI). |
| Roteiro de testes | `.specify/validations/coleta-materiais-deterministica/test-roadmap.md` escrito antes do PR: 10 passos, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback e os seis casos (180 ticks, saída antes, recusa sem destino, hash divergente, ausência de consumo parcial, determinismo). |
| Execução do roteiro | **[NV]** — nenhum build, gate, teste, commit, push, PR, merge ou deploy executado neste ciclo; a ser executado pelo orquestrador/SWE. |
| Baseline de converge | **[NV]** — `specs/039-coleta-materiais-deterministica/converge.md` é arquivo novo com a 1ª entrada; bootstrap do baseline append-only pelo orquestrador (fora da fronteira deste agente). Nenhum `converge.md` existente editado. |
| Governança | 14 artefatos na ordem da constituição v1.2.2: `... 09 checklist → revisão → 10 tasks (T012) → 11 analyze (T013, READ-ONLY) → 12 implement → 13 converge → 27 validate → 26 release`, R1 `go` (6/6 strong, 180/625), R4 antes de R7 `1.0.0`, `converge` append-only; checklist revisado (`[x]` = revisão, não implementação); **R6 suspensa por decisão ratificada**, fora do fluxo, PARE se invocada, nenhuma issue simulada; `.specify/memory/STATE.md` e alteração preexistente em `.specify/feature.json` preservados; rollback documentado. |

## R4
Aprovado para R7. A aprovação é documental: baseia-se nas fontes lidas, no contrato técnico auditado e na constituição v1.2.2 consultada. Nenhum verde é afirmado para a implementação em `lib.rs`, gates, roteiro executado, CI ou PR — todos **[NV]** até o SWE/orquestrador. A decisão 80/20 (180/625) é registrada com a fila documental esgotada como justificativa. A suspensão de R6 é requisito ratificado, não pendência, e nenhuma issue é simulada. Rollback documentado em `release.md`, nunca executado.
