# Idea Intake: Fechamento de advisories de ACR e rastreabilidade documental pós-034/041

- **Slug**: advisories-acr-rastreabilidade
- **Created**: 2026-11-22T18:31:15Z
- **Source**: ciclo SDD 035 no branch `035-advisories-acr-rastreabilidade` (base `f9d87e0`); sessão `session_id=20261122T183153Z-acr-advisories-trace`, título `insurgencia - fechamento de advisories ACR e rastreabilidade`. Âncora = PRs #39/#40/#41 (comentários de ACR `7082836571` / `7082907820` / `7082916777`), artefatos do ciclo 034 (`specs/034-gate-manifesto-cli/`, `.specify/validations/gate-manifesto-cli/evidence.md`), `.specify/memory/STATE.md`, `.gitignore` e `crates/sim/examples/verificar_manifesto.rs`.
- **Type**: improvement (governança/rastreabilidade — correção documental e de saída de CLI, sem novo contrato)

## Idea (as captured)

> "Fechar os advisories de ACR dos PRs #39/#40/#41 e a deriva documental pós-034/041: 10 advisories sem blockers (3+4+3) mais quatro divergências observadas nas auditorias de 18:31–18:34Z — `STATE.md` desatualizado, `evidence.md` inconsistente, afirmação de `.gitignore` intacto nos artefatos 034 e o CLI que imprime `manifesto.content_hash` (que pode vir maiúsculo) em vez do hash calculado em minúsculo — registrando tudo em SDD ordenado com rastreabilidade ponta a ponta."

## Restated

Não há novo contrato de produto. O trabalho é (a) consolidar o fechamento dos 10 advisories de ACR já classificados como **sem blockers** e (b) corrigir a deriva documental deixada pelos ciclos 034/041, preservando a autoridade do contrato `conferir` (011/033) e a fronteira de escrita. A saída do CLI passa a imprimir o **hash calculado em minúsculas**, e não o `content_hash` lido do manifesto — este último continua sendo aceito em maiúsculas pela comparação `eq_ignore_ascii_case` do contrato. Toda execução pendente (gates, roteiro, merge) permanece **[NV]**.

## Origin & Context

- **Raised by**: SE/orquestrador (auditorias de fechamento pós-034) · **Trigger**: 10 advisories de ACR abertos nos comentários dos PRs #39/#40/#41 e quatro divergências encontradas entre o que os artefatos afirmam e o que o HEAD contém.
- **Estratégia**: GDD §1 (soberania das regras) e §14.2 (handshake com versão, `module_id` e hash SHA-256); Constituição II (I/O apenas no adaptador); ordem normativa do SDD do hub com `analyze` (READ-ONLY) **antes** de `tasks`, R4 antes de R7.
- **80/20 registrado (Bloco 0.a)**: critério `impacto × confiança × facilidade × segurança / 625`, escala 1–5. Escolhido = **fechar advisories de ACR e deriva documental pós-034/041** = 4×5×4×5 = **400/625 = 0,64**. Alternativas: paridade Rust↔Godot 5×4×2×2 = **80/625**; instalar Godot no ambiente depende de autorização externa. `check_production_gate.py` da factory ausente → exit 2 = **N/A** (não é RED).

## Evidências de partida (observadas — não reexecutadas)

- `2026-11-22T18:32:17Z` — `./verify.sh`: `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`; 5 gates: SDD `34/0`, DAG `314 tarefas/17 arestas/0 ciclos`, task-id `0 duplicatas`, converge `17/17`, hub-docs `0 erros`.
- `2026-11-22` — PR #40 (`dev→main`) `OPEN`, head `f9d87e0`, contextos `gates pass` (runs `45050452465` / `45050458892`) e `rust pass` (4m24s / 4m47s). **Não mesclar `main`** — fila SWE.
- `2026-11-22` — ACRs sem blockers: #39 comentário `7082836571` (3 advisories), #40 comentário `7082907820` (4), #41 comentário `7082916777` (3).
- `2026-11-22T18:31Z–18:34Z` — quatro divergências confirmadas por leitura direta (detalhadas em `research.md`).

## First-Glance Unknowns

- [RESOLVIDO: escopo — 10 advisories (3+4+3) sem blockers, fechados por correção documental e de saída; nenhum reverso de contrato.]
- [RESOLVIDO: deriva `STATE.md` — ``16:59Z`` traz 33/236/33/33 e âncora `specs/033`; HEAD real é 34 specs / 237 assessments / 34 validations / 34 releases / 0 bugs / 13 research e `.specify/feature.json → specs/034`.]
- [RESOLVIDO: deriva `evidence.md` — diz "quatro execuções" com cinco capturas e "converge pendente", embora o converge esteja baselinado (`17/17`).]
- [RESOLVIDO: deriva `.gitignore` — artefatos 034 afirmam "intacto", mas `.gitignore:5-6` contém `/.specify/queue/`.]
- [RESOLVIDO: advisory CLI — `verificar_manifesto.rs:135-136` imprime `manifesto.content_hash` (pode vir maiúsculo) em vez do hash calculado em minúsculo (`hash_regras`).]
- [NV: aplicação das correções e execução do roteiro — nenhum build, teste, gate, commit, push ou PR neste ciclo; nada mesclado em `main`; nenhum verde afirmado além das evidências datadas acima.]
