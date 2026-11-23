# Implementation Plan — 047-combate-folego

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 (validate) antes de R7 (release `1.0.0-local`); `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação fica registrada, não executada.

**Fronteira explícita**: este agente escreve **somente** `specs/047-combate-folego/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/combate-047/` (branch `047-combate-folego`, base `0ac4c9f`). Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, código-fonte (salvo agentes de implementação na etapa 12), documentos derivados nem alterações preexistentes. Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]**.

**Rotulagem obrigatória**: todo número de combate é `[HIPÓTESE]` (GDD §10.1: Fôlego 100, regeneração 10/s fora e 5/s em combate, alcance contato 2 m) ou escolha de protótipo do ciclo (custo 30, dano 10, combate 90 ticks; +1 a cada 3/6 ticks a 30 Hz). **Nenhum valor pode ser apresentado como cânone** em qualquer um dos 4 artefatos.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`.
2. **01 Intake**: sessão `20261123T073500Z-combate-folego`, título `insurgencia - combate e folego`; 80/20 = **48/625** (`combate-folego`) contra rede 10/625 (bloqueada por ADR-004 até decisão da SE); as frentes 256/625 (ciclo 044), 108/625 (045) e 48/625 (046) já foram entregues.
3. **02 Research**: GDD §10.1 (Fôlego 100, regeneração 10/s fora de combate e 5/s em combate, alcance contato 2 m — todos `[HIPÓTESE]`), `.specify/memory/STATE.md` (decisão 2026-11-23T07:35Z), `crates/sim/src/lib.rs` (`ACOES_PERMITIDAS` já contém `atacar_forte`; `Jogador`, `vida`, `PASSO_SEGUNDOS`, `ponto_igual`, `fase_finish`, `hash_estado`, `CodigoRecusa`, `BTreeMap`, `BTreeSet`) e `crates/cliente/src/main.rs` (HUD e teclas `WASD`/`B`/`P`/`C`/`G`) conferidos; constituição v1.2.2 e R6 N/A citadas com fonte; ADR-004 registrada como bloqueio de rede, não reaberta.
4. **03 Define (problem)**: a sim lista `atacar_forte` em `ACOES_PERMITIDAS`, mas a ação não tem efeito; falta Fôlego, custo, dano com `saturating_sub`, validação de alvo/alcance/Fôlego, deduplicação por atacante, combate de 90 ticks, regeneração acumulada, reflexo no hash e acesso/HUD no cliente; critérios observáveis por FR/SC, com todos os valores rotulados como hipótese.
5. **04 Shape (concept)**: 2 opções avaliadas no scorecard (Combate/Fôlego e rede); recomendação = `combate-folego`, sem mudança de stack.
6. **05 Decide**: scorecard **4×2×2×3 = 48/625**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A; critério impacto × confiança × facilidade ÷ risco, denominador 625.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US7, FR-001..FR-006 e SC-001..SC-006; todas as hipóteses de GDD §10.1 e as escolhas de protótipo (30/10/90 ticks) marcadas como tais, nunca como cânone.
8. **07 Clarify**: GDD §10.1 é fonte de hipóteses, não de cânone neste ciclo; combate é transição de estado (`combate_ticks`), não física/projétil; alvo é o jogador no `destino` a ≤ 2 m (`[HIPÓTESE]`); `BTreeSet<u64>` deduplica ataques por atacante por tick; acumulador `folego_ticks` é preservado na troca de taxa; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG linear; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 nos paths de código atribuídos — dar efeito a `atacar_forte` (já em `ACOES_PERMITIDAS` na sim, sem tocar `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml`); `Jogador` ganha `folego`, `folego_ticks`, `combate_ticks` (incluídos no hash); constantes documentadas e rotuladas como `[HIPÓTESE]`/protótipo; validação de alvo no ponto/alcance e de Fôlego com `CodigoRecusa::AlvoInvalido`/`FolegoInsuficiente`; dano `saturating_sub`; um ataque por jogador por tick via `BTreeSet<u64>`; regeneração em `fase_finish` com acumulador (+1/3 ticks fora de combate, +1/6 em combate); cliente com tecla `Q` e HUD `Fôlego`/`Vida` — com `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; sem Lightyear, async, sockets, janela ou relógio externo.
15. **13 Converge (append-only)**: primeira entrada datada em `specs/047-combate-folego/converge.md`; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade, rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/combate-folego/test-roadmap.md` (fora da fronteira deste agente). Superfície: core determinístico (`crates/sim`) + cliente/HUD (`crates/cliente`); capturas de terminal; coluna `automatizável?`; rollback doc/estado.

