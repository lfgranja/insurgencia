# Implementation Plan — 046-suditos

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 (validate) antes de R7 (release `1.0.0-local`); `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação fica registrada, não executada. Âncora: `.specify/memory/STATE.md` (2026-11-23T06:58Z), scorecard Súditos/Propaganda **4×2×2×3 = 48/625**.

**Fronteira explícita**: este agente escreve **somente** `specs/046-suditos/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/suditos-046/` (branch `046-suditos`, base `d410bd6`). Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, código-fonte (salvo agentes de implementação na etapa 12), documentos derivados nem alterações preexistentes. Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]**.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`.
2. **01 Intake**: sessão `20261123T065800Z-suditos`, título `insurgencia - suditos e propaganda`; 80/20 = **48/625** (`suditos`) sobre Combate/Fôlego 48/625 e rede 10/625 (bloqueada por ADR-004 até decisão da SE); as opções 256/625 (ciclo 044) e 108/625 (ciclo 045) já foram entregues.
3. **02 Research**: GDD §9.2:464 (4 s = 120 ticks de Propaganda interrompem ataque a estruturas; dano de jogador reverte em 3 s = 90 ticks) lido; `crates/sim/src/lib.rs` (`ACOES_PERMITIDAS`, `Mundo`, `BTreeMap`, `BTreeSet`, `hash_estado`, `fase_finish`, `PASSO_SEGUNDOS`) e `crates/cliente/src/main.rs` (HUD, teclas `B`/`P`/`C`) conferidos; constituição v1.2.2 e R6 N/A citadas com fonte; ADR-004 registrada como bloqueio de rede, não reaberta.
4. **03 Define (problem)**: a sim não tem Súditos nem Propaganda; falta a regra canônica de conversão por exposição (120 ticks) e reversão por dano (90 ticks), com estado explícito, injeção determinística, hash refletivo e acesso/HUD no cliente; critérios observáveis por FR/SC.
5. **04 Shape (concept)**: 3 opções avaliadas no scorecard (Súditos/Propaganda, Combate/Fôlego, rede); recomendação = **suditos**, sem mudança de stack.
6. **05 Decide**: scorecard **4×2×2×3 = 48/625**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A; critério impacto × confiança × facilidade ÷ risco, denominador 625.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006; regra canônica (120/90 ticks a 1/30 s), `[HIPÓTESE]` de spawn/raio/funções como limitações fora de escopo.
8. **07 Clarify**: GDD §9.2:464 é canônica; spawn/6 m/funções são hipóteses fora de escopo; conversão é transição de estado, não sistema de ataque/panfletar; dano é exercitado por `aplicar_dano_a_sudito(id)`, sem física; `BTreeSet<u64>` deduplica exposição por ID por tick; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos — `propaganda` em `ACOES_PERMITIDAS` na sim (sem tocar `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`); struct pública `Sudito { id, posicao, convertido, propaganda_ticks, reversao_ticks }` em `BTreeMap<u64, Sudito>` (`Mundo::suditos`), APIs `adicionar_sudito`/`aplicar_dano_a_sudito` e constantes `TICKS_PROPAGANDA = 120`, `TICKS_REVERSAO = 90`; auto-avanço com `BTreeSet<u64>` (1 tick por Súdito por tick) e conversão no tick 120; reversão em 90 ticks por dano; `hash_estado`; HUD `Convertidos`/tecla `G` e testes — com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; sem Lightyear/async/sockets. Fora da fronteira deste agente.
15. **13 Converge (append-only)**: primeira entrada datada em `specs/046-suditos/converge.md`; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade, rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/suditos/test-roadmap.md` (fora da fronteira deste agente). Superfície: core determinístico (`crates/sim`) + cliente/HUD (`crates/cliente`); capturas de terminal; coluna `automatizável?`; rollback doc/estado.

1. Confirmar que R6 está fora do fluxo por constituição v1.2.2 (PARE se invocada; nenhuma issue simulada; dívida de automação registrada) e que a fronteira foi respeitada (somente 4 artefatos SDD nesta etapa).
2. Conferir 6 FR e 6 SC na spec e o mapeamento um-para-um no checklist, com rastreabilidade FR-001..FR-006 idêntica à dos artefatos de governança.
3. Verificar `ACOES_PERMITIDAS` com `propaganda`, a struct `Sudito`, `Mundo::suditos` (`BTreeMap<u64, Sudito>`), as APIs `adicionar_sudito`/`aplicar_dano_a_sudito` e as constantes `TICKS_PROPAGANDA = 120`, `TICKS_REVERSAO = 90`, sem tocar `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`; ausência de Lightyear/async/sockets.
4. Validar a regra canônica: 119 ticks mantêm o Súdito inimigo e o 120.º converte (para de atacar estruturas e passa a panfletar no estado); dano de jogador fixa 90 ticks e a conversão cessa exatamente no 90.º.
5. Validar o auto-avanço: dois comandos `propaganda` para o mesmo ID no mesmo tick incrementam `propaganda_ticks` exatamente 1 (`BTreeSet<u64>` deduplica); exposição e reversão avançam no máximo 1 tick por Súdito por tick, em ordem de `BTreeMap`.
6. Validar os casos limítrofes: ID inexistente em `aplicar_dano_a_sudito` sem efeito/pânico; novo dano reinicia a reversão em 90; nenhum Súdito injetado mantém o HUD em `Convertidos: 0`; hipóteses de spawn/raio/funções fora de escopo.
7. Comparar `hash_estado` entre duas execuções com o mesmo input e conferir que injeção, conversão e reversão alteram o hash.
8. Verificar o HUD com `Convertidos` e a tecla `G` gerando `propaganda`, sem novo campo de protocolo e sem remover teclas existentes.
9. Verificar o estilo Rust (`forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`) e os testes determinísticos dos casos limítrofes.
10. Rodar gates/build/testes/formatadores e comparar notas com o estado ao vivo; registrar resultados como observados ou **[NV]** — nunca afirmar verde sem saída colada; R4 antes de R7 `1.0.0-local`.

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 regra canônica (120 ticks de Propaganda, 90 ticks de reversão; GDD §9.2:464) | 12 implement | T011 |
| FR-002 contrato/estado (`propaganda`, `Sudito`, APIs, constantes, `Mundo::suditos`) | 12 implement | T011 |
| FR-003 comportamento/fase (auto-avanço com `BTreeSet`, conversão e reversão) | 12 implement | T011 |
| FR-004 hash/determinismo | 12 implement | T011 |
| FR-005 cliente/HUD `Convertidos` e tecla `G` | 12 implement | T011 |
| FR-006 governança/testes/R6 N/A | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Regra canônica reaberta ou hipótese promovida | GDD §9.2:464 é canônica (120/90 ticks); spawn/6 m/funções são `[HIPÓTESE]`, fora de escopo e rotulados como limitação |
| Conversão antecipada por flood de comandos | FR-003: `BTreeSet<u64>` deduplica por ID; no máx. 1 tick de exposição por Súdito por tick |
| Divergência determinística | FR-003/FR-004: ordem estável de `BTreeMap`, `hash_estado` refletivo, sem relógio/aleatoriedade; duas execuções idênticas comparadas |
| Sistema de combate introduzido por "dano" | Dano é exercitado apenas por `aplicar_dano_a_sudito(id)`; nenhum projétil/física — combate/Fôlego é candidato separado (48/625), fora de escopo |
| Stack/protocolo alterados | Fronteira explícita: nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, manifesto ou CI; ADR-004 mantém rede bloqueada |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida de automação registrada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) antes de R7 (T012) |
| Afirmar verde sem execução | Gates/build/testes **[NV]** até o orquestrador; nada afirmado como verde sem saída colada |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md` e código intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`expect`/`#[allow]`.
- Confirmar o determinismo com o roteiro de 10 passos e registrar capturas; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004 e as hipóteses de spawn/raio/funções como limitações registradas; R4 antes de R7 `1.0.0-local`.
