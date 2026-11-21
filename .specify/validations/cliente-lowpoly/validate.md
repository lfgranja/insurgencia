# Validação (R4) — 026-cliente-lowpoly

**Data:** 2026-11-21T02:27Z · **Branch:** `feature/026-cliente-lowpoly` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ cargo check --locked --workspace --all-targets
Finished dev profile

$ ./verify.sh
stage 1/4 cargo check ... ok
stage 2/4 cargo clippy pedantic -D warnings ... ok
stage 3/4 cargo nextest run --workspace
  34 tests run: 34 passed, 0 skipped
stage 4/4 cargo fmt --all -- --check ... ok
verify.sh: ALL STAGES OK

$ ./target/debug/insurgencia-cliente --minimizado (18s, agente)
exit=0 (processo vivo ate encerramento externo)
INFO bevy_render::renderer: AdapterInfo { name: "AMD Radeon Vega 10 Graphics (RADV RAVEN)", backend: Vulkan }
INFO bevy_winit::system: Creating new window insurgencia-cliente
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | `verify.sh` ALL STAGES OK | PASS |
| SC-002 | `Camera3d` + malhas procedurais (cubos/prismas) com iluminação | PASS |
| SC-003 | 4 ilhas por silhueta+cor, sem textura (GDD:1045) | PASS |
| SC-004 | 34 testes (5 cliente + 8 sim + 7 contratos + 8 metricas + 6 exportacao) | PASS |
| SC-005 | 5 gates do alvo preservados | PASS |

## Ressalva
Executado **minizado** conforme regra da SE; inspeção visual da cena 3D fica com a SE **[NV]**. Kenney CC0 e assets externos não foram baixados (host fora da allowlist; licença a auditar) — malhas procedurais atendem GDD:1127 por construção.
