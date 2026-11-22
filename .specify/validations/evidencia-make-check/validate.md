# Validação (R4) — 012-evidencia-make-check

**Data:** 2026-11-20T03:20Z · **Branch:** `feature/012-evidencia-make-check` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ timeout 180s make check GODOT=$G > /tmp/make-check.log 2>&1; echo "make_exit=$?"
make_exit=0
OK test_contrato
OK test_servidor
OK test_validacao
OK test_ilha_sincronia
OK test_manifesto
COMPAT_OK module_id=insurgencia.exemplo
OK total=5  COMPAT_OK=1  FAIL=0

$ head -1 docs/progress/evidence.csv
id,criterio,verdict,source,command,result,confidence,unverified,recorded_at
$ awk -F',' 'NR>1 {print NF}' docs/progress/evidence.csv | sort -u
9
$ grep -c NAO_VERIFICADO docs/progress/evidence.csv
1
$ test -f docs/progress/2026-11-19-ciclo-80-20.md && echo PRESENTE
PRESENTE
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | `make check` exit 0; 5 `OK` + `COMPAT_OK`; 0 `FAIL` | PASS |
| SC-002 | cabeçalho literal de 9 colunas; todas as 8 linhas com 9 campos | PASS |
| SC-003 | 1 linha `NAO_VERIFICADO` (conexão UDP real, `EV-20261120-008`) | PASS |
| SC-004 | registro datado `docs/progress/2026-11-19-ciclo-80-20.md` presente | PASS |
| SC-005 | 5 gates do alvo — saída anexada | PASS |

## Ressalva
`EV-20261120-008` permanece `NAO_VERIFICADO` por decisão metodológica: conexão UDP real entre dois clientes distintos não foi observada. Nenhuma linha `PASS` com `unverified` não vazio. Nenhum deploy/remote afirmado.

## Gates finais (2026-11-20T03:20:33Z)

```text
sdd-completeness:     PASS (spec_dirs=12 gaps=0 baselined=0)  exit=0
work-order-dag:       PASS (files=12 tasks=222 cycles=0)      exit=0
task-id-uniqueness:   PASS (tasks_files=12 duplicates=0)      exit=0
converge-append-only: PASS (files=0 baselined=0)              exit=0
hub-docs-gate:        PASS (0 errors, 0 warnings)             exit=0
```

`check_sdd_completeness` esteve RED (`gaps=1: release, validate`) até a criação destes artefatos; a saída acima é a reexecução.
