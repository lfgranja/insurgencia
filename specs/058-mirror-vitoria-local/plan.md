# Implementation Plan — 058-mirror-vitoria-local

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; `converge` é **append-only**; R1 `go`; R4 (validate) **condicionado ao roteiro de verificação** e antes de R7 (release `1.0.0-local`). **R6/taskstoissues é N/A/bloqueado** por v1.2.2: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. Promoção `dev→main` e deploy são **SWE-exclusivos**; PR #76 pertence a outro owner ativo e não é tocado.

**Cadeia executável exigida pelo contrato**: `tasks → analyze → implement → converge → validate → release` (respectivamente 10 → 11 → 12 → 13 → 27 → 26), com **R6 N/A** em toda a cadeia.

**Fronteira explícita**: este agente escreve **somente** `specs/058-mirror-vitoria-local/{spec.md, plan.md, checklists/requirements.md}` e `.specify/feature.json` no worktree `.specify/queue/worktrees/impl-058-mirror-vitoria/` (branch `feature/058-mirror-vitoria-local`, base `origin/dev = 1855e5c`). Não edita `analyze.md`, `tasks.md`, `converge.md`, assessment, validação, release, `crates/`, `Cargo.toml`, GDD, manifesto, CI, `STATE.md`, `AGENTS.md`, `concurrency.jsonl` ou código-fonte. Não executa gates, build, testes, formatadores, commit, push, PR, merge ou captura — tudo permanece **[NV]** até o orquestrador.

**Rotulagem obrigatória**: a âncora é GDD:275 (3 Nós de Controle, 100 de Resistência Ideológica, −30 por Nó e pulso final de −10), precedida de `[HIPÓTESE]`. **Todos os números do corte são protótipo**: ids `1..=4`, súdito `1` em `(0.0, 236.0)`, `POSICOES_NOS_LOCAIS` (`(0.5, 0.0)`, `(1.5, 0.0)`, `(2.5, 0.0)`), `TICKS_PROPAGANDA_NO = 600` (20 s × 30 Hz), `RESISTENCIA_INICIAL = 100`, `CONTRIBUICAO_NO = 30`, `PISO_DANO_COM_NOS = 10` e `TOTAL_NOS = 3`. Nenhum valor é cânone; o GDD é lido, nunca reescrito. BF-003a fecha; BF-003b (mapa completo de 12 ilhas) permanece aberto.

