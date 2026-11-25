# Concept — Coleta Eficiente (BF-002k)

**Data**: 2026-11-25 · **R1**: `go`

## Opções consideradas

### Opção A — token + validação + buff na coleta + tecla K (recomendada)
Adicionar `coleta_eficiente` como novo token (whitelist com 16 tokens) em `ACOES_PERMITIDAS` (destino obrigatório, nível 1), validar no núcleo destino igual à posição do próprio jogador, Fôlego ≥ 15 e ausência de buff/recarga; no aceite, debitar 15 Fôlego, marcar **150 ticks** de buff (5 s × 30 Hz) e **300 ticks** de recarga (10 s), sem acumulação. Em `aplicar_coleta`/`avancar_coletas_automaticas`, o limiar passa de `TICKS_COLETA = 180` para **90 ticks** enquanto o buff estiver ativo; os novos campos entram em `serializar_jogadores`/`hash_estado` e decrescem uma vez por tick. Consumidores: automação via `insurgencia-servidor` e tecla protótipo `K` no cliente Bevy, com HUD derivada do espelho local. Números de `GDD:646` permanecem `[HIPÓTESE]`; nada de GDD/ADR/CI/stack/protocolo/topologia.

**Por quê**: é a menor fatia que liga a regra já documentada à máquina de coleta já existente e a um consumidor observável, fechando o filtro eliminatório do 80/20 (1024/3125) e o DoD do ciclo.

### Opção B — apenas token (rejeitada)
Whitelistar `coleta_eficiente` sem validação, sem custo, sem buff e sem tecla/HUD. O comando passaria a ser aceito, mas **nenhum comportamento mudaria**: a conclusão continuaria em 180 ticks, o Fôlego não seria debitado, a recarga não existiria e nem sim, CLI ou cliente mostrariam diferença. Não há consumidor observável fora dos artefatos, o DoD não fecha e o score do critério de fechamento cairia de 4 para 1. Rejeitada.

## Consequências da Opção A
- Estado autoritativo muda: novos campos em `Jogador` e em `serializar_jogadores`/`hash_estado`, com atualização conjunta de golden hashes e testes. O buff é calculado somente a partir de `tick`, sem relógio nem aleatoriedade; segue `BTreeMap`/`BTreeSet`, `f64::total_cmp`, `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, `Result`/`thiserror` e sem `unwrap` em produção.
- O comando com destino diferente da posição do próprio jogador é recusado antes de qualquer débito; recusa não avança `seq`/`request_id` e não altera o hash além de `ultima_recusa`.
- A coleta em andamento não é reiniciada pela ativação: o buff altera apenas o limiar de conclusão; ao expirar, o limiar volta a 180 ticks. Sair do ponto continua zerando o acúmulo parcial, como em `avancar_coletas_automaticas`.
- Cliente e CLI usam o mesmo contrato de comando; nenhuma mensagem, `PROTOCOL_VERSION` ou dependência nova. A tecla `K` é protótipo, sem cânone inventado.
- Risco concentrado no hash e na cadência de crédito (90 ticks): mitigado por testes determinísticos em sim e CLI, `cargo check` do cliente e `./verify.sh`; execução e captura visual permanecem **[NV]** até serem observadas.

## Escopo e limites
Dentro: token, validação, custo, buff, recarga, sem acumulação, reflexo no hash, consumidor CLI e tecla `K` com HUD. Fora: passiva Ouro/XP (sem primitiva no sim), topologia das ilhas, partida real 5v5, rede, GDD/ADR/CI/stack/protocolo, PR #76, promoção `dev→main`, deploy, R6 e simulação de issues.
