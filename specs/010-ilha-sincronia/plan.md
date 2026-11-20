# Implementation Plan — Desafio da Ilha: sincronia reduzida de 2 jogadores em 5 s

**Data:** 2026-11-20T03:10Z · **Fonte:** `docs/plans/2026-11-19-ciclo-80-20-omoba.md` Task 4 (Steps 1–8, linhas 723–1025).

## Arquivos (exatamente os do plano)
Criação: `nucleo/desafios/ilha_sincronia.gd`, `modulos/exemplo/mapa.json`, `modulos/base/regras.cfg`, `tests/test_ilha_sincronia.gd`, `tests/test_loopback_ilha.gd`. Modificações: `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`. Leitura: `modulos/exemplo/regras.cfg` (já existente, não alterado).

## Steps
1. Task 4 / Step 1 — escrever `nucleo/desafios/ilha_sincronia.gd`: cinco constantes, campos e `sinalizar`/`_iniciar_gas`/`aplicar_gas`/`reiniciar` (FR-001, FR-002).
2. Task 4 / Step 2 — escrever `modulos/exemplo/mapa.json` com `ilha_meio`, `placas=[1, 2]` e `janela_segundos=5.0` (FR-003).
3. Task 4 / Step 3 — validar `alvo` em 1..2 em `nucleo/sim/validacao.gd`, recusando com `PLACA_INVALIDA` (FR-006).
4. Task 4 / Step 4 — integrar `ilha_sincronia` e `estado_persistido` ao tick de `Mundo` (`_fase_skills` e `_fase_finish`) (FR-005).
5. Task 4 / Step 5 — escrever `tests/test_ilha_sincronia.gd` com 14 verificações determinísticas (FR-007, SC-001, SC-003).
6. Task 4 / Step 6 — marco CE-3: executar `test_ilha_sincronia.gd` e conferir 14 `PASS`, `OK` e exit 0 (SC-001, SC-003).
7. Task 4 / Step 7 — escrever `modulos/base/regras.cfg` (3 jogadores) e `tests/test_loopback_ilha.gd`; executar e conferir `DELTA_EVENTO_MS`, 3 `PASS` e `OK` (FR-004, FR-007, SC-002).
8. Task 4 / Step 8 — commit atômico com staging explícito dos sete paths (a executar na implementação; não realizado nesta etapa de escrita).
9. Convergência READ-ONLY: revisar spec/plano/tasks contra a Task 4 do plano, corrigir divergências na fonte e reexecutar os 5 gates (SC-004).

## Comandos de verificação
```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
"$G" --headless --path . --script res://tests/test_ilha_sincronia.gd
"$G" --headless --path . --script res://tests/test_loopback_ilha.gd
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```
Saídas não executadas nesta etapa permanecem **[NV]** e só entram com data/hora na portaria de validação (R4); versão/changelog/rollback em R7. Nenhum artefato das portarias é escrito aqui.

## Riscos e mitigação
- `--script` exige `extends SceneTree` → os dois testes já seguem essa forma.
- Janela medida por relógio de cliente → usar sempre `tick * PASSO_SEGUNDOS` fornecido pelo servidor.
- Dano acumulado por quadro → debitar por amostra vencida de 1,0 s (8 pontos; 24 em 3 s), não a cada quadro; expulsão aos 3,0 s e nenhum gás após captura.
- Regras divergentes → janela de 5 s idêntica; `required_players=3` na base e `=2` no exemplo, verificados literalmente no loopback.
- Árvore suja com trabalho alheio → staging explícito dos sete paths, nunca `git add -A`.
