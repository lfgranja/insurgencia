# Implementation Plan — 059-mover-validado

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; `converge` é **append-only**; R1 `go`; R4 (validate) **condicionado ao roteiro de verificação** e antes de R7 (release `1.0.0-local`). **R6/taskstoissues é N/A/bloqueado** por v1.2.2: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. Promoção `dev→main` e deploy são **SWE-exclusivos**; PR #76 pertence a outro owner ativo e **não é tocado**.

**Cadeia executável exigida pelo contrato**: `tasks → analyze → implement → converge → validate → release` (respectivamente 10 → 11 → 12 → 13 → 27 → 26), com **R6 N/A** em toda a cadeia.

**Fronteira explícita**: este agente escreve **somente** `specs/059-mover-validado/{spec.md, plan.md, checklists/requirements.md}` e `.specify/feature.json` no worktree `.specify/queue/worktrees/cycle-059/` (branch `feature/059-mover-validado`, base `origin/dev = 737b876`). Não edita `analyze.md`, `tasks.md`, `converge.md`, assessment, validação, release, `crates/`, `Cargo.toml`, GDD, manifesto, CI, `STATE.md`, `AGENTS.md`, `concurrency.jsonl` ou código-fonte. Não executa gates, build, testes, formatadores, commit, push, PR, merge ou captura — tudo permanece **[NV]** até o orquestrador.

**Rotulagem obrigatória — escala de conversão**: `PIXELS_POR_METRO = 100.0` é **`[HIPÓTESE]`/protótipo da sessão**, nunca cânone; `VELOCIDADE = 260.0` px/s ÷ 100 = **2,6 m/s** e, a 30 Hz (`PASSO_SEGUNDOS = 1/30`), ≈ **0,087 m/tick**, abaixo de `LIMITE_MOVIMENTO_M = 1.0`. Os helpers puros `pixels_para_metros`/`metros_para_pixels` são o único ponto de conversão; todo destino/alvo de ação opera em metros via `pixels_para_metros`. **Nenhum** número é promovido a regra canônica e o GDD é lido, nunca reescrito. BF-012 fecha; BF-003b (mapa de 12 ilhas) e BF-013 (`contratos` consumido por binários) permanecem abertos.

**Reordenação dos sistemas (parte do contrato)**: a cadeia `Update` passa a ser `mover_jogador` → todos os `solicitar_*` → `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar` → `atualizar_texto`. `mover_jogador` valida antes de agendar e só avança `ComandoLocal.seq`/`request_id` no aceite; `sincronizar_sim` lê `Mundo.jogadores[PEER_ID].posicao` e escreve `JogadorLocal` em pixels via `metros_para_pixels`, sem injetar pixels no espelho. Assim o avatar desenha o estado autoritativo já aceito e avançado, sem dupla aplicação e sem segunda verdade.

