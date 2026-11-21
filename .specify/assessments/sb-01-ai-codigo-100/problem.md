# Problem — Código 100% IA

**Data**: 2026-11-21

Adotar "100% do código gerado por IA" como meta de processo para a Insurgência conflita com a verificação honesta (constituição VI) e com o determinismo (I): um percentual autodeclarado e infalsificável não diz se o diff compila, se é seguro ou se o autor o compreende. O risco concreto é a dívida de compreensão em Rust/Bevy — bibliotecas de domínio entendidas apenas pelo modelo que as gerou — agravado por `#![forbid(unsafe_code)]`/clippy pedantic e pela ausência de `.unwrap()`/`.expect()`, que tornam erros de borrow e de tipo dependentes do compilador, não da intenção do prompt. [NV — nenhum build/gate executado nesta análise]

## Critérios de sucesso
1. Nenhum KPI interno baseado em percentual de código IA; política explícita de que "100%" não é meta.
2. Todo diff passa por `verify.sh` (4 estágios, `--all-targets`) antes de merge.
3. Um humano que consiga explicar o módulo assina cada mudança; saída de LLM permanece informativa.
4. Nenhum crate de domínio (`contratos`/`sim`) depende de SDK de provedor de IA (constituição IV).
5. `verify.sh` ALL STAGES OK em cada entrega — nunca afirmar verde sem comando executado (VI).

## Custo de não agir
Dívida de compreensão mensurada (Anthropic: 17 p.p. menos no quiz), 45% de falha de segurança (Veracode) e erro dominante de borrow/checker que só o compilador pega (CRUST-Bench: 19/100 no primeiro disparo). Sem gate humano, a auditabilidade da tese de soberania se perde.
