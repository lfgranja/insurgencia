# Define — Mover validado no cliente

- **Slug**: `mover-validado`
- **Created**: 2026-11-24
- **Score**: **144/3125** (`4×3×3×2×2`: impacto 4 × confiança 3 × fechamento 3 × esforço⁻¹ 2 × risco⁻¹ 2)
- **Inputs used**: `intake.md` | `research.md` | `crates/cliente/src/main.rs:108-128,618-657,662-682,1117-1129` | `crates/cliente/src/lib.rs:18-21,156-187` | `crates/sim/src/lib.rs:212,2277,2414-2445` | `STATE.md:582-592` | Constituição v1.2.2

## Problem Statement

No cliente standalone, o WASD move o avatar **fora do Núcleo**: `mover_jogador` escreve `jogador.0 = deslocar(...)` em pixels (`crates/cliente/src/main.rs:618-657`) e `sincronizar_sim` copia essa posição para `Mundo` como se fosse metros (`:1117-1129`). O limite de 1 m e o validador de `mover` existem na sim (`lib.rs:212,2414-2445`), mas o cliente nunca os consulta e nunca agenda o comando. Resultado: dois problemas acoplados — (a) movimento autoritativo inexistente no cliente, com divergência de unidades; (b) espelho contaminado por coordenadas de tela, com ordem de sistemas que desenha o avatar antes do tick. O recorte resolve ambos sem tocar em regra canônica, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede: o passo de tela é convertido para metros, submetido a `ValidacaoComandos::validar` e agendado somente no aceite; a posição local passa a ser derivada do espelho. É a fatia BF-012; **BF-003b e BF-013 ficam abertos** (DoD-5).

## DoD do ciclo (referência única; idêntico nos cinco artefatos)

- **DoD-1 (mover validado)**: `mover_jogador` constrói `Comando { acao: "mover", destino: metros }`, chama `ValidacaoComandos::validar` **antes** de `agendar_comando` e só agenda no aceite.
- **DoD-2 (sem queima de contador na recusa)**: recusa (`MovimentoForaDoLimite`, replay, imobilização) preserva a posição e **não** avança `seq`/`request_id` — *no counter burn on rejection*.
- **DoD-3 (posição derivada do espelho)**: `sincronizar_sim` lê `Mundo.jogadores[PEER_ID].posicao` e escreve `JogadorLocal` em pixels; **nunca** injeta pixels no espelho. Ordem: `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`.
- **DoD-4 (unidades)**: todos os helpers de destino/alvo operam em metros via `pixels_para_metros`; `PIXELS_POR_METRO = 100.0` é `[HIPÓTESE]`, `260 px/s → 2,6 m/s ≈ 0,087 m/tick < 1,0 m`; nenhuma regra canônica alterada.
- **DoD-5 (consumidor/backlog)**: consumidor = jogador do cliente Bevy + validação da sim/CLI; **BF-012 fechado**; BF-003b e BF-013 permanecem abertos.

## Affected Users & Stakeholders

- **Users**: jogador do cliente Bevy — precisa que WASD/setas movam o avatar **apenas quando o Núcleo aceita**, sem teleporte por injeção de pixels e sem travar após uma recusa (contadores preservados); quem usa a validação da sim/CLI — precisa que o mesmo `mover` do cliente seja aceito/recusado pelo mesmo validador do servidor.
- **Stakeholders**: mantenedores do cliente — precisam de um único caminho de comando (`ValidacaoComandos::validar` → `agendar_comando`) e de conversão explícita de unidades, reaproveitando `ComandoLocal` e o padrão dos helpers existentes (`main.rs:662-682`); mantenedores da sim — precisam que `LIMITE_MOVIMENTO_M = 1.0` e o validador (`lib.rs:2414-2445`) continuem sendo a única autoridade, sem nova regra; SWE/orquestrador — precisa de uma fatia verificável por inspeção de código + comandos de prova em `STATE.md:590`, sem que o agente execute gates; designer — precisa que `PIXELS_POR_METRO = 100.0` continue rotulado `[HIPÓTESE]`. **PR #76 não é tocado** (outro owner ativo); `dev→main`/deploy são SWE-exclusivos.

## Goals

- Fazer `mover_jogador` gerar `Comando { acao: "mover", destino: metros }` e consultar `ValidacaoComandos::validar` **antes** de `agendar_comando` — **DoD-1**.
- Garantir que `seq`/`request_id` avancem somente no aceite; recusa preserva posição e monotônicos — **DoD-2**.
- Inverter `sincronizar_sim` para ler o espelho e escrever `JogadorLocal` em pixels, proibindo injeção local no `Mundo`; reordenar para `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar` — **DoD-3**.
- Converter todo destino/alvo de ações a partir do estado local de pixels via `pixels_para_metros`, com `PIXELS_POR_METRO = 100.0` (`[HIPÓTESE]`) e passo de ≈ 0,087 m/tick abaixo de 1 m — **DoD-4**.
- Fechar **BF-012** mantendo BF-003b e BF-013 abertos — **DoD-5**.
- Preservar a fronteira: nenhuma mudança em regra canônica/GDD, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, CI, stack ou rede; nenhuma issue simulada (R6 N/A).
- Registrar rastros **FR-001..FR-004** (conversão, mover validado, sync/reordenação, evidência) e constituição v1.2.2 (R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, **R6/taskstoissues N/A**).