**R3 = 0**: nenhuma clarificação em aberto. As incógnitas registradas em `.specify/assessments/mirror-vitoria-local/intake.md` (posições da semente, efeito visual da HUD, execução externa) são **limitações de escopo**, não ambiguidades bloqueantes. **Não há arquivo `clarify.md`**: o registro canônico de R3 é esta etapa 07, com os detalhes em `spec.md` → *Clarificações*.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R1 `go`; **R6/taskstoissues N/A/bloqueado** (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado); `tasks` antes de `analyze`; `converge` append-only; R4 condicionado ao roteiro e antes de R7 `1.0.0-local`; promoção `dev→main` e deploy SWE-exclusivos.
2. **01 Intake**: sessão `insurgencia - mirror vitoria local`; 80/20 = **270/3125** (`mirror-vitoria-local`, BF-003a, `5×3×3×2×3`) contra semeadura duplicada no cliente **120/3125** (`5×3×2×2×2`) e HUD/`hash_estado` sem semear **75/3125** (`1×3×1×5×5`); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵ = 3125; consumidor = jogador do cliente Bevy + testes de integração da sim; pendência BF-003a; artefatos em `.specify/assessments/mirror-vitoria-local/{intake,research}.md`.
3. **02 Research**: GDD:275 (regra de vitória e rótulo `[HIPÓTESE]`); `crates/sim/src/lib.rs` (`Mundo::new`, `adicionar_ilha`, `adicionar_sudito`, `adicionar_no`, `avancar_tick`, `propaganda_no`, `hash_estado`, constantes de vitória); `crates/cliente/src/main.rs` (`configurar`, `Simulacao`, renderização de ilhas e HUD); `crates/sim/tests/simulacao.rs` (`posicoes_nos()`, `tres_nos_geram_vitoria`); `.specify/memory/constitution.md` v1.2.2. **Achado central**: o estado é que falta, não a regra — a vitória já é implementada e testada na sim, mas o espelho do cliente não contém ilhas/súditos/nós e a HUD informa `Nós: 0/3`.
4. **03 Define (problema)**: `configurar` monta o espelho com jogadores e PNJs, mas não com ilhas/súditos/nós; `propaganda_no` não encontra Nó para converter e `hash_estado` não reflete a semente. Sem o elo sim↔cliente, a vitória local é inalcançável e os critérios observáveis de BF-003a não existem. Critérios por FR/SC, todos os números rotulados como protótipo.
5. **04 Shape (concept)**: três opções no scorecard único; recomendação = **helper público na sim + chamada no cliente** (270/3125), reaproveitando `adicionar_*`, `BTreeMap`, `propaganda_no`, `hash_estado`, `configurar` e `Simulacao`, sem stack, mapa, contrato, protocolo ou janela nova. Rejeitadas: duplicação da semeadura no cliente (120/3125, verdade do estado dividida) e HUD/`hash_estado` sem semear (75/3125, não torna a vitória alcançável). BF-003b (12 ilhas) fica fora por definição.
6. **05 Decide**: score **270/3125**, `Veredito: go` (R1), **R3 = 0**, R6 N/A; DoD 0.9 nomeia consumidor (jogador do cliente Bevy + testes de integração da sim) e comandos de prova (`cargo test --locked -p insurgencia-sim --test simulacao espelho_local_seed_viabiliza_vitoria_e_reflete_hash -- --exact`, `tres_nos_geram_vitoria`, `./verify.sh`, jobs `gates`/`rust`, roteiro de 14 passos).
7. **06 Specify**: `spec.md` com US1–US4, **FR-001..FR-004**, **SC-001..SC-004** em mapa 1:1, edge cases, `Rastreabilidade FR ↔ SC`, score 270/3125 e fronteira explícita. FR-001 helper/constantes; FR-002 chamada em `configurar`; FR-003 teste de regressão; FR-004 evidência/validação/release.
8. **07 Clarify (R3 = 0)**: nenhuma clarificação em aberto e **nenhum arquivo `clarify.md`**. Os números da semente são escolha de protótipo do contrato, não cânone; a vitória usa a rota existente de `propaganda_no`; a única fonte da semente é a sim e o cliente apenas chama o helper; captura visual da HUD e checks remotos são **[NV]**; R6/taskstoissues é N/A ≠ dívida executada. Registro nesta etapa e em `spec.md` → *Clarificações*.
9. **08 Plan**: este arquivo; passos 00→26; fronteira explícita; cadeia `tasks → analyze → implement → converge → validate → release`; R3 = 0; R6 N/A; roteiro de verificação e rastreabilidade FR → etapa.
10. **09 Checklist**: um item `[x]` por FR-001..FR-004 em `checklists/requirements.md`. **`[x]` significa revisão de requisitos, não implementação, execução ou verificação ao vivo.**
11. **Revisão do checklist**: 4 US / 4 FR / 4 SC verificáveis por inspeção; rastreabilidade 1:1 FR ↔ SC ↔ etapa; rótulos `[HIPÓTESE]` distintos de cânone; fronteira confirmada; aprovado para `tasks`.
12. **10 Tasks** (`FR-001 → etapa 12`): `tasks.md` com T-IDs únicos e DAG linear; `tasks` antes de `analyze` (READ-ONLY); R4 antes de R7; o helper `semear_espelho_local`, a chamada em `configurar`, o teste `espelho_local_seed_viabiliza_vitoria_e_reflete_hash` e os artefatos de evidência aparecem como trabalho nomeado, sem reescrever `analyze.md`.
13. **11 Analyze (READ-ONLY)** (`FR-001 → etapa 11`): convergência entre decisão 80/20, GDD:275, contrato técnico e governança; `analyze.md` lido como insumo, não reescrito; ordem preservada com `tasks` antes de `analyze`.
14. **12 Implement (código Rust)** (`FR-001`, `FR-002`, `FR-003`): `semear_espelho_local(&mut Mundo)` e `POSICOES_NOS_LOCAIS` em `crates/sim/src/lib.rs` (4 ilhas `1..=4`, súdito `1` em `(0.0, 236.0)`, 3 nós, via `adicionar_*`, com `///` e `[HIPÓTESE]`); chamada `insurgencia_sim::semear_espelho_local(&mut sim)` em `configurar` (`crates/cliente/src/main.rs`) após `adicionar_pnj` e antes de `insert_resource(Simulacao(sim))`; teste `espelho_local_seed_viabiliza_vitoria_e_reflete_hash` em `crates/sim/tests/simulacao.rs`. Sem mudança em contratos, protocolo, Cargo, CI, GDD, regra de vitória ou HUD. Convenção Rust v1.2.2 respeitada. Execução **[NV]** nesta fronteira.
15. **13 Converge (append-only)** (`FR-004 → etapa 13`): entrada datada em `specs/058-mirror-vitoria-local/converge.md`, nunca reescrita; **[NV]** para execução e capturas. Fora da fronteira deste agente.
16. **27 Validate (R4)** (`FR-003`, `FR-004 → etapa 27`): validação documental, **condicionada ao roteiro de 14 passos**, aprovando semente, vitória por `propaganda_no`, `hash_estado` alterado e governança; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7. Registro em `.specify/validations/mirror-vitoria-local/`.
17. **26 Release (R7)** (`FR-004 → etapa 26`): `release.md` em `.specify/releases/mirror-vitoria-local/` com versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback documentado (nunca executado), somente após R4. **BF-003a fechado, BF-003b aberto.**

