# Concept — Funções: Barricada do Artesão e Ataque Forte validados no servidor

**Data:** 2026-11-20T09:41Z

## Opções consideradas
1. **Manter `nucleo/sim/funcoes.gd` como regra pura com os números literais do GDD e integrar de forma aditiva** (escolhida) — constantes `CUSTO_BARREIRA_MATERIAIS=20`, `CANALIZACAO_BARREIRA_SEGUNDOS=1.5`, `PV_BARREIRA=200`, `DURACAO_BARREIRA_SEGUNDOS=12.0`, `CANALIZACAO_ATAQUE_SEGUNDOS=1.2`, `DANO_BASE=100`, `DANO_ATAQUE_FORTE=200`, `RECARGA_ATAQUE_SEGUNDOS=6.0`; acrescentar preload e os ramos `construir_barreira`/`atacar` em `nucleo/mundo.gd` com `float(tick) * PASSO_SEGUNDOS`, recusa de campos forjados e de recarga em `nucleo/sim/validacao.gd`, e a suíte `tests/test_funcoes.gd` com tempo injetado e `PASS`/`OK`/exit 0. É a fatia vertical mínima que cobre 1534–1535 sem antecipar a vitória Central.
2. **PR único com funções, vitória da Ilha Central e Rodada UDP real** (rejeitada) — amplia o diff para critérios ainda **[NV]**, mistura três frentes independentes e exigiria alegações de vitória e de UDP que esta etapa não pode fazer; quebra a revisabilidade de um ciclo 80/20.
3. **Aceitar `materiais`, `dano`, `pv`, `duracao` ou `recarga` enviados pelo cliente** (rejeitada) — é exatamente o tipo de intenção forjada que o §16.4 (`:1515`, `:1531`, `:1535`) exige descartar; o cliente envia apenas `construir_barreira` ou `atacar`, e o saldo de Materiais é lido do jogador no servidor.
4. **Medir canalização e recarga com `OS.get_ticks_msec()` do cliente** (rejeitada) — permitiria acelerar os 1,5 s/1,2 s/6 s/12 s; o tempo é sempre `float(tick) * PASSO_SEGUNDOS` do servidor, com `agora` injetado no teste.
5. **Trocar a engine (Rust/Bevy ou equivalente)** (rejeitada) — ADR-001 e a restrição GDScript/Godot 4.7.2 seguem válidas; os padrões `ponto_coleta.gd`, `mundo.gd` e `test_ilha_inferior.gd` já provam a via `--headless --script`.

## Consequências
- Arquivos restritos a `nucleo/sim/funcoes.gd`, `nucleo/mundo.gd`, `nucleo/sim/validacao.gd` e `tests/test_funcoes.gd`; nenhuma dependência externa, nenhuma alteração de gate, baseline ou `Makefile`.
- Os preloads existentes de `IlhaSincronia`, `IlhaTopo` e `PontoColeta` em `nucleo/mundo.gd` são preservados; o novo preload é aditivo, nenhuma instância é declarada com `:=` (tipagem explícita `: Mundo` no teste) e `hash_estado()` continua com `HashingContext` (sem `sha256_string`).
- O servidor debita exatamente 20 Materiais só quando a canalização é aceita; a barreira nasce em 1,5 s com 200 PV e expira em 12 s; dano interrompe a canalização com reembolso integral de 20 e destrói a barreira ao zerar os PV; o Ataque Forte aplica 200 (2×100) após 1,2 s e fica indisponível por exatamente 6 s.
- `nucleo/sim/validacao.gd` é o ponto único de aplicação da intenção: `construir_barreira` e `atacar` entram em `ACOES_PERMITIDAS`, campos forjados (`materiais`, `credito`, `saldo`, `quantidade`, `valor`, `dano`, `pv`, `recarga`) são recusados antes de qualquer mutação e o uso em recarga é recusado no servidor.
- Suíte determinística com tempo injetado, contagem binária de `PASS`/`OK` e `quit(0)`/`quit(1)`; saídas não observadas permanecem **[NV]** até registro com data/hora na portaria própria, que não é escrita neste artefato.
