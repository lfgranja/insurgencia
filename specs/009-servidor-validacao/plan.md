# Implementation Plan — Servidor headless e validação autoritativa

**Data:** 2026-11-20T03:00Z · **Fonte:** `docs/plans/2026-11-19-ciclo-80-20-omoba.md` Task 2 (Steps 1–7) e Task 3 (Steps 1–6).

## Arquivos (exatamente os do plano)
Criação: `nucleo/mundo.gd`, `nucleo/servidor.gd`, `nucleo/main_server.gd`, `tests/test_servidor.gd`, `tests/test_loopback.gd`, `nucleo/sim/validacao.gd`, `tests/test_validacao.gd`. Modificações: `nucleo/servidor.gd`, `nucleo/mundo.gd`.

## Steps
1. Task 2 / Step 1 — escrever `nucleo/mundo.gd`: passo fixo 1/30 s e fases `prepare → skills → finish` (FR-001).
2. Task 2 / Step 2 — escrever `nucleo/servidor.gd`: `abrir`, `handle_join`, `aceitar_saida` e handshake com os três códigos de recusa (FR-002).
3. Task 2 / Step 3 — escrever `nucleo/main_server.gd` (`extends SceneTree`) com `--porta=` (FR-003).
4. Task 2 / Step 4 — escrever `tests/test_servidor.gd` com 6 verificações determinísticas (FR-008, SC-001).
5. Task 2 / Step 5 — acrescentar `SNAPSHOT_HZ`, `emitir_snapshot` e `avancar_com_snapshots` com acumulador; escrever `tests/test_loopback.gd` (FR-004, SC-002).
6. Task 2 / Step 6 — marco CE-2: `test_servidor.gd` e `main_server.gd` sob `timeout 3s` (SC-001, SC-004).
7. Task 2 / Step 7 — commit atômico do Task 2 com staging explícito (a executar na implementação; não realizado nesta etapa de escrita).
8. Task 3 / Step 1 — escrever `nucleo/sim/validacao.gd` com os cinco códigos (FR-005).
9. Task 3 / Step 2 — ligar o validador em `handle_command` no servidor (FR-006).
10. Task 3 / Step 3 — acrescentar `Mundo.hash_estado()` (FR-007).
11. Task 3 / Step 4 — escrever `tests/test_validacao.gd` com 9 verificações (FR-008, SC-003).
12. Task 3 / Step 5 — marco CE-2.2/CE-2.4: `test_validacao.gd` (SC-003).
13. Task 3 / Step 6 — commit atômico do Task 3 (a executar na implementação).
14. Convergência READ-ONLY: corrigir divergências na fonte e reexecutar os 5 gates (SC-005).

## Comandos de verificação
```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
"$G" --headless --path . --script res://tests/test_servidor.gd
"$G" --headless --path . --script res://tests/test_loopback.gd
"$G" --headless --path . --script res://tests/test_validacao.gd
timeout 3s "$G" --headless --path . --script res://nucleo/main_server.gd -- --porta=7777; test $? -eq 124
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```
Saídas não executadas nesta etapa permanecem **[NV]** e só entram com data/hora na portaria de validação.

## Riscos e mitigação
- `--script` exige `extends SceneTree` → os três testes e `main_server.gd` já seguem essa forma.
- Acumulador de snapshot: conferir a contagem exata de 20 em 1 s no `test_loopback.gd`.
- Hash de paridade dependente de serialização → mesmo algoritmo literal de `hash_estado()` nos dois mundos do teste.
- Árvore suja com trabalho alheio → staging explícito dos sete paths, nunca `git add -A`.
