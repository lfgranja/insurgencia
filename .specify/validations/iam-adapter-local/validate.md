# Validação (R4) — 018-iam-adapter-local

**Data:** 2026-11-20T11:35Z · **Branch:** `feature/018-iam-adapter-local` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ timeout 25s $G --headless --path . --script res://tests/test_iam_adapter_local.gd
PASS=36 FAIL=0 OK=1 exit=0
OK test_iam_adapter_local

$ 11 suites de regressao: contrato 16, servidor 6, loopback 8, validacao 9,
  ilha_sincronia 14, loopback_ilha 3, manifesto 7, ilha_topo 16,
  ilha_inferior 16, funcoes 18, vitoria_central 18
TOTAL=131 PASS, 0 FAIL, todos exit=0

$ timeout 180s make check GODOT=$G  -> exit=0
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | 4/4 métodos com assinaturas exatas; 9/9 códigos literais verificados no teste | PASS |
| SC-002 | 36 verificações `PASS`, `OK test_iam_adapter_local`, exit 0, 0 `FAIL`; inclui `PERMISSAO_NEGADA` e `SESSAO_INVALIDA` | PASS |
| SC-003 | `make check` exit 0 | PASS |
| SC-004 | 5 gates do alvo — saída anexada abaixo | PASS |
| SC-005 | 0 segredo, 0 chave privada, 0 credencial versionada, 0 servidor, 0 federação, 0 login obrigatório, 0 5v5; R4 antes de R7 | PASS |

## Correções determinísticas
Três erros reais encontrados na verificação e corrigidos por subagentes: `extends IAMInterface` por `class_name` (trocado por caminho literal), anotações de tipo por `class_name` no teste (trocadas por constants de preload) e `\t` literal em quatro linhas (trocado por TAB). Registrados em `converge.md`.

## Ressalva
Nenhum IAM remoto, login obrigatório, federação ou credencial real foi implementado. Provedor real permanece **[NV]**.

## Gates finais (2026-11-20T11:33:21Z, após bootstrap do converge)

```text
sdd-completeness:     PASS (spec_dirs=18 gaps=0 baselined=0)  exit=0
work-order-dag:       PASS (files=18 tasks=222 cycles=0)      exit=0
task-id-uniqueness:   PASS (tasks_files=18 duplicates=0)      exit=0
converge-append-only: PASS (files=1 baselined=1)              exit=0
hub-docs-gate:        PASS (0 errors, 0 warnings)             exit=0
```

Nota: `check_converge_append_only` esteve RED (`UNBASELINED: specs/018-iam-adapter-local/converge.md`) até o bootstrap executado pelo próprio gate (`--bootstrap`), que gerou `bytes=1056` e `sha256=22286499c0d9209d0c9cca93ab545e8485b59a8140ee3bbcabe1824f9e7ee6af`. O baseline foi atualizado com a saída do gate, não com edição manual. Nenhum arquivo preexistente foi reescrito.
