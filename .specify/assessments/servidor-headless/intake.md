# Intake — Servidor headless determinístico

**Data**: 2026-11-23T04:30Z · **Slug**: `servidor-headless` · **Sessão**: `session_id=20261123T043000Z-servidor-headless`, `session_title=insurgencia - servidor headless deterministico`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 042: um **servidor headless determinístico** como novo crate `crates/servidor`, com binário `insurgencia-servidor`, dependendo **apenas** de `insurgencia-sim` — sem novas dependências externas, sem `async`, sem sockets e sem Lightyear. A interface é mínima e auditável: `--ticks N` (inteiro positivo) na linha de comando e comandos pela stdin, um por linha, no formato `peer_id acao seq request_id [x y]`. As coordenadas `x y` são **obrigatórias** para `mover`, `coletar_materiais` e `construir_barreira`, e **proibidas** para as demais ações conhecidas. Todos os comandos são agendaados em ordem **antes do tick 0**. A saída é exatamente uma linha por tick, `<tick>\t<hash sha256>`, começando em `0`. Entrada malformada ou ação desconhecida produz erro determinístico e código de saída **2**, sem pânico.

**Motivação**: entregar um "recibo" reproduzível e auditável do determinismo do Núcleo. A simulação já produz `hash_estado` SHA-256, mas hoje é exercitada por testes e pelo exemplo `demo_deterministica.rs`; falta uma fronteira executável estável, sem janela e sem plataforma, cuja saída possa ser comparada por `diff` entre execuções. O servidor headless destrava captura posterior e estabelece a verdade determinística que qualquer trabalho de rede (Lightyear) precisaria sincronizar — sem introduzir transporte, plataforma ou dependência proprietária.

## Contexto do ciclo (observado no contrato da sessão; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/servidor-headless-042/`, branch `042-servidor-headless`, base `b099093`.
- Alvo de especificação `specs/042-servidor-headless`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012.
- Constituição v1.2.2: R1 `go`, R4 antes de R7 `1.0.0-local`; **R6 N/A** — não pertence à sequência executável enquanto não existir o adaptador de issues: dívida de automação, nunca issue simulada, nunca `gh` improvisado.
- Estado verificado no checkout: `Cargo.toml` declara `members = ["crates/*"]`; `crates/` contém `cliente`, `contratos`, `exportacao`, `metricas`, `sim`; `crates/sim/src/lib.rs` expõe `Mundo::new`, `agendar_comando`, `avancar_tick` e `hash_estado` (SHA-256). `specs/042-servidor-headless/spec.md` ainda é template Draft.

## Scorecard único do ciclo

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Servidor headless (escolhida) | 4×4×4×4 = 256/625 | **256/625** |
| Só fechar a dívida documental | 2×5×5×5 = 250/625 | 250/625 |
| Combate / Fôlego | 4×2×2×3 = 48/625 | 48/625 |
| Rede Lightyear (netcode) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 5⁴ = 625 (o quarto fator é o risco invertido — segurança: 5 = risco mínimo, 1 = risco máximo). Este é o **único** scorecard do assessment; `research.md`, `shape.md` e `decision.md` repetem os mesmos valores, sem pontuação paralela.

Escolhe-se o servidor headless: é o maior valor do scorecard (256/625) e a única opção que produz um artefato executável e comparável do determinismo já existente no `sim`, com fronteira textual fechada no contrato. "Só fechar a dívida documental" (250/625) é correção necessária, mas de impacto 2 e sem capacidade nova; combate/Fôlego (48/625) contradiz a tese do MVP de desafios em vez de combate puro (GDD §16.3:1509); rede Lightyear (10/625) sincronizaria um estado que ainda não tem um meio auditável de provar determinismo, com risco máximo e facilidade mínima. Contraste rastreável: o ciclo 039 pontuou a coleta no Núcleo em 180/625, o 040 a barricada em 400/625 e o 041 a coleta jogável em 500/625 (`.specify/assessments/coleta-jogavel/decision.md`); o valor 256/625 é o máximo **dentro do conjunto de opções deste ciclo**, não uma regressão comparável entre ciclos.

## Evidências de entrada (coladas; não reexecutadas)

- Contrato da sessão (imutável): novo crate `crates/servidor`, binário `insurgencia-servidor`, dependência única `insurgencia-sim`; sem novas dependências externas, sem `async`, sem sockets, sem Lightyear. `Cargo.toml` da workspace ganha o membro; `Cargo.lock` é gerado pelo Cargo.
- Contrato da sessão (imutável): CLI `--ticks N` com inteiro positivo; comandos pela stdin em `peer_id acao seq request_id [x y]`; coordenadas obrigatórias para `mover`, `coletar_materiais` e `construir_barreira`, proibidas para as demais ações conhecidas; agendamento em ordem antes do tick 0; saída `<tick>\t<hash sha256>` por tick a partir de 0; erro determinístico e código 2 sem pânico.
- Contrato da sessão (imutável): regras Rust — `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, `Result`/`ExitCode` para erros, documentação `///` em itens públicos; testes unitários do parser e teste de integração CLI (saída idêntica entre duas execuções, contagem de linhas, rejeição de entrada inválida).
- `Cargo.toml:1-5` (lido): `members = ["crates/*"]` — a inclusão de `crates/servidor` dá o membro ao workspace pelo glob; confirmar na especificação se o contrato exige também entrada explícita.
- `crates/sim/src/lib.rs` (lido): `PASSO_SEGUNDOS = 1/30` (`:15`), `ACOES_PERMITIDAS` com seis ações (`:38-46`), `Comando { seq, request_id, destino }` (`:95-105`), `Mundo` com `tick`/`comandos_pendentes` (`:110-127`), `Mundo::new()` com `tick: 0` (`:138-148`), `agendar_comando` e `avancar_tick`/`fase_prepare` (`:199-215`), `hash_estado() -> String` SHA-256 (`:430`), `CodigoRecusa` (`:480`) e validação de comando (`:520-545`).
- `.specify/memory/constitution.md` (lido): v1.2.2 — R1 `go`; R4 antes de R7 `1.0.0-local`; `analyze` READ-ONLY, `converge` append-only; **R6 N/A** — dívida de automação, PARE se invocada, nunca issue simulada nem `gh` improvisado.
- `.specify/assessments/demo-deterministica-headless/` (lido): já recomendava saída textual headless para `diff`; a Opção 2 (novo crate) foi preterida naquele ciclo por "mais manutenção, sem ganho verificado" — aqui o crate dedicado é exigido pelo contrato, não uma escolha livre.
- `.specify/assessments/barricada-deterministica/decision.md` e `.specify/assessments/coleta-jogavel/decision.md` (lidos): fixaram 400/625, 108/625 (ponte), 48/625 (combate) e 10/625 (netcode); a dívida documental aparece como 250/625 em `coleta-jogavel`.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/servidor-headless/`. Não executa gates, build, testes ou formatadores; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `Cargo.toml`, `Cargo.lock`, `crates/` ou qualquer código-fonte. Tudo o que depende de execução permanece **[NV]**. R6 não é invocada — N/A por constituição v1.2.2.
