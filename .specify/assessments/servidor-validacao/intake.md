# Intake — Validação do servidor headless (CLI autoritativo)

**Data**: 2026-11-24T10:29Z · **Slug**: `servidor-validacao` · **Sessão**: `20261124T102734Z-servidor-validacao`, `session_title=insurgencia - validacao do servidor headless`.

## Ideia (como capturada)

> Capturar, sem executar nada neste assessment, a fatia 80/20 do ciclo 056: o CLI headless `insurgencia-servidor` passa a aceitar as **13 ações canônicas** de `ACOES_PERMITIDAS` (`crates/sim/src/lib.rs:188-202`) e a chamar `ValidacaoComandos::validar` (`sim:2087`) **antes** de `Mundo::agendar_comando`. A exigência de `destino` deixa de ser uma tabela local de 3 ações no binário e passa a ser derivada do Núcleo por `acao_exige_destino`: `true` para 11 ações, `false` apenas para `sinalizar_sincronia` e `sinalizar_alavanca`. A recusa é determinística — `exit 2`, `stdout` vazia, `stderr` `linha N: <CodigoRecusa>` —, o formato `<tick>\t<sha256>` e o hash inicial com `stdin` vazia permanecem inalterados, e a validação na fronteira **não muta** `ultima_recusa`. Nenhuma mudança de contrato, protocolo, `Cargo`, CI, stack ou rede.

Fonte colada (lida, não reexecutada): `.specify/memory/STATE.md`, entrada `2026-11-24T10:29Z — auditoria, backlog e decisão 80/20 do ciclo 056` (linhas 493-536), em particular a auditoria **[EV]**, o `backlog_de_fechamento`, a decisão 80/20 e a DoD. Nenhuma URL foi buscada; nenhuma fonte externa foi usada. Todo número citado vem dessa decisão e do checkout em `0bc2c25`; saídas de gates, build, testes, sondas, roteiro e CI permanecem **[NV]**.

## Restated

Propõe-se eliminar a divergência entre o que o CLI headless aceita e o que o Núcleo determinístico considera válido, sem criar regra nova: uma única whitelist pública no sim, uma única função de derivação de aridade e uma única chamada de validação antes do agendamento. O ganho esperado é duplo — quem consome o CLI pela `stdin` passa a receber recusas com o **mesmo código** que o Núcleo usaria, e a saída `<tick>\t<sha256>` continua sendo a prova determinística já existente, sem mutações colaterais de estado.

