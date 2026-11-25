# Decision — Coleta Eficiente (BF-002k)

**Data**: 2026-11-25 · **R1**: `go` (registrado em `.specify/memory/STATE.md:625`, nota 06:40:58Z) · **R3**: 0 clarificações · **Âncora**: `origin/dev = d3be560` · **Score**: 1024/3125 (4×4×4×4×4).

## Veredito
- **Veredito**: go
- **Opção**: A — token + validação + buff na coleta + tecla `K` (Opção B, apenas token, rejeitada).
- **Handoff**: `/speckit.specify` com os requisitos e critérios abaixo; este agente não implementa nem executa gates.

## Scorecard (6 critérios)

| Critério | Avaliação (1–4) | Justificativa |
|---|---|---|
| Validade do problema | 4 | `GDD:646` define 15 Fôlego, 10 s de recarga e 5 s de dobra, mas `ACOES_PERMITIDAS` (15 tokens) não contém `coleta_eficiente`; o ativo é inalcançável na sim. |
| Força da evidência | 4 | GDD, `crates/sim/src/lib.rs` (`TICKS_COLETA`, `coleta_ticks`, `aplicar_coleta`, `avancar_coletas_automaticas`, `hash_estado`) e `STATE.md:625-635` lidos diretamente; números marcados `[HIPÓTESE]`. |
| Fechamento | 4 | Comportamento observável (90 vs 180 ticks) e consumidores fora dos artefatos: CLI `insurgencia-servidor` e cliente Bevy com tecla `K`/HUD. |
| Esforço (inverso) | 4 | Reusa canalização de coleta existente; nenhuma mudança de protocolo, stack, CI ou topologia. |
| Risco (inverso) | 4 | Risco concentrado em hash/serialização e nos golden hashes; mitigado por testes determinísticos e ausência de relógio/aleatoriedade. |
| Alinhamento estratégico | 4 | Fila 80/20 prioriza BF-002k (1024/3125); P4 (ação coletiva) e P2 (materialismo) sem inventar cânone. |

Score `4×4×4×4×4 = 1024/3125`; filtro eliminatório atendido. **R1 `go`** já registrado na memória do ciclo. R3: 0 clarificações — os números vêm de `GDD:646` e permanecem `[HIPÓTESE]`, não há decisão canônica pendente no recorte.

## Evidence Against (sustentada em `research.md`)
Passiva Ouro/XP não tem primitiva no sim (fora de escopo); `serializar_jogadores`/`hash_estado` exigem atualização conjunta de hashes e testes; não há partida real, rede ou 5v5 — efeito só é provável em sim + CLI + cliente local. Nada disso invalida o recorte ativo; tudo permanece explicitamente delimitado.

## Handoff a `/speckit.specify`

### Requisitos funcionais
- **FR-001**: adicionar `coleta_eficiente` como novo token (whitelist com 16 tokens) em `ACOES_PERMITIDAS`, com destino exigido e nível 1, sem alterar `PROTOCOL_VERSION`, `crates/contratos`, `Cargo.toml`, stack ou CI.
- **FR-002**: `ValidacaoComandos::validar` aceita o comando somente com destino igual à posição do próprio jogador, `folego >= 15` e sem buff nem recarga ativos; recusa não debita Fôlego, não avança `seq`/`request_id` e preserva `ultima_recusa`/hash.
- **FR-003**: no aceite, debitar 15 Fôlego, marcar 150 ticks de buff (5 s × 30 Hz) e 300 ticks de recarga (10 s), decrescendo um por tick, **sem acumulação**; com buff, `aplicar_coleta`/`avancar_coletas_automaticas` concluem em 90 ticks contra 180 sem buff; novos campos entram em `serializar_jogadores`/`hash_estado`.
- **FR-004**: expor o ativo a consumidores — automação via `insurgencia-servidor` (linhas `peer_id acao seq request_id [x y]`) e jogador do cliente Bevy pela tecla protótipo `K`, com HUD derivada do espelho local.

### Critérios de sucesso
- **SC-001**: comando aceito apenas com destino na própria posição, Fôlego suficiente e sem buff/recarga; segundo aceite durante buff ou recarga é recusado.
- **SC-002**: conclusão em 90 ticks com buff e 180 ticks sem buff; duração 150 ticks e recarga 300 ticks; nenhuma acumulação e nenhum relógio de parede.
- **SC-003**: recusa preserva Fôlego, IDs monotônicos e hash; estados com buff/recarga distintos geram hashes SHA-256 distintos, com golden hashes atualizados no mesmo commit.
- **SC-004**: prova executável — `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo check --locked -p insurgencia-cliente --all-targets` e `./verify.sh`; estado de execução permanece **[NV]** até observado.

## Escopo
Núcleo determinístico (`crates/sim/src/lib.rs`), testes de sim e CLI, cliente Bevy (tecla `K` + HUD), documentação do ciclo 062. Números de `GDD:646` permanecem `[HIPÓTESE]`.

## Fora de escopo
Passiva Ouro/XP (`+25%` em pontos neutros — sem primitiva no sim), topologia/ordem canônica das ilhas, partida real 5v5, rede/matchmaking/lobby, alteração de GDD/ADR/CI/stack/protocolo, PR #76, promoção `dev→main`, deploy, R6 e emissão/simulação de issues.

## Restrições
Convenções do repositório: pt-BR, `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, `Result`/`thiserror`, sem `unwrap` em produção, `BTreeMap`/`BTreeSet`, `f64::total_cmp`, lint exceptions escopadas, `const` em funções de teste antes do primeiro statement. Sem gates, testes, build, formatador, commit, push, PR, merge ou deploy neste agente; nenhum outro arquivo tocado além dos cinco artefatos deste assessment.
