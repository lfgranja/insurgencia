# Decision — Demo determinística headless

- **Slug**: demo-deterministica-headless
- **Decided**: 2026-11-21T13:09:00Z
- **Veredito**: go (ADOPT, origem `sb-16`)

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | Ausência de demo executável verificável no alvo; gating visual depende da SE. |
| Força da evidência | strong | Fonte primária local (`sim` headless, `verify.sh --all-targets`) + verificações executadas. |
| Valor vs. inação | adequate | Destrava `sb-15`/`sb-20`; nenhum custo de plataforma. |
| Viabilidade / appetite | strong | Um arquivo, sem dependência nova. |
| Alinhamento estratégico | strong | Constituição I/IV/VI: determinismo, soberania, verificação honesta. |
| Postura de risco | strong | Sem janela, sem rede; `[NV]` apenas no gate completo por contenção de `cargo`. |

## Racional

Único item do ranking 80/20 com valor de código verificável que ainda não estava materializado; ADOPT de `sb-08`/`sb-07`/`sb-22` não gera trabalho novo.

## Condição/responsável

Nenhuma autorização externa; `verify.sh` completo deve ser reexecutado quando os processos `cargo` alheios terminarem.

## Handoff

Problema acima; abordagem = Opção 1; escopo = exemplo + artefatos SDD 030; fora de escopo = Steam/Next Fest e publicação; métricas = compilação `-D warnings`, clippy pedantic, `diff` vazio entre execuções.
