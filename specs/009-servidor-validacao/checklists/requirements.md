# Checklist de requisitos — 009-servidor-validacao

**Data:** 2026-11-20T03:00Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação nem execução.

- [x] Spec identifica FR-001..FR-009 e SC-001..SC-005 com critérios binários (contagens de `PASS`, status 124, 20 snapshots, 5 gates)
- [x] Cada FR rastreia para um caminho literal (`nucleo/`, `tests/`) ou comando `--headless` verificável
- [x] R1 `go` materializado em `.specify/assessments/servidor-validacao/decision.md` com data, gates de entrada e restrições
- [x] Opção escolhida = Tasks 2–3 literais; Rust/Bevy e PR único Tasks 2–6 registrados como rejeitados
- [x] Escopo explícito fora do ciclo declarado (Ilha, manifesto, `Makefile`, `evidence.csv`, deploy/remote, factory)
- [x] Nenhuma alegação de cobertura integral da árvore canônica nem de execução observada; itens não verificados marcados **[NV]**
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado
- [x] Fonte da verdade = gate determinístico do alvo; factory apenas fonte