**R3 = 0**: nenhuma clarificação em aberto. Captura visual da janela Bevy, execução externa de gates e comportamento exato na fronteira de 1 m são **limitações de escopo [NV]**, não ambiguidades bloqueantes. **Não há arquivo `clarify.md`**: o registro canônico de R3 é esta etapa 07, com os detalhes em `spec.md` → *Clarificações*.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R1 `go`; **R6/taskstoissues N/A/bloqueado** (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado); `tasks` antes de `analyze`; `converge` append-only; R4 condicionado ao roteiro e antes de R7 `1.0.0-local`; promoção `dev→main` e deploy SWE-exclusivos; PR #76 intocável.
2. **01 Intake**: sessão `insurgencia - mover validado no cliente`; 80/20 = **144/3125** (`mover-validado`, BF-012, `4×3×3×2×2`) contra BF-003b (270/3125 histórico inclui BF-003a fechado; depende de topologia canônica), BF-014 **250/3125** (diagnóstico) e BF-013 **100/3125** (`5×2×5×1×2`, diagnóstico); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵ = 3125; consumidor = jogador do cliente Bevy + validação da sim/CLI; decisão em `.specify/memory/STATE.md:582-592`; artefatos em `.specify/assessments/mover-validado/{intake,research,define,shape,decision}.md`.
3. **02 Research**: `crates/cliente/src/lib.rs` (`VELOCIDADE = 260.0`, `deslocar`, `aproximar_no`, `EstadoJogador.posicao` em pixels); `crates/cliente/src/main.rs` (`ComandoLocal`, `mover_jogador:618-657`, `sincronizar_sim:1117-1129`, `sincronizar_avatar`, `avancar_simulacao`, `.chain()`); `crates/sim/src/lib.rs` (`Comando`, `LIMITE_MOVIMENTO_M = 1.0`, `ValidacaoComandos::validar:2414-2445`, `agendar_comando`); `crates/sim/tests/simulacao.rs` e `crates/servidor/tests/cli.rs`; constituição v1.2.2. **Achado central**: o limite e o validador já existem na sim e são usados pela CLI; o cliente nunca os consulta e injeta pixels no espelho.
4. **03 Define (problema)**: `mover_jogador` grava `JogadorLocal` em pixels e `sincronizar_sim` copia essas coordenadas para `Mundo` como metros, sem `Comando`, sem `validar`, sem `agendar_comando`. Recusas não são observáveis no cliente e `seq`/`request_id` avançam sem aceite. DoD-1..DoD-5 nomeiam comportamento, consumidor, comandos de prova e backlog; todos os números da escala rotulados como protótipo.
5. **04 Shape (concept)**: opção A escolhida — conversão explícita + `Comando { acao: "mover", destino: metros }` validado antes de `agendar_comando` + sincronização espelho→local + reordenação dos sistemas. Rejeitadas: injeção direta de posição no espelho (quebra a autoridade da sim), validação reimplementada no cliente (duas verdades) e HUD/`hash_estado` (BF-014, diagnóstico). BF-003b e BF-013 ficam fora por definição.
6. **05 Decide**: score **144/3125** (`4×3×3×2×2`), **`Veredito: go`** (R1), **R3 = 0**, R6 N/A; DoD 0.9 nomeia consumidor (jogador do cliente Bevy + validação da sim/CLI) e comandos de prova (`cargo test --locked -p insurgencia-cliente --lib`, `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo check --locked -p insurgencia-cliente --all-targets`, `./verify.sh`, jobs `gates`/`rust`, roteiro de 14 passos).
7. **06 Specify**: `spec.md` com US1–US4, **FR-001..FR-004**, **SC-001..SC-004** em mapa 1:1, edge cases, `Rastreabilidade FR ↔ SC`, score 144/3125 e fronteira explícita. FR-001 conversão pixels↔metros; FR-002 `mover_jogador` validado; FR-003 sincronização espelho→local e reordenação; FR-004 evidência/validação/release.
8. **07 Clarify (R3 = 0)**: nenhuma clarificação em aberto e **nenhum arquivo `clarify.md`**. A escala 100 é escolha de protótipo do contrato, não cânone; a validação pertence à sim e o cliente apenas a consulta; a posição local é derivada do espelho; captura visual, testes e checks remotos são **[NV]**; R6/taskstoissues é N/A ≠ dívida executada. Registro nesta etapa e em `spec.md` → *Clarificações*.
9. **08 Plan**: este arquivo; passos 00→26; fronteira explícita; escala de conversão e reordenação dos sistemas; cadeia `tasks → analyze → implement → converge → validate → release`; R3 = 0; R6 N/A; roteiro de verificação e rastreabilidade FR → etapa.
10. **09 Checklist**: um item `[x]` por FR-001..FR-004 em `checklists/requirements.md`. **`[x]` significa revisão de requisitos, não implementação, execução ou verificação ao vivo.**
11. **Revisão do checklist**: 4 US / 4 FR / 4 SC verificáveis por inspeção; rastreabilidade 1:1 FR ↔ SC ↔ etapa; rótulo `[HIPÓTESE]` distinto de cânone; escala e ordem dos sistemas confirmadas; fronteira confirmada; aprovado para `tasks`.
12. **10 Tasks** (`FR-001 → etapa 12`): `tasks.md` com T001..T010, IDs únicos e DAG linear; `tasks` antes de `analyze` (READ-ONLY); R4 antes de R7; conversão, comando `mover` validado, sincronização/reordenação e rastros aparecem como trabalho nomeado, sem reescrever `analyze.md`.
13. **11 Analyze (READ-ONLY)** (`FR-001 → etapa 11`): convergência entre decisão 80/20, código lido, contrato técnico e governança; `analyze.md` lido como insumo, não reescrito; ordem preservada com `tasks` antes de `analyze`.
14. **12 Implement (código Rust)** (`FR-001`, `FR-002`, `FR-003`): em `crates/cliente/src/lib.rs`, `PIXELS_POR_METRO = 100.0` e helpers puros documentados (`#[must_use]`), `aproximar_no` em metros com `hypot`, cast local com `#[expect(clippy::cast_possible_truncation, reason = "...")]` e testes de `--lib`. Em `crates/cliente/src/main.rs`, `mover_jogador` com `Comando { acao: "mover", destino: metros }`, `ValidacaoComandos::validar` antes de `agendar_comando`, contadores só no aceite; todos os destinos/alvos convertidos via `pixels_para_metros`; `sincronizar_sim` lê o espelho e escreve `JogadorLocal` em pixels, sem injeção de pixels no espelho; cadeia reordenada para `mover_jogador` → `solicitar_*` → `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar` → `atualizar_texto`. Sem mudança em sim, contratos, protocolo, Cargo, CI, GDD, regra de validação ou convenção Rust (`forbid(unsafe_code)`, `deny(clippy::pedantic)`). Execução **[NV]** nesta fronteira.
15. **13 Converge (append-only)** (`FR-004 → etapa 13`): entrada datada em `specs/059-mover-validado/converge.md`, nunca reescrita; **[NV]** para execução e capturas. Fora da fronteira deste agente.
16. **27 Validate (R4)** (`FR-002`, `FR-003`, `FR-004 → etapa 27`): validação documental, **condicionada ao roteiro de 14 passos**, aprovando conversão, aceite/recusa do `mover`, sincronização autoritativa e governança; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7. Registro em `.specify/validations/mover-validado/`.
17. **26 Release (R7)** (`FR-004 → etapa 26`): `release.md` em `.specify/releases/mover-validado/` com versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback documentado (nunca executado), somente após R4. **BF-012 fechado; BF-003b e BF-013 abertos.**

