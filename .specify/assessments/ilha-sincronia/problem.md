# Problem — Desafio da Ilha: sincronia reduzida em 5 s

**Data:** 2026-11-20T03:10Z

## Problema
Após o ciclo 009 (servidor, validação e suítes Godot mesclados em `dev`), o Núcleo ainda **não implementa o Desafio da Ilha Meio**: ausentes `nucleo/desafios/ilha_sincronia.gd`, `modulos/exemplo/mapa.json`, a regra de produção em `modulos/base/regras.cfg`, a integração de `ilha_sincronia`/`estado_persistido` ao tick de `Mundo`, a validação da placa em `nucleo/sim/validacao.gd` e as suítes `tests/test_ilha_sincronia.gd` e `tests/test_loopback_ilha.gd`. CE-3 (Desafio de Ilha funcional) segue **sem implementação**; a checklist de implementação do GDD (§16.7) registra CE-3..CE-5 como não implementados e o inventário de caminhos canônicos segue parcial (`.specify/memory/STATE.md:9-10`). Toda execução do desafio, do gás e do evento entre clientes é **[NV]** até a data desta nota.

## Por que é válido
- Impacto na missão: 5 — sem a regra da Ilha, a tese de sincronia autoritativa (janela de 5 s medida pelo servidor, penalidade por amostra, estado persistido único) não pode ser provada; CE-3 permanece bloqueado e a fila 80/20 volta a parar no mesmo ponto.
- Evidência: 5 — `docs/plans/2026-11-19-ciclo-80-20-omoba.md` Task 4 (linhas 723–1025) dá caminhos literais, cinco constantes, assinaturas, motivos, contagens de `PASS`, linha `DELTA_EVENTO_MS` e comandos `--headless` com binário absoluto; gate determinístico 5/5 PASS em `2026-11-20T03:08:52Z` (`.specify/memory/STATE.md:8`).
- Custo de não agir: nenhuma captura, penalidade ou evento de desafio pode ser verificado, e as Tasks 5–6 seguem sem a fatia vertical que as precede.

## Limites
Fora de escopo: `manifesto.json`, `Makefile`, `evidence.csv`, partidas 5v5, condição completa de vitória da Ilha Central, elenco Artesão/Guerreiro, deploy, remote, factory, alteração de baselines e as portarias R4/R7. Nenhuma alegação de cobertura integral da árvore canônica é feita aqui; o que depende de execução permanece **[NV]**. Esta nota não escreve código nem artefatos de validação/release.
