# Validação (R4) — 022-simulacao-rust

**Data:** 2026-11-21T00:14Z · **Branch:** `feature/022-simulacao-rust` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ cargo generate-lockfile
Locking 24 packages; Adding sha2 v0.10.9; insurgencia-sim incluida

$ ./verify.sh
stage 1/4 cargo check ... ok
stage 2/4 cargo clippy pedantic -D warnings ... ok
stage 3/4 cargo nextest run --workspace
  15 tests run: 15 passed, 0 skipped
  (7 insurgencia-contratos::protocolo + 8 insurgencia-sim::simulacao)
stage 4/4 cargo fmt --all -- --check ... ok
verify.sh: ALL STAGES OK
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | 4 estágios do verify.sh verdes | PASS |
| SC-002 | 5 códigos de recusa implementados e testados | PASS |
| SC-003 | 3 fases na ordem; movimento e limites testados | PASS |
| SC-004 | hash SHA-256 de 64 hex; duas simulações idênticas produzem mesmo hash | PASS |
| SC-005 | 5 gates do alvo preservados (saída na entrega) | PASS |

## Ressalva
Sem servidor Bevy/Lightyear, sem benchmark de rede e sem reconexão Rust `[NV]`. O código GDScript histórico permanece como registro; nenhuma cena principal foi removida.
