# Concept — Ilha Inferior: ponto de coleta com estoque e recarga

**Data:** 2026-11-20T09:38Z

## Opções consideradas
1. **Replicar o padrão Ilha Topo com os números literais do GDD** (escolhida) — `nucleo/desafios/ilha_inferior.gd` com `CICLO_SEGUNDOS=3.0`, `MATERIAIS_POR_CICLO=5`, `CICLOS_ESTOQUE=3` e `RECARGA_SEGUNDOS=20.0`; `coletar(peer_id, agora)` com tempo injetado pelo servidor, esgotamento ao fim do 3º ciclo e recarga em 20 s; integração em `nucleo/mundo.gd`, recusa de material forjado em `nucleo/sim/validacao.gd`, declaração em `modulos/exemplo/mapa.json` e suíte `extends SceneTree` com `PASS`/`OK`/exit 0. É a fatia vertical mínima que cobre os três critérios de 1530–1531.
2. **PR único com funções, vitória Central e Rodada UDP real** (rejeitada) — amplia o diff para critérios ainda **[NV]**, mistura quatro desafios independentes e antecipa evidência de execução; quebra a revisabilidade de um ciclo 80/20 e exigiria alegações de funções/vitória/UDP que esta etapa não pode fazer.
3. **Creditar Materiais a cada requisição do cliente, com quantidade enviada no comando** (rejeitada) — é exatamente o material forjado que o GDD 1531 exige recusar; o cliente deve enviar apenas a intenção `coletar`, nunca `materiais`/`quantidade`.
4. **Ciclo e estoque lidos do relógio do cliente (`OS.get_ticks_msec()`)** (rejeitada) — permitiria acelerar a recarga e burlar os 3 s/20 s; o tempo é sempre `float(tick) * PASSO_SEGUNDOS` do servidor, com `agora` injetado no teste.
5. **Trocar a engine (Rust/Bevy ou equivalente)** (rejeitada) — ADR-001 e a restrição GDScript/Godot 4.7.2 seguem válidas; os padrões `ilha_sincronia.gd` e `ilha_topo.gd` já provam a via `--headless --script`.

## Consequências
- Arquivos restritos a `nucleo/desafios/ilha_inferior.gd`, `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `modulos/exemplo/mapa.json` e `tests/test_ilha_inferior.gd`; nenhuma dependência externa e nenhuma alteração de gate ou baseline.
- Os preloads existentes de `IlhaSincronia` e `IlhaTopo` em `nucleo/mundo.gd` são preservados; o novo preload é aditivo, e nenhuma instância é declarada com `:=` (tipagem explícita `: Mundo` no teste).
- O servidor credita exatamente 5 Materiais por ciclo de 3 s, até 3 ciclos (15 Materiais); tentativas dentro do ciclo, com estoque zerado ou durante a recarga de 20 s são recusadas com motivo literal, sem mutar o saldo.
- `nucleo/sim/validacao.gd` é o ponto único de aplicação: a ação `coletar` entra na lista permitida, mas qualquer comando que traga `materiais`/`quantidade`/`crédito` é recusado com `CREDITO_FORJADO` antes de qualquer mutação; o saldo só nasce do retorno de `coletar()`.
- Suíte determinística com tempo injetado, contagem binária de `PASS`/`OK` e `quit(0)`/`quit(1)`; saídas não observadas permanecem **[NV]** até registro com data/hora na portaria própria, que não é escrita neste artefato.
