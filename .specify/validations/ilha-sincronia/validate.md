# Validação (R4) — 010-ilha-sincronia

**Data:** 2026-11-20T03:13Z · **Branch:** `feature/010-ilha-sincronia` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ for t in test_contrato test_servidor test_loopback test_validacao test_ilha_sincronia test_loopback_ilha; do ...; done
test_contrato:      PASS=16 FAIL=0 OK=1 exit=0
test_servidor:      PASS=6  FAIL=0 OK=1 exit=0
test_loopback:      PASS=8  FAIL=0 OK=1 exit=0
test_validacao:     PASS=9  FAIL=0 OK=1 exit=0
test_ilha_sincronia:PASS=14 FAIL=0 OK=1 exit=0
test_loopback_ilha: PASS=3  FAIL=0 OK=1 exit=0
DELTA_EVENTO_MS 0
```

Total: 56 `PASS`, 0 `FAIL`, 0 testes com exit diferente de 0.

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | 14 `PASS` + `OK test_ilha_sincronia` | PASS |
| SC-002 | `DELTA_EVENTO_MS 0` + 3 `PASS` + `OK test_loopback_ilha` | PASS |
| SC-003 | 8 pontos por amostra × 3 = 24 em 3 s (verificações do teste) | PASS |
| SC-004 | 5 gates — saída anexada abaixo | PASS |

## Ressalva
Conexão UDP real entre dois clientes e handshake pela rede continuam **[NV]** — os testes acionam `handle_join`/`agendar_comando` diretamente. Regra de produção (`required_players = 3`) preservada no Módulo base, distinta do exemplo (`2`). Nenhum deploy/remote afirmado.

## Gates finais (2026-11-20T03:13:21Z)

```text
sdd-completeness:     PASS (spec_dirs=10 gaps=0 baselined=0)  exit=0
work-order-dag:       PASS (files=10 tasks=222 cycles=0)      exit=0
task-id-uniqueness:   PASS (tasks_files=10 duplicates=0)      exit=0
converge-append-only: PASS (files=0 baselined=0)              exit=0
hub-docs-gate:        PASS (0 errors, 0 warnings)             exit=0
```

`check_sdd_completeness` esteve RED (`gaps=1: release, validate`) entre a abertura do ciclo e a criação destes artefatos; a saída acima é a reexecução.