1. Confirmar que R6 está fora do fluxo por constituição v1.2.2 (PARE se invocada; nenhuma issue simulada; dívida de automação registrada) e que a fronteira foi respeitada (somente 4 artefatos SDD nesta etapa).
2. Conferir 6 FR e 6 SC na spec e o mapeamento um-para-um no checklist, com rastreabilidade FR-001..FR-006 idêntica à dos artefatos de governança e com **todo número de combate rotulado como `[HIPÓTESE]`/protótipo, nunca cânone**.
3. Verificar `atacar_forte` em `ACOES_PERMITIDAS` sem mudança de contrato, `Jogador` com `folego`/`folego_ticks`/`combate_ticks`, as sete constantes documentadas e `CodigoRecusa::FolegoInsuficiente`/`AlvoInvalido`, sem tocar `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`; ausência de Lightyear/async/sockets.
4. Validar custo e dano: ataque custa **30** (`[HIPÓTESE]`), causa **10** de dano (`[HIPÓTESE]`) com `saturating_sub` no piso e instaura **90 ticks** de combate; Fôlego zera em `30` e `folego = 29` é recusado.
5. Validar alvo e deduplicação: alvo no `destino` a ≤ **2 m** (`[HIPÓTESE]`), sem alvo/fora de alcance retorna `AlvoInvalido`; dois comandos do mesmo atacante no mesmo tick aplicam custo e dano uma única vez (`BTreeSet<u64>`); atacantes distintos não se bloqueiam.
6. Validar regeneração: +1 a cada **3 ticks** fora de combate e +1 a cada **6 ticks** em combate, acumulador preservado na troca de taxa, teto de **100** sem crédito excedente, `combate_ticks` caindo de 90 a 0 em 90 ticks.
7. Comparar `hash_estado` entre duas execuções com o mesmo input e conferir que ataque, dano, regeneração e fim de combate alteram o hash, em ordem de `BTreeMap`.
8. Verificar o HUD com `Fôlego` e `Vida` e a tecla `Q` gerando `atacar_forte`, sem novo campo de protocolo e sem remover teclas existentes.
9. Verificar o estilo Rust (`forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`) e os testes determinísticos dos casos limítrofes (replay, alvo inválido, Fôlego insuficiente, piso de vida).
10. Rodar gates/build/testes/formatadores e comparar notas com o estado ao vivo; registrar resultados como observados ou **[NV]** — nunca afirmar verde sem saída colada; R4 antes de R7 `1.0.0-local`.

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 regra de Fôlego/combate (30/10/90 ticks, regeneração 3/6 ticks, 2 m — `[HIPÓTESE]`/protótipo) | 12 implement | T011 |
| FR-002 contrato/estado (`atacar_forte`, campos do `Jogador`, constantes, `CodigoRecusa`) | 12 implement | T011 |
| FR-003 comportamento/fase (alvo/alcance/Fôlego, `BTreeSet`, `saturating_sub`, regeneração em `fase_finish`) | 12 implement | T011 |
| FR-004 hash/determinismo | 12 implement | T011 |
| FR-005 cliente/HUD `Fôlego`/`Vida` e tecla `Q` | 12 implement | T011 |
| FR-006 governança/testes/R6 N/A | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Valor hipotético promovido a cânone | GDD §10.1 é fonte de `[HIPÓTESE]`; custo 30/dano 10/90 ticks são escolhas de protótipo; rotulagem em todas as seções dos 4 artefatos e no roteiro (passo 2) |
| Dano múltiplo por flood de comandos | FR-003: `BTreeSet<u64>` deduplica por atacante; no máx. um ataque por jogador por tick |
| Vida negativa ou estouro de Fôlego | `saturating_sub` no dano; teto `FOLEGO_MAXIMO` na regeneração, sem crédito acumulado |
| Ataque à distância ou sem custo | Validação de alvo no `destino` e no alcance hipotético de 2 m (`AlvoInvalido`) e de Fôlego ≥ custo (`FolegoInsuficiente`) antes de qualquer mutação |
| Divergência determinística | FR-003/FR-004: ordem estável de `BTreeMap`, `hash_estado` refletivo, sem relógio/aleatoriedade; duas execuções idênticas comparadas |
| Sistema de combate real introduzido por "ataque" | Combate é `combate_ticks` + troca de taxa de regeneração; nenhum projétil, física, animação ou habilidade completa |
| Stack/protocolo alterados | Fronteira explícita: nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, manifesto ou CI; ADR-004 mantém rede bloqueada |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida de automação registrada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) antes de R7 (T012) |
| Afirmar verde sem execução | Gates/build/testes **[NV]** até o orquestrador; nada afirmado como verde sem saída colada |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md` e código intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`expect`/`#[allow]`.
- Confirmar o determinismo com o roteiro de 10 passos e registrar capturas; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004 e **todos os valores de combate rotulados como `[HIPÓTESE]`/protótipo, nunca cânone**; R4 antes de R7 `1.0.0-local`.
