# Implementation Plan — 045-ponte-provisoria

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 (validate) antes de R7 (release `1.0.0-local`); `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação fica registrada, não executada. Âncora: `.specify/memory/STATE.md` (2026-11-23T06:25Z), scorecard **3×3×3×4 = 108/625**, GDD §9.3:475,483 e §8.4:407, base `fa33052`, sessão `20261123T062500Z-ponte-provisoria`.

**Fronteira explícita**: este agente escreve **somente** `specs/045-ponte-provisoria/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/ponte-provisoria-045/` (branch `045-ponte-provisoria`, base `fa33052`). Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, código-fonte (salvo agentes de implementação na etapa 12), documentos derivados nem alterações preexistentes. Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]**.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`.
2. **01 Intake**: sessão `20261123T062500Z-ponte-provisoria`, título `insurgencia - ponte provisoria deterministica`; 80/20 = **108/625** (`ponte-provisoria`) sobre 48/625 (Súditos/Propaganda), 48/625 (Combate/Fôlego) e 10/625 (rede, bloqueada por ADR-004 até decisão da SE); a opção 256/625 do ciclo 044 já foi entregue.
3. **02 Research**: GDD §9.3:475,483 (15 Materiais, recusa sem consumo parcial) e §8.4:407 (60 PV, 45 s, atalho de 8 s dependente de mapa/ilhas) lidos; `crates/sim/src/lib.rs` (`ACOES_PERMITIDAS`, `Mundo`, `Barreira`, `ValidacaoComandos`, `fase_finish`, `hash_estado`, `ponto_igual`) e `crates/cliente/src/main.rs` (HUD, teclas `B`/`C`) conferidos; constituição v1.2.2 e R6 N/A citadas com fonte; ADR-004 registrada como bloqueio de rede, não reaberta.
4. **03 Define (problem)**: a sim não tem ponte provisória; falta o recurso canônico de 15 Materiais/60 PV/45 s com recusa sem consumo parcial, criação atômica, expiração determinística, hash refletivo e acesso/HUD no cliente; critérios observáveis por FR/SC.
5. **04 Shape (concept)**: 4 opções avaliadas no scorecard (ponte provisória, Súditos/Propaganda, Combate/Fôlego, rede); recomendação = **ponte-provisoria**, sem mudança de stack.
6. **05 Decide**: scorecard **3×3×3×4 = 108/625**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A; critério impacto × confiança × facilidade ÷ risco, denominador 625.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006; regra canônica (15 Materiais, 60 PV, 1350 ticks a 1/30 s), atalho de 8 s como limitação fora de escopo e 45 ticks de construção como escolha de protótipo.
8. **07 Clarify**: 15 Materiais e 60 PV/45 s são canônicos; atalho de 8 s depende de mapa/ilhas (fora de escopo); 45 ticks de construção ≠ 45 s de duração; `pv = 60` não introduz sistema de dano; ponte não credita autogestão nem altera barricadas/vitória; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos — `construir_ponte` em `ACOES_PERMITIDAS`, `Ponte`/`Mundo::pontes`/constantes, validação e débito atômico, expiração em `fase_finish`, `hash_estado`, HUD `Pontes`/tecla `P` e testes — com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; sem tocar `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`; sem Lightyear/async/sockets. Fora da fronteira deste agente.
15. **13 Converge (append-only)**: primeira entrada datada em `specs/045-ponte-provisoria/converge.md`; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade, rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/ponte-provisoria/test-roadmap.md` (fora da fronteira deste agente). Superfície: core determinístico (`crates/sim`) + cliente/HUD (`crates/cliente`); capturas de terminal; coluna `automatizável?`; rollback doc/estado.

1. Confirmar que R6 está fora do fluxo por constituição v1.2.2 (PARE se invocada; nenhuma issue simulada; dívida de automação registrada) e que a fronteira foi respeitada (somente 4 artefatos SDD nesta etapa).
2. Conferir 6 FR e 6 SC na spec e o mapeamento um-para-um no checklist, com rastreabilidade FR-001..FR-006 idêntica à dos artefatos de governança.
3. Verificar `ACOES_PERMITIDAS` com `construir_ponte`, a struct `Ponte`, `Mundo::pontes` (`BTreeMap<u64, Ponte>`) e as constantes `CUSTO_PONTE_MATERIAIS = 15`, `PV_PONTE = 60`, `TICKS_DURACAO_PONTE = 1350`, `TICKS_CONSTRUCAO_PONTE = 45`, sem tocar `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`; ausência de Lightyear/async/sockets.
4. Validar a regra canônica: 14 Materiais recusam com `SaldoInsuficiente` sem débito nem canalização; 15 Materiais iniciam a canalização sem débito antecipado; `destino` fora do ponto recusa com `DestinoInvalido`.
5. Validar a conclusão: 44 ticks sem ponte, tick 45 com débito atômico **único** de 15, criação com `pv = 60` e `ticks_restantes = 1351` (unidade extra do tick de criação); interrupção por posição ou saldo zera sem criar.
6. Validar a expiração determinística: ponte presente no tick 1349 e removida no tick 1350 por `fase_finish`, em ordem de `BTreeMap`, sem relógio/aleatoriedade e sem efeito em barricadas/jogadores.
7. Comparar `hash_estado` entre duas execuções com o mesmo input e conferir que criação, `pv` e expiração alteram o hash.
8. Verificar o HUD com `Pontes` e a tecla `P` gerando `construir_ponte` na posição atual, sem novo campo de protocolo e sem remover teclas existentes.
9. Verificar o estilo Rust (`forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`) e os testes determinísticos dos casos limítrofes.
10. Rodar gates/build/testes/formatadores e comparar notas com o estado ao vivo; registrar resultados como observados ou **[NV]** — nunca afirmar verde sem saída colada; R4 antes de R7 `1.0.0-local`.

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 regra canônica (15 Materiais, 60 PV, 1350 ticks; GDD §9.3:475,483 e §8.4:407) | 12 implement | T011 |
| FR-002 contrato/estado (`construir_ponte`, `Ponte`, constantes, `Mundo::pontes`) | 12 implement | T011 |
| FR-003 comportamento/fase (validação, débito atômico, expiração em `fase_finish`) | 12 implement | T011 |
| FR-004 hash/determinismo | 12 implement | T011 |
| FR-005 cliente/HUD `Pontes` e tecla `P` | 12 implement | T011 |
| FR-006 governança/testes/R6 N/A | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Regra canônica reaberta ou hipótese promovida | GDD §9.3:475,483 e §8.4:407 são canônicas (15/60/1350); atalho de 8 s depende de mapa/ilhas e fica fora de escopo; 45 ticks de construção rotulados como protótipo |
| Consumo parcial em recusa | FR-001/FR-003: `SaldoInsuficiente` antes de qualquer débito; débito atômico e único no tick 45, condicionado à criação |
| Divergência determinística | FR-003/FR-004: canalização 1 tick por jogador por tick, `BTreeMap` por `id`, expiração em `fase_finish`, hash refletivo, sem relógio/aleatoriedade |
| Dano/combate introduzido por causa do campo `pv` | `pv` é atributo canônico refletido no estado/hash; nenhum sistema de ataque é criado — combate é candidato separado (48/625), fora de escopo |
| Stack/protocolo alterados | Fronteira explícita: nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, manifesto ou CI; ADR-004 mantém rede bloqueada |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida de automação registrada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) antes de R7 (T012) |
| Afirmar verde sem execução | Gates/build/testes **[NV]** até o orquestrador; nada afirmado como verde sem saída colada |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md` e código intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`expect`/`#[allow]`.
- Confirmar o determinismo com o roteiro de 10 passos e registrar capturas; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004 e o atalho de 8 s como limitação registrada; R4 antes de R7 `1.0.0-local`.
