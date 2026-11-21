# Validação (R4) — 021-migracao-bevy

**Data:** 2026-11-20T23:59Z · **Branch:** `feature/021-migracao-bevy` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ cargo info bevy@0.20.1
version: 0.20.1 | license: MIT OR Apache-2.0 | rust-version: 1.97.1

$ cargo generate-lockfile
Locking 14 packages; Adding serde_json v1.0.140; Adding thiserror v2.0.0

$ ./verify.sh
stage 1/4 cargo check ... ok
stage 2/4 cargo clippy ... ok
stage 3/4 cargo nextest run --workspace
  7 tests run: 7 passed, 0 skipped
stage 4/4 cargo fmt --all -- --check ... ok
verify.sh: ALL STAGES OK
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | 4 estágios do `verify.sh` verdes | PASS |
| SC-002 | 4 códigos de recusa literais no crate de contratos | PASS |
| SC-003 | hash 64 hex e contrato versionado portados (tipos e testes) | PASS |
| SC-004 | zero `unsafe`, zero `unwrap`/`expect`; `forbid`+`deny` no código | PASS |
| SC-005 | gates do alvo — saída anexada na entrega | PASS |

## Ressalva
Ainda não há servidor Bevy nem Lightyear neste ciclo; este é o workspace Rust e o contrato. `make check` GDScript e `make test-rede-e2e` permanecem como verificação do estado histórico até a migração dos próximos módulos. Nenhum benchmark ou reconexão Rust foi medido `[NV]`.