> Os números reservados pela constituição para etapas não utilizadas neste ciclo permanecem vagos; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (14 passos)

Escrito antes do PR em `.specify/validations/mirror-vitoria-local/test-roadmap.md` (fora da fronteira deste agente). Superfície: inspeção de `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs`, `crates/sim/tests/simulacao.rs` e dos artefatos do ciclo; execução do teste nomeado, de `tres_nos_geram_vitoria`, da suíte (`./verify.sh`) e dos jobs remotos `gates`/`rust` somente pelo orquestrador/SWE. Captura visual da HUD **[NV]** (compilação do Bevy excedeu 900 s em tentativa local anterior; sem janela automatizável). Rollback documental: reverter o único commit de feature e corrigir a **fonte**, nunca a expectativa; `rm -rf "$TMP"` remove capturas fora do worktree.

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD origin/dev`, branch e `date -u`; listar artefatos e conferir a ordem `tasks` antes de `analyze` | base `1855e5c` (`origin/dev`), branch `feature/058-mirror-vitoria-local`, score **270/3125**, BF-003a fechado/BF-003b aberto; `STATE.md`, `AGENTS.md`, `concurrency.jsonl`, assessment, validação, release e paths alheiros intactos; R6 fora da cadeia | sim (inspeção/terminal) |
| 2 | `cargo test --locked -p insurgencia-sim --test simulacao espelho_local_seed_viabiliza_vitoria_e_reflete_hash -- --exact` | `1 passed; 0 failed`; 4 ilhas (`1..=4`), 1 súdito (id `1` em `(0.0, 236.0)`), 3 nós, `hash_estado` diferente da baseline e determinístico | sim (teste **[NV]** até execução) |
| 3 | `cargo test --locked -p insurgencia-sim --test simulacao tres_nos_geram_vitoria -- --exact` | `1 passed; 0 failed`; 3 nós convertidos por `propaganda_no`, resistência `0`, `ResultadoPartida::Vitoria`, estável após ticks extras | sim (teste **[NV]** até execução) |
| 4 | `cargo test --locked -p insurgencia-servidor --test cli` | suíte verde, nenhuma regressão no consumidor servidor, protocolo inalterado | sim (teste **[NV]**) |
| 5 | `./verify.sh` | `ALL STAGES OK`; estágios `check`, `clippy`, `test`, `fmt` sem falha | sim (**[NV]** até execução) |
| 6 | `python3 scripts/ci/check_sdd_completeness.py` | exit `0`, nenhuma lacuna SDD; FR-001..FR-004 e SC-001..SC-004 presentes | sim (**[NV]**) |
| 7 | `python3 scripts/ci/check_work_order_dag.py` | exit `0`, DAG sem ciclo/auto-aresta/órfã; R6 ausente da cadeia | sim (**[NV]**) |
| 8 | `python3 scripts/ci/check_task_id_uniqueness.py` | exit `0`, T-IDs únicos | sim (**[NV]**) |
| 9 | `python3 scripts/ci/check_converge_append_only.py` | exit `0`, `converge.md` acrescentado sem reescrita alheia | sim (**[NV]**) |
| 10 | `python3 scripts/ci/hub_docs_gate.py` | exit `0`, documentos-hub sem deriva introduzida pelo ciclo | sim (**[NV]**) |
| 11 | `cargo fmt --all -- --check` | exit `0`, nenhum diff de formatação | sim (**[NV]**) |
| 12 | Checks remotos no head do PR via `gh pr checks`/`gh run view` | jobs `gates` e `rust` green no SHA final, com run ID e carimbo registrados | parcial (consulta; execução remota **[NV]**) |
| 13 | Captura visual do HUD Bevy: semente e `Nós: 3/3`, depois "Autogestão estabelecida" | HUD coerente com a semente e a vitória; screenshot/carimbo em `.specify/validations/mirror-vitoria-local/evidence.md` | **não (local)** — **[NV]** |
| 14 | Rollback documentado: reverter o único commit de feature, devolvendo HEAD a `1855e5c` | HEAD = base, testes/gates no baseline, artefatos do ciclo preservados | sim (instrução; não executada por este agente) |

## Rastreabilidade (FR → etapa)

| FR | Etapas do plano | Verificação |
|:--|:--|:--|
| FR-001 helper `semear_espelho_local` + `POSICOES_NOS_LOCAIS` (4 ilhas, 1 súdito, 3 nós) | 02 research, 03 define, 04 shape, 06 specify, 11 analyze, 12 implement | SC-001; passos 1, 2 do roteiro |
| FR-002 chamada em `configurar` (`Simulacao` semeada no tick 0) | 03 define, 04 shape, 06 specify, 12 implement | SC-002; passos 1, 13 do roteiro |
| FR-003 teste `espelho_local_seed_viabiliza_vitoria_e_reflete_hash` (vitória + hash) | 05 decide, 06 specify, 12 implement, 27 validate | SC-003; passos 2, 3, 5 do roteiro |
| FR-004 evidência/validação/release (BF-003a fechado, BF-003b aberto, R6 N/A) | 01 intake, 09 checklist, 13 converge, 27 validate, 26 release | SC-004; passos 1, 5–14 do roteiro |

Cada FR tem SC correspondente em `spec.md` e etapa nomeada acima. R3 = 0 em etapa 07; R6/taskstoissues N/A em 00, 09 e 10; R4 (27) antes de R7 (26).

## Riscos e mitigação

| Risco | Mitigação |
|:--|:--|
| Hipótese promovida a cânone ou GDD reescrito | Rotulação `[HIPÓTESE]` no spec/plano/checklist; GDD:275 lido, nunca alterado; regra de vitória intacta |
| Semente duplicada no cliente (duas verdades) | Único helper na sim; cliente só chama `semear_espelho_local`; opção B rejeitada (120/3125) |
| Semeadura tratada como mapa de 12 ilhas | BF-003a é só o estado mínimo; BF-003b permanece aberto; nenhuma topologia/spawning/rota é inferida |
| Semente não refletida no hash | `hash_estado` compara mundo com/sem semente no teste; dois replays idênticos devem dar o mesmo hash; passo 2 |
| Vitória não alcançável no cliente | Mesma rota de `tres_nos_geram_vitoria`: 3 nós em `POSICOES_NOS_LOCAIS`, `propaganda_no`, `TICKS_PROPAGANDA_NO + 1`; passo 3 |
| Ordem invertida por hábito | `tasks` (10) antes de `analyze` (11, READ-ONLY); `converge` (13) append-only; R4 (27) antes de R7 (26) |
| R6 adotada, issue simulada ou promoção indevida | R6/taskstoissues N/A/bloqueado por v1.2.2; PARE se invocada; nenhum `gh` improvisado; `dev→main` e deploy SWE-exclusivos; PR #76 intocável |
| Verde afirmado sem execução | Gates/build/testes/capturas **[NV]** até o orquestrador; nada afirmado como verde sem saída colada |
| Fronteira estourada | Só 4 artefatos escritos por este agente; nenhum toque em código, assessment, validação, release, `STATE.md`, `AGENTS.md`, `concurrency.jsonl`, contratos, Cargo, CI ou rede |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar `./verify.sh` e o teste nomeado com `--exact`, além de `tres_nos_geram_vitoria` e da suíte CLI; conferir convenção Rust e ausência de `unwrap()`/`.expect()`/`#[allow]`; registrar saídas em `$TMP` fora do worktree.
- Rodar o roteiro de 14 passos; verificar os jobs remotos `gates`/`rust` no SHA final; captura visual da HUD depende de máquina com X11/GPU **[NV]**.
- Manter R6/taskstoissues fora da cadeia, a rede bloqueada por ADR-004 e **todos os números rotulados como `[HIPÓTESE]`/protótipo, nunca cânone**; R4 condicionado e antes de R7 `1.0.0-local`.
- Corrigir qualquer divergência na fonte (spec, plano, checklist, tasks ou código), nunca afrouxando expectativas de teste; rollback documentado, nunca executado neste ciclo. BF-003b permanece aberto; promover `dev→main` e publicar somente por decisão da SE.
