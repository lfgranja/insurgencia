# Implementation Plan — Evidência auditável do gate `make check` e registro do não verificado

**Data:** 2026-11-20T03:19Z · **Fonte:** `docs/plans/2026-11-19-ciclo-80-20-omoba.md` Task 6 (Steps 1–4, linhas 1219–1304).

## Arquivos (exatamente os do plano)
Modificação: `Makefile`. Criação: `docs/progress/evidence.csv`, `docs/progress/2026-11-19-ciclo-80-20.md`. Leitura (sem alteração): `tests/test_contrato.gd`, `tests/test_servidor.gd`, `tests/test_validacao.gd`, `tests/test_ilha_sincronia.gd`, `tests/test_manifesto.gd`, `tests/test_loopback.gd`, `tests/test_loopback_ilha.gd`, `ferramentas/verificar_compatibilidade.gd`. Nenhum outro path é tocado; nenhum artefato R4/R7 é escrito nesta etapa.

## Steps
1. Task 6 / Step 1 — completar o `Makefile` com a porta única: `.PHONY`, `GODOT ?= godot`, `GODOT_FLAGS := --headless --path . --script` e `check: test-contrato test-servidor test-validacao test-ilha test-manifesto check-compat`, com os cinco alvos de teste e `check-compat` intactos e falha rápida no primeiro erro (FR-001, FR-004, SC-001).
2. Task 6 / Step 2 — escrever `docs/progress/evidence.csv` com o cabeçalho literal de 9 colunas e as linhas `EV-20261119-001`..`EV-20261119-007`, todas iniciando `NAO_VERIFICADO`, `result`/`recorded_at` vazios e `unverified` não vazio; regra de transição ao `PASS` somente após comando executado e saída observada (FR-002, FR-005, SC-002, SC-003).
3. Task 6 / Step 2 — escrever `docs/progress/2026-11-19-ciclo-80-20.md` com título datado, ponteiro para o CSV e os ids, estado `ilha_concluida`, fora do escopo GDD §16.3/§16.4 e marco de reavaliação após `make check` (FR-003, SC-004).
4. Task 6 / Step 3 — marco de verificação: `make check` (sequência de `OK` + `COMPAT_OK`, exit 0), `grep -c -E 'NAO_VERIFICADO|NAO_VERIFICADO'` (`≥ 1`), `test -f docs/progress/2026-11-19-ciclo-80-20.md` (exit 0) e `make check 2>&1 | tail -1` registrado literalmente na linha CE-5.1 somente após execução, com os 5 gates PASS (FR-006, SC-001, SC-003, SC-004, SC-005).
5. Task 6 / Step 4 — commit atômico com staging explícito dos três paths (`Makefile`, `docs/progress/evidence.csv`, `docs/progress/2026-11-19-ciclo-80-20.md`); a executar na fase própria, nunca nesta etapa de escrita.
6. Convergência READ-ONLY: revisar spec/plano/tasks contra a Task 6 do plano, corrigir divergências na fonte e revalidar os 5 gates (SC-005).

## Comandos de verificação (a executar na fase de implementação)
```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
make check
grep -c -E 'NAO_VERIFICADO|NAO_VERIFICADO' docs/progress/evidence.csv
test -f docs/progress/2026-11-19-ciclo-80-20.md
make check 2>&1 | tail -1
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```
Resultados já observados nesta sessão e que servem de linha de base: `test_contrato` 16 `PASS`, `test_servidor` 6, `test_validacao` 9, `test_ilha_sincronia` 14, `test_loopback` 8, `test_loopback_ilha` 3, `test_manifesto` 7; `COMPAT_OK`; `SERVIDOR_PRONTO porta=7777`; 5 gates `PASS`. Toda saída ainda não observada permanece **[NV]** e só entra com data/hora na portaria de validação (R4); versão/changelog/rollback em R7. Nenhum artefato das portarias é escrito aqui, e nenhuma alegação de 39/39 caminhos é feita.

## Riscos e mitigação
- Falha silenciosa de suíte → `make check` encadeia dependências e para no primeiro erro; nenhuma linha final de sucesso é impressa após falha.
- Evidência otimista → toda linha começa `NAO_VERIFICADO`; `confidence` e `recorded_at` só são preenchidos com saída observada, e o marcador deve permanecer `≥ 1`.
- Loopback dado como verificado → `SERVIDOR_PRONTO porta=7777` observado não substitui a execução de `test_loopback.gd`/`test_loopback_ilha.gd`; essas linhas seguem **[NV]**.
- CSV com quebra de colunas → valores com vírgula entre aspas; cabeçalho comparado literalmente antes de qualquer edição.
- Linha agregada reescrita → `make check 2>&1 | tail -1` copiado literalmente para `result` da linha CE-5.1 apenas após execução.
- Árvore suja com trabalho alheio → staging explícito dos três paths do Task 6, nunca `git add -A`; nenhum branch, commit ou push nesta etapa.
- Convergência que mascara divergência → correção sempre na fonte (spec/plano/tasks), nunca no artefato derivado.
