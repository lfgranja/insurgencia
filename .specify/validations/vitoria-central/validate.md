# Validação (R4) — 016-vitoria-central

**Data:** 2026-11-20T10:00Z · **Branch:** `feature/016-vitoria-central` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ timeout 25s $G --headless --path . --script res://tests/test_vitoria_central.gd
PASS=18 FAIL=0 OK=1 exit=0
OK test_vitoria_central

$ 11 suites: 16+6+8+9+14+3+7+16+16+18+18 = 131 PASS, 0 FAIL, todos exit=0
$ timeout 180s make check GODOT=$G  -> exit=0
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | sem 3 ilhas a canalização não inicia | PASS |
| SC-002 | Artesão e Guerreiro vivos/presentes são obrigatórios; outros não substituem | PASS |
| SC-003 | 4,0 s contínuos (3,99 s não vence); interrupção zera o progresso | PASS |
| SC-004 | vitória declarada pelo servidor com evento `vitoria_ilha_central` | PASS |
| SC-005 | 18 `PASS`, `OK test_vitoria_central`; 131 `PASS` no total; `make check` exit 0; 5 gates | PASS |

## Ressalva
A "tela de resultado" é representada pelo estado/evento persistido do servidor; nenhuma cena de UI Godot foi implementada. Regra de produção (seis ilhas/20 s) permanece documentada em `modulos/base/regras.cfg`. UDP real entre clientes, 5v5 e deploy/remote permanecem **[NV]**.
