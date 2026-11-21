# Problem — Prototipagem rápida com IA

**Data**: 2026-11-21

A Insurgência precisa iterar rápido — shaders de teste, HUD provisório, ferramentas de diagnóstico — sem transformar um rascunho em artefato de produção e sem abrir janela normal enquanto a SE trabalha (regra `--minimizado`). O risco é duplo: o protótipo entrar no caminho crítico sem passar por `verify.sh`, e a curva desacelerar depois do primeiro rascunho quando sistemas, balanceamento e design crescem (E1/E2/E5). [NV — nenhum gate executado nesta análise]

## Critérios de sucesso
1. Protótipos são descartáveis e produzidos com agente somente-leitura sob `--minimizado`; nunca viram artefato final sem gates.
2. Nenhum código de protótipo entra em `crates/sim`/`contratos` sem `verify.sh` ALL STAGES OK.
3. ADR-004 preservada: engine permanece Bevy+Lightyear; nenhuma dependência nova no caminho crítico (IV).
4. Decisões de "o jogo é divertido/legível" ficam com humano/playtest, nunca com o prompt (constituição V).
5. Nenhuma janela normal aberta por agente; saída de LLM informativa, gate determinístico autoritativo (I/VI).

## Custo de não agir
Sem o guardrail, perde-se iteração rápida; com ele relaxado, o protótipo vira dívida de compreensão e contamina o gate — exatamente o padrão que o autor de Slotbound descreve ao desacelerar após os primeiros meses.
