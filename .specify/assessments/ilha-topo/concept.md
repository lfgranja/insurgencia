# Concept — Ilha Topo: três alavancas em ordem fixa

**Data:** 2026-11-20T09:28Z

## Opções consideradas
1. **Replicar o padrão Ilha Meio com os números literais do GDD** (escolhida) — `nucleo/desafios/ilha_topo.gd` com `ORDEM=[1,2,3]`, 8 s por alavanca, reinício em 2 s e limite de 45 s; `sinalizar_alavanca`/`reiniciar` com tempo injetado pelo servidor; integração em `nucleo/mundo.gd`, validação em `nucleo/sim/validacao.gd`, declaração em `modulos/exemplo/mapa.json` e suíte `extends SceneTree` com `PASS`/`OK`/exit 0. É a fatia vertical mínima que cobre os três critérios de 1519–1521.
2. **PR único com Ilha Inferior, funções e vitória Central** (rejeitada) — amplia o diff para critérios ainda **[NV]**, mistura quatro desafios independentes e antecipa evidência de execução; quebra a revisabilidade de um ciclo 80/20.
3. **Ordem sorteada por partida e janelas configuráveis pelo cliente** (rejeitada) — contraria "ordem fixa por partida" (GDD 1502) e abre a porta a captura inventada pelo cliente; o §16.4 exige validação no servidor, não negociação de parâmetros.
4. **Só UI/aviso visual, sem regra no Núcleo** (rejeitada) — produziria o aviso de 2 s, mas nenhuma captura, evento ou recusa validada; não fecha nenhum critério de aceitação.
5. **Trocar a engine (Rust/Bevy ou equivalente)** (rejeitada) — ADR-001 e a restrição GDScript/Godot 4.7.2 seguem válidas; o padrão `ilha_sincronia.gd` já prova a via `--headless --script`.

## Consequências
- Arquivos restritos a `nucleo/desafios/ilha_topo.gd`, `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `modulos/exemplo/mapa.json` e `tests/test_ilha_topo.gd`; nenhuma dependência externa e nenhuma alteração de gate ou baseline.
- Tempo sempre fornecido pelo servidor (`tick * PASSO_SEGUNDOS`), nunca `OS.get_ticks_msec()` do cliente; a janela de 8 s é por alavanca e o limite de 45 s cobre a sequência inteira.
- Erro fora de ordem zera o índice, liga o aviso visual e só aceita nova tentativa após 2 s; nenhum estado persistido `ilha_topo_concluida` é gravado nesse caminho.
- A captura e o evento só existem após validação no ponto único de aplicação (`nucleo/sim/validacao.gd`); três ativações fora da janela falham no servidor e mantêm a ilha neutra.
- Suíte determinística com tempo injetado, contagem binária de `PASS`/`OK` e `quit(0)`/`quit(1)`; saídas não observadas permanecem **[NV]** até registro com data/hora na portaria própria, que não é escrita neste artefato.
