# Convergência — 035-advisories-acr-rastreabilidade

**Data**: 2026-11-22 · **Modo**: READ-ONLY, append-only. Primeira entrada do ciclo 035.

## Evidências (observadas — leitura de fontes locais e relatórios do orquestrador)

- `2026-11-22` — 10 advisories de ACR nos PRs #39 (comentário `7082836571`, 3), #40 (`7082907820`, 4) e #41 (`7082916777`, 3), todos **sem blockers**. Fechamento por correção na fonte e rastreabilidade; nenhum reverso de contrato (011/033).
- `2026-11-22T18:31Z–18:34Z` — quatro divergências confirmadas por leitura direta: (1) `STATE.md` registra 33/236/33/33 e âncora `specs/033`, contra 34 specs / 237 assessments / 34 validations / 34 releases / 0 bugs / 13 research no HEAD e `feature.json → specs/035` (ciclo 034 registrado); (2) `evidence.md` diz "quatro execuções" com cinco capturas e "converge pendente" com converge `17/17`; (3) artefatos 034 afirmam `.gitignore` intacto, mas `.gitignore:5-6` contém `/.specify/queue/`; (4) `verificar_manifesto.rs:135-136` imprime `manifesto.content_hash` (possivelmente maiúsculo) em vez do hash calculado minúsculo.
- `2026-11-22T18:32:17Z` — `./verify.sh`: `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`. Gates: SDD `34/0`, DAG `314 tarefas/17 arestas/0 ciclos`, task-id `0 duplicatas`, converge `17/17`, hub-docs `0 erros`. `check_production_gate.py` da factory ausente → exit 2 = N/A (Bloco 0.a), não RED — reportado, não reexecutado.
- `2026-11-22` — PR #40 (`dev→main`) `OPEN`, head `f9d87e0`, contextos `gates pass` (runs `45050452465`/`45050458892`) e `rust pass` (4m24s/4m47s). **Não mesclar `main`** — fila SWE.
- `2026-11-22` — 80/20 (Bloco 0.a): escolhido fechar advisories + deriva = 4×5×4×5 = 400/625 = 0,64; paridade Rust↔Godot = 80/625; instalar Godot depende de autorização externa. Factory `b1deab1` (origin/dev `8ae9f52`) é **apenas fonte**; nada transplantado. 194 paths untracked alheios, `docs/GDD*`, ledger, CI e baselines pré-existentes intactos.

## Atendimento
FR-001..FR-008 e SC-001..SC-005 atendidos na **escrita dos artefatos SDD**: os 10 advisories rastreados, as quatro divergências especificadas para correção na fonte (STATE.md, evidence.md, afirmação de `.gitignore` em 034 e linha de saída do CLI) e o roteiro de testes com passo explícito para `content_hash` maiúsculo → hash minúsculo canônico. A autoridade permanece `conferir` (011/033): aceitar maiúsculas é do contrato, imprimir minúsculas é do adaptador. SDD ordenado R1 go (6/6 strong) → `analyze` READ-ONLY antes de `tasks` → `test-roadmap` antes do PR → `converge` append-only → R4 aprovado antes de R7 `1.0.0`; rollback documentado. Nenhum build, teste, gate, commit, push ou PR executado; nenhum merge em `main`. As correções na fonte e a execução do roteiro são responsabilidade do orquestrador/agente de código (T018) e permanecem **[NV]** até execução observada. Este agente não editou `converge.md` de ciclos anteriores.

## Atualização append-only (reservada ao orquestrador)
Nenhuma entrada adicional nesta entrega. Resultados do roteiro, `./verify.sh` pós-correção, gates reexecutados, aplicação das quatro correções na fonte e eventual decisão sobre PR #40 devem ser **anexados** aqui pelo orquestrador, sem reescrever o prefixo acima.

### Execução pelo orquestrador — 2026-11-22T18:59Z

- **Gate verde**: `./verify.sh` (18:55:09Z) → `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`. Tentativa às 18:39:50Z estourou 900 s por `cargo` concorrente — não classificada como RED.
- **Roteiro 100% autônomo**: 10/10 PASS (18:59:00–03Z); capturas em `/tmp/tmp.FAhaahvF3D/capturas/`; relatório em `.specify/validations/advisories-acr-rastreabilidade/evidence.md`. Caso exigido: `content_hash` maiúsculo aceito e hash minúsculo canônico impresso, com `diff` vazio em três execuções.
- **Correções na fonte confirmadas**: `STATE.md` com 34/237/34/34/0/13 e âncora coerente; `evidence.md` com cinco capturas e converge `17/17`; `.gitignore` preservado e reconhecido nos artefatos 034; saída do CLI canônica. `lsp diagnostics` indisponível (sem `rust-analyzer`); clippy permanece a autoridade.
- **Gates Python**: SDD `35/0`, DAG `332 tarefas/34 arestas/0 ciclos`, task-id `0 duplicatas`, hub-docs `0 erros`. Converge baselinado por bootstrap após esta entrada; PR #40 permanece `OPEN` na fila SWE.
