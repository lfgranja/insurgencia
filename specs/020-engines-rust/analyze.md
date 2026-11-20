# Analyze (READ-ONLY) — 020-engines-rust

**Data:** 2026-11-20T23:49Z · **Escopo:** spec, plano, checklist, tasks e relatos de pesquisa.

| # | Achado | Severidade | Tratamento |
|:--|:--|:--|:--|
| A1 | `rust-outras.md` dá Fyrox 62,6 e `rust-fyrox.md` dá 65,2 — duas pontuações divergentes para a mesma engine | alta | Usar 65,2: `rust-fyrox.md` é o relatório dedicado, com 10 notas justificadas e fonte primária mais densa; registrar divergência na seção de limitações |
| A2 | O10 é opção derivada, não construída e sem evidência própria | média | Manter na matriz apenas como derivada explícita de O3, marcada `[NV]`, sem recomendação de adoção |
| A3 | `rust-outras.md` classifica gdext como binding, não engine | informativo | Manter O8 fora da categoria "engine escrita em Rust"; score 68,6* é de integração sobre O1 |
| A4 | Nenhum benchmark local foi feito; todas as notas C6 são proxies documentais | alta | Declarar na seção 7 do documento final e nos `[NV]`; spike A permanece como próxima verificação |
| A5 | Cobertura idiomática restrita ao inglês; hosts fora da allowlist não consultados | média | Declarar cobertura real; não afirmar triangulação |
| A6 | ADR-001 mantém Godot-first vinculante | informativo | Nenhuma recomendação pode trocar motor; ADAPT apenas como padrões em GDScript |

## Veredito
Prosseguir para `implement`, com A1, A2, A4 e A5 como requisitos explícitos do documento final.
