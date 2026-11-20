# Analyze (READ-ONLY) — 018-iam-adapter-local

**Data:** 2026-11-20T11:25Z · **Escopo:** spec, plano, checklist e tasks — nenhuma edição de artefato nesta etapa.

## Achados

| # | Achado | Severidade | Correção na fonte |
|:--|:--|:--|:--|
| A1 | GDScript 4.7 não tem `interface` nativa; `IAMInterface` precisa ser uma classe `RefCounted` com métodos virtuais que falham explicitamente se chamados diretamente | média | Already refletido no plano: porta com `push_error` + retorno neutro; adaptador sobrescreve os 4 métodos |
| A2 | `SC-001` exige 9 códigos, mas `FR-003` lista só 3 (os de `autenticar`). Os outros 6 pertencem a `identidade`, `validar_permissao` e `revogar` | baixa | Coberto por `FR-004`, `FR-005`, `FR-006` e pela tabela em `docs/architecture/iam-interface.md:58-69`; teste deve exercitar os 9 |
| A3 | `validar_permissao` retorna `bool`; os rótulos `PERMISSAO_CONCEDIDA`/`PERMISSAO_NEGADA` são sentidos, não strings de retorno | média | Manter `bool`; rótulos apenas na mensagem/evidência do teste, nunca como `@export` ou tipo de retorno |
| A4 | `SC-002` diz "100% dos casos" sem número fixo, o que permitiria evadir a recusa | média | Teste DEVE ter no mínimo 14 verificações, incluindo 1 `PERMISSAO_NEGADA` e 1 `SESSAO_INVALIDA` |
| A5 | Nenhum alvo `Makefile` para o novo teste foi pedido | informativo | Fora de escopo deliberado; suíte roda por `--headless --script` e é somada ao R4 |
| A6 | ADR-003 diz "projeto, não implementado"; este ciclo é a implementação autorizada | informativo | `spec.md:108` registra que a ressalva vale para o ciclo anterior |

## Convergência
Não há conflito entre spec, plano, checklist e tasks. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto. R1 `go` presente. Nenhuma mudança de artefato foi feita nesta etapa (READ-ONLY).

## Veredito
Prosseguir para `implement`, com as ressalvas A1–A4 tratadas como requisitos de implementação e verificação.
