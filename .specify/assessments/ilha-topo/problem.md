# Problem — Ilha Topo: três alavancas em ordem fixa

**Data:** 2026-11-20T09:28Z

## Problema
A fatia CE-1..CE-5 está concluída (contrato, servidor, Ilha Meio, compatibilidade e `make check`), mas a **Ilha Topo segue sem implementação**: o GDD §16.7 mantém os critérios de 1519–1521 marcados como pendentes (linha 1590). A regra canônica do GDD §16.3 (linha 1502) fixa três alavancas em ordem fixa por partida, 8 s por alavanca, erro que reinicia a sequência em 2 s e limite de 45 s; o §16.4 (linhas 1518–1521) exige que a sequência correta em até 45 s capture e emita evento ao servidor, que uma alavanca fora de ordem reinicie em 2 s com aviso visual e sem captura, e que as três ativações fora da janela recebam falha no servidor.

Não existem, hoje, `nucleo/desafios/ilha_topo.gd`, a integração do desafio ao tick de `nucleo/mundo.gd`, a validação de alavanca em `nucleo/sim/validacao.gd`, a declaração de `ilha_topo` em `modulos/exemplo/mapa.json` nem `tests/test_ilha_topo.gd`. Consequentemente, não há prova determinística de captura validada no servidor nem de reinício em 2 s. Toda execução de Godot, gate ou suíte desta etapa é **[NV]**; este artefato é apenas documental e não escreve código.

## Por que é válido
- **Impacto 5** — a Ilha Topo é o primeiro desafio do jogador e o ponto onde a tese de "servidor valida a intenção" precisa ser demonstrada sem combate; sem ela, a Rota Superior do MVP permanece vazia e a fila 80/20 volta a parar no mesmo critério.
- **Evidência 5** — GDD §16.3 linha 1502 e §16.4 linhas 1518–1521 dão ordem, janelas e critérios literais; `docs/GDD - Insurgência.md:1590` registra a pendência; o padrão já mesclado de `nucleo/desafios/ilha_sincronia.gd`, `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `modulos/exemplo/mapa.json` e `tests/test_ilha_sincronia.gd` fornece o desenho a replicar; `.specify/memory/STATE.md:5` registra 5 gates PASS em 2026-11-20T03:21:31Z.
- **Custo de não agir** — nenhuma sequência correta pode capturar, nenhum erro pode reiniciar com aviso e nenhuma recusa fora da janela pode ser verificada no servidor; a condição de vitória da Ilha Central continua dependente de ilhas de rota não entregues, sem que este ciclo a implemente.

## Limites
Fora de escopo: partidas 5v5, condição completa de vitória da Ilha Central, Ilha Inferior, funções Artesão/Guerreiro, `manifesto.json`, `Makefile`, `evidence.csv`, deploy, remote, CI remoto, factory, alteração de baselines e portarias R4/R7. Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) ou de execução observada é feita aqui; o que depende de comando permanece **[NV]**.
