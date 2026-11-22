# Validação (R4) — 019-tela-resultado

**Data:** 2026-11-20T11:52Z · **Branch:** `feature/019-tela-resultado` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ timeout 25s $G --headless --path . --script res://tests/test_tela_resultado.gd
PASS=19 FAIL=0 OK=1 exit=0
OK test_tela_resultado

$ 12 suites de regressao: total 167 PASS, 0 FAIL, todos exit=0
$ timeout 180s make check GODOT=$G  -> exit=0
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | 1 componente `Control`, 3 rótulos criados por código, sem `.tscn` e sem `main scene` | PASS |
| SC-002 | 4 entradas inválidas/ausentes → `AGUARDANDO`, sem vitória inventada | PASS |
| SC-003 | 19 `PASS`, `OK test_tela_resultado`, exit 0; 167 `PASS` em regressão | PASS |
| SC-004 | `make check` exit 0; 5 gates anexados | PASS |
| SC-005 | nenhuma chamada de rede, arquivo ou provedor externo no componente | PASS |

## Correções determinísticas
Três erros reais corrigidos por subagentes e registrados em `converge.md`: divergência de impacto/timestamp no `decision.md`, estado inicial ausente da tela e `DirAccess.close()` inexistente.

## Ressalva
A tela consome o evento produzido pelo servidor; não há cena principal, renderização visual inspecionada ou conexão nova. Nenhuma afirmação remota é feita aqui.
