# Problem — Verificação determinística de código IA

**Data**: 2026-11-21

O gate local já checa compilação, estilo e testes, mas não cobre o risco específico de código gerado por IA: o autor pode integrar um módulo que compila e passa nos testes sem compreendê-lo, e o mesmo modelo que gerou o código tende a validá-lo com viés ("ciclo de autoengano"). Em Rust/Bevy com `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]` e sem `.unwrap()`/`.expect()`, os erros dominantes continuam sendo de tipo/borrow — só o compilador os pega. [NV — nenhum gate executado nesta análise]

## Critérios de sucesso
1. Revisor separado do autor do diff explica o módulo em linguagem natural antes do merge.
2. `proptest` em `contratos`/`sim` para invariantes (conservação, ausência de NaN, monotonicidade, RNG por seed).
3. Teste de determinismo: mesma seed + mesma sequência de comandos → mesmo `hash_estado()`.
4. `verify.sh` ALL STAGES OK é condição necessária (não suficiente) para merge; saída de LLM informativa (I/VI).
5. Regra `--minimizado` e ADR-004 preservadas; nenhuma janela normal aberta.

## Custo de não agir
Anthropic: 17 p.p. menos no quiz e maior dano em debugging; Veracode: 45% das amostras falham segurança; PGS: mesmo modelo validando seu código entra em autoengano. Sem revisão separada e PBT, a dívida de compreensão se acumula invisivelmente até o sistema ficar inauditável.
