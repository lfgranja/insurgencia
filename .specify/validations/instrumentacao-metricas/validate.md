# Validação (R4) — 024-instrumentacao-metricas

**Data:** 2026-11-21T01:12Z · **Branch:** `feature/024-instrumentacao-metricas` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ cargo generate-lockfile -> Locking 580 packages
$ ./verify.sh
stage 1/4 cargo check ... ok
stage 2/4 cargo clippy pedantic -D warnings ... ok
stage 3/4 cargo nextest run --workspace
  28 tests run: 28 passed, 0 skipped
  (7 contratos + 8 sim + 5 cliente + 8 metricas)
stage 4/4 cargo fmt --all -- --check ... ok
verify.sh: ALL STAGES OK
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | `verify.sh` ALL STAGES OK | PASS |
| SC-002 | 20 registros determinísticos em `docs/evidencias/metricas-20-partidas-2026-11-21.md` | PASS |
| SC-003 | 5 métricas calculadas (M1–M5) + taxa de conclusão | PASS |
| SC-004 | duas partidas consecutivas com cliente modificado; 3 divergências no Núcleo | PASS |
| SC-005 | relatório datado; 5 gates do alvo preservados | PASS |

## Correções determinísticas
Lockfile atualizado; sombreamento de função no teste corrigido; `manual_is_multiple_of` corrigido; `struct_excessive_bools` resolvido com 4 enums de dois estados (sem `#[allow]`).

## Ressalva
Números vêm de simulação determinística no Núcleo, não de partidas humanas ou servidores reais. M4/M5 são contagens simuladas — itens 5/6 da SE estão DEFER. Metas da §17 permanecem `[HIPÓTESE]`; nenhuma recalibração foi feita.
