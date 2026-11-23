# Intake — Coleta determinística de materiais

**Data**: 2026-11-23T01:53Z · **Slug**: `coleta-materiais-deterministica` · **Sessão**: `session_id=20261123T015356Z-coleta-materiais`, `session_title=insurgencia - coleta deterministica de materiais`.

## Ideia
Implementar, no Núcleo determinístico (`crates/sim/src/lib.rs`), a **Fase 1 — Coletar** do GDD §7.1: um Camponês que permanece **6 s (180 ticks a 1/30 s)** em ponto neutro recebe exatamente **+3 Suprimentos e +2 Materiais** no pulso; sair antes **zera o acúmulo parcial**. O sim não possui ponto neutro como entidade: usa-se `Comando.destino` como ID do ponto, **sem mudar o protocolo**. Este ciclo escreve **somente os 14 artefatos SDD**; a alteração de código fica ao SWE/orquestrador e permanece **[NV]**.

## 80/20 (decisão de escopo)
| Opção | Impacto × Confiança × Facilidade × Segurança ÷ 625 | Resultado |
|---|---|---|
| Coletar materiais (escolhida) | 5×4×3×3 = 180/625 | **180/625** |
| Documentação pós-v1.2.2 (ciclo 038) | 3×5×5×5 = 375/625 | 375/625 |

A fila documental está **esgotada** (o ciclo 038 já registrou o estado verificado pós-v1.2.2 e a deriva). Escolhe-se coletar materiais porque é o **menor incremento de produto com valor canônico** (GDD §7.1), com impacto máximo (5) e contrato técnico já auditado pelo orquestrador (confiança 4, facilidade 3, segurança 3).

## Evidências de entrada (auditadas/observadas; não reexecutadas)
- Worktree isolado `.specify/queue/worktrees/coleta-materiais-039/`, branch `039-coleta-materiais-deterministica`, base `990ab06`. `specs/039-coleta-materiais-deterministica/spec.md` é stub.
- [source: `docs/GDD - Insurgência.md` §7.1, lido] Tabela do pulso de 30 s, fase 1: Camponês em ponto neutro, 6 s, recompensa 3 Suprimentos + 2 Materiais; critério de aceitação: "sair antes zera o acúmulo parcial" (linhas ~299–314).
- [source: `crates/sim/src/lib.rs`, lido] `PASSO_SEGUNDOS = 1/30`; `ACOES_PERMITIDAS` já contém `coletar_materiais`; `Jogador` tem apenas `posicao`, `vida`, `seq`, `request_id`; `Comando.destino: Option<(f64, f64)>` hoje é usado por `mover`; `fase_skills` só aplica `mover`; `hash_estado` serializa `peer_id:posicao:vida:seq:request_id`; `ValidacaoComandos::validar` recusa `DestinoInvalido` apenas para `mover` sem destino.
- [source: contrato técnico auditado pelo orquestrador] Acrescentar `TICKS_COLETA=180`, campos `materiais`, `suprimentos`, `coleta_ticks`, `coleta_ponto` em `Jogador`; `validar` recusa `DestinoInvalido` se `coletar_materiais` sem destino ou destino ≠ posição; `fase_skills` acumula e credita atomicamente; `hash_estado` inclui os novos campos. **Nenhuma alteração** em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`.
- [source: `.specify/memory/constitution.md`, lido] Constituição **v1.2.2 do alvo** (ratificada 2026-11-19, última emenda 2026-11-23): ordem `00 → 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09 → revisão → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release`; `analyze` READ-ONLY depois de `tasks`; `converge` append-only; R1 `go`; R4 antes de R7 `1.0.0`; **R6 fora da sequência executável por decisão ratificada** — PARE se invocada, nunca simular issues.

## Restrição de execução
Este agente escreve **somente** os 14 artefatos SDD na fronteira exata. Não executa gates, build, formatador, testes, commit, push, PR, merge ou deploy; não edita `crates/sim/src/lib.rs`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `docs/GDD*`, `converge.md` de outros ciclos, `.specify/memory/STATE.md` (memória local do SWE, não versionada) nem paths fora da fronteira — inclusive a alteração preexistente em `.specify/feature.json`, que pertence ao usuário. Nenhuma issue é simulada e R6 não é executada. Tudo o que não foi executado permanece **[NV]**.
