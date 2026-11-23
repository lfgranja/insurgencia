# Implementation Plan — 040-barricada-deterministica

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 (validate) antes de R7 (release); `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação fica registrada, sem emenda neste ciclo.

**Fronteira explícita**: este agente escreve **somente** `specs/040-barricada-deterministica/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/barricada-040/` (branch `040-barricada-deterministica`, base `6b913f0`). Não toca `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, código-fonte (salvo agentes de implementação na etapa 12), documentos derivados nem alterações preexistentes. Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]**.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`.
2. **01 Intake**: sessão `20261123T024300Z-barricada-deterministica`, título `insurgencia - barricada deterministica de materiais`; 80/20 = 400/625 (`construir_barreira`) sobre 108/625, 48/625 e 10/625.
3. **02 Research**: GDD §9.3:475,483 e §10.3:574,577-579 / H4 lidos; constituição v1.2.2 e R6 N/A citadas com fonte; conflito §16.3/H4 registrado como risco, não reaberto.
4. **03 Define (problem)**: divergência determinística, ausência de custo aplicado e de feedback ao jogador; critérios observáveis por FR/SC.
5. **04 Shape (concept):** 4 opções avaliadas (barricada, ponte, combate/Fôlego, servidor+Lightyear); recomendação = barricada.
6. **05 Decide**: scorecard 5×5×4×4 = **400/625**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006; valores canônicos separados das `[HIPÓTESE]`.
8. **07 Clarify**: recusa sem consumo parcial ≠ débito na validação; limite por jogador ≠ global; `B` do cliente ≠ `Q` do GDD (input, não regra); R6 N/A ≠ dívida executada; §16.3/H4 = risco registrado.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = revisão de requisitos, **não** implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T014, IDs únicos e DAG; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: `analyze.md` fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos — constantes/estado/API, validação, `fase_skills`, `hash_estado`, cliente/HUD — com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`. Fora da fronteira deste agente.
15. **13 Converge (append-only)**: primeira entrada em `specs/040-barricada-deterministica/converge.md`, datada; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0`, changelog por FR/T, rastreabilidade, rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/barricada-deterministica/test-roadmap.md` (fora da fronteira deste agente). Superfície: core determinístico + cliente/HUD; capturas de terminal; coluna `automatizável?`; rollback doc/estado.

1. Confirmar que R6 está fora do fluxo por constituição v1.2.2 (PARE se invocada; nenhuma issue simulada; dívida de automação registrada).
2. Conferir 6 FR e 6 SC na spec e o mapeamento um-para-um no checklist.
3. Verificar constantes públicas e tipos (`Barreira`, campos de `Jogador`/`Mundo`, `CodigoRecusa::SaldoInsuficiente`) sem tocar `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`.
4. Validar recusa com 19 Materiais e com `destino != posicao`: nenhum débito, nenhuma canalização.
5. Validar conclusão em 45 ticks com débito de 20 Materiais e `Barreira` 200 PV/360 ticks.
6. Validar limite de 2 barricadas por jogador e remoção da mais antiga na 3ª.
7. Validar cancelamento por `mover` com mudança de posição e revalidação de saldo no tick final.
8. Comparar `hash_estado` entre duas execuções com o mesmo input (ordem de `BTreeMap`).
9. Verificar tecla `B` em *just pressed*: um comando, `seq`/`request_id` monotônicos, HUD com Materiais/Barreiras/Construindo.
10. Rodar gates/build/testes e comparar notas com o estado ao vivo; registrar resultados como observados ou **[NV]** — nunca afirmar verde sem saída colada.

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 API/constantes/tipos | 12 implement | T012 |
| FR-002 validação sem consumo parcial | 12 implement | T012 |
| FR-003 `fase_skills`/movimento/revalidação | 12 implement | T012 |
| FR-004 limite/remoção/PV/duração | 12 implement | T012 |
| FR-005 `hash_estado` | 12 implement | T012 |
| FR-006 input/HUD/estilo | 12 implement | T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Conflito §16.3/H4 reaberto | Registrado como risco: H4 é canônico (20/1,5 s/200/12 s); hipóteses de escala do MVP não são alteradas |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida de automação registrada |
| Consumo parcial em recusa | FR-002: débito apenas na conclusão, após revalidação (SC-002) |
| Divergência entre clientes | FR-003 + FR-005: 1 canalização por jogador/tick e hash ordenado por `BTreeMap` |
| Protocolo/contrato alterado por engano | Fronteira explícita: nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` |
| Hipóteses promovidas a requisito | Clarificações: Fôlego/alcance/recarga/dimensões fora de escopo |
| Ordem invertida por hábito | `tasks` (T010) antes de `analyze` (T011); R4 (T013) antes de R7 (T014) |
| Afirmar verde sem execução | Gates/build/testes **[NV]** até o orquestrador; nada afirmado como verde sem saída colada |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md` e código intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)
- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`expect`/`#[allow]`.
- Confirmar determinismo com o roteiro de 10 passos e registrar capturas; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo e o conflito §16.3/H4 como risco registrado.
