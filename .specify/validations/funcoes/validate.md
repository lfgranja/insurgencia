# Validação (R4) — 015-funcoes

**Data:** 2026-11-20T09:49Z · **Branch:** `feature/015-funcoes` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ timeout 25s $G --headless --path . --script res://tests/test_funcoes.gd
PASS=18 FAIL=0 OK=1 exit=0
OK test_funcoes

$ 10 suites: 16+6+8+9+14+3+7+16+16+18 = 113 PASS, 0 FAIL, todos exit=0
$ timeout 180s make check GODOT=$G  -> exit=0
```

## Correções determinísticas
1. `tests/test_funcoes.gd` — variável `i` (instância de `Funcoes`) colidia com `for i in range(...)`; renomeada para `funcoes`.
2. `tests/test_funcoes.gd` — loop do teste de recarga avançava só até 1,1667 s (< 1,2 s de canalização); ampliado para 37 iterações.

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | Barreira: 20 Materiais, 1,5 s, 200 PV, 12 s, interrupção com reembolso | PASS |
| SC-002 | Ataque Forte: 200 dano em 1,2 s, recarga 6 s, recusa em recarga | PASS |
| SC-003 | `RECURSO_FORJADO` recusado no servidor | PASS |
| SC-004 | 18 `PASS`, `OK test_funcoes`, exit 0; 113 `PASS` no total | PASS |
| SC-005 | `make check` exit 0; 5 gates anexados | PASS |

## Ressalva
Vitória da Ilha Central, 5v5 e conexão UDP real permanecem fora de escopo / **[NV]**.
