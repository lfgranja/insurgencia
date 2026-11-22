# Validação (R4) — 017-rede-e2e

**Data:** 2026-11-20T10:04Z · **Branch:** `feature/017-rede-e2e` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ timeout 90s make test-rede-e2e GODOT=$G
PASS servidor_pronto
PASS join_aceito_1
PASS comando_enviado_1
OK test_cliente_rede
PASS cliente_1
PASS join_aceito_2
PASS comando_enviado_2
OK test_cliente_rede
PASS cliente_2
OK test_e2e_rede
exit=0

$ log do servidor (trecho)
SERVIDOR_PRONTO porta=7780
PEER_CONECTADO id=595593547 aguardando_JOIN
COMANDO_ACEITO peer=595593547 acao=mover
PEER_CONECTADO id=1377054530 aguardando_JOIN
COMANDO_ACEITO peer=1377054530 acao=mover

$ regressao: 11 suites = 131 PASS, 0 FAIL, todos exit=0
$ make check GODOT=$G -> exit=0
```

## Correção determinística
`tests/test_cliente_rede.gd` usava `OS.get_ticks_msec()` (API do Godot 3); substituído por `Time.get_ticks_msec()` após `Parse Error: Static function "get_ticks_msec()" not found in base "GDScriptNativeClass"`.

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | dois processos clientes reais conectam por UDP e recebem `join_accept` | PASS |
| SC-002 | `handle_join`/`handle_command` via `peer_packet` com envelope próprio | PASS |
| SC-003 | comandos pela rede aceitos e registrados pelo servidor | PASS |
| SC-004 | orquestrador `OK test_e2e_rede`, exit 0; 131 `PASS` em regressão | PASS |
| SC-005 | `make check` exit 0; 5 gates anexados | PASS |

## Evidência e ressalva
`docs/progress/evidence.csv`: `EV-20261120-008` passou a `PASS` com resultado observado; `EV-20261120-009` registra o **[NV]** restante de NAT/clientes externos. Nenhum lobby, servidor de produção ou 5v5 é afirmado.
