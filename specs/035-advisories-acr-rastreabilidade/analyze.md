# Analyze (READ-ONLY) — 035-advisories-acr-rastreabilidade

**Data:** 2026-11-22T18:34:30Z · **Escopo:** spec, plano e checklist — nenhuma edição de artefato ou código nesta etapa. **Posição:** executa **antes** de `tasks`.

## Achados

| # | Achado | Severidade | Correção na fonte |
|:--|:--|:--|:--|
| A1 | Risco de redefinir o contrato ao "fechar" advisories, alterando `conferir`/011 | alta | FR-001/FR-005: autoridade permanece `conferir`; só se corrige documentação e a linha de saída do adaptador |
| A2 | Ordem normativa poderia ser invertida, com `tasks` antes de `analyze` | alta | Plano passos 5–6 e `tasks.md`: `analyze` (READ-ONLY) precede `tasks`; R4 precede R7 |
| A3 | Aparente contradição: "aceitar `content_hash` maiúsculo" vs. "imprimir minúsculo" | alta | FR-005 + Clarificações: `conferir` aceita maiúsculas (`eq_ignore_ascii_case`); o adaptador imprime `hash_regras` minúsculo — decisão de saída, não de validação |
| A4 | Quatro divergências poderiam ser "corrigidas" alterando a expectativa em vez da fonte | alta | FR-002..FR-004: corrigir `STATE.md`, `evidence.md` e a afirmação de `.gitignore` em 034 com os valores lidos no HEAD; regra `/.specify/queue/` preservada |
| A5 | Tentação de remover `/.specify/queue/` do `.gitignore` como "correção" | média | FR-004/Clarificações: a regra é legítima (runtime de orquestração); apenas a afirmação falsa é corrigida |
| A6 | `evidence.md` diz "quatro execuções" mas lista cinco capturas; diz "converge pendente" com `17/17` | média | FR-003: registrar cinco execuções/capturas e converge baselinado, preservando o restante do relatório |
| A7 | Saída não canônica do CLI gera `diff` falso em determinismo | alta | FR-005 + FR-007: passo explícito no roteiro — manifesto maiúsculo aceito, hash minúsculo impresso, 3 execuções idênticas por `diff` |
| A8 | Evidências locais/remotas e correções poderiam ser executadas por iniciativa do agente | média | FR-008/Pressupostos: colar `ALL STAGES OK`/41 de 41, 5 gates e PR #40 `OPEN` como observados; **[NV]** para roteiro, correções e gates |

## Convergência

Não há conflito entre spec, plano e checklist após as correções A1–A4 registradas nas fontes. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto. R1 `go` presente com scorecard 6/6 strong e 80/20 = 400/625 = 0,64. As evidências de `18:32:17Z` e o PR #40 `OPEN` são suficientes e não devem ser reexecutados; nenhum merge em `main`. R4 precede R7; release `1.0.0` com rollback documentado (nunca executado). Nenhuma mudança de código ou documento-fonte foi feita nesta etapa (READ-ONLY) — este agente escreve somente os artefatos SDD. As correções na fonte e a execução do roteiro são responsabilidade do orquestrador/agente de código e permanecem **[NV]**.

## Veredito

Prosseguir para `tasks` e, depois, para a redação do roteiro e dos artefatos de validação/release, com A1–A8 tratados como requisitos de governança e fronteira. Factory `b1deab1` permanece somente fonte; `docs/GDD*`, `.gitignore` (regra preservada), `STATE.md` (a corrigir pelo dono), ledger, CI, baselines pré-existentes e 194 paths untracked alheios intactos por este agente. `./verify.sh` e o roteiro permanecem **[NV]** até execução observada. PASS.