## Non-Goals

- Mapa completo de 12 ilhas, topologia de rotas e spawning por ilha — **BF-003b**, explicitamente fora (DoD-5).
- Consumo de `crates/contratos` por binários, `PROTOCOL_VERSION`, serialização compartilhada — **BF-013**, fora e aberto.
- Rede real, Lightyear, predição, reconciliação cliente-servidor ou lobby — ADR-004 e fronteira do contrato 059.
- Reabrir `LIMITE_MOVIMENTO_M = 1.0`, o validador da sim ou qualquer número canônico do GDD; promover `PIXELS_POR_METRO` a regra canônica.
- Novas habilidades, HUD com `hash_estado` (BF-014), captura visual, emissão de issues (R6) ou operações em `dev→main`/deploy.

## Success Metrics

- Com `mover` aceito, `Mundo.jogadores[PEER_ID].posicao` avança em metros e `JogadorLocal` reflete essa posição em pixels; o avatar/HUD só andam quando o núcleo aceita (baseline: comportamento inexistente — **[NV]** até execução).
- Com `mover` recusado (`MovimentoForaDoLimite`, replay ou imobilização), posição e `seq`/`request_id` permanecem inalterados — zero consumo de contador (baseline: **[NV]**).
- O comando gerado por `≈ 0,087 m/tick` (2,6 m/s ÷ 30 Hz) é aceito por `ValidacaoComandos::validar` com origem no espelho; um destino artificialmente além de 1 m é recusado sem agendamento (baseline: **[NV]**).
- `sincronizar_sim` não contém escrita em `remoto.posicao` a partir de `jogador.0`; a escrita é `JogadorLocal ← Mundo`, e a ordem no `.chain()` é `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar` (verificável por inspeção de código).
- Todos os helpers de destino/alvo usam `pixels_para_metros`; nenhum `f64::from(pixel)` direto permanece como destino de comando (verificável por inspeção de código).
- Nenhuma mudança em sim/contratos/protocolo/Cargo/CI/stack/rede; cobertura determinística preservada e estilo Rust (`forbid(unsafe_code)`, `deny(clippy::pedantic)`) mantido — **[NV]** até gates.

## Cost of Inaction

Sem o recorte, o cliente segue movendo o avatar por escrita local e contaminando o espelho com pixels: o limite de 1 m e o validador da sim não são exercitados pelo jogador, a reconciliação apaga o estado autoritativo e a paridade com a validação da CLI permanece quebrada. O custo não é apenas um avatar "solto": é a ausência de uma fatia onde cliente e Núcleo compartilham a mesma verdade observável antes de qualquer trabalho de rede. BF-012 continua aberto e o próximo ciclo precisa reescalonar o mesmo plumbing com risco de unit mismatch acumulado.

## Open Questions

- **Nenhuma clarificação de produto bloqueante (R3 = 0)**: o contrato fixa conversão, ordem de validação, direção da sincronização, reordenação dos sistemas, consumidor e score; nenhum `[NEEDS CLARIFICATION]` a responder neste assessment.
- **[NV]** Arredondamento pixels↔metros na fronteira exata de 1 m precisa de verificação em execução (o passo nominal ≈ 0,087 m está folgado).
- **[NV]** Nomes definitivos de SC/T na spec 059 e a conformidade de R4/R7 dependem de gates não executados aqui.
- **[NV]** Captura visual do avatar em `--minimizado` depende de janela/GPU; permanece fora deste assessment.

## Rastreabilidade (escopo do problema)

- **FR-001** → `PIXELS_POR_METRO = 100.0` e helpers `pixels_para_metros`/`metros_para_pixels` para todos os destinos/alvos.
- **FR-002** → `mover_jogador`: `Comando { acao: "mover", destino: metros }`, `ValidacaoComandos::validar` antes de `agendar_comando`, contadores só no aceite.
- **FR-003** → `sincronizar_sim` lê o espelho e escreve `JogadorLocal` em pixels; ordem `avancar_simulacao` → `sincronizar_sim` → `sincronizar_avatar`.
- **FR-004** → evidência/validação/release; prova por inspeção de código + comandos de `STATE.md:590`, **[NV]** neste assessment.
- SC-001..SC-005 → *Success Metrics* acima; T001..T010 → entregáveis da spec 059, fora desta fronteira. Os mesmos **DoD-1..DoD-5** e o score **144/3125** aparecem em `intake.md`, `research.md`, `shape.md` e `decision.md`.
