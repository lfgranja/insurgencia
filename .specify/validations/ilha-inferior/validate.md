# Validação (R4) — 014-ilha-inferior

**Data:** 2026-11-20T09:43Z · **Branch:** `feature/014-ilha-inferior` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ 9 suites: contrato 16, servidor 6, loopback 8, validacao 9, ilha_sincronia 14,
  loopback_ilha 3, manifesto 7, ilha_topo 16, ilha_inferior 16
TOTAL=95 PASS, 0 FAIL, todos exit=0

$ timeout 180s make check GODOT=$G
exit=0
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | crédito de 5 Materiais a cada 3 s (teste) | PASS |
| SC-002 | teto de 3 ciclos/15 Materiais; esgotamento e recarga em 20 s | PASS |
| SC-003 | crédito forjado e ação direta recusados (`MATERIAIS_FORJADOS`) | PASS |
| SC-004 | 16 `PASS`, `OK test_ilha_inferior`, exit 0 | PASS |
| SC-005 | `make check` exit 0; 5 gates anexados | PASS |

## Ressalva
Conexão UDP real, funções (Artesão/Guerreiro) e vitória da Ilha Central permanecem fora de escopo / **[NV]**.
