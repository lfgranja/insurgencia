# Decision — Endurecimento do check `rust` na proteção de `dev`

- **Slug**: endurecimento-check-rust
- **Decided**: 2026-11-21T20:26:00Z
- **Veredito**: **go**

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Risco verificável: proteção de `dev` com `contexts=["gates"]` e sem `rust` em `20:24:58Z`, embora `rust pass 3m48s` na CI — merge possível com Rust falha. |
| Força da evidência | strong | Comandos executados e saídas coladas: re-leitura `20:24:58Z`, `verify.sh`/5 gates `20:25:17Z`, merge `20:25:32Z`, mutação + GET `20:25:48Z` — todos com timestamp. |
| Valor vs. inação | strong | Fecha risco de confiança na integração: `dev` passa a exigir `gates` **e** `rust`; sem custo de plataforma, janela ou dependência. |
| Viabilidade / appetite | strong | Ação remota já executada sob autorização do SE; resta documentar em 12 artefatos dentro da fronteira, sem build/gate/commit. |
| Alinhamento estratégico | strong | Constituição I/VI: verificação honesta, nenhuma afirmação sem comando executado + saída colada; SDD ordenado R1 → R4 → R7. |
| Postura de risco | strong | Risco baixo e contido: autorização explícita cobre só merge e endurecimento; deploy/publicação e assinatura GPG [NV]/DEFER; rollback documentado, nunca executado. |

## Racional

O merge do PR #36 e a mutação da proteção de `dev` já foram executados e verificados por GET em `2026-11-21T20:25:48Z`, sob autorização explícita do SE. O trabalho do ciclo 032 é de exatidão e rastreabilidade documental, não de implementação. A fronteira restrita de 12 artefatos evita sobrescritas e mantém cada artefato com um único dono.

## Condição/responsável

Autorização do SE nesta sessão: (a) merge para `dev` sempre que CI verde; (b) endurecer a proteção de `dev` com o check `rust`. Nenhuma autorização para deploy/publicação ou assinatura GPG — ambos [NV]/DEFER. Evidências já coletadas são coladas, não reexecutadas; nenhum commit, nenhuma abertura de janela do cliente, nenhuma credencial.

## Handoff

Problema acima; abordagem = Opção 1; escopo = 12 artefatos da fronteira exata; fora de escopo = código, factory, 194 paths untracked alheios, deploy/publicação, assinatura GPG, builds/gates/commits, janelas, credenciais; métricas = proteção `contexts=["gates","rust"]` confirmada por GET, PR #36 mesclado, 4 evidências datadas coladas, R4 aprovado antes de R7, release `1.0.0` com rollback documentado.
