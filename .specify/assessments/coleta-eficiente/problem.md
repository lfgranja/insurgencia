# Problem — Coleta Eficiente sem efeito no núcleo determinístico

**Data**: 2026-11-25 · **Origem**: BF-002k · **Âncora**: `origin/dev = d3be560`

## Lacuna
`docs/GDD - Insurgência.md:646` descreve o ativo **Q — Coleta Eficiente** com custo 15 Fôlego, recarga 10 s e "dobra a taxa de coleta por 5 s" (`[HIPÓTESE]`). O núcleo autoritativo, porém, só sabe coletar: `crates/sim/src/lib.rs` define `TICKS_COLETA = 180`, `Jogador.coleta_ticks`, `aplicar_coleta` e `avancar_coletas_automaticas`, mas `ACOES_PERMITIDAS` tem 15 tokens e **não inclui `coleta_eficiente`**. Não existe comando para ativar o ativo, validação de custo/recarga, buff que altere o limiar de conclusão nem campo de estado que permita observar o efeito. Consequentemente, o comportamento do Camponês no cliente e na automação via CLI é indistinguível de qualquer outro jogador que apenas coleta. A nota `.specify/memory/STATE.md:625-635` (06:40:58Z) já elegeu BF-002k como próximo corte (1024/3125) justamente porque há primitiva de coleta e consumidores reais — falta ligá-los.

## Afetados
- **Jogador/Camponês (protótipo)**: não pode pagar 15 Fôlego para acelerar a coleta; o ativo do GDD não existe na simulação.
- **Núcleo determinístico (`crates/sim`)**: `fase_skills`/`aplicar_coleta` não têm caminho para o buff, e `hash_estado` não reflete duração/recarga.
- **Servidor headless (`crates/servidor`)**: a automação por stdin não tem token aceitável para exercitar o ativo nem hash que comprove o efeito.
- **Cliente Bevy (`crates/cliente`)**: nenhuma tecla e nenhum indicador na HUD derivada do espelho — o jogador não percebe a ativação.
- **Governança do ciclo 062**: sem comportamento observável e consumidor fora dos artefatos, o filtro eliminatório do 80/20 não fecha e a fila volta a parar.

## Metas
1. Whitelistar `coleta_eficiente` (novo token; whitelist com 16 tokens, com destino, nível 1), sem mudar `PROTOCOL_VERSION`, `crates/contratos`, stack ou CI.
2. Tornar a ativação condicionada a destino igual à posição do próprio jogador, 15 Fôlego disponíveis e ausência de buff/recarga ativos; recusa sem consumo de recurso e sem avanço de IDs.
3. Implementar o efeito determinístico: buff de 150 ticks (5 s × 30 Hz) que reduz o limiar de conclusão de 180 para 90 ticks, recarga de 300 ticks (10 s), decrescimento de um por tick e **sem acumulação**.
4. Refletir os novos campos em `serializar_jogadores`/`hash_estado`, preservando o determinismo entre núcleo, CLI e cliente.
5. Expor o ativo ao jogador: tecla protótipo `K` no cliente e HUD derivada do espelho; disponibilizar a mesma ação à automação via `insurgencia-servidor`.

## Critérios de sucesso observáveis
1. `ACOES_PERMITIDAS` contém `coleta_eficiente`; a validação exige destino e o aceita somente quando o destino é a posição do próprio jogador, há Fôlego ≥ 15 e não há buff nem recarga ativos.
2. Aceite consome 15 Fôlego, marca 150 ticks de buff e 300 de recarga; um segundo aceite durante buff ou recarga é recusado — sem acumulação.
3. Com buff, a coleta conclui em **90 ticks**; sem buff, em **180 ticks**. Ambos os caminhos usam o mesmo `PASSO_SEGUNDOS = 1/30` e nenhum relógio de parede.
4. Recusa preserva Fôlego, `seq`/`request_id` e `ultima_recusa`/hash; estados com buff/recarga distintos produzem hashes distintos.
5. Consumidores provam o efeito: testes em `crates/sim/tests/simulacao.rs` e `crates/servidor/tests/cli.rs`, `cargo check --locked -p insurgencia-cliente --all-targets`, tecla `K` com HUD no cliente e `./verify.sh` (remoto).
6. Números de `GDD:646` permanecem rotulados `[HIPÓTESE]`; nenhuma alteração em GDD/ADR/CI/stack/protocolo/topologia, nenhuma promoção `dev→main`, deploy ou R6.

## Custo de inação
O ativo continua existindo apenas em documento, a fila 80/20 não fecha o corte de maior score (1024/3125) e o cliente segue sem nenhuma ação do Camponês além de mover/coletar. Adiar para depois de novas mecânicas aumentaria o raio sobre o mesmo `fase_skills`/`hash_estado` e exigiria reescrever golden hashes.

## Fora de escopo
Passiva de Ouro/XP (`+25%` em pontos neutros — sem primitiva no sim), topologia/ordem canônica de ilhas, partida real 5v5, rede/matchmaking/lobby, R6 e emissão de issues, PR #76, promoção `dev→main` e deploy. Tudo o que depende de execução permanece **[NV]**.
