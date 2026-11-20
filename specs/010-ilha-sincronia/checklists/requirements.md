# Checklist de requisitos — 010-ilha-sincronia

**Data:** 2026-11-20T03:10Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação nem execução.

- [x] Spec identifica FR-001..FR-008 e SC-001..SC-004 com critérios binários (14 `PASS` + `OK` + exit 0; `DELTA_EVENTO_MS` + 3 `PASS` + `OK`; 8 por amostra e 24 em 3 s; 5 gates)
- [x] Cada FR rastreia para um caminho literal (`nucleo/`, `modulos/`, `tests/`) ou comando `--headless` verificável
- [x] R1 `go` materializado em `.specify/assessments/ilha-sincronia/decision.md` com data, gates 5/5 e restrições
- [x] Opção escolhida = Task 4 literal; PR único com Tasks 5–6 e mudança de engine registrados como rejeitados
- [x] Impacto 5 e evidência 5 justificados em `.specify/assessments/ilha-sincronia/problem.md`, com CE-3 sem implementação
- [x] Escopo explícito fora do ciclo declarado (`manifesto.json`, `Makefile`, `evidence.csv`, 5v5, vitória da Ilha Central, deploy/remote, R4/R7)
- [x] Janela de 5 s medida pelo servidor e dano por amostra de 1,0 s (não por quadro); regra de produção de 3 jogadores separada do exemplo de 2
- [x] Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) nem de execução observada; itens não verificados marcados **[NV]**
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado
- [x] Fonte da verdade = gate determinístico do alvo; factory apenas fonte; Convergence permanece READ-ONLY
