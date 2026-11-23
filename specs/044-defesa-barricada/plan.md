# Implementation Plan — 044-defesa-barricada

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 (validate) antes de R7 (release `1.0.0-local`); `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação fica registrada, não executada. Âncora: `.specify/memory/STATE.md` (2026-11-23T05:47Z), scorecard **256/625**.

**Fronteira explícita**: este agente escreve **somente** `specs/044-defesa-barricada/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/defesa-barricada-044/` (branch `044-defesa-barricada`, base `b6d2f36`). Não toca `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, código-fonte (salvo agentes de implementação na etapa 12), documentos derivados nem alterações preexistentes. Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]**.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`.
2. **01 Intake**: sessão `20261123T054700Z-defesa-barricada`, título `insurgencia - defesa de barricada`; 80/20 = **256/625** (`defesa-barricada`) sobre 108/625 (ponte), 48/625 (Súditos/Propaganda) e 10/625 (rede, bloqueada por ADR-004 até decisão da SE).
3. **02 Research**: GDD §7.1:306 (tabela «4. Defender», +1 ponto/10 s de barricada intacta), §6.2/:299 (pulso 30 s, `[HIPÓTESE]`) e :396 (sabotagem) lidos; `crates/sim/src/lib.rs` (`Jogador`, `Barreira`, `Mundo`, `fase_finish`, `hash_estado`, `ponto_igual`) conferido; constituição v1.2.2 e R6 N/A citadas com fonte; ADR-004 registrada como bloqueio de rede, não reaberta.
4. **03 Define (problem)**: a barricada do ciclo 040 (200 PV/360 ticks) não gera recompensa; falta crédito canônico de defesa, interrupção correta e reflexo no estado/hash; critérios observáveis por FR/SC.
5. **04 Shape (concept)**: 4 opções avaliadas no scorecard (defesa de barricada, ponte provisória, Súditos/Propaganda, rede); recomendação = **defesa-barricada**, sem mudança de stack.
6. **05 Decide**: scorecard **4×4×4×4 = 256/625**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A; critério impacto × confiança × facilidade ÷ risco, denominador 625.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006; regra canônica (10 s = 300 ticks a 1/30 s), papéis Guerreiro/Artesão como simplificação de escopo e hipóteses separadas.
8. **07 Clarify**: 10 s (300 ticks) ≠ «tempo alvo 8 s» da tabela; «intacta» = `pv > 0` e presente após decremento de duração; dono no ponto ≠ papéis implementados; pulso 30 s e vitória fora de escopo; `mover` sem mudança de posição não interrompe; pontos creditados não se perdem com a remoção; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos — estado/constantes, passo determinístico `fase_finish` e interrupção, `hash_estado`, HUD `Autogestão` e testes — com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; sem tocar `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`; sem Lightyear/async/sockets. Fora da fronteira deste agente.
15. **13 Converge (append-only)**: primeira entrada datada em `specs/044-defesa-barricada/converge.md`, datada; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade, rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/defesa-barricada/test-roadmap.md` (fora da fronteira deste agente). Superfície: core determinístico (`crates/sim`) + cliente/HUD; capturas de terminal; coluna `automatizável?`; rollback doc/estado.

1. Confirmar que R6 está fora do fluxo por constituição v1.2.2 (PARE se invocada; nenhuma issue simulada; dívida de automação registrada) e que a fronteira foi respeitada (somente 4 artefatos SDD).
2. Conferir 6 FR e 6 SC na spec e o mapeamento um-para-um no checklist, com rastreabilidade FR-001..FR-006 idêntica à dos artefatos de governança.
3. Verificar `Jogador.pontos_autogestao`, `Barreira.defesa_ticks` e as constantes documentadas, sem tocar `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`; ausência de Lightyear/async/sockets.
4. Validar 299 ticks sem crédito e o tick 300 creditando +1 com `defesa_ticks` zerado; confirmar o limite de 1 crédito por barricada (360 ticks).
5. Validar interrupção: `mover` com posição alterada, dono ausente e retorno ao ponto (contagem reinicia); `mover` sem mudança de posição não interrompe.
6. Validar que destruição (`pv <= 0`) e expiração (`ticks_restantes → 0`) antes de 300 não creditam, e que o tick da expiração não acumula; pontos já creditados permanecem.
7. Validar ordem determinística em `BTreeMap` por `id`, no máximo 1 incremento por barricada por tick e contadores independentes para duas barricadas no mesmo ponto.
8. Comparar `hash_estado` entre duas execuções com o mesmo input e conferir que crédito e progresso parcial alteram o hash.
9. Verificar o HUD com `Autogestão`, sem nova tecla nem campo de protocolo, e o estilo Rust (`forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`).
10. Rodar gates/build/testes/formatadores e comparar notas com o estado ao vivo; registrar resultados como observados ou **[NV]** — nunca afirmar verde sem saída colada; R4 antes de R7 `1.0.0-local`.

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 regra canônica (+1/300 ticks, GDD §7.1:306) | 12 implement | T011 |
| FR-002 contrato/estado (`pontos_autogestao`, `defesa_ticks`, constantes) | 12 implement | T011 |
| FR-003 comportamento/fase (`fase_finish`, interrupção, ordem) | 12 implement | T011 |
| FR-004 hash/determinismo | 12 implement | T011 |
| FR-005 cliente/HUD `Autogestão` | 12 implement | T011 |
| FR-006 governança/testes/R6 N/A | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Regra canônica reaberta ou hipótese promovida | GDD §7.1:306 é canônica (+1/10 s = 300 ticks a 30 Hz); pulso 30 s/`[HIPÓTESE]` e vitória ficam fora de escopo; «tempo alvo 8 s» não é intervalo de crédito |
| Papéis Guerreiro/Artesão implementados por engano | Simplificação registrada: dono no mesmo ponto é o defensor; sem sistema de papéis, reforço ou terceiros |
| Crédito de progresso interrompido | FR-003: zera em saída do dono, dono ausente, destruição e expiração; 299 não credita; contador é por barricada e descartado na remoção |
| Divergência determinística | FR-003/FR-004: passo em `fase_finish` após decremento de duração, iteração em ordem de `BTreeMap`, no máx. 1 avanço por barricada/tick, hash refletivo |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida de automação registrada |
| Stack/protocolo alterados | Fronteira explícita: nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, manifesto ou CI; ADR-004 mantém rede bloqueada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) antes de R7 (T012) |
| Afirmar verde sem execução | Gates/build/testes **[NV]** até o orquestrador; nada afirmado como verde sem saída colada |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md` e código intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)
- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`expect`/`#[allow]`.
- Confirmar o determinismo com o roteiro de 10 passos e registrar capturas; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004 e a simplificação de papéis registrada; R4 antes de R7 `1.0.0-local`.
