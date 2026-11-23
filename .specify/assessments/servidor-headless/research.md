# Research — Servidor headless determinístico

**Data**: 2026-11-23T04:30Z · Evidências coladas do contrato da sessão, do checkout lido e de avaliações anteriores; sem reexecução de gates, build ou testes.

## Evidência primária

- [source: contrato da sessão, imutável] Novo crate `crates/servidor`, binário `insurgencia-servidor`, dependência única `insurgencia-sim`; nenhuma dependência externa nova, nenhum `async`, nenhum socket, nenhum Lightyear. `Cargo.toml` da workspace ganha o membro; `Cargo.lock` é gerado pelo Cargo. (confidence: high)
- [source: contrato da sessão, imutável] CLI: `--ticks N` com inteiro positivo; comandos pela stdin, um por linha, `peer_id acao seq request_id [x y]`. Coordenadas obrigatórias para `mover`, `coletar_materiais` e `construir_barreira`; proibidas para as demais ações conhecidas. Comandos agendaados em ordem antes do tick 0. (confidence: high)
- [source: contrato da sessão, imutável] Saída: uma linha por tick, `<tick>\t<hash sha256>`, começando em `0`. Entrada malformada ou ação desconhecida → erro determinístico e código de saída `2`, sem pânico. (confidence: high)
- [source: contrato da sessão, imutável] Regras Rust e testes: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, `Result`/`ExitCode` para erros, `///` em itens públicos; testes unitários do parser e teste de integração CLI (saída idêntica entre duas execuções, contagem de linhas, rejeição de entrada inválida). (confidence: high)
- [source: `Cargo.toml:1-5`, lido] Workspace com `resolver = "2"` e `members = ["crates/*"]`. Um diretório `crates/servidor` entra automaticamente no glob; a redação do contrato ("ganha o membro") pode exigir registro explícito — tratar como pergunta a validar no `/speckit.specify`, não como divergência a reabrir. (confidence: high)
- [source: `crates/` lido] Presentes `cliente`, `contratos`, `exportacao`, `metricas`, `sim`; ainda não existe `servidor`. (confidence: high)
- [source: `crates/sim/src/lib.rs:15`, lido] `PASSO_SEGUNDOS = 1.0 / 30.0` — relógio de 30 Hz do Núcleo, base da conversão de segundos em ticks. (confidence: high)
- [source: `crates/sim/src/lib.rs:38-46`, lido] `ACOES_PERMITIDAS` já lista seis ações conhecidas: `mover`, `sinalizar_sincronia`, `sinalizar_alavanca`, `coletar_materiais`, `construir_barreira`, `atacar_forte`. A regra "coordenadas proibidas para as demais ações conhecidas" cobre portanto cinco ações além de `mover`. (confidence: high)
- [source: `crates/sim/src/lib.rs:95-105`, lido] `Comando` tem `seq: i64`, `request_id: i64` e `destino: Option<(f64, f64)>` — o par `x y` da stdin mapeia diretamente para `destino`, sem novo campo de contrato. (confidence: high)
- [source: `crates/sim/src/lib.rs:110-127,138-148`, lido] `Mundo` contém `tick`, `jogadores: BTreeMap`, `comandos_pendentes: VecDeque`, `comandos_do_tick: Vec` e `barreiras`; `Mundo::new()` inicia com `tick: 0`, `epoch: 1`, `match_id: 1` e mapas vazios. (confidence: high)
- [source: `crates/sim/src/lib.rs:199-215`, lido] `agendar_comando` faz `push_back` em `comandos_pendentes` (ordem de chegada preservada); `avancar_tick` roda `fase_prepare` → `fase_skills` → `fase_finish` e incrementa `tick`. `fase_prepare` drena **todos** os pendentes para `comandos_do_tick` — os comandos agendaados antes do tick 0 são consumidos no primeiro tick. (confidence: high)
- [source: `crates/sim/src/lib.rs:430`, lido] `hash_estado(&self) -> String` é o hash SHA-256 canônico do estado; é a única fonte de verdade para a coluna `<hash sha256>` da saída — o servidor não deve calcular hash próprio. (confidence: high)
- [source: `crates/sim/src/lib.rs:480,520-545`, lido] `CodigoRecusa` inclui `AcaoDesconhecida`, `DestinoInvalido`, `SeqOuRequestReplay`, `PeerNaoAutorizado` e `MovimentoForaDoLimite`; `validar_comando` exige `peer_id` existente, `seq`/`request_id` monotônicos, ação conhecida e destino presente para ações que o exigem. A validação de **proibição** de coordenadas para as demais ações é regra do parser da CLI, mais estrita que a do `sim`, e deve ficar no crate servidor. (confidence: high)
- [source: `specs/042-servidor-headless/spec.md:1-9`, lido] Ainda é template Draft com marcadores `[FEATURE NAME]`/`[BRANCH]`; a definição canônica de FR/SC/T será produzida pelo `/speckit.specify` no handoff. (confidence: high)
- [source: `.specify/memory/constitution.md` (lido), v1.2.2] R1 `go`; R4 antes de R7 `1.0.0-local`; `analyze` READ-ONLY, `converge` append-only; **R6 N/A** — intenção ratificada, mas fora da sequência executável sem adaptador de issues: PARE se invocada, nunca simular issue, nunca `gh` improvisado. (confidence: high)
- [source: `.specify/assessments/demo-deterministica-headless/{concept,research,decision}.md`, lidos] Já se recomendava saída textual headless comparável por `diff`, sem janela e sem plataforma; o crate novo era preterido lá por custo de manutenção sem ganho verificado, mas é obrigatório neste contrato. (confidence: high)
- [source: `.specify/assessments/barricada-deterministica/decision.md` e `.specify/assessments/coleta-jogavel/decision.md`, lidos] Scorecards anteriores: 400/625 (barricada), 108/625 (ponte), 48/625 (combate/Fôlego), 10/625 (netcode) e 250/625 (só dívida documental). (confidence: high)

