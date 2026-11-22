# Concept — Vitória da Ilha Central: três ilhas de rota e canalização conjunta de 4 s

**Data:** 2026-11-20T09:48Z

## Opções consideradas
1. **Manter `IlhaCentral` como regra pura com os números literais do GDD e integrar de forma aditiva em `nucleo/mundo.gd`, com suíte determinística** (escolhida) — constantes `ILHAS_NECESSARIAS=3`, `CANALIZACAO_SEGUNDOS=4.0`, `EVENTO_VITORIA="vitoria_ilha_central"` e `FUNCOES_EXIGIDAS=["artesao","guerreiro"]`; `iniciar_canalizacao` recusa sem três ilhas ou sem Artesão e Guerreiro vivos e presentes (identidade por `peer_id`), `avancar` só declara vitória ao atingir 4,0 s contínuos, `interromper` zera o progresso e o servidor emite o evento/resultado persistido que a tela apenas exibe. É a fatia vertical mínima que cobre 1537–1541 sem antecipar partidas 5v5, UDP real ou a escala de produção.
2. **PR único com vitória Central, Rodada UDP real e 5v5** (rejeitada) — amplia o diff para critérios ainda **[NV]**, mistura frentes independentes e exigiria alegações de rede e escala que esta etapa não pode fazer; quebra a revisabilidade de um ciclo 80/20.
3. **Aplicar a regra de produção (seis ilhas, Plano Mestre e 20 s de Propaganda) ao protótipo** (rejeitada) — o GDD `:1495`/`:1507` define essa regra como exclusiva da produção e fixa a redução do MVP em três ilhas e 4 s; substituir a escala invalida o critério canônico de 1537–1541.
4. **Aceitar vitória enviada pelo cliente ou por qualquer jogador presente** (rejeitada) — `:1540`/`:1541` exigem decisão do servidor e as duas funções específicas, vivas e presentes; Caudillo, Camponês ou outro membro não pode substituir Artesão ou Guerreiro, e nenhum campo de comando pode carregar vitória pronta.
5. **Acumular progresso entre tentativas ou medir o tempo com relógio do cliente** (rejeitada) — `:1541` exige continuidade e zeração na interrupção; o tempo é sempre `float(tick) * PASSO_SEGUNDOS` do servidor, com `agora` injetado no teste, sem `OS.get_ticks_msec()`, sono ou aleatoriedade.
6. **Trocar a engine (Rust/Bevy ou equivalente)** (rejeitada) — ADR-001 e a restrição GDScript/Godot 4.7.2 seguem válidas; os padrões `ilha_topo.gd`, `funcoes.gd`, `mundo.gd` e `test_funcoes.gd` já provam a via `--headless --script`.

## Consequências
- Arquivos de implementação restritos a `nucleo/desafios/ilha_central.gd` e `nucleo/mundo.gd`; prova em `tests/test_vitoria_central.gd`. Nenhuma dependência externa, nenhuma alteração de gate, baseline, `Makefile` ou portaria nesta etapa de escrita.
- Os preloads existentes de `IlhaSincronia`, `IlhaTopo`, `PontoColeta` e `Funcoes` em `nucleo/mundo.gd` são preservados; a integração é aditiva, nenhuma instância de `Mundo.new()` é declarada com `:=` (tipagem explícita `: Mundo` no teste) e `hash_estado()` continua com `HashingContext`, sem `sha256_string`.
- O servidor recusa a canalização com menos de três ilhas, com função faltante, morta ou ausente e com substitutos; aceita somente com as duas funções identificadas, e a vitória só é declarada em `agora >= inicio + 4,0 s`, com evento único `vitoria_ilha_central`, `declarada_por="servidor"` e estado persistido para a tela de resultado.
- Morte ou saída de qualquer uma das duas funções durante a canalização interrompe e zera o progresso; um novo período exige 4,0 s completos desde o novo início (2,5 s + interrupção + 3,0 s não vencem; 3,0 s + 4,0 s vencem).
- Suíte determinística com tempo injetado e contagem binária de `PASS`/`OK`/exit 0; saídas não observadas permanecem **[NV]** até registro com data/hora na portaria própria, que não é escrita neste artefato.
