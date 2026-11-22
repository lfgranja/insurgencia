# Validação (R4) — 013-ilha-topo

**Data:** 2026-11-20T09:37Z · **Branch:** `feature/013-ilha-topo` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ timeout 25s $G --headless --path . --script res://tests/test_ilha_topo.gd
PASS=16 FAIL=0 OK=1 exit=0
OK test_ilha_topo

$ timeout 180s make check GODOT=$G
exit=0
OK test_contrato / OK test_servidor / OK test_validacao / OK test_ilha_sincronia /
OK test_manifesto / COMPAT_OK module_id=insurgencia.exemplo

$ 8 suites completas (contrato, servidor, loopback, validacao, ilha_sincronia, loopback_ilha, manifesto, ilha_topo)
PASS=16+6+8+9+14+3+7+16 = 79, FAIL=0, todos exit=0
```

## Correções determinísticas
1. `nucleo/mundo.gd` — o `preload` de `IlhaSincronia` foi perdido na integração com `IlhaTopo`, causando `Identifier "IlhaSincronia" not declared` e travamento de 4 suítes; restaurado.
2. `tests/test_ilha_topo.gd` — inferência `:=` sobre `Mundo.new()`/`_mundo_com_topo()` causava `Cannot infer the type`; substituída por tipo explícito `: Mundo` e retorno `-> Mundo`.

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | ordem 1→2→3 em ≤45 s captura e emite evento (teste) | PASS |
| SC-002 | alavanca fora de ordem reinicia em 2 s sem captura | PASS |
| SC-003 | 3 ativações fora da janela falham; validação no servidor | PASS |
| SC-004 | 16 `PASS`, `OK test_ilha_topo`, exit 0; 79 `PASS` no total | PASS |
| SC-005 | `make check` exit 0; 5 gates anexados | PASS |

## Ressalva
Conexão UDP real entre clientes e vitória da Ilha Central permanecem **[NV]** / fora de escopo.
