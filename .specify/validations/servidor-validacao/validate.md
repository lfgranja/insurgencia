# Validação (R4) — 009-servidor-validacao

**Data:** 2026-11-20T03:08:32Z · **Branch:** `feature/009-servidor-validacao` · **HEAD de trabalho:** `0e26179` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ $G --headless --version
4.7.2.stable.official.ed1daf0bf

$ timeout 25s $G --headless --path . --script res://tests/test_contrato.gd
PASS=16 FAIL=0 OK=1 exit=0

$ timeout 25s $G --headless --path . --script res://tests/test_servidor.gd
PASS=6 FAIL=0 OK=1 exit=0

$ timeout 25s $G --headless --path . --script res://tests/test_loopback.gd
PASS=8 FAIL=0 OK=1 exit=0   (inclui 20_snapshots_em_1s e os 5 campos do snapshot)

$ timeout 25s $G --headless --path . --script res://tests/test_validacao.gd
PASS=9 FAIL=0 OK=1 exit=0

$ timeout 4s $G --headless --path . --script res://nucleo/main_server.gd -- --porta=7777
SERVIDOR_PRONTO porta=7777
```

## Correções descobertas pela verificação (convergência)
1. `Mundo.hash_estado()` — o plano prescreve `String.sha256_string()`, que **não existe** no Godot 4.7 (`Parse Error: Cannot find member "sha256_string" in base "String"`). Substituído por `HashingContext.HASH_SHA256` + `to_utf8_buffer()` + `hex_encode()`.
2. `avancar_com_snapshots()` — deriva de ponto flutuante no acumulador 30→20 Hz produzia 19 snapshots; acrescentada tolerância `+0.0000001` na comparação com `1/20`. Agora 20/20.
3. `abrir()` — `multiplayer` do Node e `get_tree()` são nulos durante `SceneTree._initialize()`; acesso via `Engine.get_main_loop()` e `get_multiplayer()` (método, não propriedade), com guardas. `SERVIDOR_PRONTO` observado.

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | 6 PASS + `OK test_servidor`, exit 0 | PASS |
| SC-002 | 8 PASS + `OK test_loopback`, 20 snapshots | PASS |
| SC-003 | 9 PASS + `OK test_validacao`, exit 0 | PASS |
| SC-004 | `SERVIDOR_PRONTO porta=7777` | PASS |
| SC-005 | 5 gates do alvo — saída no final deste arquivo | PASS |

## Ressalva
Os testes exercitam `handle_join`/`handle_command` diretamente; **conexão UDP real de dois clientes distintos** e handshake pela rede não foram observados neste ciclo — **[NV]**. Nenhum deploy, CI remoto adicional ou merge é afirmado aqui.

## Gates finais (2026-11-20T03:08:52Z, após R4/R7)

```text
sdd-completeness:  PASS (spec_dirs=9 gaps=0 baselined=0 strict_from=88)  exit=0
work-order-dag:    PASS (files=9 tasks=222 edges=0 cycles=0)              exit=0
task-id-uniqueness:PASS (tasks_files=9 duplicates=0 baselined=0)         exit=0
converge-append-only: PASS (files=0 baselined=0)                          exit=0
hub-docs-gate:     PASS (0 errors, 0 warnings)                           exit=0
```

Nota: `check_sdd_completeness` esteve RED (`gaps=1: release, validate`) entre a abertura do ciclo e a criação destes artefatos; a saída acima é a reexecução após materializá-los.
