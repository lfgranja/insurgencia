# Validação (R4) — 035-advisories-acr-rastreabilidade

**Data:** 2026-11-22T18:35:00Z · **Resultado**: **PASS (documental) — aprovado para R7** · R4 precede R7

| Verificação | Saída |
|---|---|
| Âncora | PRs #39/#40/#41 (comentários `7082836571`/`7082907820`/`7082916777`, 10 advisories sem blockers), artefatos 034, `STATE.md`, `feature.json`, `.gitignore` e `verificar_manifesto.rs` lidos; nenhum contrato redefinido. |
| Escopo | 14 artefatos SDD escritos; nenhum arquivo de código, nenhuma correção aplicada na fonte neste agente; nenhum `converge.md` de ciclo anterior editado. |
| Correções especificadas | FR-002 `STATE.md` 34/237/34/34/0/13; FR-003 `evidence.md` cinco capturas + converge `17/17`; FR-004 afirmação de `.gitignore` corrigida (regra `/.specify/queue/` preservada); FR-005 CLI imprime `hash_regras` minúsculo e aceita maiúsculo via `eq_ignore_ascii_case`. |
| Roteiro de testes | `.specify/validations/advisories-acr-rastreabilidade/test-roadmap.md` escrito antes do PR: 10 passos, superfície CLI/docs, capturas de terminal/JSON (sem UI), coluna `automatizável?`, rollback e passo explícito `content_hash` maiúsculo → hash minúsculo canônico (passo 4) + determinismo por `diff` (passo 7). |
| Evidência local (observada) | `./verify.sh` em `2026-11-22T18:32:17Z`: `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`; 5 gates: SDD `34/0`, DAG `314 tarefas/17 arestas/0 ciclos`, task-id `0 duplicatas`, converge `17/17`, hub-docs `0 erros`; `check_production_gate.py` ausente → exit 2 = N/A (Bloco 0.a). |
| Evidência remota (observada) | PR #40 (`dev→main`) `OPEN`, head `f9d87e0`, contextos `gates pass` (`45050452465`/`45050458892`) e `rust pass` (4m24s/4m47s). **Não mesclar `main`** — fila SWE. |
| Execução do roteiro e correções | **[NV]** — nenhum build, teste, gate ou aplicação das quatro correções na fonte neste ciclo; a cargo do orquestrador/agente de código (T018). |
| Governança | `analyze.md` READ-ONLY **antes** de `tasks.md`, checklist revisado, `converge.md` append-only com 1ª entrada em `2026-11-22`, R1 go (6/6 strong, 80/20 = 400/625 = 0,64), rollback documentado; SemVer `1.0.0` previsto em R7. Factory `b1deab1` somente fonte; `docs/GDD*`, `.gitignore` (regra preservada), ledger, CI, baselines pré-existentes e 194 paths untracked alheios intactos. |

## R4
Aprovado para R7. A aprovação é documental: baseia-se nas âncoras lidas, na inspeção dos artefatos e nas evidências observadas em `18:32:17Z` e no PR #40 `OPEN`. Nenhum verde é afirmado para o roteiro, para as correções aplicadas, para um `./verify.sh` pós-correção ou para gates reexecutados — todos **[NV]** até o orquestrador. A autoridade continua sendo `conferir` (011/033): aceitar `content_hash` maiúsculo é do contrato; imprimir o hash minúsculo canônico é responsabilidade do adaptador. Rollback documentado em `release.md`, nunca executado.