## Contexto do ciclo (observado no contrato da sessão; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/servidor-056/`, branch `feature/056-servidor-validacao`, base `0bc2c25`; sessão `20261124T102734Z-servidor-validacao`. Alvo de especificação `specs/056-servidor-validacao/`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012. Assess `.specify/assessments/servidor-validacao/`, validação `.specify/validations/servidor-validacao/`, release `.specify/releases/servidor-validacao/`.
- Âncora **[EV]** no checkout: `crates/sim/src/lib.rs:188-202` mantém 13 ações em `ACOES_PERMITIDAS` como constante **privada**; `nivel_minimo_para_acao` em `sim:212-220`; `ValidacaoComandos::validar` em `sim:2087-2186` não é chamada por **nenhum** binário; `CodigoRecusa` em `sim:1953-1991`; `ultima_recusa` entra em `hash_estado` (`sim:1916-1947`); `agendar_comando` apenas empilha em `comandos_pendentes` (`sim:764-766`).
- Âncora **[EV]** no servidor: `crates/servidor/src/main.rs:16-24` repete uma whitelist local de **6** ações; `analisar_linha` (`:83-116`) fixa aridade 4/6 e `exige_destino` em apenas 3 ações (`mover`, `coletar_materiais`, `construir_barreira`); `executar` (`:142-166`) agenda todos os comandos sem validar; a emissão `{tick}\t{hash}` ocorre antes de `avancar_tick` (`:168-175`); `main` (`:181-189`) já devolve `ExitCode::from(2)` e escreve a mensagem na `stderr`.
- Riscos de compatibilidade **[EV]** registrados: a fixture `crates/servidor/tests/cli.rs:19` usa `coletar_materiais 1 1 0.5 0.5`, inválida para o Núcleo (o destino deve ser o ponto do jogador); testes de parser com `mover` para `(3.5,-4)` excedem o limite de 1 m do Núcleo e **não** devem ser reescritos; `ultima_recusa` entra no hash, logo validar na fronteira não pode alterar o estado.
- Constituição v1.2.2: R1 `go`; R3 = 0 clarificações; R4 antes de R7 `1.0.0-local`; `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável sem `resolve-tasks.sh`/adaptador, registrada como BF-007. Promoção `dev→main` e deploy são SWE-exclusivos.
- Baseline: **141 testes** determinísticos em `0bc2c25`. O corte prevê **5 novos** nomeados (`todas_as_treze_acoes_do_sim_sao_aceitas_pela_whitelist_do_cli`, `acoes_com_destino_exigem_coordenadas_finitas_e_rejeitam_coordenadas_extras`, `comando_recusado_pela_validacao_retorna_exit_2_sem_stdout_e_com_codigo_na_stderr`, `comandos_validados_preservam_formato_tick_hash_e_determinismo`, `hash_inicial_com_stdin_vazia_permanece_igual_ao_estado_inicial`) → **146**.
- DoD registrada: whitelist única derivada de `ACOES_PERMITIDAS`, `acao_exige_destino` pública no sim, validação no servidor antes do agendamento, recusa `linha N: <CodigoRecusa>` com `exit 2`/`stdout` vazia, formato `<tick>\t<sha256>` e hash inicial preservados; consumidores nomeados: usuário/automação do CLI `insurgencia-servidor` e testes `crates/servidor/tests/cli.rs`; provas: `cargo test -p insurgencia-servidor --test cli`, sondas `printf ... | cargo run -p insurgencia-servidor -- --ticks N`, `./verify.sh` e CI `gates`+`rust`; backlog: **BF-011 passa a `fechado`**.

## Scorecard único do ciclo

| Candidato | I × C × Fch × E⁻¹ × R⁻¹ ÷ 3125 | Resultado | Consumidor |
|:--|:--|:--|:--|
| `servidor-validacao` (BF-011) | 4×5×4×4×4 = **1280/3125** | **1280/3125** | usuário/automação do CLI headless |
| HUD `hash_estado` (BF-014) | 3×5×3×5×5 = 1125/3125 | 1125/3125 | jogador no cliente |
| Cliente `mover` (BF-012) | 5×3×4×2×2 = 240/3125 | 240/3125 | jogador no cliente |
| Contratos consumidos (BF-013) | 5×2×5×1×2 = 100/3125 | 100/3125 | protocolo/servidor |
| Rede (BF-001) | 5×2×5×1×1 = 50/3125 — **inelegível** (autorização ADR-004) | 50/3125 | SWE decide |

Critério normativo da decisão `2026-11-24T10:29Z`: `impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`, escala 1–5, denominador **5⁵ = 3125**. Filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo. Impacto **4** (autoridade da fronteira CLI, sem nova jogabilidade), confiança **5** (whitelist, validador, formato e riscos todos lidos no checkout), fechamento **4** (usuário/automação do CLI e testes nomeados, sem consumidor de UI), esforço⁻¹ **4** (sim + servidor + 5 testes, sem stack nova), risco⁻¹ **4** (risco concentrado em `ultima_recusa`/hash e no replay do lote, sem contrato/protocolo/Cargo/CI/rede). Este é o **único** scorecard do assessment: `research.md`, `shape.md` e `decision.md` repetem os mesmos valores, sem pontuação paralela.

## Incógnitas de primeira vista

- **Nenhuma clarificação bloqueante (R3 = 0)**: o contrato fixa as 13 ações, as 2 exceções de `destino`, a precedência da validação, o formato da recusa e o formato de saída. Não há `[NEEDS CLARIFICATION]` de produto a responder neste assessment.
- Incógnitas operacionais, não de produto, ficam **[NV]** até verificação: nomes definitivos de FR-001..FR-006 / SC-001..SC-006 / T001..T012 em `specs/056-servidor-validacao/` (em construção), rendered da mensagem `CodigoRecusa` na `stderr`, resultado das sondas e conformidade de R4/R7 com gates não executados aqui.

## Restrição de execução

Este agente escreve **somente** os 5 artefatos em `.specify/assessments/servidor-validacao/`. Não executa gates, build, testes, formatadores, `./verify.sh`, sondas com `cargo run`, comandos de CI, git remoto ou GitHub; não faz commit, push ou PR; não toca `crates/sim`, `crates/servidor`, specs, validações, releases, `STATE.md`, `AGENTS.md`, `Cargo.toml`, `.specify/queue/concurrency.jsonl` nem qualquer código-fonte. Tudo o que depende de execução permanece **[NV]**. **R6 não é invocada** — N/A por constituição v1.2.2 (BF-007): PARE se invocada, nunca issue simulada, nunca `gh` improvisado. Promoção `dev→main` e deploy são SWE-exclusivos; este ciclo termina no escalonamento, não em merge em `main`.
