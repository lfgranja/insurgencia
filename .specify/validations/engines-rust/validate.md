# Validação (R4) — 020-engines-rust

**Data:** 2026-11-20T23:53Z · **Branch:** `feature/020-engines-rust` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ grep -n '^## ' docs/AVALIACAO-ENGINES-RUST.md
1. Critérios, pesos e escala / 2. Opções avaliadas / 3. Matriz comparativa /
4. Análise explicativa por critério / 5. Tabela de evidência /
6. Recomendação final e spikes / 7. Limitações e próxima verificação

$ scores presentes: 82,6 / 77,1 / 76,4 / 68,6 / 66,8 / 65,2 / 60,4 / 57,0 / 54,8 / 54,5
$ opções na matriz: 10 (O1–O10)
$ linhas de evidência com github.com: 22
$ resultado atual da última linha: "Estado: documento de apoio a ADR-001; nenhuma troca de motor autorizada"
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | 7 seções do protocolo factory presentes | PASS |
| SC-002 | Σ pesos C1–C10 = 100; escala 0–10 declarada | PASS |
| SC-003 | score recalculado em todas as 10 linhas (1 casa decimal) | PASS |
| SC-004 | ≥10 opções com categorias explícitas (engine/binding/biblioteca/framework) | PASS |
| SC-005 | 5 gates do alvo — saída anexada abaixo; `make check` inalterado | PASS |

## Ressalva
Nenhum benchmark pareado, reconexão Rust ou migração de toolchain foi executado; notas `*` e C6 são proxies documentais. Cobertura idiomática restrita ao inglês. Nenhuma afirmação remota além de páginas `github.com` consultadas ao vivo em 20/11/2026. ADR-001 não foi reaberta.

## Gates finais (após bootstrap do converge)

```text
sdd-completeness:     PASS (spec_dirs=20 gaps=0 baselined=0)  exit=0
work-order-dag:       PASS (files=20 tasks=222 cycles=0)      exit=0
task-id-uniqueness:   PASS (tasks_files=20 duplicates=0)      exit=0
converge-append-only: PASS (files=3 baselined=3)              exit=0
hub-docs-gate:        PASS (0 errors, 0 warnings)             exit=0
```

`converge-append-only` esteve RED (`UNBASELINED: specs/020-engines-rust/converge.md`) até o bootstrap executado pelo próprio gate; o baseline foi atualizado com a saída do comando, não com edição manual de hashes.
