# Decision — Evidência auditável do gate `make check` e registro do não verificado

**Data:** 2026-11-20T03:19Z · **R1:** `go`

## Veredito
**go** — problema válido (impacto 5, evidência 5), opção recomendada = Opção 1 (Task 6 literal do plano).

## Gate de entrada
- Gate determinístico do alvo: 5/5 PASS observado nesta sessão; o STATE prévio (`2026-11-20T03:08:52Z`) regista `spec_dirs=9`, `tasks=222`, `cycles=0`. `pytest -q` N/A (exit 5) e `check_production_gate.py` N/A por Bloco 0.a, herdados do ciclo anterior.
- `.specify/feature.json` → `specs/012-evidencia-make-check`; branch `feature/012-evidencia-make-check`, HEAD `e2e8894`. Estado remoto nesta sessão é **[NV]**.
- Binário fixado: `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`; toda invocação usa `--headless --path . --script`.
- Resultados **realmente observados nesta sessão**: `test_contrato` 16 `PASS`, `test_servidor` 6, `test_validacao` 9, `test_ilha_sincronia` 14, `test_loopback` 8, `test_loopback_ilha` 3, `test_manifesto` 7; `COMPAT_OK`; `SERVIDOR_PRONTO porta=7777`; 5 gates `PASS`. Essas contagens cobrem as suítes executadas — não a árvore canônica completa, e nenhuma alegação de 39/39 caminhos é feita.
- CE-5 segue sem artefatos: `Makefile`, `docs/progress/evidence.csv` e `docs/progress/2026-11-19-ciclo-80-20.md` ainda não escritos nesta etapa.

## Restrições de execução
- Código literal da Task 6: `.PHONY`, `GODOT ?= godot`, `GODOT_FLAGS := --headless --path . --script`, ordem `test-contrato test-servidor test-validacao test-ilha test-manifesto check-compat`, cabeçalho de 9 colunas e ids `EV-20261119-001`..`EV-20261119-007` inalterados; nenhuma reescrita "melhorada".
- Toda linha do CSV começa `NAO_VERIFICADO` com `unverified` não vazio; `verdict` só muda após comando executado e saída observada em `result`, com `confidence` e `recorded_at` preenchidos nesse momento. `grep -c` do marcador deve retornar `≥ 1`.
- A linha agregada CE-5.1 usa `make check 2>&1 | tail -1` copiado literalmente, somente após a execução; nenhuma saída é afirmada sem observação (**[NV]**).
- `test_loopback.gd` e `test_loopback_ilha.gd` permanecem `NAO_VERIFICADO` até execução local: `SERVIDOR_PRONTO porta=7777` não equivale a cobertura integral nem a aceitação automática.
- Saídas com data/hora vão para a portaria de validação R4 na fase própria; versão, changelog e rollback para R7. Nenhum artefato R4/R7 é escrito aqui.
- Sem escrever `Makefile` ou `evidence.csv` nesta etapa de escrita; sem tocar nos sete scripts de teste ou no verificador de compatibilidade; sem branch, commit, push ou execução de Godot/gates/testes/lint/formatadores.
- Fora de escopo: 5v5, Ilha Meio, vitória Central, elenco, lobby/worker, release, deploy/remote, CI, factory, baselines e `converge.md`.
- Convergence/analyze é READ-ONLY: divergência corrigida na fonte (spec/plano/tasks) e revalidada nos 5 gates, nunca mascarada no artefato derivado. Nenhum percentual de conclusão é afirmado (convenção 80/20).
