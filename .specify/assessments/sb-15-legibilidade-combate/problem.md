# Problem — Legibilidade do combate (sb-15)

**Data**: 2026-11-21

O relato do vídeo ("dev atualizou a demo para melhorar estabilidade, desempenho e legibilidade") é hipótese **[NV]**. O problema concreto no Insurgência é anterior: o cliente não avança a simulação — `Mundo` recebe posições via `sincronizar_sim` mas nunca chama `avancar_tick` — logo não há eventos de combate cuja causa possa ser exibida. Quando o tick for ligado, o HUD em texto (`atualizar_texto`) terá que apresentar intenção, ordem e causa sem ultrapassar o teto de complexidade e sem depender de assets ou janelas normais.

## Sucesso observável
- Com semente fixa, a sequência de `hash_estado()` é idêntica antes e depois das mudanças de apresentação.
- Cada evento de combate relevante tem intenção telegrafada e causa numerada no HUD; nenhum efeito retira informação necessária à decisão.
- O HUD continua 100% procedural (`format!`), sem fontes/texturas externas; mudanças restritas a `sincronizar_avatar`/`atualizar_texto`.
- `verify.sh` ALL STAGES OK (34 testes) e 5/5 gates PASS; captura/inspeção apenas com `--minimizado` ou headless.

## Custo de não agir
Sem ligar `avancar_tick` e sem apresentação legível, o combate do cliente é uma simulação adormecida: não é possível demonstrar determinismo, auditar causalidade nem produzir evidência visual reutilizável em sb-14/sb-20. Iterar "juice" antes disso seria cosmético sobre um estado vazio.

**[NV]** Métricas de estabilidade/desempenho da demo do Slotbound não observadas; não são critério para este projeto.
