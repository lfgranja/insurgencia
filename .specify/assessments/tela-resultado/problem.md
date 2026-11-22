# Problem — Tela de resultado: vitória declarada pelo servidor sem apresentação

**Data:** 2026-11-20T11:39Z

## Problema

A regra de vitória da Ilha Central já existe, foi integrada e tem prova determinística: `nucleo/desafios/ilha_central.gd` produz o sinal `vitoria_ilha_central(resultado: Dictionary)` com `EVENTO_VITORIA="vitoria_ilha_central"`, `declarada_por="servidor"`, `tempo_segundos` e `estado_persistido`, e o checklist 16.7 marca a declaração pelo servidor como concluída (`docs/GDD - Insurgência.md:1596`). Falta, porém, a outra metade literal do critério de aceitação: `docs/GDD - Insurgência.md:1539` exige que o servidor "declare vitória **e exiba tela de resultado**", e essa apresentação ainda não existe. Não há diretório `nucleo/apresentacao/`, nenhum `.tscn` de resultado, e `.specify/validations/vitoria-central/validate.md:27` registra explicitamente que a "tela de resultado" é representada apenas pelo estado/evento persistido do servidor — nenhuma cena de UI Godot foi implementada. Em `.specify/memory/STATE.md:9,17,25`, "UI de resultado" permanece entre os remanescentes **[NV]**. Sem a tela, o jogador não recebe a declaração, nem o tick em que ela ocorreu, nem o estado da partida; o MVP fecha a decisão, mas não a comunica.

## Por que é válido

- **Impacto 3** — moderado, não estrutural: a decisão, a validação das três ilhas, das duas funções vivas e presentes e dos 4 s de canalização já estão implementadas e marcadas como verificadas em `:1596`. O que falta é a fatia de apresentação que torna a vitória observável ao jogador e completa literalmente `:1539` — o último item `[ ]` do checklist 16.7 voltado ao MVP. Não é uma lacuna de arquitetura nem de regra de jogo, mas é a única parte do critério de aceitação que ainda não pode ser demonstrada na interface.
- **Confiança 5** — a evidência é documental, local e inequívoca: GDD `:1537-1541` e `:1569` dão a norma e a pendência; `ilha_central.gd:11,14,70-82` dá as chaves reais do evento (`evento`, `declarada_por="servidor"`, `tempo_segundos`, `estado="VITORIA"`); `evidence.csv:8-10` e `validate.md:27` confirmam que há cobertura da regra e ausência de cena de UI; `STATE.md:9,17,25` repete o marcador **[NV]**. Não há suposição sobre comportamento remoto ou execução observada nesta etapa.
- **Custo de não agir** — a vitória continua existindo apenas como dicionário em memória e estado persistido: nenhum jogador a vê, nenhum tick de declaração é comunicado e nenhum estado de resultado é distinguido de uma partida em andamento. A tese "servidor decide" fica correta, mas incompleta perante o critério literal de `:1539`, e o fechamento do MVP permanece dependente de um item `[ ]`.

## Metas

1. **Vitória declarada pelo servidor** — a tela mostra a declaração somente quando o resultado consumido tiver `evento=="vitoria_ilha_central"` e `declarada_por=="servidor"`, usando as chaves literais de `ilha_central.gd`; sem evento válido, permanece neutra e nunca inventa, infere ou revalida a vitória.
2. **Tick** — a tela exibe o `tempo_segundos` do resultado, isto é, o tick/instante em que o servidor declarou a vitória, sem recalculá-lo nem substituí-lo por tempo local.
3. **Estado** — a tela expõe o estado observável do resultado (`VITORIA` quando a declaração válida chega; `AGUARDANDO` na ausência dela), sem terceiro estado e sem derivar estado de progresso, ilhas ou vida.

A verificação acontece em `SceneTree`, com o componente `Control` instanciado por `preload`, TAB, tipos explícitos, `PASS`/`OK` e `quit(0/1)`; nenhuma execução é reexecutada nesta etapa e as saídas permanecem **[NV]** até a portaria própria.

## Limites

Fora de escopo: partidas 5v5, lobby, matchmaking, NAT/Internet, rede nova, UDP adicional ou servidor suplementar; nova `main scene`; arquivo `.tscn`; alteração da regra de vitória ou das chaves do evento; regra de produção (seis ilhas, Plano Mestre, 20 s de Propaganda); custos de Fôlego/alcance/dimensões das versões de produção; `manifesto.json`, `Makefile`, `evidence.csv`, gates, baselines, factory e portarias R4/R7; deploy, remote, CI e qualquer consulta externa — estes permanecem **[NV]**. Nenhuma alegação de vitória efetivamente exibida, de execução de Godot, de cobertura integral da árvore canônica (39/39 caminhos) ou de verificação ao vivo é feita aqui.
