# Checklist de requisitos — 012-evidencia-make-check

**Data:** 2026-11-20T03:19Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação nem execução de Godot/gates, nem escrita de `Makefile`, `evidence.csv` ou artefatos R4/R7.

- [x] Spec identifica FR-001..FR-006 e SC-001..SC-005 com critérios binários (`make check` com sequência de `OK` + `COMPAT_OK` e exit 0; cabeçalho literal de 9 colunas; `grep -c` do marcador `≥ 1`; `test -f` do registro datado; 5 gates PASS)
- [x] Cada FR rastreia para um caminho literal (`Makefile`, `docs/progress/evidence.csv`, `docs/progress/2026-11-19-ciclo-80-20.md`) ou comando `make`/`grep -c`/`test -f` verificável
- [x] R1 `go` materializado em `.specify/assessments/evidencia-make-check/decision.md` com data, HEAD `e2e8894`, gate 5/5 PASS e restrições; branch `feature/012-evidencia-make-check`, `.specify/feature.json` → `specs/012-evidencia-make-check`
- [x] Opção escolhida = Task 6 literal; PR único com Tasks 4–6, gate remoto com lobby/worker/release e evidência otimista registrados como rejeitados
- [x] Impacto 5 e evidência 5 justificados em `.specify/assessments/evidencia-make-check/problem.md`, com CE-5 ainda sem artefatos de evidência
- [x] Cabeçalho de 9 colunas literal e na ordem `id,criterio,verdict,source,command,result,confidence,unverified,recorded_at`, com ids `EV-20261119-001`..`EV-20261119-007` ligados a CE-1..CE-5
- [x] Regra do não verificado explicitada: linha começa `NAO_VERIFICADO`, `result`/`recorded_at` vazios, `unverified` não vazio, e `confidence` só muda após saída observada; loopbacks seguem **[NV]**
- [x] Registro datado `docs/progress/2026-11-19-ciclo-80-20.md` com estado `ilha_concluida`, fora do escopo GDD §16.3/§16.4 e marco de reavaliação após `make check`
- [x] Escopo explícito fora do ciclo declarado (R4/R7, 5v5, Ilha, vitória Central, lobby/worker, release, deploy/remote, CI, factory, baselines)
- [x] Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) nem de execução não observada; itens não verificados marcados **[NV]**
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado
- [x] Fonte da verdade = gate determinístico do alvo; saídas pertencem a R4 e versão/changelog/rollback a R7, sem artefatos escritos aqui; Convergence T010 permanece READ-ONLY