## Interpretação

O Núcleo já tem as peças determinísticas — relógio fixo, fila de comandos em ordem, `avancar_tick` por fases e `hash_estado` SHA-256. O que falta é uma **fronteira executável** que transforme essas peças em evidência comparável: um binário que leia uma sequência de comandos textual, gere o mesmo estado e imprima o mesmo hash sempre que receber a mesma entrada. A contribuição 80/20 é de integração e disciplina de interface, não de design de simulação: o contrato já fecha crate, dependências, formato de comando, regras de coordenadas, formato de saída, código de erro e testes. O risco não está no "o que fazer", mas em detalhes de implementação — parser sem pânico, erro determinístico com `ExitCode`, ordenação do agendamento e uso exclusivo de `hash_estado`.

O servidor headless também corrige uma dependência de ordem: o assessment de demo headless já apontava que a saída textual é o recibo mais barato e auditável, enquanto a rede (Lightyear) só faz sentido depois que existir um estado cujo determinismo seja verificável. Fazer o servidor agora não antecipa netcode — cria a pré-condição para ele, sem sockets, sem `async` e sem dependência proprietária (Constituição II e IV).

## Scorecard do ciclo (repetição do único scorecard)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Servidor headless (escolhida) | 4×4×4×4 = 256/625 | **256/625** |
| Só fechar a dívida documental | 2×5×5×5 = 250/625 | 250/625 |
| Combate / Fôlego | 4×2×2×3 = 48/625 | 48/625 |
| Rede Lightyear (netcode) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625 (quarto fator = risco invertido: 5 = risco mínimo, 1 = risco máximo). Mesmos valores de `intake.md`, `shape.md` e `decision.md`, sem pontuação paralela.

## Alternativas rejeitadas (mesmo scorecard único)

1. **Só fechar a dívida documental** — rejeitada como entrega isolada (250/625): impacto 2. É necessária e barata, mas não cria capacidade executável nem recibo comparável; também é a alternativa mais próxima em pontuação (diferença de apenas 6 pontos), o que reforça a necessidade de justificar a escolha pelo impacto, não pela pontuação bruta isolada.
2. **Combate / Fôlego** — rejeitada (48/625): confiança 2 e facilidade 2, além de contradizer GDD §16.3:1509 — o MVP prova jogabilidade por desafios de ilha, "em vez de combate puro". Nenhum número novo de combate é canônico para este ciclo.
3. **Rede Lightyear (netcode)** — rejeitada (10/625): impacto 5, mas confiança 2, facilidade 1 e risco máximo (fator 1). Sincronizar exige antes uma verdade determinística observável; GDD §16.3:1490 pede Núcleo isolado, não transporte primeiro. O contrato deste ciclo veta explicitamente sockets e Lightyear.

## Riscos registrados (não reabertos)

- **Parser sem pânico**: toda conversão de token (`peer_id`, `seq`, `request_id`, `x`, `y`) e toda contagem de argumentos precisam retornar `Result`, nunca `unwrap`/`expect` — o contrato exige erro determinístico código 2.
- **Regra de coordenadas divergente**: `validar_comando` do `sim` trata destino ausente, mas não proíbe destino em ações que não o usam; a proibição é do parser da CLI e precisa ser aplicada antes de montar `Comando`, sem alterar `crates/contratos`.
- **Ordem de agendamento**: `fase_prepare` consome todos os pendentes no primeiro tick; a ordem de leitura da stdin deve ser preservada por `push_back`, e a especificação precisa fixar se o tick 0 imprime o hash antes ou depois de `avancar_tick` (a saída deve começar em `0` e ter exatamente N linhas).
- **Existência do `peer_id`**: `validar_comando` recusa `PeerNaoAutorizado` se o jogador não existe; o contrato não detalha `adicionar_jogador`, então a especificação deve definir como o binário garante o jogador antes de agendar.
- **Fonte do hash**: usar somente `Mundo::hash_estado()`; reimplementar serialização no servidor criaria duas verdades e quebraria a comparabilidade com o `sim`.
- **Dívida R6 (automação, N/A)**: fora da sequência executável na v1.2.2; nenhuma issue simulada e nenhum `gh` improvisado. Não é dívida de produto.
- **Verificação**: gates, build, testes, R4 e R7 permanecem **[NV]** — nenhum foi executado neste assessment.

## Rastreabilidade

Fatos colados como observados no contrato, no checkout lido e em avaliações anteriores, sem reexecução. Nenhum comando de gate, build, git remoto, GitHub ou emissão de issue foi executado por este agente. Tudo o que depende de execução permanece **[NV]**. Rastros do ciclo: FR-001..FR-006 (crate/CLI/parser/agendamento/saída/erros), SC-001..SC-006 (critérios observáveis), T001..T012 (entregáveis); a definição canônica viverá em `specs/042-servidor-headless/`. R6 = N/A por constituição v1.2.2.
