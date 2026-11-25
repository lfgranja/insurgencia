# Convergência — 058-mirror-vitoria-local

**Data:** 2026-11-24 · **Modo:** READ-ONLY, **append-only** — esta é a **1ª entrada do ciclo 058**; entradas futuras são acrescentadas **abaixo**, sem reescrever este histórico nem modificar `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, código, GDD, constituição, baseline ou artefatos de outros ciclos. **Não-execução:** nenhum build, gate, formatador, teste, captura, commit, push, PR, merge, deploy, issue ou promoção de versão executado nesta escrita; saídas citadas foram **observadas pelo orquestrador/SWE** e registradas em `evidence.md`.

## Evidências revisadas (inspeção, não reexecução)

- **Âncora/escopo** — worktree `.specify/queue/worktrees/impl-058-mirror-vitoria/`, branch `feature/058-mirror-vitoria-local`, `HEAD = 1855e5c` (= `origin/dev`); sessão `insurgencia - mirror vitoria local`, constituição v1.2.2, R1 `go`, score **270/3125** (`5×3×3×2×3`) para BF-003a; BF-003b (12 ilhas) permanece aberto. Decisão e DoD em `.specify/assessments/mirror-vitoria-local/{intake,research,define,shape,decision}.md`, `specs/058-mirror-vitoria-local/spec.md`, `plan.md`, `checklists/requirements.md` e `.specify/memory/STATE.md`. Nenhuma regra canônica alterada; GDD:275 lido como `[HIPÓTESE]`.
- **Cadeia SDD** — `spec.md` (FR-001..FR-004 ↔ SC-001..SC-004 1:1), `tasks.md` (T001..T010, DAG linear, `tasks` T008 antes de `analyze` T009, R4 antes de R7 `1.0.0-local`), `analyze.md` (READ-ONLY, quatro FR consistentes, contradições C1–C4 resolvidas, veredito de prosseguir). R6/taskstoissues é **N/A** sob v1.2.2: PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado.
- **Código em escopo** — `crates/sim/src/lib.rs:564-585`: `POSICOES_NOS_LOCAIS` e `semear_espelho_local` documentados `[HIPÓTESE]`, com 4 ilhas (`1..=4`), 1 súdito (id `1`, `(0.0, 236.0)`) e 3 nós via `adicionar_*`; `hash_estado` reflete a semente. `crates/cliente/src/main.rs:597`: chamada única em `configurar` antes de `insert_resource(Simulacao(sim))`, sem segundo espelho no cliente. `crates/sim/tests/simulacao.rs:4233` (`espelho_local_seed_viabiliza_vitoria_e_reflete_hash`) e `:2558` (`tres_nos_geram_vitoria`, preservado): rota de `propaganda_no` → vitória com resistência zerada.
- **Execução observada (verbatim)** — `cargo test --locked -p insurgencia-sim --test simulacao` → `103 passed (1 suite, 0.28s)`; `cargo test --locked -p insurgencia-servidor --test cli` → `10 passed (1 suite, 0.11s)`; `cargo check --locked -p insurgencia-sim --tests` → `Finished dev profile ... in 14.09s`; `work-order-dag PASS (57 files, 622 tasks, 302 edges, 0 cycles)`; `task-id PASS (57, 0 duplicates)`; `converge PASS (39 baselined/39)`; `hub-docs PASS (0 errors, 0 warnings)`. `check_sdd_completeness` falhava apenas pelo `release.md` ausente, agora escrito. Detalhamento e os 14 passos em `.specify/validations/mirror-vitoria-local/evidence.md`.
- **Fronteira** — nenhuma alteração em contratos, `PROTOCOL_VERSION`, Cargo, GDD, CI, baseline, `AGENTS.md`, `concurrency.jsonl`, `STATE.md` (exceto a nota append-only do ciclo 058), PR #76 ou código alheio. Nenhuma janela, credencial, socket ou rede utilizada.

## Classificação dos achados (gap-types normativos)

Cada item do inventário de intenção (FR-001..FR-004, SC-001..SC-004, T001..T010 e princípios v1.2.2 aplicáveis) foi confrontado com o código e os artefatos em escopo. Tipos possíveis: `missing` (ausente), `partial` (presente, mas incompleto), `contradicts` (conflita com a intenção/constituição), `unrequested` (fora do pedido).

| ID | Gap-type | Severidade | source-ref | Evidência | Trabalho restante |
|:--|:--|:--|:--|:--|:--|
| F1 | `partial` | MEDIUM | FR-002 / SC-002 (HUD), passo 13 | Código da chamada no cliente está presente (`cliente:597`), mas a captura visual de `Ilhas`/`Sinais`/`Nós`/`Resultado` e "Autogestão estabelecida" é **[NV]** (build Bevy > 900 s e sem janela automatizável). Cobertura indireta por FR-003. | SWE/orquestrador executa em X11/GPU e anexa screenshot + carimbo a `evidence.md`. |
| F2 | `partial` | LOW | FR-004 / SC-004, passos 5, 11, 12 | Saídas locais e quatro gates observados; `./verify.sh` completo, `cargo fmt --all -- --check` e os jobs remotos `gates`/`rust` seguem **[NV]**. O gate de converge foi lido antes da criação deste arquivo (baseline `39/39`). | SWE confirma `gates`/`rust` green no head, roda `./verify.sh`/`--check` e inclui este `converge.md` no baseline (`--bootstrap`/ratchet). |
| — | `missing` | — | FR-001, FR-003, FR-004 (código/artefatos) | **Nenhum achado**: helper/constantes, chamada no cliente, teste de regressão, roteiro, validação, release e esta entrada existem e foram inspecionados. | Nenhuma tarefa de código. |
| — | `contradicts` | — | FR-001..FR-004, constituição v1.2.2 | **Nenhum achado**: nenhuma regra canônica promovida, nenhum estado duplicado, nenhum R6/issue simulada, nenhum merge/deploy, nenhuma reescrita de histórico. | Nenhuma tarefa de código. |
| — | `unrequested` | — | spec/plan/tasks 058 | **Nenhum achado**: extensão restrita aos 4 arquivos de código do diff e aos 5 artefatos desta fronteira; sem mapa de 12 ilhas, RNG/relógio, protocolo, Cargo ou CI novos. | Nenhuma tarefa de código. |

**Totais**: 4 FR + 4 SC + T001..T010 + princípios v1.2.2 verificados; 0 `missing`, 2 `partial` (ambos de **evidência**, não de comportamento de código), 0 `contradicts`, 0 `unrequested`; severidade máxima MEDIUM. Nenhuma tarefa de correção é anexada a `tasks.md`: o código inspecionado satisfaz a intenção e as duas pendências são execuções **[NV]** de responsabilidade do SWE.

## Convergências

1. **Semente → vitória → hash**: os três nós de `POSICOES_NOS_LOCAIS` coincidem com a rota existente de `propaganda_no`; `tres_nos_geram_vitoria` não foi alterado e o teste novo prova contagens 4/1/3 e mudança de `hash_estado` sem RNG, relógio ou rede.
2. **Fonte única**: a sim continua autoritativa; o cliente apenas chama o helper uma vez em `configurar`, sem espelho paralelo, sem nova regra de validação e sem impacto em contrato, protocolo, Cargo ou CI.
3. **Recorte BF-003a/BF-003b**: a vitória local mínima é entregue sem decidir topologia/spawning/rotas de 12 ilhas; nenhum valor `[HIPÓTESE]` de GDD:275 é promovido a cânone.
4. **Governança**: ordem `tasks → analyze (READ-ONLY) → implement → converge (append-only) → R4 → R7 1.0.0-local` preservada; R6 **N/A**, PR #76 intocado, promoção `dev→main` e deploy SWE-exclusivos; rollback = revert do único commit de feature, documentado e não executado.

## Veredito

**CONVERGIDO no escopo inspecionável**: a implementação satisfaz spec, plano e tarefas, com 0 achados `missing`/`contradicts`/`unrequested`. A aprovação de R4 permanece **CONDICIONADA** aos dois itens **[NV]** (jobs remotos `gates`/`rust` green e captura visual do HUD); a release `1.0.0-local` vem **depois** de R4 e é estritamente local. Se a execução ao vivo revelar divergência, corrigir a **fonte**, nunca a expectativa do teste, e registrar nova entrada append-only abaixo — sem reescrever esta.