> Os números reservados pela constituição para etapas não utilizadas neste ciclo permanecem vagos; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (14 passos)

Escrito antes do PR em `.specify/validations/mover-validado/test-roadmap.md` (fora da fronteira deste agente). Superfície: inspeção de `crates/cliente/src/lib.rs`, `crates/cliente/src/main.rs`, `crates/sim/src/lib.rs` e dos artefatos do ciclo; execução dos testes nomeados, `cargo check`, clippy, fmt, gates determinísticos e jobs remotos somente pelo orquestrador/SWE. Captura visual do avatar/HUD **[NV]** (janela Bevy depende de X11/GPU; compilação pesada excedeu 900 s em tentativa local anterior). Rollback documental: reverter o único commit de feature e corrigir a **fonte**, nunca a expectativa; capturas ficam fora do worktree.

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD origin/dev`, branch e `date -u`; listar artefatos e conferir `tasks` antes de `analyze` | base `737b876` (`origin/dev`), branch `feature/059-mover-validado`, score **144/3125**, BF-012 fechado/BF-003b e BF-013 abertos; `STATE.md`, `AGENTS.md`, `concurrency.jsonl`, assessment, validação, release e paths alheiros intactos; R6 fora da cadeia | sim (inspeção/terminal) |
| 2 | `cargo test --locked -p insurgencia-cliente --lib` | suíte verde; `pixels_para_metros((260.0, 0.0)) == (2.6, 0.0)`, ida/volta dentro de `1e-6`, `aproximar_no` em metros | sim (teste **[NV]** até execução) |
| 3 | `cargo test --locked -p insurgencia-sim --test simulacao` | suíte verde; `movimento_aplica_destino`, `replay_de_seq_e_request_id` e `limite_de_movimento` preservados, validador e limite inalterados | sim (teste **[NV]**) |
| 4 | `cargo test --locked -p insurgencia-servidor --test cli` | suíte verde; paridade com a validação da CLI, formato `<tick>\t<sha256>` inalterado | sim (teste **[NV]**) |
| 5 | `cargo check --locked -p insurgencia-cliente --all-targets` | exit `0`, nenhum erro de tipo/emprestimo após a reordenação e as conversões | sim (**[NV]** até execução) |
| 6 | `./verify.sh` | `ALL STAGES OK`; estágios `check`, `clippy`, `test`, `fmt` sem falha | sim (**[NV]** até execução) |
| 7 | `python3 scripts/ci/check_sdd_completeness.py` | exit `0`, nenhuma lacuna SDD; FR-001..FR-004 e SC-001..SC-004 presentes | sim (**[NV]**) |
| 8 | `python3 scripts/ci/check_work_order_dag.py` | exit `0`, DAG sem ciclo/auto-aresta/órfã; `tasks` antes de `analyze`, R6 ausente da cadeia | sim (**[NV]**) |
| 9 | `python3 scripts/ci/check_task_id_uniqueness.py` | exit `0`, T001..T010 únicos | sim (**[NV]**) |
| 10 | `python3 scripts/ci/check_converge_append_only.py` | exit `0`, `converge.md` acrescentado sem reescrita alheia | sim (**[NV]**) |
| 11 | `python3 scripts/ci/hub_docs_gate.py` | exit `0`, documentos-hub sem deriva introduzida pelo ciclo | sim (**[NV]**) |
| 12 | `cargo clippy --workspace --all-targets --all-features -- -D warnings` e `cargo fmt --all -- --check` | exit `0`; sem avisos e sem diff de formatação | sim (**[NV]**) |
| 13 | Cheques remotos no head do PR via `gh pr checks`/`gh run view` | jobs `gates` e `rust` green no SHA final, com run ID e carimbo registrados | parcial (consulta; execução remota **[NV]**) |
| 14 | Captura visual do avatar/HUD Bevy e rollback documentado: `mover` aceito move o avatar; recusa não; reverter o único commit devolve HEAD a `737b876` | HUD coerente com a posição autoritativa; artefatos do ciclo preservados; nenhuma expectativa afrouxada | **não (local)** — **[NV]** (X11/GPU; instrução de rollback não executada) |

## Rastreabilidade (FR → etapa)

| FR | SC | Etapas do plano | T |
|:--|:--|:--|:--|
| FR-001 conversão determinística pixels↔metros (`PIXELS_POR_METRO = 100.0`, `pixels_para_metros`/`metros_para_pixels`, 2,6 m/s ≈ 0,087 m/tick < 1,0 m; destinos/alvos em metros) | SC-001 | 02 research, 04 shape, 07 clarify, 12 implement | T002, T004, T005, T007, T010 |
| FR-002 `mover_jogador`: `Comando { acao: "mover", destino: metros }`, `ValidacaoComandos::validar` antes de `agendar_comando`, `seq`/`request_id` só no aceite | SC-002 | 03 define, 06 specify, 12 implement | T003, T005, T010 |
| FR-003 `sincronizar_sim` lê `Mundo` e grava `JogadorLocal` em pixels, sem injetar pixels no espelho; `avancar_simulacao` antes de `sincronizar_sim` e `sincronizar_avatar` depois | SC-003 | 03 define, 04 shape, 07 clarify, 12 implement | T003, T004, T005, T007, T010 |
| FR-004 evidência/validação/release: BF-012 fechado, BF-003b/BF-013 abertos, `./verify.sh` (`gates`/`rust`), R4 antes de R7 `1.0.0-local`, R6 N/A | SC-004 | 00 constituição, 06 specify, 27 validate, 26 release | T001, T005, T010 |

Cada FR tem SC correspondente em `spec.md` e etapa nomeada acima. R3 = 0 em etapa 07; R6/taskstoissues N/A em 00, 09 e 10; R4 (27) antes de R7 (26).

## Riscos e mitigação

| Risco | Mitigação |
|:--|:--|
| Escala prototípica promovida a cânone ou GDD reescrito | Rotulação `[HIPÓTESE]` no spec/plano/checklist; GDD lido, nunca alterado; limite e validador intactos |
| Unit mismatch pixels↔metros no espelho | Única conversão em `pixels_para_metros`/`metros_para_pixels`; `sincronizar_sim` lê o espelho, nunca escreve nele pixels |
| Dupla aplicação de movimento / ordem invertida | Cadeia `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`; posição local derivada, não fonte |
| Queima de contador na recusa | `validar` com os contadores candidatos antes do incremento; aceite é a única condição de avanço |
| Destino > 1 m recusado por `MovimentoForaDoLimite` | Escala 100: passo ≈ 0,087 m/tick < 1,0 m; arredondamento na fronteira exata **[NV]** |
| Ordem de cadeia alterada por hábito | `tasks` (10) antes de `analyze` (11, READ-ONLY); `converge` (13) append-only; R4 (27) antes de R7 (26) |
| R6 adotada, issue simulada ou promoção indevida | R6/taskstoissues N/A/bloqueado por v1.2.2; PARE se invocada; nenhum `gh` improvisado; `dev→main` e deploy SWE-exclusivos; PR #76 intocável |
| Verde afirmado sem execução | Gates/build/testes/capturas **[NV]** até o orquestrador; nada afirmado como verde sem saída colada |
| Fronteira estourada | Só 4 artefatos escritos por este agente; nenhum toque em código, assessment, validação, release, `STATE.md`, `AGENTS.md`, `concurrency.jsonl`, contratos, Cargo, CI ou rede |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar `--lib` do cliente, `--test simulacao`, `--test cli`, `cargo check --all-targets`, clippy/fmt e `./verify.sh`; conferir convenção Rust e ausência de `unwrap()`/`.expect()`/`#[allow]`; registrar saídas em `$TMP` fora do worktree.
- Rodar o roteiro de 14 passos; verificar os jobs remotos `gates`/`rust` no SHA final; captura visual do avatar depende de X11/GPU **[NV]**.
- Manter R6/taskstoissues fora da cadeia, a rede bloqueada por ADR-004 e **todos os números rotulados como `[HIPÓTESE]`/protótipo, nunca cânone**; R4 condicionado e antes de R7 `1.0.0-local`.
- Corrigir qualquer divergência na fonte (spec, plano, checklist, tasks ou código), nunca afrouxando expectativas de teste; rollback documentado, nunca executado neste ciclo. BF-003b e BF-013 permanecem abertos; promover `dev→main` e publicar somente por decisão da SE.
