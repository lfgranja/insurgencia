# Validação (R4) — 025-soberania-exportacao

**Data:** 2026-11-21T01:22Z · **Branch:** `feature/025-soberania-exportacao` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ cargo generate-lockfile -> Locking 580 packages
$ ./verify.sh
stage 1/4 cargo check ... ok
stage 2/4 cargo clippy pedantic -D warnings ... ok
stage 3/4 cargo nextest run --workspace
  34 tests run: 34 passed, 0 skipped
  (7 contratos + 8 sim + 5 cliente + 8 metricas + 6 exportacao)
stage 4/4 cargo fmt --all -- --check ... ok
verify.sh: ALL STAGES OK

$ test -f COPYRIGHT.txt -> presente
$ bash scripts/dco-check.sh (no commit do ciclo) -> DCO OK
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | `verify.sh` ALL STAGES OK | PASS |
| SC-002 | exportar→importar roundtrip; versão divergente e campos inválidos recusados | PASS |
| SC-003 | `COPYRIGHT.txt` presente com MIT/ADR-002, fonte e data | PASS |
| SC-004 | DCO `Signed-off-by` no commit do ciclo | PASS |
| SC-005 | 5 gates do alvo preservados | PASS |

## Ressalva
**Assinatura criptográfica GPG não foi aplicada**: não há chave no ambiente e gerar uma exige autorização da SE. O GDD:1452 ("todo commit assinado") foi satisfeito como assinatura de origem (DCO), interpretada neste ciclo; a forma GPG permanece **[NV]**. Exportação é local, sem servidor; nenhuma conta real foi criada.
